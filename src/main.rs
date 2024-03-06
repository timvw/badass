mod compile;

use clap::{Args, Parser, Subcommand};

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
}

fn main() {
    let args = BadAssArgs::parse();
    match args.command {
        Command::Compile => compile::do_compile(),
    }
}
