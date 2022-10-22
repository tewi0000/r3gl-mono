use cgmath::{vec3, Quaternion, Zero, vec4, Vector2, vec2, MetricSpace};
use wcore::{screen::Screen, graphics::{context::Context, bindable::Bindable, texture::Texture, drawable::Drawable, scene::Scene2D, primitive::mesh::{instanced::InstancedMesh, data::{vertex::Vertex, model::{ModelRaw, Model}}}, pipeline::{model::ModelPipeline, shader::scene::SceneSlot, Pipeline}, camera::Projection, utils}, collider::collide, input::Input, app::AppState};
use winit::event::{WindowEvent, MouseButton, ElementState};

use crate::{state::State, graphics::{primitive::mesh::taiko::{Circle, CircleRaw}, pipeline::taiko::TaikoCirclePipeline}, identifier::Identifier};
use color_eyre::eyre::Result;

const OFFSET: f32 = 200.0;
const SCALE: f32 = 0.8;
const CIRCLE_SIZE: f32 = 128.0 * 0.75;
const DEAD_ZONE: f32 = 20.0;

pub struct TaikoScreen {
    pub pipeline_taiko: TaikoCirclePipeline,
    pub pipeline_field: ModelPipeline,
    pub pipeline_model: ModelPipeline,
    
    pub scene: Scene2D,
    

    
    pub mesh_circle: InstancedMesh<Circle, CircleRaw, Vertex>,
    pub mesh_model_hit: InstancedMesh<Model, ModelRaw, Vertex>,
    pub mesh_model_selection: InstancedMesh<Model, ModelRaw, Vertex>,
    pub mesh_model_selection_box: InstancedMesh<Model, ModelRaw, Vertex>,

    pub selection: Vec<usize>,
    pub selection_start: Vector2<f32>,
}

impl TaikoScreen {
    pub fn new(graphics: &Context) -> Result<Self> {
        let mesh_model_selection_box = InstancedMesh::new(&graphics.device, Vertex::vertices_rect(0.0, 1.0), vec![]);
        let mesh_model_selection = InstancedMesh::new(&graphics.device, Vertex::vertices_rect(-0.5, 0.5), vec![]);
        let mesh_circle = InstancedMesh::new(&graphics.device, Vertex::vertices_rect(-0.5, 0.5), vec![]);
        let mesh_model_hit = InstancedMesh::new(&graphics.device, Vertex::vertices_rect(-0.5, 0.5), vec![Model {
            position: vec3(OFFSET, OFFSET, 0.0),
            rotation: Quaternion::zero(),
            scale: vec3(CIRCLE_SIZE, CIRCLE_SIZE, 1.0),
            color: vec4(1.0, 1.0, 1.0, 0.5)
        }]);

        let width = graphics.surface_configuration.width;
        let height = graphics.surface_configuration.height;
        
        let scene = Scene2D::new(&graphics.device, width, height);
        let pipeline_taiko = TaikoCirclePipeline::new(&graphics.device, &graphics.surface_configuration, &scene);
        let pipeline_field = ModelPipeline::new(&graphics.device, &graphics.surface_configuration, &scene);
        let pipeline_model = ModelPipeline::new(&graphics.device, &graphics.surface_configuration, &scene);

        let selection = vec![];

        return Ok(Self {
            pipeline_taiko,
            pipeline_field,
            pipeline_model,
            scene,

            mesh_circle,
            mesh_model_hit,
            mesh_model_selection,
            mesh_model_selection_box,

            selection,
            selection_start: (0.0, 0.0,).into(),
        });
    }
}

