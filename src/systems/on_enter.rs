use bevy::prelude::*;
use bevy_asset_loader::{dynamic_asset::DynamicAssets, standard_dynamic_asset::StandardDynamicAsset};

use crate::{components::SplashScreenTag, resources::SplashScreenConfiguration, state::SplashScreenState};

pub fn on_enter<T: States>(
    mut commands: Commands,
    mut dynamic_assets: ResMut<DynamicAssets>,
    splash_screen_configuration: Res<SplashScreenConfiguration<T>>,
    mut splash_screen_state: ResMut<NextState<SplashScreenState>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(splash_screen_configuration.clear_color),
                ..Default::default()
            },
            projection: OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                scaling_mode: splash_screen_configuration.camera_scaling_mode,
                ..Default::default()
            },
            ..Default::default()
        },
        SplashScreenTag,
    ));

    dynamic_assets.register_asset("splash_screen_path", Box::new(StandardDynamicAsset::Folder { path: splash_screen_configuration.path.clone() }));

    splash_screen_state.set(SplashScreenState::Initialize);
}
