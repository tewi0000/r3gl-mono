use std::path::{PathBuf, Path};

use serde::{Deserialize, Serialize};

use super::project::Project;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Projects {
    #[serde(skip)]
    pub current: Option<Project>,

    #[serde(default)]
    pub recent: Vec<ProjectInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectInfo {
    pub path: PathBuf,
    pub name: String,
}

impl Default for Projects {
    fn default() -> Self {
        return Self {
            current: None,
            recent: vec![]
        };
    }
}

impl Projects {
    pub fn open(&mut self, path: impl AsRef<Path>) {
        // TODO: handle errors case properly
        let path = path.as_ref();
        let project = Project::from_path(path).unwrap();

        let recent = &mut self.recent;
        if recent.len() > 1 { // `split_at_mut()` panics otherwise
            let (first, rest) = recent.split_at_mut(1);
            // TODO: fix a bug with duplicate paths?
            let project_info = rest.iter_mut().find(|proj| proj.path == path);
            if let Some(project_info) = project_info {
                std::mem::swap(&mut first[0], project_info);
            } else {
                recent.insert(0, project.info());
            }
        } else {
            recent.insert(0, project.info());
        }

        self.current = Some(project);
    }
}