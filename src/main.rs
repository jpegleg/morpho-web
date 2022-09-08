use std::{fs::File, io::BufReader};
use actix_files::Files;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use actix_web::{middleware, App, HttpServer, get, Responder};
use actix_files::NamedFile;
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use uuid::Uuid;
use chrono::prelude::*;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let txid = Uuid::new_v4();
    let peer = req.peer_addr();
    // To log the headers, bring in requ and add that to the info log, like at the end.
    //let requ = req.headers();
    log::info!("{} Transaction ID generated for {:?} visiting website root", txid, peer);
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
            // Comment this out since it is handled by the above RedirectHttps to set the header
            // instead.
            //.wrap(middleware::DefaultHeaders::new().add(("strict-transport-security", "max-age=31536000; includeSubdomains;")))
            // Note: Expect-CT header not used by default. Add as required, following the same
            // style below.
            .wrap(middleware::DefaultHeaders::new().add(("x-content-type-options", "nosniff")))
            .wrap(middleware::DefaultHeaders::new().add(("x-frame-options", "SAMEORIGIN")))
            .wrap(middleware::DefaultHeaders::new().add(("x-xss-protection", "1; mode=block")))
            // this access logging can be optionally commented to only log Transaction function
            // data above :)
            //.wrap(middleware::Logger::default())
            // We'll bring in just a more minimal format by default for the middleware logger:
            .wrap(middleware::Logger::new("%a -> HTTP %s %r size: %b time: %T"))
            .service(index)
            // add additional services here after index service before the static Files service below
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
        eprintln!("{} - morpho FATAL - Open of privkey.pem paired with cert.pem failed, server must shutdown. Use PKCS8 PEM", readu);
        std::process::exit(1);
    }
    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
