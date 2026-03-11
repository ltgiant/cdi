use cdi::{get_subdir, list_subdirs};
use clap::Parser;
use std::env;
use std::process;

#[derive(Parser)]
#[command(
    version,
    about = "Quick directory navigation with numeric indices",
    long_about = "Quick directory navigation with numeric indices.\n\nRun without arguments to list subdirectories with indices.\nRun with an index to navigate to that directory.\n\nExample:\n  cdi        # list subdirectories\n  cdi 2      # navigate to index 2"
)]
struct Cli {
    /// Index of subdirectory to navigate to (0-based)
    index: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let current_dir = env::current_dir().unwrap_or_else(|e| {
        eprintln!("cdi: {}", e);
        process::exit(1);
    });

    match cli.index {
        None => {
            let dirs = list_subdirs(&current_dir).unwrap_or_else(|e| {
                eprintln!("cdi: {}", e);
                process::exit(1);
            });
            if dirs.is_empty() {
                eprintln!("cdi: no subdirectories found");
                process::exit(1);
            }
            for (i, name) in dirs.iter().enumerate() {
                println!("  {}) {}", i, name);
            }
        }
        Some(index) => {
            let target = get_subdir(&current_dir, index).unwrap_or_else(|e| {
                eprintln!("cdi: {}", e);
                process::exit(1);
            });
            println!("{}", target.display());
        }
    }
}
