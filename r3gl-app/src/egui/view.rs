use wcore::graphics::context::Context;

use crate::state::State;

pub trait View {
    fn show(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context, ctx: &egui::Context);
}