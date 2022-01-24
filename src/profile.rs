use log::{debug, trace, warn};

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
        ProfileCommand::Remove { .. } => {
            debug!("executing `profile remove`");
            todo!()
        }
    }

    Ok(())
}
