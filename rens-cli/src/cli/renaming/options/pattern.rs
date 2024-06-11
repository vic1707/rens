/* Built-in imports */
use core::num::NonZeroUsize;
/* Dependencies */
use clap::Args;

#[derive(Debug, Args)]
#[group(id = "pattern_options")]
#[command(next_help_heading = "Pattern Options")]
pub struct Options {
    /// Wether or not the pattern should be made case sensitive.
    ///
    /// Note: No effect if regex pattern already includes case settings at its beggining.
    #[arg(long, default_value_t = false)]
    pub case_insensitive: bool,

    /// Number of replacements to be done.
    #[arg(long, short, value_name = "number of repetitions")]
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
        pub options: Options,
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
