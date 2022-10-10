use egui::{TopBottomPanel, menu};
use wcore::{graphics::context::Context, egui::view::View};

use crate::state::State;

pub struct MenuView {}

impl MenuView {
    pub fn new() -> Self {
        return Self {};
    }
}

impl View<State> for MenuView {
    #[allow(unused_variables)]
    fn show(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context, ctx: &egui::Context) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open File").clicked() {
                        ui.close_menu();
                        todo!("drag & drop files instead for now");
                    }

                    if ui.button("Open Folder").clicked() {
                        ui.close_menu();
                        todo!("drag & drop files instead for now");
                    }

                    ui.menu_button("Open Recent", |ui| {
                        let mut recent = vec![]; // I hate rust
                        for project in &state.projects.recent {
                            recent.push((ui.button(&project.name), project.path.clone()));
                        }

                        for (button, path) in recent {
                            if button.clicked() {
                                state.editor.open_project(path, &mut state.projects);
                                ui.close_menu();
                            }
                        }
                    });

                    ui.separator();

                    if ui.button("Close Project").clicked() {
                        state.editor.close_project(&mut state.projects);
                        ui.close_menu();
                    }

                });
            });
        });
    }
}