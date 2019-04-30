use oxygengine::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct OwnedBy(Entity);

impl OwnedBy {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }

    pub fn entity(self) -> Entity {
        self.0
    }
}

impl Component for OwnedBy {
    type Storage = VecStorage<Self>;
}
