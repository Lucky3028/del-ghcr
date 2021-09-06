use crate::domains::{ContextParser, GhcrClient, TError};
use prettytable::{cell, format, row, table};
use seahorse::{Command, Context, Flag, FlagType};

pub fn command() -> Command {
    Command::new("list")
        .description("Show the list of untagged images.")
        .alias("ls")
        .flag(Flag::new("token", FlagType::String).alias("t"))
        .usage("del-ghcr list --token [token] --container [container name]")
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
    match client.fetch_images() {
        Ok(images) if images.is_empty() => println!("There are no untagged images: {}", client.url),
        Ok(images) => {
            let mut table = table!(["id", "name", "created_at", "updated_at"]);
            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
            images.iter().for_each(|image| {
                table.add_row(row!(
                    image.id,
                    image.name,
                    image.created_at,
                    image.updated_at
                ));
            });
            table.printstd();
        }
        Err(err) => err.log(),
    }
}
