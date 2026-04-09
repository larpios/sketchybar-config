use anyhow::Result;
use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text, ImageType, ImageProps};
use crate::themes::CATPUCCIN_MOCHA;
use std::env;
use std::process::Command;

pub fn update() -> Result<()> {
    let sender = env::var("SENDER").unwrap_or_else(|_| "".to_string());

    match sender.as_str() {
        "mouse.scrolled.up" => {
            let _ = Command::new("osascript")
                .args(["-e", "tell application \"Music\" to next track", "-e", "tell application \"Spotify\" to next track"])
                .status();
        }
        "mouse.scrolled.down" => {
            let _ = Command::new("osascript")
                .args(["-e", "tell application \"Music\" to previous track", "-e", "tell application \"Spotify\" to previous track"])
                .status();
        }
        "mouse.clicked" => {
            let _ = Command::new("osascript")
                .args(["-e", "tell application \"Music\" to playpause", "-e", "tell application \"Spotify\" to playpause"])
                .status();
        }
        _ => {}
    }

    let script = r#"
        tell application "System Events"
            set isMusicRunning to (count of (every process whose name is "Music")) > 0
            set isSpotifyRunning to (count of (every process whose name is "Spotify")) > 0
        end tell

        if isMusicRunning then
            tell application "Music"
                if player state is playing then
                    return (get artist of current track) & " - " & (get name of current track)
                end if
            end tell
        end if

        if isSpotifyRunning then
            tell application "Spotify"
                if player state is playing then
                    return (get artist of current track) & " - " & (get name of current track)
                end if
            end tell
        end if

        return ""
    "#;

    let output = Command::new("osascript")
        .args(["-e", script])
        .output()?;

    let media_text = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if media_text.is_empty() {
        api::set_args("media", &["drawing=off"])?;
    } else {
        // Truncate if too long
        let mut display_text = media_text;
        if display_text.len() > 30 {
            display_text.truncate(27);
            display_text.push_str("...");
        }

        api::set_args("media", &[
            "drawing=on",
            &format!("label={}", display_text)
        ])?;
    }

    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    let mut item = BarItem::new("media".to_string(), ComponentPosition::Center);
    item.scripting.update_freq = 5;
    item.scripting.script = Some(ScriptType::String(format!("{} --update-media", exe_path)));
    
    item.icon.icon = Some("󰎆".to_string());
    
    let mut icon_props = Text::default();
    icon_props.color = Some(CATPUCCIN_MOCHA.green.clone());
    item.icon.props = Some(icon_props);

    let mut bg = BackgroundProps::new();
    bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
    bg.drawing = Some(true);
    let mut image_props = ImageProps::new(ImageType::MediaArtwork);
    image_props.drawing = true;
    bg.image = Some(image_props);
    item.geometry.background = Some(bg);

    // Give some padding to accommodate the thumbnail
    item.geometry.padding_left = Some(10);
    item.geometry.padding_right = Some(10);

    api::add_item(&item)?;
    api::subscribe("media", "mouse.scrolled.up")?;
    api::subscribe("media", "mouse.scrolled.down")?;
    api::subscribe("media", "mouse.clicked")?;
    
    Ok(())
}
