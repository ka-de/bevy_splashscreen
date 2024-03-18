use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Asset collection holding all of the splash screen handles
#[derive(AssetCollection, Resource)]
pub struct SplashScreenImages {
    #[asset(key = "splash_screen_path", collection(typed))]
    pub images: Vec<Handle<Image>>,
}
