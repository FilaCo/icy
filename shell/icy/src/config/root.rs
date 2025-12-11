use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::LookAndFeel;
use crate::util::FromRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Root {
    look_and_feel: LookAndFeel,
}

impl Root {
    pub async fn from_file(config_path: PathBuf) -> Self {}
}

impl FromRef<Root> for LookAndFeel {
    fn from_ref(value: &Root) -> Self {
        value.look_and_feel.clone()
    }
}
