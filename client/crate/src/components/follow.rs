use oxygengine::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum FollowMode {
    Stick,
    Tween(Scalar),
    /// (grid size, cols offset, rows offset)
    SnapToGrid(Vec2, i32, i32),
}

#[derive(Debug, Copy, Clone)]
pub struct Follow {
    pub entity: Entity,
    pub mode: FollowMode,
}

impl Follow {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            mode: FollowMode::Stick,
        }
    }

    pub fn with_mode(entity: Entity, mode: FollowMode) -> Self {
        Self { entity, mode }
    }
}

impl Component for Follow {
    type Storage = VecStorage<Self>;
}
