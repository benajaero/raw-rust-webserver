// Credit: Ben Ajaero

use clap::{Parser, Subcommand};

mod scaffold;

#[derive(Parser)]
#[command(name = "raw", version, about = "CLI tools for the Raw web framework")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new Raw application
    New { name: String },
    /// Print registered routes (coming soon)
    Routes,
    /// Run the development server (coming soon)
    Run,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => match scaffold::create_project(&name) {
            Ok(()) => println!("Created Raw project: {}", name),
            Err(err) => {
                eprintln!("Failed to create project: {}", err);
                std::process::exit(1);
            }
        },
        Commands::Routes => {
            eprintln!("raw routes is not implemented yet.");
        }
        Commands::Run => {
            eprintln!("raw run is not implemented yet.");
        }
    }
}
