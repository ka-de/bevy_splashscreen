use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
#[cfg(feature = "progress_tracking")]
use bevy_progress::prelude::*;

use crate::{
    resources::{ SplashScreenConfiguration, SplashScreenImages },
    state::SplashScreenState,
    systems::{ on_enter, on_exit, update_splash_screen },
};

pub struct SplashScreenPlugin<T: States + Clone>(pub SplashScreenConfiguration<T>);

impl<T: States + Clone> Plugin for SplashScreenPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_state::<SplashScreenState>();

        // Spawn camera
        // Handles SplashScreenState::Idle -> SplashScreenState::Initialize
        app.add_systems(OnEnter(self.0.run_state.clone()), on_enter::<T>);

        #[cfg(feature = "progress_tracking")]
        {
            app.add_plugins(
                ProgressPlugin::new(SplashScreenState::Initialize).continue_to(
                    SplashScreenState::Update
                )
            );

            app.add_loading_state(
                LoadingState::new(
                    SplashScreenState::Initialize
                ).load_collection::<SplashScreenImages>()
            );
        }

        // Load all the splash screens as images
        // Handles SplashScreenState::Initialize -> SplashScreenState::Update
        #[cfg(not(feature = "progress_tracking"))]
        {
            app.add_loading_state(
                LoadingState::new(SplashScreenState::Initialize)
                    .load_collection::<SplashScreenImages>()
                    .continue_to_state(SplashScreenState::Update)
            );
        }

        // Display each splash screen
        // Handles T::run_state -> T::next_state
        app.add_systems(
            Update,
            update_splash_screen::<T>.run_if(
                in_state(self.0.run_state.clone()).and_then(in_state(SplashScreenState::Update))
            )
        );

        // Despawn any SplashScreenTag
        // Unload SplashScreens
        // Handles SplashScreenState::Update -> SplashScreenState::Idle
        app.add_systems(OnExit(self.0.run_state.clone()), on_exit);

        // Use our settings later
        app.insert_resource(self.0.clone());
    }
}
