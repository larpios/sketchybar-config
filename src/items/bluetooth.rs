use crate::api;
use crate::api::item::{Argb, BarItem, ComponentPosition, ItemBuilder, PopupAlign, ToggleState};
use crate::api::types::{Font, FontStyle};
use crate::daemon::DaemonCmd;
use crate::events::Event;
use crate::items::SketchybarItem;
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BluetoothDeviceData {
    pub name: String,
    pub address: String,
    pub connected: bool,
    pub is_nearby: bool,
    pub device_type: Option<String>,
}

pub struct Bluetooth;

impl Bluetooth {
    pub async fn fetch_status() -> Result<(bool, Vec<BluetoothDeviceData>)> {
        // 1. Get paired devices from system_profiler
        let output = Command::new("system_profiler")
            .args(["SPBluetoothDataType", "-json"])
            .output()?;

        let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;

        let mut devices = Vec::new();
        let mut is_on = false;

        if let Some(data_array) = json.get("SPBluetoothDataType").and_then(|v| v.as_array()) {
            for data in data_array {
                if let Some(props) = data.get("controller_properties")
                    && let Some(state) = props.get("controller_state").and_then(|v| v.as_str())
                {
                    is_on = state == "attrib_on";
                }

                let mut process_devices = |device_list: &serde_json::Value, connected: bool| {
                    if let Some(list) = device_list.as_array() {
                        for entry in list {
                            if let Some(map) = entry.as_object() {
                                for (name, info) in map {
                                    if let Some(address) =
                                        info.get("device_address").and_then(|v| v.as_str())
                                    {
                                        let device_type = info
                                            .get("device_minorType")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string());
                                        devices.push(BluetoothDeviceData {
                                            name: name.clone(),
                                            address: address.to_string(),
                                            connected,
                                            is_nearby: false,
                                            device_type,
                                        });
                                    }
                                }
                            }
                        }
                    }
                };

                if let Some(connected) = data.get("device_connected") {
                    process_devices(connected, true);
                }
                if let Some(not_connected) = data.get("device_not_connected") {
                    process_devices(not_connected, false);
                }
            }
        }

