use crate::components::{
    player_controller::PlayerController, speed::Speed, velocity::Velocity, PlayerTag,
};
use oxygengine::prelude::*;

pub struct PlayerControlSystem;

impl<'s> System<'s> for PlayerControlSystem {
    type SystemData = (
        Read<'s, InputController>,
        ReadExpect<'s, AppLifeCycle>,
        ReadStorage<'s, Speed>,
        ReadStorage<'s, PlayerController>,
        ReadStorage<'s, PlayerTag>,
        WriteStorage<'s, CompositeTransform>,
        WriteStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (input, lifecycle, speed, controller, tag, mut transforms, mut velocities): Self::SystemData,
    ) {
        let dt = lifecycle.delta_time_seconds() as Scalar;
        let hor = -input.axis_or_default("move-left") + input.axis_or_default("move-right");
        let ver = -input.axis_or_default("move-up") + input.axis_or_default("move-down");
        let offset = Vec2::new(hor, ver);

        for (_, _, speed, transform, velocity) in
            (&controller, &tag, &speed, &mut transforms, &mut velocities).join()
        {
            let vel = offset * speed.0;
            transform.set_translation(transform.get_translation() + vel * dt);
            velocity.0 = vel;
        }
    }
}
