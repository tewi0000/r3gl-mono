use wcore::app::Input;
use wcore::egui::egui::EGui;
use wcore::egui::view::View;
use wcore::egui::window::Window;
use wcore::graphics::context::Context;
use color_eyre::eyre::Result;
use wcore::screen::Screen;

use crate::state::State;
use crate::view::menu::MenuView;
use crate::view::window::startup::StartupWindow;
use crate::view::window::timeline::TimelineWindow;

pub struct EGuiScreen {
    egui: EGui,
    
    menu: MenuView,
    startup: StartupWindow,
    timeline: TimelineWindow,
}

impl EGuiScreen {
    pub fn new(graphics: &Context) -> Result<Self> {
        return Ok(Self {
            egui: EGui::new(&graphics.device, &graphics.surface_configuration, graphics.scale_factor),
            
            menu: MenuView::new(),
            startup: StartupWindow::new(),
            timeline: TimelineWindow::new(),
        });
    }
}

#[allow(unused_variables)]
impl Screen<State> for EGuiScreen {
    fn render(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context) {
        self.egui.render(view, graphics, |ctx: &egui::Context, graphics: &mut Context| {        
            View::show(&mut self.menu, state, view, graphics, ctx);
            View::show(&mut self.startup, state, view, graphics, ctx);
            View::show(&mut self.timeline, state, view, graphics, ctx);
        });
    }

    #[allow(unused_variables)]
    fn input(&mut self, state: &mut State, input: &Input) {
        self.egui.input(input); // Todo: pass keyboard events only
        if self.egui.egui_ctx.wants_keyboard_input() {
            return;
        }

        match input {
            Input::DroppedFile(file) => {
                state.editor.open_project(file, &mut state.projects);
                self.startup.set_visible(false);
            }

            Input::KeyboardInput { input, .. } => {
                use winit::event::ElementState;
                use winit::event::VirtualKeyCode;
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        VirtualKeyCode::Space => {
                            if input.state == ElementState::Pressed {
                                state.editor.pause();
                            }
                        }
    
                        _ => {}
                    }
                }

            }

            Input::MouseWheel { device_id, delta, phase, .. } => {
                
            }

            _ => {}
        }
    }

    #[allow(unused_variables)]
    fn resize(&mut self, state: &mut State, graphics: &mut Context, width: i32, height: i32) {
        self.egui.resize(width, height);
    }
}