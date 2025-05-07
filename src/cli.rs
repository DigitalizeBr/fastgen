use clap::{Parser, Subcommand};
use crate::generator::project::generate_project;
use crate::plugin::apply_plugin;
use crate::workspace::init::create_workspace;
use crate::workspace::service::add_service;
use crate::workspace::external::add_external_service;
use crate::config::Config;

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
    },
    AddExt {
        #[arg(short, long)]
        name: String,
        #[arg(long)]
        to: String,
    },
}

pub fn run(config: Config) {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate { blueprint } => generate_project(&blueprint),
        Commands::Plugin { name, project } => apply_plugin(&name, &project, &config),
        Commands::NewWorkspace { name } => create_workspace(&name),
        Commands::AddService { name, to } => add_service(&name, &to),
        Commands::AddExt { name, to } => add_external_service(&name, &to),
    }
}