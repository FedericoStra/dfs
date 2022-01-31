//! # Dotfiles synchronizer
//!
//! [![GitHub](https://img.shields.io/badge/github-FedericoStra/dfs-master?logo=github)](https://github.com/FedericoStra/dfs)
//! [![Crates.io](https://img.shields.io/crates/v/dfs?logo=rust)](https://crates.io/crates/dfs)
//! [![docs.rs](https://img.shields.io/docsrs/dfs?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48cGF0aCBkPSJNNDg4LjYgMjUwLjJMMzkyIDIxNFYxMDUuNWMwLTE1LTkuMy0yOC40LTIzLjQtMzMuN2wtMTAwLTM3LjVjLTguMS0zLjEtMTcuMS0zLjEtMjUuMyAwbC0xMDAgMzcuNWMtMTQuMSA1LjMtMjMuNCAxOC43LTIzLjQgMzMuN1YyMTRsLTk2LjYgMzYuMkM5LjMgMjU1LjUgMCAyNjguOSAwIDI4My45VjM5NGMwIDEzLjYgNy43IDI2LjEgMTkuOSAzMi4ybDEwMCA1MGMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAzLjktNTIgMTAzLjkgNTJjMTAuMSA1LjEgMjIuMSA1LjEgMzIuMiAwbDEwMC01MGMxMi4yLTYuMSAxOS45LTE4LjYgMTkuOS0zMi4yVjI4My45YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43ek0zNTggMjE0LjhsLTg1IDMxLjl2LTY4LjJsODUtMzd2NzMuM3pNMTU0IDEwNC4xbDEwMi0zOC4yIDEwMiAzOC4ydi42bC0xMDIgNDEuNC0xMDItNDEuNHYtLjZ6bTg0IDI5MS4xbC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnptMjQwIDExMmwtODUgNDIuNXYtNzkuMWw4NS0zOC44djc1LjR6bTAtMTEybC0xMDIgNDEuNC0xMDItNDEuNHYtLjZsMTAyLTM4LjIgMTAyIDM4LjJ2LjZ6IiBzdHlsZT0iZmlsbDojZmZmZmZmIj48L3BhdGg+PC9zdmc+Cg==)](https://docs.rs/dfs)
//! [![MIT license](https://img.shields.io/crates/l/dfs)](https://github.com/FedericoStra/dfs/blob/master/LICENSE)
//! [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/FedericoStra/dfs/Rust)](https://github.com/FedericoStra/dfs/actions/workflows/rust.yml)
//! ![Lines of code](https://tokei.rs/b1/github/FedericoStra/dfs?category=code)
//!
//! [`dfs`]() is a CLI utility to synchronize your dotfiles.
//!
//! It supports managing several profiles with operations such as
//! add, remove, update, link, apply...
//!
//! You can access the help for the subcommands by executing:
//! ```bash
//! dfs help <SUBCOMMAND>
//! ```

use clap::{AppSettings, Parser, Subcommand};

use env_logger;
use log::{error, trace};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: CliCommand,

    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    /// Show or edit the configuration
    Config {
        #[clap(subcommand)]
        command: ConfigCommand,
    },

    /// Show current status
    Status,

    /// Manage profiles
    Profile {
        #[clap(subcommand)]
        command: ProfileCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    /// Initialize a new default configuration
    Init,

    /// Show the current configuration
    Show,
}

#[derive(Debug, Subcommand)]
enum ProfileCommand {
    /// List the available profiles
    List,

    /// Select a profile
    Select {
        /// Apply the profile
        #[clap(short, long)]
        apply: bool,

        /// Profile to select
        name: String,
    },

    /// Create a new profile
    New {
        /// Profile name
        name: String,
    },

    /// Remove a profile
    Remove {
        /// Profile to remove
        name: String,

        /// Do not actually remove the profile
        #[clap(short, long)]
        dry_run: bool,
    },
}

mod config;
mod dirs;
mod profile;

fn main() {
    let cli = Cli::parse();

    set_up_env_logger(cli.verbose);
    trace!("env_logger set up");

    trace!("cli = {cli:#?}");

    if let Err(err) = run(cli) {
        error!("{err:?}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    let dirs = dirs::project_dirs()?;

    match cli.command {
        CliCommand::Config { command } => config::run(command, &dirs),
        CliCommand::Status => todo!(),
        CliCommand::Profile { command } => profile::run(command, &dirs),
    }
}

fn set_up_env_logger(verbose: bool) {
    use std::io::Write;
    env_logger::Builder::new()
        .filter_level(if verbose {
            log::LevelFilter::Info
        } else {
            log::LevelFilter::Warn
        })
        .format_timestamp(None)
        .format_module_path(false)
        .format(|buf, record| {
            use env_logger::fmt::Color::*;
            use log::Level::*;

            let mut gray = buf.style();
            gray.set_color(Black).set_intense(true);

            let mut level_style = buf.style();
            match record.level() {
                Error => level_style.set_color(Red).set_intense(true),
                Warn => level_style.set_color(Yellow).set_intense(true),
                Info => level_style.set_color(Green).set_intense(true),
                Debug => level_style.set_color(Magenta).set_intense(true),
                Trace => level_style.set_color(Black).set_intense(true),
            };

            let message_style = if record.level() == Info {
                buf.style()
            } else {
                level_style.clone()
            };

            writeln!(
                buf,
                "{}{level:>5}{} {message}",
                gray.value('['),
                gray.value(']'),
                level = level_style.value(record.level()),
                message = message_style.value(record.args())
            )
        })
        .parse_default_env()
        .init();
}
