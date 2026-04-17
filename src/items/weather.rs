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
        let temp = match ureq::get("https://wttr.in/?format=%t").call() {
            Ok(response) => {
                let text = response
                    .into_string()
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                if text.is_empty()
                    || text.to_lowercase().contains("unknown")
                    || text.to_lowercase().contains("err")
                {
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
        item.props.scripting.update_freq = 1800;
        item.props.scripting.script =
            Some(ScriptType::String(format!("{} --update-weather", exe_path)));
        item.props.scripting.click_script =
            Some(ScriptType::String("open 'https://weather.com'".to_string()));

        item.props.icon.icon = Some("".to_string());

        let icon_props = Text {
            color: Some(CATPUCCIN_MOCHA.yellow.clone()),
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
}
