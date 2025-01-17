use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Pizza {
    pub id: i32,
    pub title: String,
    pub ingredients: Vec<String>,
    pub price: i16,
}

#[derive(Debug, Serialize)]
pub struct GetPizzas {
    pub pizzas: Vec<Pizza>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePizza {
    pub title: String,
    pub ingredients: Vec<String>,
    pub price: i16,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePizza {
    pub title: String,
    pub ingredients: Vec<String>,
    pub price: i16,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePizzaPartial {
    pub title: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub price: Option<i16>,
}
