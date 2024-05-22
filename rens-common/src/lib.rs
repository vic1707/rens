/* Modules */
pub mod traits;
mod file;
/* Dependencies */
use derive_more::{Constructor, Display};
use regex::Regex;
/* Re-exports */
pub use file::{File, RenameTarget};

#[derive(Debug, Display, Constructor)]
#[display("{pattern}\n{with}\n{limit}")]
pub struct Strategy {
    pattern: Regex,
    with: String,
    /// 0 means all
    limit: usize,
}
