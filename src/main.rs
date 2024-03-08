mod args;
mod compile;
mod infra;
mod materialize;
mod settings;

use anyhow::Result;
use args::{BadassArgs, Command};
use settings::Settings;

fn try_main() -> Result<()> {
    let settings = Settings::default();
    let args = BadassArgs::parse();
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
