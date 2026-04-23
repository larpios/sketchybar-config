use std::{env, path::PathBuf};

pub fn data_dir() -> PathBuf {
    if let Ok(data_home) = env::var("XDG_DATA_HOME") {
        format!("{}/sketchybar", data_home).into()
    } else if let Some(home) = env::home_dir() {
        format!("{}/.local/share/sketchybar", home.display()).into()
    } else {
        panic!("Could not determine data directory");
    }
}
