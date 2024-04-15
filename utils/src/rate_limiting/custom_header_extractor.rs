use tower_governor::GovernorError;
use axum::http::{ Request};
use tower_governor::key_extractor::KeyExtractor;
use crate::rate_limiting::custom_header::CUSTOM_HEADER;

#[derive(Clone)]
pub struct CustomHeaderExtractor;

impl KeyExtractor for CustomHeaderExtractor {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        let headers = req.headers();

        match headers.get(CUSTOM_HEADER) {
            Some(res) => {
                let res = res.to_str().map_err(|_| GovernorError::UnableToExtractKey)?;
                if res != "root" {
                    return Err(GovernorError::UnableToExtractKey);
                }
                Ok(res.to_owned())
            }
            None => Err(GovernorError::UnableToExtractKey)
        }
    }
}
