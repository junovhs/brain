// FILE: docs/scripts/src/main.rs

use anyhow::Result;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::PathBuf;

mod api_client; // <-- ADDED
mod context;
mod governor; // <-- ADDED
mod loader;
mod model;
mod next;
mod verifier;

#[derive(Parser, Debug)]
#[command(author, version, about = "The BRAIN Protocol Command-Line Interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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
    Configure, // <-- ADDED: For setting API keys
}

pub struct AppState {
    project_root: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let app_state = AppState::new()?;

    match cli.command {
        Some(command) => {
            let result = run_command(&app_state, command);
            if let Err(e) = result {
                eprintln!("\x1b[31mError: {:?}\x1b[0m", e);
                std::process::exit(1);
            }
        }
        None => {
            run_repl(&app_state)?;
        }
    }

    Ok(())
}

fn run_command(state: &AppState, command: Commands) -> Result<()> {
    match command {
        Commands::Context { task_id } => context::run(state, &task_id),
        Commands::Verify { task_id } => verifier::run(state, &task_id),
        Commands::Next => next::run(state),
        Commands::Configure => {
            println!("// TODO: Implement 'configure' command logic.");
            Ok(())
        }
        Commands::Prompt { role } => {
            println!("// TODO: Implement 'prompt' command logic for role: {}", role);
            Ok(())
        }
    }
}

fn run_repl(state: &AppState) -> Result<()> {
    println!("Welcome to the BRAIN interactive shell. Type 'exit' to quit.");
    let mut rl = DefaultEditor::new()?;

    loop {
        match next::get_next_tasks(state) {
            Ok(tasks) if !tasks.is_empty() => {
                println!("\n--- Task(s) In Progress ---");
                for task in tasks {
                    println!("- [{}] {}", task.id, task.label);
                }
                println!("\nCommands: [c]ontext <id>, [v]erify <id>, [conf]igure, [e]xit");
            }
            _ => {
                println!("\n--- No Active Task ---");
                println!("All tasks are completed or blocked.");
                println!("\nCommands: [N]ew Task (not implemented), [conf]igure, [e]xit");
            }
        }

        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.trim().eq_ignore_ascii_case("exit") {
                    break;
                }

                rl.add_history_entry(line.as_str())?;
                let args = shlex::split(&line).unwrap_or_default();
                if args.is_empty() {
                    continue;
                }

                let clap_args = ["brain-cli"].iter().map(|s| *s).chain(args.iter().map(|s| s.as_str()));

                match Cli::try_parse_from(clap_args) {
                    Ok(cli) => {
                        if let Some(command) = cli.command {
                            if let Err(e) = run_command(state, command) {
                                eprintln!("\x1b[31mError: {:?}\x1b[0m", e);
                            }
                        }
                    }
                    Err(e) => {
                        e.print()?;
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
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