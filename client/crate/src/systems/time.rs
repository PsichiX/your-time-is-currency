use crate::components::{owned_by::OwnedBy, time::Time, TimerTag};
use oxygengine::prelude::*;

pub struct TimeSystem;

impl<'s> System<'s> for TimeSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, AppLifeCycle>,
        WriteStorage<'s, Time>,
        WriteStorage<'s, CompositeRenderable>,
        ReadStorage<'s, TimerTag>,
        ReadStorage<'s, OwnedBy>,
    );

    fn run(
        &mut self,
        (entities, lifecycle, mut timers, mut renderables, timer_tag, owned_by): Self::SystemData,
    ) {
        let dt = lifecycle.delta_time_seconds() as Scalar;

        for (entity, mut timer) in (&entities, &mut timers).join() {
            timer.0 -= dt;
            if timer.0 <= 0.0 {
                // TODO: remove from `GameState::entities`.
                drop(entities.delete(entity));
            }
        }

        for (_, owned_by, renderable) in (&timer_tag, &owned_by, &mut renderables).join() {
            if let Some(time) = timers.get(owned_by.entity()) {
                match &mut renderable.0 {
                    Renderable::Text(text) => {
                        let time = (time.0 as i32).max(0);
                        let minutes = time / 60;
                        let seconds = time % 60;
                        text.text = format!("{:02}:{:02}", minutes, seconds).into();
                    }
                    _ => {}
                }
            }
        }
    }
}
