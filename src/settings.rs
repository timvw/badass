use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Models {
    pub location: PathBuf,
}

impl Default for Models {
    fn default() -> Self {
        Models {
            location: PathBuf::from("./demo/models"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Output {
    pub location: PathBuf,
}

impl Default for Output {
    fn default() -> Self {
        Output {
            location: PathBuf::from("./target/compiled"),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    pub models: Models,
    pub output: Output,
}

impl Settings {
    pub fn from_path(path: &Path) -> Self {
        Settings {
            models: Models {
                location: PathBuf::from(&path).join("models"),
            },
            ..Default::default()
        }
    }
}
