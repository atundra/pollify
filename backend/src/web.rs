mod settings;
mod storage;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema)]
struct ExampleSchema {}

/// Get a Hello World page
#[utoipa::path(
    responses(
        (status = 200, description = "Succesfull greetings")
    )
)]
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// Get an echo of the request body
#[utoipa::path(
    responses(
        (status = 200, description = "Succesfull echo")
    )
)]
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// Get a "hey" page
#[utoipa::path(
    get,
    path = "/hey",
    responses(
        (status = 200, description = "Succesfull greetings in another way")
    )
)]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(OpenApi)]
#[openapi(paths(hello, echo, manual_hello), components(schemas(ExampleSchema,)))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
