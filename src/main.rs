use anyhow::{Ok, Result};
use sketchybarrc::api;
use sketchybarrc::api::bar::Bar;
use sketchybarrc::api::bar::BarPosition;
use sketchybarrc::themes::CATPUCCIN_MOCHA;
use std::env;

use sketchybarrc::items;
use sketchybarrc::watcher;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Handle watcher command
        if args[1] == "--watcher" {
            return watcher::watch_media();
        }

        // Handle all other commands through registry
        if items::ItemRegistry::execute(&args[1], &args[2..])
            .await?
            .is_some()
        {
            return Ok(());
        }
    }

    // Initialize bar
    let _ = std::process::Command::new("sketchybar")
        .args(["--bar", "hidden=off"])
        .status();
    let _ = std::process::Command::new("sketchybar")
        .args(["--remove", "/.*/"])
        .status();

    let bar = Bar {
        color: CATPUCCIN_MOCHA.base,
        position: BarPosition::Top,
        height: 28,
        margin: 8,
        y_offset: 4,
        corner_radius: 21,
        border_width: 2,
        border_color: CATPUCCIN_MOCHA.surface1,
        padding_left: 8,
        padding_right: 8,
        notch_width: 200,
        ..Default::default()
    };

    api::add_bar(&bar)?;

    // Set defaults from nushell config
    api::set_default([
        "icon.font=JetBrainsMono Nerd Font:Regular:14.0",
        &format!("icon.color={}", CATPUCCIN_MOCHA.text),
        "label.font=JetBrainsMono Nerd Font:Regular:12.0",
        &format!("label.color={}", CATPUCCIN_MOCHA.text),
        "padding_left=4",
        "padding_right=4",
        "icon.padding_left=6",
        "icon.padding_right=6",
        "label.padding_left=6",
        "label.padding_right=6",
        &format!("background.color={}", CATPUCCIN_MOCHA.surface0),
        "background.corner_radius=9",
        "background.height=20",
        &format!("icon.highlight_color={}", CATPUCCIN_MOCHA.mauve),
        &format!("label.highlight_color={}", CATPUCCIN_MOCHA.mauve),
    ])?;

    let exe_path = env::current_exe()?.to_string_lossy().to_string();

    // Start media watcher in background
    let _ = std::process::Command::new(&exe_path)
        .arg("--watcher")
        .spawn();

    // Setup all items
    items::ItemRegistry::setup_all(&exe_path).await?;

    api::update()?;
    api::trigger_evt("workspace_change")?;

    Ok(())
}
