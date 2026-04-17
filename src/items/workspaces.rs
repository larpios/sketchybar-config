use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;

pub fn update() -> Result<()> {
    let ws_id = match std::env::var("FOCUSED_WORKSPACE") {
        Ok(v) => v,
        _ => {
            let output = std::process::Command::new("aerospace")
                .args(["list-workspaces", "--focused"])
                .output()?;
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
    }
    .trim_start_matches("workspace.")
    .to_string();

    for i in 1..=9 {
        let ws_name = format!("workspace.{}", i);
        let is_active = i == ws_id.parse().unwrap_or(0);

        let active_bg = CATPUCCIN_MOCHA.mauve.clone();
        let inactive_bg = CATPUCCIN_MOCHA.surface1.clone();
        let active_icon = CATPUCCIN_MOCHA.crust.clone();
        let inactive_icon = CATPUCCIN_MOCHA.text.clone();

        let args = &[
            format!(
                "background.color={}",
                if is_active { active_bg } else { inactive_bg }
            ),
            format!(
                "icon.color={}",
                if is_active {
                    active_icon
                } else {
                    inactive_icon
                }
            ),
        ];
        api::set_args(&ws_name, args)?;
    }

    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    let mut ws_items = (1..=9)
        .map(|i| {
            let ws_name = format!("workspace.{}", i);
            let mut ws_item = BarItem::new(ws_name.clone(), ComponentPosition::Left);
            ws_item.props.icon.icon = Some(i.to_string());

            let bg = BackgroundProps {
                color: Some(CATPUCCIN_MOCHA.surface1.clone()),
                corner_radius: Some(6),
                drawing: Some(true),
                ..Default::default()
            };
            ws_item.props.geometry.background = Some(bg);

            let label_props = Text {
                drawing: Some(false),
                ..Default::default()
            };

            ws_item.props.label.props = Some(label_props);

            ws_item.props.scripting.click_script =
                Some(ScriptType::String(format!("aerospace workspace {}", i)));

            ws_item
        })
        .collect::<Vec<BarItem>>();

    for item in ws_items.iter_mut() {
        item.props.scripting.click_script = Some(ScriptType::String(format!(
            "aerospace workspace {} && {} --update-workspaces",
            item.name.trim_start_matches("workspace."),
            exe_path
        )));
    }
    ws_items[0].props.scripting.script = Some(ScriptType::String(format!(
        "{} --update-workspaces",
        exe_path
    )));

    let first_ws_name = ws_items[0].name.clone();

    for item in ws_items {
        api::add_item(&item)?;
    }

    api::add_event("aerospace_workspace_change")?;
    api::add_event("workspace_change")?;
    api::subscribe(
        &first_ws_name,
        [
            "aerospace_workspace_change",
            "space_change",
            "workspace_change",
        ],
    )?;

    Ok(())
}
