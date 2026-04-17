use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
use crate::props::types::Font;
use crate::themes::CATPUCCIN_MOCHA;
pub fn setup() -> anyhow::Result<()> {
    let mut apple_item = BarItem::new("apple.logo".to_string(), ComponentPosition::Left);
    apple_item.props.icon.icon = Some("".to_string());

    let icon_props = Text {
        color: Some(CATPUCCIN_MOCHA.text.clone()),
        font: Some(Font {
            family: "JetBrainsMono Nerd Font".to_string(),
            type_: crate::props::types::FontType::Regular,
            size: 18.0,
        }),
        ..Default::default()
    };
    apple_item.props.icon.props = Some(icon_props);

    let label_props = Text {
        drawing: Some(false),
        ..Default::default()
    };
    apple_item.props.label.props = Some(label_props);

    apple_item.props.geometry.padding_right = Some(15);

    let apple_bg = BackgroundProps {
        color: Some(crate::props::types::Argb::transparent()),
        drawing: Some(true),
        ..Default::default()
    };
    apple_item.props.geometry.background = Some(apple_bg);

    apple_item.props.scripting.click_script =
        Some(ScriptType::String("open -a 'System Settings'".to_string()));

    api::add_item(&apple_item)?;

    Ok(())
}
