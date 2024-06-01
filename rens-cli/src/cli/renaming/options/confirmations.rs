/* Dependencies */
use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct Confirmations {
    /// Behavior when a renamed file already exists.
    #[arg(
        global = true,
        long,
        value_enum,
        default_value = "ask",
        default_missing_value = "allow",
        require_equals = true,
        num_args = 0..=1,
    )]
    pub allow_override: OverrideOption,

    /// Behavior when upon effective renaming.
    #[arg(
        global = true,
        long,
        value_enum,
        default_value = "each",
        default_missing_value = "once",
        require_equals = true,
        num_args = 0..=1,
    )]
    pub confirm: ConfirmOption,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum OverrideOption {
    #[clap(help = "Ask for every change.")]
    Ask,
    #[clap(help = "Always allow.")]
    Allow,
    #[clap(help = "Always deny.")]
    Deny,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum ConfirmOption {
    #[clap(help = "Ask for every change.")]
    Each,
    #[clap(help = "Ask once after showing every change.")]
    Once,
    #[clap(help = "Always allow.")]
    Never,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};

    #[derive(Debug, Parser)]
    struct TestParser {
        #[command(flatten)]
        pub confirmations: Confirmations,
    }

    #[test]
    fn confirmations_are_valid() {
        TestParser::command().debug_assert();
    }

    #[test]
    fn test_default_values() {
        let args = TestParser::parse_from::<[_; 0], &str>([]);
        assert_eq!(args.confirmations.allow_override, OverrideOption::Ask);
        assert_eq!(args.confirmations.confirm, ConfirmOption::Each);
    }

    #[test]
    fn test_allow_override() {
        assert_eq!(
            TestParser::parse_from(["rens-cli", "--allow-override"])
                .confirmations
                .allow_override,
            OverrideOption::Allow
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--allow-override=ask"])
                .confirmations
                .allow_override,
            OverrideOption::Ask
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--allow-override=allow"])
                .confirmations
                .allow_override,
            OverrideOption::Allow
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--allow-override=deny"])
                .confirmations
                .allow_override,
            OverrideOption::Deny
        );
    }

    #[test]
    fn test_confirm() {
        assert_eq!(
            TestParser::parse_from(["rens-cli", "--confirm"])
                .confirmations
                .confirm,
            ConfirmOption::Once
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--confirm=each"])
                .confirmations
                .confirm,
            ConfirmOption::Each
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--confirm=never"])
                .confirmations
                .confirm,
            ConfirmOption::Never
        );

        assert_eq!(
            TestParser::parse_from(["rens-cli", "--confirm=once"])
                .confirmations
                .confirm,
            ConfirmOption::Once
        );
    }
}
