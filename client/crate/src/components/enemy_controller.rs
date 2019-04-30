use oxygengine::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct EnemyController(u32);

impl EnemyController {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

impl Component for EnemyController {
    type Storage = VecStorage<Self>;
}
