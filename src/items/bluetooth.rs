use crate::api;
use crate::props::item::{
    BackgroundProps, BarItem, ComponentPosition, PopupAlign, PopupProperties, ScriptType, Text,
};
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

pub async fn scan_nearby() -> Result<Vec<BluetoothDevice>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("No bluetooth adapter found"))?;

    central.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_secs(10)).await;

    let peripherals = central.peripherals().await?;
    let mut nearby = Vec::new();

    for p in peripherals {
        let properties = p.properties().await?;
        if let Some(props) = properties {
            let name = props
                .local_name
                .unwrap_or_else(|| "Unknown Device".to_string());
            let address = props.address.to_string();

            nearby.push(BluetoothDevice {
                name,
                address,
                connected: p.is_connected().await.unwrap_or(false),
                is_nearby: true,
                device_type: None,
            });
        }
    }

    let _ = central.stop_scan().await;
    Ok(nearby)
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

    let mut args = vec![format!("icon={}", icon)];
    if connected_count > 0 {
        args.push("icon.highlight=on".to_string());
        args.push(format!("label={}", connected_count));
        args.push("label.drawing=on".to_string());
    } else {
        args.push("icon.highlight=off".to_string());
        args.push("label.drawing=off".to_string());
    }

    api::set_args("bluetooth", args)?;
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
    let mut loading = BarItem::new("bluetooth.loading".to_string(), ComponentPosition::Right);
    loading.props.geometry.position = None;
    loading.props.icon.icon = Some("󰑐".to_string());
    loading.props.label.label = Some("Searching for nearby devices...".to_string());
    loading.props.icon.props = Some(Text {
        color: Some(CATPUCCIN_MOCHA.yellow.clone()),
        padding_left: Some(12),
        ..Default::default()
    });

    let font = crate::props::types::Font {
        family: "JetBrainsMono Nerd Font".to_string(),
        type_: crate::props::types::FontType::Italic,
        size: 11.0,
    };
    loading.props.label.props = Some(Text {
        color: Some(CATPUCCIN_MOCHA.subtext0.clone()),
        font: Some(font),
        ..Default::default()
    });
    loading.props.geometry.width = Some(crate::props::item::WidthMode::Value(320));
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
                let _ = Command::new("sketchybar")
                    .args(["--set", "bluetooth.loading", &format!("icon={}", spinners[spinner_idx])])
                    .status();
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
    let mut paired_header = BarItem::new("".into(), ComponentPosition::Right);
    paired_header.props.geometry.position = None;
    api::add_special_item(
        "item",
        "bluetooth.section.paired",
        "popup.bluetooth",
        &paired_header,
    )?;
    api::set_args(
        "bluetooth.section.paired",
        [
            "label=MY DEVICES",
            "icon.drawing=off",
            &format!("label.color={}", CATPUCCIN_MOCHA.overlay1),
            "label.font=JetBrainsMono Nerd Font:Bold:10.0",
            "padding_left=12",
            "width=320",
        ],
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
    let mut nearby_header = BarItem::new("".into(), ComponentPosition::Right);
    nearby_header.props.geometry.position = None;
    api::add_special_item(
        "item",
        "bluetooth.section.nearby",
        "popup.bluetooth",
        &nearby_header,
    )?;
    api::set_args(
        "bluetooth.section.nearby",
        [
            "label=NEARBY DEVICES",
            "icon.drawing=off",
            &format!("label.color={}", CATPUCCIN_MOCHA.overlay1),
            "label.font=JetBrainsMono Nerd Font:Bold:10.0",
            "padding_left=12",
            "padding_top=10",
            "width=320",
        ],
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

    let mut item = BarItem::new(name.clone(), ComponentPosition::Right);
    item.props.geometry.position = None;

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

    item.props.icon.icon = Some(icon.to_string());

    let icon_color = if device.connected {
        CATPUCCIN_MOCHA.blue.clone()
    } else if device.is_nearby {
        CATPUCCIN_MOCHA.yellow.clone()
    } else {
        CATPUCCIN_MOCHA.overlay0.clone()
    };

    item.props.icon.props = Some(Text {
        color: Some(icon_color),
        padding_left: Some(12),
        ..Default::default()
    });

    let status_text = if device.connected {
        "Connected"
    } else if device.is_nearby {
        "Nearby"
    } else {
        "Not Connected"
    };

    let label_color = if device.connected {
        CATPUCCIN_MOCHA.text.clone()
    } else {
        CATPUCCIN_MOCHA.overlay2.clone()
    };

    item.props.label.props = Some(Text {
        color: Some(label_color),
        padding_right: Some(12),
        width: Some(crate::props::item::WidthMode::Value(260)),
        ..Default::default()
    });

    item.props.label.label = Some(format!("{} | {}", device.name, status_text));

    item.props.geometry.width = Some(crate::props::item::WidthMode::Value(320));
    item.props.geometry.background = Some(BackgroundProps {
        color: Some(CATPUCCIN_MOCHA.transparent.clone()),
        height: Some(36),
        ..Default::default()
    });

    item.props.scripting.click_script = Some(ScriptType::String(format!(
        "{} --toggle-bluetooth-device {}",
        exe_path, device.address
    )));

    api::add_special_item("item", &name, "popup.bluetooth", &item)?;

    let click_script = format!(
        "sketchybar --set $NAME background.highlight=on; sleep 0.1; sketchybar --set $NAME background.highlight=off; {}",
        item.props.scripting.click_script.unwrap()
    );

    api::set_args(
        &name,
        [
            &format!("background.highlight_color={}", CATPUCCIN_MOCHA.surface0),
            &format!("click_script={}", click_script),
            "background.corner_radius=8",
            "background.drawing=on",
        ],
    )?;

    Ok(())
}

pub async fn toggle_device(address: &str) -> Result<()> {
    // Immediate feedback: Find the item name and update its label
    let sanitized_address = address.replace([':', '-'], "");
    let item_name = format!("bluetooth.device.{}", sanitized_address);

    let _ = Command::new("sketchybar")
        .args([
            "--set",
            &item_name,
            "label.color=0xfffab387",
            "label=Processing...",
        ])
        .status();

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
    update_popup(false).await?;
    Ok(())
}

pub async fn setup(exe_path: &str) -> Result<()> {
    let mut item = BarItem::new("bluetooth".to_string(), ComponentPosition::Right);
    item.props.scripting.update_freq = 5;
    item.props.scripting.script = Some(ScriptType::String(format!(
        "{} --update-bluetooth",
        exe_path
    )));
    item.props.scripting.click_script = Some(ScriptType::String(format!(
        "sketchybar --set bluetooth popup.drawing=toggle; {} --scan-bluetooth",
        exe_path
    )));
    item.props.icon.icon = Some("".to_string());
    item.props.icon.props = Some(Text {
        color: Some(CATPUCCIN_MOCHA.blue.clone()),
        ..Default::default()
    });
    item.props.label.props = Some(Text {
        drawing: Some(false),
        ..Default::default()
    });
    item.props.geometry.background = Some(BackgroundProps {
        color: Some(CATPUCCIN_MOCHA.surface0.clone()),
        drawing: Some(true),
        ..Default::default()
    });

    let popup_bg = BackgroundProps {
        color: Some(CATPUCCIN_MOCHA.base.clone()),
        border_color: Some(CATPUCCIN_MOCHA.surface1.clone()),
        border_width: Some(2),
        corner_radius: Some(12),
        ..Default::default()
    };
    item.props.popup = Some(PopupProperties {
        align: PopupAlign::Center,
        background: Some(popup_bg),
        ..Default::default()
    });

    api::add_item(&item)?;

    update_popup(false).await?;
    Ok(())
}
