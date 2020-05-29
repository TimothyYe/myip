extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(req: &HttpRequest) -> String {
    if let Some(ip) = req.connection_info().remote() {
        let v: Vec<&str> = ip.split(":").collect();
            return v[0].to_string()
        }
    
    String::from("")
}

fn main() {
    let addr = "0.0.0.0:8000";
    println!("Server listening at: {}", addr);
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(addr)
        .expect("Can not bind to port 8000")
        .run();
}
