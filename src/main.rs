use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    if let Some(ip) = req.connection_info().realip_remote_addr() {
        let v: Vec<&str> = ip.split(":").collect();

        if v.len() <= 2 {
            return v[0].to_string();
        }
        return ip.to_string();
    }

    String::from("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "::";
    let port = 8000;
    println!("Server listening at: {}:{}", addr, port);

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind((addr, port))?
        .run()
        .await
}
