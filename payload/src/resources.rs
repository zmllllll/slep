use std::hash::{Hash, Hasher};

use super::*;

use sqlx::{Any, Postgres, Sqlite};

pub mod group;
pub mod member;
pub mod message;
pub mod office_automation_task;
pub mod profile_picture;
pub mod read_status;
pub mod reviewer;
pub mod stream;
pub mod stream_level;
pub mod task_id;
pub mod task_receipt;
pub mod topic;
pub mod topic_level;
pub mod user;
pub mod username;

pub async fn gen_id() -> i64 {
    IDS.lock().await.get_id()
}

pub fn gen_timestamp() -> i64 {
    chrono::Local::now().timestamp_millis()
}

pub(crate) async fn gen_gid(num: i64) -> Vec<i64> {
    let gid_request = tonic::Request::new(crate::rpc::client::global_id::CreateIdReq {
        num,
        trace_id: "0".to_string(),
    });
    crate::rpc::gid_grpc_client()
        .await
        .create_id(gid_request)
        .await
        .map_err(|e| e.to_string())
        .map(|res| {
            let crate::rpc::client::global_id::CreateIdReply {
                count: _,
                global_id,
                trace_id: _,
            } = res.into_inner();
            global_id
        })
        .ok()
        .unwrap()
}

pub fn gen_addr(gid: i64, stream: &str) -> i64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    format!("[{gid}]{stream}").hash(&mut hasher);
    // format!("{:x}", hasher.finish())
    hasher.finish() as i64
}

// addr: stream or uid
pub fn gen_topic_key(gid: Option<i64>, addr: &str, topic: &str) -> i64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    format!("[{gid:?}]{addr:?}:{topic}").hash(&mut hasher);
    // format!("{:x}", hasher.finish())
    hasher.finish() as i64
}
