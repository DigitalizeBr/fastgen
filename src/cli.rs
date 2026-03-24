use clap::{Parser, Subcommand};
use crate::generator::project::generate_project;
use crate::plugin::apply_plugin;
use crate::workspace::init::create_workspace;
use crate::workspace::service::add_service;
use crate::workspace::external::add_external_service;
use crate::workspace::devui::start_dev_ui;
use crate::config::Config;
use crate::ai::cli::run_ai_generation;

#[derive(Parser)]
#[command(name = "fastgen", version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long)]
        blueprint: String,
    },
    Plugin {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        project: String,
    },
    NewWorkspace {
        #[arg(short, long)]
        name: String,
    },
    AddService {
        #[arg(short, long)]
        name: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        ai: bool,
    },
    AddExt {
        #[arg(short, long)]
        name: String,
        #[arg(long)]
        to: String,
    },
    DevUi {
        #[arg(long)]
        repo: String,
        #[arg(long)]
        ai_path: Option<String>,
    },
    AiGenerate {
        #[arg(short, long)]
        path: String,
    },
}

pub fn run(config: Config) {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate { blueprint } => generate_project(&blueprint),
        Commands::Plugin { name, project } => apply_plugin(&name, &project, &config),
        Commands::NewWorkspace { name } => create_workspace(&name),
        Commands::AddService { name, to, ai } => add_service(&name, &to, ai),
        Commands::AddExt { name, to } => add_external_service(&name, &to),
        Commands::DevUi { repo, ai_path } => start_dev_ui(&repo, ai_path.as_deref(), &config),
        Commands::AiGenerate { path } => run_ai_generation(&path, &config),
    }
}
