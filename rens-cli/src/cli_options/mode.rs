/* Built-in imports */
use core::num::NonZeroUsize;
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
            } => Self::new(
                pattern,
                with,
                options.occurence.map_or(0, usize::from),
            ),
            Mode::String {
                pattern,
                with,
                options,
            } => Self::new(
                // safety guarenteed by [`regex::escape`]
                #[allow(clippy::expect_used)]
                Regex::new(&regex::escape(&pattern))
                    .expect("Unable to build regex."),
                with,
                options.occurence.map_or(0, usize::from),
            ),
        }
    }
}

#[derive(Debug, Args)]
pub struct PatternOptions {
    /// Weather or not the pattern should be made case sensitive.
    ///
    /// Note: No effect if regex pattern already includes case settings at its beggining.
    #[arg(long, default_value_t = false)]
    pub case_insensitive: bool,

    /// Number of replacements to be done.
    #[arg(global = true, long, short, value_name = "number of repetitions")]
    // Note: None gets used as `All`.
    pub occurence: Option<NonZeroUsize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};

    #[derive(Debug, Parser)]
    struct TestParser {
        #[command(flatten)]
        pub options: PatternOptions,
    }

    #[test]
    fn pattern_options_are_valid() {
        TestParser::command().debug_assert();
    }

    #[test]
    fn test_default_values() {
        let args = TestParser::parse_from::<[_; 0], &str>([]);
        assert!(!args.options.case_insensitive);
        assert_eq!(args.options.occurence, None);
    }

    #[test]
    fn test_case_sensitive() {
        assert!(
            TestParser::parse_from(["rens-cli", "--case-insensitive"])
                .options
                .case_insensitive
        );
    }

    #[test]
    fn test_occurence() {
        // should fail if no value provided
        TestParser::try_parse_from(["rens-cli", "-o"]).unwrap_err();
        // should fail if 0 is provided
        TestParser::try_parse_from(["rens-cli", "-o", "0"]).unwrap_err();

        assert_eq!(
            TestParser::parse_from(["rens-cli", "-o", "5"])
                .options
                .occurence,
            Some(NonZeroUsize::try_from(5).unwrap())
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--occurence", "5"])
                .options
                .occurence,
            Some(NonZeroUsize::try_from(5).unwrap())
        );
    }
}
