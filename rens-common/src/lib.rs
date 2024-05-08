/* Clippy config */
#![allow(clippy::pattern_type_mismatch)]
/* Modules */
pub mod traits;
mod file_name;
/* Built-in imports */
use core::num::ParseIntError;
/* Dependencies */
use derive_more::{Constructor, Display, FromStr, IsVariant};
use regex::Regex;
/* Re-exports */
pub use file_name::{FileName, RenameTarget};

#[derive(Debug, Display, Constructor)]
#[display("{pattern}\n{with}\n{occurences}")]
pub struct Renamer {
    pattern: Regex,
    with: String,
    occurences: Occurence,
    target: RenameTarget,
}

impl Renamer {
    #[inline]
    #[must_use]
    pub fn to_renamed_file(&self, file: &FileName) -> FileName {
        let num = self.occurences.get_number_of_replacements();

        match self.target {
            RenameTarget::Both => {
                let file_name = file.to_string();
                let renamed =
                    self.pattern.replacen(&file_name, num, &self.with);
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
                    self.pattern.replacen(&ext, num, &self.with).to_string()
                }),
            ),
            RenameTarget::Stem => FileName::new(
                self.pattern
                    .replacen(file.stem(), num, &self.with)
                    .to_string(),
                file.extension().clone(),
            ),
        }
    }
}

#[derive(Debug, Display, IsVariant, Clone)]
#[allow(clippy::exhaustive_enums)]
pub enum Occurence {
    Global,
    Numbered(usize),
}

impl Occurence {
    /// Meant to be used with [`regex::Regex::renamen`] method.
    /// In that method 0 replacements means all (global).
    const fn get_number_of_replacements(&self) -> usize {
        match *self {
            Self::Global => 0,
            Self::Numbered(num) => num,
        }
    }
}

impl FromStr for Occurence {
    type Err = ParseIntError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "all" | "All" | "global" | "Global" | "g" => Ok(Self::Global),
            str => match usize::from_str(str) {
                Ok(count) => Ok(Self::Numbered(count)),
                Err(err) => Err(err),
            },
        }
    }
}
