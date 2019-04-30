use crate::components::{enemy_controller::EnemyController, velocity::Velocity, EnemyTag};
use oxygengine::prelude::*;

pub struct EnemyControlSystem;

impl<'s> System<'s> for EnemyControlSystem {
    type SystemData = (
        ReadExpect<'s, AppLifeCycle>,
        ReadStorage<'s, EnemyController>,
        ReadStorage<'s, EnemyTag>,
        WriteStorage<'s, CompositeTransform>,
        ReadStorage<'s, Velocity>,
    );

    fn run(&mut self, (lifecycle, controller, tag, mut transforms, velocities): Self::SystemData) {
        let dt = lifecycle.delta_time_seconds() as Scalar;

        for (_, _, transform, velocity) in (&controller, &tag, &mut transforms, &velocities).join()
        {
            transform.set_translation(transform.get_translation() + velocity.0 * dt);
        }
    }
}
