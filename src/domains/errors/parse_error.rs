use super::trait_error::TError;

#[derive(Debug)]
pub enum ParseError {
    Token,
    Container,
}

impl TError for ParseError {
    fn get_log_as_str(&self) -> String {
        format!("Arguments are missing: {:?}", self)
    }
}
