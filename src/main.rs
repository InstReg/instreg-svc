extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;

use router::Router;

use rustc_serialize::json;

use std::io::Read;
use chrono::prelude::*;

extern crate chrono;

#[derive(RustcDecodable)]
struct User {
    name: String
}

#[derive(RustcEncodable)]
struct UserResponse {
    message: String
}


fn health(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn message(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).expect("JSON body expected");

    let user: User = json::decode(&payload).expect("User object expected");

    let greeting = UserResponse{message: format!("Hello {}", user.name) };
    let payload = json::encode(&greeting).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, payload)))
}
fn time(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).expect("JSON body expecte");

    let dt = Local::now();
    let greeting = UserResponse{message: format!("Current time {}", dt)};
    let payload = json::encode(&greeting).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, payload)))
}

fn main() {
    let mut router = Router::new();
    router.get("/health", health, "index");
    router.post("/message", message, "message");
    router.post("/time", time, "time");

    println!("Running on http://0.0.0.0:8080");
    Iron::new(router).http("0.0.0.0:8080").unwrap();
}
