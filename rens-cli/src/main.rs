#![allow(clippy::shadow_unrelated)]
/* Modules */
mod cli_options;
/* Built-in imports */
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
/* Crate imports */
use cli_options::{CliOptions, ConfirmOption, OverrideOption};
use rens_common::{
    traits::{IteratorExt, PathExt, ResultIteratorExt},
    File, Strategy,
};
/* Dependencies */
use clap::Parser;
use log::{debug, error};
use tap::{Pipe, Tap};

fn main() {
    let CliOptions {
        mode,
        canonicalize_paths,
        confirmations,
        recursion,
        target,
        paths,
        ..
    } = CliOptions::parse().tap(|options| {
        env_logger::Builder::new()
            .filter_level(options.verbose.log_level_filter())
            .init();
        debug!("{options:#?}");
    });

    let strategy: Strategy = mode.into();

    let files = paths
        .into_iter()
        // remove dir paths if recursive mode is disabled
        .filter(|path| recursion.recursive || !path.is_dir())
        // if recursive mode is enabled turn all dir paths into their child files paths
        .flat_map(|path| {
            if path.is_dir() {
                traverse_dir(path, recursion.depth, recursion.allow_hidden)
            } else {
                vec![path]
            }
        })
        .map_if(
            |_| canonicalize_paths,
            // ensured in path parsing
            #[allow(clippy::expect_used)]
            |path| dunce::canonicalize(path).expect("Canonicalization failed"),
        )
        .map(File::from_path)
        .filter_map_ok(|err| error!("{err}"))
        // Filter those for which nothing needs to be done
        .filter(|file| {
            let will_rename = file.needs_rename(&strategy, target);
            if !will_rename {
                println!("Nothing to do for {}", file.path().display());
            }
            will_rename
        })
        // Log every rename that can be done
        .tap_for_each(|file| {
            println!(
                "{} -> {}",
                file.path().display(),
                file.renamed_name(&strategy, target)
            );
        })
        // If needed, ask for confirmation
        .filter(|_| {
            (!matches!(confirmations.confirm, ConfirmOption::Each))
                || ask_for_confirm("Ok to rename?")
        })
        .pipe(Iterator::collect::<Vec<_>>);

    // If needed, ask for global confirmation
    if matches!(confirmations.confirm, ConfirmOption::Once)
        && !ask_for_confirm("All good ?")
    {
        return println!("Canceled...");
    }

    files
        .into_iter()
        // Check overrides and ask if necessary
        .filter(|file| {
            if file.renamed_path(&strategy, target).exists() {
                confirmations.allow_override.can_override(&format!(
                    "{} -> {}",
                    file.path().display(),
                    file.renamed_name(&strategy, target)
                ))
            } else {
                true
            }
        })
        .filter_map(|file| file.rename(&strategy, target).err())
        .for_each(|err| error!("{err}"));
}

#[allow(clippy::expect_used)]
fn ask_for_confirm(prompt: &str) -> bool {
    loop {
        print!("{prompt} (yes/no): ");
        let mut input = String::new();
        io::stdout().lock().flush().expect("Failed to flush stdin.");
        io::stdin().read_line(&mut input).expect("Failed to stdin.");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no'."),
        }
    }
}

impl OverrideOption {
    fn can_override(&self, rename_prompt: &str) -> bool {
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

#[allow(clippy::expect_used)]
fn traverse_dir<P: AsRef<Path>>(
    path: P,
    depth: u8,
    allow_hidden: bool,
) -> Vec<PathBuf> {
    debug_assert!(path.as_ref().is_dir(), "Cannot traverse a non directory");
    if depth == 0 {
        return vec![];
    }

    fs::read_dir(path)
        .map(|dir| {
            dir.filter_ok()
                .filter(|entry| allow_hidden || !entry.path().is_hidden())
                .map(|entry| entry.path())
                .flat_map(|path| {
                    if path.is_dir() {
                        traverse_dir(path, depth - 1, allow_hidden)
                    } else {
                        vec![path]
                    }
                })
                .collect()
        })
        .expect("msg")
}
