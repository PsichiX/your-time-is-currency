use oxygengine::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct Time(pub Scalar);

impl Component for Time {
    type Storage = VecStorage<Self>;
}
