mod args;
mod commands;
mod infra;
mod logging;
mod settings;

use anyhow::{Context, Result};
use args::{BadassArgs, Command};
use settings::Settings;

fn try_main() -> Result<()> {
    logging::configure_logging()?;

    let settings = Settings::new().with_context(|| "Failed to build settings")?;
    log::debug!("the settings are: {settings:?}");

    let args = BadassArgs::parse();
    log::debug!("the args are: {args:?}");

    match args.command {
        Command::Compile(compile_args) => commands::compile::do_compile(&settings, &compile_args),
        Command::Materialize(materialize_args) => {
            commands::materialize::do_materialize(&settings, &materialize_args)
        }
        Command::Show(show_args) => commands::show::do_show(&settings, &show_args),
        Command::Settings => commands::settings::do_show(),
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
