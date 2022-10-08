use egui::{menu, TopBottomPanel};
use wcore::app::Input;
use wcore::egui::EGui;
use wcore::graphics::context::Context;
use color_eyre::eyre::Result;
use wcore::screen::Screen;

use crate::egui::view::View;
use crate::egui::window::Window;
use crate::state::State;
use crate::windows::startup::StartupWindow;
use crate::windows::timeline::TimelineWindow;

pub struct EGuiScreen {
    egui: EGui,
    
    startup: StartupWindow,
    timeline: TimelineWindow,
}

impl EGuiScreen {
    pub fn new(graphics: &Context) -> Result<Self> {
        return Ok(Self {
            egui: EGui::new(&graphics.device, &graphics.surface_configuration, graphics.scale_factor),
            
            startup: StartupWindow::new(),
            timeline: TimelineWindow::new(),
        });
    }
}

#[allow(unused_variables)]
impl Screen<State> for EGuiScreen {
    fn render(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context) {
        self.egui.render(view, graphics, |ctx: &egui::Context, graphics: &mut Context| {        
            { // Menu bar
                TopBottomPanel::top("menu").show(ctx, |ui| {
                    menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Open File").clicked() {
                                ui.close_menu();
                                todo!("drag & drop files instead for now");
                            }

                            if ui.button("Open Folder").clicked() {
                                ui.close_menu();
                                todo!("drag & drop files instead for now");
                            }

                            ui.menu_button("Open Recent", |ui| {
                                let mut recent = vec![]; // I hate rust
                                for project in &state.projects.recent {
                                    recent.push((ui.button(&project.name), project.path.clone()));
                                }

                                for (button, path) in recent {
                                    if button.clicked() {
                                        state.editor.open_project(path, &mut state.projects, &state.player);
                                        ui.close_menu();
                                    }
                                }
                            });

                            ui.separator();

                            if ui.button("Close Project").clicked() {
                                state.editor.close_project(&mut state.projects, &mut state.player);
                                self.startup.set_visible(true);
                                ui.close_menu();
                            }

                        });
                    });
                });
            }

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
                state.editor.open_project(file, &mut state.projects, &state.player);
                self.startup.set_visible(false);
            }

            Input::KeyboardInput { input, .. } => {
                use winit::event::ElementState;
                use winit::event::VirtualKeyCode;
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        VirtualKeyCode::Space => {
                            if input.state == ElementState::Pressed {
                                state.editor.pause(&mut state.player);
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