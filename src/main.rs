extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use actix_web::{http, middleware, server, App, HttpResponse, Responder, Json, HttpMessage, AsyncResponder};
use actix_web::HttpRequest;
use futures::Future;
use actix_web::Error;

#[derive(Debug, Deserialize)]
struct Payload {
    message: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

fn check(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()
        .and_then(|val: Payload| {
            println!("model: {:?}", val);
            let pong = vec![Response{ message: "pong".to_string() }];
            Ok(HttpResponse::Ok().json(pong))
        })
        .responder()
}

fn main() {
    env_logger::init();
    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .configure(|app| {
            middleware::cors::Cors::for_app(app)
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .resource("/check", |r| {
                    r.method(http::Method::POST).f(check);
                })
                .register()})
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}
