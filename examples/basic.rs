use ldtk_rust::Project;
// Loads an LDtk Project file along with any external level files
// that it references.
fn main() {
    let file_path = "assets/SeparateLevelFiles.ldtk".to_string();
    let ldtk = Project::new(file_path);
    println!("First level pixel height is {}!", ldtk.levels[0].px_hei);
}
