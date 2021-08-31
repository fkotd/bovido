use bevy::{input::system::exit_on_esc_system, prelude::*};

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 480.;
const COURT_WIDTH: i32 = 5;
const COURT_HEIGHT: i32 = 9;
const COURT_X_TRANSLATION: i32 = 2;
const TILE_SIZE: i32 = 32;

struct Character {
    speed: f32,
}

struct Projectile {
    velocity: Vec2,
}

struct Gravity(f32);

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

    // create court
    let court_height = COURT_HEIGHT * TILE_SIZE;
    let court_width = COURT_WIDTH * TILE_SIZE;
    let court_x_translation = COURT_X_TRANSLATION * TILE_SIZE;

    for i in 0..COURT_WIDTH {
        for j in 0..COURT_HEIGHT {
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(ground_texture_handle.clone().into()),
                transform: Transform::from_translation(Vec3::new(
                    ((i * TILE_SIZE) - (court_width / 2) - court_x_translation) as f32,
                    ((-j * TILE_SIZE) + (court_height / 2)) as f32,
                    0.,
                )),
                ..Default::default()
            });
        }
    }

    // create player character
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(character_a_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(
                (-court_x_translation) as f32,
                (-court_height / 2 + TILE_SIZE) as f32,
                0.,
            )),
            ..Default::default()
        })
        .insert(Character { speed: 150. });

    // create opponent character
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(character_b_texture_handle.into()),
        transform: Transform::from_translation(Vec3::new(
            (-court_x_translation - TILE_SIZE) as f32,
            (court_height / 2) as f32,
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
        return;
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

            // TODO: get from player input
            let parabola_half_distance = 60.;
            let parabola_height = 40.;

            let initial_velocity_x = 100.;
            let initial_velocity_y =
                (2. * parabola_height * initial_velocity_x) / parabola_half_distance;

            let gravity = (-2. * parabola_height * initial_velocity_x * initial_velocity_x)
                / (parabola_half_distance * parabola_half_distance);

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(ball_texture_handle.into()),
                    transform: transform.clone(),
                    ..Default::default()
                })
                .insert(Projectile {
                    velocity: Vec2::new(initial_velocity_x, initial_velocity_y),
                })
                .insert(Gravity(gravity));
        }
    }
}

fn ball_movement(time: Res<Time>, mut ball: Query<(&mut Transform, &mut Projectile, &Gravity)>) {
    for (mut transform, mut projectile, gravity) in ball.iter_mut() {
        let delta = time.delta_seconds();

        transform.translation.x += projectile.velocity.x * delta + (gravity.0 * delta * delta / 2.);
        transform.translation.y += projectile.velocity.y * delta + (gravity.0 * delta * delta / 2.);

        projectile.velocity.y += gravity.0 * delta;
    }
}
