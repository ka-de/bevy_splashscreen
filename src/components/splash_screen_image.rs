use bevy::prelude::*;

/// Component for showing the splash screen image
#[derive(Component)]
pub struct SplashScreenImage {
    pub timer: Timer,
    pub index: usize,
}
