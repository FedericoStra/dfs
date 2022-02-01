use log::{debug, info};

use anyhow::Context;

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
            ensure_project_dirs(dirs).context("cannot initialize config")?
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
