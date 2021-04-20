// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::*;
use std::collections::HashMap;

/// This file is a JSON schema of files created by LDtk level editor (https://ldtk.io).
///
/// This is the root of any Project JSON file. It contains:  - the project settings, - an
/// array of levels, - and a definition object (that can probably be safely ignored for most
/// users).
#[derive(Serialize, Deserialize)]
pub struct Project {
    /// Number of backup files to keep, if the `backupOnSave` is TRUE
    #[serde(rename = "backupLimit")]
    pub backup_limit: i64,
    /// If TRUE, an extra copy of the project will be created in a sub folder, when saving.
    #[serde(rename = "backupOnSave")]
    pub backup_on_save: bool,
    /// Project background color
    #[serde(rename = "bgColor")]
    pub bg_color: String,
    /// Default grid size for new layers
    #[serde(rename = "defaultGridSize")]
    pub default_grid_size: i64,
    /// Default background color of levels
    #[serde(rename = "defaultLevelBgColor")]
    pub default_level_bg_color: String,
    /// Default new level height
    #[serde(rename = "defaultLevelHeight")]
    pub default_level_height: i64,
    /// Default new level width
    #[serde(rename = "defaultLevelWidth")]
    pub default_level_width: i64,
    /// Default X pivot (0 to 1) for new entities
    #[serde(rename = "defaultPivotX")]
    pub default_pivot_x: f64,
    /// Default Y pivot (0 to 1) for new entities
    #[serde(rename = "defaultPivotY")]
    pub default_pivot_y: f64,
    /// A structure containing all the definitions of this project
    pub defs: Definitions,
    /// If TRUE, all layers in all levels will also be exported as PNG along with the project
    /// file (default is FALSE)
    #[serde(rename = "exportPng")]
    pub export_png: bool,
    /// If TRUE, a Tiled compatible file will also be generated along with the LDtk JSON file
    /// (default is FALSE)
    #[serde(rename = "exportTiled")]
    pub export_tiled: bool,
    /// If TRUE, one file will be saved for the project (incl. all its definitions) and one file
    /// in a sub-folder for each level.
    #[serde(rename = "externalLevels")]
    pub external_levels: bool,
    /// An array containing various advanced flags (ie. options or other states). Possible
    /// values: `DiscardPreCsvIntGrid`, `IgnoreBackupSuggest`
    pub flags: Vec<Flag>,
    /// File format version
    #[serde(rename = "jsonVersion")]
    pub json_version: String,
    /// The default naming convention for level identifiers.
    #[serde(rename = "levelNamePattern")]
    pub level_name_pattern: String,
    /// All levels. The order of this array is only relevant in `LinearHorizontal` and
    /// `linearVertical` world layouts (see `worldLayout` value). Otherwise, you should refer to
    /// the `worldX`,`worldY` coordinates of each Level.
    pub levels: Vec<Level>,
    /// If TRUE, the Json is partially minified (no indentation, nor line breaks, default is
    /// FALSE)
    #[serde(rename = "minifyJson")]
    pub minify_json: bool,
    /// Next Unique integer ID available
    #[serde(rename = "nextUid")]
    pub next_uid: i64,
    /// File naming pattern for exported PNGs
    #[serde(rename = "pngFilePattern")]
    pub png_file_pattern: Option<String>,
    /// Height of the world grid in pixels.
    #[serde(rename = "worldGridHeight")]
    pub world_grid_height: i64,
    /// Width of the world grid in pixels.
    #[serde(rename = "worldGridWidth")]
    pub world_grid_width: i64,
    /// An enum that describes how levels are organized in this project (ie. linearly or in a 2D
    /// space). Possible values: `Free`, `GridVania`, `LinearHorizontal`, `LinearVertical`
    #[serde(rename = "worldLayout")]
    pub world_layout: WorldLayout,
}

