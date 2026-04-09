use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;

pub fn update() -> Result<()> {
    let output = std::process::Command::new("aerospace")
        .args(["list-workspaces", "--focused"])
        .output()?;
    let active_workspace = String::from_utf8_lossy(&output.stdout).trim().to_string();

    for i in 1..=9 {
        let ws_name = format!("workspace.{}", i);

        if active_workspace == i.to_string() {
            api::set_args(
                &ws_name,
                &[
                    &format!("background.color={}", CATPUCCIN_MOCHA.mauve),
                    "icon.color=0xff11111b", // Crust
                ],
            )?;
        } else {
            api::set_args(
                &ws_name,
                &[
                    &format!("background.color={}", CATPUCCIN_MOCHA.surface1),
                    "icon.color=0xffcdd6f4", // Text
                ],
            )?;
        }
    }
    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    api::add_event("aerospace_workspace_change")?;

    for i in 1..=9 {
        let ws_name = format!("workspace.{}", i);
        let mut ws_item = BarItem::new(ws_name.clone(), ComponentPosition::Left);
        ws_item.icon.icon = Some(i.to_string());

        let mut bg = BackgroundProps::new();
        bg.color = Some(CATPUCCIN_MOCHA.surface1.clone());
        bg.corner_radius = Some(6);
        bg.drawing = Some(true); // Initially all look same until update script runs
        ws_item.geometry.background = Some(bg);

        let mut label_props = Text::default();
        label_props.drawing = Some(false);
        ws_item.label.props = Some(label_props);

        ws_item.scripting.click_script = Some(ScriptType::String(format!("aerospace workspace {}", i)));

        if i == 1 {
            ws_item.scripting.script = Some(ScriptType::String(format!("{} --update-workspaces", exe_path)));
        }

        api::add_item(&ws_item)?;
        if i == 1 {
            api::subscribe(&ws_name, "aerospace_workspace_change")?;
        }
    }

    Ok(())
}
