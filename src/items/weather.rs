use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Weather {
    pub temp: String,
}

impl Weather {
    pub fn update_command() -> Result<()> {
        let data = Self::fetch()?;
        Self::update_items(&data)
    }
    pub fn fetch() -> anyhow::Result<Self> {
        let temp = match ureq::get("https://wttr.in/?format=%t").call() {
            Ok(response) => {
                let text = response
                    .into_body()
                    .read_to_string()
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
        use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, ToggleState};
        use crate::themes::CATPUCCIN_MOCHA;

        let item = BarItem::new("weather")
            .position(ComponentPosition::Right)
            .update_freq(300)
            .script(&format!("{} --update-weather", exe_path))
            .click_script("open 'https://weather.com'")
            .icon("")
            .icon_color(CATPUCCIN_MOCHA.yellow)
            .background_color(CATPUCCIN_MOCHA.surface0)
            .background_drawing(ToggleState::On);

        item.add()?;

        // Initial update
        let data = Self::fetch()?;
        Self::update_items(&data)?;

        Ok(())
    }

    pub fn update_items(data: &Self) -> anyhow::Result<()> {
        use crate::api::item::{BarItem, ItemBuilder};

        BarItem::new("weather").label(&data.temp).set()?;

        Ok(())
    }
}
