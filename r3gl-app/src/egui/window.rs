use egui::WidgetText;
use wcore::graphics::context::Context;

use crate::state::State;

use super::view::View;

pub trait Window: View {
    type Title: Into<WidgetText>;
    fn title() -> Self::Title;

    #[allow(unused_variables)]
    fn build<'a>(window: egui::Window<'a>, ctx: &'_ egui::Context) -> egui::Window<'a> { window }

    fn set_visible(&mut self, value: bool);
    fn get_visible(&self) -> bool;

    #[allow(unused_variables)]
    fn show(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context, ui: &mut egui::Ui);
}

impl<T: Window> View for T  {
    fn show(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context, ctx: &egui::Context) {
        let mut show_startup = self.get_visible();
        Self::build(egui::Window::new(Self::title()), ctx)
          .open(&mut show_startup)
          .show(ctx, |ui| {
            Window::show(self, state, view, graphics, ui);
        });

        self.set_visible(self.get_visible() && show_startup);
    }
}