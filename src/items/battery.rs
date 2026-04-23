use crate::items::Item;
use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Battery {
    pub percentage: Option<u8>,
    pub icon: String,
    pub wattage: Option<f32>,
    pub cycle_count: Option<u32>,
    pub health: Option<u8>,
    pub status: String,
}

impl Item for Battery {
    fn fetch() -> Result<Self> {
        Battery::fetch()
    }

    fn update_items(&self) -> Result<()> {
        Battery::update_items(self)
    }

    fn setup(exe_path: &str) -> Result<()> {
        Battery::setup(exe_path)
    }
}

impl Battery {
    pub fn update_command() -> Result<()> {
        let data = Self::fetch()?;
        Self::update_items(&data)
    }
    pub fn fetch() -> anyhow::Result<Self> {
        let sender = std::env::var("SENDER").unwrap_or_default();
        let info = std::env::var("INFO").unwrap_or_default();

        let output = Command::new("pmset").args(["-g", "batt"]).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        let percentage = stdout.lines().nth(1).and_then(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() > 1 {
                let status = parts[1];
                let pct_str = status.split('%').next().unwrap_or("0");
                pct_str.trim().parse::<u8>().ok()
            } else {
                None
            }
        });

        let mut wattage = None;
        let mut cycle_count = None;
        let mut health = None;
        let mut status = if stdout.contains("Battery Power") {
            "Discharging".to_string()
        } else if stdout.contains("AC Power") {
            "AC Power".to_string()
        } else {
            "Unknown".to_string()
        };

        // Try to get more info from ioreg
        let ioreg_output = Command::new("ioreg")
            .args(["-n", "AppleSmartBattery", "-r"])
            .output()?;
        let ioreg_stdout = String::from_utf8_lossy(&ioreg_output.stdout);

        if !ioreg_stdout.is_empty() {
            // Parse cycle count
            if let Some(line) = ioreg_stdout.lines().find(|l| l.contains("\"CycleCount\"")) {
                cycle_count = line
                    .split('=')
                    .next_back()
                    .and_then(|v| v.trim().parse().ok());
            }

            // Parse Health (MaxCapacity / DesignCapacity)
            let max_cap: Option<f32> = ioreg_stdout
                .lines()
                .find(|l| l.contains("\"MaxCapacity\""))
                .and_then(|l| l.split('=').next_back())
                .and_then(|v| v.trim().parse().ok());
            let design_cap: Option<f32> = ioreg_stdout
                .lines()
                .find(|l| l.contains("\"DesignCapacity\""))
                .and_then(|l| l.split('=').next_back())
                .and_then(|v| v.trim().parse().ok());

            if let (Some(max), Some(design)) = (max_cap, design_cap) {
                health = Some(((max / design) * 100.0) as u8);
            }

            // Parse Amperage and Voltage for real-time wattage
            let amperage: Option<f32> = ioreg_stdout
                .lines()
                .find(|l| l.contains("\"Amperage\""))
                .and_then(|l| l.split('=').next_back())
                .and_then(|v| v.trim().parse().ok());
            let voltage: Option<f32> = ioreg_stdout
                .lines()
                .find(|l| l.contains("\"Voltage\""))
                .and_then(|l| l.split('=').next_back())
                .and_then(|v| v.trim().parse().ok());

            if let (Some(amp), Some(volt)) = (amperage, voltage) {
                // Amperage is negative when discharging, positive when charging
                wattage = Some((amp * volt).abs() / 1_000_000.0);
            }

            if ioreg_stdout.contains("\"IsCharging\" = Yes") {
                status = "Charging".to_string();
            }
        }

        // Handle power_source_change event info
        if sender == "power_source_change" {
            if info == "AC" {
                status = "Charging".to_string();
            } else if info == "Battery" {
                status = "Discharging".to_string();
            }
        }

