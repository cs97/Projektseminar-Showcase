pub mod models;
pub mod schema;
use crate::schema::comment::id;

use self::models::*;
use self::schema::comment::dsl::comment as comment_dsl;
use actix_files as fs;
use actix_web::Error;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use diesel::prelude::*;
use dotenvy::dotenv;
use json::object;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
//use std::env;

const PATH: &str = "/var/www/uni";
const PATH_DEBUG: &str = "static";

//------------------------------------------------------------------
// start diesel

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url =
        "postgres://postgres:MLUISCOOL@localhost/Projektseminar-Showcase-database".to_string();
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
struct CommentJson {
    title: String,
    content: String,
}

// submit
async fn submit(req_body: String) -> impl Responder {
    if cfg!(debug_assertions) {
        println!("{:?}", &req_body);
    }

    let data = serde_json::from_str::<CommentJson>(&req_body).unwrap();

    let mut conn = establish_connection();

    let new_comment = Comment {
        title: data.title,
        body: data.content,
    };

    diesel::insert_into(crate::schema::comment::table)
        .values(&new_comment)
        .execute(&mut conn)
        .expect("Fehler beim Speichern des Beitrags");

    HttpResponse::Ok()
}

async fn get_all_comment() -> impl Responder {
    let connection = &mut establish_connection();
    let results = comment_dsl
        .limit(10)
        .order_by(id.desc())
        .select(Comment::as_select())
        .load(connection)
        .expect("Error loading posts");

    let mut arr = r#"{"arr":["#.to_string();

    for p in results {
        let single_comment = object! {
            title: p.title,
            content: p.body,
        };
        arr = format!("{arr}{},", single_comment);
    }
    arr.pop();
    arr = format!("{}{}", arr, "]}");

    HttpResponse::Ok().body(arr)
}
// end api
//------------------------------------------------------------------

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

async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        println!("Server running at http://127.0.0.1:8080");

        HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(index))
                .route("/blog", web::get().to(index))
                .route("/getall", web::get().to(get_all_comment))
                .route("/submit", web::post().to(submit))
                .route("/ping", web::get().to(pong))
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
                .route("/blog", web::get().to(index))
                .route("/getall", web::get().to(get_all_comment))
                .route("/submit", web::post().to(submit))
                .route("/ping", web::get().to(pong))
                .service(fs::Files::new("/", PATH))
        })
        .bind_openssl("0.0.0.0:4443", builder)?
        .run()
        .await
    }
}
