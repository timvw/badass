mod compile;
mod core;
mod materialize;
mod settings;

use crate::settings::Settings;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::Path;

// https://rust-cli-recommendations.sunshowers.io/handling-arguments.html

#[derive(Debug, Parser)]
#[clap(name = "badass", version)]
pub struct BadAssArgs {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Args)]
struct GlobalOpts {}

#[derive(Debug, Subcommand)]
enum Command {
    /// Render the sql templates
    Compile,
    /// Materialize the templates
    Materialize,
}

fn try_main() -> Result<()> {
    //let settings = Settings::default();
    let settings = Settings::from_path(Path::new("./demo"));
    let args = BadAssArgs::parse();
    match args.command {
        Command::Compile => compile::do_compile(&settings),
        Command::Materialize => materialize::do_materialize(&settings),
    }
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("ERROR: {}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }
}
