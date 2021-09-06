use crate::domains::{ContextParser, GhcrClient, TError};
use seahorse::{Command, Context};

pub fn command() -> Command {
    Command::new("delete")
        .description("Delete all untagged images.")
        .alias("del")
        .usage("delete-ghcr delete --token [token] --container [container name]")
        .action(executor)
}

fn executor(context: &Context) {
    let client = match ContextParser::new(context) {
        Ok(res) => GhcrClient::new(res.token, res.container),
        Err(err) => {
            err.log();
            return;
        }
    };
    if let Err(err) = client.delete_images("a") {
        err.log();
    };
}
