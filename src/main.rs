use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::controller::user_controller::{get_data, save_user};
use crate::model::rate_limiter::RateLimiter;

mod controller;
mod model;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create shared state for rate limiting
    let rate_limiter = Arc::new(Mutex::new(RateLimiter::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(rate_limiter.clone())) // Share rate limiter state
            .route("/api/user", web::post().to(get_data)) // POST endpoint to filter users by IDs
            .route("/api/users/save", web::post().to(save_user)) // New POST endpoint to save a user
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}



// use actix_web::{web, App, HttpServer, Responder, HttpResponse};
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::{BufReader};
// use csv::Reader;

// #[derive(Serialize)]
// struct Person {
//     id: String,
//     first_name: String,
//     last_name: String,
// }

// #[derive(Deserialize)]
// struct GetUserRequest {
//     ids: Vec<u32>, // List of IDs to filter the users
// }

// fn read_csv(file_path: &str) -> Vec<Person> {
//     let file = File::open(file_path).expect("Failed to open CSV file");
//     let mut reader = Reader::from_reader(BufReader::new(file));

//     reader
//         .records()
//         .filter_map(|record| record.ok()) // Handle potential parsing errors
//         .map(|record| Person {
//             id: record[0].to_string(),
//             first_name: record[1].to_string(),
//             last_name: record[2].to_string(),
//         })
//         .collect()
// }

// async fn get_data(req: web::Json<GetUserRequest>) -> impl Responder {
//     // Read all users from the CSV file
//     let data = read_csv("data.csv");

//     // Filter users based on the provided IDs
//     let filtered_data: Vec<Person> = data
//         .into_iter()
//         .filter(|person| req.ids.contains(&(person.id.parse::<u32>().unwrap_or(0))))
//         .collect();

//     if filtered_data.is_empty() {
//         return HttpResponse::NotFound().body("No users found for the given IDs.");
//     }

//     // Return the filtered users as JSON
//     HttpResponse::Ok().json(filtered_data)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/api/user", web::post().to(get_data)) // POST endpoint to filter users by IDs
//     })
//     .bind(("0.0.0.0", 8080))?
//     .run()
//     .await
// }
