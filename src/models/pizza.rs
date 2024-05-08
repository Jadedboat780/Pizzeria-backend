use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Pizza {
    pub id: i64,
    pub title: String,
    pub ingredients: Vec<String>,
    pub price: i16,
    pub image_url: String
}

#[derive(Debug, Serialize)]
pub struct GetPizzas {
    pub pizzas: Vec<Pizza>
}

#[derive(Debug, Deserialize)]
pub struct CreatePizza {
    pub title: String,
    pub ingredients: Vec<String>,
    pub price: i16,
    pub image_url: String
}

#[derive(Debug, Deserialize)]
pub struct PutPizza {
    pub title: String,
    pub ingredients: Vec<String>,
    pub price: i16,
    pub image_url: String
}

#[derive(Debug, Deserialize)]
pub struct PatchPizza {
    pub title: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub price: Option<i16>,
    pub image_url: Option<String>
}
