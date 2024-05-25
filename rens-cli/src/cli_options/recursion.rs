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
        default_value_t = true,
        default_value_if("depth", ArgPredicate::IsPresent, "true"),
        action = ArgAction::SetTrue
    )]
    pub recursive: bool,

    /// If recursive mode is enabled, decides how deep the renaming goes.
    ///
    /// Note: 0 means as deep as possible.
    #[arg(global = true, long, default_value_t = 1, value_name = "number")]
    pub depth: u8,

    /// When traversing directories, include hidden files.
    #[arg(
        global = true,
        long, short,
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    pub allow_hidden: bool,
}
