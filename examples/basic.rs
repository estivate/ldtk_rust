use ldtk_rust::LdtkFile;

fn main() {
    let file_path = "assets/test_game.ldtk".to_string();
    let ldtk = LdtkFile::new(file_path);
    println!("First level pixel height is {}!", ldtk.levels[0].px_hei);
}
