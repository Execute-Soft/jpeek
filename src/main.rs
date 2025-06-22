mod app;
mod cli;
mod jwt;

use app::App;
use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    App::run(cli);
}
