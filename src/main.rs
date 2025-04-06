use clap::{CommandFactory, Parser};
use cli::Cli;

use error_management::ResultExt;

mod cli;
mod cmds;
mod error_management;
mod repo_schema;
mod git_utils;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Init => cmds::init::call().unwrap_print(),
        cli::Commands::Titles => todo!(),
        cli::Commands::Grep { term } => todo!(),
        cli::Commands::Create => cmds::create::call().unwrap_print(),
        cli::Commands::Config { command } => todo!(),
        cli::Commands::Now => todo!(),
        cli::Commands::Agenda => todo!(),
        cli::Commands::Import { path } => todo!(),
        cli::Commands::Edit { term } => todo!(),
        cli::Commands::Sync => cmds::sync::call().unwrap_print(),
    }
}
