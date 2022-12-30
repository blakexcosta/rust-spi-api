use mongodb::bson::oid::ObjectId;
use rocket::serde::json::serde_json::Number;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SPI {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub spi_rank: String,
    pub country: String,
    pub spi_score: Number,
    pub basic_human_needs: Number,
    pub wellbeing: Number,
    pub opportunity: Number,
    pub basic_nutri_med_care: Number,
    pub water_sanitation: Number,
    pub shelter: Number,
    pub personal_safety: Number,
    pub access_basic_knowledge: Number,
    pub access_info_comm: Number,
    pub health_wellness: Number,
    pub env_quality: Number,
    pub personal_rights: Number,
    pub personal_freedom_choice: Number,
    pub inclusiveness: Number,
    pub access_adv_edu: Number,
}