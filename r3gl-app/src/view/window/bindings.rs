use crossbeam::channel::{Receiver};
use egui::RichText;
use wcore::{graphics::context::Context, egui::window::Window, bindings::{BindingManager, KeyCombination}};

use crate::{state::State, identifier::Identifier};

pub struct BindingsWindow {
    visible: bool,
    editing: Option<(Identifier, KeyCombination)>,
}

impl BindingsWindow {
    pub fn new() -> Self {
        return Self {
            visible: false,
            editing: None,
        };
    }
}

impl Window<(&mut State, &mut BindingManager<State, Identifier>, &Receiver<KeyCombination>, &mut bool)> for BindingsWindow {
    type Title = &'static str;
    fn title() -> Self::Title {
        return "Bindings";
    }

    #[allow(unused_variables)]
    fn build<'a>(window: egui::Window<'a>, ctx: &'_ egui::Context) -> egui::Window<'a> {
        window
            .default_pos([64.0, 64.0])
            .default_size([220.0, 260.0])
            .collapsible(true)
            .resizable(true)
            .title_bar(true)
    }

    fn set_visible(&mut self, value: bool) { self.visible = value; }
    fn get_visible(&self) -> bool { return self.visible; }

    #[allow(unused_variables)]
    fn show(&mut self, (state, binding_manager, grab_key, want_key): (&mut State, &mut BindingManager<State, Identifier>, &Receiver<KeyCombination>, &mut bool), view: &wgpu::TextureView, graphics: &mut Context, ui: &mut egui::Ui) {
        if let Ok(new) = grab_key.try_recv() {
        if let Some((identifier, old)) = self.editing {
        if let Some(category) = binding_manager.get_mut(&identifier) {
            if let Some(value) = category.remove(&old) {
                category.insert(new, value);
            }
        } } }

        for (category, bindings) in binding_manager.iter_mut() {
            ui.label(RichText::new(category.to_string()).heading().strong());
            for (bind, action) in bindings.iter_mut() {


                egui::Grid::new("bindings_grid")
                  .num_columns(2)
                  .spacing([40.0, 4.0])
                  .striped(true)
                  .show(ui, |ui| {
                    ui.label(&action.name);
                    ui.vertical_centered_justified(|ui| {
                        if *want_key {
                            if ui.button("press any key").clicked() {
                                *want_key = false;
                            }
                        } else {
                            if ui.button(&format!("{}", bind)).clicked() {
                                *want_key = true;
                                self.editing = Some((*category, *bind));
                            }
                        }
                    });

                    ui.end_row();
                });
            }
        }

        ui.allocate_space(ui.available_size());
    }
}