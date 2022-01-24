use log::{debug, info};

use anyhow::Context;

use crate::dirs::ProjectDirs;
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
            init_proj_dirs(dirs).context("cannot initialize config")?
        }
    }

    Ok(())
}

fn init_proj_dirs(dirs: &ProjectDirs) -> anyhow::Result<()> {
    debug!("{:?}", dirs);

    std::fs::create_dir(dirs.config_dir())
        .with_context(|| format!("cannot create config dir {:?}", dirs.config_dir()))?;
    std::fs::create_dir(dirs.data_dir())
        .with_context(|| format!("cannot create data dir {:?}", dirs.data_dir()))?;

    info!("created config dir: {:?}", dirs.config_dir());
    info!("created data dir: {:?}", dirs.data_dir());

    Ok(())
}
