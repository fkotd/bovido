use crate::ball::Target;
use crate::cartesian::CartesianTransform;
use crate::events::ReturnBallEvent;
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_characters.system())
            .add_system(move_player.system())
            .add_system(move_opponent.system());
    }
}

struct Character {
    speed: f32,
}

pub struct Player();
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
            transform: Transform::from_translation(Vec3::new(110., -205., 5.)),
        })
        .insert(Character { speed: 150. })
        .insert(Player());

    let opponent_texture_handle = asset_server.load("sprites/character_b.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(opponent_texture_handle.into()),
            ..Default::default()
        })
        .insert(CartesianTransform {
            transform: Transform::from_translation(Vec3::new(40., 0., 5.)),
        })
        .insert(Character { speed: 150. })
        .insert(Opponent());
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_position: Query<(&mut CartesianTransform, &Character), With<Player>>,
) {
    let (mut cartesian, character) = player_position
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
    mut opponent_position: Query<
        (&mut CartesianTransform, &Character),
        (With<Opponent>, Without<Target>),
    >,
    mut return_ball_event: EventWriter<ReturnBallEvent>,
    time: Res<Time>,
) {
    let (mut opponent_cartesian, opponent_character) = opponent_position
        .single_mut()
        .expect("There should always be exactly one opponent in the game.");

    for target_cartesian in target_positions.iter() {
        let direction = (target_cartesian.transform.translation
            - opponent_cartesian.transform.translation)
            .normalize();

        if opponent_cartesian.transform.translation.x.abs()
            < target_cartesian.transform.translation.x.abs() - 5.
            && opponent_cartesian.transform.translation.y.abs()
                < target_cartesian.transform.translation.y.abs() - 5.
        {
            opponent_cartesian.transform.translation +=
                direction * time.delta_seconds() * opponent_character.speed;
        } else {
            return_ball_event.send(ReturnBallEvent());
        }
    }
}
