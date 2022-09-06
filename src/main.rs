use std::{fs::File, io::BufReader};
use actix_files::Files;
use actix_web_lab::web::redirect;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use actix_web::{middleware, App, HttpServer};
use actix_web_middleware_redirect_https::RedirectHTTPS;
use chrono::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let readi: DateTime<Utc> = Utc::now();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = load_rustls_config();
    log::info!("morpho initialized at {} >>> morpho HTTPS server on port 443 using rustls TLSv1.3 and TLSv1.2", readi);
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("STRICT_TRANSPORT_SECURITY", "max-age=31536000; includeSubdomains;")))
            .wrap(middleware::DefaultHeaders::new().add(("X_CONTENT_TYPE_OPTIONS", "nosniff")))
            .wrap(middleware::DefaultHeaders::new().add(("X_FRAME_OPTIONS", "SAMEORIGIN")))
            .wrap(middleware::DefaultHeaders::new().add(("X_XSS_PROTECTION", "1; mode=block")))
            .wrap(middleware::Logger::default())
            .wrap(RedirectHTTPS::with_replacements(&[(":80".to_owned(), ":443".to_owned())]))
            .service(redirect("/", "/index.html"))
             // Note how two redirects are desired here, one with the trailing slash.
             // This will apply for subdirectory content, directory browsing and automatic redirects are blocked.
             //.service(redirect("/art", "/art/index.html"))
             //.service(redirect("/art/", "/art/index.html"))
             // The directory /app/static is the web root.
             // The ./static directory during the container image build copies in ./static recursively.
             // To override or add to the static assets, use a volume mount to /app/static/ etc.
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
