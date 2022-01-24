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
