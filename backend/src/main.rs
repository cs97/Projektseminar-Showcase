use actix_files as fs;
use actix_web::Error;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn index() -> Result<fs::NamedFile, Error> {
    if cfg!(not(debug_assertions)) {
        return Ok(fs::NamedFile::open(
            "/root/Projekt-Showcase/static/index.html",
        )?);
    } else {
        return Ok(fs::NamedFile::open("../static/index.html")?);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        println!("Server running at http://127.0.0.1:8080");

        HttpServer::new(|| {
            App::new()
                //.wrap(middleware::Compress::default())
                .route("/", web::get().to(index))
                .service(fs::Files::new("/", "/root/static"))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    } else {
        println!("running server");
        let key = "/etc/letsencrypt/live/chrswr.de/privkey.pem";
        let ca_cert = "/etc/letsencrypt/live/chrswr.de/fullchain.pem";

        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file(ca_cert).unwrap();

        HttpServer::new(|| {
            App::new()
                //.wrap(middleware::Compress::default())
                .route("/", web::get().to(index))
                .service(fs::Files::new("/", "/root/static"))
        })
        .bind_openssl("0.0.0.0:443", builder)?
        .run()
        .await
    }
}
