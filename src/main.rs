mod cli;
mod generator;
mod plugin;
mod utils;
mod workspace;
mod config;
mod ai;


fn main() {
    let config = config::Config::load();

    cli::run(config);
}
