use std::{fs::File, io::BufReader};
use actix_files::Files;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use actix_web::{middleware, App, HttpServer, get, Responder};
use actix_files::NamedFile;
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};

use chrono::prelude::*;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let readi: DateTime<Utc> = Utc::now();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let config = load_rustls_config();
    log::info!("morpho initialized at {} >>> morpho HTTPS server on port 443 using rustls TLSv1.3 and TLSv1.2", readi);
    HttpServer::new(|| {
        App::new()
            .wrap(RedirectHttps::default())
            .wrap(RedirectHttps::with_hsts(StrictTransportSecurity::recommended()))
            //.wrap(middleware::DefaultHeaders::new().add(("strict-transport-security", "max-age=31536000; includeSubdomains;")))
            .wrap(middleware::DefaultHeaders::new().add(("x-content-type-options", "nosniff")))
            .wrap(middleware::DefaultHeaders::new().add(("x-frame-options", "SAMEORIGIN")))
            .wrap(middleware::DefaultHeaders::new().add(("x-xss-protection", "1; mode=block")))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(Files::new("/", "static"))

    })
    .bind_rustls("0.0.0.0:443", config)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("privkey.pem").unwrap());
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    if keys.is_empty() {
        let readu: DateTime<Utc> = Utc::now();
        eprintln!("{} - morpho FATAL - Open of /app/privkey.pem paired with /app/cert.pem failed, server must shutdown.", readu);
        std::process::exit(1);
    }
    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
