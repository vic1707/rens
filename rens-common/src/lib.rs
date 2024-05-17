/* Modules */
pub mod traits;
mod file_name;
/* Dependencies */
use derive_more::{Constructor, Display};
use regex::Regex;
/* Re-exports */
pub use file_name::{FileName, RenameTarget};

#[derive(Debug, Display, Constructor)]
#[display("{pattern}\n{with}\n{limit}")]
pub struct Strategy {
    pattern: Regex,
    with: String,
    /// 0 means all
    limit: usize,
}
