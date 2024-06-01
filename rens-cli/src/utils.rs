/* Built-in imports */
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
/* Dependencies */
use log::error;
use rens_common::traits::{IteratorExt, PathExt, ResultIteratorExt};

use crate::cli::renaming::OverrideOption;

#[allow(clippy::expect_used)]
pub fn ask_for_confirm(prompt: &str) -> bool {
    loop {
        print!("{prompt} (yes/no): ");
        let mut input = String::new();
        io::stdout().lock().flush().expect("Failed to flush stdin.");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin.");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no'."),
        }
    }
}

impl OverrideOption {
    pub fn can_override(&self, rename_prompt: &str) -> bool {
        match *self {
            Self::Allow => true,
            Self::Deny => false,
            Self::Ask => {
                println!("{rename_prompt}");
                ask_for_confirm("Will override...")
            },
        }
    }
}

pub fn traverse_dir<P: AsRef<Path>>(
    path: P,
    depth: Option<usize>,
    allow_hidden: bool,
) -> Vec<PathBuf> {
    debug_assert!(path.as_ref().is_dir(), "Cannot traverse a non directory");

    match fs::read_dir(&path) {
        Ok(dir_entry) => dir_entry
            .filter_map_ok(|err| error!("{err:#?}"))
            .map(|entry| entry.path())
            .filter(|path| allow_hidden || !path.is_hidden())
            .filter(|path| !(path.is_dir() && matches!(depth, Some(0))))
            .flat_map_if(
                |path| path.is_dir(),
                |path| {
                    traverse_dir(path, depth.map(|dp| dp - 1), allow_hidden)
                        .into_iter()
                },
            )
            .collect(),
        Err(err) => {
            error!("{}: {err}", path.as_ref().display());
            vec![]
        },
    }
}
