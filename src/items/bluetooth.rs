use crate::api;
use crate::api::item::{Argb, BarItem, ComponentPosition, ItemBuilder, PopupAlign, ToggleState};
use crate::api::types::{Font, FontStyle};
use crate::daemon::DaemonCmd;
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::{Result, anyhow};
use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub name: String,
    pub address: String,
    pub connected: bool,
    pub is_nearby: bool,
    pub device_type: Option<String>,
}

pub async fn fetch_status() -> Result<(bool, Vec<BluetoothDevice>)> {
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
                                    devices.push(BluetoothDevice {
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
    let (is_on, devices) = fetch_status().await?;
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
        .label_drawing(ToggleState::Off)
        .apply_if(connected_count > 0, |item| {
            item.label(&format!("{}", connected_count))
                .label_drawing(ToggleState::On)
        });

    item.set()?;
    Ok(())
}

pub async fn update_popup(scan: bool) -> Result<()> {
    let (_, devices) = fetch_status().await?;
    render_device_list(devices, Vec::new()).await?;

    if scan {
        // Start continuous background scan
        tokio::spawn(async move {
            let _ = continuous_scan().await;
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
    let loading = BarItem::new("bluetooth.loading")
        .icon("󰑐")
        .label("Searching for nearby devices...")
        .icon_color(CATPUCCIN_MOCHA.yellow)
        .label_color(CATPUCCIN_MOCHA.subtext0)
        .label_font(Font {
            family: "JetBrainsMono Nerd Font".to_string(),
            style: FontStyle::Italic,
            size: 11.0,
        })
        .width(320);

    api::add_special_item("item", "bluetooth.loading", "popup.bluetooth", &loading)?;
    let _ = Command::new("sketchybar")
        .args(["--set", "bluetooth.loading", "before=/.*/"])
        .status();

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

                        if !nearby_discovered.iter().any(|d: &BluetoothDevice| d.address == address) {
                            let device = BluetoothDevice {
                                name,
                                address,
                                connected: false,
                                is_nearby: true,
                                device_type: None,
                            };
                            nearby_discovered.push(device.clone());
                            render_single_device(device, false).await?;
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
    paired: Vec<BluetoothDevice>,
    nearby: Vec<BluetoothDevice>,
) -> Result<()> {
    let _ = Command::new("sketchybar")
        .args([
            "--remove",
            "/bluetooth\\.device\\..*/",
            "bluetooth.section.*",
        ])
        .status();

    // MY DEVICES Header
    let paired_header = BarItem::new("bluetooth.section.paired")
        .label("MY DEVICES")
        .label_color(CATPUCCIN_MOCHA.overlay1)
        .label_font(Font {
            family: "JetBrainsMono Nerd Font".into(),
            style: FontStyle::Bold,
            size: 10.0,
        })
        .padding_left(12)
        .width(320);

    api::add_special_item(
        "item",
        "bluetooth.section.paired",
        "popup.bluetooth",
        &paired_header,
    )?;

    for device in paired {
        render_single_device(device, true).await?;
    }

    // NEARBY DEVICES Header
    if !nearby.is_empty() {
        render_nearby_header().await?;
        for device in nearby {
            render_single_device(device, false).await?;
        }
    }

    Ok(())
}

async fn render_nearby_header() -> Result<()> {
    let nearby_header = BarItem::new("bluetooth.section.nearby")
        .label("NEARBY DEVICES")
        .label_color(CATPUCCIN_MOCHA.overlay1)
        .label_font(Font {
            family: "JetBrainsMono Nerd Font".into(),
            style: FontStyle::Bold,
            size: 10.0,
        })
        .padding_left(12)
        .width(320);

    api::add_special_item(
        "item",
        "bluetooth.section.nearby",
        "popup.bluetooth",
        &nearby_header,
    )?;
    Ok(())
}

async fn render_single_device(device: BluetoothDevice, is_paired: bool) -> Result<()> {
    let exe_path = std::env::current_exe()?.to_string_lossy().to_string();
    let sanitized_address = device.address.replace([':', '-'], "");
    let name = format!("bluetooth.device.{}", sanitized_address);

    // If it's nearby and the header doesn't exist, add it
    if !is_paired {
        let _ = render_nearby_header().await;
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
        "sketchybar --set $NAME background.highlight=on; sleep 0.1; sketchybar --set $NAME background.highlight=off; {}",
        toggle_cmd
    );

    let item = BarItem::new(&name)
        .icon(icon)
        .icon_color(icon_color)
        .label(&format!("{} | {}", device.name, status_text))
        .label_color(label_color)
        .width(320)
        .background_color(CATPUCCIN_MOCHA.transparent)
        .background_height(36)
        .background_drawing(ToggleState::On)
        .background_corner_radius(8)
        .click_script(&click_script);

    api::add_special_item("item", &name, "popup.bluetooth", &item)?;

    Ok(())
}

pub async fn toggle_device(address: &str) -> Result<()> {
    // Immediate feedback: Find the item name and update its label
    let sanitized_address = address.replace([':', '-'], "");
    let item_name = format!("bluetooth.device.{}", sanitized_address);

    BarItem::new(&item_name)
        .label_color(Argb::from_u32(0xfffab387))
        .label("Processing...")
        .set()?;

    let swift_script = format!(
        r#"
import IOBluetooth
let address = "{}"
guard let device = IOBluetoothDevice(addressString: address) else {{
    exit(1) 
}}
if device.isConnected() {{ device.closeConnection() }} else {{ device.openConnection() }}
"#,
        address
    );

    let _ = Command::new("swift").arg("-e").arg(swift_script).status();

    // Wait a bit for the connection to actually settle before refreshing
    tokio::time::sleep(Duration::from_millis(500)).await;
    Ok(())
}

/// Build a shell command that sends a daemon command via `--send`.
fn daemon_send_script(exe_path: &str, cmd: &DaemonCmd) -> String {
    let json = serde_json::to_string(cmd).unwrap_or_default();
    // Escape single quotes for shell: ' → '\''
    let safe_json = json.replace('\'', "'\\''");
    format!("{} --send '{}'", exe_path, safe_json)
}

pub async fn setup(exe_path: &str) -> Result<()> {
    let scan_cmd = daemon_send_script(exe_path, &DaemonCmd::UpdateBluetoothPopup { scan: true });
    let item = BarItem::new("bluetooth")
        .position(ComponentPosition::Right)
        .update_freq(5)
        .script(&format!("{} --update-bluetooth", exe_path))
        .click_script(&format!(
            "sketchybar --set bluetooth popup.drawing=toggle; {}",
            scan_cmd
        ))
        .icon("")
        .icon_color(CATPUCCIN_MOCHA.blue)
        .label_drawing(ToggleState::Off)
        .background_color(CATPUCCIN_MOCHA.surface0)
        .background_drawing(ToggleState::On)
        .popup_align(PopupAlign::Center)
        .popup_background_color(CATPUCCIN_MOCHA.base)
        .popup_background_border_color(CATPUCCIN_MOCHA.surface1)
        .popup_background_border_width(2)
        .popup_background_corner_radius(12);

    item.add()?;

    update_popup(false).await?;
    Ok(())
}
