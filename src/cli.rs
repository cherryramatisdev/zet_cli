use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zet", version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the repo in the current empty directory
    #[command(visible_alias("i"))]
    Init,
    /// Print all the titles from the entries (the tags are presented alongside the title, for easy
    /// filtering in notes like journaling or other tag)
    #[command(visible_alias("t"))]
    Titles,
    /// Open a entry by a substring of the title.
    #[command(visible_alias("e"))]
    Edit {
        term: String,
    },
    #[command(visible_alias("g"))]
    /// Search for particular substring or regex pattern within your entries content 
    Grep {
        term: String,
    },
    /// Create a new entry, register on the index file and open your `$EDITOR` on the particular
    /// file.
    #[command(visible_alias("c"))]
    Create,
    /// Sub commands related to the config
    #[command(visible_alias("conf"))]
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Create a new entry with the title predefined for the current date and a special tag for
    /// journaling, that way it can be filtered differently from the rest of the notes
    #[command(visible_alias("n"))]
    Now,
    /// List all the checkboxes `- [ ]` in your repository and print out a list of unfinished todos
    /// with optional schedules (with the syntax `@schedule <date>`)
    #[command(visible_alias("a"))]
    Agenda,
    /// Import a directory with a *particular structure* into our another repo
    #[command(visible_alias("im"))]
    Import {
        path: String,
    },
    #[command(visible_alias("s"))]
    Sync,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Print all the values currently set for the config (including defaults, the default should
    /// contain a label expliciting its default value)
    #[command(visible_alias("p"))]
    Print,
    /// Change or create a particular key on the config, errors out if the key doesn't exist on the
    /// spec
    #[command(visible_alias("m"))]
    Modify { key: String, value: String },
    /// Print the value for a particular config key, errors out if the key doens't exist on the
    /// config file
    #[command(visible_alias("g"))]
    Get { key: String },
}