impl Screen<State, Identifier> for TaikoScreen {
    fn render(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, view: &wgpu::TextureView) {
        utils::submit(&app.graphics.queue, &app.graphics.device, |encoder| {
            utils::render(encoder, &view, None, |mut render_pass| {
                /* Hit position */
                self.scene.camera.position.x = 0.0;                       // Manipulate camera
                self.pipeline_model.attach(&mut render_pass);             // Attach to renderpass
                self.pipeline_model.update(&app.graphics.queue, &self.scene); // Update camera (! buffred !)
                state.textures.t_hit_position.bind(&mut render_pass, 1);  // Bind texture
                self.mesh_model_hit.draw(&mut render_pass);               // Draw

                /* Circles */
                self.pipeline_taiko.attach(&mut render_pass);

                // Scene
                let time = state.editor.get_time();
                self.scene.camera.position.x = -((time as f32 * SCALE) - OFFSET);
                self.pipeline_taiko.update(&app.graphics.queue, &self.scene);
                
                // Textures
                state.textures.t_circle.bind(&mut render_pass, 1);
                state.textures.t_overlay.bind(&mut render_pass, 2);
                state.textures.t_big_circle.bind(&mut render_pass, 3);
                state.textures.t_big_overlay.bind(&mut render_pass, 4);

                // Drawing
                if let Some(beatmap) = &state.editor.beatmap {
                    self.mesh_circle.instances.clear();
                    for obj in beatmap.objects.iter().rev() {
                        if time > obj.time {
                            continue;
                        }

                        self.mesh_circle.instances.push(Circle {
                            position: vec3(obj.time as f32 * SCALE, 200.0, 0.0), 
                            rotation: Quaternion::zero(),
                            scale: if obj.big { vec3(CIRCLE_SIZE * 1.55, CIRCLE_SIZE * 1.55, 1.0) }
                                   else       { vec3(CIRCLE_SIZE       , CIRCLE_SIZE       , 1.0) },

                            color: if obj.kat { vec4(0.0, 0.47, 0.67, 1.0) }
                                   else       { vec4(0.92, 0.0, 0.27, 1.0) },

                            finisher: obj.big,
                        });
                    }

                    self.mesh_circle.bake_instances(&app.graphics.device);
                    self.mesh_circle.draw(&mut render_pass);
                }

                /* Selection */
                self.pipeline_model.attach(&mut render_pass);         // Attach to renderpass
                state.textures.t_selection_box.bind(&mut render_pass, 1);           // Bind texture
                self.mesh_model_selection_box.bake_instances(&app.graphics.device);
                self.mesh_model_selection_box.draw(&mut render_pass); // Draw

                self.pipeline_field.attach(&mut render_pass);             // Attach to renderpass
                self.pipeline_field.update(&app.graphics.queue, &self.scene); // Update camera (! buffred !)
                state.textures.t_selection.bind(&mut render_pass, 1);               // Bind texture

                // Draw
                self.mesh_model_selection.instances.clear();
                for index in self.selection.iter().rev() {
                    if let Some(obj) = self.mesh_circle.instances.get(*index) {
                        self.mesh_model_selection.instances.push(Model {
                            position: obj.position, 
                            rotation: obj.rotation,
                            scale: obj.scale,
    
                            color: vec4(1.0, 1.0, 1.0, 1.0)
                        });
                    }
                }

                self.mesh_model_selection.bake_instances(&app.graphics.device);
                self.mesh_model_selection.draw(&mut render_pass);
            });
        });
    }

    #[allow(unused_variables)]
    fn input(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, event: &WindowEvent, input: &Input) {
        #[allow(deprecated)]
        match event {
            WindowEvent::CursorMoved { device_id, position, modifiers: _ } => {
                if input.mouse_button == MouseButton::Left && input.mouse_state == ElementState::Pressed {
                    if self.selection_start.distance(vec2(position.x as f32, position.y as f32)) < DEAD_ZONE {
                        return
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
                    let offset = -((time as f32 * SCALE) - OFFSET);
                    self.selection.clear();
                    self.selection = self.mesh_circle.instances.iter().enumerate().filter_map(|(i, x)| {
                        let mut pos = x.position.truncate();
                        pos.x += offset;

                        if collide::square(selection_box.position.truncate(), selection_box.scale.truncate(), pos) {
                            Some(i)
                        } else { None }
                    }).collect();

                }
            }

            WindowEvent::MouseInput { device_id, state: button_state, button, modifiers: _ } => {
                let time = state.editor.get_time();
                let offset = -((time as f32 * SCALE) - OFFSET);
                if *button == MouseButton::Left {
                    match *button_state {
                        ElementState::Pressed => {
                            self.selection_start = input.cursor_position;
    
                            let selection = self.mesh_circle.instances.iter().rev().position(|x| {
                                collide::circle(vec2(x.position.x + offset, x.position.y), input.cursor_position, CIRCLE_SIZE / 2.0)
                            });
    
                            if let Some(reverse_index) = selection {
                                let len = self.mesh_circle.instances.len();
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

    #[allow(unused_variables)]
    fn resize(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, width: i32, height: i32) {
        self.scene.projection.resize(width as u32, height as u32);
    }
}