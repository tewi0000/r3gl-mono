use wcore::app::AppState;
use wcore::egui::egui::EGui;
use wcore::egui::view::View;
use wcore::egui::window::Window;
use wcore::graphics::context::Context;
use color_eyre::eyre::Result;
use wcore::input::Input;
use wcore::screen::Screen;
use winit::event::WindowEvent;

use crate::identifier::Identifier;
use crate::state::State;
use crate::view::menu::MenuView;
use crate::view::window::bindings::BindingsWindow;
use crate::view::window::startup::StartupWindow;
use crate::view::window::timeline::TimelineWindow;

pub(crate) struct Windows {
    pub startup: StartupWindow,
    pub bindings: BindingsWindow,
    pub timeline: TimelineWindow,
}

pub struct EGuiScreen {
    egui: EGui,
    
    menu: MenuView,
    windows: Windows,
}

impl EGuiScreen {
    pub fn new(graphics: &Context) -> Result<Self> {
        return Ok(Self {
            egui: EGui::new(&graphics.device, &graphics.surface_configuration, graphics.scale_factor),
            
            menu: MenuView::new(),
            windows: Windows {
                startup: StartupWindow::new(),
                bindings: BindingsWindow::new(),
                timeline: TimelineWindow::new(),
            }
        });
    }
}

#[allow(unused_variables)]
impl Screen<State, Identifier> for EGuiScreen {
    fn render(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, view: &wgpu::TextureView) {
        self.egui.render(view, &mut app.graphics, |ctx: &egui::Context, graphics: &mut Context| {        
            self.windows.startup.set_visible(state.projects.current.is_none());
            
            View::show(&mut self.menu, (state, &mut self.windows), view, graphics, ctx);
            View::show(&mut self.windows.startup, state, view, graphics, ctx);
            View::show(&mut self.windows.bindings, (state, &mut app.bindings, &app.grab_key, &mut app.want_key), view, graphics, ctx);
            View::show(&mut self.windows.timeline, state, view, graphics, ctx);
        });
    }

    #[allow(unused_variables)]
    fn input(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, event: &WindowEvent, input: &Input) -> bool {
        self.egui.input(event); // Todo: pass keyboard events only
        if self.egui.egui_ctx.wants_keyboard_input() {
            return false;
        }

        match event {
            WindowEvent::DroppedFile(file) => {
                state.editor.open_project(file, &mut state.projects);
                self.windows.startup.set_visible(false);
            }

            WindowEvent::MouseWheel { device_id, delta, phase, .. } => {
                
            }

            _ => {}
        }

        if self.egui.egui_ctx.wants_pointer_input() {
            return false;
        }
     
        return true;
    }

    #[allow(unused_variables)]
    fn resize(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, width: i32, height: i32) {
        self.egui.resize(width, height);
    }

    fn identifier(&mut self) -> Identifier { Identifier::Editor }
}