use cgmath::{vec3, Quaternion, Zero, vec4};
use wcore::{screen::Screen, graphics::{context::Context, bindable::Bindable, drawable::Drawable, scene::Scene2D, primitive::mesh::{instanced::InstancedMesh, data::{vertex::Vertex, model::{ModelRaw, Model}}}, pipeline::{model::ModelPipeline, shader::scene::SceneSlot, Pipeline}, camera::Projection, utils}, input::Input, app::AppState, unit::Unit};
use winit::event::WindowEvent;

use crate::{state::State, graphics::{primitive::mesh::taiko::{Circle, CircleRaw}, pipeline::taiko::TaikoCirclePipeline}, identifier::Identifier, beatmap::component::adapter::taiko::TaikoVariantAdapter, unit::selection::SelectionUnit};
use color_eyre::eyre::Result;

pub const OFFSET: f32 = 200.0;
pub const SCALE: f32 = 0.8;
pub const CIRCLE_SIZE: f32 = 128.0 * 0.75;
pub const DEAD_ZONE: f32 = 20.0;

pub struct TaikoScreen {
    pub pipeline_taiko: TaikoCirclePipeline,
    pub pipeline_field: ModelPipeline,
    pub pipeline_model: ModelPipeline,
    
    pub scene: Scene2D,
    
    pub mesh_circle: InstancedMesh<Circle, CircleRaw, Vertex>,
    pub mesh_model_hit: InstancedMesh<Model, ModelRaw, Vertex>,

    selection_unit: SelectionUnit
}

impl TaikoScreen {
    pub fn new(graphics: &Context) -> Result<Self> {
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


        return Ok(Self {
            pipeline_taiko,
            pipeline_field,
            pipeline_model,
            scene,

            mesh_circle,
            mesh_model_hit,

            selection_unit: SelectionUnit::new(graphics),
        });
    }
}

impl Screen<State, Identifier> for TaikoScreen {
    fn render(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, view: &wgpu::TextureView) {
        utils::submit(&app.graphics.queue, &app.graphics.device, |encoder| {
            utils::render(encoder, &view, None, |mut render_pass| {
                /* Hit position */
                self.scene.camera.position.x = 0.0;                           // Manipulate camera
                self.pipeline_model.attach(&mut render_pass);                 // Attach to renderpass
                self.pipeline_model.update(&app.graphics.queue, &self.scene); // Update camera (! buffred !)
                state.textures.t_hit_position.bind(&mut render_pass, 1);      // Bind texture
                self.mesh_model_hit.draw(&mut render_pass);                   // Draw

                /* Circles */
                self.pipeline_taiko.attach(&mut render_pass);

                // Scene
                let time = state.editor.get_time();
                self.scene.camera.position.x = -((time.as_ms() as f32 * SCALE) - OFFSET);
                self.pipeline_taiko.update(&app.graphics.queue, &self.scene);

                // Textures
                state.textures.t_circle.bind(&mut render_pass, 1);
                state.textures.t_overlay.bind(&mut render_pass, 2);
                state.textures.t_big_circle.bind(&mut render_pass, 3);
                state.textures.t_big_overlay.bind(&mut render_pass, 4);

                // Drawing
                if let Some(objects) = &state.editor.hitobjects {
                    if objects[0].time().is_some()
                       && objects[0].variant().is_some() {
                        self.mesh_circle.instances.clear();
                        for obj in objects.iter().rev() {
                            let obj_time = obj.time().unwrap().0;
                            let obj_variant = obj.variant().unwrap();

                            if time > obj_time {
                                continue;
                            }

                            self.mesh_circle.instances.push(Circle {
                                position: vec3(obj_time.as_ms() as f32 * SCALE, 200.0, 0.0),
                                rotation: Quaternion::zero(),
                                scale: if obj_variant.is_big() { vec3(CIRCLE_SIZE * 1.55, CIRCLE_SIZE * 1.55, 1.0) }
                                else                           { vec3(CIRCLE_SIZE       , CIRCLE_SIZE       , 1.0) },

                                color: if obj_variant.is_kat() { vec4(0.0, 0.47, 0.67, 1.0) }
                                else                           { vec4(0.92, 0.0, 0.27, 1.0) },

                                finisher: obj_variant.is_big(),
                            });
                        }

                        self.mesh_circle.bake_instances(&app.graphics.device);
                        self.mesh_circle.draw(&mut render_pass);
                    }

                } else if !self.mesh_circle.instances.is_empty() {
                    self.mesh_circle.instances.clear();
                }

                self.pipeline_model.attach(&mut render_pass);
                self.selection_unit.render((&state.textures, &mut self.pipeline_field, &self.pipeline_model, &self.scene, &self.mesh_circle), &mut render_pass, &app.graphics);
            });
        });
    }

    #[allow(unused_variables)]
    fn input(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, event: &WindowEvent, input: &Input) -> bool {
        self.selection_unit.input((state, &self.mesh_circle), event, input);

        return true;
    }

    #[allow(unused_variables)]
    fn resize(&mut self, state: &mut State, app: &mut AppState<State, Identifier>, width: i32, height: i32) {
        self.scene.projection.resize(width as u32, height as u32);
    }
}