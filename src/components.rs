use specs::{Component, VecStorage, World, WorldExt};
use crate::ImageResource;

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub enum RenderableKind {
    Static,
    Animated,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable<IR: ImageResource + 'static>
    where <IR as ImageResource>::TextureId: Send + Sync {
    ids: Vec<IR::TextureId>,
}

impl<IR: ImageResource> Renderable<IR>
    where <IR as ImageResource>::TextureId: Send + Sync {
    pub fn new_static(id: IR::TextureId) -> Self {
        Self { ids: vec![id] }
    }

    pub fn new_animated(ids: Vec<IR::TextureId>) -> Self {
        Self { ids }
    }

    pub fn kind(&self) -> RenderableKind {
        match self.ids.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn texture_id(&self, id_index: usize) -> IR::TextureId {
        self.ids[id_index % self.ids.len()].clone()
    }
}

pub(crate) fn register_components<IR: ImageResource + 'static>(world: &mut World)
    where <IR as ImageResource>::TextureId: Send + Sync {
    world.register::<Position>();
    world.register::<Renderable<IR>>();
}