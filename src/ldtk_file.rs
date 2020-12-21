use serde::Deserialize;

use crate::{defs::Defs, levels::Level};

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
