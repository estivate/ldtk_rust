
use serde::*;
use std::collections::HashMap;

/// This file is a JSON schema of files created by LDtk level editor (https://ldtk.io).
///
/// This is the root of any Project JSON file. It contains:  - the project settings, - an
/// array of levels - and a definition object (that can probably be safely ignored for most
/// users).
#[derive(Serialize, Deserialize)]
pub struct Project {
    /// Project background color
    #[serde(rename = "bgColor")]
    pub bg_color: String,
    /// Default grid size for new layers
    #[serde(rename = "defaultGridSize")]
    pub default_grid_size: i64,
    /// Default background color of levels
    #[serde(rename = "defaultLevelBgColor")]
    pub default_level_bg_color: String,
    /// Default X pivot (0 to 1) for new entities
    #[serde(rename = "defaultPivotX")]
    pub default_pivot_x: f64,
    /// Default Y pivot (0 to 1) for new entities
    #[serde(rename = "defaultPivotY")]
    pub default_pivot_y: f64,
    /// A structure containing all the definitions of this project
    pub defs: Option<Definitions>,
    /// If TRUE, a Tiled compatible file will also be generated along with the LDtk JSON file
    /// (default is FALSE)
    #[serde(rename = "exportTiled")]
    pub export_tiled: bool,
    /// If TRUE, one file will be saved the project (incl. all its definitions) and one file
    /// per-level in a sub-folder.
    #[serde(rename = "externalLevels")]
    pub external_levels: bool,
    /// File format version
    #[serde(rename = "jsonVersion")]
    pub json_version: String,
    /// All levels. The order of this array is only relevant in `LinearHorizontal` and
    /// `linearVertical` world layouts (see `worldLayout` value). Otherwise, you should refer to
    /// the `worldX`,`worldY` coordinates of each Level.
    pub levels: Vec<Level>,
    /// If TRUE, the Json is partially minified (no indentation, nor line breaks, default is
    /// FALSE)
    #[serde(rename = "minifyJson")]
    pub minify_json: bool,
    #[serde(rename = "nextUid")]
    pub next_uid: i64,
    /// Height of the world grid in pixels.
    #[serde(rename = "worldGridHeight")]
    pub world_grid_height: i64,
    /// Width of the world grid in pixels.
    #[serde(rename = "worldGridWidth")]
    pub world_grid_width: i64,
    /// An enum that describes how levels are organized in this project (ie. linearly or in a 2D
    /// space). Possible values: `Free`, `GridVania`, `LinearHorizontal`, `LinearVertical`
    #[serde(rename = "worldLayout")]
    pub world_layout: Option<WorldLayout>,
}

