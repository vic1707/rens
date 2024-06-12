/* Modules */
pub mod options;
/* Crate imports */
use self::options::{Options, PatternOpt};
/* Dependencies */
use clap::Subcommand;
use regex::{Regex, RegexBuilder};
use rens_common::{SedPattern, Strategy};

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
        pattern_opt: PatternOpt,
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
        pattern_opt: PatternOpt,
        #[command(flatten)]
        options: Options,
    },
    /// Perform renaming from a sed pattern.
    Sed {
        /// The sed pattern used to rename.
        /// Follows the pattern /regex/string/options.
        /// [supported options: g, i, I, x, U, <number>]
        ///
        /// Notes:
        ///  - `g` flag is enabled by default (pass any number to restrict).
        ///  - You can use anything as a separator.
        ///  - The regex must comply with `regex` crate syntax.
        ///  - You can escape the separator (any other escape sequence will be kept as is).
        #[arg(verbatim_doc_comment)]
        sed_pattern: SedPattern,
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
                pattern_opt,
                options,
            } => {
                let limit = pattern_opt.occurence.map_or(0, usize::from);
                if pattern_opt.case_insensitive {
                    pattern = to_regex_case_insensitive(&pattern);
                }

                (Strategy::new(pattern, with, limit), options)
            },
            Self::String {
                pattern,
                with,
                pattern_opt,
                options,
            } => {
                let limit = pattern_opt.occurence.map_or(0, usize::from);
                // safety guarenteed by [`regex::escape`]
                #[allow(clippy::expect_used)]
                let mut regex_pattern = Regex::new(&regex::escape(&pattern))
                    .expect("Unable to build regex.");

                if pattern_opt.case_insensitive {
                    regex_pattern = to_regex_case_insensitive(&regex_pattern);
                }

                (Strategy::new(regex_pattern, with, limit), options)
            },
            Self::Sed {
                sed_pattern,
                options,
            } => {
                let (pattern, with, limit) = sed_pattern.export();
                (Strategy::new(pattern, with, limit), options)
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
