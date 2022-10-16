use std::path::{PathBuf, Path};
use color_eyre::eyre::Result;

use super::project_manager::ProjectInfo;

#[derive(Debug, Clone)]
pub struct Project {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
}

impl Project {
    pub fn from_path(path: impl AsRef<Path>, name: String) -> Result<Self> {
        return Ok(Project {
            path: path.as_ref().to_owned(),
            name,
        });
    }

    pub fn info(&self) -> ProjectInfo {
        return ProjectInfo {
            path: self.path.clone(),
            name: self.name.clone(),
        };
    }
}