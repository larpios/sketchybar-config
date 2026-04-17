use crate::api;
use crate::props::components::Slider;
use crate::props::item::{
    BackgroundProps, BarItem, ComponentPosition, ItemProps, PopupAlign, PopupProperties,
    ScriptType, Scripting, Text,
};
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use std::env;
use std::process::Command;

pub fn update() -> Result<()> {
    let sender = env::var("SENDER").unwrap_or_else(|_| "".to_string());
    let name = env::var("NAME").unwrap_or_default();

    if name == "volume.slider" {
        // Get the slider percentage from SLIDER_PERCENTAGE or fetch current volume
        let slider_value = env::var("SLIDER_PERCENTAGE").unwrap_or_else(|_| {
            // Fallback: query sketchybar for current slider value
            let output = Command::new("sketchybar")
                .args(["--query", "volume.slider"])
                .output()
                .ok();
            if let Some(output) = output {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .find_map(|line| {
                        if line.contains("\"slider.percentage\"") {
                            line.split(":")
                                .nth(1)?
                                .trim()
                                .trim_end_matches(",")
                                .parse::<f32>()
                                .ok()
                                .map(|v| (v as u8).to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| "50".to_string())
            } else {
                "50".to_string()
            }
        });

        if let Ok(vol) = slider_value.parse::<u8>() {
            let _ = Command::new("osascript")
                .args(["-e", &format!("set volume output volume {}", vol)])
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
    item.props.scripting.script = Some(ScriptType::String(format!("{} --update-volume", exe_path)));

    let bg = BackgroundProps {
        color: Some(CATPUCCIN_MOCHA.surface0.clone()),
        drawing: Some(true),
        ..Default::default()
    };
    item.props.geometry.background = Some(bg);
    item.props.geometry.scroll_texts = Some(true);

    let icon_props = Text {
        color: Some(CATPUCCIN_MOCHA.blue.clone()),
        ..Default::default()
    };
    item.props.icon.props = Some(icon_props);

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
    item.props.popup = Some(popup_props);

    api::add_item(&item)?;
    api::subscribe(
        &item.name,
        [
            "volume_change",
            "mouse.scrolled.up",
            "mouse.scrolled.down",
            "mouse.clicked",
        ],
    )?;

    // Setup Slider
    let slider = Slider {
        width: Some(100),
        highlight_color: Some(CATPUCCIN_MOCHA.blue.clone()),
        knob: Some("".to_string()),
        knob_props: Some(Text {
            color: Some(CATPUCCIN_MOCHA.blue.clone()),
            font: Some(crate::props::types::Font {
                family: "JetBrainsMono Nerd Font".to_string(),
                size: 14.0,
                type_: crate::props::types::FontType::Regular,
            }),
            ..Default::default()
        }),
        background: Some(BackgroundProps {
            color: Some(CATPUCCIN_MOCHA.surface0.clone()),
            height: Some(5),
            corner_radius: Some(3),
            padding_left: Some(15),
            padding_right: Some(15),
            ..Default::default()
        }),
        item: Some(ItemProps {
            scripting: Scripting {
                script: Some(ScriptType::String(format!("{} --update-volume", exe_path))),
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    api::add_special_item("slider", "volume.slider", "popup.volume", &slider)?;
    api::subscribe("volume.slider", ["slider.changed"])?;

    // Initial trigger
    let _ = Command::new("sketchybar")
        .args(["--trigger", "volume_change"])
        .status();

    Ok(())
}
