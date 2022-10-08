use cgmath::{vec3, Quaternion, Zero, vec4};
use wcore::{screen::Screen, app::Input, graphics::{context::Context, bindable::Bindable, texture::Texture, drawable::Drawable, scene::Scene2D, primitive::mesh::{instanced::InstancedMesh, data::{vertex::Vertex, model::{ModelRaw, Model}}}, pipeline::{model::ModelPipeline, shader::scene::SceneSlot, Pipeline}}, utils};

use crate::{state::State, graphics::{primitive::mesh::taiko::{Circle, CircleRaw}, pipeline::taiko::TaikoCirclePipeline}};
use color_eyre::eyre::Result;

const OFFSET: f32 = 200.0;

pub struct TaikoScreen {
    pub pipeline_taiko: TaikoCirclePipeline,
    pub pipeline_model: ModelPipeline,
    
    pub scene: Scene2D,
    
    pub circle: Texture,
    pub overlay: Texture,
    pub big_circle: Texture,
    pub big_overlay: Texture,
    pub hit_position: Texture,
    
    pub mesh_circle: InstancedMesh<Circle, CircleRaw, Vertex>,
    pub mesh_model: InstancedMesh<Model, ModelRaw, Vertex>,
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

        let size = 128.0 * 0.75;
        let mesh_model = InstancedMesh::new(&graphics.device, vec![
            Vertex { pos: (-0.5, -0.5, 0.0).into(), uv: (0.0, 0.0).into() },
            Vertex { pos: (-0.5,  0.5, 0.0).into(), uv: (0.0, 1.0).into() },
            Vertex { pos: ( 0.5,  0.5, 0.0).into(), uv: (1.0, 1.0).into() },
            Vertex { pos: ( 0.5,  0.5, 0.0).into(), uv: (1.0, 1.0).into() },
            Vertex { pos: ( 0.5, -0.5, 0.0).into(), uv: (1.0, 0.0).into() },
            Vertex { pos: (-0.5, -0.5, 0.0).into(), uv: (0.0, 0.0).into() },
        ], vec![Model {
            position: vec3(0.0, OFFSET, 0.0),
            rotation: Quaternion::zero(),
            scale: vec3(size, size, 1.0),
            color: vec4(1.0, 1.0, 1.0, 0.5)
        }]);

        let texture      = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("taikohitcircle.png"), wgpu::FilterMode::Linear, "circle")?;
        let overlay      = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("taikohitcircleoverlay.png"), wgpu::FilterMode::Linear, "overlay")?;
        let big_texture  = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("taikobigcircle.png"), wgpu::FilterMode::Linear, "big_circle")?;
        let big_overlay  = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("taikobigcircleoverlay.png"), wgpu::FilterMode::Linear, "big_overlay")?;
        let hit_position = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("approachcircle.png"), wgpu::FilterMode::Linear, "big_overlay")?;

        let width = graphics.surface_configuration.width;
        let height = graphics.surface_configuration.height;
        
        let scene = Scene2D::new(&graphics.device, width, height);
        let pipeline_taiko = TaikoCirclePipeline::new(&graphics.device, &graphics.surface_configuration, &scene);
        let pipeline_model = ModelPipeline::new(&graphics.device, &graphics.surface_configuration, &scene);

        return Ok(Self {
            pipeline_taiko,
            pipeline_model,
            scene,

            mesh_circle,
            mesh_model,
            
            circle: texture,
            overlay,
            big_circle: big_texture,
            big_overlay,
            hit_position,
        });
    }
}

impl Screen<State> for TaikoScreen {
    fn render(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context) {
        utils::submit(&graphics.queue, &graphics.device, |encoder| {
            utils::render(encoder, &view, None, |mut render_pass| {
                let scale = 0.8;
            
                /* Hit position */
                self.pipeline_model.attach(&mut render_pass);  

                // Scene
                self.scene.camera.position.x = OFFSET;
                self.pipeline_model.update(&graphics.queue, &self.scene);

                // Textures
                self.hit_position.bind(&mut render_pass, 1);

                // Drawing
                self.mesh_model.draw(&mut render_pass);

                /* Circles */
                self.pipeline_taiko.attach(&mut render_pass);

                // Scene
                let time = state.editor.time();
                self.scene.camera.position.x = -((time as f32 * scale) - OFFSET);
                self.pipeline_taiko.update(&graphics.queue, &self.scene);
                
                // Textures
                self.circle.bind(&mut render_pass, 1);
                self.overlay.bind(&mut render_pass, 2);
                self.big_circle.bind(&mut render_pass, 3);
                self.big_overlay.bind(&mut render_pass, 4);

                // Drawing
                if let Some(beatmap) = &state.editor.beatmap {
                    self.mesh_circle.instances.clear();
                    for obj in beatmap.objects.iter().rev() {
                        if time > obj.time {
                            continue;
                        }

                        let size = 128.0 * 0.75;
                        self.mesh_circle.instances.push(Circle {
                            position: vec3(obj.time as f32 * scale, 200.0, 0.0), 
                            rotation: Quaternion::zero(),
                            scale: if obj.big { vec3(size * 1.55, size * 1.55, 1.0) }
                                   else       { vec3(size       , size       , 1.0) },

                            color: if obj.kat { vec4(0.0, 0.47, 0.67, 1.0) }
                                   else       { vec4(0.92, 0.0, 0.27, 1.0) },

                            finisher: obj.big,
                        });
                    }

                    self.mesh_circle.bake_instances(&graphics.device);
                }
                self.mesh_circle.draw(&mut render_pass);
            });
        });
    }

    #[allow(unused_variables)]
    fn input(&mut self, state: &mut State, input: &Input) {
    }

    #[allow(unused_variables)]
    fn resize(&mut self, state: &mut State, graphics: &mut Context, width: i32, height: i32) {
    }
}