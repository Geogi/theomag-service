use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::{http, middleware, server, App, AsyncResponder, HttpMessage, HttpResponse};
use futures::Future;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Payload {
    equipments: Vec<PayloadEquipment>,
    routes: Vec<PayloadRoute>,
}

#[derive(Deserialize)]
struct PayloadEquipment {
    key: String,
    capacity: Option<u64>,
    #[serde(rename = "type")]
    eq_type: PayloadEquipmentType,
}

#[derive(Deserialize)]
enum PayloadEquipmentType {
    OJN,
    DP,
    CP,
    OE,
    JU,
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

fn check_do(_payload: Payload) -> Vec<String> {
    vec![String::from("IL Y A UNE ERREUR!!!")]
}

fn has_correct_parent(_eq: PayloadEquipment, _all: Payload) -> bool {
    true
}

fn check(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()
        .and_then(|p: Payload| {
            let errors = Response {
                errors: check_do(p),
            };
            Ok(HttpResponse::Ok().json(errors))
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
