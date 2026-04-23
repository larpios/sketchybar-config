use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, ToggleState};
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;

pub fn update_command() -> Result<()> {
    update()
}

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

        BarItem::new(&ws_name)
            .background_color(if is_active { active_bg } else { inactive_bg })
            .icon_color(if is_active {
                active_icon
            } else {
                inactive_icon
            })
            .set()?;
    }

    Ok(())
}

pub fn setup(exe_path: &str) -> Result<()> {
    for i in 1..=9 {
        let ws_name = format!("workspace.{}", i);
        let mut item = BarItem::new(&ws_name)
            .position(ComponentPosition::Left)
            .icon(&i.to_string())
            .background_color(CATPUCCIN_MOCHA.surface1.clone())
            .background_corner_radius(6)
            .background_drawing(ToggleState::On)
            .label_drawing(ToggleState::Off)
            .click_script(&format!(
                "aerospace workspace {} && {} --update-workspaces",
                i, exe_path
            ));

        if i == 1 {
            item = item.script(&format!("{} --update-workspaces", exe_path));
        }

        item.add()?;
    }

    use crate::api::event::BarEvent;

    crate::api::add_event("aerospace_workspace_change")?;
    crate::api::add_event("workspace_change")?;

    BarItem::new("workspace.1").subscribe([
        BarEvent::from("aerospace_workspace_change"),
        BarEvent::SpaceChange,
        BarEvent::from("workspace_change"),
    ])?;

    Ok(())
}
