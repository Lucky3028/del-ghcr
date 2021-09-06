use super::errors::ParseError;
use seahorse::Context;
use std::env;

pub struct ContextParser {
    pub token: String,
    pub container: String,
}

impl ContextParser {
    pub fn new(context: &Context) -> Result<Self, ParseError> {
        let token = if let Ok(token) = context
            .string_flag("token")
            .or_else(|_| env::var("GHCR_TOKEN"))
        {
            token
        } else {
            return Err(ParseError::Token);
        };
        let container = if let Ok(container) = context.string_flag("container") {
            container
        } else {
            return Err(ParseError::Container);
        };

        Ok(Self { token, container })
    }
}
