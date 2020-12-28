use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use ldtk_rust::{EntityInstance, LdtkFile, TileInstance};

use std::collections::HashMap;

// Tiles come in all sizes, this gives us easy control over
// making them bigger and smaller.
const TILE_SCALE: f32 = 2.5;
// Extend the LdtkFile object with whatever you need for your
// game engine. In a real game you might need a variety of
// fields to control how and when you use the LDtk information.
struct Map {
    ldtk_file: LdtkFile,
    redraw: bool,
    current_level: usize,
}

// When working with the LDtk info, you'll have nested loops
// for layers and tiles. You'll loop through the layers and
// for each layer you'll loop through the tiles for that layer.
// This struct holds the relevant layer information so we can
// easily pass it all at once to each tile.
#[derive(Clone, Copy)]
struct LayerInfo {
    width: f32,
    height: f32,
    depth: i32,
}

// Bevy specific app setup. This just means we are opening a
// window for our game, running the setup() function once at
// startup and then running update() every game loop.
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

// Our setup system will run once and will read in the LDtk file
// and then register whatever we need in "global" scope as
// Bevy Resources. Here I choose to save the parsed LDtk info
// as a Resource as well as any graphical tilemap assets as
// Texture Atlases. This seemed simplest as an example.
fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Path to LDtk JSON file
    let source_file = "assets/AutoLayers_4_Advanced.ldtk".to_string();

    // Create a new Map instance and set the values. This is where we
    // actually load in the LDtk file.
    let map = Map {
        ldtk_file: LdtkFile::new(source_file),
        redraw: true,
        current_level: 0,
    };

    // For each tileset referenced in the LDtk file, create a Texture Atlas
    // and store a Handle in a Hash. The key to the Hash is the value LDtk
    // assigns as the tileset's UID. If you know you only have one tileset
    // asset, you could simplify this and just load it like any other asset
    // using map.ldtk_file.defs.tilesets[0].rel_path
    //
    // Other game engines will approach this differently, but in essence all
    // we are doing is making sure we have a way to call the appropriate sprite
    // sheet when loading tiles.
    let mut texture_atlas_handles: HashMap<i32, Handle<TextureAtlas>> = HashMap::new();
    for tileset in map.ldtk_file.defs.tilesets.iter() {
        let texture_handle = asset_server.load(&tileset.rel_path[..]);

        let t_size = tileset.tile_grid_size;
        let t_col = tileset.px_wid / t_size;
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

    // LDtk IntGrids support setting colors for walls, sky, etc. If you are doing this
    // in your game you'd probably want to loop through your IntGrid layers and create
    // materials for each integer value. The Bevy snake tutorial has some good sample
    // code for using materials: https://mbuffett.com/posts/bevy-snake-tutorial/

    // LDtk supports placement of Entities in levels (player, chest, health potion, etc.)
    // If you are using this feature you may want to do additional setup here beyond
    // loading in the tilemap assets above.

    // Bevy-specific code to add the Map instance and the texture atlas handles as
    // global variables so we can access them in update(). We also go ahead and spawn
    // a default camera so we will be able to see the things we spawn next.
    commands
        .insert_resource(map)
        .insert_resource(texture_atlas_handles)
        .spawn(Camera2dBundle::default());
}

