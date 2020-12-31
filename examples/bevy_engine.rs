// This example is designed to try and cover as many use cases
// as possible from a wide variety of LDtk source files. That's
// great for an example file, but it's probably too complex if
// you know what your game wants from LDtk.
//
// In any particular game you'll likely just load stuff up
// by referencing it directly from the LdtkFile instance instead
// of looping through and matching for all the options like I
// do here.

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use ldtk_rust::{EntityInstance, IntGrid, LdtkFile, TileInstance};

use std::collections::HashMap;

// Constants
const LDTK_FILE_PATH: &str = "assets/test_game.ldtk";
const TILE_SCALE: f32 = 2.5;

// Extend the LdtkFile object with whatever you need for your
// game engine. In a real game you might need a variety of
// fields to control how and when you use the LDtk information.
struct Map {
    ldtk_file: LdtkFile,
    redraw: bool,
    current_level: usize,
}

// We need a place to store the assets that LDtk references
// (the spritesheets and the color materials).These could be
// added to the Map struct above, but most games likely need
// these for purposes other than tilemaps so we store them
// in a seperate struct.
//
// If you only use one spritesheet you could just store it
// directly instead of using a Hash and doing all the
// looping below.
#[derive(Clone)]
struct VisualAssets {
    int_grid_materials: HashMap<i32, Vec<Handle<ColorMaterial>>>,
    spritesheets: HashMap<i32, Handle<TextureAtlas>>,
    entity_materials: HashMap<i32, Handle<ColorMaterial>>,
}

// When working with the LDtk info, you'll have nested loops
// for layers and tiles. You'll loop through the layers and
// for each layer you'll loop through the tiles for that layer.
// This struct holds the relevant layer information so we can
// easily pass it all at once to each tile.
//
// LDtk offers grid-based and pixel-based co-ordinates to work
// with. We include both here.
#[derive(Clone, Copy)]
struct LayerInfo {
    grid_width: i32,
    _grid_height: i32,
    grid_cell_size: i32,
    z_index: i32,
    px_width: f32,
    px_height: f32,
}

// There are a few things we need for entities that aren't
// in the Entity Instances but over in the Entity Defs in
// the LDtk JSON. In other words, we need to make a few
// more "__field" fields.
#[derive(Copy, Clone)]
struct ExtraEntDefs {
    __tile_id: i32,
    __width: i32,
    __height: i32,
    __scale: f32,
}
// implement a new() method
impl ExtraEntDefs {
    fn new() -> Self {
        Self {
            __tile_id: 0,
            __width: 0,
            __height: 0,
            __scale: 1.0,
        }
    }
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

// Our setup system will run once and will read in the LDtk file.
// It then loops through any referenced tilesets and sets those
// up as Bevy Assets. It does the same thing for color materials.
// Finally it saves the LdtkFile instance and handles to all the
// assets as Bevy Resources, which makes them "globals".
fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Create a new Map instance and set the values. This is where we
    // actually load in the LDtk file.
    let map = Map {
        ldtk_file: LdtkFile::new(LDTK_FILE_PATH.to_string()),
        redraw: true,
        current_level: 0,
    };

    // Create a new VisualAssets instance. This is where we will save
    // handles to all our assets so we can call them in our update()
    // function later.
    let mut visual_assets = VisualAssets {
        int_grid_materials: HashMap::new(),
        spritesheets: HashMap::new(),
        entity_materials: HashMap::new(),
    };

    // For each tileset referenced in the LDtk file, create a Texture Atlas
    // and store a Handle in a Hash. The key to the Hash is the value LDtk
    // assigns as the tileset's UID. If you know you only have one tileset
    // asset, you could simplify this and just load it like any other asset
    // using map.ldtk_file.defs.tilesets[0].rel_path
    for tileset in map.ldtk_file.defs.tilesets.iter() {
        let texture_handle = asset_server.load(&tileset.rel_path[..]);

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(tileset.tile_grid_size as f32, tileset.tile_grid_size as f32),
            (tileset.px_wid / tileset.tile_grid_size) as usize,
            (tileset.px_hei / tileset.tile_grid_size) as usize,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        visual_assets
            .spritesheets
            .insert(tileset.uid, texture_atlas_handle);
    }

    // LDtk IntGrids support setting colors for walls, sky, etc. If you are doing this
    // in your game you'd probably want to loop through your IntGrid layers and create
    // materials for each integer value. The Bevy snake tutorial has some good sample
    // code for using materials: https://mbuffett.com/posts/bevy-snake-tutorial/

