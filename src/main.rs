use std::{fs::File, io::BufReader};
use actix_files::Files;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use actix_web::{middleware, App, HttpServer, get, Responder, HttpRequest, HttpResponse};
use actix_files::NamedFile;
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use uuid::Uuid;
use chrono::prelude::*;
use std::env;
use curl::easy::{Easy, List};

#[get("/health")]
async fn healthchecks(req: HttpRequest) -> impl Responder {
    let txid = Uuid::new_v4().to_string();
    env::set_var("txid", &txid);
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    log::info!("{} {:?} /health GET request (health check) - from {:?} - {:?}", &txid, readi, peer, &requ);
    let reada: DateTime<Utc> = Utc::now();
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let mut list = List::new();
    list.append(&("txid:".to_owned() + &txid)).unwrap();
    // see https://github.com/jpegleg/squirrel-tactix
           // change to backend DNS name instead of localhost, etc
    handle.url("http://localhost:8007/health").unwrap();
    handle.http_headers(list).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
       
        transfer.perform().unwrap();
    }
    let mez: String = String::from_utf8(data.clone()).unwrap();
    let nid = env::var("txid").unwrap();
    log::info!("{} {:?} /health GET response from backend: {:?} - {:?}", nid, reada, &mez, requ);
    HttpResponse::Ok().body(mez)
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let txid = Uuid::new_v4().to_string();
    env::set_var("txid", &txid);
    let peer = req.peer_addr();
    let requ = req.headers(); 
    // Please note that the Transction ID is a sticky environment variable, so
    // there can be things that don't hit '/' in this case that use the same transaction id!
    // This behavior can be adjusted as needed using additional handlers etc. The idea is that
    // the middleware logger has a loose association with regular website visitors/users/systems
    // while any requests passed to a backend/db can have a tight transaction id association.
    log::info!("{} {:?} visiting website - {:?}", txid, peer, requ);
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
            // This access logging can be optionally commented to only log Transaction function
            // data above :)
            //.wrap(middleware::Logger::default())
            // We'll bring in a custom that includes the transaction ID by default for the middleware logger:
            .wrap(middleware::Logger::new("%{txid}e %a -> HTTP %s %r size: %b server-time: %T %{Referer}i %{User-Agent}i"))
            .service(index)
            .service(healthchecks)
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
        eprintln!("{} - morpho FATAL - Open of privkey.pem paired with cert.pem failed, server must shutdown. Use PKCS8 PEM compatible with rustls.", readu);
        std::process::exit(1);
    }
    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
