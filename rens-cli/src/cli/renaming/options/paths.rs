/* Dependencies */
use clap::{ArgAction, Args};

#[derive(Debug, Args)]
#[group(skip)]
pub struct Options {
    /// Canonicalize all paths instead of using relative ones.
    #[arg(
        long,
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    pub canonicalize_paths: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};

    #[derive(Parser)]
    struct TestParser {
        #[command(flatten)]
        pub options: Options,
    }

    #[test]
    fn verify_conformity() {
        TestParser::command().debug_assert();
    }
}
