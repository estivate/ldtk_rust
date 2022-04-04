use ldtk_rust::Project;
// Loads an LDtk Project file along with any external level files
// that it references.
fn main() {
    let file_path = "assets/game_1-1-0.ldtk".to_string();
    let project: Project = Project::new(file_path);

    //println!("First level pixel height is {}!", project.levels[0].px_hei);
    println!("First level pixel height is {}!", project.levels[0].px_hei);
}
