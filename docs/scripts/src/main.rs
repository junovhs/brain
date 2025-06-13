use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod context;
mod model;
mod verifier;

#[derive(Parser, Debug)]
#[command(author, version, about="The BRAIN Protocol Interactive Shell", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Builds and prints the LLM context for a specific task.
    Context { task_id: String },
    /// Verifies a specific task against its acceptance criteria.
    Verify { task_id: String },
    /// Lists tasks that can be worked on.
    Next,
}

/// Contains shared state like the project root path.
pub struct AppState {
    project_root: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Establish AppState once.
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
        Commands::Next => {
            println!("// TODO: Implement 'next' command logic.");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("\x1b[31mError: {:?}\x1b[0m", e);
        std::process::exit(1);
    }
}

impl AppState {
    /// Creates a new AppState, determining the project root.
    fn new() -> Result<Self> {
        let exe_path = std::env::current_exe()?;
        let scripts_dir = exe_path.parent().and_then(|p| p.parent()).and_then(|p| p.parent()).ok_or_else(|| anyhow::anyhow!("Could not determine scripts directory from executable path."))?;
        let project_root = scripts_dir.parent().and_then(|p| p.parent()).ok_or_else(|| anyhow::anyhow!("Could not determine project root from scripts directory."))?;
        Ok(AppState {
            project_root: project_root.to_path_buf(),
        })
    }
}