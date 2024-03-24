use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "badass", version)]
pub struct BadassArgs {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub command: Command,
}

impl BadassArgs {
    pub fn parse() -> Self {
        Parser::parse()
    }
}

// https://rust-cli-recommendations.sunshowers.io/handling-arguments.html

#[derive(Debug, Args)]
pub struct GlobalOpts {}

#[derive(Debug, Args)]
pub struct ShowArgs {
    pub model: String,
}

#[derive(Debug, Args)]
pub struct MaterializeArgs {
    pub model: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Render the sql templates
    Compile,
    /// Materialize the templates
    Materialize(MaterializeArgs),
    /// Show
    Show(ShowArgs),
    /// Settings
    Settings,
}
