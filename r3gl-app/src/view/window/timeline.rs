use egui::{Align2, vec2, Button, Slider};
use wcore::{graphics::context::Context, egui::window::Window};

use crate::state::State;

const OFFSET: f32 = 12.0;

pub struct TimelineWindow {
    visible: bool,

    was_playing: bool,
}

impl TimelineWindow {
    pub fn new() -> Self {
        return Self {
            visible: true,
            was_playing: false,
        };
    }
}

impl Window<&mut State> for TimelineWindow {
    type Title = &'static str;
    fn title() -> Self::Title {
        return "Timeline";
    }

    fn build<'a>(window: egui::Window<'a>, ctx: &'_ egui::Context) -> egui::Window<'a> {
        let rect = ctx.available_rect();
        let size = rect.size();

        window
            .anchor(Align2::CENTER_TOP, vec2(0.0, 96.0))
            .anchor(Align2::CENTER_BOTTOM, vec2(0.0, -OFFSET))
            .fixed_size(vec2(size.x - OFFSET * 3.0, 240.0))
            .collapsible(false)
            .title_bar(false)
    }

    fn set_visible(&mut self, value: bool) { self.visible = value; }
    fn get_visible(&self) -> bool { return self.visible; }

    #[allow(unused_variables)]
    fn show(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context, ui: &mut egui::Ui) {
        let time = state.editor.get_time().as_ms();
        let length = state.editor.get_length();
        
        ui.horizontal(|ui| {
            ui.set_enabled(state.projects.current.is_some());

            // Play button
            let play_button_text = if state.editor.is_paused() { "▶" } else { "⏸" };
            let play_button = ui.add_sized(vec2(24.0, ui.available_height()), Button::new(play_button_text));
            if play_button.clicked() {
                state.editor.toggle_paused();
            };

            // Time display
            ui.label(&format!("{:02}:{:02}:{:03} / {:02}:{:02}:{:03}",
                  time / (60 * 1000),   time / 1000 % 60,   time % 1000,
                length / (60 * 1000), length / 1000 % 60, length % 1000));

            // Time slider
            let slider_width = ui.available_width();
            let style = ui.style_mut();
            style.spacing.slider_width = slider_width;

            let mut time64 = time as u64;
            let slider = Slider::new(&mut time64, 0 ..= (length as u64)).show_value(false);
            let slider = ui.add(slider);               

            if slider.drag_started() {
                self.was_playing = state.editor.is_paused();
            }

            if slider.changed() {
                state.editor.set_paused(true);
                state.editor.set_time(time64 as u32);
            }

            if slider.drag_released() {
                if state.projects.current.is_some() {
                    state.editor.set_paused(self.was_playing);
                }
            }
        }); 
    }
}