use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response
};
use utils::{
    jwt::JWT,
    api_response::AuthError
};

pub async fn validate_jwt(
    JWT(token_data): JWT,
    mut request: Request<Body>,
    next: Next
) ->  Result<Response, AuthError> {
    let user_id = token_data.sub;
    request.extensions_mut().insert(user_id);
    let response = next.run(request).await;
    Ok(response)
}
