use anyhow::Result;
use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
use crate::themes::CATPUCCIN_MOCHA;
use std::process::Command;

pub fn update() -> Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("defaults read /Library/Preferences/com.apple.Bluetooth ControllerPowerState")
        .output()?;
        
    let is_on = String::from_utf8_lossy(&output.stdout).trim() == "1";
    let icon = if is_on { "" } else { "󰂲" };
    
    api::set_args("bluetooth", &[
        &format!("icon={}", icon),
    ])?;
    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    let mut item = BarItem::new("bluetooth".to_string(), ComponentPosition::Right);
    item.scripting.update_freq = 5;
    item.scripting.script = Some(ScriptType::String(format!("{} --update-bluetooth", exe_path)));
    item.icon.icon = Some("".to_string());
    
    let mut icon_props = Text::default();
    icon_props.color = Some(CATPUCCIN_MOCHA.blue.clone());
    item.icon.props = Some(icon_props);

    let mut bg = BackgroundProps::new();
    bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
    bg.drawing = Some(true);
    item.geometry.background = Some(bg);

    api::add_item(&item)?;
    Ok(())
}
