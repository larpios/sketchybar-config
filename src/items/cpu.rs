use crate::items::Item;
use anyhow::Result;
use lazy_static::lazy_static;
use std::{sync::Mutex, thread, time::Duration};
use sysinfo::{CpuRefreshKind, System};

lazy_static! {
    static ref SYS: Mutex<System> = Mutex::new(System::new_all());
}

#[derive(Debug, Clone)]
pub struct Cpu {
    pub load: u8,
}

impl Item for Cpu {
    fn fetch() -> Result<Self> {
        Cpu::fetch()
    }

    fn update_items(&self) -> Result<()> {
        Cpu::update_items(self)
    }

    fn setup(exe_path: &str) -> Result<()> {
        Cpu::setup(exe_path)
    }
}

impl Cpu {
    pub fn update_command() -> Result<()> {
        let data = Self::fetch()?;
        Self::update_items(&data)
    }
    pub fn fetch() -> anyhow::Result<Self> {
        let mut sys = SYS.lock().unwrap();

        sys.refresh_cpu_specifics(CpuRefreshKind::everything());

        thread::sleep(Duration::from_millis(200));

        sys.refresh_cpu_specifics(CpuRefreshKind::everything());

        let load = (sys.global_cpu_usage().round() as u8).clamp(0, 100);

        Ok(Self { load })
    }

    pub fn setup(exe_path: &str) -> anyhow::Result<()> {
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

    pub fn update_items(data: &Self) -> anyhow::Result<()> {
        use crate::api::item::{BarItem, ItemBuilder};

        BarItem::new("cpu")
            .label(&format!("{}%", data.load))
            .set()?;

        Ok(())
    }
}
