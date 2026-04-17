use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use std::process::Command;

pub fn update() -> Result<()> {
    let output = Command::new("defaults")
        .args([
            "read",
            "/Library/Preferences/com.apple.Bluetooth",
            "ControllerPowerState",
        ])
        .output()?;

    let is_on = String::from_utf8_lossy(&output.stdout).trim() == "1";
    let icon = if is_on { "" } else { "󰂲" };

    api::set_args("bluetooth", [&format!("icon={}", icon)])?;
    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    let mut item = BarItem::new("bluetooth".to_string(), ComponentPosition::Right);
    item.props.scripting.update_freq = 5;
    item.props.scripting.script = Some(ScriptType::String(format!(
        "{} --update-bluetooth",
        exe_path
    )));
    item.props.icon.icon = Some("".to_string());

    let icon_props = Text {
        color: Some(CATPUCCIN_MOCHA.blue.clone()),
        ..Default::default()
    };

    item.props.icon.props = Some(icon_props);

    let bg = BackgroundProps {
        color: Some(CATPUCCIN_MOCHA.surface0.clone()),
        drawing: Some(true),
        ..Default::default()
    };
    item.props.geometry.background = Some(bg);

    api::add_item(&item)?;
    Ok(())
}
