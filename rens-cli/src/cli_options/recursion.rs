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
        action = ArgAction::SetTrue
    )]
    pub recursive: bool,

    /// If recursive mode is enabled, decides how deep the renaming goes.
    ///
    /// Note: 0 means as deep as possible.
    #[arg(global = true, long, default_value_t = 0, value_name = "number")]
    pub depth: u8,
}
