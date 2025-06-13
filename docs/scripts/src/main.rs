use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod context;
mod loader; // New shared module
mod model;
mod verifier;
// mod next; // We will add this back when we implement the `next` task

#[derive(Parser, Debug)]
#[command(author, version, about="The BRAIN Protocol Command-Line Interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    #[command(alias = "c")]
    Context { task_id: String },
    #[command(alias = "v")]
    Verify { task_id: String },
    #[command(alias = "n")]
    Next,
    #[command(alias = "p")]
    Prompt { role: String },
}

pub struct AppState {
    project_root: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let app_state = match AppState::new() {
        Ok(state) => state,
        Err(e) => {
            eprintln!("\x1b[31mInitialization Error: {:?}\x1b[0m", e);
            std::process::exit(1);
        }
    };

    let result = match &cli.command {
        Commands::Context { task_id } => context::run(&app_state, task_id),
        Commands::Verify { task_id } => verifier::run(&app_state, task_id),
        // Placeholder for now
        Commands::Next => {
            println!("// TODO: Implement 'next' command logic.");
            Ok(())
        },
        Commands::Prompt { role } => {
            println!("// TODO: Implement 'prompt' command logic for role: {}", role);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("\x1b[31mError: {:?}\x1b[0m", e);
        std::process::exit(1);
    }
}

impl AppState {
    fn new() -> Result<Self> {
        let exe_path = std::env::current_exe()?;
        let scripts_dir = exe_path.parent().and_then(|p| p.parent()).and_then(|p| p.parent()).ok_or_else(|| anyhow::anyhow!("Could not determine scripts directory"))?;
        let project_root = scripts_dir.parent().and_then(|p| p.parent()).ok_or_else(|| anyhow::anyhow!("Could not determine project root"))?;
        Ok(AppState {
            project_root: project_root.to_path_buf(),
        })
    }
}