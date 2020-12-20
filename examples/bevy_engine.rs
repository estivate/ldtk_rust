use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use ldtk::{self, Project};
use std::collections::HashMap;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "title".to_string(),
            width: 1024,
            height: 768,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // when starting up we need to read in the json file from ldtk
    // and instantiate an LdtkProject struct. We can then add this
    // struct as a bevy resource, or we can tease out certain areas
    // of data and add them as resources.
    let ldtk = Project::new_from_file("assets/AutoLayers_1_basic.ldtk".to_string());

    // we also need to load each tileset asset into bevy
    let mut texture_atlas_handles: HashMap<i32, Handle<TextureAtlas>> = HashMap::new();
    for tileset in ldtk.defs.tilesets.iter() {
        // load the asset
        let texture_handle = asset_server.load(&tileset.rel_path[..]);

        // calculate the atlas
        let t_size = tileset.tile_grid_size;
        let t_col = tileset.px_wid / t_size; //TODO: add spacing
        let t_rows = tileset.px_hei / t_size;
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(t_size as f32, t_size as f32),
            t_col as usize,
            t_rows as usize,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        texture_atlas_handles.insert(tileset.uid, texture_atlas_handle);
    }

    // add all the things
    commands
        .insert_resource(ldtk)
        .insert_resource(texture_atlas_handles);
}

fn update(
    mut commands: Commands,
    mut ldtk: ResMut<Project>,
    handles: Res<HashMap<i32, Handle<TextureAtlas>>>,
) {
    if !ldtk.redraw {
        return;
    }

    let tile_scale = 3.0;
    commands.spawn(Camera2dComponents::default());
    for level in ldtk.levels.iter() {
        commands.insert_resource(ClearColor(Color::hex(&level.__bg_color[1..]).unwrap()));
        for layer in level.layer_instances.iter() {
            if layer.__type == "IntGrid" || layer.__type == "AutoLayer" {
                let tileset_uid = layer
                    .__tileset_def_uid
                    .expect("No uid for tileset in IntGrid");
                let layer_width = layer.__c_wid as f32 * (layer.__grid_size as f32 * tile_scale);
                let layer_height = layer.__c_hei as f32 * (layer.__grid_size as f32 * tile_scale);
                for tile in layer.auto_layer_tiles.iter() {
                    commands.spawn(SpriteSheetComponents {
                        transform: Transform {
                            translation: convert_to_world(
                                layer_width,
                                layer_height,
                                tile_scale,
                                tile.px[0],
                                tile.px[1],
                            ),
                            scale: Vec3::splat(tile_scale),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(tile.t as u32),
                        texture_atlas: handles[&tileset_uid].clone(),
                        ..Default::default()
                    });
                }
            }
        }
    }

    ldtk.redraw = false;
}

fn convert_to_world(width: f32, height: f32, scale: f32, x: i32, y: i32) -> Vec3 {
    let world_x = (x as f32 * scale) - (width / 2.);
    let world_y = -(y as f32 * scale) + (height / 2.);
    Vec3::new(world_x, world_y, 0.)
}
