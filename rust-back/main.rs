use std::env;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{Registry, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_bunyan_formatter::{JsonStorageLayer, BunyanFormattingLayer};
use tracing_log::LogTracer;
use tracing::{info};

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

enum LoggingKind {
    Fmt,
    Json,
}

#[derive(Debug)]
enum Address {
    TCP(String),
    UNIX(String),
}

#[actix_web::main]
#[tracing::instrument]
async fn main() -> std::io::Result<()> {
    let addr =
        match env::var("R_BACK_SOCK") {
            Ok(v) => Address::UNIX(v),
            Err(_) => Address::TCP(String::from("127.0.0.1:8086")),
        }
    ;
    let lkind = LoggingKind::Fmt;

    LogTracer::init().expect("Unable to setup log tracer!");

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
    let fmt_sub = tracing_subscriber::fmt::Subscriber::default()
        .with(EnvFilter::new("INFO"));
    let json_sub = Registry::default()
        .with(EnvFilter::new("INFO"))
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);

    match lkind {
        LoggingKind::Fmt => tracing::subscriber::set_global_default(fmt_sub).unwrap(),
        LoggingKind::Json => tracing::subscriber::set_global_default(json_sub).unwrap(),
    }

    match addr {
        Address::TCP(address) =>
            HttpServer::new(|| {
                App::new()
                    .wrap(TracingLogger)
                    .service(hello)
            })
            .bind(&address)?
            .run()
            .await,
        Address::UNIX(address) =>
            HttpServer::new(|| {
                App::new()
                    .wrap(TracingLogger)
                    .service(hello)
            })
            .bind_uds(&address)?
            .run()
            .await,
    }
}
