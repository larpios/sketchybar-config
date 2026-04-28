use crate::api;
use crate::api::event::BarEvent;
use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, PopupAlign, ToggleState};
use crate::api::types::{Font, FontStyle};
use crate::daemon::DaemonCmd;
use crate::events::Event;
use crate::items::SketchybarItem;
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use local_ip_address::local_ip;
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct NetworkData {
    pub is_connected: bool,
    pub is_wifi: bool,
    pub is_wifi_on: bool,
    pub ssid: String,
    pub ip: String,
    pub device: String,
}

#[derive(Debug, Deserialize, Clone)]
struct WifiNetwork {
    #[serde(rename = "_name")]
    ssid: String,
    #[serde(rename = "spairport_network_security")]
    security: Option<String>,
    #[serde(rename = "spairport_network_signal_strength")]
    signal: Option<i32>,
}

pub struct Network;

impl Network {
    pub fn update_command() -> Result<()> {
        let data = Self::fetch()?;
        Self::update_items(&data)
    }

    pub fn fetch() -> Result<NetworkData> {
        let scutil_output = Command::new("scutil").arg("--nwi").output()?;
        let stdout = String::from_utf8_lossy(&scutil_output.stdout);
        let mut device = String::new();
        let mut ip = String::new();

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

        if ip.is_empty()
            && let Ok(local_ip) = local_ip()
        {
            ip = local_ip.to_string();
        }

        if device.is_empty() {
            return Ok(NetworkData {
                is_connected: false,
                is_wifi: false,
                is_wifi_on: false,
                ssid: String::new(),
                ip: String::new(),
                device: String::new(),
            });
        }

        // Check if the primary interface is Wi-Fi by querying its AirPort state
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
            let hardware_output = Command::new("networksetup")
                .args(["-listallhardwareports"])
                .output()?;
            let hw_stdout = String::from_utf8_lossy(&hardware_output.stdout);
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

        let mut is_wifi_on = false;
        if is_wifi {
            let power_output = Command::new("networksetup")
                .args(["-getairportpower", &device])
                .output()?;
            is_wifi_on = String::from_utf8_lossy(&power_output.stdout).contains("On");
        }

        Ok(NetworkData {
            is_connected: !ip.is_empty() && ip != "127.0.0.1",
            is_wifi,
            is_wifi_on,
            ssid,
            ip,
            device,
        })
    }

    pub fn update_items(data: &NetworkData) -> Result<()> {
        let item = if !data.is_connected {
            BarItem::new("network").icon("󰤮")
        } else {
            let icon = if data.is_wifi { "󰤨" } else { "󰈀" };
            BarItem::new("network").icon(icon)
        };
        item.set()?;
        Ok(())
    }

    pub async fn update_popup() -> Result<()> {
        let data = Self::fetch()?;
        if !data.is_wifi {
            return Ok(());
        }

        // Clear old popup items
        let _ = Command::new("sketchybar")
            .args([
                "--remove",
                "/network\\.device\\..*/",
                "/network\\.section\\..*/",
                "network.toggle",
                "network.loading",
            ])
            .status();

        let exe_path = std::env::current_exe()?.to_string_lossy().to_string();

        // Wi-Fi toggle row: single item with colored icon indicating on/off
        let (toggle_icon, toggle_color) = if data.is_wifi_on {
            ("󰤨", CATPUCCIN_MOCHA.blue)
        } else {
            ("󰤫", CATPUCCIN_MOCHA.overlay0)
        };
        let next_state = if data.is_wifi_on { "off" } else { "on" };
        let toggle_script = daemon_send_script(
            &exe_path,
            &DaemonCmd::ToggleWifiPower {
                device: data.device.clone(),
                state: next_state.to_string(),
            },
        );

        let toggle_item = BarItem::new("network.toggle")
            .icon(toggle_icon)
            .icon_color(toggle_color)
            .label("Wi-Fi")
            .width(250)
            .padding_left(8)
            .click_script(&toggle_script);

        api::add_special_item("item", "network.toggle", "popup.network", &toggle_item)?;

        if !data.is_wifi_on {
            return Ok(());
        }

        // Show loading indicator while scanning
        let loading = BarItem::new("network.loading")
            .icon("󰑐")
            .icon_color(CATPUCCIN_MOCHA.yellow)
            .label("Scanning…")
            .width(250)
            .padding_left(8);

        api::add_special_item("item", "network.loading", "popup.network", &loading)?;

        // Blocking scan
        let networks = Self::scan_networks(&data.device).unwrap_or_default();
        let known = Self::fetch_known_networks(&data.device).unwrap_or_default();

        let _ = Command::new("sketchybar")
            .args(["--remove", "network.loading"])
            .status();

        Self::render_network_list(networks, known, &data.device, &exe_path).await?;

        Ok(())
    }