        // If no wattage from ioreg (maybe on AC), try pmset -g adapter
        if wattage.is_none() || wattage == Some(0.0) {
            let adapter_output = Command::new("pmset").args(["-g", "adapter"]).output()?;
            let adapter_stdout = String::from_utf8_lossy(&adapter_output.stdout);
            if let Some(line) = adapter_stdout.lines().find(|l| l.contains("Wattage")) {
                wattage = line
                    .split('=')
                    .next_back()
                    .and_then(|v| v.trim().trim_end_matches('W').parse().ok());
            }
        }

        let is_charging = status == "Charging" || status == "AC Power";
        let icon = match percentage {
            Some(p) => if is_charging {
                match p {
                    p if p > 95 => "󰂅",
                    p if p > 90 => "󰂋",
                    p if p > 80 => "󰂊",
                    p if p > 70 => "󰢞",
                    p if p > 60 => "󰂉",
                    p if p > 50 => "󰢝",
                    p if p > 40 => "󰂈",
                    p if p > 30 => "󰂇",
                    p if p > 20 => "󰂆",
                    p if p > 10 => "󰢜",
                    _ => "󰢟",
                }
            } else {
                match p {
                    p if p > 95 => "󰁹",
                    p if p > 90 => "󰂂",
                    p if p > 80 => "󰂁",
                    p if p > 70 => "󰂀",
                    p if p > 60 => "󰁿",
                    p if p > 50 => "󰁾",
                    p if p > 40 => "󰁽",
                    p if p > 30 => "󰁼",
                    p if p > 20 => "󰁻",
                    p if p > 10 => "󰁺",
                    _ => "󰂎",
                }
            }
            .to_string(),
            None => "".to_string(),
        };

        Ok(Self {
            percentage,
            icon,
            wattage,
            cycle_count,
            health,
            status,
        })
    }

    pub fn setup(exe_path: &str) -> anyhow::Result<()> {
        let battery_data = Self::fetch()?;

        use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, PopupAlign};
        use crate::api::types::ToggleState;
        use crate::themes::CATPUCCIN_MOCHA;

        let item = BarItem::new("battery")
            .position(ComponentPosition::Right)
            .drawing(ToggleState::On)
            .update_freq(60)
            .script(&format!("{} --update-battery", exe_path))
            .click_script("sketchybar --set battery popup.drawing=toggle")
            .background_color(CATPUCCIN_MOCHA.surface0.clone())
            .background_drawing(ToggleState::On)
            .popup_align(PopupAlign::Center)
            .popup_background_color(CATPUCCIN_MOCHA.base.clone())
            .popup_background_corner_radius(8)
            .popup_background_border_width(2)
            .popup_background_border_color(CATPUCCIN_MOCHA.surface1.clone())
            .add_item(BarItem::new("battery.status").icon("Status:"))
            .add_item(BarItem::new("battery.wattage").icon("Power:"))
            .add_item(BarItem::new("battery.health").icon("Health:"));

        use crate::api::event::BarEvent;

        item.add()?;
        item.subscribe([BarEvent::SystemWoke, BarEvent::PowerSourceChange])?;

        // Initial update using the data
        Self::update_items(&battery_data)?;

        Ok(())
    }

    pub fn update_items(data: &Self) -> anyhow::Result<()> {
        use crate::api::item::{BarItem, ItemBuilder};
        use crate::api::types::ToggleState;

        let label = if let Some(percentage) = data.percentage {
            format!("{}%", percentage)
        } else {
            "AC".to_string()
        };

        BarItem::new("battery")
            .icon(&data.icon)
            .label(&label)
            .drawing(ToggleState::On)
            .set()?;

        BarItem::new("battery.status").label(&data.status).set()?;
        BarItem::new("battery.wattage")
            .label(&format!("{:.1}W", data.wattage.unwrap_or(0.0)))
            .set()?;
        BarItem::new("battery.health")
            .label(&format!(
                "{}% ({} cycles)",
                data.health.unwrap_or(100),
                data.cycle_count.unwrap_or(0)
            ))
            .set()?;

        Ok(())
    }
}
