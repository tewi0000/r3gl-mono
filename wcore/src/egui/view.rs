use crate::graphics::context::Context;

pub trait View<State> {
    fn show(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context, ctx: &egui::Context);
}