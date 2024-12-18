use serde::{Serialize};

#[derive(Serialize)]
pub struct Person {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}
