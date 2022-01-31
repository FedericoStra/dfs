use log::{debug, info, trace, warn};

use anyhow::Context;

use crate::dirs::ProjectDirs;
use crate::ProfileCommand;

pub(crate) fn run(cmd: ProfileCommand, dirs: &ProjectDirs) -> anyhow::Result<()> {
    match cmd {
        ProfileCommand::List => {
            debug!("executing `profile list`");
            for entry in std::fs::read_dir(dirs.data_dir())? {
                match entry {
                    Ok(entry) => {
                        trace!("analyzing {entry:?}");
                        let file_type = entry.file_type()?;
                        let path = entry.path();
                        let file_name = path
                            .file_name()
                            .expect("unexpected path terminating in `..`")
                            .to_string_lossy();
                        if file_type.is_dir() {
                            println!("{file_name}: {path:?}",);
                        } else {
                            warn!("{:?} is not a directory", entry.path());
                        }
                    }
                    Err(e) => {
                        warn!("{}", e)
                    }
                }
            }
        }
        ProfileCommand::Select { .. } => {
            debug!("executing `profile select`");
            todo!()
        }
        ProfileCommand::New { name } => {
            debug!("executing `profile new`");
            let data_dir = dirs.data_dir();
            std::fs::create_dir_all(data_dir)
                .with_context(|| format!("cannot create data dir {:?}", data_dir))?;
            let profile_dir = data_dir.join(&name);
            std::fs::create_dir(&profile_dir)
                .with_context(|| format!("cannot create profile dir {:?}", profile_dir))?;
            std::process::Command::new("git")
                .arg("init")
                .current_dir(&profile_dir)
                .status()?;
            info!("new profile created at {:?}", profile_dir);
        }
        ProfileCommand::Remove { name, dry_run } => {
            debug!("executing `profile remove`");
            let profile_dir = dirs.data_dir().join(&name);
            if dry_run {
                println!(
                    "This would remove the profile {:?} located at {:?}",
                    name, profile_dir
                );
                info!("would remove profile {:?} at {:?}", name, profile_dir);
            } else {
                std::fs::remove_dir_all(&profile_dir)
                    .with_context(|| format!("cannot remove profile dir {:?}", profile_dir))?;
                info!("profile {:?} removed from {:?}", name, profile_dir);
            }
        }
    }

    Ok(())
}
