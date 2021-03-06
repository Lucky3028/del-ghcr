use super::errors::ParseError;
use seahorse::Context;
use std::env;

pub struct ContextParser {
    pub token: String,
    pub container: String,
    pub is_dry_running: bool,
    pub is_forced: bool,
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

        Ok(Self {
            token,
            container,
            is_dry_running: context.bool_flag("dry-run"),
            is_forced: context.bool_flag("force"),
        })
    }
}
