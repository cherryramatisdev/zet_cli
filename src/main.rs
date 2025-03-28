use clap::Parser;
use cli::Cli;

use error_management::ResultExt;

mod cli;
mod cmds;
mod error_management;

fn main() {
    let cli = Cli::try_parse();

    match cli {
        Ok(c) => match c.command {
            cli::Commands::Init => cmds::init::call().unwrap_print(),
            cli::Commands::Titles => todo!(),
            cli::Commands::Grep { term } => todo!(),
            cli::Commands::Create => todo!(),
            cli::Commands::Config { command } => todo!(),
            cli::Commands::Now => todo!(),
            cli::Commands::Agenda => todo!(),
            cli::Commands::Import { path } => todo!(),
        },
        Err(_) => {
            // TODO: this will be the <query> part of the command
            todo!();
        }
    }
}
