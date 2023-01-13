use common::grpc::poll_service::poll_service_server::{PollService, PollServiceServer};
use common::grpc::poll_service::{PollKind, PollKindsResponse};
use tonic::codegen::http::Method;
use tonic::{transport::Server, Request, Response, Status};

use common::grpc::helloworld::greeter_server::{Greeter, GreeterServer};
use common::grpc::helloworld::{HelloReply, HelloRequest};
use http::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Default)]
pub struct MyPollService {}

#[tonic::async_trait]
impl PollService for MyPollService {
    async fn poll_kinds(&self, _: Request<()>) -> Result<Response<PollKindsResponse>, Status> {
        let reply = PollKindsResponse {
            kinds: vec![PollKind { id: 0 }, PollKind { id: 1 }, PollKind { id: 2 }],
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = GreeterServer::new(MyGreeter::default());
    let poll_service = PollServiceServer::new(MyPollService::default());

    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(AllowOrigin::list(HeaderValue::from_str(
            "http://localhost:8080",
        )))
        .allow_headers([CONTENT_TYPE, HeaderName::from_bytes(b"x-grpc-web")?]);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(greeter)
        .add_service(poll_service)
        .serve(addr)
        .await?;

    Ok(())
}
