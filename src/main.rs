#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::config::{Config, Environment};
use std::env;

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    env::var("PORT")
        .ok().and_then(|p| p.parse().ok())
        .unwrap_or(8000)
}

#[get("/webhooks")]
fn webhooks() -> &'static str {
    "Hello, world!, webhooks"
}

fn main() {
    let config = Config::build(Environment::active().unwrap())
        .port(get_server_port())
        .unwrap();

    rocket::custom(config, true)
        .mount("/", routes![webhooks])
        .launch();
}