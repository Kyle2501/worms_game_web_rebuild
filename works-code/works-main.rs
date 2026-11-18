use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "works", version = "0.1.0", about = "Works Game Build & Environment CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bundle game assets and shaders for release
    Bundle {
        #[arg(short, long, default_value = "./assets")]
        input: String,
        #[arg(short, long, default_value = "./dist")]
        output: String,
    },
    /// Validate the Works environment and dependencies
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bundle { input, output } => {
            println!("Building Works package from {} to {}...", input, output);
            // TODO: Integrate rayon parallel asset processing here
        }
        Commands::Doctor => {
            println!("Checking Works environment dependencies...");
            // TODO: Validate shader compilers, SDK paths, etc.
        }
    }
}
