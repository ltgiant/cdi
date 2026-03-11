use cdi::{get_subdir, list_subdirs};
use clap::{Parser, Subcommand, ValueEnum};
use std::env;
use std::process;

#[derive(Parser)]
#[command(
    version,
    about = "Quick directory navigation with numeric indices",
    long_about = "Quick directory navigation with numeric indices.\n\nRun without arguments to list subdirectories with indices.\nRun with an index to navigate to that directory.\n\nExample:\n  cdi        # list subdirectories\n  cdi 2      # navigate to index 2\n\nSetup:\n  eval \"$(cdi init zsh)\"    # add to ~/.zshrc"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Index of subdirectory to navigate to (0-based)
    index: Option<usize>,
}

#[derive(Subcommand)]
enum Command {
    /// Print shell integration code
    Init {
        /// Shell type
        shell: Shell,
    },
}

#[derive(Clone, ValueEnum)]
enum Shell {
    Zsh,
    Bash,
    Fish,
    Powershell,
}

const SHELL_ZSH: &str = r#"cdi() {
  if [ $# -eq 0 ]; then
    command cdi
  elif [[ "$1" == -* ]]; then
    command cdi "$@"
  else
    local target
    target=$(command cdi "$1") && cd "$target"
  fi
}
"#;

const SHELL_BASH: &str = r#"cdi() {
  if [ $# -eq 0 ]; then
    command cdi
  elif [[ "$1" == -* ]]; then
    command cdi "$@"
  else
    local target
    target=$(command cdi "$1") && cd "$target"
  fi
}
"#;

const SHELL_FISH: &str = r#"function cdi
  if test (count $argv) -eq 0
    command cdi
  else if string match -q -- '-*' $argv[1]
    command cdi $argv
  else
    set -l target (command cdi $argv[1])
    and cd $target
  end
end
"#;

const SHELL_POWERSHELL: &str = r#"function cdi {
  if ($args.Count -eq 0) {
    & cdi.exe
  } elseif ($args[0] -like '-*') {
    & cdi.exe @args
  } else {
    $target = & cdi.exe $args[0]
    if ($LASTEXITCODE -eq 0) { Set-Location $target }
  }
}
"#;

fn main() {
    let cli = Cli::parse();
    let current_dir = env::current_dir().unwrap_or_else(|e| {
        eprintln!("cdi: {}", e);
        process::exit(1);
    });

    if let Some(Command::Init { shell }) = cli.command {
        match shell {
            Shell::Zsh => print!("{}", SHELL_ZSH),
            Shell::Bash => print!("{}", SHELL_BASH),
            Shell::Fish => print!("{}", SHELL_FISH),
            Shell::Powershell => print!("{}", SHELL_POWERSHELL),
        }
        return;
    }

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
