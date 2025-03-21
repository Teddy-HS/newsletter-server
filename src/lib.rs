pub mod configuration;
pub mod routes;
pub mod startup;
// test commit

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// We were returning `impl Responder` at the very beginning.
// We are now spelling out the type explicitly given that we have
// become more familiar with `actix-web`.
// There is no performance difference! Just a stylistic choice :)
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// using web::Form<FormData> as a parameter to the handler function
// tells actix-web to parse the request body as form data
// and to pass it to the handler function as a FormData instance
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We need to mark 'run' as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantations
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    /* HttpServer Explained
    This is where should the the application be listening requests? etc
    HttpServer handles all transport level concerns
    */

    /* App Explained
    What does HttpServer do when it has established a connection?
    App is where we define the application logic: routes, middleware, etc
    */
    let server = HttpServer::new(|| {
        App::new()
            /* route takes two parameters:
            path: a string, possibly templated to accommodate dynamic path segments;
            route: an instance of the Route struct
            Route combines a handler with a set of guards
            guards specify conditions that a request must satisfy in order to "match" and be passed over to handler
            web::get() is a guard that matches GET requests
            .to(health_check) is a handler function, health_check is handler
            */
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
