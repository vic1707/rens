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

#[derive(Debug, Clone, ValueEnum)]
pub enum OverrideOption {
    #[clap(help = "Ask for every change.")]
    Ask,
    #[clap(help = "Always allow.")]
    Allow,
    #[clap(help = "Always deny.")]
    Deny,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ConfirmOption {
    #[clap(help = "Ask for every change.")]
    Each,
    #[clap(help = "Ask once after showing every change.")]
    Once,
    #[clap(help = "Always allow.")]
    Never,
}
