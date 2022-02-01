use log::debug;

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
    match cmd {
        ConfigCommand::Show => {
            debug!("executing `config show`");

            fn report_dir(qual: &str, dir: &std::path::Path) {
                if dir.is_dir() {
                    println!("{qual} dir: {dir:?}");
                } else {
                    println!("{qual} dir: {dir:?} (does not exist or is not a directory)");
                }
            }

            report_dir("config", dirs.config_dir());
            report_dir("data", dirs.data_dir());
            // println!("current profile: {:?}", todo!());
        }
        ConfigCommand::Init => {
            debug!("executing `config init`");
            ensure_project_dirs(dirs).context("cannot initialize config")?
        }
    }

    Ok(())
}
