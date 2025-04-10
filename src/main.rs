use clap::Parser;
use cli::{Cli, Commands};
use strum::IntoEnumIterator;

use error_management::ResultExt;

mod cli;
mod cmds;
mod error_management;
mod git_utils;
mod repo_schema;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmds = Commands::iter()
        .map(|v| v.to_string().to_lowercase())
        .collect::<Vec<String>>();

    if args.len() > 1 {
        let cmd = &args[1];

        if !cmds.contains(cmd) {
            return cmds::edit::call(cmd.to_string()).unwrap_print();
        }
    }

    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Init => cmds::init::call().unwrap_print(),
        cli::Commands::Titles => cmds::titles::call().unwrap_print(),
        cli::Commands::Grep { term } => cmds::grep::call(term).unwrap_print(),
        cli::Commands::Create => cmds::create::call().unwrap_print(),
        cli::Commands::Config { command } => todo!(),
        cli::Commands::Now => cmds::now::call().unwrap_print(),
        cli::Commands::Agenda => todo!(),
        cli::Commands::Import { path } => cmds::import::call(path).unwrap_print(),
        cli::Commands::Edit { term } => cmds::edit::call(term).unwrap_print(),
        cli::Commands::Sync => cmds::sync::call().unwrap_print(),
        cli::Commands::View { term } => cmds::view::call(term).unwrap_print(),
    }
}
