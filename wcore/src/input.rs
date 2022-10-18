use cgmath::Vector2;
use winit::event::{ModifiersState, MouseButton, ElementState};

pub struct Input {
    pub cursor_position : Vector2<f32>,
    pub mouse_button    : MouseButton,
    pub mouse_state     : ElementState,
    pub modifiers       : ModifiersState,
}

impl Default for  Input {
    fn default() -> Self {
        return Self {
            cursor_position : (0.0, 0.0).into(),
            mouse_button    : MouseButton::Left,
            mouse_state     : ElementState::Released,
            modifiers       : ModifiersState::empty(),
        };
    }
}