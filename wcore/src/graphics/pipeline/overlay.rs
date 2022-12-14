use wgpu::include_wgsl;

use crate::{graphics::{texture::Texture, scene::Scene, primitive::mesh::data::{vertex::Vertex, model::ModelRaw}, uniform::Uniform, bindable::Bindable, utils}};

use super::{shader::scene::SceneSlot, Pipeline};

pub struct OverlayPipeline {
    pipeline: wgpu::RenderPipeline,
    scene_uniform: Uniform<[[f32; 4]; 4]>
}

impl Pipeline for OverlayPipeline {
    fn attach<'a, 'b: 'a>(&'b self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        self.scene_uniform.bind(render_pass, 0);
    }
}

impl OverlayPipeline {
    pub fn new(device: &wgpu::Device, surface_configuration: &wgpu::SurfaceConfiguration, scene: &impl Scene) -> Self {
        let shader = device.create_shader_module(include_wgsl!("overlay.wgsl"));
        let layout = Texture::default_layout(device);
        let pipeline = utils::pipeline(device, &shader, surface_configuration, &[
            scene.layout(),
            &layout,
            &layout,
        ], &[
            Vertex::describe(),
            ModelRaw::describe(),
        ], false);

        let scene_uniform = Uniform::new(device);

        return Self {
            pipeline,
            scene_uniform,
        };
    }
}

unsafe impl SceneSlot for OverlayPipeline {
    fn update(&self, queue: &wgpu::Queue, scene: &impl Scene) {
        self.scene_uniform.update(queue, &scene.apply().into());
    }
}