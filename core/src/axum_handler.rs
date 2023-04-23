use axum::{
    debug_handler,
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use serde::Serialize;

use crate::{event::Event, TokioUnboundedSender};

#[debug_handler]
pub(crate) async fn register(
    Extension(collect_tx): Extension<TokioUnboundedSender<Event>>,
    Json(user_info): Json<crate::user::UserInfo>,
) -> Response {
    let (sender, recv) = tokio::sync::oneshot::channel::<Result<(), anyhow::Error>>();

    tracing::info!("user_info: {:?}", user_info);
    let cmd =
        resource::Commands::Single(crate::resources::Resources::User(resource::Command::new(
            0,
            resource::GeneralAction::Insert {
                id: user_info.id,
                resource: payload::resources::user::User::new(
                    user_info.name,
                    user_info.profile_picture,
                ),
            },
            "Register".to_string(),
        )));
    let _ = collect_tx.send(Event::Resource(0, Box::new(cmd), Some(sender)));
    match recv.await {
        Ok(res) => match res {
            Ok(()) => serde_json::to_string(&Res::Data("register successful".to_string()))
                .unwrap()
                .into_response(),
            Err(e) => {
                let res = serde_json::to_string(&Res::Error(e.to_string())).unwrap();
                (StatusCode::BAD_REQUEST, res).into_response()
            }
        },
        Err(e) => {
            let res = serde_json::to_string(&Res::Error(e.to_string())).unwrap();
            (StatusCode::BAD_REQUEST, res).into_response()
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Res {
    Data(String),
    Error(String),
}