    pub async fn toggle_wifi_power(device: &str, state: &str) -> Result<()> {
        let _ = Command::new("networksetup")
            .args(["-setairportpower", device, state])
            .status();

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        Self::update_popup().await?;
        Self::update_command()?;
        Ok(())
    }

    pub async fn connect_network(ssid: &str, device: &str, security: &str) -> Result<()> {
        let is_protected = security != "Open" && !security.is_empty();

        let password = if is_protected {
            let script = format!(
                "display dialog \"Enter password for Wi-Fi \\\"{}\\\":\" default answer \"\" with hidden answer",
                ssid
            );
            let output = Command::new("osascript")
                .args(["-e", &script, "-e", "text returned of result"])
                .output()?;

            if !output.status.success() {
                return Err(anyhow!("Password entry cancelled"));
            }
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            String::new()
        };

        let mut cmd = Command::new("networksetup");
        cmd.args(["-setairportnetwork", device, ssid]);
        if !password.is_empty() {
            cmd.arg(&password);
        }
        let _ = cmd.status();

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        Ok(())
    }

    fn fetch_known_networks(device: &str) -> Result<Vec<String>> {
        let output = Command::new("networksetup")
            .args(["-listpreferredwirelessnetworks", device])
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout
            .lines()
            .skip(1)
            .map(|l| l.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }

    fn scan_networks(device: &str) -> Result<Vec<WifiNetwork>> {
        let output = Command::new("system_profiler")
            .args(["SPAirPortDataType", "-json"])
            .output()?;

        let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        let mut networks = Vec::new();

        if let Some(data_array) = json.get("SPAirPortDataType").and_then(|v| v.as_array()) {
            for data in data_array {
                if let Some(interfaces) = data
                    .get("spairport_airport_interfaces")
                    .and_then(|v| v.as_array())
                {
                    for interface in interfaces {
                        if interface.get("_name").and_then(|v| v.as_str()) == Some(device)
                            && let Some(list) = interface
                                .get("spairport_network_list")
                                .and_then(|v| v.as_array())
                        {
                            for entry in list {
                                if let Ok(network) =
                                    serde_json::from_value::<WifiNetwork>(entry.clone())
                                {
                                    networks.push(network);
                                }
                            }
                        }
                    }
                }
            }
        }

        networks.sort_by_key(|n| std::cmp::Reverse(n.signal.unwrap_or(-100)));

        let mut seen = std::collections::HashSet::new();
        networks.retain(|n| seen.insert(n.ssid.clone()));

        Ok(networks)
    }

    async fn render_network_list(
        networks: Vec<WifiNetwork>,
        known_ssids: Vec<String>,
        device: &str,
        exe_path: &str,
    ) -> Result<()> {
        let _ = Command::new("sketchybar")
            .args([
                "--remove",
                "/network\\.device\\..*/",
                "/network\\.section\\..*/",
            ])
            .status();

        let mut known = Vec::new();
        let mut other = Vec::new();
        for n in networks {
            if known_ssids.contains(&n.ssid) {
                known.push(n);
            } else {
                other.push(n);
            }
        }

        if !known.is_empty() {
            Self::render_section_header("network.section.known", "KNOWN NETWORKS")?;
            for network in known {
                Self::render_single_network(network, device, exe_path)?;
            }
        }

        if !other.is_empty() {
            Self::render_section_header("network.section.other", "OTHER NETWORKS")?;
            for network in other {
                Self::render_single_network(network, device, exe_path)?;
            }
        }

        Ok(())
    }

    fn render_section_header(name: &str, label: &str) -> Result<()> {
        let header = BarItem::new(name)
            .label(label)
            .label_color(CATPUCCIN_MOCHA.overlay1)
            .label_font(Font {
                family: "JetBrainsMono Nerd Font".into(),
                style: FontStyle::Bold,
                size: 10.0,
            })
            .padding_left(8)
            .width(250);

        api::add_special_item("item", name, "popup.network", &header)?;
        Ok(())
    }

    fn render_single_network(network: WifiNetwork, device: &str, exe_path: &str) -> Result<()> {
        let sanitized_ssid = network
            .ssid
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>();
        let name = format!("network.device.{}", sanitized_ssid);

        let security = network.security.as_deref().unwrap_or("Open");
        let icon = if security != "Open" && !security.is_empty() {
            "󰤪"
        } else {
            "󰤨"
        };

        let connect_script = daemon_send_script(
            exe_path,
            &DaemonCmd::ConnectNetwork {
                ssid: network.ssid.clone(),
                device: device.to_string(),
                security: security.to_string(),
            },
        );

        let click_script = format!(
            "sketchybar --animate sin 10 --set $NAME background.highlight=on; sleep 0.1; sketchybar --animate sin 10 --set $NAME background.highlight=off; {}",
            connect_script
        );

        let item = BarItem::new(&name)
            .icon(icon)
            .label(&network.ssid)
            .width(250)
            .padding_left(8)
            .background_color(CATPUCCIN_MOCHA.transparent)
            .background_height(28)
            .background_drawing(ToggleState::On)
            .background_corner_radius(6)
            .click_script(&click_script);

        api::add_special_item("item", &name, "popup.network", &item)?;
        api::subscribe(&name, [BarEvent::MouseEntered, BarEvent::MouseExited])?;

        Ok(())
    }
}

#[async_trait]
impl SketchybarItem for Network {
    async fn setup(&self, exe_path: &str) -> Result<()> {
        let popup_cmd = daemon_send_script(exe_path, &DaemonCmd::UpdateNetworkPopup);

        let item = BarItem::new("network")
            .position(ComponentPosition::Right)
            .update_freq(10)
            .script(&format!("{} --update-network", exe_path))
            .click_script(&format!(
                "sketchybar --animate sin 15 --set network popup.drawing=toggle; {}",
                popup_cmd
            ))
            .label_drawing(ToggleState::Off)
            .background_color(CATPUCCIN_MOCHA.surface0)
            .background_drawing(ToggleState::On)
            .popup_align(PopupAlign::Right)
            .popup_background_color(CATPUCCIN_MOCHA.base)
            .popup_background_border_color(CATPUCCIN_MOCHA.surface1)
            .popup_background_border_width(2)
            .popup_background_corner_radius(12);

        item.add()?;
        item.subscribe([BarEvent::WifiChange])?;

        let data = Self::fetch()?;
        Self::update_items(&data)?;

        Ok(())
    }

