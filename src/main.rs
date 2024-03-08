mod args;
mod compile;
mod infra;
mod logging;
mod materialize;
mod settings;
mod show;

use anyhow::Result;
use args::{BadassArgs, Command};
use settings::Settings;

fn try_main() -> Result<()> {
    logging::configure_logging()?;

    let settings = Settings::new()?;
    log::debug!("the settings are: {settings:?}");

    let args = BadassArgs::parse();
    log::debug!("the args are: {args:?}");

    match args.command {
        Command::Compile => compile::do_compile(&settings),
        Command::Materialize => materialize::do_materialize(&settings),
        Command::Show(show_args) => show::do_show(&settings, &show_args),
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
