use crate::api::item::{BarItem, ComponentPosition, ItemBuilder};
use crate::api::types::{Font, FontStyle, ToggleState};
use crate::themes::CATPUCCIN_MOCHA;

pub fn setup() -> anyhow::Result<()> {
    let apple_item = BarItem::new("apple.logo")
        .position(ComponentPosition::Left)
        .icon("")
        .icon_color(CATPUCCIN_MOCHA.text.clone())
        .icon_font(Font {
            family: "JetBrainsMono Nerd Font".to_string(),
            style: FontStyle::Regular,
            size: 18.0,
        })
        .background_drawing(ToggleState::Off)
        .label_drawing(ToggleState::Off)
        .padding_right(15)
        .click_script("open -a 'System Settings'");

    apple_item.add()?;

    Ok(())
}
