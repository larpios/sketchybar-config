use crate::api;
use crate::props::item::{
    BackgroundProps, BarItem, ComponentPosition, PopupAlign, PopupProperties, ScriptType, Text,
};
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use std::env;
use std::process::Command;

pub fn update() -> Result<()> {
    let sender = env::var("SENDER").unwrap_or_else(|_| "".to_string());
    let name = env::var("NAME").unwrap_or_default();

    if name == "volume.slider" {
        if let Ok(info) = env::var("INFO") {
            let _ = Command::new("osascript")
                .args(["-e", &format!("set volume output volume {}", info)])
                .status();
        }
        return Ok(());
    }

    match sender.as_str() {
        "mouse.scrolled.up" => {
            let _ = Command::new("osascript")
                .args([
                    "-e",
                    "set volume output volume ((output volume of (get volume settings)) + 5)",
                ])
                .status();
        }
        "mouse.scrolled.down" => {
            let _ = Command::new("osascript")
                .args([
                    "-e",
                    "set volume output volume ((output volume of (get volume settings)) - 5)",
                ])
                .status();
        }
        "mouse.clicked" => {
            // Only toggle popup if we clicked the volume item itself, not the slider
            if name == "volume" {
                let _ = Command::new("sketchybar")
                    .args(["--set", "volume", "popup.drawing=toggle"])
                    .status();
            }
        }
        _ => {}
    }

    // Always update the UI based on INFO or fallback to current volume via osascript
    let vol_str = if sender == "volume_change" {
        env::var("INFO").unwrap_or_else(|_| "50".to_string())
    } else {
        // Fallback: fetch current volume
        let output = Command::new("osascript")
            .args(["-e", "output volume of (get volume settings)"])
            .output()?;
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    };

    let vol: u8 = vol_str.parse().unwrap_or(50);

    // Check if muted
    let muted_output = Command::new("osascript")
        .args(["-e", "output muted of (get volume settings)"])
        .output();
    let is_muted = match muted_output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "true",
        Err(_) => false,
    };

    let icon = if is_muted {
        "󰝟"
    } else {
        match vol {
            0 => "󰝟",
            1..=33 => "󰕿",
            34..=66 => "󰖀",
            _ => "󰕾",
        }
    };

    api::set_args(
        "volume",
        [&format!("icon={}", icon), &format!("label={}%", vol)],
    )?;

    // Keep slider in sync
    api::set_args("volume.slider", [&format!("slider.percentage={}", vol)])?;

    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    api::add_event("volume_change")?;

    let mut item = BarItem::new("volume".to_string(), ComponentPosition::Right);
    item.scripting.script = Some(ScriptType::String(format!("{} --update-volume", exe_path)));

    let mut bg = BackgroundProps::new();
    bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
    bg.drawing = Some(true);
    item.geometry.background = Some(bg);
    item.geometry.scroll_texts = Some(true);

    let icon_props = Text {
        color: Some(CATPUCCIN_MOCHA.blue.clone()),
        ..Default::default()
    };
    item.icon.props = Some(icon_props);

    // Add popup properties to volume item
    let popup_bg = BackgroundProps {
        color: Some(CATPUCCIN_MOCHA.base.clone()),
        corner_radius: Some(8),
        border_width: Some(2),
        border_color: Some(CATPUCCIN_MOCHA.surface1.clone()),
        ..Default::default()
    };
    let popup_props = PopupProperties {
        align: PopupAlign::Center,
        background: Some(popup_bg),
        ..Default::default()
    };
    item.popup = Some(popup_props);

    api::add_item(&item)?;
    api::subscribe(
        &item.name,
        &[
            "volume_change",
            "mouse.scrolled.up",
            "mouse.scrolled.down",
            "mouse.clicked",
        ],
    )?;

    // Add slider to popup
    Command::new("sketchybar")
        .args([
            "--add",
            "slider",
            "volume.slider",
            "popup.volume",
            "--set",
            "volume.slider",
            "slider.highlight_color=0xff8aadf4",
            "slider.background.height=5",
            "slider.background.corner_radius=3",
            "slider.background.color=0xff313244",
            "slider.knob=",
            "slider.knob.drawing=on",
            "slider.knob.font=JetBrainsMono Nerd Font:Regular:14.0",
            "padding_left=15",
            "padding_right=15",
            "width=100",
            &format!("script={} --update-volume", exe_path),
        ])
        .status()?;

    // Initial trigger
    let _ = Command::new("sketchybar")
        .args(["--trigger", "volume_change"])
        .status();

    Ok(())
}
