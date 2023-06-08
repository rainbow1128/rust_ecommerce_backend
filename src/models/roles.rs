use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Roles {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub role_name: String,
    pub models: Option<Vec<Permissions>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub model_name: Option<String>,
    pub create: Option<bool>,
    pub read: Option<bool>,
    pub update: Option<bool>,
    pub delete: Option<bool>,
}

impl Default for Roles {
    fn default() -> Self {
        Roles {
            id: None,
            role_name: String::default(),
            models: Some(vec![]),
        }
    }
}
