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
    // It's returning a `MessageApp`, but `Self` is the idiomatic way
    // to write it.  This doesn't take `&self` because it's run on the
    // type, not an instance of the type.
    pub fn new(port: u16) -> Self {
        MessageApp { port } // works like JS objects: { port: port }
    }

    // `&self` or `self` (including mut versions) are used with dot
    // syntaxes on _instances_ of the type, using dot syntax. This
    // takes an immutable reference, so the calling code maintains
    // ownership. If it used `self`, then ownership would be passed in,
    // and the function would usually transform the type into something
    // else (for example with interfaces that use the builder pattern).
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
