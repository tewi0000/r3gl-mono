use wcore::{graphics::{texture::Texture, scene::Scene, primitive::mesh::data::vertex::Vertex, uniform::Uniform, pipeline::{shader::scene::SceneSlot, Pipeline}, bindable::Bindable}, utils};
use wgpu::include_wgsl;

use crate::graphics::primitive::mesh::taiko::CircleRaw;

pub struct TaikoCirclePipeline {
    pipeline: wgpu::RenderPipeline,
    scene_uniform: Uniform<[[f32; 4]; 4]>
}

impl Pipeline for TaikoCirclePipeline {
    fn attach<'a, 'b: 'a>(&'b self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        self.scene_uniform.bind(render_pass, 0);
    }
}

impl TaikoCirclePipeline {
    pub fn new(device: &wgpu::Device, surface_configuration: &wgpu::SurfaceConfiguration, scene: &impl Scene) -> Self {
        let shader = device.create_shader_module(include_wgsl!("taiko.wgsl"));
        let texture_layout = Texture::default_layout(device);
        let pipeline = utils::pipeline(device, &shader, surface_configuration, &[
            scene.layout(),
            &texture_layout,
            &texture_layout,
            &texture_layout,
            &texture_layout,
        ], &[
            Vertex::describe(),
            CircleRaw::describe(),
        ], false);

        let scene_uniform = Uniform::new(device);

        return Self {
            pipeline,
            scene_uniform,
        };
    }
}

unsafe impl SceneSlot for TaikoCirclePipeline {
    fn update(&self, queue: &wgpu::Queue, scene: &impl Scene) {
        self.scene_uniform.update(queue, &scene.apply().into());
    }
}