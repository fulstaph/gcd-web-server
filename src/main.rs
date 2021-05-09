mod routes;
mod gcd;
mod config;

extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use router::Router;
use routes::{get_form, post_gcd, health};

fn main() {
    let c = config::get_config();

    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    router.get("/health", health, "health");

    println!("Serving on http://127.0.0.1{}...", c.get_port());
    Iron::new(router).http(format!("127.0.0.1{}", c.get_port())).unwrap();
}

