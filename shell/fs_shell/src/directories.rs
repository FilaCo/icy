use std::{path::Path, sync::LazyLock};

use directories::ProjectDirs;

pub fn config() -> &'static Path {
    PROJECT_DIRS.config_local_dir()
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "FilaCo", "FilaCo Shell").expect("unable to retrieve project dirs")
}

static PROJECT_DIRS: LazyLock<ProjectDirs> = LazyLock::new(project_dirs);
