use crate::api;
use crate::api::event::BarEvent;
use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, PopupAlign, Slider};
use crate::api::types::{Font, ToggleState};
use crate::events::Event;
use crate::items::SketchybarItem;
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use async_trait::async_trait;
use std::env;
use std::process::Command;

pub struct Volume;

impl Volume {
    pub fn update_command() -> Result<()> {
        Self::update()
    }

    pub fn update() -> Result<()> {
        const VOLUME_SCROLL_SENSITIVITY: f32 = 0.2;

        let sender = env::var("SENDER").unwrap_or_default();
        let name = env::var("NAME").unwrap_or_default();
        let info = env::var("INFO").unwrap_or_default();
        let percentage = env::var("PERCENTAGE").unwrap_or_default();
        let scroll_delta = env::var("SCROLL_DELTA").unwrap_or_default();

        if name == "volume.slider" && !percentage.is_empty() {
            if let Ok(vol) = percentage.parse::<f32>() {
                let vol_int = vol as u8;
                let _ = Command::new("osascript")
                    .args([
                        "-e",
                        "on run argv",
                        "-e",
                        "set volume output volume (item 1 of argv)",
                        "-e",
                        "end run",
                        &vol_int.to_string(),
                    ])
                    .status();
            }
        } else {
            match sender.as_str() {
                "mouse.scrolled" => {
                    if let Ok(delta) = scroll_delta.parse::<f32>() {
                        let vol_delta = -delta * VOLUME_SCROLL_SENSITIVITY;
                        let _ = Command::new("osascript")
                            .args([
                                "-e",
                                "on run argv",
                                "-e",
                                "set volume output volume ((output volume of (get volume settings)) + (item 1 of argv))",
                                "-e",
                                "end run",
                                &vol_delta.to_string(),
                            ])
                            .status();
                    }
                }
                "mouse.clicked" if name == "volume" => {
                    BarItem::new("volume")
                        .popup_drawing(ToggleState::Toggle)
                        .animate_set("sin", 15)?;
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

        let vol: u8 = vol_str.parse::<f32>().map(|f| f as u8).unwrap_or(50);

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
}

#[async_trait]
impl SketchybarItem for Volume {
    async fn setup(&self, exe_path: &str) -> Result<()> {
        let current_vol_output = Command::new("osascript")
            .args(["-e", "output volume of (get volume settings)"])
            .output()?;
        let current_vol: u32 = String::from_utf8_lossy(&current_vol_output.stdout)
            .trim()
            .parse::<f32>()
            .map(|f| f as u32)
            .unwrap_or(50);

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
                    .percentage(current_vol)
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
                    .script(&format!("{} --update-volume", exe_path)),
            );

        item.add()?;
        item.subscribe([
            BarEvent::VolumeChange,
            BarEvent::MouseScrolled,
            BarEvent::MouseClicked,
        ])?;

        // Slider also needs mouse clicked subscription
        api::subscribe("volume.slider", [BarEvent::MouseClicked])?;

        Ok(())
    }

    async fn spawn_background_task(&self, mut bus: tokio::sync::broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            while let Ok(event) = bus.recv().await {
                if matches!(event, Event::UpdateVolume)
                    && let Err(e) = Self::update_command()
                {
                    eprintln!("[volume] update error: {e}");
                }
            }
        });
    }
}
