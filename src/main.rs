use bevy::{input::system::exit_on_esc_system, prelude::*};

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

enum Direction {
    Up,
    Down,
}

struct Character {
    speed: f32,
}

struct Ball {
    direction: Direction,
    max_speed: f32,
}

struct ThrowBallEvent();

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bovino!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_event::<ThrowBallEvent>()
        .add_system(exit_on_esc_system.system())
        .add_system(throw_ball.system())
        .add_system(character_movement.system())
        .add_system(ball_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let character_a_texture_handle = asset_server.load("sprites/character_a.png");
    let character_b_texture_handle = asset_server.load("sprites/character_b.png");
    let ground_texture_handle = asset_server.load("maps/ground_top.png");

    let width = 5;
    let height = 8;
    let tile_size = 32;

    for i in 0..width {
        for j in 0..height {
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(ground_texture_handle.clone().into()),
                transform: Transform::from_translation(Vec3::new(
                    (i * tile_size) as f32,
                    (-j * tile_size) as f32,
                    0.,
                )),
                ..Default::default()
            });
        }
    }

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(character_a_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(
                0.,
                ((-tile_size * height) + tile_size) as f32,
                0.,
            )),
            ..Default::default()
        })
        .insert(Character { speed: 150. });

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(character_b_texture_handle.into()),
        transform: Transform::from_translation(Vec3::new(
            ((tile_size * width / 2) - tile_size) as f32,
            0.,
            0.,
        )),
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn character_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut character_positions: Query<(&mut Transform, &Character)>,
    mut throw_ball_event: EventReader<ThrowBallEvent>,
) {
    for event in throw_ball_event.iter() {
        eprintln!("We are throwing a ball!");
        return
    }

    for (mut transform, character) in character_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= character.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += character.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= character.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += character.speed * time.delta_seconds();
        }
    }
}

fn throw_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    character_position: Query<&Transform, With<Character>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut throw_ball_event: EventWriter<ThrowBallEvent>,
) {
    if keyboard_input.pressed(KeyCode::E) {
        throw_ball_event.send(ThrowBallEvent());
    }

    if keyboard_input.just_released(KeyCode::E) {
        for transform in character_position.iter() {
            let ball_texture_handle = asset_server.load("sprites/ball.png");

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(ball_texture_handle.into()),
                    transform: transform.clone(),
                    ..Default::default()
                })
                .insert(Ball {
                    direction: Direction::Up,
                    max_speed: 300.,
                });
        }
    }
}

fn ball_movement(time: Res<Time>, mut ball_positions: Query<(&mut Transform, &Ball)>) {
    for (mut transform, ball) in ball_positions.iter_mut() {
        match ball.direction {
            Direction::Up => transform.translation.y += ball.max_speed * time.delta_seconds(),
            Direction::Down => transform.translation.y -= ball.max_speed * time.delta_seconds(),
        }
    }
}
