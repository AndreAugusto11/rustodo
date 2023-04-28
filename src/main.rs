#[macro_use]
extern crate diesel;
extern crate dotenv;

mod args;
mod ops;
mod db;
mod schema;
mod models;

use ops::user_ops::handle_user_command;
use ops::todo_ops::handle_todo_command;
use args::EntityType;
use args::RustodoArgs;
use clap::Parser;

fn main() {
    let args = RustodoArgs::parse();

    match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
        EntityType::Todo(video) => handle_todo_command(video),
    };
}