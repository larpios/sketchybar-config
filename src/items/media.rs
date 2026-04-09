use anyhow::Result;
use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text, ImageType, ImageProps};
use crate::themes::CATPUCCIN_MOCHA;
use std::env;
use std::process::Command;
use media_remote::{NowPlaying, NowPlayingPerl, Controller};

pub fn update() -> Result<()> {
    let name = env::var("NAME").unwrap_or_else(|_| "".to_string());
    let sender = env::var("SENDER").unwrap_or_else(|_| "".to_string());

    if sender == "mouse.clicked" {
        let now_playing = NowPlaying::new();
        match name.as_str() {
            "media.prev" => {
                now_playing.previous();
            }
            "media.next" => {
                now_playing.next();
            }
            "media.play" => {
                now_playing.toggle();
            }
            _ => {}
        }
        // Small delay to let system update
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let now_playing = NowPlaying::new();
    let info_guard = now_playing.get_info();
    
    if let Some(info) = &*info_guard {
        return update_with_info(info);
    }

    // Fallback to Perl adapter if needed
    let perl = NowPlayingPerl::new();
    let perl_guard = perl.get_info();
    if let Some(info) = &*perl_guard {
        return update_with_info(info);
    }

    // If both fail or no info
    api::set_args("media", &["drawing=off"])?;
    Ok(())
}

fn update_with_info(info: &media_remote::NowPlayingInfo) -> Result<()> {
    let title = info.title.as_deref().unwrap_or_default();
    let artist = info.artist.as_deref().unwrap_or_default();
    let is_playing = info.is_playing.unwrap_or(false);

    if title.is_empty() {
         api::set_args("media", &[
            "drawing=off",
        ])?;
        return Ok(());
    }

    let track_info = if artist.is_empty() {
        title.to_string()
    } else {
        format!("{} - {}", artist, title)
    };

    // Handle Artwork
    let mut has_artwork = false;
    if let Some(image) = &info.album_cover {
         if image.save("/tmp/sketchybar_artwork.png").is_ok() {
             has_artwork = true;
         }
    }

    // Update main label (bar item)
    let mut display_text = track_info.to_string();
    if display_text.len() > 25 {
        display_text.truncate(22);
        display_text.push_str("...");
    }

    let mut args = vec![
        "drawing=on".to_string(),
        format!("label={}", display_text),
    ];

    if has_artwork {
        args.push("background.image=/tmp/sketchybar_artwork.png".to_string());
        args.push("background.image.drawing=on".to_string());
    } else {
        args.push("background.image.drawing=off".to_string());
    }

    api::set_args("media", &args.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    // Update popup items
    if has_artwork {
        api::set_args("media.cover", &["background.image=/tmp/sketchybar_artwork.png", "drawing=on"])?;
    } else {
        api::set_args("media.cover", &["drawing=off"])?;
    }
    
    api::set_args("media.title", &[&format!("label={}", title)])?;
    api::set_args("media.artist", &[&format!("label={}", artist)])?;
    
    let play_icon = if is_playing { "󰏤" } else { "󰐎" };
    api::set_args("media.play", &[&format!("icon={}", play_icon)])?;

    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    let mut item = BarItem::new("media".to_string(), ComponentPosition::Left);
    item.scripting.update_freq = 2;
    item.scripting.script = Some(ScriptType::String(format!("{} --update-media", exe_path)));
    item.scripting.click_script = Some(ScriptType::String("sketchybar --set media popup.drawing=toggle".to_string()));
    
    item.icon.icon = Some("󰎆".to_string());
    let mut icon_props = Text::default();
    icon_props.color = Some(CATPUCCIN_MOCHA.green.clone());
    item.icon.props = Some(icon_props);

    let mut image_props = ImageProps::new(ImageType::Path("/tmp/sketchybar_artwork.png".to_string()));
    image_props.drawing = false;
    image_props.scale = 0.15;
    image_props.corner_radius = 4;
    image_props.padding_right = 5;
    
    let mut bg = BackgroundProps::new();
    bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
    bg.drawing = Some(true);
    bg.image = Some(image_props);
    item.geometry.background = Some(bg);

    item.geometry.padding_left = Some(10);
    item.geometry.padding_right = Some(10);

    // Popup setup
    let mut popup_props = crate::props::item::PopupProperties::default();
    popup_props.align = crate::props::item::PopupAlign::Center;
    let mut popup_bg = BackgroundProps::new();
    popup_bg.color = Some(CATPUCCIN_MOCHA.base.clone());
    popup_bg.corner_radius = Some(12);
    popup_bg.border_width = Some(2);
    popup_bg.border_color = Some(CATPUCCIN_MOCHA.surface1.clone());
    popup_props.background = Some(popup_bg);
    item.popup = Some(popup_props);

    api::add_item(&item)?;

    // 1. Cover
    Command::new("sketchybar")
        .args([
            "--add", "item", "media.cover", "popup.media",
            "--set", "media.cover",
            "background.image.scale=0.6",
            "background.image.corner_radius=12",
            "background.image.drawing=on",
            "background.image.blur_radius=30",
            "width=240",
            "height=160",
            "align=center",
        ])
        .status()?;

    // 2. Title and Artist
    Command::new("sketchybar")
        .args([
            "--add", "item", "media.title", "popup.media",
            "--set", "media.title",
            "label.font=JetBrainsMono Nerd Font:Bold:16.0",
            "width=240",
            "align=center",
            "label.padding_left=0",
            "label.padding_right=0",
        ])
        .status()?;

    Command::new("sketchybar")
        .args([
            "--add", "item", "media.artist", "popup.media",
            "--set", "media.artist",
            "label.font=JetBrainsMono Nerd Font:Regular:13.0",
            "label.color=0xffbac2de",
            "width=240",
            "align=center",
            "label.padding_left=0",
            "label.padding_right=0",
        ])
        .status()?;

    // 3. Horizontal Controls
    Command::new("sketchybar")
        .args([
            "--add", "item", "media.prev", "popup.media",
            "--set", "media.prev",
            "icon=󰒮",
            "icon.font=JetBrainsMono Nerd Font:Regular:20.0",
            "width=60",
            "align=center",
            &format!("click_script={} --update-media", exe_path),
        ])
        .status()?;

    Command::new("sketchybar")
        .args([
            "--add", "item", "media.play", "popup.media",
            "--set", "media.play",
            "icon=󰐎",
            "icon.font=JetBrainsMono Nerd Font:Regular:24.0",
            "width=60",
            "align=center",
            &format!("click_script={} --update-media", exe_path),
        ])
        .status()?;

    Command::new("sketchybar")
        .args([
            "--add", "item", "media.next", "popup.media",
            "--set", "media.next",
            "icon=󰒭",
            "icon.font=JetBrainsMono Nerd Font:Regular:20.0",
            "width=60",
            "align=center",
            &format!("click_script={} --update-media", exe_path),
        ])
        .status()?;

    Command::new("sketchybar")
        .args([
            "--add", "bracket", "media.controls", "media.prev", "media.play", "media.next",
            "--set", "media.controls",
            "background.drawing=on",
            "background.color=0x22ffffff",
            "background.corner_radius=8",
        ])
        .status()?;

    Ok(())
}
