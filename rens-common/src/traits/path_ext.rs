/* Built-in imports */
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
#[cfg(windows)]
use std::os::windows::fs::{FileTypeExt, MetadataExt};
use std::{fs, io, path::Path};
/* Dependencies */
use derive_more::Display;

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum Kind {
    File,
    Directory,
    Symlink,
    #[cfg(unix)]
    UnixSocket,
    Other,
}

pub trait PathExt: AsRef<Path> {
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

    #[inline]
    #[cfg(unix)]
    fn is_hidden(&self) -> bool {
        use std::ffi::OsStr;

        self.as_ref()
            .file_name()
            .map(OsStr::to_string_lossy)
            .is_some_and(|name| name.starts_with('.'))
    }

    #[inline]
    #[cfg(windows)]
    fn is_hidden(&self) -> bool {
        /// See: <https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants>
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x0000_0002;

        self.as_ref()
            .metadata()
            .map(|metadata| metadata.file_attributes())
            .is_ok_and(|attr| attr & FILE_ATTRIBUTE_HIDDEN != 0)
    }
}

impl<P: AsRef<Path>> PathExt for P {}
