use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub roles: Option<Vec<ObjectId>>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            username: String::default(),
            full_name: String::default(),
            email: String::default(),
            password: String::default(),
            phone_number: String::default(),
            roles: None,
        }
    }
}
