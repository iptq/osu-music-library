use std::path::PathBuf;

use failure::Error;

#[cfg(any(target_os = "unix", target_os = "linux"))]
pub fn create_link(from: PathBuf, to: PathBuf) -> Result<(), Error> {
    use std::os::unix::fs::symlink;
    symlink(from, to)?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn create_link(from: PathBuf, to: PathBuf) -> Result<(), Error> {
    use std::os::windows::fs::symlink_file;
    symlink_file(from, to)?;
    Ok(())
}
