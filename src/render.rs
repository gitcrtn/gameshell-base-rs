use std::collections::HashMap;
use specs::{Join, Read, ReadStorage};
use crate::components::{Position, Renderable, RenderableKind};
use crate::ImageResource;
use crate::resources::Time;
use crate::texture::RenderContext;

struct DrawParam {
    pub x: i32,
    pub y: i32,
}

pub struct RenderingHelper<'a, IR: ImageResource> {
    pub context: &'a mut RenderContext<IR>,
}

impl<IR: ImageResource> RenderingHelper<'_, IR> {
    pub fn get_texture_id(&self, renderable: &Renderable<IR>, ticks: u32)
                      -> IR::TextureId where <IR as ImageResource>::TextureId: Send + Sync {
        let id_index = match renderable.kind() {
            RenderableKind::Static => {
                0
            }
            RenderableKind::Animated => {
                ((ticks % 1000) / 250) as usize
            }
        };

        renderable.texture_id(id_index)
    }

    pub fn draw_renderables(&mut self, positions: &ReadStorage<Position>, renderables: &ReadStorage<Renderable<IR>>, time: &Read<Time>)
        where <IR as ImageResource>::TextureId: Send + Sync {

        let rendering_data = (positions, renderables).join().collect::<Vec<_>>();
        let mut rendering_batches: HashMap<u8, HashMap<IR::TextureId, Vec<DrawParam>>> = HashMap::new();

        for (position, renderable) in rendering_data.iter() {
            let texture_id = self.get_texture_id(renderable, time.last_ticks);

            let (x, y) = self.context.resource.get_tile_position(position);
            let z = position.z;

            let draw_param = DrawParam { x, y };
            rendering_batches
                .entry(z)
                .or_default()
                .entry(texture_id)
                .or_default()
                .push(draw_param);
        }

        let mut z_indexes = rendering_batches.keys().cloned().collect::<Vec<u8>>();
        z_indexes.sort();

        for z in z_indexes.iter() {
            for (texture_id, draw_params) in rendering_batches.get(&z).unwrap().iter() {
                for draw_param in draw_params.iter() {
                    self.context.draw(texture_id.clone(), draw_param.x, draw_param.y);
                }
            }
        }
    }
}