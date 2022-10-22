use egui::{TopBottomPanel, menu};
use wcore::{graphics::context::Context, egui::{view::View, window::Window}};

use crate::{state::State, screen::egui::Windows};

pub struct MenuView {}

impl MenuView {
    pub fn new() -> Self {
        return Self {};
    }
}

impl View<(&mut State, &mut Windows)> for MenuView {
    #[allow(unused_variables)]
    fn show(&mut self, (state, windows): (&mut State, &mut Windows), view: &wgpu::TextureView, graphics: &mut Context, ctx: &egui::Context) {
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
                                ui.close_menu();

                                state.editor.open_project(path, &mut state.projects);
                            }
                        }
                    });

                    ui.separator();

                    if ui.button("Close Project").clicked() {
                        ui.close_menu();

                        state.editor.close_project(&mut state.projects);
                    }

                });

                ui.menu_button("Prefrences", |ui| {
                    if ui.button("Bindings").clicked() {
                        ui.close_menu();
                        
                        windows.bindings.set_visible(true);
                    }
                });
            });
        });
    }
}