use bevy::prelude::*;

use crate::{components::{SplashScreenImage, SplashScreenTag}, resources::{SplashScreenConfiguration, SplashScreenImages}};

pub fn update_splash_screen<T: States + Clone>(
    mut commands: Commands,
    time: Res<Time>,
    splash_screen_configuration: Res<SplashScreenConfiguration<T>>,
    splash_screen_handles: Res<SplashScreenImages>,
    mut q_splash_screens: Query<(&mut SplashScreenImage, &mut Handle<Image>)>,
    mut app_state: ResMut<NextState<T>>,
) {
    match q_splash_screens.get_single_mut() {
        Ok((mut splash_screen_image, mut image_handle)) => {
            splash_screen_image.timer.tick(time.delta());
            if splash_screen_image.timer.just_finished() {
                splash_screen_image.index += 1;

                if splash_screen_image.index < splash_screen_handles.images.len() {
                    *image_handle = splash_screen_handles.images[splash_screen_image.index].clone_weak();
                    splash_screen_image.timer = Timer::from_seconds(splash_screen_configuration.splash_timer, TimerMode::Once);
                } else {
                    app_state.set(splash_screen_configuration.next_state.clone());
                }
            }
        },
        Err(_) => {
            if !splash_screen_handles.images.is_empty() {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: splash_screen_configuration.custom_size,
                            ..Default::default()
                        },
                        texture: splash_screen_handles.images[0].clone(),
                        ..Default::default()
                    },
                    SplashScreenImage {
                        timer: Timer::from_seconds(splash_screen_configuration.splash_timer, TimerMode::Once),
                        index: 0,
                    },
                    SplashScreenTag,
                ));
            } else {
                app_state.set(splash_screen_configuration.next_state.clone());
            }
        },
    }
}
