/* Built-in imports */
use core::fmt;
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};
/* Dependencies */
use derive_more::{Constructor, Display, FromStr};

use crate::{
    traits::{FileKind, PathExt},
    Strategy,
};

#[derive(Debug, Constructor, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name {
    stem: String,
    extension: Option<String>,
}

impl Name {
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
    pub fn to_renamed(
        &self,
        strategy: &Strategy,
        target: RenameTarget,
    ) -> Self {
        match target {
            RenameTarget::Both => {
                let file_name = self.to_string();
                let renamed = strategy.pattern.replacen(
                    &file_name,
                    strategy.limit,
                    &strategy.with,
                );
                match renamed.rsplit_once('.') {
                    Some((stem, ext)) => Self {
                        stem: stem.to_owned(),
                        extension: Some(ext.to_owned()),
                    },
                    None => Self {
                        stem: renamed.to_string(),
                        extension: None,
                    },
                }
            },
            RenameTarget::Stem => Self {
                stem: strategy
                    .pattern
                    .replacen(self.stem(), strategy.limit, &strategy.with)
                    .to_string(),
                extension: self.extension().clone(),
            },
            RenameTarget::Extension => Self {
                stem: self.stem().clone(),
                extension: self.extension().clone().map(|ext| {
                    strategy
                        .pattern
                        .replacen(&ext, strategy.limit, &strategy.with)
                        .to_string()
                }),
            },
        }
    }

    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file_path = path.as_ref();
        let stem = file_path
            .file_stem()
            .and_then(OsStr::to_str)
            .map(ToString::to_string)
            .ok_or(Error::NoFileStem(file_path.to_path_buf()))?;
        let extension = file_path
            .extension()
            .and_then(OsStr::to_str)
            .map(ToString::to_string);

        Ok(Self::new(stem, extension))
    }
}

#[derive(Debug, Constructor, PartialEq, Eq)]
pub struct File {
    name: Name,
    kind: FileKind,
    parent: PathBuf,
}

impl File {
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &Name {
        &self.name
    }

    #[inline]
    #[must_use]
    pub const fn kind(&self) -> FileKind {
        self.kind
    }

    #[inline]
    #[must_use]
    pub const fn parent(&self) -> &PathBuf {
        &self.parent
    }

    #[inline]
    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.parent.join(self.name().to_string())
    }

    #[inline]
    // Can't use TryFrom because it conflicts with Into, I'm sad.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file_path = path.as_ref();
        let parent = file_path
            .parent()
            .ok_or(Error::NoParent(file_path.to_path_buf()))?
            .to_path_buf();
        let kind = file_path.kind()?;
        let name = Name::from_path(file_path)?;

        Ok(Self { name, kind, parent })
    }

    #[inline]
    #[must_use]
    pub fn renamed_name(
        &self,
        strategy: &Strategy,
        target: RenameTarget,
    ) -> Name {
        self.name().to_renamed(strategy, target)
    }

    #[inline]
    #[must_use]
    pub fn renamed_path(
        &self,
        strategy: &Strategy,
        target: RenameTarget,
    ) -> PathBuf {
        self.parent()
            .join(self.name().to_renamed(strategy, target).to_string())
    }

    #[inline]
    #[must_use]
    pub fn needs_rename(
        &self,
        strategy: &Strategy,
        target: RenameTarget,
    ) -> bool {
        self.name() != &self.name().to_renamed(strategy, target)
    }

    #[inline]
    pub fn rename(
        &self,
        strategy: &Strategy,
        target: RenameTarget,
    ) -> io::Result<()> {
        fs::rename(self.path(), self.renamed_path(strategy, target))
    }
}

impl fmt::Display for Name {
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
    #[error("{0} doesn't have a filestem.")]
    NoFileStem(PathBuf),
    #[error("{0} doesn't have a parent folder.")]
    NoParent(PathBuf),
    #[error("io::Error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Default, Display, Clone, Copy, FromStr, PartialEq, Eq)]
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
            let name = Name::from_path(path).unwrap();
            assert_eq!(
                name.to_string(),
                Path::new(path).file_name().unwrap().to_string_lossy()
            );
        }
    }
}
