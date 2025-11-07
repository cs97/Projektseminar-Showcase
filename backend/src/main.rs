pub mod models;
pub mod schema;
use self::models::*;
use self::schema::comment::dsl::comment as comment_dsl;
use actix_files as fs;
use actix_web::Error;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use diesel::prelude::*;
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use std::env;

const PATH: &str = "/var/www/uni";
const PATH_DEBUG: &str = "../static";

async fn index() -> Result<fs::NamedFile, Error> {
    if cfg!(not(debug_assertions)) {
        return Ok(fs::NamedFile::open(format!(
            "{}/Projekt-Showcase/static/index.html",
            PATH
        ))?);
    } else {
        return Ok(fs::NamedFile::open(format!("{}/index.html", PATH_DEBUG))?);
    }
}

//------------------------------------------------------------------
// start diesel

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_all_posts() -> Vec<Comment> {
    let connection = &mut establish_connection();
    let results = comment_dsl
        .limit(5)
        .select(Comment::as_select())
        .load(connection)
        .expect("Error loading posts");
    return results;
}
// end diesel
//------------------------------------------------------------------

//------------------------------------------------------------------
// start api
#[derive(Serialize, Deserialize)]
struct Comment_json {
    text: String,
}

async fn create_comment(comment_json: web::Json<Comment_json>) -> impl Responder {
    let comment_json_s: Comment_json = Comment_json {
        text: format!("Created: {}", comment_json.text),
    };
    HttpResponse::Ok().json(comment_json_s)
}

async fn get_all_comment() -> impl Responder {
    HttpResponse::Ok()
}
// end api
//------------------------------------------------------------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        println!("Server running at http://127.0.0.1:8080");

        HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(index))
                .service(fs::Files::new("/", PATH_DEBUG))
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
                .route("/", web::get().to(index))
                .service(fs::Files::new("/", PATH))
        })
        .bind_openssl("0.0.0.0:443", builder)?
        .run()
        .await
    }
}
