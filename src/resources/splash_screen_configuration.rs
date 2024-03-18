use bevy::{prelude::*, render::camera::ScalingMode};

/// Configuration for the SplashScreenPlugin
#[derive(Resource, Reflect, Debug, Clone)]
pub struct SplashScreenConfiguration<T: States + Clone> {
    /// This is the state which the splash screens should be displayed
    pub run_state: T,
    /// This is the state after all the splash screens have been displayed
    pub next_state: T,
    /// This is a path relative to the `assets` folder which contains the splash screens to be displayed
    pub path: String,
    /// This is a way to override the size of each image.
    pub custom_size: Option<Vec2>,
    /// This is how long in seconds each splash screen should be displayed
    pub splash_timer: f32,
    /// This is the clear color for the splash camera
    pub clear_color: Color,
    /// This is the scaling mode to be set for the splash camera
    pub camera_scaling_mode: ScalingMode,
}

impl<T: States + Clone> SplashScreenConfiguration<T> {
    /// Creates a new configuration with default values
    pub fn new(run_state: T, next_state: T) -> Self {
        Self {
            run_state,
            next_state,
            path: String::from("splash_screens"),
            custom_size: Some(Vec2::ONE),
            splash_timer: 3.0,
            clear_color: Color::BLACK,
            camera_scaling_mode: ScalingMode::Fixed {
                width: 1.0,
                height: 1.0,
            },
        }
    }

    /// Change the path relative to the `assets` folder which contains the splash screens to be displayed
    pub fn with_path(mut self, asset_folder: impl ToString) -> Self {
        self.path = asset_folder.to_string();
        self
    }

    /// Change the override for each image size
    pub fn with_custom_size(mut self, custom_size: Option<Vec2>) -> Self {
        self.custom_size = custom_size;
        self
    }

    /// Change how long in seconds each splash screen should be displayed
    pub fn with_timer(mut self, seconds: f32) -> Self {
        self.splash_timer = seconds;
        self
    }

    /// Change the clear color for the splash camera
    pub fn with_clear_color(mut self, clear_color: Color) -> Self {
        self.clear_color = clear_color;
        self
    }

    /// Change the scaling mode to be set for the splash camera
    pub fn with_scaling_mode(mut self, scaling_mode: ScalingMode) -> Self {
        self.camera_scaling_mode = scaling_mode;
        self
    }
}
