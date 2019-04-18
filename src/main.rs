extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(req: &HttpRequest) -> String {
    let conn = req.connection_info();
    let ip = conn.remote().unwrap();
    let v: Vec<&str> = ip.split(":").collect();
    v[0].to_string()
}

fn main() {
    let addr = "0.0.0.0:8000";
    println!("Server listening at: {}", addr);
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(addr)
        .expect("Can not bind to port 8000")
        .run();
}
