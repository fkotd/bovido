use crate::cartesian::CartesianTransform;
use crate::character::Player;
use bevy::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(throw_ball.system())
            .add_system(move_ball.system())
            .add_system(add_ball_height.system().after("cartesian_to_iso"));
    }
}

pub struct Projectile {
    velocity: Vec2,
    height: f32,
    max_position: Vec3,
}

pub struct Target();

struct Gravity(f32);

fn throw_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    player_position: Query<(&Transform, &CartesianTransform), With<Player>>,
) {
    if keyboard_input.just_released(KeyCode::E) {
        let parabola_half_distance = spawn_ball(
            &mut commands,
            &asset_server,
            &mut materials,
            &player_position,
        );
        spawn_target(
            &mut commands,
            &asset_server,
            &mut materials,
            &player_position,
            parabola_half_distance,
        );
    }
}

fn spawn_ball(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_position: &Query<(&Transform, &CartesianTransform), With<Player>>,
) -> f32 {
    let (player_transform, player_cartesian) = player_position
        .single()
        .expect("There should always be exactly one player in the game.");

    let ball_texture_handle = asset_server.load("sprites/ball.png");

    // TODO: get from player input
    let parabola_half_distance = 70.;
    let parabola_height = 20.;
    let initial_velocity_x = 200.;

    let initial_velocity_y = (2. * parabola_height * initial_velocity_x) / parabola_half_distance;
    let gravity = (-2. * parabola_height * initial_velocity_x * initial_velocity_x)
        / (parabola_half_distance * parabola_half_distance);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(ball_texture_handle.into()),
            transform: player_transform.clone(),
            ..Default::default()
        })
        .insert(Projectile {
            velocity: Vec2::new(initial_velocity_x, initial_velocity_y),
            height: 0.,
            max_position: Vec3::new(
                player_cartesian.transform.translation.x,
                player_cartesian.transform.translation.y + 2. * parabola_half_distance,
                0.,
            ),
        })
        .insert(CartesianTransform {
            transform: player_cartesian.transform.clone(),
        })
        .insert(Gravity(gravity));

    parabola_half_distance
}

fn spawn_target(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_position: &Query<(&Transform, &CartesianTransform), With<Player>>,
    parabola_half_distance: f32,
) {
    let (player_transform, player_cartesian) = player_position
        .single()
        .expect("There should always be exactly one player in the game.");

    let target_texture_handle = asset_server.load("sprites/target.png");

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(target_texture_handle.into()),
            transform: player_transform.clone(),
            ..Default::default()
        })
        .insert(CartesianTransform {
            transform: Transform::from_translation(Vec3::new(
                player_cartesian.transform.translation.x + 4.,
                player_cartesian.transform.translation.y + 2. * parabola_half_distance - 2.,
                0.,
            )),
        })
        .insert(Target());
}

fn move_ball(
    time: Res<Time>,
    mut ball: Query<(&mut CartesianTransform, &mut Projectile, &Gravity)>,
) {
    for (mut cartesian, mut projectile, gravity) in ball.iter_mut() {
        if cartesian.transform.translation.y > projectile.max_position.y {
            continue;
        }

        let delta = time.delta_seconds();

        cartesian.transform.translation.y +=
            projectile.velocity.y * delta + (gravity.0 * delta * delta / 2.);
        projectile.height += projectile.velocity.x * delta + (gravity.0 * delta * delta / 2.);
        projectile.velocity.x += gravity.0 * delta;
    }
}

fn add_ball_height(mut ball: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, projectile) in ball.iter_mut() {
        transform.translation.y += projectile.height / 2. + 5.;
    }
}
