#![allow(dead_code, unused_variables)]
mod auth;
mod database;
use rand::prelude::*;

pub use auth::{AuthCredentials, Status};
pub use database::connect_to_db;

pub fn authenticate(creds: AuthCredentials) {
    let random_number = thread_rng().gen_range(100..500);
    println!("Rnadom Number is {}", random_number);
    if let Status::Connected = connect_to_db() {
        auth::login(creds)
    } else {
        println!("User could not be logged in")
    }
}
