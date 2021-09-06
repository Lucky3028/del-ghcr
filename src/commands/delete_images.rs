use crate::domains::{ContextParser, GhcrClient, TError};
use itertools::Itertools;
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
        println!("These images below will be deleted by executing without `--dry-run` option.");
        let mut table = Table::new();
        table.set_titles(row!["id", "name", "created_at", "updated_at"]);
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
        // let results = images
        //     .iter()
        //     .map(|image| (image, client.delete_image(image.id)))
        //     .collect_vec();
        // let deleted = results
        //     .iter()
        //     .filter(|(_, res)| res.is_ok())
        //     .map(|(image, _)| image.id)
        //     .collect_vec();
        // if deleted.is_empty() {
        //     println!("There was no deleted image.");
        // } else {
        //     println!("The following ID images have been deleted.");
        //     deleted.iter().for_each(|id| println!("{}", id));
        // }
        // let not_deleted = results
        //     .into_iter()
        //     .filter(|(_, res)| res.is_err())
        //     .map(|(image, res)| (image.id, res.unwrap_err()))
        //     .collect_vec();
        // if not_deleted.is_empty() {
        //     println!("There was no image that was not deleted.");
        // } else {
        //     println!("The following ID images have not been deleted.");
        //     not_deleted
        //         .iter()
        //         .for_each(|(id, err)| {
        //             print!("{}: ", id);
        //             err.log();
        //         });
        // }
    }
}
