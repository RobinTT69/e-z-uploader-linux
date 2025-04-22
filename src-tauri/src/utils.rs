use crate::config;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

pub fn add_int_to_uploaded_files(app_handle: &AppHandle) {
    let mut config = config::get_config(app_handle);

    config.upload_count += 1;

    config::set_config(app_handle, config)
}

pub fn get_screenshot_dir() -> PathBuf {
    let path = if cfg!(windows) {
        home::home_dir()
            .expect("Failed to get home directory")
            .join("Pictures")
            .join("Screenshots")
    } else {
        home::home_dir()
            .expect("Failed to get home directory")
            .join("screenshots")
    };

    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create screenshot directory");
    }

    path
}
