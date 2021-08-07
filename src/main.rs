use bevy::{input::system::exit_on_esc_system, prelude::*};

struct Character;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bovino!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_system(character_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let character_texture_handle = asset_server.load("sprites/persono.png");
    let ground_texture_handle = asset_server.load("maps/tile.png");

    let width = 12;
    let height = 16;
    let tile_size = 32;
    let tile_half_size = tile_size / 2;
    let tile_quarter_size = tile_size / 4;

    for i in 0..width {
        for j in 0..height {
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(ground_texture_handle.clone().into()),
                    transform: Transform::from_translation(Vec3::new(
                            ((i * tile_half_size) + (-j * tile_half_size)) as f32,
                            ((-i * tile_quarter_size) + (-j * tile_quarter_size)) as f32,
                            0.
                    )),
                    ..Default::default()
                });
        }
    }
    
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(character_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(
                    0.,
                    0.,
                    0.,
            )),
            ..Default::default()
        })
        .insert(Character);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());


}

fn character_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut character_positions: Query<&mut Transform, With<Character>>
) {
    for mut transfrom in character_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transfrom.translation.x -=2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transfrom.translation.x +=2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transfrom.translation.y -=2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transfrom.translation.y +=2.;
        }
    } 
}
