use anyhow::Result;
use colored::Colorize;

pub fn err_print(msg: String) {
    println!("{}", msg.red().bold());
}

pub trait ResultExt<T> {
    fn unwrap_print(&self);
}

// Implement the trait for anyhow::Result<T>
impl<T> ResultExt<T> for Result<T> {
    fn unwrap_print(&self) {
        if let Err(msg) = self {
            err_print(msg.to_string());
            std::process::exit(1);
        }
    }
}
