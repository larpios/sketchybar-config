use crate::property;
use crate::props::types::{Property, ToSketchybarArgs};

#[derive(Debug, Clone)]
pub struct Weather {
    pub temp: String,
}

impl ToSketchybarArgs for Weather {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        vec![property!("label", &self.temp)]
    }
}

impl Weather {
    pub fn fetch() -> anyhow::Result<Self> {
        let output = std::process::Command::new("curl")
            .args(["-s", "wttr.in/?format=%t"])
            .output();

        let temp = match output {
            Ok(output) if output.status.success() => {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if text.is_empty() || text.to_lowercase().contains("unknown") || text.to_lowercase().contains("err") {
                    "N/A".to_string()
                } else {
                    text
                }
            }
            _ => "N/A".to_string(),
        };

        Ok(Self { temp })
    }

    pub fn setup(exe_path: &str) -> anyhow::Result<()> {
        use crate::api;
        use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
        use crate::themes::CATPUCCIN_MOCHA;

        let mut item = BarItem::new("weather".to_string(), ComponentPosition::Right);
        item.scripting.update_freq = 1800;
        item.scripting.script = Some(ScriptType::String(format!("{} --update-weather", exe_path)));
        item.scripting.click_script = Some(ScriptType::String("open 'https://weather.com'".to_string()));

        item.icon.icon = Some("".to_string());
        
        let mut icon_props = Text::default();
        icon_props.color = Some(CATPUCCIN_MOCHA.yellow.clone());
        item.icon.props = Some(icon_props);

        let mut bg = BackgroundProps::new();
        bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
        bg.drawing = Some(true);
        item.geometry.background = Some(bg);

        api::add_item(&item)?;
        Ok(())
    }
}
