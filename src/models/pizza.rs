use serde::{Serialize};

#[derive(Debug, Serialize)]
struct Pizza {
    title: String,
    ingredients: &'static [String],
    price: i16,
    rating: Option<f32>,
    photo_url: Option<String>
}
