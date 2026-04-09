use crate::property;
use crate::props::types::{Property, ToSketchybarArgs};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Network {
    pub is_connected: bool,
    pub is_wifi: bool,
    pub ssid: String,
    pub ip: String,
    pub device: String,
}

impl ToSketchybarArgs for Network {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        if !self.is_connected {
            vec![property!("icon", "󰤮"), property!("label", "Disconnected")]
        } else {
            let icon = if self.is_wifi { "󰤨" } else { "󰈀" };
            let display_ssid = if self.ssid.is_empty() {
                if self.is_wifi {
                    "Wi-Fi".to_string()
                } else {
                    "Ethernet".to_string()
                }
            } else {
                self.ssid.clone()
            };

            vec![property!("icon", icon), property!("label", &display_ssid)]
        }
    }
}

impl Network {
    pub fn fetch() -> anyhow::Result<Self> {
        // Find active interface via default route
        let route_output = Command::new("sh")
            .arg("-c")
            .arg("route get default | grep interface | awk '{print $2}'")
            .output()?;

        let device = String::from_utf8_lossy(&route_output.stdout).trim().to_string();

        if device.is_empty() {
            return Ok(Self {
                is_connected: false,
                is_wifi: false,
                ssid: String::new(),
                ip: String::new(),
                device: String::new(),
            });
        }

        let mut is_wifi = false;
        let mut ssid = String::new();

        // Try getting wifi network name
        let networksetup_output = Command::new("sh")
            .arg("-c")
            .arg(&format!("networksetup -getairportnetwork {}", device))
            .output();
            
        if let Ok(output) = networksetup_output {
            let ssid_str = String::from_utf8_lossy(&output.stdout);
            if ssid_str.contains("Current Wi-Fi Network: ") {
                ssid = ssid_str.replace("Current Wi-Fi Network: ", "").trim().to_string();
                is_wifi = true;
            } else if !ssid_str.contains("is not a Wi-Fi interface") && !ssid_str.contains("You are not associated") {
                 // Fallback for getting wifi status
                 is_wifi = true;
            }
        }

        // Get IP
        let ip_output = Command::new("sh")
            .arg("-c")
            .arg(&format!("ipconfig getifaddr {}", device))
            .output();

        let ip = if let Ok(output) = ip_output {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            String::new()
        };

        Ok(Self {
            is_connected: true,
            is_wifi,
            ssid,
            ip,
            device,
        })
    }

    pub fn setup(exe_path: &str) -> anyhow::Result<()> {
        use crate::props::item::{BarItem, BackgroundProps, ComponentPosition, ScriptType};
        use crate::themes::CATPUCCIN_MOCHA;
        use crate::api;

        let mut item = BarItem::new("network".to_string(), ComponentPosition::Right);
        item.scripting.update_freq = 10;
        item.scripting.script = Some(ScriptType::String(format!("{} --update-network", exe_path)));
        let mut bg = BackgroundProps::new();
        bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
        bg.drawing = Some(true);
        item.geometry.background = Some(bg);
        api::add_item(&item)?;
        Ok(())
    }
}
