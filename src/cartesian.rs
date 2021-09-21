use bevy::prelude::*;

pub struct CartesianPlugin;

impl Plugin for CartesianPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(cartesian_to_iso.system().label("cartesian_to_iso"));
    }
}

pub struct CartesianTransform {
    pub transform: Transform,
}

fn cartesian_to_iso(mut q: Query<(&mut Transform, &CartesianTransform)>) {
    for (mut transform, cartesian) in q.iter_mut() {
        transform.translation.x =
            cartesian.transform.translation.x + cartesian.transform.translation.y;
        transform.translation.y =
            (cartesian.transform.translation.y - cartesian.transform.translation.x) / 2.;
    }
}
