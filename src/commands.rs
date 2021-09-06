use seahorse::{Command, Flag, FlagType};

mod delete_images;
mod list_images;

fn add_flags(cmd: Command) -> Command {
    cmd.flag(Flag::new("token", FlagType::String).alias("t"))
        .flag(Flag::new("container", FlagType::String).alias("c"))
        .flag(Flag::new("dry-run", FlagType::Bool).alias("d"))
}

pub fn delete_images() -> Command {
    add_flags(delete_images::command())
}

pub fn list_images() -> Command {
    add_flags(list_images::command())
}
