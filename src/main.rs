use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::Confirm;
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "filemaker", about = "CLI to safely create files with auto directories")]
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

                // Create parent directories if missing
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        println!("{}", format!("Creating dir: {}", parent.display()).green());
                        if let Err(e) = fs::create_dir_all(parent) {
                            eprintln!("{} {}", "Failed to create dir:".red(), e);
                            continue;
                        }
                    }
                }

                // Handle file creation
                if path.exists() {
                    let overwrite = Confirm::new()
                        .with_prompt(format!(
                            "{} exists. Overwrite?",
                            file.yellow()
                        ))
                        .interact()
                        .unwrap();

                    if !overwrite {
                        println!("{}", "Skipped.".dimmed());
                        continue;
                    }
                }

                match fs::write(path, b"") {
                    Ok(_) => println!("{} {}", "Created:".green(), file),
                    Err(e) => eprintln!("{} {}", "Error creating file:".red(), e),
                }
            }
        }
    }
}

