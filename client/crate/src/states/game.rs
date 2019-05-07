use crate::{
    components::{
        enemy_controller::EnemyController,
        follow::{Follow, FollowMode},
        owned_by::OwnedBy,
        player_controller::PlayerController,
        speed::Speed,
        time::Time,
        velocity::Velocity,
        EnemyTag, PlayerTag, TimerTag,
    },
    consts::SEND_STATE_DELAY,
    messages::{MessageData, MsgPlayerInfo, MsgPlayerState},
    states::lobby::LobbyState,
};
use oxygengine::prelude::*;
use std::collections::{HashMap, HashSet};

#[rustfmt::skip]
const WATER_MASK_ALPHA_MATRIX: &[Scalar] = &[
    0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05,
    0.05, 0.10, 0.10, 0.10, 0.10, 0.10, 0.05,
    0.05, 0.10, 0.15, 0.15, 0.15, 0.10, 0.05,
    0.05, 0.10, 0.15, 0.20, 0.15, 0.10, 0.05,
    0.05, 0.10, 0.15, 0.15, 0.15, 0.10, 0.05,
    0.05, 0.10, 0.10, 0.10, 0.10, 0.10, 0.05,
    0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05,
];

pub struct GameState {
    client: ClientID,
    info: MsgPlayerInfo,
    entities: HashSet<Entity>,
    water: Option<Entity>,
    camera: Option<Entity>,
    player: Option<Entity>,
    enemies: HashMap<u32, Entity>,
    send_state_timer: f64,
}

impl GameState {
    pub fn new(id: ClientID, info: MsgPlayerInfo) -> Self {
        Self {
            client: id,
            info,
            entities: Default::default(),
            water: None,
            camera: None,
            player: None,
            enemies: Default::default(),
            send_state_timer: 0.0,
        }
    }

    fn create_water(&mut self, world: &mut World) {
        let mut commands = vec![Command::Store];
        for col in -3..=3 {
            for row in -3..=3 {
                let c: i32 = col + 3;
                let r: i32 = row + 3;
                let alpha = WATER_MASK_ALPHA_MATRIX[r as usize * 7 + c as usize];
                let align = Vec2::new(0.5 + col as Scalar, 0.5 + row as Scalar);
                commands.push(Command::Alpha(alpha));
                commands.push(Command::Draw(Image::new("water.png").align(align).into()));
            }
        }
        commands.push(Command::Restore);

        let water = world
            .create_entity()
            .with(CompositeRenderable(Renderable::Commands(commands)))
            .with(CompositeRenderDepth(-10.0))
            .with(CompositeTransform::default())
            .with(Follow::with_mode(
                self.player.unwrap(),
                FollowMode::SnapToGrid(128.0.into(), 0, 0),
            ))
            .with(Tag("default".into()))
            .build();
        self.water = Some(water);
        self.entities.insert(water);
    }

    fn create_player(&mut self, world: &mut World) {
        let player = world
            .create_entity()
            .with(CompositeRenderable(
                Image {
                    image: "ferris.png".into(),
                    source: None,
                    destination: Some([0.0, 0.0, 175.0, 175.0].into()),
                    alignment: 0.5.into(),
                }
                .into(),
            ))
            .with(CompositeRenderDepth(1.0))
            .with(CompositeTransform::translation(self.info.position))
            .with(PlayerController::new(self.info.id))
            .with(Speed(400.0))
            .with(Velocity(0.0.into()))
            .with(PlayerTag)
            .with(Time(self.info.time))
            .with(Tag("default".into()))
            .build();
        self.player = Some(player);
        self.entities.insert(player);

        world
            .create_entity()
            .with(CompositeRenderable(
                Text {
                    color: Color::white(),
                    font: "Verdana".into(),
                    align: TextAlign::Center,
                    text: self.info.name.clone().into(),
                    position: 0.0.into(),
                    size: 24.0,
                }
                .into(),
            ))
            .with(CompositeRenderDepth(1.0))
            .with(CompositeTransform::translation([0.0, -64.0].into()))
            .with(Parent(player))
            .with(PlayerTag)
            .with(Tag("default".into()))
            .build();

        world
            .create_entity()
            .with(CompositeRenderable(
                Text {
                    color: Color::white(),
                    font: "Verdana".into(),
                    align: TextAlign::Center,
                    text: self.info.time.to_string().into(),
                    position: 0.0.into(),
                    size: 24.0,
                }
                .into(),
            ))
            .with(CompositeRenderDepth(1.0))
            .with(CompositeTransform::translation([0.0, 76.0].into()))
            .with(Parent(player))
            .with(PlayerTag)
            .with(TimerTag)
            .with(OwnedBy::new(player))
            .with(Tag("default".into()))
            .build();
    }

