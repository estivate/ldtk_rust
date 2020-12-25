use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub identifier: String,
    pub uid: i32,
    pub world_x: i32,
    pub world_y: i32,
    pub px_wid: i32,
    pub px_hei: i32,
    #[serde(rename = "__bgColor")]
    pub __bg_color: String,
    pub bg_color: Option<String>,
    pub layer_instances: Vec<LayerInstance>,
    //#[serde(rename = "__neighbours")]
    //pub __neighbours: Vec<Neighbor>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInstance {
    #[serde(rename = "__identifier")]
    pub __identifier: String,
    #[serde(rename = "__type")]
    pub __type: String,
    #[serde(rename = "__cWid")]
    pub __c_wid: i32,
    #[serde(rename = "__cHei")]
    pub __c_hei: i32,
    #[serde(rename = "__gridSize")]
    pub __grid_size: i32,
    #[serde(rename = "__opacity")]
    pub __opacity: f32,
    #[serde(rename = "__pxTotalOffsetX")]
    pub __px_total_offset_x: i32,
    #[serde(rename = "__pxTotalOffsetY")]
    pub __px_total_offset_y: i32,
    #[serde(rename = "__tilesetDefUid")]
    pub __tileset_def_uid: Option<i32>,
    #[serde(rename = "__tilesetRelPath")]
    pub __tileset_rel_path: Option<String>,
    pub level_id: i32,
    pub layer_def_uid: i32,
    pub px_offset_x: i32,
    pub px_offset_y: i32,
    pub int_grid: Vec<IntGrid>,
    pub auto_layer_tiles: Vec<TileInstance>,
    pub seed: i32,
    pub grid_tiles: Vec<TileInstance>,
    pub entity_instances: Vec<EntityInstance>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntGrid {
    pub coord_id: i32,
    pub v: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileInstance {
    pub px: Vec<i32>,
    pub src: Vec<i32>,
    pub f: u8,
    pub t: i32,
    pub d: Vec<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityInstance {
    #[serde(rename = "__grid")]
    pub __grid: Vec<i32>,
    #[serde(rename = "__identifier")]
    pub __identifier: String,
    #[serde(rename = "__tile")]
    pub __tile: Option<OptionalTile>,
    pub def_uid: i32,
    pub field_instances: Vec<FieldInstance>,
    pub px: Vec<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalTile {
    pub src_rect: Vec<i32>,
    pub tileset_uid: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInstance {
    #[serde(rename = "__identifier")]
    pub __identifier: String,
    #[serde(rename = "__type")]
    pub __type: String,
    #[serde(rename = "__value")]
    pub __value: Value,
    pub def_uid: i32,
}

// #[derive(Debug, Deserialize, Clone)]
// #[serde(untagged)]
// pub enum FieldInstanceValueTypes {
//     S(String),
//     I(i32),
//     B(bool),
//     F(f32),
//     VS(Vec<String>),
//     VI(Vec<i32>),
//     VB(Vec<bool>),
//     VF(Vec<f32>),
// }
