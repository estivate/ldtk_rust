use bevy::prelude::*;

// example rewrite in progress, I'm not using Bevy so welcome
// assist here.
//
//const LDTK_FILE_PATH: &str = "assets/game_1-1-3.ldtk";

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "title".to_string(),
        width: 1024.0,
        height: 768.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 50.0));
    commands.spawn_bundle(camera);
}