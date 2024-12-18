use actix_web::{web, HttpResponse, Responder};
use crate::service::user_service;
use crate::model::person::Person;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::model::rate_limiter::RateLimiter;

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub ids: Vec<u32>, // List of IDs to filter the users
}

#[derive(Deserialize, Serialize)]
pub struct SaveUserRequest {
    pub first_name: String,
    pub last_name: String,
}

pub async fn get_data(
    req: web::Json<GetUserRequest>, 
    data: web::Data<Arc<Mutex<RateLimiter>>>
) -> impl Responder {
    // Check if the rate limit allows this request
    let mut rate_limiter = data.lock().unwrap();
    if !rate_limiter.allow_request() {
        return HttpResponse::TooManyRequests().body("Rate limit exceeded. Please try again later.");
    }

    // Read all users from the CSV file
    let data = user_service::read_csv("data.csv");

    // Filter users based on the provided IDs
    let filtered_data: Vec<Person> = user_service::filter_users_by_ids(data, req.ids.clone());

    if filtered_data.is_empty() {
        return HttpResponse::NotFound().body("No users found for the given IDs.");
    }

    // Return the filtered users as JSON
    HttpResponse::Ok().json(filtered_data)
}

pub async fn save_user(
    req: web::Json<SaveUserRequest>, 
    data: web::Data<Arc<Mutex<RateLimiter>>>
) -> impl Responder {
    // Check if the rate limit allows this request
    let mut rate_limiter = data.lock().unwrap();
    if !rate_limiter.allow_request() {
        return HttpResponse::TooManyRequests().body("Rate limit exceeded. Please try again later.");
    }

    // Generate a new ID for the user
    let new_id = user_service::generate_new_id("data.csv");

    // Create a Person struct with the generated ID
    let person = Person {
        id: new_id.to_string(),
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
    };

    // Save the person data to the CSV
    match user_service::save_user_to_csv("data.csv", &person) {
        Ok(_) => HttpResponse::Created().json(person), // 201 Created
        Err(e) => HttpResponse::InternalServerError().body(format!("Error saving user: {}", e)),
    }
}
