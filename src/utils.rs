use std::{
    fs,
    path::Path,
};
use failure::Fail;


use crate::Result;

pub fn create_dir_if_not_exists(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        return fs::create_dir(path)
            .map_err(|err|
                err.context(format!("Not able to create dir {}", path)).into()
            )
    }
    Ok(())
}