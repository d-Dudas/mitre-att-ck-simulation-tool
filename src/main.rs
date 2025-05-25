mod app;
mod simulations;
mod utils;

use clap::Parser;
use app::app::App;
use utils::args::Args;

fn main() {
    let args = Args::parse();

    let mut app = App::new(args);
    app.run();
}
