use crate::cartesian::CartesianTransform;
use crate::character::Character;
use crate::events::ThrowBallEvent;
use bevy::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(throw_ball.system())
            .add_system(move_ball.system())
            .add_system(cartesian_to_iso.system());
    }
}

pub struct Projectile {
    velocity: Vec2,
    start_height: f32,
}

pub struct Target();

struct Gravity(f32);

fn throw_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    character_position: Query<(&Transform, &CartesianTransform), With<Character>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut throw_ball_event: EventWriter<ThrowBallEvent>,
) {
    if keyboard_input.pressed(KeyCode::E) {
        throw_ball_event.send(ThrowBallEvent());
    }

    if keyboard_input.just_released(KeyCode::E) {
        for (transform, cartesian) in character_position.iter() {
            let ball_texture_handle = asset_server.load("sprites/ball.png");

            // TODO: get from player input
            let parabola_half_distance = 70.;
            let parabola_height = 20.;

            let initial_velocity_x = 200.;
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
                    start_height: cartesian.transform.clone().translation.y,
                })
                .insert(CartesianTransform {
                    transform: cartesian.transform.clone(),
                })
                .insert(Gravity(gravity));

            let target_texture_handle = asset_server.load("sprites/target.png");

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(target_texture_handle.into()),
                    transform: transform.clone(),
                    ..Default::default()
                })
                .insert(CartesianTransform {
                    transform: Transform::from_translation(Vec3::new(
                        cartesian.transform.translation.x,
                        cartesian.transform.translation.y + (parabola_half_distance * 2.),
                        0.,
                    )),
                })
                .insert(Target());
        }
    }
}

fn move_ball(
    time: Res<Time>,
    mut ball: Query<(&mut CartesianTransform, &mut Projectile, &Gravity)>,
) {
    for (mut cartesian, mut projectile, gravity) in ball.iter_mut() {
        let delta = time.delta_seconds();

        if cartesian.transform.translation.y < projectile.start_height {
            continue;
        }

        // store the value of the height in z instead of x
        cartesian.transform.translation.z +=
            projectile.velocity.x * delta + (gravity.0 * delta * delta / 2.);
        cartesian.transform.translation.y +=
            projectile.velocity.y * delta + (gravity.0 * delta * delta / 2.);

        projectile.velocity.x += gravity.0 * delta;
    }
}

// TODO: apply the height for the ball in another system and make a unique system for
// cartesian_to_iso
fn cartesian_to_iso(mut ball: Query<(&mut Transform, &CartesianTransform), With<Projectile>>) {
    for (mut transform, cartesian) in ball.iter_mut() {
        transform.translation.x =
            cartesian.transform.translation.x + cartesian.transform.translation.y;
        transform.translation.y =
            (cartesian.transform.translation.y - cartesian.transform.translation.x) / 2.;

        // TODO: cleaner way to do that?
        transform.translation.y +=
            cartesian.transform.translation.z.sqrt() * cartesian.transform.translation.z.sqrt();
    }
}
