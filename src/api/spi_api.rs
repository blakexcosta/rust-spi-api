use crate::{models::spi_model::SPI, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult}; //modify here
use rocket::{http::Status, serde::json::Json, State};

    // POST Endpoint, to post new data
    // #[post("/user", data = "<new_user>")]
    // pub fn create_user( db: &State<MongoRepo>, new_user: Json<User>,) -> Result<Json<InsertOneResult>, Status> {
    //     let data = User {
    //         id: None,
    //         name: new_user.name.to_owned(),
    //         location: new_user.location.to_owned(),
    //         title: new_user.title.to_owned(),
    //     };
    //     let user_detail = db.create_user(data);
    //     match user_detail {
    //         Ok(user) => Ok(Json(user)),
    //         Err(_) => Err(Status::InternalServerError),
    //     }
    // }

    // GET Endpoint for a specific user
    #[get("/spi/<path>")]
    pub fn get_spi(db: &State<MongoRepo>, path: String) -> Result<Json<SPI>, Status> {
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let spi_detail = db.get_spi(&id);
        match spi_detail {
            Ok(spi) => Ok(Json(spi)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

    // PUT Endpoint for a specific user
    // #[put("/user/<path>", data = "<new_user>")]
    // pub fn update_user( db: &State<MongoRepo>, path: String, new_user: Json<User>,) -> Result<Json<User>, Status> {
    //     let id = path;
    //     if id.is_empty() {
    //         return Err(Status::BadRequest);
    //     };
    //     let data = User {
    //         id: Some(ObjectId::parse_str(&id).unwrap()),
    //         name: new_user.name.to_owned(),
    //         location: new_user.location.to_owned(),
    //         title: new_user.title.to_owned(),
    //     };
    //     let update_result = db.update_user(&id, data);
    //     match update_result {
    //         Ok(update) => {
    //             if update.matched_count == 1 {
    //                 let updated_user_info = db.get_user(&id);
    //                 return match updated_user_info {
    //                     Ok(user) => Ok(Json(user)),
    //                     Err(_) => Err(Status::InternalServerError),
    //                 };
    //             } else {
    //                 return Err(Status::NotFound);
    //             }
    //         }
    //         Err(_) => Err(Status::InternalServerError),
    //     }
    // }

    // DELETE Endpoint for a specific user
    // #[delete("/user/<path>")]
    // pub fn delete_user(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    //     let id = path;
    //     if id.is_empty() {
    //         return Err(Status::BadRequest);
    //     };
    //     let result = db.delete_user(&id);
    //     match result {
    //         Ok(res) => {
    //             if res.deleted_count == 1 {
    //                 return Ok(Json("User successfully deleted!"));
    //             } else {
    //                 return Err(Status::NotFound);
    //             }
    //         }
    //         Err(_) => Err(Status::InternalServerError),
    //     }
    // }

    // GET all users in an endpoint
    #[get("/spis")]
    pub fn get_all_spis(db: &State<MongoRepo>) -> Result<Json<Vec<SPI>>, Status> {
        let spis = db.get_all_spis();
        match spis {
            Ok(spis) => Ok(Json(spis)),
            Err(_) => Err(Status::InternalServerError),
        }
    }