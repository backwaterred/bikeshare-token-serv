use std::collections::HashMap;
use std::io;
use std::fs;
use std::sync::Mutex;
use std::time;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod token;

// User Endpoints

fn token(log: web::Data<Mutex<HashMap<String, Option<u32>>>>, bike_id: String) -> HttpResponse {
    match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        Ok(sys_time) => {
            let plain_token = token::build(bike_id, sys_time);
            let crypt_token = token::encrypt(&plain_token);

            {
                let token = crypt_token.clone();
                let mut log = log.lock().expect("Unable to lock log");

                log.insert(token, None);
            }

            HttpResponse::Ok().body(crypt_token)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("{:?}", e))
        },
    }
}

fn finalize(log: web::Data<Mutex<HashMap<String, Option<u32>>>>, token: web::Path<String>, duration: web::Path<u32>) -> HttpResponse {
    let token = token.into_inner();
    let duration = duration.into_inner();

    let mut log = log.lock().expect("Unable to lock log");

    match log.get(&token) {
        // This nutty-ness b.c. get returns Option<Val>
        Some(None) => {
            log.remove(&token);
            log.insert(token, Some(duration));

            HttpResponse::Ok().body("Ride finalized.")
        },
        Some(Some(_dur)) =>
            HttpResponse::Forbidden().body("Ride already finalized. Not updated."),
        None =>
            HttpResponse::BadRequest().body("No record for given token."),
    }
}

// Mgmt Endpoints

fn audit(log: web::Data<Mutex<HashMap<String, Option<u32>>>>, token: web::Path<String>, check_dur: web::Path<u32>) -> HttpResponse {
    let token = token.into_inner();
    let check_dur = check_dur.into_inner();

    let log = log.lock().expect("Unable to lock log");

    match log.get(&token) {
        // This nutty-ness b.c. get returns Option<Val>
        Some(Some(server_dur)) if *server_dur == check_dur =>
            HttpResponse::Ok().body("Audit OK. Values match"),
        Some(Some(_dur)) =>
            HttpResponse::Ok().body("Fraud detected. Given value does not match value on record."),
        Some(None) =>
            HttpResponse::Ok().body("Possible Fraud. No update for given token."),
        None =>
            HttpResponse::BadRequest().body("No record for given token."),
    }
}

fn summary(log: web::Data<Mutex<HashMap<String, Option<u32>>>>) -> HttpResponse {
    let mut rept = String::new();
    rept.push_str("<html>");
    rept.push_str("<body>");
    rept.push_str("<table>");
    rept.push_str("<tr><th>Row</th><th>Token</th><th>Time</th></tr>");

    {
        let log = log.lock().expect("Unable to acquire mutex lock");

        for (n, (tok, time)) in log.iter().enumerate() {
            let mut short_tok = tok.clone();
            short_tok.truncate(20);

            let row = match time {
                Some(time) =>
                    format!("<tr><th>{}</th><th>{}...</th><th>{}</th></tr>", n+1, short_tok, time),
                None =>
                    format!("<tr><th>{}</th><th>{}...</th><th>*Not Finalized*</th></tr>", n+1, short_tok),
            };
            rept.push_str(row.as_str());
        }
    }

    rept.push_str("</table>");
    rept.push_str("</body>");
    rept.push_str("</html>");

    HttpResponse::Ok().body(rept)
}

// Dev  Endpoints

fn test() -> HttpResponse {
    let p_token = token::build(String::from("INVALID-TEST-BIKEID"),
                              time::Duration::from_secs(0));

    let c_token = token::encrypt(&p_token);

    HttpResponse::Ok().body(c_token)
}

fn timestamp() -> HttpResponse {
    let t = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap();

    HttpResponse::Ok().body(format!("We are currently {} seconds from 1970-01-01 00:00",
                                    t.as_secs()))
}

// Helper(s)

fn file(f_name: &str) -> HttpResponse {
    match fs::read_to_string(f_name) {
        Ok(content) =>
            HttpResponse::Ok().body(content),
        Err(e) =>
            HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Unable to create ssl builder");
    ssl_builder.set_private_key_file("/etc/letsencrypt/live/getyrtokens.ddns.net/privkey.pem", SslFiletype::PEM)
               .expect("Couldn't find the private key for SSL cxn");
    ssl_builder.set_certificate_chain_file("/etc/letsencrypt/live/getyrtokens.ddns.net/fullchain.pem")
               .expect("Couldn't find the chain file for SSL cxn");

    let token_log =
        web::Data::new(
            Mutex::new(
                HashMap::<String, Option<u32>>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(token_log.clone())
        // User Endpoints
            .route("/token",          web::post().to(token))
            .route("/token/{bikeid}", web::get().to(token))
            .route("/finalize/{token}/{duration}", web::get().to(finalize))
        // Management Endpoints
            .route("/admin/audit/{token}/{duration}", web::get().to(audit))
            .route("/admin/summary",                  web::get().to(summary))
        // Dev Endpoints
            .route("/",          web::get().to(|| {file("public/docs.html")}))
            .route("/cert",      web::get().to(|| {file("id_rsa_pem.pub")}))
            .route("/style.css", web::get().to(|| {file("public/style.css")}))
            .route("/test",      web::get().to(test))
            .route("/timestamp", web::get().to(timestamp))
            .wrap(
                Cors::default()
            )
    })
    .bind("0.0.0.0:80")?
    .bind_openssl("0.0.0.0:443", ssl_builder)?
    .run()
    .await
}
