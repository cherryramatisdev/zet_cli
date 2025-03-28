use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Init => todo!(),
        cli::Commands::Titles => todo!(),
        cli::Commands::Grep { term } => todo!(),
        cli::Commands::Create => todo!(),
        cli::Commands::Config { command } => todo!(),
        cli::Commands::Now => todo!(),
        cli::Commands::Agenda => todo!(),
        cli::Commands::Import { path } => todo!(),
        _ => {
            // TODO: this will be the <query> part of the command
            todo!()
        }
    }
}
