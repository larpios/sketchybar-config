use crate::api::item::{BarItem, ComponentPosition, ItemBuilder};
use crate::api::types::{Font, FontStyle, PopupAlign, ToggleState};
use crate::children;
use crate::items::SketchybarItem;
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use async_trait::async_trait;

pub struct Apple;

#[async_trait]
impl SketchybarItem for Apple {
    async fn setup(&self, _exe_path: &str) -> Result<()> {
        let apple_item = BarItem::new_with_pos("apple.logo", ComponentPosition::Left)
            .icon("")
            .icon_props(|p| {
                p.color(CATPUCCIN_MOCHA.text).font(Font {
                    family: "JetBrainsMono Nerd Font".to_string(),
                    style: FontStyle::Regular,
                    size: 18.0,
                })
            })
            .background(|b| b.drawing(ToggleState::Off))
            .label_props(|p| p.drawing(ToggleState::Off))
            .click_script("sketchybar --animate sin 15 --set $NAME popup.drawing=toggle")
            .popup(|p| {
                p.drawing(ToggleState::Off)
                    .align(PopupAlign::Left)
                    .blur_radius(30)
                    .background(|b| {
                        b.color(CATPUCCIN_MOCHA.base.alpha(0.7))
                            .corner_radius(10)
                            .border_width(2)
                            .border_color(CATPUCCIN_MOCHA.crust)
                    })
            });

        apple_item
            .with_children(children![
                menu_item(
                    "apple.about",
                    "About This Mac",
                    "open x-apple.systempreferences:com.apple.SystemProfiler.AboutExtension"
                ),
                divider("apple.div1"),
                menu_item("apple.settings", "System Settings...", "open -a 'System Settings'"),
                menu_item("apple.appstore", "App Store...", "open -a 'App Store'"),
                divider("apple.div2"),
                menu_item(
                    "apple.forcequit",
                    "Force Quit...",
                    "osascript -e 'tell application \"System Events\" to key code 53 using {command down, option down}'"
                ),
                divider("apple.div3"),
                menu_item("apple.sleep", "Sleep", "pmset displaysleepnow"),
                menu_item(
                    "apple.restart",
                    "Restart...",
                    "osascript -e 'tell application \"System Events\" to restart'"
                ),
                menu_item(
                    "apple.shutdown",
                    "Shut Down...",
                    "osascript -e 'tell application \"System Events\" to shut down'"
                ),
                divider("apple.div4"),
                menu_item(
                    "apple.lock",
                    "Lock Screen",
                    "osascript -e 'tell application \"System Events\" to keystroke \"q\" using {control down, command down}'"
                ),
                menu_item(
                    "apple.logout",
                    "Log Out...",
                    "osascript -e 'tell application \"System Events\" to log out'"
                ),
            ])
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
            crate::api::subscribe(
                item,
                vec!["mouse.entered", "mouse.exited", "mouse.exited.global"],
            )?;
        }

        Ok(())
    }
}

fn menu_item(name: &str, label: &str, command: &str) -> BarItem {
    let script = format!(
        // sh
        r#"if [ "$SENDER" = "mouse.entered" ]; then
            sketchybar --animate sin 10 --set $NAME background.drawing={}
        elif [ "$SENDER" = "mouse.exited" ] || [ "$SENDER" = "mouse.exited.global" ]; then
            sketchybar --animate sin 10 --set $NAME background.drawing={}
        fi"#,
        "on", "off"
    );

    BarItem::new(name)
        .label(label)
        .label_props(|p| {
            p.font(Font {
                family: "JetBrainsMono Nerd Font".to_string(),
                style: FontStyle::Bold,
                size: 13.0,
            })
            .color(CATPUCCIN_MOCHA.text)
        })
        .icon_props(|p| p.drawing(ToggleState::Off))
        .width(180)
        .padding_left(5)
        .padding_right(5)
        .background(|b| {
            b.color(CATPUCCIN_MOCHA.blue)
                .corner_radius(5)
                .drawing(ToggleState::Off)
        })
        .script(&script)
        .click_script(&format!(
            "sketchybar --animate sin 15 --set apple.logo popup.drawing=off && {}",
            command
        ))
}

fn divider(name: &str) -> BarItem {
    BarItem::new(name)
        .width(180)
        .background(|b| {
            b.drawing(ToggleState::On)
                .color(CATPUCCIN_MOCHA.surface1)
                .height(1)
        })
        .padding_left(10)
        .padding_right(10)
        .icon_props(|p| p.drawing(ToggleState::Off))
        .label_props(|p| p.drawing(ToggleState::Off))
}
