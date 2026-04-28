use crate::events::Event;
use crate::items::SketchybarItem;
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct WeatherData {
    pub temp: String,
}

pub struct Weather;

impl Weather {
    pub fn update_command() -> Result<()> {
        let data = Self::fetch()?;
        Self::update_items(&data)
    }
    pub fn fetch() -> anyhow::Result<WeatherData> {
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

        Ok(WeatherData { temp })
    }

    pub fn update_items(data: &WeatherData) -> anyhow::Result<()> {
        use crate::api::item::{BarItem, ItemBuilder};

        BarItem::new("weather").label(&data.temp).set()?;

        Ok(())
    }
}

#[async_trait]
impl SketchybarItem for Weather {
    async fn setup(&self, exe_path: &str) -> Result<()> {
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

    async fn spawn_background_task(&self, mut bus: tokio::sync::broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            while let Ok(event) = bus.recv().await {
                if matches!(event, Event::UpdateWeather)
                    && let Err(e) = Self::update_command()
                {
                    eprintln!("[weather] update error: {e}");
                }
            }
        });
    }
}
