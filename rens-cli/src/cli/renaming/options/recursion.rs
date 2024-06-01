/* Dependencies */
use clap::{builder::ArgPredicate, ArgAction, Args};

#[derive(Debug, Args)]
pub struct Recursion {
    /// Decides if folder paths includes their children recursively.
    ///
    /// Note: implied if --depth is used.
    #[arg(
        global = true,
        long, short,
        default_value_t = false,
        default_value_if("depth", ArgPredicate::IsPresent, "true"),
        action = ArgAction::SetTrue,
    )]
    pub recursive: bool,

    /// If recursive mode is enabled, decides how deep the renaming goes.
    #[arg(global = true, long, value_name = "depth")]
    // Note: None gets used as `As deep as possible`.
    pub depth: Option<usize>,

    /// When traversing directories, include hidden files.
    #[arg(
        global = true,
        long, short,
        default_value_t = false,
        action = ArgAction::SetTrue,
    )]
    pub allow_hidden: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};

    #[derive(Debug, Parser)]
    struct TestParser {
        #[command(flatten)]
        pub options: Recursion,
    }

    #[test]
    fn recursion_options_are_valid() {
        TestParser::command().debug_assert();
    }

    #[test]
    fn test_default_values() {
        let args = TestParser::parse_from::<[_; 0], &str>([]);

        assert!(!args.options.recursive);
        assert_eq!(args.options.depth, None);
        assert!(!args.options.allow_hidden);
    }

    #[test]
    fn test_recursive_flag() {
        assert!(TestParser::parse_from(["test-cli", "-r"]).options.recursive);

        assert!(
            TestParser::parse_from(["test-cli", "--recursive"])
                .options
                .recursive
        );
    }

    #[test]
    fn test_depth() {
        // should fail if no value provided
        TestParser::try_parse_from(["rens-cli", "--depth"]).unwrap_err();

        let args = TestParser::parse_from(["test-cli", "--depth", "5"]);
        assert_eq!(args.options.depth, Some(5));
        assert!(args.options.recursive);
    }

    #[test]
    fn test_allow_hidden() {
        assert!(
            TestParser::parse_from(["test-cli", "-a"])
                .options
                .allow_hidden
        );

        assert!(
            TestParser::parse_from(["test-cli", "--allow-hidden"])
                .options
                .allow_hidden
        );
    }
}
