use cgmath::{vec3, Quaternion, Zero, vec4, Vector2, vec2};
use wcore::{screen::Screen, graphics::{context::Context, bindable::Bindable, texture::Texture, drawable::Drawable, scene::Scene2D, primitive::mesh::{instanced::InstancedMesh, data::{vertex::Vertex, model::{ModelRaw, Model}}}, pipeline::{model::ModelPipeline, shader::scene::SceneSlot, Pipeline}, camera::Projection}, utils, collider::collide};
use winit::event::{WindowEvent, MouseButton, ElementState, ModifiersState};

use crate::{state::State, graphics::{primitive::mesh::taiko::{Circle, CircleRaw}, pipeline::taiko::TaikoCirclePipeline}, identifier::Identifier};
use color_eyre::eyre::Result;

const OFFSET: f32 = 200.0;
const SCALE: f32 = 0.8;
const CIRCLE_SIZE: f32 = 128.0 * 0.75;

pub struct TaikoScreen {
    pub pipeline_taiko: TaikoCirclePipeline,
    pub pipeline_field: ModelPipeline,
    pub pipeline_model: ModelPipeline,
    
    pub scene: Scene2D,
    
    pub t_circle: Texture,
    pub t_overlay: Texture,
    pub t_big_circle: Texture,
    pub t_big_overlay: Texture,
    pub t_hit_position: Texture,
    pub t_selection: Texture,
    
    pub mesh_circle: InstancedMesh<Circle, CircleRaw, Vertex>,
    pub mesh_model_hit: InstancedMesh<Model, ModelRaw, Vertex>,
    pub mesh_model_selection: InstancedMesh<Model, ModelRaw, Vertex>,

    pub cursor: Vector2<f32>,

    pub selection: Vec<usize>,
}

impl TaikoScreen {
    pub fn new(graphics: &Context) -> Result<Self> {
        let mesh_circle = InstancedMesh::new(&graphics.device, vec![
            Vertex { pos: (-0.5, -0.5, 0.0).into(), uv: (0.0, 0.0).into() },
            Vertex { pos: (-0.5,  0.5, 0.0).into(), uv: (0.0, 1.0).into() },
            Vertex { pos: ( 0.5,  0.5, 0.0).into(), uv: (1.0, 1.0).into() },
            Vertex { pos: ( 0.5,  0.5, 0.0).into(), uv: (1.0, 1.0).into() },
            Vertex { pos: ( 0.5, -0.5, 0.0).into(), uv: (1.0, 0.0).into() },
            Vertex { pos: (-0.5, -0.5, 0.0).into(), uv: (0.0, 0.0).into() },
        ], vec![]);

        let mesh_model_hit = InstancedMesh::new(&graphics.device, vec![
            Vertex { pos: (-0.5, -0.5, 0.0).into(), uv: (0.0, 0.0).into() },
            Vertex { pos: (-0.5,  0.5, 0.0).into(), uv: (0.0, 1.0).into() },
            Vertex { pos: ( 0.5,  0.5, 0.0).into(), uv: (1.0, 1.0).into() },
            Vertex { pos: ( 0.5,  0.5, 0.0).into(), uv: (1.0, 1.0).into() },
            Vertex { pos: ( 0.5, -0.5, 0.0).into(), uv: (1.0, 0.0).into() },
            Vertex { pos: (-0.5, -0.5, 0.0).into(), uv: (0.0, 0.0).into() },
        ], vec![Model {
            position: vec3(OFFSET, OFFSET, 0.0),
            rotation: Quaternion::zero(),
            scale: vec3(CIRCLE_SIZE, CIRCLE_SIZE, 1.0),
            color: vec4(1.0, 1.0, 1.0, 0.5)
        }]);

        let mesh_model_selection = InstancedMesh::new(&graphics.device, vec![
            Vertex { pos: (-0.5, -0.5, 0.0).into(), uv: (0.0, 0.0).into() },
            Vertex { pos: (-0.5,  0.5, 0.0).into(), uv: (0.0, 1.0).into() },
            Vertex { pos: ( 0.5,  0.5, 0.0).into(), uv: (1.0, 1.0).into() },
            Vertex { pos: ( 0.5,  0.5, 0.0).into(), uv: (1.0, 1.0).into() },
            Vertex { pos: ( 0.5, -0.5, 0.0).into(), uv: (1.0, 0.0).into() },
            Vertex { pos: (-0.5, -0.5, 0.0).into(), uv: (0.0, 0.0).into() },
        ], vec![]);

        let t_circle       = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("taikohitcircle.png"), wgpu::FilterMode::Linear, "circle")?;
        let t_overlay      = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("taikohitcircleoverlay.png"), wgpu::FilterMode::Linear, "overlay")?;
        let t_big_circle   = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("taikobigcircle.png"), wgpu::FilterMode::Linear, "big_circle")?;
        let t_big_overlay  = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("taikobigcircleoverlay.png"), wgpu::FilterMode::Linear, "big_overlay")?;
        let t_hit_position = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("approachcircle.png"), wgpu::FilterMode::Linear, "big_overlay")?;
        let t_selection    = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("selection.png"), wgpu::FilterMode::Linear, "selection")?;

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
            
            t_circle,
            t_overlay,
            t_big_circle,
            t_big_overlay,
            t_hit_position,
            t_selection,

            cursor: (0.0, 0.0).into(),

            selection,
        });
    }
}

