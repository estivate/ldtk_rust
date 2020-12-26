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
    pub display_opacity: f32,
    pub px_offset_x: i32,
    pub px_offset_y: i32,
    pub int_grid_values: Vec<IntGridValuesColors>,
    pub auto_tileset_def_uid: Option<i32>,
    pub auto_rule_groups: Vec<AutoRuleGroup>,
    pub auto_source_layer_def_uid: Option<i32>,
    pub tileset_def_uid: Option<i32>,
    pub tile_pivot_x: i32,
    pub tile_pivot_y: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntGridValuesColors {
    pub color: String,
    pub identifier: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoRuleGroup {
    pub active: bool,
    pub collapsed: bool,
    pub name: String,
    pub rules: Vec<AutoLayerRuleDefinition>,
    pub uid: i32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoLayerRuleDefinition {
    pub active: bool,
    pub break_on_match: bool,
    pub chance: f32,
    pub checker: String, // documented as boolean?
    pub flip_x: bool,
    pub flip_y: bool,
    pub pattern: Vec<i32>,
    pub perlin_active: bool,
    pub perlin_octaves: f32,
    pub perlin_scale: f32,
    pub perlin_seed: f32,
    pub pivot_x: f32,
    pub pivot_y: f32,
    pub size: i32,
    pub tile_ids: Vec<i32>,
    //pub tile_mode:
    pub uid: i32,
    pub x_modulo: i32,
    pub y_modulo: i32,
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
