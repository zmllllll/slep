mod coder;

// use tonic::{Request, Response, Status};

// use crate::resources::Resources;

// #[derive(serde::Deserialize, serde::Serialize, Debug)]
// pub struct Login {
//     token: i64,
//     uuid: String,
// }

// pub mod core_server {
//     include!(concat!(env!("OUT_DIR"), "/json.core.Core.rs"));
// }
// use core_server::core_server::Core;

// type ResourcesStream =
//     std::pin::Pin<Box<dyn futures::Stream<Item = Result<Resources, Status>> + Send>>;

// #[derive(Default)]
// pub struct RpcCore {}

// #[tonic::async_trait]
// impl Core for RpcCore {
//     type CoreStreamStream = ResourcesStream;

//     async fn command(&self, request: Request<Resources>) -> Result<Response<Resources>, Status> {
//         println!("Got a request \n{request:#?}");

//         // let reply = HelloResponse {
//         //     message: format!("Hello {}!", request.into_inner().name),
//         // };
//         // Ok(Response::new(reply))
//         todo!()
//     }

//     async fn core_stream(
//         &self,
//         request: Request<Login>,
//     ) -> Result<tonic::Response<Self::CoreStreamStream>, tonic::Status> {
//         println!("Got a request \n{request:#?}");

//         // // creating infinite stream with requested message
//         // let repeat = std::iter::repeat(EchoResponse {
//         //     message: req.into_inner().message,
//         // });
//         // let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

//         // // spawn and channel are required if you want handle "disconnect" functionality
//         // // the `out_stream` will not be polled after client disconnect
//         // let (tx, rx) = mpsc::channel(128);
//         // tokio::spawn(async move {
//         //     while let Some(item) = stream.next().await {
//         //         match tx.send(Result::<_, Status>::Ok(item)).await {
//         //             Ok(_) => {
//         //                 // item (server response) was queued to be send to client
//         //             }
//         //             Err(_item) => {
//         //                 // output_stream was build from rx and both are dropped
//         //                 break;
//         //             }
//         //         }
//         //     }
//         //     println!("\tclient disconnected");
//         // });

//         // let output_stream = ReceiverStream::new(rx);
//         // Ok(Response::new(
//         //     Box::pin(output_stream) as Self::ServerStreamingEchoStream
//         // ))

//         todo!()
//     }
// }
// // #[tokio::main]
// // async fn main() -> Result<(), Box<dyn std::error::Error>> {
// //     let addr = "[::1]:50051".parse().unwrap();
// //     let greeter = MyGreeter::default();

// //     println!("GreeterServer listening on {}", addr);

// //     Server::builder()
// //         .add_service(GreeterServer::new(greeter))
// //         .serve(addr)
// //         .await?;

// //     Ok(())
// // }
