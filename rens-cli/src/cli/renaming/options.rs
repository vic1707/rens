/* Modules */
mod confirmations;
mod git;
mod paths;
mod pattern;
mod recursion;
/* Built-in imports */
use std::{io, path::PathBuf};
/* Dependencies */
use clap::{Args, ValueHint};
use rens_common::RenameTarget;
/* Re-exports */
pub use self::{
    confirmations::{ConfirmOption, Confirmations, OverrideOption},
    git::Options as GitOpt,
    paths::Options as PathsOpt,
    pattern::Options as PatternOpt,
    recursion::Recursion,
};

#[derive(Debug, Args)]
#[command(next_display_order = 0)]
pub struct Options {
    /// Wether to rename the file stem, extension or both.
    ///
    /// Note: filename = <stem>.<extension>
    #[arg(long, short, default_value = "both", value_enum)]
    pub target: RenameTarget,

    /// Paths to the elements you want to rename.
    #[arg(
        required = true,
        value_parser = path_exists,
        value_hint = ValueHint::AnyPath,
    )]
    pub paths: Vec<PathBuf>,

    #[command(flatten)]
    pub confirmations: Confirmations,

    #[command(flatten)]
    pub git_opt: GitOpt,

    #[command(flatten)]
    pub paths_opt: PathsOpt,

    #[command(flatten)]
    pub recursion: Recursion,
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

    #[derive(Debug, Parser)]
    struct TestParser {
        #[command(flatten)]
        pub options: Options,
    }

    #[test]
    fn verify_conformity() {
        TestParser::command().debug_assert();
    }

    #[test]
    fn test_target() {
        TestParser::try_parse_from(["rens-cli", "--target", "."]).unwrap_err();

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--target=both", "."])
                .options
                .target,
            RenameTarget::Both
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--target=extension", "."])
                .options
                .target,
            RenameTarget::Extension
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--target=stem", "."])
                .options
                .target,
            RenameTarget::Stem
        );
    }
}
