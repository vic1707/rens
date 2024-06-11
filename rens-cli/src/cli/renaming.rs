/* Modules */
pub mod options;
/* Crate imports */
use self::options::Options;
/* Dependencies */
use clap::Subcommand;
use regex::{Regex, RegexBuilder};
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
        options: Options,
    },
    /// Perform renaming from a regex pattern.
    #[command(alias = "re")]
    Regex {
        /// The regex pattern you're looking to rename.
        ///
        /// Must comply with the `regex` crate syntax.
        pattern: Regex,
        /// The string you with to replace it with.
        with: String,
        #[command(flatten)]
        options: Options,
    },
}

impl Mode {
    pub fn get_strategy_and_options(self) -> (Strategy, Options) {
        match self {
            Self::Regex {
                mut pattern,
                with,
                options,
            } => {
                let limit =
                    options.pattern_opt.occurence.map_or(0, usize::from);
                if options.pattern_opt.case_insensitive {
                    pattern = to_regex_case_insensitive(&pattern);
                }

                (Strategy::new(pattern, with, limit), options)
            },
            Self::String {
                pattern,
                with,
                options,
            } => {
                let limit =
                    options.pattern_opt.occurence.map_or(0, usize::from);
                // safety guarenteed by [`regex::escape`]
                #[allow(clippy::expect_used)]
                let mut regex_pattern = Regex::new(&regex::escape(&pattern))
                    .expect("Unable to build regex.");

                if options.pattern_opt.case_insensitive {
                    regex_pattern = to_regex_case_insensitive(&regex_pattern);
                }

                (Strategy::new(regex_pattern, with, limit), options)
            },
        }
    }
}

#[allow(clippy::expect_used)]
fn to_regex_case_insensitive(regex: &Regex) -> Regex {
    RegexBuilder::new(regex.as_str())
        .case_insensitive(true)
        .build()
        .expect("Failed to build case insensitive Regex.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};

    #[derive(Debug, Parser)]
    struct TestParser {
        #[command(subcommand)]
        pub mode: Mode,
    }

    #[test]
    fn pattern_options_are_valid() {
        TestParser::command().debug_assert();
    }
}
