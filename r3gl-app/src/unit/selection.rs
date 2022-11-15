use cgmath::{vec2, vec3, Quaternion, vec4, Vector2, MetricSpace, Zero};
use wcore::{unit::Unit, graphics::{primitive::mesh::{data::{model::{Model, ModelRaw}, vertex::Vertex}, instanced::InstancedMesh}, context::Context, pipeline::{model::ModelPipeline, Pipeline, shader::scene::SceneSlot}, scene::Scene2D, bindable::Bindable, drawable::Drawable}, collider::collide, input::Input};
use winit::event::{WindowEvent, MouseButton, ElementState};

use crate::{state::State, screen::taiko::{DEAD_ZONE, SCALE, OFFSET, CIRCLE_SIZE}, graphics::primitive::mesh::taiko::{Circle, CircleRaw}, store::texture::TextureStore};

pub struct SelectionUnit {
    pub mesh_model_selection: InstancedMesh<Model, ModelRaw, Vertex>,
    pub mesh_model_selection_box: InstancedMesh<Model, ModelRaw, Vertex>,

    pub selection: Vec<usize>,
    pub selection_start: Vector2<f32>,
}

impl SelectionUnit {
    pub fn new(graphics: &Context) -> Self {
        let mesh_model_selection_box = InstancedMesh::new(&graphics.device, Vertex::vertices_rect(0.0, 1.0), vec![]);
        let mesh_model_selection = InstancedMesh::new(&graphics.device, Vertex::vertices_rect(-0.5, 0.5), vec![]);
        let selection = vec![];

        return Self {
            mesh_model_selection,
            mesh_model_selection_box,

            selection,
            selection_start: (0.0, 0.0,).into(),
        };
    }
}

impl Unit for SelectionUnit {
    type RenderState<'a> = (&'a TextureStore, &'a ModelPipeline, &'a ModelPipeline, &'a Scene2D, &'a InstancedMesh<Circle, CircleRaw, Vertex>);
    type InputState<'a> = (&'a mut State, &'a InstancedMesh<Circle, CircleRaw, Vertex>);

    fn render<'a: 'b, 'b>(&'a mut self, (textures, pipeline_field, pipeline_model, scene, mesh_circle): Self::RenderState<'a>, render_pass: &mut wgpu::RenderPass<'b>, graphics: &Context) {
        /* Selection */
        pipeline_model.attach(render_pass);                             // Attach to renderpass
        textures.t_selection_box.bind(render_pass, 1);                  // Bind texture
        self.mesh_model_selection_box.bake_instances(&graphics.device);
        self.mesh_model_selection_box.draw(render_pass);                // Draw

        pipeline_field.attach(render_pass);            // Attach to renderpass
        pipeline_field.update(&graphics.queue, scene); // Update camera (! buffred !)
        textures.t_selection.bind(render_pass, 1);     // Bind texture

        // Draw
        self.mesh_model_selection.instances.clear();
        for index in self.selection.iter().rev() {
            if let Some(obj) = mesh_circle.instances.get(*index) {
                self.mesh_model_selection.instances.push(Model {
                    position: obj.position,
                    rotation: obj.rotation,
                    scale: obj.scale,

                    color: vec4(1.0, 1.0, 1.0, 1.0)
                });
            }
        }

        self.mesh_model_selection.bake_instances(&graphics.device);
        self.mesh_model_selection.draw(render_pass);
    }

    fn input<'a>(&mut self, (state, mesh_circle): Self::InputState<'a>, event: &WindowEvent, input: &Input) {
        #[allow(deprecated)]
        match event {
            WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                if input.mouse_button == MouseButton::Left && input.mouse_state == ElementState::Pressed {
                    if self.selection_start.distance(vec2(position.x as f32, position.y as f32)) < DEAD_ZONE {
                        return;
                    }

                    self.mesh_model_selection_box.instances.push(Model {
                        position: vec3(0.0, 0.0, 0.0),
                        rotation: Quaternion::zero(),
                        scale: vec3(0.0, 0.0, 1.0),
                        color: vec4(1.0, 1.0, 1.0, 0.2)
                    });

                    let selection_box = &mut self.mesh_model_selection_box.instances[0];
                    let size = input.cursor_position - self.selection_start;

                    match (size.x > 0.0, size.y > 0.0) {
                        (true, true) => {
                            selection_box.position = self.selection_start.extend(0.0);
                            selection_box.scale = (size).extend(0.0);
                        }

                        (true, false) => {
                            selection_box.position = vec3(self.selection_start.x, self.selection_start.y + size.y, 0.0);
                            selection_box.scale = vec3(size.x, size.y.abs(), 0.0);
                        }

                        (false, true) => {
                            selection_box.position = vec3(self.selection_start.x + size.x, self.selection_start.y, 0.0);
                            selection_box.scale = vec3(size.x.abs(), size.y, 0.0);
                        }

                        (false, false) => {
                            selection_box.position = vec3(self.selection_start.x + size.x, self.selection_start.y + size.y, 0.0);
                            selection_box.scale = vec3(size.x.abs(), size.y.abs(), 0.0);
                        }
                    }

                    let time = state.editor.get_time();
                    let offset = -((time.as_ms() as f32 * SCALE) - OFFSET);
                    self.selection.clear();
                    self.selection = mesh_circle.instances.iter().enumerate().filter_map(|(i, x)| {
                        let mut pos = x.position.truncate();
                        pos.x += offset;

                        if collide::square(selection_box.position.truncate(), selection_box.scale.truncate(), pos) {
                            Some(i)
                        } else { None }
                    }).collect();

                }
            }

            WindowEvent::MouseInput { device_id: _, state: button_state, button, modifiers: _ } => {
                let time = state.editor.get_time();
                let offset = -((time.as_ms() as f32 * SCALE) - OFFSET);
                if *button == MouseButton::Left {
                    match *button_state {
                        ElementState::Pressed => {
                            self.selection_start = input.cursor_position;

                            let selection = mesh_circle.instances.iter().rev().position(|x| {
                                collide::circle(vec2(x.position.x + offset, x.position.y), input.cursor_position, CIRCLE_SIZE / 2.0)
                            });

                            if let Some(reverse_index) = selection {
                                let len = mesh_circle.instances.len();
                                if !input.modifiers.ctrl() {
                                    self.selection.clear();
                                }

                                let i = len - reverse_index - 1;
                                if let Some(selected) = self.selection.iter().copied().position(|x| x == i) {
                                    self.selection.remove(selected);
                                } else { self.selection.push(i); }
                            } else {
                                if !input.modifiers.ctrl() {
                                    self.selection.clear();
                                }
                            }
                        }

                        ElementState::Released => {
                            self.mesh_model_selection_box.instances.clear();
                        }
                    }
                }
            },

            _ => {}
        }
    }
}