use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use ldtk_rust::LdtkFile;

use std::collections::HashMap;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "title".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}
fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // load & parse the LDtk JSON file
    let mut ldtk = LdtkFile::new("assets/AutoLayers_4_Advanced.ldtk".to_string());

    // the redraw field gives us some control on when to spawn
    ldtk.redraw = true;

    // now set up the tile assets
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

    // add the LDtk object and the tile assets as resources
    commands
        .insert_resource(ldtk)
        .insert_resource(texture_atlas_handles);
}

fn update(
    commands: &mut Commands,
    mut ldtk: ResMut<LdtkFile>,
    handles: Res<HashMap<i32, Handle<TextureAtlas>>>,
) {
    if !ldtk.redraw {
        return;
    }

    let tile_scale = 2.5;
    commands.spawn(Camera2dBundle::default());
    for level in ldtk.levels.iter() {
        commands.insert_resource(ClearColor(Color::hex(&level.__bg_color[1..]).unwrap()));
        for layer in level.layer_instances.iter() {
            if layer.__type == "IntGrid" || layer.__type == "AutoLayer" {
                let tileset_uid = layer.__tileset_def_uid.unwrap_or(-1);
                if tileset_uid >= 0 {
                    let layer_width =
                        layer.__c_wid as f32 * (layer.__grid_size as f32 * tile_scale);
                    let layer_height =
                        layer.__c_hei as f32 * (layer.__grid_size as f32 * tile_scale);
                    for tile in layer.auto_layer_tiles.iter() {
                        commands.spawn(SpriteSheetBundle {
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
    }
    ldtk.redraw = false;
}

// LDtk provides pixel locations starting in the top left. For Bevy we need to
// flip the Y axis and offset from the center of the screen.
fn convert_to_world(width: f32, height: f32, scale: f32, x: i32, y: i32) -> Vec3 {
    let world_x = (x as f32 * scale) - (width / 2.);
    let world_y = -(y as f32 * scale) + (height / 2.);
    Vec3::new(world_x, world_y, 0.)
}
