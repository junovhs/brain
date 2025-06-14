// File: docs/cli/src/main.rs
use anyhow::{anyhow, Context, Result}; // Added Context
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env;
use std::fs; // Added for file system operations
use std::path::{Path, PathBuf};
use crate::db::Connection;

mod api_client;
mod conclude;
mod context;
mod db;
mod governor;
mod loader;
mod manifest;
mod model;
mod next;
mod reflect;
mod sketch;
mod utils;
mod verifier;
mod versioning;

#[derive(Parser, Debug)]
#[command(author, version, about = "The BRAIN Protocol Command-Line Interface", long_about = None)]
struct BrainCli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug, Clone)]
enum Commands {
    #[command(alias = "c")]
    Context { task_id: String },
    #[command(alias = "v")]
    Verify { task_id: String },
    #[command(alias = "n")]
    Next,
    #[command(alias = "r")]
    Reflect { task_id: String },
    #[command(alias = "d", visible_alias = "done")]
    Conclude { task_id: String },
    #[command(alias = "p")]
    Prompt { role: String },
    Configure,
}

#[derive(Parser, Debug)]
#[command(no_binary_name = true, about = "REPL commands")]
struct ReplCli {
    #[command(subcommand)]
    command: Commands,
}

pub struct AppState {
    project_root: PathBuf,
    db_conn: Connection,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = BrainCli::try_parse();
    let app_state = AppState::new()?;
    match cli {
        Ok(parsed_cli) => {
            if let Some(command) = parsed_cli.command {
                if let Err(e) = run_command(&app_state, command).await {
                    eprintln!("\x1b[31mError: {:?}\x1b[0m", e);
                    std::process::exit(1);
                }
            } else {
                run_repl(&app_state).await?;
            }
        }
        Err(_) => {
            run_repl(&app_state).await?;
        }
    }
    Ok(())
}

async fn run_command(state: &AppState, command: Commands) -> Result<()> {
    match command {
        Commands::Context { task_id } => context::run(state, &task_id),
        Commands::Verify { task_id } => verifier::run(state, &task_id),
        Commands::Next => next::run(state),
        Commands::Conclude { task_id } => conclude::run(state, &task_id),
        Commands::Reflect { task_id } => reflect::run(state, &task_id).await,
        Commands::Configure => {
            println!("// TODO: Implement 'configure' command logic.");
            Ok(())
        }
        // THE FIX: Implement the 'prompt' command logic
        Commands::Prompt { role } => {
            let prompt_file_name = format!("{}.md", role);
            let prompt_path = state.project_root.join("docs/prompts").join(prompt_file_name);

            let content = fs::read_to_string(&prompt_path)
                .with_context(|| format!("Failed to read prompt file at {:?}", prompt_path))?;
            
            println!("{}", content);
            Ok(())
        }
    }
}

fn display_status_bar() -> Result<()> {
    let budget_status = governor::get_budget_status();
    println!("\n--------------------------------------------------");
    println!("{}", budget_status);
    println!("--------------------------------------------------");
    Ok(())
}

async fn run_repl(state: &AppState) -> Result<()> {
    println!("Welcome to the BRAIN interactive shell. Type 'exit' to quit.");
    let mut rl = DefaultEditor::new()?;
    loop {
        display_status_bar()?;
        match next::get_next_tasks(state) {
            Ok(tasks) if !tasks.is_empty() => {
                println!("\n--- Task(s) In Progress ---");
                for task in tasks {
                    println!("- [{}] {}", task.id, task.label);
                }
                println!("\nCommands: context <id>, verify <id>, done <id>, reflect <id>, next, exit");
            }
            _ => {
                println!("\n--- No Active Task ---");
                println!("All tasks are completed or blocked.");
                println!("\nCommands: reflect <id>, new (not implemented), exit");
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
                if args.is_empty() { continue; }
                match ReplCli::try_parse_from(args) {
                    Ok(cli) => {
                        if let Err(e) = run_command(state, cli.command).await {
                            eprintln!("\x1b[31mError: {:?}\x1b[0m", e);
                        }
                    }
                    Err(e) => { e.print()?; }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
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
        let current_dir = env::current_dir()?;
        let project_root = find_project_root(&current_dir).ok_or_else(|| { 
            anyhow!("Cannot find project root containing BRAIN.md from the current directory.")
        })?;

        let conn = db::open_db_connection(project_root).map_err(|e| anyhow!("DB stub error: {}", e))?;
        db::initialize_database(&conn).map_err(|e| anyhow!("DB init stub error: {}", e))?;

        Ok(AppState {
            project_root: project_root.to_path_buf(),
            db_conn: conn,
        })
    }
}

fn find_project_root(start_dir: &Path) -> Option<&Path> {
    let mut current = start_dir;
    loop {
        if current.join("BRAIN.md").exists() {
            return Some(current);
        }
        current = current.parent()?;
    }
}