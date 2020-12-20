use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LdtkFile {
    pub json_version: String,
    pub default_pivot_x: i32,
    pub default_pivot_y: i32,
    pub default_grid_size: i32,
    pub bg_color: String,
    pub default_level_bg_color: String,
    pub next_uid: i32,
    pub minify_json: bool,
    pub export_tiled: bool,
    pub world_layout: String,
    pub world_grid_width: i32,
    pub world_grid_height: i32,
    pub defs: Defs,
    pub levels: Vec<Level>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    identifier: String,
    grid_size: i32,
    display_opacity: i32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defs {
    pub layers: Vec<Layer>,
    // entities: Vec<Entity>,
    pub tilesets: Vec<TileSet>,
    // enums:
    // external_enums:
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub identifier: String,
    pub world_x: i32,
    pub world_y: i32,
    pub px_wid: i32,
    pub px_hei: i32,
    #[serde(rename = "__bgColor")]
    pub __bg_color: String,
    pub layer_instances: Vec<LayerInstance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInstance {
    #[serde(rename = "__identifier")]
    pub __identifier: String,
    #[serde(rename = "__type")]
    pub __type: String,
    #[serde(rename = "__tilesetDefUid")]
    pub __tileset_def_uid: Option<i32>,
    #[serde(rename = "__cWid")]
    pub __c_wid: i32,
    #[serde(rename = "__cHei")]
    pub __c_hei: i32,
    #[serde(rename = "__gridSize")]
    pub __grid_size: i32,
    pub int_grid: Vec<IntGridObject>,
    pub auto_layer_tiles: Vec<AutoLayerTile>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntGridObject {
    pub coord_id: i32,
    pub v: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoLayerTile {
    pub px: Vec<i32>,
    pub src: Vec<i32>,
    pub f: u8,
    pub t: i32,
    pub d: Vec<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileSet {
    pub uid: i32,
    pub identifier: String,
    pub rel_path: String,
    pub px_wid: i32,
    pub px_hei: i32,
    pub tile_grid_size: i32,
    pub spacing: i32,
    pub padding: i32,
}