/// A structure containing all the definitions of this project
///
/// If you're writing your own LDtk importer, you should probably ignore MOST stuff in the
/// `defs` section, as it contains data that are specifically useful to the editor. Data that
/// is useful to game devs is duplicated in fields prefixed with a double underscore (eg.
/// `__identifier` or `__type`).   The 2 only definition types you might need here are
/// **Tilesets** and **Enums**.
#[derive(Serialize, Deserialize)]
pub struct Definitions {
    pub entities: Vec<EntityDefinition>,
    pub enums: Vec<EnumDefinition>,
    /// Note: external enums are exactly the same as `enums`, except they have a `relPath` to
    /// point to an external source file.
    #[serde(rename = "externalEnums")]
    pub external_enums: Vec<EnumDefinition>,
    pub layers: Vec<LayerDefinition>,
    pub tilesets: Vec<TilesetDefinition>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDefinition {
    /// Base entity color
    pub color: String,
    /// Array of field definitions
    #[serde(rename = "fieldDefs")]
    pub field_defs: Vec<FieldDefinition>,
    /// Pixel height
    pub height: i64,
    /// Unique String identifier
    pub identifier: String,
    /// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
    #[serde(rename = "limitBehavior")]
    pub limit_behavior: Option<LimitBehavior>,
    /// Max instances per level
    #[serde(rename = "maxPerLevel")]
    pub max_per_level: i64,
    /// Pivot X coordinate (from 0 to 1.0)
    #[serde(rename = "pivotX")]
    pub pivot_x: f64,
    /// Pivot Y coordinate (from 0 to 1.0)
    #[serde(rename = "pivotY")]
    pub pivot_y: f64,
    /// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
    #[serde(rename = "renderMode")]
    pub render_mode: Option<RenderMode>,
    /// Display entity name in editor
    #[serde(rename = "showName")]
    pub show_name: bool,
    /// Tile ID used for optional tile display
    #[serde(rename = "tileId")]
    pub tile_id: Option<i64>,
    /// Possible values: `Stretch`, `Crop`
    #[serde(rename = "tileRenderMode")]
    pub tile_render_mode: Option<TileRenderMode>,
    /// Tileset ID used for optional tile display
    #[serde(rename = "tilesetId")]
    pub tileset_id: Option<i64>,
    /// Unique Int identifier
    pub uid: i64,
    /// Pixel width
    pub width: i64,
}

/// This section is mostly only intended for the LDtk editor app itself. You can safely
/// ignore it.
#[derive(Serialize, Deserialize)]
pub struct FieldDefinition {
    /// Human readable value type (eg. `Int`, `Float`, `Point`, etc.). If the field is an array,
    /// this field will look like `Array<...>` (eg. `Array<Int>`, `Array<Point>` etc.)
    #[serde(rename = "__type")]
    pub field_definition_type: String,
    /// Optional list of accepted file extensions for FilePath value type. Includes the dot:
    /// `.ext`
    #[serde(rename = "acceptFileTypes")]
    pub accept_file_types: Option<Vec<String>>,
    /// Array max length
    #[serde(rename = "arrayMaxLength")]
    pub array_max_length: Option<i64>,
    /// Array min length
    #[serde(rename = "arrayMinLength")]
    pub array_min_length: Option<i64>,
    /// TRUE if the value can be null. For arrays, TRUE means it can contain null values
    /// (exception: array of Points can't have null values).
    #[serde(rename = "canBeNull")]
    pub can_be_null: bool,
    /// Default value if selected value is null or invalid.
    #[serde(rename = "defaultOverride")]
    pub default_override: Option<serde_json::Value>,
    #[serde(rename = "editorAlwaysShow")]
    pub editor_always_show: bool,
    /// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `PointStar`,
    /// `PointPath`, `RadiusPx`, `RadiusGrid`
    #[serde(rename = "editorDisplayMode")]
    pub editor_display_mode: Option<EditorDisplayMode>,
    /// Possible values: `Above`, `Center`, `Beneath`
    #[serde(rename = "editorDisplayPos")]
    pub editor_display_pos: Option<EditorDisplayPos>,
    /// Unique String identifier
    pub identifier: String,
    /// TRUE if the value is an array of multiple values
    #[serde(rename = "isArray")]
    pub is_array: bool,
    /// Max limit for value, if applicable
    pub max: Option<f64>,
    /// Min limit for value, if applicable
    pub min: Option<f64>,
    /// Optional regular expression that needs to be matched to accept values. Expected format:
    /// `/some_reg_ex/g`, with optional "i" flag.
    pub regex: Option<String>,
    /// Internal type enum
    #[serde(rename = "type")]
    pub purple_type: Option<serde_json::Value>,
    /// Unique Intidentifier
    pub uid: i64,
}

#[derive(Serialize, Deserialize)]
pub struct EnumDefinition {
    #[serde(rename = "externalFileChecksum")]
    pub external_file_checksum: Option<String>,
    /// Relative path to the external file providing this Enum
    #[serde(rename = "externalRelPath")]
    pub external_rel_path: Option<String>,
    /// Tileset UID if provided
    #[serde(rename = "iconTilesetUid")]
    pub icon_tileset_uid: Option<i64>,
    /// Unique String identifier
    pub identifier: String,
    /// Unique Int identifier
    pub uid: i64,
    /// All possible enum values, with their optional Tile infos.
    pub values: Vec<HashMap<String, Option<serde_json::Value>>>,
}

#[derive(Serialize, Deserialize)]
pub struct LayerDefinition {
    /// Type of the layer (*IntGrid, Entities, Tiles or AutoLayer*)
    #[serde(rename = "__type")]
    pub layer_definition_type: String,
    /// Contains all the auto-layer rule definitions.
    #[serde(rename = "autoRuleGroups")]
    pub auto_rule_groups: Vec<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "autoSourceLayerDefUid")]
    pub auto_source_layer_def_uid: Option<i64>,
    /// Reference to the Tileset UID being used by this auto-layer rules
    #[serde(rename = "autoTilesetDefUid")]
    pub auto_tileset_def_uid: Option<i64>,
    /// Opacity of the layer (0 to 1.0)
    #[serde(rename = "displayOpacity")]
    pub display_opacity: f64,
    /// Width and height of the grid in pixels
    #[serde(rename = "gridSize")]
    pub grid_size: i64,
    /// Unique String identifier
    pub identifier: String,
    #[serde(rename = "intGridValues")]
    pub int_grid_values: Vec<HashMap<String, Option<serde_json::Value>>>,
    /// X offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: i64,
    /// Y offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: i64,
    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    #[serde(rename = "tilePivotX")]
    pub tile_pivot_x: f64,
    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    #[serde(rename = "tilePivotY")]
    pub tile_pivot_y: f64,
    /// Reference to the Tileset UID being used by this tile layer
    #[serde(rename = "tilesetDefUid")]
    pub tileset_def_uid: Option<i64>,
    /// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
    /// `AutoLayer`
    #[serde(rename = "type")]
    pub purple_type: Option<Type>,
    /// Unique Int identifier
    pub uid: i64,
}

