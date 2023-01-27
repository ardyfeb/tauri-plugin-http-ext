use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Method(#[from] http::method::InvalidMethod),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error(transparent)]
    Http(#[from] http::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
