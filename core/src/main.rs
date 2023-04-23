#![feature(
    result_option_inspect,
    async_closure,
    associated_type_defaults,
    associated_type_bounds,
    impl_trait_projections,
    async_fn_in_trait,
    let_chains
)]
#![allow(
    clippy::upper_case_acronyms,
    clippy::too_many_arguments,
    incomplete_features
)]

use anyhow::Result;
use event::{ConnectionEvent, Event};
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

type TokioUnboundedSender<T> = tokio::sync::mpsc::UnboundedSender<T>;
type TokioOneShotSender<T> = tokio::sync::oneshot::Sender<T>;
type Sender = TokioUnboundedSender<ConnectionEvent>;

mod axum_handler;
mod builder;
mod check;
mod collect;
mod constant;
mod dispatcher;
mod event;
mod receiver;
mod resources;
mod rpc;
mod services;
mod storage;
mod user;

pub(crate) static PG_POOL: once_cell::sync::OnceCell<sqlx::Pool<sqlx::Any>> =
    once_cell::sync::OnceCell::new();

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let _ = init_log();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the event source...
    let (collect_tx, collect_rx) = tokio::sync::mpsc::unbounded_channel::<Event>();
    let collect_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(collect_rx);

    let local = tokio::task::LocalSet::new();
    local.spawn_local(async move {
        event::handle(collect_rx).await;
    });

    // let config =
    //     axum_server::tls_rustls::RustlsConfig::from_pem_file("tls/cert.pem", "tls/key.rsa")
    //         .await
    //         .unwrap();

    use axum::{
        extract::Extension,
        routing::{get, post},
        Router,
    };
    use tower_http::cors::Any as tower_any;
    let app = Router::new()
        .route("/login/:token/:uuid", get(services::login))
        .route("/register", post(axum_handler::register))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_headers(tower_any)
                .allow_methods(tower_any)
                .allow_origin(tower_any)
                .expose_headers(tower_any),
        )
        .layer(Extension(collect_tx.clone()));

    local.spawn_local(async move {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        let server = axum::Server::bind(&addr).serve(app.into_make_service());
        if let Err(err) = server.await {
            tracing::error!("server error: {}", err);
        }
        // axum::Server::bind(&addr)
        //     .serve(app.into_make_service())
        //     .await
        //     .unwrap();
    });
    local.await;
}

fn init_log() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        // .with(fmt::layer())
        .with(formatting_layer)
        .init();
    // color_eyre::install()?;
    Ok(())
}