    for layer in map
        .ldtk_file
        .defs
        .layers
        .iter()
        .filter(|f| f.__type == "IntGrid")
    {
        let mut colors = Vec::new();
        for values in layer.int_grid_values.iter() {
            let clr = match Color::hex(&values.color.clone()[1..]) {
                Ok(t) => t,
                Err(e) => {
                    println!("Error: {:?}", e);
                    Color::BLUE
                }
            };
            let col_mat = materials.add(ColorMaterial::from(clr));
            colors.push(col_mat);
        }
        visual_assets.int_grid_materials.insert(layer.uid, colors);
    }

    // LDtk supports placement of Entities in levels (player, chest, health potion, etc.)
    // If you are using this feature you may want to do additional setup here beyond
    // loading in the tilemap assets above.
    for ent in map.ldtk_file.defs.entities.iter() {
        let clr = match Color::hex(&ent.color.clone()[1..]) {
            Ok(t) => t,
            Err(e) => {
                println!("Error: {:?}", e);
                Color::BLUE
            }
        };
        let col_mat = materials.add(ColorMaterial::from(clr));

        visual_assets.entity_materials.insert(ent.uid, col_mat);
    }

    // add the LDtk object and the tile assets as resources and spawn a camera
    commands
        .insert_resource(map)
        .insert_resource(visual_assets)
        .spawn(Camera2dBundle::default());
}

