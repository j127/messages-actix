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

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

// `Result` was explicitly imported from `actix_web`, which defines its error type.
#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        // a header: `hello: something` as `Option<&HeaderValue>`
        .get("hello")
        // If there is a value, call closure with the value. `to_str` on
        // `&HeaderValue` returns `Result<&str, ToStrError>`. `.ok()`
        // turns an success value from a `Result` into an `Ok` of an
        // `Option`, but if the `Result` has an error, it turns it into
        // a `None`.
        .and_then(|v| v.to_str().ok())
        // It's taking an `Option` here and unwrapping it. If it's a
        // `None`, it will run the function, which returns an `&str`.
        .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
        // You can convert the &str to String with `to_owned`,
        // `to_string`, or `into`.
        message: hello.to_owned(),
    }))
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

        // This is a closure with no arguments. The `move` keyword moves
        // the variables into the closure, otherwise references are
        // passed in. It's here to indicate that the closure is entirely
        // owned by the `HttpServer`.
        HttpServer::new(move || {
            App::new() // a collection of routes and their handlers
                .wrap(middleware::Logger::default())
                .service(index)
        })
        // The `?` means that if the returned `Result` is `Err`, return
        // early with the unwrapped `Err`(?)
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}
