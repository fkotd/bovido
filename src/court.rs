use bevy::prelude::*;

const TILE_SIZE: i32 = 32;
const COURT_WIDTH: i32 = 12;
const COURT_HEIGHT: i32 = 16;

pub struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_court.system());
    }
}

fn create_court(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ground_texture_handle = asset_server.load("maps/tile.png");

    for i in 0..COURT_WIDTH {
        for j in 0..COURT_HEIGHT {
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(ground_texture_handle.clone().into()),
                transform: Transform::from_translation(Vec3::new(
                    ((i * (TILE_SIZE / 2)) + (-j * (TILE_SIZE / 2))) as f32,
                    ((-i * (TILE_SIZE / 4)) + (-j * (TILE_SIZE / 4))) as f32,
                    0.,
                )),
                ..Default::default()
            });
        }
    }
}
