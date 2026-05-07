use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, ToggleState};
use crate::api::types::AnimationCurve;
use crate::events::Event;
use crate::items::SketchybarItem;
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use async_trait::async_trait;

const MINIMIZED_WIDTH: u32 = 22;
const MAXIMIZED_WIDTH: u32 = 150;
pub struct Window;

impl Window {
    pub fn update_command() -> Result<()> {
        Self::update()
    }

    pub fn update() -> Result<()> {
        let output = std::process::Command::new("aerospace")
            .args([
                "list-windows",
                "--focused",
                "--format",
                "%{app-name} | %{window-title}",
            ])
            .output()?;

        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();

        let is_empty = title.is_empty() || title.contains("error");

        BarItem::new("window_title")
            .label(&title)
            .width(if is_empty {
                MINIMIZED_WIDTH
            } else {
                MAXIMIZED_WIDTH
            })
            .animate_set(AnimationCurve::Circ, 14)?;

        Ok(())
    }
}

#[async_trait]
impl SketchybarItem for Window {
    async fn setup(&self, exe_path: &str) -> Result<()> {
        crate::api::add_event("aerospace_focus_changed")?;

        BarItem::new("window_title")
            .position(ComponentPosition::Left)
            .icon("󰖯")
            .icon_props(|i| i.color(CATPUCCIN_MOCHA.maroon))
            .width(MINIMIZED_WIDTH)
            .scroll_texts(ToggleState::On)
            .label_props(|l| l.max_chars(17).scroll_duration(150))
            .background(|b| b.color(CATPUCCIN_MOCHA.surface0).drawing(ToggleState::On))
            .padding_left(4)
            .padding_right(4)
            .script(&format!("{} --update-window", exe_path))
            .add()?;

        BarItem::new("window_title").subscribe([
            crate::api::event::BarEvent::FrontAppSwitched,
            crate::api::event::BarEvent::from("aerospace_workspace_change"),
            crate::api::event::BarEvent::SpaceChange,
            crate::api::event::BarEvent::from("aerospace_focus_changed"),
        ])?;

        Self::update()?;

        Ok(())
    }

    async fn spawn_background_task(&self, mut _bus: tokio::sync::broadcast::Receiver<Event>) {}
}
