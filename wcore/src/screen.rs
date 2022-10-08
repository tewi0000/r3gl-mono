use instant::Instant;
use crate::app::Input;
use crate::graphics::context::Context;

#[allow(unused_variables)]
pub trait Screen<State> {
    fn update(&mut self, state: &mut State, now: Instant) { }
    fn render(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context) { }
    fn resize(&mut self, state: &mut State, graphics: &mut Context, width: i32, height: i32) { }
    fn scale(&mut self, state: &mut State, graphics: &mut Context, scale: f64) { }
    fn mouse(&mut self, state: &mut State, x_delta: f32, y_delta: f32) { }
    fn input(&mut self, state: &mut State, input: &Input) { }
}