use std::collections::HashMap;
use std::hash::Hash;

use instant::Instant;
use winit::event::{VirtualKeyCode, ModifiersState, WindowEvent};
use crate::graphics::context::Context;

pub trait Identifier: Hash + Clone + Copy + PartialEq + Eq + Default {}

#[allow(unused_variables)]
pub trait Screen<S, I: Identifier> {
    fn update(&mut self, state: &mut S, now: Instant) { }
    fn render(&mut self, state: &mut S, view: &wgpu::TextureView, graphics: &mut Context) { }
    fn resize(&mut self, state: &mut S, graphics: &mut Context, width: i32, height: i32) { }
    fn scale(&mut self, state: &mut S, graphics: &mut Context, scale: f64) { }
    fn mouse(&mut self, state: &mut S, x_delta: f32, y_delta: f32) { }
    fn input(&mut self, state: &mut S, event: &WindowEvent, modifiers: ModifiersState) { }

    fn identifier(&mut self) -> I { I::default() }
}

pub type Bindings<S> = HashMap<(VirtualKeyCode, ModifiersState), Action<S>>;

pub struct Action<State> {
    pub name: String,
    pub description: String,
    function: Box<dyn FnMut(&mut State) + 'static>,
}

impl<S> Action<S> {
    pub fn new(name: String, description: String, function: impl FnMut(&mut S) + 'static) -> Self {
        return Self {
            name,
            description,
            function: Box::new(function),
        }
    }

    pub fn invoke(&mut self, state: &mut S) {
        (*self.function)(state);
    }
}