use anyhow::Context;
use log::{debug, info};

pub use directories::ProjectDirs;

pub fn project_dirs() -> anyhow::Result<ProjectDirs> {
    if let Some(dirs) = ProjectDirs::from("", "", clap::crate_name!()) {
        log::trace!("project dirs: {dirs:#?}");
        Ok(dirs)
    } else {
        log::warn!("cannot determine the home directory");
        anyhow::bail!("cannot obtain project directories names");
    }
}

pub fn ensure_project_dirs(dirs: &ProjectDirs) -> anyhow::Result<()> {
    debug!("{:?}", dirs);
    std::fs::create_dir_all(dirs.config_dir())
        .with_context(|| format!("cannot create config dir {:?}", dirs.config_dir()))?;
    info!("created config dir: {:?}", dirs.config_dir());
    std::fs::create_dir_all(dirs.data_dir())
        .with_context(|| format!("cannot create data dir {:?}", dirs.data_dir()))?;
    info!("created data dir: {:?}", dirs.data_dir());
    Ok(())
}
