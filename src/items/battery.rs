use crate::property;
use crate::props::types::{Property, ToSketchybarArgs};
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

impl ToSketchybarArgs for Battery {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let label = if let Some(percentage) = self.percentage {
            format!("{}%", percentage)
        } else {
            "AC".to_string()
        };

        vec![
            property!("icon", &self.icon),
            property!("label", &label),
            property!("drawing", "on"),
        ]
    }
}

impl Battery {
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

        use crate::api;
        use crate::props::item::{
            BackgroundProps, BarItem, ComponentPosition, PopupAlign, PopupProperties, ScriptType,
        };
        use crate::themes::CATPUCCIN_MOCHA;

        let mut item = BarItem::new("battery".to_string(), ComponentPosition::Right);
        item.props.geometry.drawing = Some(true);
        item.props.scripting.update_freq = 60;
        item.props.scripting.script =
            Some(ScriptType::String(format!("{} --update-battery", exe_path)));
        let bg = BackgroundProps {
            color: Some(CATPUCCIN_MOCHA.surface0.clone()),
            drawing: Some(true),
            ..Default::default()
        };
        item.props.geometry.background = Some(bg);

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
        item.props.scripting.click_script = Some(ScriptType::String(
            "sketchybar --set battery popup.drawing=toggle".to_string(),
        ));

        api::add_item(&item)?;
        api::subscribe("battery", ["system_woke", "power_source_change"])?;

        // Popup items
        let mut status_item = BarItem::new("battery.status".to_string(), ComponentPosition::Right);
        status_item.props.icon.icon = Some("Status:".to_string());
        api::add_item(&status_item)?;
        api::set_args("battery.status", ["position=popup.battery"])?;

        let mut wattage_item =
            BarItem::new("battery.wattage".to_string(), ComponentPosition::Right);
        wattage_item.props.icon.icon = Some("Power:".to_string());
        api::add_item(&wattage_item)?;
        api::set_args("battery.wattage", ["position=popup.battery"])?;

        let mut health_item = BarItem::new("battery.health".to_string(), ComponentPosition::Right);
        health_item.props.icon.icon = Some("Health:".to_string());
        api::add_item(&health_item)?;
        api::set_args("battery.health", ["position=popup.battery"])?;

        api::set_item("battery", &battery_data)?;

        // Initialize popup items
        api::set_args(
            "battery.status",
            [&format!("label={}", battery_data.status)],
        )?;
        api::set_args(
            "battery.wattage",
            [&format!(
                "label={:.1}W",
                battery_data.wattage.unwrap_or(0.0)
            )],
        )?;
        api::set_args(
            "battery.health",
            [&format!(
                "label={}% ({} cycles)",
                battery_data.health.unwrap_or(100),
                battery_data.cycle_count.unwrap_or(0)
            )],
        )?;

        Ok(())
    }
}
