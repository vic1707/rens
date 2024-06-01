/* Modules */
pub mod renaming;
/* Crate imports */
use renaming::Mode;
/* Dependencies */
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(flatten)]
    Renaming(Mode),
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_conformity() {
        Cli::command().debug_assert();
    }
}
