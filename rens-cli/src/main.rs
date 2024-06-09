#![allow(clippy::shadow_unrelated)]
/* Modules */
mod cli;
mod utils;
/* Built-in imports */
use std::{fs, io};
/* Crate imports */
use cli::{
    renaming::{ConfirmOption, Options},
    Cli, Commands,
};
use utils::{ask_for_confirm, traverse_dir};
/* Dependencies */
use clap::{CommandFactory, Parser};
use log::{debug, error};
use rens_common::{
    traits::{BoolExt, IteratorExt, ResultIteratorExt},
    File,
};
use tap::{Pipe, Tap};

#[allow(clippy::too_many_lines)]
fn main() -> anyhow::Result<()> {
    let Cli {
        command,
        verbose: _,
    } = Cli::parse().tap(|options| {
        env_logger::Builder::new()
            .filter_level(options.verbose.log_level_filter())
            .init();
        debug!("{options:#?}");
    });

    match command {
        Commands::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut io::stdout());
        },
        Commands::Man { path } => {
            let command = Cli::command();
            if !path.exists() {
                fs::create_dir_all(&path)?;
            }

            for subcommand in command.get_subcommands() {
                let subcommand_filename =
                    format!("{}-{}", command.get_name(), subcommand.get_name());
                let cmd = subcommand.clone().name(subcommand_filename);
                clap_mangen::Man::new(cmd).generate_to(&path)?;
            }

            clap_mangen::Man::new(command).generate_to(path)?;
        },
        Commands::Renaming(mode) => {
            let (
                strategy,
                Options {
                    confirmations,
                    paths_opt,
                    recursion,
                    target,
                    paths,
                    pattern_opt: _,
                },
            ) = mode.get_strategy_and_options();

            let files = paths
                .into_iter()
                // remove dir paths if recursive mode is disabled
                .filter(|path| recursion.recursive || !path.is_dir())
                // if recursive mode is enabled turn all dir paths into their child files paths
                .flat_map_if(
                    |path| path.is_dir(),
                    |path| {
                        traverse_dir(
                            path,
                            recursion.depth,
                            recursion.allow_hidden,
                        )
                        .into_iter()
                    },
                )
                .map_if(
                    |_| paths_opt.canonicalize_paths,
                    // ensured in path parsing
                    #[allow(clippy::expect_used)]
                    |path| {
                        dunce::canonicalize(path)
                            .expect("Canonicalization failed")
                    },
                )
                .map(File::from_path)
                .filter_map_ok(|err| error!("{err}"))
                // Filter those for which nothing needs to be done
                .filter(|file| {
                    file.needs_rename(&strategy, target).tap_if_false(|| {
                        println!("Nothing to do for {}", file.path().display());
                    })
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
                    confirmations.confirm != ConfirmOption::Each
                        || ask_for_confirm("Ok to rename?")
                })
                .pipe(Iterator::collect::<Vec<_>>);

            // If needed, ask for global confirmation
            if confirmations.confirm == ConfirmOption::Once
                && !ask_for_confirm("All good ?")
            {
                println!("Canceled...");
                return Ok(());
            }

            files
                .into_iter()
                // Check overrides and ask if necessary
                .filter(|file| {
                    !file.renamed_path(&strategy, target).exists()
                        || confirmations.allow_override.can_override(&format!(
                            "{} -> {}",
                            file.path().display(),
                            file.renamed_name(&strategy, target)
                        ))
                })
                .filter_map(|file| file.rename(&strategy, target).err())
                .for_each(|err| error!("{err}"));
        },
    }

    Ok(())
}
