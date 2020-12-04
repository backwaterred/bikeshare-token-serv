use std::io;
use std::fs;
use std::time;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod token;

fn test() -> HttpResponse {
    let p_token = token::build(String::from("INVALID-TEST-BIKEID"),
                              time::Duration::from_secs(0));

    let c_token = token::encrypt(&p_token);

    HttpResponse::Ok().body(c_token)
}

fn token(bike_id: String) -> HttpResponse {
    match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        Ok(sys_time) => {
            let plain_token = token::build(bike_id, sys_time);
            let crypt_token = token::encrypt(&plain_token);

            HttpResponse::Ok().body(crypt_token)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("{:?}", e))
        },
    }
}

fn file(f_name: &str) -> HttpResponse {
    match fs::read_to_string(f_name) {
        Ok(content) =>
            HttpResponse::Ok().body(content),
        Err(e) =>
            HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}

fn timestamp() -> HttpResponse {
    let t = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap();

    HttpResponse::Ok().body(format!("We are currently {} seconds from 1970-01-01 00:00",
                                    t.as_secs()))
}

#[actix_web::main]
async fn main() -> io::Result<()> {

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Unable to create ssl builder");
    ssl_builder.set_private_key_file("/etc/letsencrypt/live/getyrtokens.ddns.net/privkey.pem", SslFiletype::PEM)
               .expect("Couldn't find the private key for SSL cxn");
    ssl_builder.set_certificate_chain_file("/etc/letsencrypt/live/getyrtokens.ddns.net/fullchain.pem")
               .expect("Couldn't find the chain file for SSL cxn");

    HttpServer::new(|| {
        App::new()
            .route("/",      web::get().to(|| {file("public/docs.html")}))
            .route("/test",  web::get().to(test))
            .route("/timestamp",  web::get().to(timestamp))
            .route("/cert",  web::get().to(|| {file("id_rsa_pem.pub")}))
            .route("/token", web::post().to(token))
            .wrap(
                Cors::default()
            )
    })
    .bind("0.0.0.0:80")?
    .bind_openssl("0.0.0.0:443", ssl_builder)?
    .run()
    .await
}
