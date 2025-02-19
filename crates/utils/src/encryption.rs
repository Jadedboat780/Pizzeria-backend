use bcrypt::DEFAULT_COST;

pub async fn hash_password(password: &str) -> String {
    bcrypt::hash(password, DEFAULT_COST).expect("Error hash")
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).expect("Error verify")
}
