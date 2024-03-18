# Bevy Splash Screen

A simple splash screen plugin for [Bevy](https://github.com/bevyengine/bevy/tree/main)

## Usage
```rust
use bevy_splash_screen::prelude::*;
[...]
let config = SplashScreenConfiguration {
    run_state: AppState::SplashScreen,
    next_state: AppState::MainMenu,
    path: String::from("splash_screens"),
    custom_size: Some(Vec2::ONE),
    splash_timer: 3.0,
    clear_color: Color::BLACK,
    camera_scaling_mode: ScalingMode::Fixed { width: 1.0, height: 1.0 },
};

[or]

let config = SplashScreenConfiguration::new(AppState::SplashScreen, AppState::MainMenu).with_timer(5.0);

app.add_plugin(SplashScreenPlugin(config));
[...]
```

## How it works
We're using [bevy_asset_loader]() to load a folder containing `Image`s before displaying each of them. We create our own camera to display the splash screens, and destroy it after use. By default, we set a scaling mode to display 1 world unit and set the image size of each image to 1 world unit causing each image to possibly be stretched/shrunk to fit the window size.

## TODO
Eventually, I'd like to have some fade effects possibly from [bevy_tweening](https://github.com/djeedai/bevy_tweening)