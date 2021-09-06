use super::trait_error::TError;

#[derive(Debug)]
pub enum ParseError {
    Token,
    Container,
}

impl TError for ParseError {
    fn log(&self) {
        eprintln!("Arguments are missing: {:?}", self);
    }
}
