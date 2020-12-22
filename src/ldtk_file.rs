use serde::Deserialize;

use crate::{defs::Defs, levels::Level};
use std::{fs::File, path::Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LdtkFile {
    pub json_version: String,
    pub default_pivot_x: f32,
    pub default_pivot_y: f32,
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
    #[serde(skip)]
    pub redraw: bool,
}
impl LdtkFile {
    /// Takes a path to an LDtk file and returns a rust struct
    /// of the parsed JSON. It will mirror the original JSON
    /// structure for the most part, but with CamelCase names
    /// converted to their idiomatic rust counterparts (for
    /// instance, "jsonVersion" will become "json_version").
    ///
    /// # Example
    ///
    /// ```
    /// extern crate ldtk;
    ///
    /// fn main() {
    ///   let file_path = "assets/AutoLayers_4_Advanced.ldtk".to_string();
    ///   let ldtk = LdtkFile::new(file_path);
    ///   println!("First level pixel height is {}!", ldtk.levels[0].px_hei);
    /// }
    /// ```
    ///
    ///
    pub fn new(f: String) -> Self {
        let json_file_path = Path::new(&f);
        let file = File::open(json_file_path).expect("file not found");
        let o: LdtkFile = serde_json::from_reader(file).expect("error while reading");
        o
    }
}
