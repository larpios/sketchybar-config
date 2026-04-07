use anyhow::{Ok, Result};
use sketchybar::api;
use sketchybar::props::bar::Bar;
use sketchybar::props::bar::BarPosition;
use sketchybar::themes::CATPUCCIN_MOCHA;

fn main() -> Result<()> {
    let bar = Bar {
        color: CATPUCCIN_MOCHA.base.clone(),
        position: BarPosition::Top,
        height: 28,
        margin: 8,
        y_offset: 4,
        corner_radius: 21,
        border_width: 2,
        border_color: CATPUCCIN_MOCHA.surface1.clone(),
        padding_left: 8,
        padding_right: 8,
        notch_width: 200,
        ..Default::default()
    };

    api::add_bar(&bar)?;
    api::update()?;
    api::trigger_evt("workspace_change")?;

    Ok(())
}
