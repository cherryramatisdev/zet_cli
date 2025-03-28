use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the repo in the current empty directory
    Init,
    /// Print all the titles from the entries (the tags are presented alongside the title, for easy
    /// filtering in notes like journaling or other tag)
    Titles,
    Edit {
        term: String,
    },
    /// Search for particular substring or regex pattern within your entries content 
    Grep {
        term: String,
    },
    /// Create a new entry, register on the index file and open your `$EDITOR` on the particular
    /// file.
    Create,
    /// Sub commands related to the config
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Create a new entry with the title predefined for the current date and a special tag for
    /// journaling, that way it can be filtered differently from the rest of the notes
    Now,
    /// List all the checkboxes `- [ ]` in your repository and print out a list of unfinished todos
    /// with optional schedules (with the syntax `@schedule <date>`)
    Agenda,
    /// Import a directory with a *particular structure* into our another repo
    Import {
        path: String,
    }
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Print all the values currently set for the config (including defaults, the default should
    /// contain a label expliciting its default value)
    Print,
    /// Change or create a particular key on the config, errors out if the key doesn't exist on the
    /// spec
    Modify { key: String, value: String },
    /// Print the value for a particular config key, errors out if the key doens't exist on the
    /// config file
    Get { key: String },
}
