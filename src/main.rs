use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{console::Term, Confirm};
use std::{env, fs, path::Path};

#[derive(Parser)]
#[command(name = "filecooker", about = "CLI to safely create files with auto directories")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create one or more files
    Create {
        /// List of file paths to create
        #[arg(required = true)]
        files: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { files } => {
            for file in files {
                let path = Path::new(&file);

                // Create parent directories if needed
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        println!("{}", format!("Creating dir: {}", parent.display()).green());
                        if let Err(e) = fs::create_dir_all(parent) {
                            eprintln!("{} {}", "Failed to create dir:".red(), e);
                            continue;
                        }
                    }
                }

                // Handle existing file
                let should_write = if path.exists() {
                    match env::var("FORCE_YN").ok().as_deref() {
                        Some("y") => true,
                        Some("n") => false,
                        _ => {
                            Confirm::new()
                                .with_prompt(format!("{} exists. Overwrite?", file.yellow()))
                                .interact_on(&Term::stdout())
                                .unwrap()
                        }
                    }
                } else {
                    true
                };

                if !should_write {
                    println!("{}", "Skipped.".dimmed());
                    continue;
                }

                match fs::write(path, b"") {
                    Ok(_) => println!("{} {}", "Created:".green(), file),
                    Err(e) => eprintln!("{} {}", "Error creating file:".red(), e),
                }
            }
        }
    }
}
