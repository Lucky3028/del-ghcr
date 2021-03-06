use del_ghcr::commands;
use itertools::Itertools;
use seahorse::{App, Flag, FlagType};
use std::env;

fn main() {
    let args = env::args().collect_vec();
    let package_name = env!("CARGO_PKG_NAME");
    let app = App::new(package_name)
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [args]", package_name))
        .flag(Flag::new("token", FlagType::String).alias("t"))
        .flag(Flag::new("container", FlagType::String).alias("c"))
        .flag(Flag::new("dry-run", FlagType::Bool).alias("d"))
        .action(commands::delete_images);

    app.run(args);
}
