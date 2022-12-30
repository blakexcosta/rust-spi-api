mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

// This is based off of this tutorial:
// https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5

// add import below
use api::spi_api::{get_spi, get_all_spis};//update_user, delete_user, get_all_users}; //import the handler here
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        // .mount("/", routes![create_user])
        .mount("/", routes![get_spi])
        // .mount("/", routes![update_user])
        // .mount("/", routes![delete_user])
        .mount("/", routes![get_all_spis])
}

