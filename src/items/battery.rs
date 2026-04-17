use crate::property;
use crate::props::types::{Property, ToSketchybarArgs};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Battery {
    pub percentage: u8,
    pub icon: String,
}

impl ToSketchybarArgs for Battery {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let label = format!("{}%", self.percentage);

        vec![
            property!("icon", &self.icon),
            property!("label", &label),
            property!("label.drawing", "on"),
            property!("icon.drawing", "on"),
        ]
    }
}

impl Battery {
    pub fn fetch() -> anyhow::Result<Self> {
        let output = Command::new("pmset").args(["-g", "batt"]).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Example output:
        // Now drawing from 'Battery Power'
        // -InternalBattery-0 (id=8847458)        85%; discharging; 6:49 remaining present: true

        let percentage = stdout
            .lines()
            .nth(1)
            .and_then(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() > 1 {
                    let status = parts[1];
                    let pct_str = status.split('%').next().unwrap_or("0");
                    pct_str.trim().parse::<u8>().ok()
                } else {
                    None
                }
            })
            .unwrap_or(0);

        let icon = if percentage > 80 {
            ""
        } else if percentage > 50 {
            ""
        } else if percentage > 30 {
            ""
        } else if percentage > 10 {
            ""
        } else {
            ""
        }
        .to_string();

        Ok(Self { percentage, icon })
    }

    pub fn setup(exe_path: &str) -> anyhow::Result<()> {
        use crate::api;
        use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType};
        use crate::themes::CATPUCCIN_MOCHA;

        let mut item = BarItem::new("battery".to_string(), ComponentPosition::Right);
        item.props.scripting.update_freq = 60;
        item.props.scripting.script =
            Some(ScriptType::String(format!("{} --update-battery", exe_path)));
        let bg = BackgroundProps {
            color: Some(CATPUCCIN_MOCHA.surface0.clone()),
            drawing: Some(true),
            ..Default::default()
        };
        item.props.geometry.background = Some(bg);
        api::add_item(&item)?;
        Ok(())
    }
}
