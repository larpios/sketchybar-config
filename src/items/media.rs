use anyhow::Result;
use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text, ImageType, ImageProps};
use crate::themes::CATPUCCIN_MOCHA;
use std::env;
use std::process::Command;

pub fn update() -> Result<()> {
    let name = env::var("NAME").unwrap_or_else(|_| "".to_string());
    let sender = env::var("SENDER").unwrap_or_else(|_| "".to_string());

    if sender == "mouse.clicked" {
        match name.as_str() {
            "media.prev" => {
                let _ = Command::new("osascript")
                    .args(["-e", "tell application \"Music\" to previous track", "-e", "tell application \"Spotify\" to previous track"])
                    .status();
            }
            "media.next" => {
                let _ = Command::new("osascript")
                    .args(["-e", "tell application \"Music\" to next track", "-e", "tell application \"Spotify\" to next track"])
                    .status();
            }
            "media.play" => {
                let _ = Command::new("osascript")
                    .args(["-e", "tell application \"Music\" to playpause", "-e", "tell application \"Spotify\" to playpause"])
                    .status();
            }
            _ => {}
        }
    }

    let script = r#"
        tell application "System Events"
            set isMusicRunning to (count of (every process whose name is "Music")) > 0
            set isSpotifyRunning to (count of (every process whose name is "Spotify")) > 0
        end tell

        set playerState to "stopped"
        set trackInfo to ""

        if isMusicRunning then
            tell application "Music"
                try
                    set playerState to player state as string
                    set trackInfo to (get artist of current track) & " - " & (get name of current track)
                on error
                    set playerState to "stopped"
                end try
            end tell
        else if isSpotifyRunning then
            tell application "Spotify"
                try
                    set playerState to player state as string
                    set trackInfo to (get artist of current track) & " - " & (get name of current track)
                on error
                    set playerState to "stopped"
                end try
            end tell
        end if

        return playerState & "|" & trackInfo
    "#;

    let output = Command::new("osascript")
        .args(["-e", script])
        .output()?;

    let response = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let parts: Vec<&str> = response.split('|').collect();
    
    let player_state = parts.get(0).unwrap_or(&"stopped").to_lowercase();
    let track_info = parts.get(1).unwrap_or(&"");

    if player_state == "stopped" || track_info.is_empty() {
        api::set_args("media", &[
            "drawing=on",
            "icon=󰎆",
            "label=\"\"",
            "background.image.drawing=off",
        ])?;
    } else {
        // Update main label
        let mut display_text = track_info.to_string();
        if display_text.len() > 30 {
            display_text.truncate(27);
            display_text.push_str("...");
        }

        api::set_args("media", &[
            "drawing=on",
            "icon=󰎆",
            "background.image.drawing=on",
            &format!("label={}", display_text)
        ])?;

        // Update play/pause icon
        let play_icon = if player_state == "playing" { "󰏤" } else { "󰐎" };
        api::set_args("media.play", &[&format!("icon={}", play_icon)])?;
    }

    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    let mut item = BarItem::new("media".to_string(), ComponentPosition::Center);
    item.scripting.update_freq = 5;
    item.scripting.script = Some(ScriptType::String(format!("{} --update-media", exe_path)));
    item.scripting.click_script = Some(ScriptType::String("sketchybar --set media popup.drawing=toggle".to_string()));
    
    item.icon.icon = Some("󰎆".to_string());
    
    let mut icon_props = Text::default();
    icon_props.color = Some(CATPUCCIN_MOCHA.green.clone());
    item.icon.props = Some(icon_props);

    let mut image_props = ImageProps::new(ImageType::MediaArtwork);
    image_props.drawing = false;
    image_props.scale = 0.15;
    image_props.corner_radius = 4;
    image_props.padding_right = 5;
    
    let mut bg = BackgroundProps::new();
    bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
    bg.drawing = Some(true);
    bg.image = Some(image_props);
    item.geometry.background = Some(bg);

    // Give some padding to accommodate the thumbnail
    item.geometry.padding_left = Some(10);
    item.geometry.padding_right = Some(10);

    // Popup
    let mut popup_props = crate::props::item::PopupProperties::default();
    popup_props.align = crate::props::item::PopupAlign::Center;
    let mut popup_bg = BackgroundProps::new();
    popup_bg.color = Some(CATPUCCIN_MOCHA.base.clone());
    popup_bg.corner_radius = Some(8);
    popup_bg.border_width = Some(2);
    popup_bg.border_color = Some(CATPUCCIN_MOCHA.surface1.clone());
    popup_props.background = Some(popup_bg);
    item.popup = Some(popup_props);

    api::add_item(&item)?;
    
    // Previous button
    Command::new("sketchybar")
        .args([
            "--add", "item", "media.prev", "popup.media",
            "--set", "media.prev",
            "icon=󰒮",
            "icon.font=JetBrainsMono Nerd Font:Regular:16.0",
            "icon.padding_left=10",
            "icon.padding_right=10",
            &format!("click_script={} --update-media", exe_path),
        ])
        .status()?;

    // Play/Pause button
    Command::new("sketchybar")
        .args([
            "--add", "item", "media.play", "popup.media",
            "--set", "media.play",
            "icon=󰐎",
            "icon.font=JetBrainsMono Nerd Font:Regular:18.0",
            "icon.padding_left=10",
            "icon.padding_right=10",
            &format!("click_script={} --update-media", exe_path),
        ])
        .status()?;

    // Next button
    Command::new("sketchybar")
        .args([
            "--add", "item", "media.next", "popup.media",
            "--set", "media.next",
            "icon=󰒭",
            "icon.font=JetBrainsMono Nerd Font:Regular:16.0",
            "icon.padding_left=10",
            "icon.padding_right=10",
            &format!("click_script={} --update-media", exe_path),
        ])
        .status()?;

    Ok(())
}
