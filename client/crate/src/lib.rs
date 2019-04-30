extern crate byteorder;
extern crate oxygengine;

#[macro_use]
mod macros;

mod components;
mod consts;
mod messages;
mod states;
mod systems;

use crate::{
    states::loading::LoadingState,
    systems::{
        enemy_control::EnemyControlSystem, follow::FollowSystem,
        player_control::PlayerControlSystem, time::TimeSystem,
    },
};
use oxygengine::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let app = App::build()
        .with_bundle(
            oxygengine::core::assets::bundle_installer,
            (WebFetchEngine::default(), |assets| {
                oxygengine::composite_renderer::protocols_installer(assets);
            }),
        )
        .with_bundle(oxygengine::input::bundle_installer, |input| {
            input.register(WebKeyboardInputDevice::new(get_event_target_document()));
            input.map_axis("move-up", "keyboard", "KeyW");
            input.map_axis("move-down", "keyboard", "KeyS");
            input.map_axis("move-left", "keyboard", "KeyA");
            input.map_axis("move-right", "keyboard", "KeyD");
        })
        .with_bundle(oxygengine::network::bundle_installer::<WebClient, ()>, 0)
        .with_bundle(
            oxygengine::composite_renderer::bundle_installer,
            WebCompositeRenderer::with_state(
                get_canvas_by_id("screen"),
                RenderState::new(Some(Color::rgb(11, 72, 107))),
            ),
        )
        .with_system(PlayerControlSystem, "player_control", &[])
        .with_system(EnemyControlSystem, "enemy_control", &[])
        .with_system(FollowSystem, "follow", &[])
        .with_system(TimeSystem, "time", &[])
        .build(LoadingState::default(), WebAppTimer::default());

    AppRunner::new(app).run::<WebAppRunner, _>()?;

    Ok(())
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
