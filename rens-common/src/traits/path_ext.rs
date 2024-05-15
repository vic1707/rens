/* Built-in imports */
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
#[cfg(windows)]
use std::os::windows::fs::FileTypeExt;
use std::{fs, io, path::Path};
/* Dependencies */
use derive_more::Display;

#[derive(Debug, Display)]
#[non_exhaustive]
pub enum Kind {
    File,
    Directory,
    Symlink,
    #[cfg(unix)]
    UnixSocket,
    Other,
}

pub trait PathExt {
    fn kind(&self) -> io::Result<Kind>;
}

impl<P: AsRef<Path>> PathExt for P {
    #[inline]
    fn kind(&self) -> io::Result<Kind> {
        let metadata = fs::symlink_metadata(self)?;
        if metadata.is_dir() {
            return Ok(Kind::Directory);
        }
        if metadata.file_type().is_symlink() {
            return Ok(Kind::Symlink);
        }
        #[cfg(unix)]
        if metadata.file_type().is_socket() {
            return Ok(Kind::UnixSocket);
        }
        #[cfg(windows)]
        if metadata.file_type().is_symlink_dir()
            || metadata.file_type().is_symlink_file()
        {
            return Ok(Kind::Symlink);
        }
        if metadata.is_file() {
            return Ok(Kind::File);
        }

        Ok(Kind::Other)
    }
}
