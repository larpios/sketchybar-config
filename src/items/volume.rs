use crate::api;
use crate::api::event::BarEvent;
use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, PopupAlign, Slider};
use crate::api::types::{Font, ToggleState};
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use std::env;
use std::process::Command;

pub fn update_command() -> Result<()> {
    update()
}

pub fn update() -> Result<()> {
    const VOLUME_SCROLL_SENSITIVITY: f32 = 0.2;

    let sender = env::var("SENDER").unwrap_or_default();
    let name = env::var("NAME").unwrap_or_default();
    let info = env::var("INFO").unwrap_or_default();
    let percentage = env::var("PERCENTAGE").unwrap_or_default();
    let scroll_delta = env::var("SCROLL_DELTA").unwrap_or_default();

    if name == "volume.slider" && !percentage.is_empty() {
        if let Ok(vol) = percentage.parse::<u8>() {
            let _ = Command::new("osascript")
                .args(["-e", &format!("set volume output volume {}", vol)])
                .status();
        }
    } else {
        match sender.as_str() {
            "mouse.scrolled" => {
                if let Ok(delta) = scroll_delta.parse::<f32>() {
                    let _ = Command::new("osascript")
                    .args([
                        "-e",
                        format!("set volume output volume ((output volume of (get volume settings)) + {})", -delta * VOLUME_SCROLL_SENSITIVITY).as_str(),
                    ])
                    .status();
                }
            }
            "mouse.clicked" if name == "volume" => {
                BarItem::new("volume")
                    .popup_drawing(ToggleState::Toggle)
                    .set()?;
            }
            _ => {}
        }
    }

    // Fetch current volume level
    let vol_str = if sender == "volume_change" && !info.is_empty() {
        info
    } else if name == "volume.slider" && !percentage.is_empty() {
        percentage
    } else {
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

    BarItem::new("volume")
        .icon(icon)
        .label(&format!("{}%", vol))
        .set()?;

    Slider::new("volume.slider").percentage(vol as u32).set()?;

    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    api::add_event("volume_change")?;

    let item = BarItem::new("volume")
        .position(ComponentPosition::Right)
        .script(&format!("{} --update-volume", exe_path))
        .background_color(CATPUCCIN_MOCHA.surface0)
        .background_drawing(ToggleState::On)
        .icon_color(CATPUCCIN_MOCHA.blue)
        .popup_align(PopupAlign::Center)
        .popup_background_color(CATPUCCIN_MOCHA.base)
        .popup_background_corner_radius(8)
        .popup_background_border_width(2)
        .popup_background_border_color(CATPUCCIN_MOCHA.surface1)
        .add_slider(
            Slider::new("volume.slider")
                .percentage(50)
                .background_drawing(ToggleState::Off)
                .slider_width(100)
                .slider_background_height(5)
                .slider_background_corner_radius(3)
                .slider_background_color(CATPUCCIN_MOCHA.surface1)
                .slider_background_drawing(ToggleState::On)
                .padding_left(5)
                .padding_right(5)
                .highlight_color(CATPUCCIN_MOCHA.lavender)
                .knob("󰝥")
                .knob_color(CATPUCCIN_MOCHA.blue)
                .knob_font(Font::default())
                .script(r#"osascript -e "set volume output volume $PERCENTAGE""#),
        );

    item.add()?;
    item.subscribe([
        BarEvent::VolumeChange,
        BarEvent::MouseScrolled,
        BarEvent::MouseClicked,
    ])?;

    // Slider also needs mouse clicked subscription
    api::subscribe("volume.slider", [BarEvent::MouseClicked])?;

    // Initial trigger
    let _ = Command::new("sketchybar")
        .args(["--trigger", "volume_change"])
        .status();

    Ok(())
}
