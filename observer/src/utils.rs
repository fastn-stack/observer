use failure::Fail;
use std::{fs, path::Path};

use crate::Result;

#[derive(Copy, Clone, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "File Exists")]
    FileExists,
}

pub fn create_dir_all_if_not_exists(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        return fs::create_dir_all(path).map_err(|err| {
            err.context(format!("Not able to create dir {}", path))
                .into()
        });
    }
    Ok(())
}

pub fn create_dir_if_not_exists(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        return fs::create_dir(path).map_err(|err| {
            err.context(format!("Not able to create dir {}", path))
                .into()
        });
    }
    Ok(())
}

pub fn create_file(dir_path: &str, file_name: &str) -> Result<fs::File> {
    let file_path = format!("{}/{}", dir_path, file_name);
    if !Path::new(&file_path).exists() {
        fs::File::create(&file_path)
            .map_err(|err| err.context(format!("Can't open file {}", file_path)).into())
    } else {
        Err(ErrorKind::FileExists
            .context(format!("File already exists: {}", file_path))
            .into())
    }
}
