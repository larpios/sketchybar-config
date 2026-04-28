use crate::api::item::{BarItem, ComponentPosition, ItemBuilder};
use crate::api::types::{Font, FontStyle, PopupAlign, ToggleState};
use crate::items::SketchybarItem;
use crate::themes::{CATPUCCIN_MOCHA, set_alpha};
use anyhow::Result;
use async_trait::async_trait;

pub struct Apple;

#[async_trait]
impl SketchybarItem for Apple {
    async fn setup(&self, _exe_path: &str) -> Result<()> {
        let apple_item = BarItem::new("apple.logo")
            .position(ComponentPosition::Left)
            .icon("")
            .icon_color(CATPUCCIN_MOCHA.text)
            .icon_font(Font {
                family: "JetBrainsMono Nerd Font".to_string(),
                style: FontStyle::Regular,
                size: 18.0,
            })
            .background_drawing(ToggleState::Off)
            .label_drawing(ToggleState::Off)
            .padding_right(15)
            .click_script("sketchybar -m --set $NAME popup.drawing=toggle")
            .popup_drawing(ToggleState::Off)
            .popup_align(PopupAlign::Left)
            .popup_blur_radius(30)
            .popup_background_color(set_alpha(CATPUCCIN_MOCHA.base, 0.7))
            .popup_background_corner_radius(10)
            .popup_background_border_width(2)
            .popup_background_border_color(CATPUCCIN_MOCHA.crust);

        apple_item
            .clone()
            .add_item(menu_item("apple.about", "About This Mac", "open x-apple.systempreferences:com.apple.SystemProfiler.AboutExtension"))
            .add_item(divider("apple.div1"))
            .add_item(menu_item("apple.settings", "System Settings...", "open -a 'System Settings'"))
            .add_item(menu_item("apple.appstore", "App Store...", "open -a 'App Store'"))
            .add_item(divider("apple.div2"))
            .add_item(menu_item("apple.forcequit", "Force Quit...", "osascript -e 'tell application \"System Events\" to key code 53 using {command down, option down}'"))
            .add_item(divider("apple.div3"))
            .add_item(menu_item("apple.sleep", "Sleep", "pmset displaysleepnow"))
            .add_item(menu_item("apple.restart", "Restart...", "osascript -e 'tell application \"System Events\" to restart'"))
            .add_item(menu_item("apple.shutdown", "Shut Down...", "osascript -e 'tell application \"System Events\" to shut down'"))
            .add_item(divider("apple.div4"))
            .add_item(menu_item("apple.lock", "Lock Screen", "osascript -e 'tell application \"System Events\" to keystroke \"q\" using {control down, command down}'"))
            .add_item(menu_item("apple.logout", "Log Out...", "osascript -e 'tell application \"System Events\" to log out'"))
            .add()?;

        let items = vec![
            "apple.about",
            "apple.settings",
            "apple.appstore",
            "apple.forcequit",
            "apple.sleep",
            "apple.restart",
            "apple.shutdown",
            "apple.lock",
            "apple.logout",
        ];

        for item in items {
            crate::api::subscribe(item, vec!["mouse.entered", "mouse.exited"])?;
        }

        Ok(())
    }
}

fn menu_item(name: &str, label: &str, command: &str) -> BarItem {
    let script = format!(
        // sh
        r#"if [ "$SENDER" = "mouse.entered" ]; then 
            sketchybar --set $NAME background.drawing={}
        elif [ "$SENDER" = "mouse.exited" ]; then 
            sketchybar --set $NAME background.drawing={}
        fi"#,
        "on", "off"
    );

    BarItem::new(name)
        .label(label)
        .label_font(Font {
            family: "JetBrainsMono Nerd Font".to_string(),
            style: FontStyle::Bold,
            size: 13.0,
        })
        .label_color(CATPUCCIN_MOCHA.text)
        .icon_drawing(ToggleState::Off)
        .width(180)
        .padding_left(5)
        .padding_right(5)
        .background_color(CATPUCCIN_MOCHA.blue)
        .background_corner_radius(5)
        .background_drawing(ToggleState::Off)
        .script(&script)
        .click_script(&format!(
            "sketchybar -m --set apple.logo popup.drawing=off && {}",
            command
        ))
}

fn divider(name: &str) -> BarItem {
    BarItem::new(name)
        .width(180)
        .background_drawing(ToggleState::On)
        .background_color(CATPUCCIN_MOCHA.surface1)
        .background_height(1)
        .padding_left(10)
        .padding_right(10)
        .icon_drawing(ToggleState::Off)
        .label_drawing(ToggleState::Off)
}
