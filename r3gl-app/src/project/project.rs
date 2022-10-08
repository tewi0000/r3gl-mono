use std::path::{PathBuf, Path};
use color_eyre::eyre::Result;

use super::projects::ProjectInfo;

#[derive(Debug, Clone)]
pub struct Project {
    pub(crate) path: PathBuf,
}

impl Project {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        return Ok(Project {
            path: path.as_ref().to_owned(),
        });
    }

    pub fn info(&self) -> ProjectInfo {
        return ProjectInfo {
            path: self.path.clone(),
            name: self.path.file_name().unwrap().to_os_string().into_string().unwrap()
        };
    }
}