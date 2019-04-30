use oxygengine::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerController(u32);

impl PlayerController {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    // pub fn id(self) -> u32 {
    //     self.0
    // }
}

impl Component for PlayerController {
    type Storage = VecStorage<Self>;
}
