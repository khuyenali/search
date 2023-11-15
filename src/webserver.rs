use std::{path::PathBuf, sync::Arc};

use crate::indexer::{Config, Indexer};
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

// const MAX_SIZE: usize = 262_144; // max payload size is 256k

const FILE_PATH: &str = "inverted_index.json";

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    keys: String,
}

struct AppState {
    indexer: Indexer,
}

#[derive(Serialize)]
struct SearchResponse {
    results: Vec<String>,
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./src/static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/search")]
async fn search(
    query: web::Query<SearchQuery>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    println!("Query: {}", query.keys);
    let results = data.indexer.search(&query.keys);
    // println!("{:#?}", results);

    Ok(HttpResponse::Ok().json(SearchResponse { results }))
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let indexer = web::Data::from(Arc::new(AppState {
        indexer: Indexer::build(Config::Load(FILE_PATH))?,
    }));

    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:8080")
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE);
        // .max_age(3600);
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(indexer.clone())
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
