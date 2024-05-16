#![allow(clippy::shadow_unrelated)]
/* Modules */
mod cli_options;
mod rens_target;
/* Built-in imports */
use std::io::{self, Write};
use std::path::PathBuf;
/* Crate imports */
use rens_common::{traits::IteratorExt, FileName, Renamer};
use rens_target::RensTarget;
/* Dependencies */
use anyhow::anyhow;
use clap::Parser;
use cli_options::{CliOptions, ConfirmOption, Mode};
use log::{debug, error};
use regex::Regex;
use tap::{Pipe, Tap};

use crate::cli_options::OverrideOption;

fn main() -> anyhow::Result<()> {
    let CliOptions {
        mode,
        recursion,
        target,
        paths,
        confirmations,
        canonicalize,
        ..
    } = CliOptions::parse().tap(|options| {
        env_logger::Builder::new()
            .filter_level(options.verbose.log_level_filter())
            .init();
        debug!("{options:#?}");
    });

    let rens_renamer = match mode {
        Mode::Regex {
            pattern,
            with,
            options,
        } => Renamer::new(pattern, with, options.occurence.into(), target),
        Mode::String {
            pattern,
            with,
            options,
        } => Renamer::new(
            Regex::new(&regex::escape(&pattern))?,
            with,
            options.occurence.into(),
            target,
        ),
    };

    let ok_targets = paths
        .into_iter()
        // Cannonicalize if asked
        .map_if(
            |_| canonicalize,
            #[allow(clippy::expect_used)] // ensured in path parsing
            |path| dunce::canonicalize(path).expect("Canonicalization failed"),
        )
        // Build the rename informations
        .map::<anyhow::Result<RensTarget>, _>(|path| {
            let filename = FileName::from_path(path.clone())?;
            let target_dir = path
                .parent()
                .map(PathBuf::from)
                .ok_or_else(|| anyhow!("No parent for {} !", path.display()))?;

            let renamed = rens_renamer.to_renamed_file(&filename);
            Ok(RensTarget {
                path,
                filename,
                renamed,
                target_dir,
            })
        })
        // Filter out the errors (and log them)
        .filter(|target_res| match *target_res {
            Ok(_) => true,
            Err(ref err) => {
                error!("{err}");
                false
            },
        })
        .map(Result::unwrap)
        // Filter those for which nothing needs to be done
        .filter(|target| {
            let will_rename = target.original_path() != target.renamed_path();
            if !will_rename {
                println!(
                    "Nothing to do for {}",
                    target.original_path().display()
                );
            }
            will_rename
        })
        // Log every rename that can be done
        .i_tap(|target| println!("{}", target.rename_prompt()))
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
        return Err(anyhow!("Canceled..."));
    }

    ok_targets
        .into_iter()
        // Check overrides and ask if necessary
        .filter(|target| {
            if target.renamed_path().exists() {
                confirmations
                    .allow_override
                    .can_override(&target.rename_prompt())
            } else {
                true
            }
        })
        // Does the actual rename
        .map(RensTarget::rename)
        // Log the errors
        .filter(Result::is_err)
        .map(Result::unwrap_err)
        .for_each(|err| error!("{err}"));

    Ok(())
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