/// The `Tileset` definition is the most useful part among project definitions. It contains
/// some extra informations about each integrated tileset. If you only had to parse one
/// definition section, that would be the one.
#[derive(Serialize, Deserialize)]
pub struct TilesetDefinition {
    /// The following data is used internally for various optimizations. It's always synced with
    /// source image changes.
    #[serde(rename = "cachedPixelData")]
    pub cached_pixel_data: Option<HashMap<String, Option<serde_json::Value>>>,
    /// Unique String identifier
    pub identifier: String,
    /// Distance in pixels from image borders
    pub padding: i64,
    /// Image height in pixels
    #[serde(rename = "pxHei")]
    pub px_hei: i64,
    /// Image width in pixels
    #[serde(rename = "pxWid")]
    pub px_wid: i64,
    /// Path to the source file, relative to the current project JSON file
    #[serde(rename = "relPath")]
    pub rel_path: String,
    /// Array of group of tiles selections, only meant to be used in the editor
    #[serde(rename = "savedSelections")]
    pub saved_selections: Vec<HashMap<String, Option<serde_json::Value>>>,
    /// Space in pixels between all tiles
    pub spacing: i64,
    #[serde(rename = "tileGridSize")]
    pub tile_grid_size: i64,
    /// Unique Intidentifier
    pub uid: i64,
}

