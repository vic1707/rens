/* Crate imports */
use super::utils::GlobalOrNumbered;
/* Dependencies */
use clap::{Args, Subcommand};
use regex::Regex;
use rens_common::Strategy;

#[derive(Debug, Subcommand)]
pub enum Mode {
    /// Perform renaming from a string pattern.
    #[command(alias = "str")]
    String {
        /// The pattern you're looking to rename.
        pattern: String,
        /// The string you with to replace it with.
        with: String,
        #[command(flatten)]
        options: PatternOptions,
    },
    /// Perform renaming from a regex pattern.
    #[command(alias = "re")]
    Regex {
        /// The regex pattern you're looking to rename.
        pattern: Regex,
        /// The string you with to replace it with.
        with: String,
        #[command(flatten)]
        options: PatternOptions,
    },
}

impl From<Mode> for Strategy {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Regex {
                pattern,
                with,
                options,
            } => Self::new(pattern, with, options.occurence.into()),
            Mode::String {
                pattern,
                with,
                options,
            } => Self::new(
                // safety guarenteed by [`regex::escape`]
                #[allow(clippy::expect_used)]
                Regex::new(&regex::escape(&pattern)).expect("Unable to build regex."),
                with,
                options.occurence.into(),
            ),
        }
    }
}

#[derive(Debug, Args)]
pub struct PatternOptions {
    /// Weather or not the pattern should be made case sensitive.
    ///
    /// Note: No effect if regex pattern already includes case settings at its beggining.
    #[arg(long, short, default_value_t = true)]
    pub case_sensitive: bool,

    /// Number of replacements to be done.
    #[arg(
        global = true,
        long, short,
        value_enum,
        default_value_t = GlobalOrNumbered::Global,
    )]
    pub occurence: GlobalOrNumbered,
}