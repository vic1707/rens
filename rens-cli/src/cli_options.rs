/* Modules */
mod confirmations;
mod mode;
mod recursion;
pub mod utils;
/* Built-in imports */
use std::{io, path::PathBuf};
/* Dependencies */
use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    ArgAction, Parser, ValueHint,
};
use clap_verbosity_flag::Verbosity;
use rens_common::RenameTarget;
/* Re-exports */
pub use self::{
    confirmations::{ConfirmOption, Confirmations, OverrideOption},
    mode::Mode,
    recursion::Recursion,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliOptions {
    #[command(subcommand)]
    pub mode: Mode,

    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(flatten)]
    pub confirmations: Confirmations,

    #[command(flatten)]
    pub recursion: Recursion,

    /// Canonicalize all paths instead of using relative ones.
    #[arg(
        global = true,
        long, short,
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    pub canonicalize: bool,

    /// Weather to rename the file stem, extension or both.
    #[arg(
        global = true,
        long, short,
        default_value = "both",
        value_parser = PossibleValuesParser::new(["stem", "extension", "both"])
            .map(|s| s.parse::<RenameTarget>().unwrap())
    )]
    pub target: RenameTarget,

    /// Paths to the elements you want to rename.
    #[arg(
        global = true,
        value_parser = path_exists,
        value_hint = ValueHint::AnyPath,
    )]
    pub paths: Vec<PathBuf>,
}

fn path_exists(input: &str) -> io::Result<PathBuf> {
    let path: PathBuf = input.into();
    if path.exists() {
        // simply ensure the path is canonicalizable
        dunce::canonicalize(&path)?;
        Ok(path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No such file or directory: '{input}'."),
        ))
    }
}
