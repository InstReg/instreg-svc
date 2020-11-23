extern crate iron;
extern crate router;
extern crate time;

use iron::prelude::*;
use iron::{typemap, AfterMiddleware, BeforeMiddleware};
use iron::mime::Mime;
use iron::status;

use router::Router;

use rustc_serialize::json;

use std::io::Read;
use chrono::prelude::*;
use time::precise_time_ns;

extern crate chrono;
struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

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

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    println!("Running on http://0.0.0.0:8080");
    Iron::new(chain).http("0.0.0.0:8080").unwrap();
}
