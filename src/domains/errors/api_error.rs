use super::trait_error::TError;

#[derive(Debug)]
pub enum ApiError {
    Ureq(Box<ureq::Error>),
    Json(Box<std::io::Error>),
}

impl TError for ApiError {
    fn log(&self) {
        match self {
            Self::Json(e) => eprintln!("Failed to parse to JSON: {}", e),
            Self::Ureq(e) => eprintln!("Failed to communicate with external API: {}", e),
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        Self::Json(Box::new(err))
    }
}

impl From<ureq::Error> for ApiError {
    fn from(err: ureq::Error) -> Self {
        Self::Ureq(Box::new(err))
    }
}
