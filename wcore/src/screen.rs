use std::fmt::Display;
use std::hash::Hash;

use instant::Instant;
use winit::event::WindowEvent;
use crate::{input::Input, app::AppState};

pub trait Identifier: Hash + Clone + Copy + PartialEq + Eq + Default + Display {}

#[allow(unused_variables)]
pub trait Screen<S, I: Identifier> {
    fn update(&mut self, state: &mut S, app: &mut AppState<S, I>, now: Instant) { }
    fn render(&mut self, state: &mut S, app: &mut AppState<S, I>, view: &wgpu::TextureView) { }
    fn resize(&mut self, state: &mut S, app: &mut AppState<S, I>, width: i32, height: i32) { }
    fn scale(&mut self, state: &mut S, app: &mut AppState<S, I>, scale: f64) { }
    fn mouse(&mut self, state: &mut S, app: &mut AppState<S, I>, x_delta: f32, y_delta: f32) { }
    fn input(&mut self, state: &mut S, app: &mut AppState<S, I>, event: &WindowEvent, input: &Input) -> bool { true }

    fn identifier(&mut self) -> I { I::default() }
}