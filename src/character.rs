use bevy::prelude::*;
use crate::cartesian::CartesianTransform;
use crate::events::ThrowBallEvent;
use crate::ball::Projectile;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_characters.system())
            .add_system(move_character.system())
            .add_system(cartesian_to_iso.system());
    }
}

pub struct Character {
    speed: f32,
}

fn create_characters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_texture_handle = asset_server.load("sprites/character_a.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(player_texture_handle.into()),
            ..Default::default()
        })
        .insert(CartesianTransform {
            transform: Transform::from_translation(Vec3::new(110., -205., 0.)),
        })
        .insert(Character { speed: 150. });

    let opponent_texture_handle = asset_server.load("sprites/character_b.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(opponent_texture_handle.into()),
            ..Default::default()
        })
        .insert(CartesianTransform {
            transform: Transform::from_translation(Vec3::new(40., 0., 0.)),
        });
}

fn move_character(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut character_positions: Query<(&mut CartesianTransform, &Character)>,
    mut throw_ball_event: EventReader<ThrowBallEvent>,
) {
    for _event in throw_ball_event.iter() {
    // eprintln!("We are throwing a ball!");
    return;
    }

    for (mut cartesian, character) in character_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            cartesian.transform.translation.x -= character.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            cartesian.transform.translation.x += character.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Down) {
            cartesian.transform.translation.y -= character.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Up) {
            cartesian.transform.translation.y += character.speed * time.delta_seconds();
        }
    }
}

fn cartesian_to_iso(mut ball: Query<(&mut Transform, &CartesianTransform), Without<Projectile>>) {
    for (mut transform, cartesian) in ball.iter_mut() {
        transform.translation.x =
            cartesian.transform.translation.x + cartesian.transform.translation.y;
        transform.translation.y =
            (cartesian.transform.translation.y - cartesian.transform.translation.x) / 2.;
    }
}