/// A structure containing all the definitions of this project
///
/// If you're writing your own LDtk importer, you should probably just ignore *most* stuff in
/// the `defs` section, as it contains data that are mostly important to the editor. To keep
/// you away from the `defs` section and avoid some unnecessary JSON parsing, important data
/// from definitions is often duplicated in fields prefixed with a double underscore (eg.
/// `__identifier` or `__type`).  The 2 only definition types you might need here are
/// **Tilesets** and **Enums**.
#[derive(Serialize, Deserialize)]
pub struct Definitions {
    /// All entities, including their custom fields
    pub entities: Vec<EntityDefinition>,
    pub enums: Vec<EnumDefinition>,
    /// Note: external enums are exactly the same as `enums`, except they have a `relPath` to
    /// point to an external source file.
    #[serde(rename = "externalEnums")]
    pub external_enums: Vec<EnumDefinition>,
    pub layers: Vec<LayerDefinition>,
    /// An array containing all custom fields available to all levels.
    #[serde(rename = "levelFields")]
    pub level_fields: Vec<FieldDefinition>,
    pub tilesets: Vec<TilesetDefinition>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDefinition {
    /// Base entity color
    pub color: String,
    /// Array of field definitions
    #[serde(rename = "fieldDefs")]
    pub field_defs: Vec<FieldDefinition>,
    #[serde(rename = "fillOpacity")]
    pub fill_opacity: f64,
    /// Pixel height
    pub height: i64,
    pub hollow: bool,
    /// Unique String identifier
    pub identifier: String,
    /// Only applies to entities resizable on both X/Y. If TRUE, the entity instance width/height
    /// will keep the same aspect ratio as the definition.
    #[serde(rename = "keepAspectRatio")]
    pub keep_aspect_ratio: bool,
    /// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
    #[serde(rename = "limitBehavior")]
    pub limit_behavior: LimitBehavior,
    /// If TRUE, the maxCount is a "per world" limit, if FALSE, it's a "per level". Possible
    /// values: `PerLayer`, `PerLevel`, `PerWorld`
    #[serde(rename = "limitScope")]
    pub limit_scope: LimitScope,
    #[serde(rename = "lineOpacity")]
    pub line_opacity: f64,
    /// Max instances count
    #[serde(rename = "maxCount")]
    pub max_count: i64,
    /// Pivot X coordinate (from 0 to 1.0)
    #[serde(rename = "pivotX")]
    pub pivot_x: f64,
    /// Pivot Y coordinate (from 0 to 1.0)
    #[serde(rename = "pivotY")]
    pub pivot_y: f64,
    /// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
    #[serde(rename = "renderMode")]
    pub render_mode: RenderMode,
    /// If TRUE, the entity instances will be resizable horizontally
    #[serde(rename = "resizableX")]
    pub resizable_x: bool,
    /// If TRUE, the entity instances will be resizable vertically
    #[serde(rename = "resizableY")]
    pub resizable_y: bool,
    /// Display entity name in editor
    #[serde(rename = "showName")]
    pub show_name: bool,
    /// An array of strings that classifies this entity
    pub tags: Vec<String>,
    /// Tile ID used for optional tile display
    #[serde(rename = "tileId")]
    pub tile_id: Option<i64>,
    /// Possible values: `Cover`, `FitInside`, `Repeat`, `Stretch`
    #[serde(rename = "tileRenderMode")]
    pub tile_render_mode: TileRenderMode,
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
    #[serde(rename = "editorCutLongValues")]
    pub editor_cut_long_values: bool,
    /// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `Points`,
    /// `PointStar`, `PointPath`, `PointPathLoop`, `RadiusPx`, `RadiusGrid`
    #[serde(rename = "editorDisplayMode")]
    pub editor_display_mode: EditorDisplayMode,
    /// Possible values: `Above`, `Center`, `Beneath`
    #[serde(rename = "editorDisplayPos")]
    pub editor_display_pos: EditorDisplayPos,
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
    /// Possible values: &lt;`null`&gt;, `LangPython`, `LangRuby`, `LangJS`, `LangLua`, `LangC`,
    /// `LangHaxe`, `LangMarkdown`, `LangJson`, `LangXml`
    #[serde(rename = "textLangageMode")]
    pub text_langage_mode: Option<TextLangageMode>,
    /// Internal type enum
    #[serde(rename = "type")]
    pub purple_type: Option<serde_json::Value>,
    /// Unique Int identifier
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
    pub values: Vec<EnumValueDefinition>,
}

#[derive(Serialize, Deserialize)]
pub struct EnumValueDefinition {
    /// An array of 4 Int values that refers to the tile in the tileset image: `[ x, y, width,
    /// height ]`
    #[serde(rename = "__tileSrcRect")]
    pub tile_src_rect: Option<Vec<i64>>,
    /// Optional color
    pub color: i64,
    /// Enum value
    pub id: String,
    /// The optional ID of the tile
    #[serde(rename = "tileId")]
    pub tile_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct LayerDefinition {
    /// Type of the layer (*IntGrid, Entities, Tiles or AutoLayer*)
    #[serde(rename = "__type")]
    pub layer_definition_type: String,
    /// Contains all the auto-layer rule definitions.
    #[serde(rename = "autoRuleGroups")]
    pub auto_rule_groups: Vec<AutoLayerRuleGroup>,
    #[serde(rename = "autoSourceLayerDefUid")]
    pub auto_source_layer_def_uid: Option<i64>,
    /// Reference to the Tileset UID being used by this auto-layer rules. WARNING: some layer
    /// *instances* might use a different tileset. So most of the time, you should probably use
    /// the `__tilesetDefUid` value from layer instances.
    #[serde(rename = "autoTilesetDefUid")]
    pub auto_tileset_def_uid: Option<i64>,
    /// Opacity of the layer (0 to 1.0)
    #[serde(rename = "displayOpacity")]
    pub display_opacity: f64,
    /// An array of tags to forbid some Entities in this layer
    #[serde(rename = "excludedTags")]
    pub excluded_tags: Vec<String>,
    /// Width and height of the grid in pixels
    #[serde(rename = "gridSize")]
    pub grid_size: i64,
    /// Unique String identifier
    pub identifier: String,
    /// An array that defines extra optional info for each IntGrid value. The array is sorted
    /// using value (ascending).
    #[serde(rename = "intGridValues")]
    pub int_grid_values: Vec<IntGridValueDefinition>,
    /// X offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: i64,
    /// Y offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: i64,
    /// An array of tags to filter Entities that can be added to this layer
    #[serde(rename = "requiredTags")]
    pub required_tags: Vec<String>,
    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    #[serde(rename = "tilePivotX")]
    pub tile_pivot_x: f64,
    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    #[serde(rename = "tilePivotY")]
    pub tile_pivot_y: f64,
    /// Reference to the Tileset UID being used by this Tile layer. WARNING: some layer
    /// *instances* might use a different tileset. So most of the time, you should probably use
    /// the `__tilesetDefUid` value from layer instances.
    #[serde(rename = "tilesetDefUid")]
    pub tileset_def_uid: Option<i64>,
    /// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
    /// `AutoLayer`
    #[serde(rename = "type")]
    pub purple_type: Type,
    /// Unique Int identifier
    pub uid: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AutoLayerRuleGroup {
    pub active: bool,
    pub collapsed: bool,
    #[serde(rename = "isOptional")]
    pub is_optional: bool,
    pub name: String,
    pub rules: Vec<AutoLayerRuleDefinition>,
    pub uid: i64,
}

/// This complex section isn't meant to be used by game devs at all, as these rules are
/// completely resolved internally by the editor before any saving. You should just ignore
/// this part.
#[derive(Serialize, Deserialize)]
pub struct AutoLayerRuleDefinition {
    /// If FALSE, the rule effect isn't applied, and no tiles are generated.
    pub active: bool,
    /// When TRUE, the rule will prevent other rules to be applied in the same cell if it matches
    /// (TRUE by default).
    #[serde(rename = "breakOnMatch")]
    pub break_on_match: bool,
    /// Chances for this rule to be applied (0 to 1)
    pub chance: f64,
    /// Checker mode Possible values: `None`, `Horizontal`, `Vertical`
    pub checker: Checker,
    /// If TRUE, allow rule to be matched by flipping its pattern horizontally
    #[serde(rename = "flipX")]
    pub flip_x: bool,
    /// If TRUE, allow rule to be matched by flipping its pattern vertically
    #[serde(rename = "flipY")]
    pub flip_y: bool,
    /// Default IntGrid value when checking cells outside of level bounds
    #[serde(rename = "outOfBoundsValue")]
    pub out_of_bounds_value: Option<i64>,
    /// Rule pattern (size x size)
    pub pattern: Vec<i64>,
    /// If TRUE, enable Perlin filtering to only apply rule on specific random area
    #[serde(rename = "perlinActive")]
    pub perlin_active: bool,
    #[serde(rename = "perlinOctaves")]
    pub perlin_octaves: f64,
    #[serde(rename = "perlinScale")]
    pub perlin_scale: f64,
    #[serde(rename = "perlinSeed")]
    pub perlin_seed: f64,
    /// X pivot of a tile stamp (0-1)
    #[serde(rename = "pivotX")]
    pub pivot_x: f64,
    /// Y pivot of a tile stamp (0-1)
    #[serde(rename = "pivotY")]
    pub pivot_y: f64,
    /// Pattern width & height. Should only be 1,3,5 or 7.
    pub size: i64,
    /// Array of all the tile IDs. They are used randomly or as stamps, based on `tileMode` value.
    #[serde(rename = "tileIds")]
    pub tile_ids: Vec<i64>,
    /// Defines how tileIds array is used Possible values: `Single`, `Stamp`
    #[serde(rename = "tileMode")]
    pub tile_mode: TileMode,
    /// Unique Int identifier
    pub uid: i64,
    /// X cell coord modulo
    #[serde(rename = "xModulo")]
    pub x_modulo: i64,
    /// Y cell coord modulo
    #[serde(rename = "yModulo")]
    pub y_modulo: i64,
}

/// IntGrid value definition
#[derive(Serialize, Deserialize)]
pub struct IntGridValueDefinition {
    pub color: String,
    /// Unique String identifier
    pub identifier: Option<String>,
    /// The IntGrid value itself
    pub value: i64,
}

/// The `Tileset` definition is the most important part among project definitions. It
/// contains some extra informations about each integrated tileset. If you only had to parse
/// one definition section, that would be the one.
#[derive(Serialize, Deserialize)]
pub struct TilesetDefinition {
    /// Grid-based height
    #[serde(rename = "__cHei")]
    pub c_hei: i64,
    /// Grid-based width
    #[serde(rename = "__cWid")]
    pub c_wid: i64,
    /// The following data is used internally for various optimizations. It's always synced with
    /// source image changes.
    #[serde(rename = "cachedPixelData")]
    pub cached_pixel_data: Option<HashMap<String, Option<serde_json::Value>>>,
    /// An array of custom tile metadata
    #[serde(rename = "customData")]
    pub custom_data: Vec<HashMap<String, Option<serde_json::Value>>>,
    /// Tileset tags using Enum values specified by `tagsSourceEnumId`. This array contains 1
    /// element per Enum value, which contains an array of all Tile IDs that are tagged with it.
    #[serde(rename = "enumTags")]
    pub enum_tags: Vec<HashMap<String, Option<serde_json::Value>>>,
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
    /// Optional Enum definition UID used for this tileset meta-data
    #[serde(rename = "tagsSourceEnumUid")]
    pub tags_source_enum_uid: Option<i64>,
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
    /// Position informations of the background image, if there is one.
    #[serde(rename = "__bgPos")]
    pub bg_pos: Option<LevelBackgroundPosition>,
    /// An array listing all other levels touching this one on the world map. In "linear" world
    /// layouts, this array is populated with previous/next levels in array, and `dir` depends on
    /// the linear horizontal/vertical layout.
    #[serde(rename = "__neighbours")]
    pub neighbours: Vec<NeighbourLevel>,
    /// Background color of the level. If `null`, the project `defaultLevelBgColor` should be
    /// used.
    #[serde(rename = "bgColor")]
    pub level_bg_color: Option<String>,
    /// Background image X pivot (0-1)
    #[serde(rename = "bgPivotX")]
    pub bg_pivot_x: f64,
    /// Background image Y pivot (0-1)
    #[serde(rename = "bgPivotY")]
    pub bg_pivot_y: f64,
    /// An enum defining the way the background image (if any) is positioned on the level. See
    /// `__bgPos` for resulting position info. Possible values: &lt;`null`&gt;, `Unscaled`,
    /// `Contain`, `Cover`, `CoverDirty`
    #[serde(rename = "bgPos")]
    pub level_bg_pos: Option<BgPos>,
    /// The *optional* relative path to the level background image.
    #[serde(rename = "bgRelPath")]
    pub bg_rel_path: Option<String>,
    /// This value is not null if the project option "*Save levels separately*" is enabled. In
    /// this case, this **relative** path points to the level Json file.
    #[serde(rename = "externalRelPath")]
    pub external_rel_path: Option<String>,
    /// An array containing this level custom field values.
    #[serde(rename = "fieldInstances")]
    pub field_instances: Vec<FieldInstance>,
    /// Unique String identifier
    pub identifier: String,
    /// An array containing all Layer instances. **IMPORTANT**: if the project option "*Save
    /// levels separately*" is enabled, this field will be `null`.<br/>  This array is **sorted
    /// in display order**: the 1st layer is the top-most and the last is behind.
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
    /// If TRUE, the level identifier will always automatically use the naming pattern as defined
    /// in `Project.levelNamePattern`. Becomes FALSE if the identifier is manually modified by
    /// user.
    #[serde(rename = "useAutoIdentifier")]
    pub use_auto_identifier: bool,
    /// World X coordinate in pixels
    #[serde(rename = "worldX")]
    pub world_x: i64,
    /// World Y coordinate in pixels
    #[serde(rename = "worldY")]
    pub world_y: i64,
}

/// Level background image position info
#[derive(Serialize, Deserialize)]
pub struct LevelBackgroundPosition {
    /// An array of 4 float values describing the cropped sub-rectangle of the displayed
    /// background image. This cropping happens when original is larger than the level bounds.
    /// Array format: `[ cropX, cropY, cropWidth, cropHeight ]`
    #[serde(rename = "cropRect")]
    pub crop_rect: Vec<f64>,
    /// An array containing the `[scaleX,scaleY]` values of the **cropped** background image,
    /// depending on `bgPos` option.
    pub scale: Vec<f64>,
    /// An array containing the `[x,y]` pixel coordinates of the top-left corner of the
    /// **cropped** background image, depending on `bgPos` option.
    #[serde(rename = "topLeftPx")]
    pub top_left_px: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldInstance {
    /// Field definition identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,
    /// Type of the field, such as `Int`, `Float`, `Enum(my_enum_name)`, `Bool`, etc.
    #[serde(rename = "__type")]
    pub field_instance_type: String,
    /// Actual value of the field instance. The value type may vary, depending on `__type`
    /// (Integer, Boolean, String etc.)<br/>  It can also be an `Array` of those same types.
    #[serde(rename = "__value")]
    pub value: Option<serde_json::Value>,
    /// Reference of the **Field definition** UID
    #[serde(rename = "defUid")]
    pub def_uid: i64,
    /// Editor internal raw values
    #[serde(rename = "realEditorValues")]
    pub real_editor_values: Vec<Option<serde_json::Value>>,
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
    /// Layer definition identifier
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
    /// **WARNING**: this deprecated value will be *removed* completely on version 0.9.1+
    /// Replaced by: `intGridCsv`
    #[serde(rename = "intGrid")]
    pub int_grid: Option<Vec<IntGridValueInstance>>,
    /// A list of all values in the IntGrid layer, stored from left to right, and top to bottom
    /// (ie. first row from left to right, followed by second row, etc). `0` means "empty cell"
    /// and IntGrid values start at 1. This array size is `__cWid` x `__cHei` cells.
    #[serde(rename = "intGridCsv")]
    pub int_grid_csv: Vec<i64>,
    /// Reference the Layer definition UID
    #[serde(rename = "layerDefUid")]
    pub layer_def_uid: i64,
    /// Reference to the UID of the level containing this layer instance
    #[serde(rename = "levelId")]
    pub level_id: i64,
    /// An Array containing the UIDs of optional rules that were enabled in this specific layer
    /// instance.
    #[serde(rename = "optionalRules")]
    pub optional_rules: Vec<i64>,
    /// This layer can use another tileset by overriding the tileset UID here.
    #[serde(rename = "overrideTilesetUid")]
    pub override_tileset_uid: Option<i64>,
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
    /// Layer instance visibility
    pub visible: bool,
}

/// This structure represents a single tile from a given Tileset.
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
    /// Entity definition identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,
    /// Pivot coordinates  (`[x,y]` format, values are from 0 to 1) of the Entity
    #[serde(rename = "__pivot")]
    pub pivot: Vec<f64>,
    /// Optional Tile used to display this entity (it could either be the default Entity tile, or
    /// some tile provided by a field value, like an Enum).
    #[serde(rename = "__tile")]
    pub tile: Option<EntityInstanceTile>,
    /// Reference of the **Entity definition** UID
    #[serde(rename = "defUid")]
    pub def_uid: i64,
    /// An array of all custom fields and their values.
    #[serde(rename = "fieldInstances")]
    pub field_instances: Vec<FieldInstance>,
    /// Entity height in pixels. For non-resizable entities, it will be the same as Entity
    /// definition.
    pub height: i64,
    /// Pixel coordinates (`[x,y]` format) in current level coordinate space. Don't forget
    /// optional layer offsets, if they exist!
    pub px: Vec<i64>,
    /// Entity width in pixels. For non-resizable entities, it will be the same as Entity
    /// definition.
    pub width: i64,
}

/// Tile data in an Entity instance
#[derive(Serialize, Deserialize)]
pub struct EntityInstanceTile {
    /// An array of 4 Int values that refers to the tile in the tileset image: `[ x, y, width,
    /// height ]`
    #[serde(rename = "srcRect")]
    pub src_rect: Vec<i64>,
    /// Tileset ID
    #[serde(rename = "tilesetUid")]
    pub tileset_uid: i64,
}

/// IntGrid value instance
#[derive(Serialize, Deserialize)]
pub struct IntGridValueInstance {
    /// Coordinate ID in the layer grid
    #[serde(rename = "coordId")]
    pub coord_id: i64,
    /// IntGrid value
    pub v: i64,
}

/// Nearby level info
#[derive(Serialize, Deserialize)]
pub struct NeighbourLevel {
    /// A single lowercase character tipping on the level location (`n`orth, `s`outh, `w`est,
    /// `e`ast).
    pub dir: String,
    #[serde(rename = "levelUid")]
    pub level_uid: i64,
}

/// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `Points`,
/// `PointStar`, `PointPath`, `PointPathLoop`, `RadiusPx`, `RadiusGrid`
#[derive(Serialize, Deserialize)]
pub enum EditorDisplayMode {
    EntityTile,
    Hidden,
    NameAndValue,
    PointPath,
    PointPathLoop,
    PointStar,
    Points,
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

#[derive(Serialize, Deserialize)]
pub enum TextLangageMode {
    LangC,
    LangHaxe,
    #[serde(rename = "LangJS")]
    LangJs,
    LangJson,
    LangLua,
    LangMarkdown,
    LangPython,
    LangRuby,
    LangXml,
}

/// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
#[derive(Serialize, Deserialize)]
pub enum LimitBehavior {
    DiscardOldOnes,
    MoveLastOne,
    PreventAdding,
}

/// If TRUE, the maxCount is a "per world" limit, if FALSE, it's a "per level". Possible
/// values: `PerLayer`, `PerLevel`, `PerWorld`
#[derive(Serialize, Deserialize)]
pub enum LimitScope {
    PerLayer,
    PerLevel,
    PerWorld,
}

/// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
#[derive(Serialize, Deserialize)]
pub enum RenderMode {
    Cross,
    Ellipse,
    Rectangle,
    Tile,
}

/// Possible values: `Cover`, `FitInside`, `Repeat`, `Stretch`
#[derive(Serialize, Deserialize)]
pub enum TileRenderMode {
    Cover,
    FitInside,
    Repeat,
    Stretch,
}

/// Checker mode Possible values: `None`, `Horizontal`, `Vertical`
#[derive(Serialize, Deserialize)]
pub enum Checker {
    Horizontal,
    None,
    Vertical,
}

/// Defines how tileIds array is used Possible values: `Single`, `Stamp`
#[derive(Serialize, Deserialize)]
pub enum TileMode {
    Single,
    Stamp,
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

#[derive(Serialize, Deserialize)]
pub enum Flag {
    DiscardPreCsvIntGrid,
    IgnoreBackupSuggest,
}

#[derive(Serialize, Deserialize)]
pub enum BgPos {
    Contain,
    Cover,
    CoverDirty,
    Unscaled,
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
