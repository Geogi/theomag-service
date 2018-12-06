extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use actix_web::{http, middleware, server, App, HttpResponse, HttpMessage, AsyncResponder};
use actix_web::HttpRequest;
use futures::Future;
use actix_web::Error;

#[derive(Deserialize)]
struct Payload {
    equipments: Vec<PayloadEquipment>,
    routes: Vec<PayloadRoute>,
}

#[derive(Deserialize)]
struct PayloadEquipment {
    key: String,
    capacity: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PayloadRoute {
    upstream_key: String,
    downstream_key: String,
}

#[derive(Serialize)]
struct Response {
    errors: Vec<String>,
}

fn check(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(|payload: Payload| {
            let dummy_response = Response {
                errors: vec![format!("test: {} equipments, {} routes", payload.equipments.len(), payload.routes.len())]
            };
            Ok(HttpResponse::Ok().json(dummy_response))
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
                    .register()
            })
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}
