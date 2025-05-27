use bevy::{prelude::*, window::WindowResolution};

// example rewrite in progress, I'm not using Bevy so welcome
// assist here.
//
//const LDTK_FILE_PATH: &str = "assets/game_1-1-3.ldtk";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "title".to_string(),
                resolution: WindowResolution::new(1024.0, 768.0),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let camera = Camera2d::default();
    commands.spawn((
        camera,
        Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
    ));
}
