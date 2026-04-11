use crate::api;
use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType, Text};
use crate::props::types::Font;
use crate::themes::CATPUCCIN_MOCHA;
pub fn setup() -> anyhow::Result<()> {
    let mut apple_item = BarItem::new("apple.logo".to_string(), ComponentPosition::Left);
    apple_item.icon.icon = Some("".to_string());

    let mut icon_props = Text::default();
    icon_props.color = Some(CATPUCCIN_MOCHA.text.clone());
    icon_props.font = Some(Font {
        family: "JetBrainsMono Nerd Font".to_string(),
        type_: crate::props::types::FontType::Regular,
        size: 18.0,
    });
    apple_item.icon.props = Some(icon_props);

    let mut label_props = Text::default();
    label_props.drawing = Some(false);
    apple_item.label.props = Some(label_props);

    apple_item.geometry.padding_right = Some(15);

    let mut apple_bg = BackgroundProps::new();
    apple_bg.color = Some(crate::props::types::ARGB::new(0, 0, 0, 0)); // Transparent
    apple_bg.drawing = Some(true);
    apple_item.geometry.background = Some(apple_bg);

    apple_item.scripting.click_script = Some(ScriptType::String(
        "open -a 'System Settings'".to_string(),
    ));

    api::add_item(&apple_item)?;

    Ok(())
}
