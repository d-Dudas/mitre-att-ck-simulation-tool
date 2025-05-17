use clap::Parser;

mod app;
mod args;
mod simulations;
mod utils;

use app::app::App;
use args::Args;

fn main() {
    let args = Args::parse();

    let app = App::new();
    app.run(args);
}
