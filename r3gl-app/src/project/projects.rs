use std::{path::{PathBuf, Path}, fs};

use serde::{Deserialize, Serialize};

use crate::beatmap::{parser::osu_taiko::OsuTaikoParser, beatmap::Beatmap, taiko::hitobject::HitObject};

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
    pub fn open(&mut self, path: impl AsRef<Path>) -> Beatmap<HitObject> {
        // TODO: handle errors case properly
        let path = path.as_ref();
        let data = fs::read_to_string(&path).unwrap();
        let beatmap = OsuTaikoParser::parse(&data);
        let project = Project::from_path(path, format!("{} - {}", &beatmap.artist, &beatmap.title)).unwrap();


        let recent = &mut self.recent;
        if recent.len() > 1 { // `split_at_mut()` panics otherwise
            let (first, rest) = recent.split_at_mut(1);
            let project_info = rest.iter_mut().find(|proj| proj.path == path);
            if let Some(project_info) = project_info {
                std::mem::swap(&mut first[0], project_info);
            } else {
                recent.insert(0, project.info());
            }
        } else {
            if !recent.iter().any(|proj| proj.path == path) {
                recent.insert(0, project.info());
            }
        }

        self.current = Some(project);

        return beatmap;
    }
}