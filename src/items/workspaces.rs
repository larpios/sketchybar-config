use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;

pub fn update() -> Result<()> {
    let next_ws_id = std::env::var("FOCUSED_WORKSPACE")?;

    let output = std::process::Command::new("aerospace")
        .args(["list-workspaces", "--focused"])
        .output()?;
    let active_ws = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let prev_ws = format!("workspace.{}", active_ws);
    let next_ws = format!("workspace.{}", next_ws_id);

    let active_flag = &[
        format!("background.color={}", CATPUCCIN_MOCHA.mauve),
        format!("icon.color={}", CATPUCCIN_MOCHA.crust),
    ];
    let inactive_flag = &[
        format!("background.color={}", CATPUCCIN_MOCHA.surface1),
        format!("icon.color={}", CATPUCCIN_MOCHA.text),
    ];

    api::set_args(&next_ws, active_flag)?;
    api::set_args(&prev_ws, inactive_flag)?;

    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    let mut ws_items = (1..=9)
        .map(|i| {
            let ws_name = format!("workspace.{}", i);
            let mut ws_item = BarItem::new(ws_name.clone(), ComponentPosition::Left);
            ws_item.icon.icon = Some(i.to_string());

            let mut bg = BackgroundProps::new();
            bg.color = Some(CATPUCCIN_MOCHA.surface1.clone());
            bg.corner_radius = Some(6);
            bg.drawing = Some(true); // Initially all look same until update script runs
            ws_item.geometry.background = Some(bg);

            let label_props = Text {
                drawing: Some(false),
                ..Default::default()
            };

            ws_item.label.props = Some(label_props);

            ws_item.scripting.click_script =
                Some(ScriptType::String(format!("aerospace workspace {}", i)));

            ws_item
        })
        .collect::<Vec<BarItem>>();

    ws_items[0].scripting.script = Some(ScriptType::String(format!(
        "{} --update-workspaces",
        exe_path
    )));
    api::subscribe(&ws_items[0].name, &["aerospace_workspace_change"])?;
    for item in ws_items {
        api::add_item(&item)?;
    }

    Ok(())
}
