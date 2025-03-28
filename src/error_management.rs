use anyhow::Result;
use colored::Colorize;

pub trait ResultExt<T> {
    fn unwrap_print(&self);
}

// Implement the trait for anyhow::Result<T>
impl<T> ResultExt<T> for Result<T> {
    fn unwrap_print(&self) {
        if let Err(msg) = self {
            println!("{}", msg.to_string().red().bold());
            std::process::exit(1);
        }
    }
}
