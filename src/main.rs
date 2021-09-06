use itertools::Itertools;
use ls_ghcr::commands;
use seahorse::{App, Command};
use std::env;

fn main() {
    let args = env::args().collect_vec();
    let package_name = env!("CARGO_PKG_NAME");
    let app = App::new(package_name)
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [args]", package_name))
        .command(commands::list_images())
        .command(commands::delete_images())
        .command(
            Command::new("version")
                .alias("-v")
                .description("Show version.")
                .action(|_| println!("version: {}", env!("CARGO_PKG_VERSION"))),
        );

    app.run(args);
}
