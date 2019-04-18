extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};

fn index(req: &HttpRequest) -> impl Responder {
    let conn = req.connection_info();
    let ip = conn.remote().unwrap();
    let v: Vec<&str> = ip.split(":").collect();
    format!("{}", &v[0])
}

fn main() {
    let addr = "0.0.0.0:8000";
    println!("Server listening at: {}", addr);
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(addr)
        .expect("Can not bind to port 8000")
        .run();
}
