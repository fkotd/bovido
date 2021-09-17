use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ThrowBallEvent>();
    }
}

pub struct ThrowBallEvent();
