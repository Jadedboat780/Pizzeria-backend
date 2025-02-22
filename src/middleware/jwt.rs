use api_response::AuthError;
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use jwt::JWT;

pub async fn validate_jwt(
    JWT(token_data): JWT,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AuthError> {
    let user_id = token_data.sub;
    request.extensions_mut().insert(user_id);
    let response = next.run(request).await;
    Ok(response)
}
