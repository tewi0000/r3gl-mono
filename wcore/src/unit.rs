use instant::Instant;
use winit::event::WindowEvent;

use crate::{graphics::context::Context, input::Input};

#[allow(unused_variables)]
pub trait Unit {
    type UpdateState<'a> = ();
    type RenderState<'a> where Self: 'a = ();
    type InputState<'a> = ();
    type MouseState<'a> = ();

    fn update<'a>(&mut self, state: Self::UpdateState<'a>, now: Instant) { }
    fn render<'a: 'b, 'b>(&'a mut self, state: Self::RenderState<'a>, render_pass: &mut wgpu::RenderPass<'b>, graphics: &Context) { }
    fn input<'a>(&mut self, state: Self::InputState<'a>, event: &WindowEvent, input: &Input) { }
    fn mouse<'a>(&mut self, state: Self::MouseState<'a>, x_delta: f32, y_delta: f32) { }
}