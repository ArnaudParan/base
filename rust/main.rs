use std::env;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{Registry, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_bunyan_formatter::{JsonStorageLayer, BunyanFormattingLayer};
use tracing_log::LogTracer;
//use tracing::{info};

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
        match env::var("RHOST") {
            Ok(v) => Address::TCP(v),
            Err(_) => Address::TCP(String::from("127.0.0.1:8000")),
        }
    ;
    let lkind =
        match env::var("PRODUCTION") {
            Ok(_) => LoggingKind::Json,
            Err(_) => LoggingKind::Fmt,
        }
    ;

    LogTracer::init().expect("Unable to setup log tracer!");

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let (non_blocking_writer_fmt, _guard_fmt) = tracing_appender::non_blocking(std::io::stderr());
    let (non_blocking_writer_json, _guard_json) = tracing_appender::non_blocking(std::io::stderr());
    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer_json);
    let format = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_writer_fmt);
    let fmt_sub = Registry::default()
        .with(EnvFilter::new("INFO"))
        .with(format);
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