// Our update system runs every game loop and, if the tiles are not spawned, will spawn them.
// In practice you'll likely want to split some of this up, but it's a decent start to
// having the ability to regenerate tiles as the current level changes, etc.
fn update(
    commands: &mut Commands,
    mut map: ResMut<Map>,
    handles: Res<HashMap<i32, Handle<TextureAtlas>>>,
) {
    // If we don't need to redraw the tiles, go ahead and return (do nothing)
    if !map.redraw {
        return;
    }

    // Add a background color. The "__bg_color" field should always be populated
    // with either the default background color or the level's custom color.
    commands.insert_resource(ClearColor(
        Color::hex(&map.ldtk_file.levels[0].__bg_color[1..]).unwrap(),
    ));

    // For the current level, loop through the Layer Instances and start spawning
    // tiles. These Layer Instances can be one of four different kinds of layers:
    // IntGrid, Entities, Tiles or AutoLayer. Tiles and AutoLayers are easy: you
    // always iterate through Tile Instances to do your work. IntGrids may or may
    // not use a tileset (they can use flat colors). If they have tiles, you handle
    // them just like Tiles and AutLayers (but you have to check for a tileset first).
    // For entities you iterate through Entity Instances instead of Tile Instances.
    //
    // Your game only needs to handle the kinds of layers you want to use in LDtk.
    // Here I try to organize the code to accommodate all the options, but then
    // implement some simplistic code for the ones that use actual tilesets.
    //
    // Using .rev() allows us to handle things "bottom to top" and makes sorting
    // on the z-axis easier to reason about.
    for (idx, layer) in map.ldtk_file.levels[map.current_level]
        .layer_instances
        .iter()
        .enumerate()
        .rev()
    {
        // This gets us a unique ID to refer to the tileset if there is one.
        // If there's no tileset, it's value is set to -1, which could be used
        // as a check. Currently it is used only as a key to the hash of asset
        // handles.
        let tileset_uid = layer.__tileset_def_uid.unwrap_or(-1);

        // Multiply the grid size by the tile size and our scaling constant
        // to calculate the total width and height of our layer. For depth
        // we pick a starting point and add the loop index so we always draw
        // tiles on top of previous iterations. We do all this in a struct
        // instance so we can easily pass it around to functions later.
        let layer_info = LayerInfo {
            width: layer.__c_wid as f32 * (layer.__grid_size as f32 * TILE_SCALE),
            height: layer.__c_hei as f32 * (layer.__grid_size as f32 * TILE_SCALE),
            depth: 1 + idx as i32,
        };

        // Finally we match on the four possible kinds of Layer Instances and
        // handle each accordingly.
        match &layer.__type[..] {
            "Tiles" => {
                println!("Generating Tile Layer: {}", layer.__identifier);
                for tile in layer.grid_tiles.iter() {
                    display_tile(layer_info, tile, commands, handles[&tileset_uid].clone());
                }
            }
            "AutoLayer" => {
                println!("Generating AutoTile Layer: {}", layer.__identifier);
                for tile in layer.auto_layer_tiles.iter() {
                    display_tile(layer_info, tile, commands, handles[&tileset_uid].clone());
                }
            }
            "IntGrid" => {
                println!("Generating IntGrid AutoTile Layer: {}", layer.__identifier);
                match layer.__tileset_def_uid {
                    Some(i) => {
                        // we have tiles, so handle just like Tiles and AutoLayers
                        for tile in layer.auto_layer_tiles.iter() {
                            display_tile(layer_info, tile, commands, handles[&i].clone());
                        }
                    }
                    None => {
                        // we do NOT have a corresponding tileset, so we need to use
                        // the color values to represent the level visually.
                        println!("Flat color tiles for IntGrid: {:?}", layer.int_grid);
                    }
                }
            }
            "Entities" => {
                println!("Generating Entities Layer: {}", layer.__identifier);
                match layer.__tileset_def_uid {
                    Some(i) => {
                        // we have graphical tiles for entities so we can use them
                        for entity in layer.entity_instances.iter() {
                            display_entity(layer_info, entity, commands, handles[&i].clone());
                        }
                    }
                    None => {
                        // no corresponding tileset, so we want to do something different
                        println!("Need something to display for this entity");
                    }
                }
            }
            _ => {
                println!("Not Implemented: {}", layer.__identifier);
            }
        }
    }

    // Whew, we've draw everyting so update the Map instance so we don't do it every game loop.
    map.redraw = false;
}

// Spawn a tile. Check to see if it needs to flip on the x and/or y axis before spawning.
fn display_tile(
    layer_info: LayerInfo,
    tile: &TileInstance,
    commands: &mut Commands,
    handle: Handle<TextureAtlas>,
) {
    let mut flip_x = false;
    let mut flip_y = false;
    match tile.f {
        1 => flip_x = true,
        2 => flip_y = true,
        3 => {
            flip_x = true;
            flip_y = true
        }
        _ => (),
    }
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: convert_to_world(
                layer_info.width,
                layer_info.height,
                TILE_SCALE,
                tile.px[0],
                tile.px[1],
                layer_info.depth,
            ),
            rotation: flip(flip_x, flip_y),
            scale: Vec3::splat(TILE_SCALE),
        },
        sprite: TextureAtlasSprite::new(tile.t as u32),
        texture_atlas: handle,
        ..Default::default()
    });
}

// spawn your entities. This is likely very game dependant, but
// here's a very basic example.
fn display_entity(
    layer_info: LayerInfo,
    entity: &EntityInstance,
    commands: &mut Commands,
    handle: Handle<TextureAtlas>,
) {
    println!("handling entitiy: {}", entity.__identifier);

    let optional_tile = &entity.__tile;
    match optional_tile {
        Some(t) => {
            // entity has a graphical component so spawn a sprite
            commands.spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: convert_to_world(
                        layer_info.width,
                        layer_info.height,
                        TILE_SCALE,
                        entity.__grid[0],
                        entity.__grid[1],
                        layer_info.depth,
                    ),
                    scale: Vec3::splat(TILE_SCALE),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(t.tileset_uid as u32),
                texture_atlas: handle,
                ..Default::default()
            });
        }
        None => {
            // entity has no tile so spawn in another way.
            println!("No tile associated for entity: {:?}", entity);
        }
    }
}

// LDtk provides pixel locations starting in the top left. For Bevy we need to
// flip the Y axis and offset from the center of the screen.
fn convert_to_world(width: f32, height: f32, scale: f32, x: i32, y: i32, z: i32) -> Vec3 {
    let world_x = (x as f32 * scale) - (width / 2.);
    let world_y = -(y as f32 * scale) + (height / 2.);
    let world_z = 50.0 - z as f32;
    Vec3::new(world_x, world_y, world_z)
}

// Bevy doesn't have sprite flipping built in, so if tile needs to flip
//  on either axis, flip it.
fn flip(x: bool, y: bool) -> Quat {
    let mut q1 = Quat::default();
    let mut q2 = Quat::default();
    if x {
        q1 = Quat::from_rotation_y(std::f32::consts::PI);
    }
    if y {
        q2 = Quat::from_rotation_x(std::f32::consts::PI);
    }
    q1 * q2
}
