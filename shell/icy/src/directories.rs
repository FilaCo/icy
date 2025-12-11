use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use directories::ProjectDirs;

pub fn config() -> &'static Path {
    PROJECT_DIRS.config_local_dir()
}

pub fn logs() -> &'static Path {
    &LOGS_DIR
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "FilaCo", "icy").expect("unable to retrieve project dirs")
}

static PROJECT_DIRS: LazyLock<ProjectDirs> = LazyLock::new(project_dirs);
static LOGS_DIR: LazyLock<PathBuf> = LazyLock::new(|| PROJECT_DIRS.data_local_dir().join("logs"));