    async fn spawn_background_task(&self, mut bus: tokio::sync::broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            while let Ok(event) = bus.recv().await {
                match event {
                    Event::UpdateNetwork => {
                        if let Err(e) = Self::update_popup().await {
                            eprintln!("[network] popup update error: {e}");
                        }
                    }
                    Event::NetworkAction(data) => {
                        if data.action.starts_with("toggle-power:") {
                            let state = data.action.split(':').nth(1).unwrap_or("on");
                            let device = data.ssid.unwrap_or_default();
                            if let Err(e) = Self::toggle_wifi_power(&device, state).await {
                                eprintln!("[network] toggle power error: {e}");
                            }
                        } else if data.action.starts_with("connect:") {
                            let parts: Vec<&str> = data.action.split(':').collect();
                            let security = parts.get(1).unwrap_or(&"Open");
                            let device = parts.get(2).map(|s| s.to_string()).unwrap_or_else(|| {
                                Self::fetch().map(|d| d.device).unwrap_or_default()
                            });
                            let ssid = data.ssid.unwrap_or_default();
                            if let Err(e) = Self::connect_network(&ssid, &device, security).await {
                                eprintln!("[network] connect error: {e}");
                            } else {
                                let _ = Self::update_command();
                                let _ = Self::update_popup().await;
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

/// Build a shell command that sends a daemon command via `--send`.
fn daemon_send_script(exe_path: &str, cmd: &DaemonCmd) -> String {
    let json = serde_json::to_string(cmd).unwrap_or_default();
    // Escape single quotes for shell: ' → '\''
    let safe_json = json.replace('\'', "'\\''");
    format!("{} --send '{}'", exe_path, safe_json)
}
