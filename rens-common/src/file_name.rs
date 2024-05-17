/* Built-in imports */
use core::fmt;
use std::{ffi::OsStr, io, path::Path};
/* Dependencies */
use derive_more::{Constructor, Display, FromStr};

use crate::Strategy;

#[derive(Debug, Constructor, PartialEq, Eq)]
pub struct FileName {
    stem: String,
    extension: Option<String>,
}

impl FileName {
    #[inline]
    #[must_use]
    pub const fn stem(&self) -> &String {
        &self.stem
    }

    #[inline]
    #[must_use]
    pub const fn extension(&self) -> &Option<String> {
        &self.extension
    }

    #[inline]
    #[must_use]
    pub fn to_renamed(&self, strat: &Strategy, target: RenameTarget) -> Self {
        match target {
            RenameTarget::Both => {
                let file_name = self.to_string();
                let renamed = strat.pattern.replacen(
                    &file_name,
                    strat.limit,
                    &strat.with,
                );
                match renamed.rsplit_once('.') {
                    Some((stem, ext)) => {
                        Self::new(stem.to_owned(), Some(ext.to_owned()))
                    },
                    None => Self::new(renamed.to_string(), None),
                }
            },
            RenameTarget::Stem => Self::new(
                strat
                    .pattern
                    .replacen(self.stem(), strat.limit, &strat.with)
                    .to_string(),
                self.extension().clone(),
            ),
            RenameTarget::Extension => Self::new(
                self.stem().clone(),
                self.extension().clone().map(|ext| {
                    strat
                        .pattern
                        .replacen(&ext, strat.limit, &strat.with)
                        .to_string()
                }),
            ),
        }
    }

    #[inline]
    // Can't use TryFrom because it conflicts with Into, I'm sad.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file_path = path.as_ref();
        let stem = file_path
            .file_stem()
            .and_then(OsStr::to_str)
            .map(ToString::to_string)
            .ok_or(Error::NoFileStem)?;
        let extension = file_path
            .extension()
            .and_then(OsStr::to_str)
            .map(ToString::to_string);

        Ok(Self::new(stem, extension))
    }
}

impl fmt::Display for FileName {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}{}",
            self.stem,
            self.extension
                .as_ref()
                .map_or_else(String::new, |ext| format!(".{ext}"))
        )
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Given path doesn't have a filestem.")]
    NoFileStem,
    #[error("io::Error: {0}")]
    IoError(#[from] io::Error),
}

#[derive(Debug, Default, Display, Clone, Copy, FromStr)]
#[non_exhaustive]
pub enum RenameTarget {
    /// File stem is the filename without the extension
    /// See: <https://doc.rust-lang.org/std/path/struct.Path.html#method.file_stem>
    Stem,
    Extension,
    #[default]
    Both,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_fs_file_name() {
        const PATHS: [&str; 9] = [
            "./file.txt",
            "./.hidden_file",
            "./.hidden_script.sh",
            "./data",
            "./README",
            "./oups.",
            "./archive.tar.gz",
            "./source_code.rs.",
            "./looking_for_trouble.rs....",
        ];

        for path in PATHS {
            let file_name = FileName::from_path(path).unwrap();
            assert_eq!(
                format!("{file_name}"),
                Path::new(path).file_name().unwrap().to_string_lossy()
            );
        }
    }
}