// Our update system runs every game loop and, if the tiles are not spawned, will spawn them.
// In practice you'll likely want to split some of this up, but it's a decent start to
// having the ability to regenerate tiles as the current level changes, etc.
fn update(commands: &mut Commands, mut map: ResMut<Map>, visual_assets: Res<VisualAssets>) {
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
            grid_width: layer.__c_wid,
            _grid_height: layer.__c_hei,
            grid_cell_size: layer.__grid_size,
            z_index: 50 - idx as i32,
            px_width: layer.__c_wid as f32 * (layer.__grid_size as f32 * TILE_SCALE),
            px_height: layer.__c_hei as f32 * (layer.__grid_size as f32 * TILE_SCALE),
        };

        // Finally we match on the four possible kinds of Layer Instances and
        // handle each accordingly.
        match &layer.__type[..] {
            "Tiles" => {
                println!("Generating Tile Layer: {}", layer.__identifier);
                for tile in layer.grid_tiles.iter() {
                    display_tile(
                        layer_info,
                        tile,
                        commands,
                        visual_assets.spritesheets[&tileset_uid].clone(),
                    );
                }
            }
            "AutoLayer" => {
                println!("Generating AutoTile Layer: {}", layer.__identifier);
                for tile in layer.auto_layer_tiles.iter() {
                    display_tile(
                        layer_info,
                        tile,
                        commands,
                        visual_assets.spritesheets[&tileset_uid].clone(),
                    );
                }
            }
            "IntGrid" => {
                match layer.__tileset_def_uid {
                    Some(i) => {
                        // we have tiles, so handle just like Tiles and AutoLayers
                        println!("Generating IntGrid Layer w/ Tiles: {}", layer.__identifier);
                        for tile in layer.auto_layer_tiles.iter() {
                            display_tile(
                                layer_info,
                                tile,
                                commands,
                                visual_assets.spritesheets[&i].clone(),
                            );
                        }
                    }
                    None => {
                        // we do NOT have a corresponding tileset, so we need to use
                        // the color values to represent the level visually.
                        println!(
                            "Generating IntGrid Layer w/ Color Materials: {}",
                            layer.__identifier
                        );
                        for tile in layer.int_grid.iter() {
                            display_color(
                                layer_info,
                                tile,
                                commands,
                                visual_assets.int_grid_materials[&layer.layer_def_uid]
                                    [tile.v as usize]
                                    .clone(),
                            )
                        }
                    }
                }
            }
            "Entities" => {
                println!("Generating Entities Layer: {}", layer.__identifier);
                // Entities reference their tiles and colors within the instances
                for entity in layer.entity_instances.iter() {
                    // we need some extra fields from the defs section of the
                    // JSON that aren't included in the entity instances.
                    let mut extra_ent_defs = ExtraEntDefs::new();
                    for ent in map.ldtk_file.defs.entities.iter() {
                        if ent.uid == entity.def_uid {
                            extra_ent_defs.__tile_id = 0;
                            extra_ent_defs.__width = ent.width;
                            extra_ent_defs.__height = ent.height;
                            //__color: ent.color.clone(),
                        }
                        if ent.render_mode == "Tile" {
                            extra_ent_defs.__tile_id = ent.tile_id.unwrap();
                            for ts in map.ldtk_file.defs.tilesets.iter() {
                                if ts.uid == ent.tileset_id.unwrap() {
                                    extra_ent_defs.__scale =
                                        ent.width as f32 / ts.tile_grid_size as f32;
                                }
                            }
                        }
                    }

                    display_entity(
                        layer_info,
                        entity,
                        commands,
                        visual_assets.clone(),
                        &extra_ent_defs,
                    );
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
                layer_info.px_width,
                layer_info.px_height,
                layer_info.grid_cell_size,
                TILE_SCALE,
                tile.px[0],
                tile.px[1],
                layer_info.z_index,
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
// here's a basic example.
fn display_entity(
    layer_info: LayerInfo,
    entity: &EntityInstance,
    commands: &mut Commands,
    visual_assets: VisualAssets,
    extra_ent_defs: &ExtraEntDefs,
) {
    match &entity.__tile {
        Some(t) => {
            // process tile asset
            let handle: Handle<TextureAtlas> = visual_assets.spritesheets[&t.tileset_uid].clone();

            commands.spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: convert_to_world(
                        layer_info.px_width,
                        layer_info.px_height,
                        extra_ent_defs.__height,
                        TILE_SCALE,
                        entity.__grid[0] * layer_info.grid_cell_size,
                        entity.__grid[1] * layer_info.grid_cell_size,
                        layer_info.z_index,
                    ),
                    scale: Vec3::splat(extra_ent_defs.__scale * TILE_SCALE),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(extra_ent_defs.__tile_id as u32),
                texture_atlas: handle,
                ..Default::default()
            });
        }
        None => {
            // process color shape
            let handle: Handle<ColorMaterial> =
                visual_assets.entity_materials[&entity.def_uid].clone();
            commands.spawn(SpriteBundle {
                material: handle,
                sprite: Sprite::new(Vec2::new(
                    extra_ent_defs.__width as f32,
                    extra_ent_defs.__height as f32,
                )),
                transform: Transform {
                    translation: convert_to_world(
                        layer_info.px_width,
                        layer_info.px_height,
                        extra_ent_defs.__height,
                        TILE_SCALE,
                        entity.__grid[0] * layer_info.grid_cell_size,
                        entity.__grid[1] * layer_info.grid_cell_size,
                        layer_info.z_index,
                    ),
                    scale: Vec3::splat(TILE_SCALE),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn display_color(
    layer_info: LayerInfo,
    tile: &IntGrid,
    commands: &mut Commands,
    handle: Handle<ColorMaterial>,
) {
    let x = tile.coord_id % layer_info.grid_width;
    let y = tile.coord_id / layer_info.grid_width;
    //println!("Tile ({},{}) is {}... {:?}", x, y, tile.v, handle);
    commands.spawn(SpriteBundle {
        material: handle,
        sprite: Sprite::new(Vec2::new(
            layer_info.grid_cell_size as f32,
            layer_info.grid_cell_size as f32,
        )),
        transform: Transform {
            translation: convert_to_world(
                layer_info.px_width,
                layer_info.px_height,
                layer_info.grid_cell_size,
                TILE_SCALE,
                x * layer_info.grid_cell_size,
                y as i32 * layer_info.grid_cell_size,
                layer_info.z_index,
            ),
            scale: Vec3::splat(TILE_SCALE),
            ..Default::default()
        },
        ..Default::default()
    });
}

// LDtk provides pixel locations starting in the top left. For Bevy we need to
// flip the Y axis and offset from the center of the screen.
fn convert_to_world(
    width: f32,
    height: f32,
    grid_size: i32,
    scale: f32,
    x: i32,
    y: i32,
    z: i32,
) -> Vec3 {
    let world_x = (x as f32 * scale) + (grid_size as f32 * scale / 2.) - (width / 2.);
    let world_y = -(y as f32 * scale) - (grid_size as f32 * scale / 2.) + (height / 2.);
    let world_z = z as f32;
    // println!(
    //     "Drawing at {}, {}, {} with scale {}",
    //     world_x, world_y, world_z, scale
    // );
    Vec3::new(world_x, world_y, world_z)
}

// Bevy doesn't have sprite flipping built in, so if tile needs to flip
//  on either axis, flip it
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
