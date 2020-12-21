extern crate ldtk_rust;
fn main() {
    let file_path = "assets/AutoLayers_1_basic.ldtk".to_string();
    let ldtk = ldtk_rust::new_from_file(file_path);
    println!("First level pixel height is {}!", ldtk.levels[0].px_hei);
}
