use crate::property;
use crate::props::types::{Property, ToSketchybarArgs};
use chrono::{Local, Utc};

const ICON: &str = "";

#[derive(Debug, Clone)]
pub struct Clock {
    pub icon: String,
    pub time: String,
    pub full_date: String,
    pub utc_time: String,
}

impl ToSketchybarArgs for Clock {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        vec![property!("label", &self.time)]
    }
}

impl Clock {
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
        use crate::api;
        use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
        use crate::themes::CATPUCCIN_MOCHA;

        let mut item = BarItem::new("clock".to_string(), ComponentPosition::Right);
        item.scripting.update_freq = 10;
        item.scripting.script = Some(ScriptType::String(format!("{} --update-clock", exe_path)));
        item.icon.icon = Some("󰥔".to_string());
        
        let mut icon_props = Text::default();
        icon_props.color = Some(CATPUCCIN_MOCHA.blue.clone());
        item.icon.props = Some(icon_props);

        let mut bg = BackgroundProps::new();
        bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
        bg.drawing = Some(true);
        item.geometry.background = Some(bg);

        let mut popup_props = crate::props::item::PopupProperties::default();
        popup_props.align = crate::props::item::PopupAlign::Center;
        let mut popup_bg = BackgroundProps::new();
        popup_bg.color = Some(CATPUCCIN_MOCHA.base.clone());
        popup_bg.corner_radius = Some(8);
        popup_bg.border_width = Some(2);
        popup_bg.border_color = Some(CATPUCCIN_MOCHA.surface1.clone());
        popup_props.background = Some(popup_bg);
        item.popup = Some(popup_props);
        item.scripting.click_script = Some(ScriptType::String(
            "sketchybar --set clock popup.drawing=toggle".to_string(),
        ));
        api::add_item(&item)?;

        // Add Clock Popups
        let mut clock_date_item = BarItem::new("clock.date".to_string(), ComponentPosition::Right);
        clock_date_item.icon.icon = Some("Date:".to_string());
        clock_date_item.label.label = Some("Loading...".to_string());
        api::add_item(&clock_date_item)?;
        api::set_args("clock.date", &["position=popup.clock"])?;

        let mut clock_utc_item = BarItem::new("clock.utc".to_string(), ComponentPosition::Right);
        clock_utc_item.icon.icon = Some("UTC:".to_string());
        clock_utc_item.label.label = Some("Loading...".to_string());
        api::add_item(&clock_utc_item)?;
        api::set_args("clock.utc", &["position=popup.clock"])?;

        Ok(())
    }
}
