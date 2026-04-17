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
        item.props.scripting.update_freq = 10;
        item.props.scripting.script =
            Some(ScriptType::String(format!("{} --update-clock", exe_path)));
        item.props.icon.icon = Some("󰥔".to_string());

        let icon_props = Text {
            color: Some(CATPUCCIN_MOCHA.blue.clone()),
            ..Default::default()
        };
        item.props.icon.props = Some(icon_props);

        let bg = BackgroundProps {
            color: Some(CATPUCCIN_MOCHA.surface0.clone()),
            drawing: Some(true),
            ..Default::default()
        };
        item.props.geometry.background = Some(bg);

        let popup_bg = BackgroundProps {
            color: Some(CATPUCCIN_MOCHA.base.clone()),
            corner_radius: Some(8),
            border_width: Some(2),
            border_color: Some(CATPUCCIN_MOCHA.surface1.clone()),
            ..Default::default()
        };
        let popup_props = crate::props::item::PopupProperties {
            align: crate::props::item::PopupAlign::Center,
            background: Some(popup_bg),
            ..Default::default()
        };
        item.props.popup = Some(popup_props);
        item.props.scripting.click_script = Some(ScriptType::String(
            "sketchybar --set clock popup.drawing=toggle".to_string(),
        ));
        api::add_item(&item)?;

        // Add Clock Popups
        let mut clock_date_item = BarItem::new("clock.date".to_string(), ComponentPosition::Right);
        clock_date_item.props.icon.icon = Some("Date:".to_string());
        clock_date_item.props.label.label = Some("Loading...".to_string());
        api::add_item(&clock_date_item)?;
        api::set_args("clock.date", ["position=popup.clock"])?;

        let mut clock_utc_item = BarItem::new("clock.utc".to_string(), ComponentPosition::Right);
        clock_utc_item.props.icon.icon = Some("UTC:".to_string());
        clock_utc_item.props.label.label = Some("Loading...".to_string());
        api::add_item(&clock_utc_item)?;
        api::set_args("clock.utc", ["position=popup.clock"])?;

        Ok(())
    }
}
