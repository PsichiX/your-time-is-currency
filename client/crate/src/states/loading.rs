use crate::states::lobby::LobbyState;
use oxygengine::prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct LoadingState {
    entities: HashSet<Entity>,
}

impl State for LoadingState {
    fn on_enter(&mut self, world: &mut World) {
        world
            .write_resource::<AssetsDatabase>()
            .load("set://assets.txt")
            .expect("cannot load `assets.txt`");

        let camera = world
            .create_entity()
            .with(CompositeCamera::new(CompositeScalingMode::CenterAspect))
            .with(CompositeTransform::scale(1024.0.into()))
            .build();
        self.entities.insert(camera);

        let label = world
            .create_entity()
            .with(CompositeRenderable(
                Text {
                    color: Color::white(),
                    font: "Verdana".into(),
                    align: TextAlign::Center,
                    text: "Loading...".into(),
                    position: 0.0.into(),
                    size: 64.0,
                }
                .into(),
            ))
            .with(CompositeTransform::translation([0.0, 300.0].into()))
            .build();
        self.entities.insert(label);
    }

    fn on_exit(&mut self, world: &mut World) {
        for entity in self.entities.drain() {
            drop(world.delete_entity(entity));
        }
    }

    fn on_process(&mut self, world: &mut World) -> StateChange {
        let assets = &world.read_resource::<AssetsDatabase>();
        if assets.is_ready() {
            StateChange::Swap(Box::new(LobbyState::default()))
        } else {
            StateChange::None
        }
    }
}