/// This section contains all the level data. It can be found in 2 distinct forms, depending
/// on Project current settings:  - If "*Separate level files*" is **disabled** (default):
/// full level data is *embedded* inside the main Project JSON file, - If "*Separate level
/// files*" is **enabled**: level data is stored in *separate* standalone `.ldtkl` files (one
/// per level). In this case, the main Project JSON file will still contain most level data,
/// except heavy sections, like the `layerInstances` array (which will be null). The
/// `externalRelPath` string points to the `ldtkl` file.  A `ldtkl` file is just a JSON file
/// containing exactly what is described below.
#[derive(Serialize, Deserialize)]
pub struct Level {
    /// Background color of the level (same as `bgColor`, except the default value is
    /// automatically used here if its value is `null`)
    #[serde(rename = "__bgColor")]
    pub bg_color: String,
    /// An array listing all other levels touching this one on the world map. The `dir` is a
    /// single lowercase character tipping on the level location (`n`orth, `s`outh, `w`est,
    /// `e`ast). In "linear" world layouts, this array is populated with previous/next levels in
    /// array, and `dir` depends on the linear horizontal/vertical layout.
    #[serde(rename = "__neighbours")]
    pub neighbours: Vec<HashMap<String, Option<serde_json::Value>>>,
    /// Background color of the level. If `null`, the project `defaultLevelBgColor` should be
    /// used.
    #[serde(rename = "bgColor")]
    pub level_bg_color: Option<String>,
    /// This value is not null if the project option "*Save levels separately*" is enabled. In
    /// this case, this **relative** path points to the level Json file.
    #[serde(rename = "externalRelPath")]
    pub external_rel_path: Option<String>,
    /// Unique String identifier
    pub identifier: String,
    /// An array containing all Layer instances. **IMPORTANT**: if the project option "*Save
    /// levels separately*" is enabled, this field will be `null`.<br/>  TODO specify array order
    #[serde(rename = "layerInstances")]
    pub layer_instances: Option<Vec<LayerInstance>>,
    /// Height of the level in pixels
    #[serde(rename = "pxHei")]
    pub px_hei: i64,
    /// Width of the level in pixels
    #[serde(rename = "pxWid")]
    pub px_wid: i64,
    /// Unique Int identifier
    pub uid: i64,
    /// World X coordinate in pixels
    #[serde(rename = "worldX")]
    pub world_x: i64,
    /// World Y coordinate in pixels
    #[serde(rename = "worldY")]
    pub world_y: i64,
}

