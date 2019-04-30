use crate::components::{
    follow::{Follow, FollowMode},
    velocity::Velocity,
};
use oxygengine::prelude::*;

pub struct FollowSystem;

impl<'s> System<'s> for FollowSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Follow>,
        WriteStorage<'s, CompositeTransform>,
        ReadStorage<'s, Velocity>,
    );

    fn run(&mut self, (entities, follows, mut transforms, velocities): Self::SystemData) {
        let meta = (&entities, &follows)
            .join()
            .filter_map(|(entity, follow)| {
                if let Some(velocity) = velocities.get(follow.entity) {
                    transforms
                        .get(follow.entity)
                        .map(|source| (entity, source.get_translation(), velocity.0, follow.mode))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (entity, position, velocity, mode) in meta {
            if let Some(transform) = transforms.get_mut(entity) {
                match mode {
                    FollowMode::Stick => {
                        transform.set_translation(position);
                    }
                    FollowMode::Tween(factor) => {
                        transform.set_translation(position + velocity * factor);
                    }
                    FollowMode::SnapToGrid(size, offset_cols, offset_rows) => {
                        let x = if size.x > 0.0 {
                            ((position.x / size.x).round() + offset_cols as Scalar) * size.x
                        } else {
                            position.x
                        };
                        let y = if size.y > 0.0 {
                            ((position.y / size.y).round() + offset_rows as Scalar) * size.y
                        } else {
                            position.y
                        };
                        transform.set_translation(Vec2::new(x, y));
                    }
                }
            }
        }
    }
}
