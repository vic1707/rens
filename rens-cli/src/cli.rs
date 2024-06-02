/* Modules */
pub mod renaming;
/* Built-in imports */
use std::path::PathBuf;
/* Crate imports */
use renaming::Mode;
/* Dependencies */
use clap::{Parser, Subcommand, ValueHint};
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
    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        shell: clap_complete_command::Shell,
    },
    /// Generate man page
    Man {
        /// The dir path to generate man-pages to.
        ///
        /// Note: Will get created if doesn't exist.
        #[arg(value_hint = ValueHint::DirPath)]
        path: PathBuf,
    },
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
