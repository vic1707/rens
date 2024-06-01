/* Modules */
mod confirmations;
mod paths;
mod pattern;
mod recursion;
/* Built-in imports */
use std::{io, path::PathBuf};
/* Dependencies */
use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    Args, ValueHint,
};
use rens_common::RenameTarget;
/* Re-exports */
pub use self::{
    confirmations::{ConfirmOption, Confirmations, OverrideOption},
    paths::Options as PathsOpt,
    pattern::Options as PattenrOpt,
    recursion::Recursion,
};

#[derive(Debug, Args)]
pub struct Options {
    /// Weather to rename the file stem, extension or both.
    #[arg(
        global = true,
        long, short,
        default_value = "both",
        value_parser = PossibleValuesParser::new(["stem", "extension", "both"])
            .map(|s| s.parse::<RenameTarget>().unwrap())
    )]
    pub target: RenameTarget,

    #[command(flatten)]
    pub recursion: Recursion,

    #[command(flatten)]
    pub confirmations: Confirmations,

    #[command(flatten)]
    pub paths_opt: PathsOpt,

    #[command(flatten)]
    pub pattern_opt: PattenrOpt,

    /// Paths to the elements you want to rename.
    #[arg(
        required = true,
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};

    #[derive(Parser)]
    struct TestParser {
        #[command(flatten)]
        pub options: Options,
    }

    #[test]
    fn verify_conformity() {
        TestParser::command().debug_assert();
    }
}
