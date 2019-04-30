use oxygengine::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct Velocity(pub Vec2);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
