mod handlers;
mod models;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use tera::Tera;

use crate::handlers::{about, increment, index};
use crate::models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    // Initialize template engine
    let templates = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Error parsing templates: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize shared application state
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(templates.clone()))
            .app_data(app_state.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))
            .route("/increment", web::post().to(increment))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
