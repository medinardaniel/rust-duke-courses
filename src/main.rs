use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Query {
    key: String,
}

async fn get_value(info: web::Query<Query>) -> impl Responder {
    let file_path = "/usr/src/rust_duke_courses/class_data.json";
    println!("Attempting to open file at: {}", file_path);
    println!("Current directory: {:?}", std::env::current_dir());
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().body("Courses file not found"),
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return HttpResponse::InternalServerError().body("Failed to read the file");
    }

    let data: Value = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse JSON"),
    };

    match data.get(&info.key) {
        Some(value) => HttpResponse::Ok().json(value),
        None => HttpResponse::NotFound().body("Key not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Retrieve the port number from the environment variable
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()); // Default to port 8080 if not set

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .route("/get_value", web::get().to(get_value))
    })
    .bind(format!("0.0.0.0:{}", port))? // Bind to the port provided by Heroku
    .run()
    .await
}