        Ok((is_on, devices))
    }

    pub async fn update() -> Result<()> {
        let (is_on, devices) = Self::fetch_status().await?;
        let connected_count = devices.iter().filter(|d| d.connected).count();

        let icon = if !is_on {
            "󰂲"
        } else if connected_count > 0 {
            "󰂱"
        } else {
            ""
        };

        let item = BarItem::new("bluetooth")
            .icon(icon)
            .label_props(|p| p.drawing(ToggleState::Off))
            .apply_if(connected_count > 0, |item| {
                item.label(&format!("{}", connected_count))
                    .label_props(|p| p.drawing(ToggleState::On))
            });

        item.set()?;

        Ok(())
    }

    pub async fn update_popup(scan: bool) -> Result<()> {
        let (_, devices) = Self::fetch_status().await?;
        Self::render_device_list(devices, Vec::new()).await?;

        if scan {
            // Start continuous background scan
            tokio::spawn(async move {
                let _ = Self::continuous_scan().await;
            });
        }

        Ok(())
    }

    async fn continuous_scan() -> Result<()> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        let central = adapters
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No adapter"))?;

        central.start_scan(ScanFilter::default()).await?;

        // Show "Searching..." indicator at the top
        let loading = BarItem::new_with_pos(
            "bluetooth.loading",
            ComponentPosition::Popup("bluetooth".to_string()),
        )
        .icon("󰑐")
        .label("Searching for nearby devices...")
        .icon_props(|p| p.color(CATPUCCIN_MOCHA.yellow))
        .label_props(|p| {
            p.color(CATPUCCIN_MOCHA.subtext0).font(Font {
                family: "JetBrainsMono Nerd Font".to_string(),
                style: FontStyle::Italic,
                size: 11.0,
            })
        })
        .width(320);

        loading.add()?;

        use futures::StreamExt;
        let mut events = central.events().await?;
        let mut nearby_discovered = Vec::new();

        let mut check_interval = tokio::time::interval(Duration::from_millis(500));
        let spinners = ["", "", "", "", "", ""];
        let mut spinner_idx = 0;

        loop {
            tokio::select! {
                Some(event) = events.next() => {
                    if let btleplug::api::CentralEvent::DeviceDiscovered(id) = event
                        && let Ok(Some(props)) = central.peripheral(&id).await?.properties().await {
                            let name = props.local_name.unwrap_or_else(|| "Unknown Device".to_string());
                            let address = props.address.to_string();

                            if !nearby_discovered.iter().any(|d: &BluetoothDeviceData| d.address == address) {
                                let device = BluetoothDeviceData {
                                    name,
                                    address,
                                    connected: false,
                                    is_nearby: true,
                                    device_type: None,
                                };
                                nearby_discovered.push(device.clone());
                                Self::render_single_device(device, false).await?;
                            }
                        }
                }
                _ = check_interval.tick() => {
                    // Check if popup is still open
                    let output = Command::new("sketchybar").args(["--query", "bluetooth"]).output()?;
                    let query: serde_json::Value = serde_json::from_slice(&output.stdout)?;
                    if let Some(drawing) = query.get("popup").and_then(|v| v.get("drawing")).and_then(|v| v.as_str()) {
                        if drawing == "off" {
                            break;
                        }
                    } else {
                        break;
                    }

                    // Animate spinner
                    spinner_idx = (spinner_idx + 1) % spinners.len();
                    BarItem::new("bluetooth.loading").icon(spinners[spinner_idx]).set()?;
                }
            }
        }

        let _ = central.stop_scan().await;
        let _ = Command::new("sketchybar")
            .arg("--remove")
            .arg("bluetooth.loading")
            .status();
        Ok(())
    }

    async fn render_device_list(
        paired: Vec<BluetoothDeviceData>,
        nearby: Vec<BluetoothDeviceData>,
    ) -> Result<()> {
        let _ = Command::new("sketchybar")
            .args([
                "--remove",
                "/bluetooth\\.device\\..*/",
                "bluetooth.section.*",
            ])
            .status();

        // MY DEVICES Header
        let paired_header = BarItem::new_with_pos(
            "bluetooth.section.paired",
            ComponentPosition::Popup("bluetooth".to_string()),
        )
        .label("MY DEVICES")
        .label_props(|p| {
            p.color(CATPUCCIN_MOCHA.overlay1).font(Font {
                family: "JetBrainsMono Nerd Font".into(),
                style: FontStyle::Bold,
                size: 10.0,
            })
        })
        .padding_left(12)
        .width(320);

        paired_header.add()?;

        for device in paired {
            Self::render_single_device(device, true).await?;
        }

        // NEARBY DEVICES Header
        if !nearby.is_empty() {
            Self::render_nearby_header().await?;
            for device in nearby {
                Self::render_single_device(device, false).await?;
            }
        }

        Ok(())
    }

    async fn render_nearby_header() -> Result<()> {
        let nearby_header = BarItem::new_with_pos(
            "bluetooth.section.nearby",
            ComponentPosition::Popup("bluetooth".to_string()),
        )
        .label("NEARBY DEVICES")
        .label_props(|p| {
            p.color(CATPUCCIN_MOCHA.overlay1).font(Font {
                family: "JetBrainsMono Nerd Font".into(),
                style: FontStyle::Bold,
                size: 10.0,
            })
        })
        .padding_left(12)
        .width(320);

        api::add_item(&nearby_header)
    }

    async fn render_single_device(device: BluetoothDeviceData, is_paired: bool) -> Result<()> {
        let exe_path = std::env::current_exe()?.to_string_lossy().to_string();
        let sanitized_address = device.address.replace([':', '-'], "");
        let name = format!("bluetooth.device.{}", sanitized_address);

        // If it's nearby and the header doesn't exist, add it
        if !is_paired {
            let _ = Self::render_nearby_header().await;
        }

        let icon = if device.connected {
            "󰂱"
        } else {
            match device.device_type.as_deref() {
                Some("Headset") | Some("Headphones") => "󰋋",
                Some("Mouse") => "󰍽",
                Some("Keyboard") => "󰌌",
                Some("Trackpad") => "󰟡",
                _ => {
                    if device.is_nearby {
                        "󱗿"
                    } else {
                        "󰂯"
                    }
                }
            }
        };

        let icon_color = if device.connected {
            CATPUCCIN_MOCHA.blue
        } else if device.is_nearby {
            CATPUCCIN_MOCHA.yellow
        } else {
            CATPUCCIN_MOCHA.overlay0
        };

        let status_text = if device.connected {
            "Connected"
        } else if device.is_nearby {
            "Nearby"
        } else {
            "Not Connected"
        };

        let label_color = if device.connected {
            CATPUCCIN_MOCHA.text
        } else {
            CATPUCCIN_MOCHA.overlay2
        };

        let toggle_cmd = daemon_send_script(
            &exe_path,
            &DaemonCmd::ToggleBluetoothDevice {
                address: device.address.clone(),
            },
        );

        let click_script = format!(
            "sketchybar --animate sin 10 --set $NAME background.highlight=on; sleep 0.1; sketchybar --animate sin 10 --set $NAME background.highlight=off; {}",
            toggle_cmd
        );

        let item = BarItem::new_with_pos(&name, ComponentPosition::Popup("bluetooth".to_string()))
            .icon(icon)
            .icon_props(|p| p.color(icon_color))
            .label(&format!("{} | {}", device.name, status_text))
            .label_props(|p| p.color(label_color))
            .width(320)
            .background(|b| {
                b.color(CATPUCCIN_MOCHA.transparent)
                    .height(36)
                    .drawing(ToggleState::On)
                    .corner_radius(8)
            })
            .click_script(&click_script);

        api::add_item(&item)
    }

    pub async fn toggle_device(address: &str) -> Result<()> {
        // Immediate feedback: Find the item name and update its label
        let sanitized_address = address.replace([':', '-'], "");
        let item_name = format!("bluetooth.device.{}", sanitized_address);

        BarItem::new(&item_name)
            .label_props(|p| p.color(Argb::from_u32(0xfffab387)))
            .label("Processing...")
            .set()?;

        let swift_script = r#"
import IOBluetooth
let address = CommandLine.arguments[1]
guard let device = IOBluetoothDevice(addressString: address) else {
    exit(1)
}
if device.isConnected() { device.closeConnection() } else { device.openConnection() }
"#;

        let _ = Command::new("swift")
            .arg("-e")
            .arg(swift_script)
            .arg(address)
            .status();

        // Wait a bit for the connection to actually settle before refreshing
        tokio::time::sleep(Duration::from_millis(500)).await;
        Ok(())
    }
}

