use crate::api::item::{BarItem, ItemBuilder};
use crate::events::Event;
use crate::items::SketchybarItem;
use anyhow::Result;
use async_trait::async_trait;
use sysinfo::System;

pub struct Cpu;

impl Cpu {
    pub fn update_command() -> Result<()> {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();

        let mut total_usage = 0.0;
        let cpus = sys.cpus();
        for cpu in cpus {
            total_usage += cpu.cpu_usage();
        }
        let avg_usage = total_usage / cpus.len() as f32;

        BarItem::new("cpu")
            .label(&format!("{:.0}%", avg_usage))
            .set()?;

        Ok(())
    }
}

#[async_trait]
impl SketchybarItem for Cpu {
    async fn setup(&self, exe_path: &str) -> Result<()> {
        use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, ToggleState};
        use crate::themes::CATPUCCIN_MOCHA;

        let item = BarItem::new_with_pos("cpu", ComponentPosition::Right)
            .update_freq(2)
            .script(&format!("{} --update-cpu", exe_path))
            .icon("")
            .icon_props(|p| p.color(CATPUCCIN_MOCHA.red))
            .background(|b| b.color(CATPUCCIN_MOCHA.surface0).drawing(ToggleState::On));

        item.add()?;

        Self::update_command()?;

        Ok(())
    }

    async fn spawn_background_task(&self, mut bus: tokio::sync::broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            while let Ok(event) = bus.recv().await {
                if matches!(event, Event::UpdateCpu)
                    && let Err(e) = Self::update_command()
                {
                    eprintln!("[cpu] update error: {e}");
                }
            }
        });
    }
}
