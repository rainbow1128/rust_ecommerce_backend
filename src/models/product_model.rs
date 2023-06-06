use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub source: String,
    pub removed_status: bool,
    pub order_limit: Option<i32>,
    pub new_arrival: bool,
    pub to_display: bool,
    pub slug: String,
    pub is_featured: bool,
    pub category: ObjectId,
    pub sub_category: ObjectId,
    pub super_sub_category: ObjectId,
    pub price: f32,
    pub discount: f32,
    pub selling_price: Option<f32>,
    pub stock: i32,
    pub description: String,
    pub tags: Option<Vec<String>>,
    pub heroimage: Option<String>,
    pub dimension: Dimension,
    pub rating: i32,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub information: Option<Vec<Information>>,
    pub related: Option<Vec<Product>>,
    pub review: Review,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub height: f32,
    pub length: f32,
    pub width: f32,
    pub weight: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Information {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub name: String,
    pub rating: i32,
    pub comment: String,
    pub published: bool,
    pub user: ObjectId,
}

impl Default for Product {
    fn default() -> Self {
        Product {
            id: None,
            name: String::default(),
            source: String::default(),
            removed_status: false,
            order_limit: Some(5),
            new_arrival: false,
            to_display: false,
            slug: String::default(),
            is_featured: false,
            category: ObjectId::default(),
            sub_category: ObjectId::default(),
            super_sub_category: ObjectId::default(),
            price: f32::default(),
            discount: 0.0,
            selling_price: None,
            stock: i32::default(),
            description: String::default(),
            tags: None,
            heroimage: Some("default.jpg".to_string()),
            dimension: Dimension {
                height: f32::default(),
                length: f32::default(),
                width: f32::default(),
                weight: f32::default(),
            },
            rating: i32::default(),
            seo_title: None,
            seo_description: None,
            information: None,
            related: None,
            review: Review {
                name: String::default(),
                rating: i32::default(),
                comment: String::default(),
                published: false,
                user: ObjectId::default(),
            },
        }
    }
}
