use crate::items::Item;
use anyhow::Result;
use chrono::{Local, Utc};

const ICON: &str = "";

#[derive(Debug, Clone)]
pub struct Clock {
    pub icon: String,
    pub time: String,
    pub full_date: String,
    pub utc_time: String,
}

impl Item for Clock {
    fn fetch() -> Result<Self> {
        Clock::fetch()
    }

    fn update_items(&self) -> Result<()> {
        Clock::update_items(self)
    }

    fn setup(exe_path: &str) -> Result<()> {
        Clock::setup(exe_path)
    }
}

impl Clock {
    pub fn update_command() -> Result<()> {
        let data = Self::fetch()?;
        Self::update_items(&data)
    }

    pub fn fetch() -> anyhow::Result<Self> {
        let now_local = Local::now();
        let now_utc = Utc::now();

        let time = now_local.format("%a %d %b %H:%M").to_string();
        let full_date = now_local.format("%A, %d %b %Y").to_string();
        let utc_time = now_utc.format("%H:%M").to_string();

        Ok(Self {
            icon: ICON.to_string(),
            time,
            full_date,
            utc_time,
        })
    }

    pub fn setup(exe_path: &str) -> anyhow::Result<()> {
        use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, PopupAlign, ToggleState};
        use crate::themes::CATPUCCIN_MOCHA;

        let item = BarItem::new("clock")
            .position(ComponentPosition::Right)
            .update_freq(10)
            .script(&format!("{} --update-clock", exe_path))
            .icon("󰥔")
            .icon_color(CATPUCCIN_MOCHA.blue)
            .background_color(CATPUCCIN_MOCHA.surface0)
            .background_drawing(ToggleState::On)
            .popup_align(PopupAlign::Center)
            .popup_background_color(CATPUCCIN_MOCHA.base)
            .popup_background_corner_radius(8)
            .popup_background_border_width(2)
            .popup_background_border_color(CATPUCCIN_MOCHA.surface1)
            .click_script("sketchybar --set clock popup.drawing=toggle")
            .add_item(BarItem::new("clock.date").icon("Date:").label("Loading..."))
            .add_item(BarItem::new("clock.utc").icon("UTC:").label("Loading..."));

        item.add()?;

        // Initial update
        let data = Self::fetch()?;
        Self::update_items(&data)?;

        Ok(())
    }

    pub fn update_items(data: &Self) -> anyhow::Result<()> {
        use crate::api::item::{BarItem, ItemBuilder};

        BarItem::new("clock").label(&data.time).set()?;
        BarItem::new("clock.date").label(&data.full_date).set()?;
        BarItem::new("clock.utc").label(&data.utc_time).set()?;

        Ok(())
    }
}
