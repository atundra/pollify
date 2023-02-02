mod settings;
mod storage;

use std::net::SocketAddr;

use common::grpc::poll_service::poll_service_server::{PollService, PollServiceServer};
use common::grpc::poll_service::{
    ClosePollRequest, ClosePollResponse, CreatePollRequest, CreatePollResponse,
    GetPollBySlugRequest, GetPollBySlugResponse, PollKind, PollKindsResponse, SubmitVoteRequest,
    SubmitVoteResponse, VoteOption,
};
use settings::SETTINGS;
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
        println!("Got a request: {request:?}");

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

    async fn create_poll(
        &self,
        _request: Request<CreatePollRequest>,
    ) -> Result<Response<CreatePollResponse>, Status> {
        Ok(Response::new(CreatePollResponse {
            id: 42069,
            slug: String::from("asd"),
        }))
    }

    async fn get_poll_by_slug(
        &self,
        request: Request<GetPollBySlugRequest>,
    ) -> Result<Response<GetPollBySlugResponse>, Status> {
        let message = request.into_inner();

        Ok(Response::new(GetPollBySlugResponse {
            id: 42069,
            title: String::from("Absolutely unnecessary supermarket/darkstore delivery poll"),
            kind: Some(PollKind { id: 0 }),
            slug: message.slug,
            options: vec![
                VoteOption {
                    id: 0,
                    title: String::from("Wolt Market"),
                    description: Some(String::from("The greatest of them all")),
                },
                VoteOption {
                    id: 1,
                    title: String::from("Bolt Market"),
                    description: Some(String::from("No alcohol but works at late night")),
                },
                VoteOption {
                    id: 2,
                    title: String::from("Glovo Delivery"),
                    description: Some(String::from("Everything you can imagine")),
                },
            ],
            finished: false,
            ballot_id: 0,
        }))
    }

    async fn submit_vote(
        &self,
        _request: Request<SubmitVoteRequest>,
    ) -> Result<Response<SubmitVoteResponse>, Status> {
        Ok(Response::new(SubmitVoteResponse {}))
    }

    async fn close_poll(
        &self,
        _request: Request<ClosePollRequest>,
    ) -> Result<Response<ClosePollResponse>, Status> {
        Ok(Response::new(ClosePollResponse {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(SETTINGS.grpc.host, SETTINGS.grpc.port);
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
