use crate::domains::{ContextParser, GhcrClient, TError};
use prettytable::{cell, format, row, Table};
use question::{Answer, Question};
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
    if args.is_dry_running {
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
        if !args.is_forced {
            let ans = Question::new("Do you want to delete all untagged images?")
                .yes_no()
                .tries(1)
                .default(Answer::NO)
                .show_defaults()
                .ask();
            if ans != Some(Answer::YES) {
                println!("The operation was cancelled.");
                return;
            };
        }
        println!("Deleteing all untagged images...");
        images
            .iter()
            .map(|image| (image.id, client.delete_image(image.id)))
            .for_each(|(id, result)| {
                let msg = match result {
                    Ok(_) => "Succeeded",
                    Err(err) => format!("Failed: {}", err.get_log_as_str()),
                };
                println!("{}: {}", id, msg);
            });
    }
}
