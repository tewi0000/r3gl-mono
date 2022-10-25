use std::{path::{PathBuf, Path}, fs};

use serde::{Deserialize, Serialize};

use crate::{beatmap::{parser::osu_taiko::TaikoCircle, beatmap::Beatmap}};

use super::project::Project;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectManager {
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

impl Default for ProjectManager {
    fn default() -> Self {
        return Self {
            current: None,
            recent: vec![]
        };
    }
}

impl ProjectManager {
    pub fn open(&mut self, path: impl AsRef<Path>) -> (Beatmap, Vec<TaikoCircle>) {
        // TODO: handle errors case properly
        let path = path.as_ref();
        let data = fs::read_to_string(&path).unwrap();
        let (beatmap, objects) = TaikoCircle::parse(&data);
        let project = Project::from_path(path, format!("{} - {}", &beatmap.artist, &beatmap.title)).unwrap();


        let recent = &mut self.recent;
        if recent.len() > 1 { // `split_at_mut()` panics otherwise
            let (first, rest) = recent.split_at_mut(1);
            let project_info = rest.iter_mut().find(|proj| proj.path == path);
            if let Some(project_info) = project_info {
                std::mem::swap(&mut first[0], project_info);
            } else {
                if first[0].path != path {
                    recent.insert(0, project.info());
                }
            }
        } else {
            if !recent.iter().any(|proj| proj.path == path) {
                recent.insert(0, project.info());
            }
        }

        self.current = Some(project);

        return (beatmap, objects);
    }
}