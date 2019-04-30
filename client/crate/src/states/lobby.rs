use crate::{consts::HOST_URL, messages::MessageData, states::game::GameState};
use oxygengine::prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct LobbyState {
    client: Option<ClientID>,
    entities: HashSet<Entity>,
}

impl State for LobbyState {
    fn on_enter(&mut self, world: &mut World) {
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
                    text: "Connecting...".into(),
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
        let network = &mut world.write_resource::<Network<WebClient>>();
        if self.client.is_none() {
            self.client = network.open_client(HOST_URL);
        }
        if let Some(client) = self.client {
            if network.has_client(client) {
                drop(if let Some(messages) = network.read(client) {
                    for msg in messages {
                        let msg = MessageData::from(msg);
                        match msg {
                            MessageData::InitPlayer(info) => {
                                self.client = None;
                                return StateChange::Swap(Box::new(GameState::new(client, info)));
                            }
                            _ => {}
                        }
                    }
                });
            } else {
                self.client = None;
            }
        }
        StateChange::None
    }
}
