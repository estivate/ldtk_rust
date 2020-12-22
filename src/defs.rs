use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defs {
    pub layers: Vec<Layer>,
    pub entities: Vec<Entity>,
    pub tilesets: Vec<TileSet>,
    //pub enums:
    //pub external_enums:
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    #[serde(rename = "__type")]
    pub __type: String,
    pub identifier: String,
    #[serde(rename = "type")]
    pub layer_type: String,
    pub uid: i32,
    pub grid_size: i32,
    pub display_opacity: i32,
    pub px_offset_x: i32,
    pub px_offset_y: i32,
    //int_grid_values: UNUSED?
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub identifier: String,
    pub uid: i32,
    pub width: i32,
    pub height: i32,
    pub color: String,
    pub render_mode: String,
    pub show_name: bool,
    pub tileset_id: Option<i32>,
    pub tile_id: Option<i32>,
    pub tile_render_mode: String,
    pub max_per_level: i32,
    pub limit_behavior: String,
    pub pivot_x: f32,
    pub pivot_y: f32,
    pub field_defs: Vec<FieldDefs>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileSet {
    pub identifier: String,
    pub uid: i32,
    pub rel_path: String,
    pub px_wid: i32,
    pub px_hei: i32,
    pub tile_grid_size: i32,
    pub spacing: i32,
    pub padding: i32,
    //saved_selections: UNUSED
    //cached_pixel_data: UNUSED
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefs {
    pub identifier: String,
    #[serde(rename = "__type")]
    pub __type: String,
    pub uid: i32,
    // #[serde(rename = "type")]
    //pub field_def_type: String, // this can be string OR object?!
    pub is_array: bool,
    pub can_be_null: bool,
    pub array_min_length: Option<i32>,
    pub array_max_length: Option<i32>,
    // much more, any of this used?
}