impl Screen<State, Identifier> for TaikoScreen {
    fn render(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context) {
        utils::submit(&graphics.queue, &graphics.device, |encoder| {
            utils::render(encoder, &view, None, |mut render_pass| {
                /* Hit position */
                self.scene.camera.position.x = 0.0;                       // Manipulate camera
                self.pipeline_model.attach(&mut render_pass);             // Attach to renderpass
                self.pipeline_model.update(&graphics.queue, &self.scene); // Update camera (! buffred !)
                self.t_hit_position.bind(&mut render_pass, 1);            // Bind texture
                self.mesh_model_hit.draw(&mut render_pass);               // Draw

                /* Circles */
                self.pipeline_taiko.attach(&mut render_pass);

                // Scene
                let time = state.editor.get_time();
                self.scene.camera.position.x = -((time as f32 * SCALE) - OFFSET);
                self.pipeline_taiko.update(&graphics.queue, &self.scene);
                
                // Textures
                self.t_circle.bind(&mut render_pass, 1);
                self.t_overlay.bind(&mut render_pass, 2);
                self.t_big_circle.bind(&mut render_pass, 3);
                self.t_big_overlay.bind(&mut render_pass, 4);

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

                    self.mesh_circle.bake_instances(&graphics.device);
                    self.mesh_circle.draw(&mut render_pass);
                }

                /* Selection */
                self.pipeline_field.attach(&mut render_pass);             // Attach to renderpass
                self.pipeline_field.update(&graphics.queue, &self.scene); // Update camera (! buffred !)
                self.t_selection.bind(&mut render_pass, 1);               // Bind texture

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

                self.mesh_model_selection.bake_instances(&graphics.device);
                self.mesh_model_selection.draw(&mut render_pass);
            });
        });
    }

    #[allow(unused_variables)]
    fn input(&mut self, state: &mut State, input: &WindowEvent, modifiers: ModifiersState) {
        #[allow(deprecated)]
        match input {
            WindowEvent::CursorMoved { device_id, position, modifiers: _ } => {
                self.cursor.x = position.x as f32;
                self.cursor.y = position.y as f32;
            }

            WindowEvent::MouseInput { device_id, state: button_state, button, modifiers: _ } => {
                let time = state.editor.get_time();
                let offset = -((time as f32 * SCALE) - OFFSET);
                if *button == MouseButton::Left && *button_state == ElementState::Pressed {
                    let selection = self.mesh_circle.instances.iter().rev().position(|x| {
                        collide::circle(vec2(x.position.x + offset, x.position.y), self.cursor, CIRCLE_SIZE / 2.0)
                    });

                    if let Some(reverse_index) = selection {
                        let len = self.mesh_circle.instances.len();
                        if !modifiers.ctrl() {
                            self.selection.clear();
                        }

                        let i = len - reverse_index - 1;
                        if let Some(selected) = self.selection.iter().copied().position(|x| x == i) {
                                 self.selection.remove(selected);
                        } else { self.selection.push(i); }

                    }
                }
            },

            _ => {}
        }
    }

    #[allow(unused_variables)]
    fn resize(&mut self, state: &mut State, graphics: &mut Context, width: i32, height: i32) {
        self.scene.projection.resize(width as u32, height as u32);
    }
}