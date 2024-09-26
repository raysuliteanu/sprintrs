use std::path::PathBuf;

use color_eyre::eyre::{self};
use directories::ProjectDirs;
use lazy_static::lazy_static;

use crate::common;

lazy_static! {
    static ref DATA_DIR_ENV_VAR: String = String::from(common::APP_NAME) + "DATA";
    static ref CONF_DIR_ENV_VAR: String = String::from(common::APP_NAME) + "CONFIG";
}

pub fn get_data_dir() -> eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var(DATA_DIR_ENV_VAR.as_str()) {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("com", common::ORG_NAME, common::APP_NAME) {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        return Err(eyre::eyre!(
            "Unable to find data directory for {}",
            common::APP_NAME
        ));
    };
    Ok(directory)
}

pub fn get_config_dir() -> eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var(CONF_DIR_ENV_VAR.as_str()) {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("com", common::ORG_NAME, common::APP_NAME) {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(eyre::eyre!(
            "Unable to find config directory for {}",
            common::APP_NAME
        ));
    };
    Ok(directory)
}
