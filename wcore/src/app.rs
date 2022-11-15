use crossbeam::channel::{Receiver, bounded};
use instant::Instant;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, Event, WindowEvent, ElementState, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};
use crate::bindings::{BindingManager, KeyCombination};
use crate::graphics::context::Context;
use crate::input::Input;
use crate::screen::{Screen, Identifier};

pub struct AppState<S, I: Identifier> {
    pub bindings: BindingManager<S, I>,
    pub grab_key: Receiver<KeyCombination>,
    pub want_key: bool,
    pub graphics: Context,
}

pub struct App<'a, S, I: Identifier> {
    pub title: String,
    
    pub width: i32,
    pub height: i32,
    
    pub screens: Vec<&'a mut dyn Screen<S, I>>,
}

impl<'a, S, I: Identifier> App<'a, S, I> {
    pub fn run(mut self, state: impl FnOnce(&mut Context) -> S, init: impl FnOnce(&mut Self, &mut AppState<S, I>)) {
        pollster::block_on(async {
            let mut event_loop = EventLoop::new();
            let window = create_window(&event_loop, self.width, self.height);
            let mut graphics = Context::new(&window).await.unwrap();
            
            let (sender, receiver) = bounded(1);

            let mut state = state(&mut graphics);
            let mut app_state = AppState {
                bindings: Default::default(),
                grab_key: receiver,
                want_key: false,
                graphics: graphics
            };
            
            init(&mut self, &mut app_state);

            let mut focused = false;
            let mut input_data = Input::default();
            event_loop.run_return(move |event, _, control_flow| {
                match event {
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }

                    Event::RedrawRequested(window_id) if window_id == window.id() => {
                        let now = Instant::now();
                        if let Ok(output) = app_state.graphics.surface.get_current_texture() {
                            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                
                            for screen in &mut self.screens {
                                screen.update(&mut state, &mut app_state, now);
                                screen.render(&mut state, &mut app_state, &view);
                            }
                
                            output.present();
                        }
                    }

                    Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta, }, .. }  => {
                        if focused { self.mouse(&mut state, &mut app_state, delta.0 as f32, delta.1 as f32) }
                    }

                    #[allow(deprecated)]
                    #[allow(unused_variables)]
                    Event::WindowEvent { event, window_id } if window_id == window.id() => {
                        match event {
                            ref event @ WindowEvent::CursorMoved { device_id, position, modifiers } => {
                                input_data.cursor_position = (position.x as f32, position.y as f32).into();
                                self.input(&mut state, &mut app_state, &event, &mut input_data);
                            }

                            ref event @ WindowEvent::MouseInput { device_id, state: mouse_state, button, modifiers } => {
                                input_data.mouse_button = button;
                                input_data.mouse_state = mouse_state;
                                self.input(&mut state, &mut app_state, &event, &mut input_data);
                            }

                            WindowEvent::Focused(is_focused) => {
                                focused = is_focused;
                            }

                            WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit;
                            }

                            WindowEvent::Resized( physical_size ) => {
                                self.resize(&mut state, &mut app_state, physical_size.width as i32, physical_size.height as i32);
                            }

                            WindowEvent::ScaleFactorChanged { scale_factor, ..  } => {
                                self.scale(&mut state, &mut app_state, scale_factor);
                            }

                            WindowEvent::ModifiersChanged(new_modifiers) => {
                                input_data.modifiers = new_modifiers;
                            }

                            ref event @ WindowEvent::KeyboardInput { device_id, input, is_synthetic } => if focused {
                                if app_state.want_key { // Used for stuff like binds
                                    if let Some(vkeycode) = input.virtual_keycode {
                                        // TODO: use our own input, quick fix
                                        #[allow(unused_must_use)]
                                        if vkeycode != VirtualKeyCode::LShift   && vkeycode != VirtualKeyCode::RShift
                                        && vkeycode != VirtualKeyCode::LAlt     && vkeycode != VirtualKeyCode::RAlt
                                        && vkeycode != VirtualKeyCode::LControl && vkeycode != VirtualKeyCode::RControl {
                                            sender.try_send(KeyCombination::from((vkeycode, input_data.modifiers)));
                                            app_state.want_key = false;
                                        }
                                    }
                                }

                                self.input(&mut state, &mut app_state, &event, &mut input_data);
                            }

                            event @ WindowEvent::DroppedFile(_) => {
                                self.input(&mut state, &mut app_state, &event, &mut input_data);
                            }

                            _ => if focused { self.input(&mut state, &mut app_state, &event, &mut input_data); }
                        }
                    }

                    _ => {}
                }
            });
        });
    }

    fn resize(&mut self, state: &mut S, app: &mut AppState<S, I>, width: i32, height: i32) {
        if width > 0 && height > 0 {
            self.width = width;
            self.height = height;
            let graphics = &mut app.graphics;
            graphics.surface_configuration.width = width as u32;
            graphics.surface_configuration.height = height as u32;
            graphics.surface.configure(&graphics.device, &graphics.surface_configuration);

            for screen in &mut self.screens {
                screen.resize(state, app, width, height);
            }
        }
    }

    fn scale(&mut self, state: &mut S, app: &mut AppState<S, I>, scale: f64) {
        app.graphics.scale_factor = scale;
        for screen in &mut self.screens {
            screen.scale(state, app, scale);
        }
    }
    
    fn mouse(&mut self, state: &mut S, app: &mut AppState<S, I>, x_delta: f32, y_delta: f32) {
        for screen in &mut self.screens {
            screen.mouse(state, app, x_delta, y_delta);
        }
    }

    fn input(&mut self, state: &mut S, app: &mut AppState<S, I>, event: &WindowEvent, input: &mut Input) {
        let mut input_blocked = false;
        for screen in self.screens.iter_mut().rev() {
            if let WindowEvent::KeyboardInput { input: key_input, .. } = event
            && let Some(key) = key_input.virtual_keycode && let Some(bindings) = app.bindings.get_mut(&screen.identifier())
            && let Some(action) = bindings.get_mut(&KeyCombination::from((key, input.modifiers))) {
                if key_input.state == ElementState::Pressed {
                    action.invoke(state);
                }
            }

            if !input_blocked && !screen.input(state, app, event, input) {
                input.mouse_state = ElementState::Released;
                input_blocked = true;
            }
        }
    }
}

fn create_window(event_loop: &EventLoop<()>, width: i32, height: i32) -> Window {
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();

    return window;
}