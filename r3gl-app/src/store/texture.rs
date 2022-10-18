use std::path::Path;

use wcore::graphics::{texture::Texture, context::Context};

pub struct TextureStore {
    pub t_circle: Texture,
    pub t_overlay: Texture,
    pub t_big_circle: Texture,
    pub t_big_overlay: Texture,
    pub t_hit_position: Texture,
    pub t_selection: Texture,
    pub t_selection_box: Texture,
}

impl TextureStore {
    pub fn from_path(path: impl AsRef<Path>, graphics: &Context) -> Self {
        let t_circle        = Texture::from_path(&graphics.device, &graphics.queue, path.as_ref().join("taikohitcircle.png"), wgpu::FilterMode::Linear, "circle").unwrap();
        let t_overlay       = Texture::from_path(&graphics.device, &graphics.queue, path.as_ref().join("taikohitcircleoverlay.png"), wgpu::FilterMode::Linear, "overlay").unwrap();
        let t_big_circle    = Texture::from_path(&graphics.device, &graphics.queue, path.as_ref().join("taikobigcircle.png"), wgpu::FilterMode::Linear, "big_circle").unwrap();
        let t_big_overlay   = Texture::from_path(&graphics.device, &graphics.queue, path.as_ref().join("taikobigcircleoverlay.png"), wgpu::FilterMode::Linear, "big_overlay").unwrap();
        let t_hit_position  = Texture::from_path(&graphics.device, &graphics.queue, path.as_ref().join("approachcircle.png"), wgpu::FilterMode::Linear, "big_overlay").unwrap();
        let t_selection     = Texture::from_path(&graphics.device, &graphics.queue, path.as_ref().join("selection.png"), wgpu::FilterMode::Linear, "selection").unwrap();
        let t_selection_box = Texture::from_path(&graphics.device, &graphics.queue, path.as_ref().join("selectionbox.png"), wgpu::FilterMode::Linear, "selection").unwrap();

        return Self {
            t_circle,
            t_overlay,
            t_big_circle,
            t_big_overlay,
            t_hit_position,
            t_selection,
            t_selection_box,
        };
    }
}