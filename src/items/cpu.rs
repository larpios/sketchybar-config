use crate::events::Event;
use crate::items::SketchybarItem;
use anyhow::Result;
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::{sync::Mutex, thread, time::Duration};
use sysinfo::{CpuRefreshKind, System};

lazy_static! {
    static ref SYS: Mutex<System> = Mutex::new(System::new_all());
}

#[derive(Debug, Clone)]
pub struct CpuData {
    pub load: u8,
}

pub struct Cpu;

impl Cpu {
    pub(super) fn update_command() -> Result<()> {
        let data = Self::fetch()?;
        Self::update_items(&data)
    }
    pub(super) fn fetch() -> anyhow::Result<CpuData> {
        let mut sys = SYS
            .lock()
            .map_err(|_| anyhow::anyhow!("SYS mutex poisoned"))?;

        sys.refresh_cpu_specifics(CpuRefreshKind::everything());

        thread::sleep(Duration::from_millis(200));

        sys.refresh_cpu_specifics(CpuRefreshKind::everything());

        let load = (sys.global_cpu_usage().round() as u8).clamp(0, 100);

        Ok(CpuData { load })
    }

    pub(super) fn update_items(data: &CpuData) -> anyhow::Result<()> {
        use crate::api::item::{BarItem, ItemBuilder};

        BarItem::new("cpu")
            .label(&format!("{}%", data.load))
            .set()?;

        Ok(())
    }
}

#[async_trait]
impl SketchybarItem for Cpu {
    async fn setup(&self, exe_path: &str) -> Result<()> {
        use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, ToggleState};
        use crate::themes::CATPUCCIN_MOCHA;

        let item = BarItem::new("cpu")
            .position(ComponentPosition::Right)
            .update_freq(2)
            .script(&format!("{} --update-cpu", exe_path))
            .icon("")
            .icon_color(CATPUCCIN_MOCHA.red)
            .background_color(CATPUCCIN_MOCHA.surface0)
            .background_drawing(ToggleState::On);

        item.add()?;

        // Initial update
        let data = Self::fetch()?;
        Self::update_items(&data)?;

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
