use std::{fs, path::PathBuf};

#[derive(Clone)]
pub struct AppPaths {
    pub cache: PathBuf,
    pub config: PathBuf,
    pub data: PathBuf,
}

#[must_use]
pub fn get_app_paths() -> AppPaths {
    let project_dir = directories::ProjectDirs::from("app", "wiremann", "wiremann")
        .expect("Couldn't get application paths");

    let cache = project_dir.cache_dir().to_path_buf();
    let config = project_dir.config_dir().to_path_buf();
    let data = project_dir.data_dir().to_path_buf();

    AppPaths {
        cache,
        config,
        data,
    }
}

pub fn ensure_app_paths(app_paths: &AppPaths) {
    fs::create_dir_all(app_paths.cache.as_path()).expect("failed to create cache directory");

    fs::create_dir_all(app_paths.config.as_path()).expect("failed to create cache directory");

    fs::create_dir_all(app_paths.data.as_path()).expect("failed to create cache directory");
}
