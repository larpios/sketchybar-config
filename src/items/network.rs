use crate::property;
use crate::props::types::{Property, ToSketchybarArgs};
use local_ip_address::local_ip;
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
        // Use scutil to find the primary interface - much faster and more reliable on macOS
        let scutil_output = Command::new("scutil").arg("--nwi").output()?;

        let stdout = String::from_utf8_lossy(&scutil_output.stdout);
        let mut device = String::new();
        let mut ip = String::new();

        // Parse scutil --nwi output
        // Look for "Network interfaces: <device>" line or the primary interface in IPv4 section
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Network interfaces:") {
                device = trimmed
                    .replace("Network interfaces:", "")
                    .trim()
                    .split(' ')
                    .next()
                    .unwrap_or("")
                    .to_string();
            } else if trimmed.contains("address") && ip.is_empty() {
                ip = trimmed.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }

        // Fallback for IP if scutil didn't give it or if we want to be sure
        if ip.is_empty()
            && let Ok(local_ip) = local_ip()
        {
            ip = local_ip.to_string();
        }

        if device.is_empty() {
            return Ok(Self {
                is_connected: false,
                is_wifi: false,
                ssid: String::new(),
                ip: String::new(),
                device: String::new(),
            });
        }

        // Determine if it's wifi and get SSID
        // We check if it's a wifi interface using scutil as well
        let mut child = Command::new("scutil")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(
                    format!("show State:/Network/Interface/{}/AirPort\n", device).as_bytes(),
                )?;
            }
        }

        let airport_output = child.wait_with_output()?;

        let airport_stdout = String::from_utf8_lossy(&airport_output.stdout);
        let mut ssid = String::new();
        let mut is_wifi = false;

        if !airport_stdout.is_empty() && !airport_stdout.contains("no such key") {
            is_wifi = true;
            for line in airport_stdout.lines() {
                if line.contains("SSID_STR :") {
                    ssid = line.split(':').nth(1).unwrap_or("").trim().to_string();
                }
            }
        } else {
            // Check if it's Wi-Fi even if not associated
            let hardware_output = Command::new("networksetup")
                .args(["-listallhardwareports"])
                .output()?;
            let hw_stdout = String::from_utf8_lossy(&hardware_output.stdout);
            if hw_stdout.contains(&format!("Device: {}", device)) {
                // Find if the port for this device is Wi-Fi
                let lines: Vec<&str> = hw_stdout.lines().collect();
                for i in 0..lines.len() {
                    if lines[i].contains(&format!("Device: {}", device))
                        && i > 0
                        && lines[i - 1].contains("Wi-Fi")
                    {
                        is_wifi = true;
                        break;
                    }
                }
            }
        }

        Ok(Self {
            is_connected: true,
            is_wifi,
            ssid,
            ip,
            device,
        })
    }

    pub fn setup(exe_path: &str) -> anyhow::Result<()> {
        use crate::api;
        use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType};
        use crate::themes::CATPUCCIN_MOCHA;

        let mut item = BarItem::new("network".to_string(), ComponentPosition::Right);
        item.props.scripting.update_freq = 10;
        item.props.scripting.script =
            Some(ScriptType::String(format!("{} --update-network", exe_path)));
        let bg = BackgroundProps {
            color: Some(CATPUCCIN_MOCHA.surface0.clone()),
            drawing: Some(true),
            ..Default::default()
        };
        item.props.geometry.background = Some(bg);
        api::add_item(&item)?;
        Ok(())
    }
}
