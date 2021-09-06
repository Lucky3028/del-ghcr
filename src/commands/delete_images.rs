use crate::domains::{ContextParser, GhcrClient, TError};
use prettytable::{cell, format, row, Table};
use seahorse::Context;

pub fn executor(context: &Context) {
    let args = match ContextParser::new(context) {
        Ok(res) => res,
        Err(err) => {
            err.log();
            return;
        }
    };
    let client = GhcrClient::new(args.token, args.container);
    let images = match client.fetch_images() {
        Ok(images) if images.is_empty() => {
            println!("There are no untagged images: {}", client.url);
            return;
        }
        Ok(images) => images,
        Err(err) => {
            err.log();
            return;
        }
    };
    if args.is_dry_run {
        let mut table = Table::new();
        table.set_titles(row![c => "id", "name", "created_at", "updated_at"]);
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        images.iter().for_each(|image| {
            table.add_row(row![
                image.id,
                image.name,
                image.created_at,
                image.updated_at
            ]);
        });
        table.printstd();
    } else {
        // FIXME: コメントアウトを解除
        // if let Err(err) = client.delete_image("a") {
        //     err.log();
        // };
    }
}
