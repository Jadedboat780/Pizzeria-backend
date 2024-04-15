use axum::response::IntoResponse;
use axum_extra::{
    TypedHeader,
    headers::{Header, HeaderName, HeaderValue},
};

static X: HeaderName = HeaderName::from_static("x-custom-key");
pub static CUSTOM_HEADER: &HeaderName = &X;

pub struct CustomHeader(String);

impl CustomHeader {
    pub fn key(self) -> String {
        self.0
    }
}

impl Header for CustomHeader {
    fn name() -> &'static HeaderName {
        CUSTOM_HEADER
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum_extra::headers::Error>
        where I: Iterator<Item=&'i HeaderValue>
    {
        let value = values
            .next()
            .ok_or_else(axum_extra::headers::Error::invalid)?;

        Ok(CustomHeader(value.to_str().unwrap().to_owned()))
    }

    fn encode<E>(&self, values: &mut E)
        where E: Extend<HeaderValue>
    {
        let s = &self.0;
        let value = HeaderValue::from_str(s).unwrap();
        values.extend(std::iter::once(value));
    }
}

async fn register(
    TypedHeader(_header): TypedHeader<CustomHeader>,
) -> impl IntoResponse {
    todo!()
}