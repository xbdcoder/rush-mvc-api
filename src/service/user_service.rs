use crate::model::person::Person;
use csv::{WriterBuilder, ReaderBuilder};
use std::fs::{OpenOptions, File};
use std::error::Error;

pub fn read_csv(file_path: &str) -> Vec<Person> {
    let file = File::open(file_path).expect("Failed to open CSV file");
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    reader
        .records()
        .filter_map(|record| record.ok()) // Handle potential parsing errors
        .map(|record| Person {
            id: record[0].to_string(),
            first_name: record[1].to_string(),
            last_name: record[2].to_string(),
        })
        .collect()
}

// New function to generate an auto-incrementing ID
pub fn generate_new_id(file_path: &str) -> u32 {
    let data = read_csv(file_path);
    let highest_id = data.iter()
        .filter_map(|person| person.id.parse::<u32>().ok()) // Try to parse the ID as a u32
        .max() // Get the highest ID
        .unwrap_or(0); // Default to 0 if no valid ID is found

    highest_id + 1 // Increment the highest ID for the new user
}

// New function to append a user to the CSV file
pub fn save_user_to_csv(file_path: &str, person: &Person) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(person)?;
    Ok(())
}

pub fn filter_users_by_ids(data: Vec<Person>, ids: Vec<u32>) -> Vec<Person> {
    data.into_iter()
        .filter(|person| ids.contains(&(person.id.parse::<u32>().unwrap_or(0))))
        .collect()
}
