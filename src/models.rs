#[derive(Debug)]
struct User {
    id: i64,
    username: String,
    email: String,
    password: String,
    address: String,
    phone: String,
    avatar_url: String,
}

#[derive(Debug)]
struct Pizza {
    id: i64,
    title: String,
    ingredients: &'static [String],
    price: u16,
    rating: f64,
    photo_url: String,
}
