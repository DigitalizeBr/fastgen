mod cli;
mod generator;
mod plugin;
mod utils;
mod workspace;
mod config;


fn main() {
    let config = config::Config::load();

    cli::run(config);
}