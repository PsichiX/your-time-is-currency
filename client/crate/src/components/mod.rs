pub mod enemy_controller;
pub mod follow;
pub mod owned_by;
pub mod player_controller;
pub mod speed;
pub mod time;
pub mod velocity;

use oxygengine::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerTag;

impl Component for PlayerTag {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct EnemyTag;

impl Component for EnemyTag {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TimerTag;

impl Component for TimerTag {
    type Storage = NullStorage<Self>;
}
