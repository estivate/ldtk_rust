use ldtk_rust::{Level, Project};

const BASE_DIR: &str = "assets/";
const PROJECT_FILE: &str = "SeparateLevelFiles.ldtk";

fn main() {
    let current_level_uid: i64 = 0;
    let mut current_level_path: Option<String> = None;
    let mut current_level_data: Option<Level> = None;

    // load the main project file
    let project = Project::load_project(BASE_DIR.to_owned() + PROJECT_FILE);

    // loop through all the levels and get the relative path to
    // the level that matches the current_level_id
    for level in project.levels.iter() {
        if level.uid == current_level_uid {
            current_level_path = level.external_rel_path.clone();
            break;
        }
    }

    // try to find a file for the level id.
    match current_level_path {
        Some(t) => current_level_data = Some(Level::new(BASE_DIR.to_owned() + &t)),
        None => println!("that level ID is not correct."),
    };

    // print some data
    match current_level_data {
        Some(t) => println!(
            "Level {} has {} layer instaces.",
            current_level_uid,
            t.layer_instances.unwrap().len()
        ),
        None => println!("no level data"),
    }
}
