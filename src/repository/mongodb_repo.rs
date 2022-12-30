use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, //modify here
    results::{ InsertOneResult, UpdateResult, DeleteResult}, //modify here
    sync::{Client, Collection},
};
use crate::models::spi_model::SPI;

pub struct MongoRepo {
    col: Collection<SPI>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap(); // gets our connection, parse from MONGOURI
        let db = client.database("spi-db"); // gets a handle to rustDB database
        let col: Collection<SPI> = db.collection("spi-collection"); // get the db user collection
        MongoRepo { col }
    }

    // pub fn create_spi(&self, new_spi: SPI) -> Result<InsertOneResult, Error> {
    //     let new_doc = SPI {
    //         id: None, // tells mongoDB to auto generate user's id
    //         spi_rank: new_spi.spi_rank,
    //         country: new_spi.country,
    //         spi_score: new_spi.spi_score,
    //         basic_human_needs: new_spi.basic_human_needs,
    //         wellbeing: new_spi.wellbeing,
    //         opportunity: new_spi.opportunity,
    //         basic_nutri_med_care: new_spi.basic_nutri_med_care,
    //         water_sanitation: new_spi.water_sanitation,
    //         shelter: new_spi.shelter,
    //         personal_safety: new_spi.personal_safety,
    //         access_basic_knowledge: new_spi.access_basic_knowledge,
    //         access_info_comm: new_spi.access_info_comm,
    //         health_wellness: new_spi.health_wellness,
    //         env_quality: new_spi.env_quality,
    //         personal_rights: new_spi.personal_rights,
    //         personal_freedom_choice: new_spi.personal_freedom_choice,
    //         inclusiveness: new_spi.inclusiveness,
    //         access_adv_edu: new_spi.access_adv_edu,
    //     };
    //     let user = self
    //         .col
    //         .insert_one(new_doc, None)
    //         .ok()
    //         .expect("Error creating user");
    //     Ok(user)
    // }

    pub fn get_spi(&self, id: &String) -> Result<SPI, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    // pub fn update_spi(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let new_doc = doc! {
    //         "$set":
    //             {
    //                 "id": new_user.id,
    //                 "name": new_user.name,
    //                 "location": new_user.location,
    //                 "title": new_user.title
    //             },
    //     };
    //     let updated_doc = self
    //         .col
    //         .update_one(filter, new_doc, None)
    //         .ok()
    //         .expect("Error updating user");
    //     Ok(updated_doc)
    // }

    // pub fn delete_spi(&self, id: &String) -> Result<DeleteResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let user_detail = self
    //         .col
    //         .delete_one(filter, None)
    //         .ok()
    //         .expect("Error deleting user");
    //     Ok(user_detail)
    // }

    pub fn get_all_spis(&self) -> Result<Vec<SPI>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of spis");
        let spis = cursors.map(|doc| doc.unwrap()).collect();
        Ok(spis)
    }

}