    fn create_enemy(&mut self, info: MsgPlayerInfo, world: &mut World) {
        if info.time <= 0.0 || info.id == self.info.id || self.enemies.contains_key(&info.id) {
            return;
        }

        let enemy = world
            .create_entity()
            .with(CompositeRenderable(
                Image {
                    image: "ferris.png".into(),
                    source: None,
                    destination: Some([0.0, 0.0, 175.0, 175.0].into()),
                    alignment: 0.5.into(),
                }
                .into(),
            ))
            .with(CompositeTransform::translation(info.position))
            .with(EnemyController::new(info.id))
            .with(Velocity(0.0.into()))
            .with(EnemyTag)
            .with(Time(info.time))
            .with(Tag("default".into()))
            .build();
        self.entities.insert(enemy);
        self.enemies.insert(info.id, enemy);

        world
            .create_entity()
            .with(CompositeRenderable(
                Text {
                    color: Color::yellow(),
                    font: "Verdana".into(),
                    align: TextAlign::Center,
                    text: info.name.clone().into(),
                    position: 0.0.into(),
                    size: 24.0,
                }
                .into(),
            ))
            .with(CompositeTransform::translation([0.0, -64.0].into()))
            .with(Parent(enemy))
            .with(EnemyTag)
            .with(Tag("default".into()))
            .build();

        world
            .create_entity()
            .with(CompositeRenderable(
                Text {
                    color: Color::yellow(),
                    font: "Verdana".into(),
                    align: TextAlign::Center,
                    text: self.info.time.to_string().into(),
                    position: 0.0.into(),
                    size: 24.0,
                }
                .into(),
            ))
            .with(CompositeTransform::translation([0.0, 76.0].into()))
            .with(Parent(enemy))
            .with(EnemyTag)
            .with(TimerTag)
            .with(OwnedBy::new(enemy))
            .with(Tag("default".into()))
            .build();
    }

    fn update_enemy(&mut self, state: MsgPlayerState, world: &mut World) {
        if self.info.id == state.id {
            return;
        }

        if let Some(entity) = self.enemies.get(&state.id) {
            if let Some(time) = world.write_storage::<Time>().get_mut(*entity) {
                time.0 = state.time;
            }
            if let Some(velocity) = world.write_storage::<Velocity>().get_mut(*entity) {
                velocity.0 = state.velocity;
            }
            if let Some(transform) = world.write_storage::<CompositeTransform>().get_mut(*entity) {
                transform.set_translation(state.position);
            }
        }
    }

    fn destroy_enemy(&mut self, id: u32, world: &mut World) {
        if let Some(entity) = self.enemies.remove(&id) {
            self.entities.remove(&entity);
            drop(world.delete_entity(entity));
        }
    }
}

impl State for GameState {
    fn on_enter(&mut self, world: &mut World) {
        self.create_player(world);
        self.create_water(world);

        {
            let mut camera = CompositeCamera::new(CompositeScalingMode::CenterAspect);
            camera.tags = vec!["default".into()];
            let camera = world
                .create_entity()
                .with(camera)
                .with(CompositeTransform::scale(1024.0.into()))
                .with(Follow::new(self.player.unwrap()))
                .build();
            self.camera = Some(camera);
            self.entities.insert(camera);
        }

        {
            let mut camera = CompositeCamera::new(CompositeScalingMode::None);
            camera.tags = vec!["ui".into()];
            self.entities.insert(
                world
                    .create_entity()
                    .with(CompositeTransform::default())
                    .with(camera)
                    .with(CompositeRenderDepth(1.0))
                    .build(),
            );

            self.entities.insert(
                world
                    .create_entity()
                    .with(CompositeRenderable(Image::new("logo.png").into()))
                    .with(CompositeTransform::translation(30.0.into()))
                    .with(Tag("ui".into()))
                    .build(),
            );
        }
    }

    fn on_exit(&mut self, world: &mut World) {
        for entity in self.entities.drain() {
            drop(world.delete_entity(entity));
        }
    }

    fn on_process(&mut self, world: &mut World) -> StateChange {
        let network = world.read_resource::<Network<WebClient>>();
        if !network.has_client(self.client) {
            return StateChange::Swap(Box::new(LobbyState::default()));
        }

        for entity in world.read_resource::<HierarchyChangeRes>().removed() {
            self.entities.remove(entity);
        }

        // process messages.
        let messages = network
            .read(self.client)
            .map(|messages| {
                messages
                    .map(|msg| MessageData::from(msg))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        drop(network);
        for msg in messages {
            match msg {
                MessageData::NewPlayer(info) => {
                    self.create_enemy(info, world);
                }
                MessageData::PlayerState(state) => {
                    self.update_enemy(state, world);
                }
                MessageData::PlayerDisconnected(id) => {
                    if id != self.info.id {
                        self.destroy_enemy(id, world);
                    }
                }
                MessageData::PlayerEat(t) => {
                    if let Some(player) = self.player {
                        if let Some(time) = world.write_storage::<Time>().get_mut(player) {
                            time.0 += t;
                        }
                    }
                }
                _ => {}
            };
        }

        // check time out.
        if let Some(player) = self.player {
            let time = world
                .read_storage::<Time>()
                .get(player)
                .map(|t| t.0)
                .unwrap_or_default();
            if time <= 0.0 {
                world
                    .write_resource::<Network<WebClient>>()
                    .close_client(self.client);
                return StateChange::Swap(Box::new(LobbyState::default()));
            }
        }

        // send player state.
        self.send_state_timer -= world.read_resource::<AppLifeCycle>().delta_time_seconds();
        if self.send_state_timer <= 0.0 {
            self.send_state_timer = SEND_STATE_DELAY;
            if let Some(player) = self.player {
                let time = world
                    .read_storage::<Time>()
                    .get(player)
                    .map(|t| t.0)
                    .unwrap_or_default();
                let position = world
                    .read_storage::<CompositeTransform>()
                    .get(player)
                    .map(|t| t.get_translation())
                    .unwrap_or_default();
                let velocity = world
                    .read_storage::<Velocity>()
                    .get(player)
                    .map(|v| v.0)
                    .unwrap_or_default();
                let message = MessageData::PlayerState(MsgPlayerState {
                    id: self.info.id,
                    time,
                    position,
                    velocity,
                });
                let id = message.id();
                let data: Vec<u8> = message.into();
                world
                    .write_resource::<Network<WebClient>>()
                    .send(self.client, id, &data);
            }
        }

        StateChange::None
    }
}
