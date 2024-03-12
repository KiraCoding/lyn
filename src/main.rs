pub mod app;
pub mod theme;
pub mod widget;

use app::App;
use clap::Parser;
use std::io::Result;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Cli {
    pub path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    App::new(cli.path)?.run()
}
