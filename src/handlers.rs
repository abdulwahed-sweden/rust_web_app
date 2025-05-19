use actix_web::{web, HttpResponse, Result};
use chrono::Local;
use tera::Tera;

use crate::models::{AboutTemplateData, AppState, IndexTemplateData};

// Handler for the index page
pub async fn index(tmpl: web::Data<Tera>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let counter = *data.counter.lock().unwrap();

    let context = IndexTemplateData {
        name: "Rust Developer".to_string(),
        counter,
    };

    let rendered = tmpl
        .render(
            "index.html",
            &tera::Context::from_serialize(&context).unwrap(),
        )
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// Handler for the about page
pub async fn about(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let context = AboutTemplateData { time: now };

    let rendered = tmpl
        .render(
            "about.html",
            &tera::Context::from_serialize(&context).unwrap(),
        )
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// Handler for incrementing the counter
pub async fn increment(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    // Redirect back to index page
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}
