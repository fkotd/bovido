use crate::ball::{Target};
use crate::cartesian::CartesianTransform;
use crate::events::ThrowBallEvent;
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_characters.system())
            .add_system(move_character.system())
            .add_system(move_opponent.system());
    }
}

pub struct Character {
    speed: f32,
}

struct Opponent();

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
        })
        .insert(Opponent());
}

fn move_character(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut character_position: Query<(&mut CartesianTransform, &Character)>,
    mut throw_ball_event: EventReader<ThrowBallEvent>,
) {
    for _event in throw_ball_event.iter() {
        // eprintln!("We are throwing a ball!");
        return;
    }

    let (mut cartesian, character) = character_position
        .single_mut()
        .expect("There should always be exactly one player in the game.");

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

// TODO: split in multiple system to have simpler queries?
fn move_opponent(
    target_positions: Query<&CartesianTransform, (With<Target>, Without<Opponent>)>,
    mut opponent_positions: Query<&mut CartesianTransform, (With<Opponent>, Without<Target>)>,
) {
    for target_cartesian in target_positions.iter() {
        for mut opponent_cartesian in opponent_positions.iter_mut() {
            let direction = (target_cartesian.transform.translation
                - opponent_cartesian.transform.translation)
                .normalize();

            opponent_cartesian.transform.translation += direction
        }
    }
}
