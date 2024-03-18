pub(crate) mod components;
pub(crate) mod resources;
pub(crate) mod state;
pub(crate) mod systems;

mod splash_screen_plugin;
pub use self::splash_screen_plugin::*;

pub mod prelude {
    pub use crate::splash_screen_plugin::SplashScreenPlugin;

    pub use crate::resources::SplashScreenConfiguration;
}
