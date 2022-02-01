use anyhow::Context;
use std::io::Write;
// use config::{Config, Environment, File};
use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::dirs::{ensure_project_dirs, ProjectDirs};
use crate::ConfigCommand;

// use thiserror::Error;
// #[derive(Debug, Error)]
// pub enum ConfigError {
//     #[error("cannot initialize project directories")]
//     InitError {
//         #[from]
//         source: std::io::Error,
//     },
// }

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Configuration {
    profile: Option<String>,
}

pub(crate) fn run(cmd: ConfigCommand, dirs: &ProjectDirs) -> anyhow::Result<()> {
    fn report_dir(qual: &str, dir: &std::path::Path) {
        if dir.is_dir() {
            println!("{qual} dir: {dir:?}");
        } else {
            println!("{qual} dir: {dir:?} (does not exist or is not a directory)");
        }
    }

    match cmd {
        ConfigCommand::Show => {
            debug!("executing `config show`");
            report_dir("config", dirs.config_dir());
            report_dir("data", dirs.data_dir());
            // println!("current profile: {:?}", todo!());
        }
        ConfigCommand::Init => {
            debug!("executing `config init`");
            ensure_project_dirs(dirs).context("cannot initialize config")?;
            let conf_file_path = dirs.config_dir().join("config.toml");
            let mut configuration = Configuration::default();
            configuration.profile = Some("default".to_string());
            let conf_string = toml::to_string(&configuration)?;
            let mut conf_file = std::fs::File::create(&conf_file_path)
                .with_context(|| format!("cannot open configuration file {:?}", conf_file_path))?;
            conf_file
                .write(conf_string.as_ref())
                .with_context(|| format!("cannot write configuration file {:?}", conf_file_path))?;
            info!("config initialized");
        }
        ConfigCommand::Purge { force } => {
            debug!("executing `config purge`");
            if force {
                std::fs::remove_dir_all(dirs.config_dir())
                    .with_context(|| format!("cannot remove config dir {:?}", dirs.config_dir()))?;
                std::fs::remove_dir_all(dirs.data_dir())
                    .with_context(|| format!("cannot remove data dir {:?}", dirs.data_dir()))?;
                info!("removed config dir {:?}", dirs.config_dir());
                info!("removed data dir {:?}", dirs.data_dir());
            } else {
                println!("This would purge:");
                report_dir("config", dirs.config_dir());
                report_dir("data", dirs.data_dir());
                println!("To actually purge everything re-run with `--force`");
                info!("would purge everything!");
            }
        }
    }

    Ok(())
}
