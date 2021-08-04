use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_tiled_prototype::TiledMapCenter;

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
        .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_system(character_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let character_texture_handle = asset_server.load("sprites/persono.png");
    let map_handle = asset_server.load("maps/ground.tmx");

    commands
        .spawn_bundle(bevy_tiled_prototype::TiledMapBundle {
            map_asset: map_handle,
            center: TiledMapCenter(true),
            ..Default::default()
        });
    commands
        .spawn_bundle(SpriteBundle {
        material: materials.add(character_texture_handle.into()),
        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
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
            transfrom.translation.x -=1.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transfrom.translation.x +=1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transfrom.translation.y -=1.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transfrom.translation.y +=1.;
        }
    } 
}
