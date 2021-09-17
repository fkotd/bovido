use ball::BallPlugin;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use character::CharacterPlugin;
use court::CourtPlugin;
use events::EventsPlugin;

mod ball;
mod cartesian;
mod character;
mod court;
mod events;

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 480.;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Bovido".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BallPlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(CourtPlugin)
        .add_plugin(EventsPlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
