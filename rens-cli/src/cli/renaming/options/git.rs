/* Dependencies */
use clap::{ArgAction, Args};

#[derive(Debug, Args)]
#[group(id = "git_options")]
#[command(next_help_heading = "Git integration Options", display_order = 0)]
pub struct Options {
    #[arg(
        name = "ignore",
        long, short,
        default_value_t = false,
        action = ArgAction::SetTrue,
    )]
    /// Parse and follow `.gitignore` (local and global), `.ignore` and `.git/info/exclude` files.
    pub auto_ignore: bool,
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
    fn verify_conformity() {
        TestParser::command().debug_assert();
    }
}
