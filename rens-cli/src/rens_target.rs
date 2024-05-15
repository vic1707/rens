/* Built-in imports */
use std::{
    fs, io,
    path::{Path, PathBuf},
};
/* Crate imports */
use rens_common::FileName;

pub struct RensTarget {
    pub path: PathBuf,
    pub filename: FileName,
    pub renamed: FileName,
    pub target_dir: PathBuf,
}

impl RensTarget {
    pub fn rename(self) -> io::Result<()> {
        fs::rename(&self.path, self.renamed_path())
    }
    pub fn copy(self) -> io::Result<()> {
        fs::copy(&self.path, self.renamed_path())?;
        Ok(())
    }
    pub fn original_path(&self) -> &Path {
        &self.path
    }
    pub fn renamed_path(&self) -> PathBuf {
        self.target_dir.join(self.renamed.to_string())
    }
    pub fn rename_prompt(&self) -> String {
        format!(
            "{} -> {}",
            self.original_path().display(),
            self.renamed_path().display()
        )
    }
}
