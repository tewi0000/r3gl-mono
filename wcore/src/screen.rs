use std::collections::HashMap;

use instant::Instant;
use winit::event::{VirtualKeyCode, ModifiersState};
use crate::app::Input;
use crate::graphics::context::Context;

pub type Actions<State> = HashMap<(VirtualKeyCode, ModifiersState), Action<State>>;

#[allow(unused_variables)]
pub trait Screen<State> {
    fn update(&mut self, state: &mut State, now: Instant) { }
    fn render(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context) { }
    fn resize(&mut self, state: &mut State, graphics: &mut Context, width: i32, height: i32) { }
    fn scale(&mut self, state: &mut State, graphics: &mut Context, scale: f64) { }
    fn mouse(&mut self, state: &mut State, x_delta: f32, y_delta: f32) { }
    fn input(&mut self, state: &mut State, input: &Input) { }

    fn actions(&mut self) -> Option<&mut Actions<State>> { None }
}

pub struct Action<State> {
    pub name: String,
    pub description: String,
    function: Box<dyn FnMut(&mut State)>,
}

impl<State> Action<State> {
    pub fn new(name: String, description: String, function: impl FnMut(&mut State) + 'static) -> Self {
        return Self {
            name,
            description,
            function: Box::new(function),
        }
    }

    pub fn invoke(&mut self, state: &mut State) {
        (*self.function)(state);
    }
}