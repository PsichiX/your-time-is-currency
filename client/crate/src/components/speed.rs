use oxygengine::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct Speed(pub Scalar);

impl Component for Speed {
    type Storage = VecStorage<Self>;
}
