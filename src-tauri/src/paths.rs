use directories::ProjectDirs;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    if let Some(proj_dirs) = get_project_dir() {
        return proj_dirs.config_dir().to_path_buf();
    }

    PathBuf::new()
}

pub fn get_log_path() -> PathBuf {
    get_appdata_path().join("logs")
}

pub fn get_appdata_path() -> PathBuf {
    if let Some(proj_dirs) = get_project_dir() {
        if let Some(parent) = proj_dirs.config_dir().parent() {
            return parent.to_path_buf();
        }
    }

    PathBuf::new()
}

fn get_project_dir() -> Option<ProjectDirs> {
    directories::ProjectDirs::from("net", "notice_ical", "notice_ical")
}
