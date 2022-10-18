use instant::Instant;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, Event, WindowEvent, ModifiersState, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};
use crate::bindings::BindingManager;
use crate::graphics::context::Context;
use crate::screen::{Screen, Identifier};

pub struct App<'a, S, I: Identifier> {
    pub title: String,

    pub width: i32,
    pub height: i32,

    pub screens: Vec<&'a mut dyn Screen<S, I>>,
    pub bindings: BindingManager<S, I>,
}

impl<'a, S, I: Identifier> App<'a, S, I> {
    pub fn run(mut self, mut state: S, init: impl FnOnce(&mut Self, &mut Context)) {
        pollster::block_on(async {
            let mut event_loop = EventLoop::new();
            let window = create_window(&event_loop, self.width, self.height);
            let mut graphics = Context::new(&window).await.unwrap();
            init(&mut self, &mut graphics);

            let mut focused = false;
            let mut modifiers = ModifiersState::empty();
            event_loop.run_return(move |event, _, control_flow| {
                match event {
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }

                    Event::RedrawRequested(window_id) if window_id == window.id() => {
                        let now = Instant::now();
                        if let Ok(output) = graphics.surface.get_current_texture() {
                            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                
                            for screen in &mut self.screens {
                                screen.update(&mut state, now);
                                screen.render(&mut state, &view, &mut graphics);
                            }
                
                            output.present();
                        }
                    }

                    Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta, }, .. }  => {
                        if focused { self.mouse(&mut state, delta.0 as f32, delta.1 as f32) }
                    }

                    Event::WindowEvent { event, window_id } if window_id == window.id() => {
                        match event {
                            WindowEvent::Focused(is_focused) => {
                                focused = is_focused;
                            }

                            WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit;
                            }

                            WindowEvent::Resized( physical_size ) => {
                                self.resize(&mut state, &mut graphics,
                                            physical_size.width as i32,
                                            physical_size.height as i32);
                            }

                            WindowEvent::ScaleFactorChanged { scale_factor, ..  } => {
                                self.scale(&mut state, &mut graphics, scale_factor);
                            }

                            WindowEvent::ModifiersChanged(new_modifiers) => {
                                modifiers = new_modifiers;
                            }

                            dropped_file @ WindowEvent::DroppedFile(_) => { self.input(&mut state, &dropped_file, modifiers); }
                            _ => if focused { self.input(&mut state, &event, modifiers); }
                        }
                    }

                    _ => {}
                }
            });
        });
    }

    fn resize(&mut self, state: &mut S, graphics: &mut Context, width: i32, height: i32) {
        if width > 0 && height > 0 {
            self.width = width;
            self.height = height;
            graphics.surface_configuration.width = width as u32;
            graphics.surface_configuration.height = height as u32;
            graphics.surface.configure(&graphics.device, &graphics.surface_configuration);

            for screen in &mut self.screens {
                screen.resize(state, graphics, width, height);
            }
        }
    }

    fn scale(&mut self, state: &mut S, graphics: &mut Context, scale: f64) {
        graphics.scale_factor = scale;
        for screen in &mut self.screens {
            screen.scale(state, graphics, scale);
        }
    }
    
    fn mouse(&mut self, state: &mut S, x_delta: f32, y_delta: f32) {
        for screen in &mut self.screens {
            screen.mouse(state, x_delta, y_delta);
        }
    }

    fn input(&mut self, state: &mut S, event: &WindowEvent, modifiers: ModifiersState) {
        for screen in &mut self.screens {
            screen.input(state, event, modifiers);
            if let WindowEvent::KeyboardInput { input, .. } = event {
                if let Some(key) = input.virtual_keycode {
                    if let Some(bindings) = self.bindings.get_mut(&screen.identifier()) {
                        if let Some(action) = bindings.get_mut(&(key, modifiers)) {
                            if input.state == ElementState::Pressed {
                                action.invoke(state);
                            }
                        }
                    }
                }
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