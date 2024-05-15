/* Clippy config */
#![allow(clippy::pattern_type_mismatch)]
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
pub struct Renamer {
    pattern: Regex,
    with: String,
    /// 0 means all
    limit: usize,
    target: RenameTarget,
}

impl Renamer {
    #[inline]
    #[must_use]
    pub fn to_renamed_file(&self, file: &FileName) -> FileName {
        match self.target {
            RenameTarget::Both => {
                let file_name = file.to_string();
                let renamed =
                    self.pattern.replacen(&file_name, self.limit, &self.with);
                match renamed.rsplit_once('.') {
                    Some((stem, ext)) => {
                        FileName::new(stem.to_owned(), Some(ext.to_owned()))
                    },
                    None => FileName::new(renamed.to_string(), None),
                }
            },
            RenameTarget::Extension => FileName::new(
                file.stem().clone(),
                file.extension().clone().map(|ext| {
                    self.pattern
                        .replacen(&ext, self.limit, &self.with)
                        .to_string()
                }),
            ),
            RenameTarget::Stem => FileName::new(
                self.pattern
                    .replacen(file.stem(), self.limit, &self.with)
                    .to_string(),
                file.extension().clone(),
            ),
        }
    }
}
