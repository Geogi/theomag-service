extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate futures;

use actix_web::{http, server, App, HttpResponse, Responder, Json, HttpMessage, AsyncResponder};
use actix_web::HttpRequest;
use actix_web::Error;
use futures::Future;

#[derive(Debug, Deserialize)]
struct Payload {
    //ptos: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

fn check(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()  // convert all errors into `Error`
        .and_then(|val: Payload| {
            println!("model: {:?}", val);
            let response = vec![Response{message: String::from("uie")}];
            Ok(HttpResponse::Ok().json(response))  // <- send response
        })
        .responder()
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/check", |r|
                r.method(http::Method::POST).f(check),
            )
    })
        .bind("127.0.0.1:8102")
        .expect("Can not bind to port 8000")
        .run();
}
