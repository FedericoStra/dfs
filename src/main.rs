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

    /// Remove a profile
    Remove {
        /// Profile to remove
        name: String,
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
