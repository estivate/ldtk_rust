mod ldtk_file;
use ldtk_file::*;
use std::fs::File;
use std::path::Path;

/// Project is the top-level struct used to house all content
/// read in from an LDTK .json or .ldtk file.
#[derive(Debug)]
pub struct Project {
    pub file: String,
    pub defs: Defs,
    pub levels: Vec<Level>,
    pub redraw: bool,
}
impl Project {
    /// Creates a Project object from a LDTK file
    ///
    /// # Example
    ///
    /// ```
    /// extern crate ldtk;
    ///
    /// fn main() {
    ///   let file_path = "assets/AutoLayers_1_basic.ldtk".to_string();
    ///   let ldtk = ldtk::Project::new_from_file(file_path);
    ///   println!("First level pixel height is {}!", ldtk.levels[0].px_hei);
    /// }
    /// ```
    ///
    ///
    pub fn new_from_file(f: String) -> Self {
        let json_file_path = Path::new(&f);
        let file = File::open(json_file_path).expect("file not found");
        let o: LdtkFile = serde_json::from_reader(file).expect("error while reading");

        Project {
            file: f,
            defs: o.defs,
            levels: o.levels,
            redraw: true,
        }
    }

    pub fn debug(&self, db_level: i32) {
        match db_level {
            0 => println!("{:?}", self),
            1 => println!("{:?}", self.defs),
            2 => println!("{:?}", self.levels),
            _ => println!("{:?}", self),
        }
    }
}