#[async_trait]
impl SketchybarItem for Bluetooth {
    async fn setup(&self, exe_path: &str) -> Result<()> {
        let scan_cmd =
            daemon_send_script(exe_path, &DaemonCmd::UpdateBluetoothPopup { scan: true });
        let item = BarItem::new_with_pos("bluetooth", ComponentPosition::Right)
            .update_freq(5)
            .script(&format!("{} --update-bluetooth", exe_path))
            .click_script(&format!(
                "sketchybar --animate sin 15 --set bluetooth popup.drawing=toggle; {}",
                scan_cmd
            ))
            .icon("")
            .icon_props(|p| p.color(CATPUCCIN_MOCHA.blue).drawing(ToggleState::On))
            .label_props(|p| p.drawing(ToggleState::Off))
            .background(|b| b.color(CATPUCCIN_MOCHA.surface0).drawing(ToggleState::On))
            .popup(|p| {
                p.align(PopupAlign::Center).background(|b| {
                    b.color(CATPUCCIN_MOCHA.base)
                        .border_color(CATPUCCIN_MOCHA.surface1)
                        .border_width(2)
                        .corner_radius(12)
                })
            });

        item.add()?;

        Self::update_popup(false).await?;
        Ok(())
    }

    async fn spawn_background_task(&self, mut bus: tokio::sync::broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            while let Ok(event) = bus.recv().await {
                match event {
                    Event::UpdateBluetooth { scan } => {
                        if let Err(e) = Self::update_popup(scan).await {
                            eprintln!("[bluetooth] popup update error: {e}");
                        }
                    }
                    Event::BluetoothAction(data) => {
                        if data.action == "toggle"
                            && let Some(address) = data.address
                        {
                            if let Err(e) = Self::toggle_device(&address).await {
                                eprintln!("[bluetooth] toggle error: {e}");
                            } else {
                                let _ = Self::update().await;
                                let _ = Self::update_popup(false).await;
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
