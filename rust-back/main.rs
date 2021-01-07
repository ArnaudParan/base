use std::env;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(10, 20);
    }
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr =
        match env::var("R_BACK_SOCK") {
            Ok(v) => v,
            Err(_) => String::from("127.0.0.1:8086"),
        }
    ;
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind_uds(&addr)?
    .run()
    .await
}
