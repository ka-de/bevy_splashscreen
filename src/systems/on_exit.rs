use bevy::prelude::*;

use crate::{components::SplashScreenTag, resources::SplashScreenImages, state::SplashScreenState};

pub fn on_exit(
    mut commands: Commands,
    q_splash_screen: Query<Entity, With<SplashScreenTag>>,
    mut splash_screen_state: ResMut<NextState<SplashScreenState>>,
) {
    for entity in q_splash_screen.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<SplashScreenImages>();

    splash_screen_state.set(SplashScreenState::Idle);
}
