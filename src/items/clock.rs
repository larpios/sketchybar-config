use crate::items::SketchybarItem;
use crate::{api::item::ItemBuilder, events::Event};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Local, Utc};

const ICON: &str = "";

#[derive(Debug, Clone)]
pub struct ClockData {
    pub icon: String,
    pub time: String,
    pub full_date: String,
    pub utc_time: String,
}

pub struct Clock;

impl Clock {
    pub fn update_command() -> Result<()> {
        let data = Self::fetch()?;
        Self::update_items(&data)
    }

    pub fn fetch() -> anyhow::Result<ClockData> {
        let now_local = Local::now();
        let now_utc = Utc::now();

        let time = now_local.format("%a %d %b %H:%M").to_string();
        let full_date = now_local.format("%A, %d %b %Y").to_string();
        let utc_time = now_utc.format("%H:%M").to_string();

        Ok(ClockData {
            icon: ICON.to_string(),
            time,
            full_date,
            utc_time,
        })
    }

    pub fn update_items(data: &ClockData) -> anyhow::Result<()> {
        use crate::api::item::BarItem;

        BarItem::new("clock").label(&data.time).set()?;
        BarItem::new("clock.date").label(&data.full_date).set()?;
        BarItem::new("clock.utc").label(&data.utc_time).set()?;

        Ok(())
    }
}

#[async_trait]
impl SketchybarItem for Clock {
    async fn setup(&self, exe_path: &str) -> Result<()> {
        use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, PopupAlign, ToggleState};
        use crate::children;
        use crate::themes::CATPUCCIN_MOCHA;

        let item = BarItem::new_with_pos("clock", ComponentPosition::Right)
            .update_freq(10)
            .script(&format!("{} --update-clock", exe_path))
            .icon("󰥔")
            .icon_props(|p| p.color(CATPUCCIN_MOCHA.blue))
            .background(|b| b.color(CATPUCCIN_MOCHA.surface0).drawing(ToggleState::On))
            .popup(|p| {
                p.align(PopupAlign::Right).background(|b| {
                    b.color(CATPUCCIN_MOCHA.base)
                        .corner_radius(8)
                        .border_width(2)
                        .border_color(CATPUCCIN_MOCHA.surface1)
                })
            })
            .click_script("sketchybar --animate sin 15 --set clock popup.drawing=toggle")
            .with_children(children![
                BarItem::new("clock.date").icon("Date:").label("Loading..."),
                BarItem::new("clock.utc").icon("UTC:").label("Loading..."),
            ]);

        item.add()?;

        let data = Self::fetch()?;
        Self::update_items(&data)?;

        Ok(())
    }

    async fn spawn_background_task(&self, mut bus: tokio::sync::broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            while let Ok(event) = bus.recv().await {
                if matches!(event, Event::UpdateClock)
                    && let Err(e) = Self::update_command()
                {
                    eprintln!("[clock] update error: {e}");
                }
            }
        });
    }
}
