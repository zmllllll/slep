#![feature(
    let_chains,
    async_closure,
    async_fn_in_trait,
    associated_type_defaults,
    associated_type_bounds,
    result_option_inspect,
    iterator_try_collect
)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(
    unused,
    clippy::upper_case_acronyms,
    clippy::too_many_arguments,
    incomplete_features
)]
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    collections::{BTreeMap, HashSet},
    ops::Deref,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tracing::{debug, error, info, warn};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};
use tungstenite::protocol::Message as TungMessage;

use payload::{
    error,
    resources::{gen_addr, gen_topic_key},
};
use resource::{Command, Commands, GeneralAction};
mod command;
mod constant;
mod init;
mod notify;
mod report;
mod resources;
mod sqlite_operator;
mod system_tray;
mod topic;
mod websocket;

pub(crate) static PG_POOL: once_cell::sync::OnceCell<sqlx::Pool<sqlx::Postgres>> =
    once_cell::sync::OnceCell::new();
pub(crate) static mut SQLITE_POOL: once_cell::sync::OnceCell<sqlx::Pool<sqlx::Any>> =
    once_cell::sync::OnceCell::new();
// pub(crate) static CHANNEL_SENDER: once_cell::sync::OnceCell<UnboundedSender<TungMessage>> =
//     once_cell::sync::OnceCell::new();
pub(crate) static NOTIFY_TX: once_cell::sync::OnceCell<UnboundedSender<notify::Notify>> =
    once_cell::sync::OnceCell::new();
pub(crate) static STORAGE: once_cell::sync::OnceCell<std::path::PathBuf> =
    once_cell::sync::OnceCell::new();
pub(crate) static SYSTEM_TRAY: once_cell::sync::OnceCell<system_tray::SystemTray> =
    once_cell::sync::OnceCell::new();

// lazy_static::lazy_static! {
//     static ref SQLITE_POOL : std::sync::Mutex<SqlitePool>  = std::sync::Mutex::new(SqlitePool(None)) ;
// }
// pub(crate) static SQLITE_POOL: once_cell::sync::Lazy<tokio::sync::RwLock<SqlitePool>> =
//     once_cell::sync::Lazy::new(|| tokio::sync::RwLock::new(SqlitePool(None)));
pub(crate) static CHANNEL_SENDER: once_cell::sync::Lazy<tokio::sync::RwLock<ChannelSender>> =
    once_cell::sync::Lazy::new(|| tokio::sync::RwLock::new(ChannelSender(None)));
pub(crate) static CLIENT: once_cell::sync::Lazy<tokio::sync::Mutex<Client>> =
    once_cell::sync::Lazy::new(|| tokio::sync::Mutex::new(Client { log_status: false }));

pub(crate) struct SqlitePool(pub(crate) sqlx::Pool<sqlx::Any>);

pub(crate) struct ChannelSender(pub(crate) Option<UnboundedSender<TungMessage>>);

pub(crate) struct Client {
    log_status: bool,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(id: i64) -> i64 {
    format!("Hello, {}! You've been greeted from Rust!", id);
    id
}

#[tokio::main]
async fn main() {
    let _ = init::init_log();

    let (notify_tx, notify_rx) = tokio::sync::mpsc::unbounded_channel::<notify::Notify>();
    let notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
    NOTIFY_TX.set(notify_tx).unwrap();

    // tokio::task::spawn(async {
    //     command::user::login("1606174953750073345").await;
    // });

    use tauri::Manager;
    use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
    use window_shadows::set_shadow;
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator) // ✅ 菜单分割线
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            SYSTEM_TRAY.set(system_tray::SystemTray {
                handle: app.tray_handle(),
            });
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                    }
                    _ => {}
                },
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    tokio::task::spawn(async { command::user::log_out().await });
                }
                _ => {}
            }
        })
        .setup(|app| {
            let app_dir = app
                .app_handle()
                .path_resolver()
                .app_local_data_dir()
                .unwrap_or_default();

            let storage = app_dir.join("storage");
            tracing::debug!("creating storage directory: {storage:?}");
            std::fs::create_dir_all(&storage).unwrap();
            STORAGE.set(storage).unwrap();
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");

            // let _id = app.listen_global("click", |event| {
            //     tracing::info!("got event with payload {:?}", event.payload());
            // });

            tokio::task::spawn(notify::handle(notify_rx, window));
            // app.emit_all(
            //     "click",
            //     Payload {
            //         message: '6'.to_string(),
            //     },
            // )
            // .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // command::common::err,
            // user
            command::user::login,
            command::user::log_out,
            command::user::rename_username,
            command::user::query::get_user_info,
            // group
            command::group::create_group,
            command::group::dismiss_group,
            command::group::update_group_info,
            command::group::query::get_sub_group,
            command::group::query::get_group_by_uid,
            command::group::query::get_group_member_list,
            // member
            command::member::add_member,
            command::member::dismiss_member,
            command::member::update_group_member_info,
            command::member::query::get_private_chat_list,
            // reviewer
            command::reviewer::appoint_reviewer,
            command::reviewer::dismiss_reviewer,
            // message
            command::message::send_message,
            command::message::revoke_message,
            // command::message::delete_message,
            command::message::query::get_topic_message,
            command::message::query::get_stream_message,
            command::message::query::get_private_message,
            // stream
            command::stream::update_stream_settings,
            command::stream::query::get_stream_list,
            // topic
            command::topic::update_topic_settings,
            command::topic::query::get_topic_settings,
            command::topic::query::get_topic_list,
            // office_automation_task
            command::office_automation_task::assign_task,
            command::office_automation_task::query::get_task_info,
            command::office_automation_task::query::get_task_list,
            command::office_automation_task::query::get_task_list_by_typ,
            command::office_automation_task::query::get_task_list_by_consignor,
            // task_receipt
            command::task_receipt::add_task_receipt,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    log::info!("world");
}
