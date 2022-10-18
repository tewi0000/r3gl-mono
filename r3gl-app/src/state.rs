use std::env;

use wcore::graphics::context::Context;

use crate::{project::project_manager::ProjectManager, load_or_default, save, editor::Editor, store::texture::TextureStore};

pub struct State {
    pub textures: TextureStore,
    pub projects: ProjectManager,
    pub editor: Editor,
}

impl State {
    pub fn new(graphics: &Context) -> Self {
        let t_path = env::current_dir().unwrap().join("resources").join("textures");
        return Self {
            textures: TextureStore::from_path(t_path, graphics),
            projects: load_or_default("projects.toml"),
            editor: Editor::new(),
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        save(&self.projects, "projects.toml");
    }
}