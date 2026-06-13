use crate::api::item::{BarItem, ItemBuilder};
use crate::events::Event;
use crate::items::SketchybarItem;
use anyhow::Result;
use async_trait::async_trait;
use sysinfo::System;

pub struct Cpu;

impl Cpu {
    pub fn update_command() -> Result<()> {
        // This is now mostly for manual triggers.
        // For accuracy in short-lived processes, sysinfo needs a delay between refreshes.
        let mut sys = System::new();
        sys.refresh_cpu_usage();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_usage();

        Self::update_with_sys(&mut sys)
    }

    fn update_with_sys(sys: &mut System) -> Result<()> {
        let mut total_usage = 0.0;
        let cpus = sys.cpus();
        if cpus.is_empty() {
            return Ok(());
        }

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
    async fn setup(&self, _exe_path: &str) -> Result<()> {
        use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, ToggleState};
        use crate::themes::CATPUCCIN_MOCHA;

        let item = BarItem::new_with_pos("cpu", ComponentPosition::Right)
            .icon("")
            .icon_props(|p| p.color(CATPUCCIN_MOCHA.red))
            .background(|b| b.color(CATPUCCIN_MOCHA.surface0).drawing(ToggleState::On));

        item.add()?;

        // Initial update will be handled by the background task
        // but we can do a quick one here if we want (it will be 0% or sleep-based)
        // Self::update_command()?;

        Ok(())
    }

    async fn spawn_background_task(&self, mut bus: tokio::sync::broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            let mut sys = System::new();
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        sys.refresh_cpu_usage();
                        if let Err(e) = Self::update_with_sys(&mut sys) {
                            eprintln!("[cpu] background update error: {e}");
                        }
                    }
                    Ok(event) = bus.recv() => {
                        if matches!(event, Event::UpdateCpu) {
                            sys.refresh_cpu_usage();
                            if let Err(e) = Self::update_with_sys(&mut sys) {
                                eprintln!("[cpu] manual update error: {e}");
                            }
                        }
                    }
                }
            }
        });
    }
}
