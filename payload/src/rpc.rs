pub mod client;

pub(crate) use client::global_id::global_id_client::GlobalIdClient;

pub(crate) async fn gid_grpc_client() -> GlobalIdClient<tonic::transport::Channel> {
    GlobalIdClient::connect("http://levitas.quakeai.tech:31041")
        .await
        .unwrap()
}
