extern crate core;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate indoc;
#[cfg(test)]
#[macro_use]
extern crate insta;
#[cfg(test)]
#[macro_use]
mod test;

use std::process::exit;

use color_eyre::{Help, Report, SectionExt};
use console::{style, Term};
use eyre::Result;
use itertools::Itertools;

use crate::cli::version::VERSION;
use crate::cli::Cli;

#[macro_use]
mod output;

#[macro_use]
mod regex;

#[macro_use]
mod cmd;

pub mod build_time;
mod cache;
mod cli;
mod config;
mod default_shorthands;
mod direnv;
mod dirs;
pub mod duration;
mod env;
mod env_diff;
mod errors;
mod fake_asdf;
mod file;
mod git;
pub mod github;
mod hash;
mod hook_env;
mod http;
mod install_context;
mod lock_file;
mod logger;
mod migrate;
mod path_env;
mod plugins;
mod rand;
mod runtime_symlinks;
mod shell;
mod shims;
mod shorthands;
pub mod tera;
pub mod timeout;
mod toml;
mod toolset;
mod ui;

fn main() -> Result<()> {
    let args = env::args().collect_vec();
    color_eyre::install()?;
    handle_ctrlc();
    migrate::run();

    match Cli::run(&args).with_section(|| VERSION.to_string().header("Version:")) {
        Ok(()) => Ok(()),
        Err(err) if log::max_level() < log::LevelFilter::Debug => {
            display_friendly_err(err);
            exit(1);
        }
        Err(err) => {
            Err(err).suggestion("Run with --verbose or RTX_VERBOSE=1 for more information.")
        }
    }
}

fn handle_ctrlc() {
    let _ = ctrlc::set_handler(move || {
        let _ = Term::stderr().show_cursor();
        debug!("Ctrl-C pressed, exiting...");
        exit(1);
    });
}

fn display_friendly_err(err: Report) {
    for err in err.chain() {
        error!("{err}");
    }
    let dim = |s| style(s).dim().for_stderr();
    error!(
        "{}",
        dim("Run with --verbose or RTX_VERBOSE=1 for more information")
    );
}
