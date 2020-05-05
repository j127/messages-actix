// this is an older way to import macros
#[macro_use]
extern crate actix_web;

// `Result` here is a type alias that actix_web uses to define its error
// type
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;

pub struct MessageApp {
    port: u16,
}

// A type can have multiple impl blocks, but there is typically only one
// main one.
impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("starting http server on 127.0.0.1:{}", self.port);

        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}