#[derive(Serialize, Deserialize)]
pub struct LayerInstance {
    /// Grid-based height
    #[serde(rename = "__cHei")]
    pub c_hei: i64,
    /// Grid-based width
    #[serde(rename = "__cWid")]
    pub c_wid: i64,
    /// Grid size
    #[serde(rename = "__gridSize")]
    pub grid_size: i64,
    /// Unique String identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,
    /// Layer opacity as Float [0-1]
    #[serde(rename = "__opacity")]
    pub opacity: f64,
    /// Total layer X pixel offset, including both instance and definition offsets.
    #[serde(rename = "__pxTotalOffsetX")]
    pub px_total_offset_x: i64,
    /// Total layer Y pixel offset, including both instance and definition offsets.
    #[serde(rename = "__pxTotalOffsetY")]
    pub px_total_offset_y: i64,
    /// The definition UID of corresponding Tileset, if any.
    #[serde(rename = "__tilesetDefUid")]
    pub tileset_def_uid: Option<i64>,
    /// The relative path to corresponding Tileset, if any.
    #[serde(rename = "__tilesetRelPath")]
    pub tileset_rel_path: Option<String>,
    /// Layer type (possible values: IntGrid, Entities, Tiles or AutoLayer)
    #[serde(rename = "__type")]
    pub layer_instance_type: String,
    /// An array containing all tiles generated by Auto-layer rules. The array is already sorted
    /// in display order (ie. 1st tile is beneath 2nd, which is beneath 3rd etc.).<br/><br/>
    /// Note: if multiple tiles are stacked in the same cell as the result of different rules,
    /// all tiles behind opaque ones will be discarded.
    #[serde(rename = "autoLayerTiles")]
    pub auto_layer_tiles: Vec<TileInstance>,
    #[serde(rename = "entityInstances")]
    pub entity_instances: Vec<EntityInstance>,
    #[serde(rename = "gridTiles")]
    pub grid_tiles: Vec<TileInstance>,
    #[serde(rename = "intGrid")]
    pub int_grid: Vec<HashMap<String, Option<serde_json::Value>>>,
    /// Reference the Layer definition UID
    #[serde(rename = "layerDefUid")]
    pub layer_def_uid: i64,
    /// Reference to the UID of the level containing this layer instance
    #[serde(rename = "levelId")]
    pub level_id: i64,
    /// X offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to
    /// the `LayerDef` optional offset, see `__pxTotalOffsetX`)
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: i64,
    /// Y offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to
    /// the `LayerDef` optional offset, see `__pxTotalOffsetY`)
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: i64,
    /// Random seed used for Auto-Layers rendering
    pub seed: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TileInstance {
    /// Internal data used by the editor.<br/>  For auto-layer tiles: `[ruleId, coordId]`.<br/>
    /// For tile-layer tiles: `[coordId]`.
    pub d: Vec<i64>,
    /// "Flip bits", a 2-bits integer to represent the mirror transformations of the tile.<br/>
    /// - Bit 0 = X flip<br/>   - Bit 1 = Y flip<br/>   Examples: f=0 (no flip), f=1 (X flip
    /// only), f=2 (Y flip only), f=3 (both flips)
    pub f: i64,
    /// Pixel coordinates of the tile in the **layer** (`[x,y]` format). Don't forget optional
    /// layer offsets, if they exist!
    pub px: Vec<i64>,
    /// Pixel coordinates of the tile in the **tileset** (`[x,y]` format)
    pub src: Vec<i64>,
    /// The *Tile ID* in the corresponding tileset.
    pub t: i64,
}

#[derive(Serialize, Deserialize)]
pub struct EntityInstance {
    /// Grid-based coordinates (`[x,y]` format)
    #[serde(rename = "__grid")]
    pub grid: Vec<i64>,
    /// Unique String identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,
    /// Optional Tile used to display this entity (it could either be the default Entity tile, or
    /// some tile provided by a field value, like an Enum).
    #[serde(rename = "__tile")]
    pub tile: Option<HashMap<String, Option<serde_json::Value>>>,
    /// Reference of the **Entity definition** UID
    #[serde(rename = "defUid")]
    pub def_uid: i64,
    #[serde(rename = "fieldInstances")]
    pub field_instances: Vec<FieldInstance>,
    /// Pixel coordinates (`[x,y]` format) in current level coordinate space. Don't forget
    /// optional layer offsets, if they exist!
    pub px: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldInstance {
    /// Unique String identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,
    /// Type of the field, such as Int, Float, Enum(enum_name), Bool, etc.
    #[serde(rename = "__type")]
    pub field_instance_type: String,
    /// Actual value of the field instance. The value type may vary, depending on `__type`
    /// (Integer, Boolean, String etc.)<br/>  It can also be an `Array` of those same types.
    #[serde(rename = "__value")]
    pub value: Option<serde_json::Value>,
    /// Reference of the **Field definition** UID
    #[serde(rename = "defUid")]
    pub def_uid: i64,
    #[serde(rename = "realEditorValues")]
    pub real_editor_values: Vec<Option<serde_json::Value>>,
}

/// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `PointStar`,
/// `PointPath`, `RadiusPx`, `RadiusGrid`
#[derive(Serialize, Deserialize)]
pub enum EditorDisplayMode {
    EntityTile,
    Hidden,
    NameAndValue,
    PointPath,
    PointStar,
    RadiusGrid,
    RadiusPx,
    ValueOnly,
}

/// Possible values: `Above`, `Center`, `Beneath`
#[derive(Serialize, Deserialize)]
pub enum EditorDisplayPos {
    Above,
    Beneath,
    Center,
}

/// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
#[derive(Serialize, Deserialize)]
pub enum LimitBehavior {
    DiscardOldOnes,
    MoveLastOne,
    PreventAdding,
}

/// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
#[derive(Serialize, Deserialize)]
pub enum RenderMode {
    Cross,
    Ellipse,
    Rectangle,
    Tile,
}

/// Possible values: `Stretch`, `Crop`
#[derive(Serialize, Deserialize)]
pub enum TileRenderMode {
    Crop,
    Stretch,
}

/// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
/// `AutoLayer`
#[derive(Serialize, Deserialize)]
pub enum Type {
    AutoLayer,
    Entities,
    IntGrid,
    Tiles,
}

/// An enum that describes how levels are organized in this project (ie. linearly or in a 2D
/// space). Possible values: `Free`, `GridVania`, `LinearHorizontal`, `LinearVertical`
#[derive(Serialize, Deserialize)]
pub enum WorldLayout {
    Free,
    GridVania,
    LinearHorizontal,
    LinearVertical,
}
