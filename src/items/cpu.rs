use crate::items::Item;
use anyhow::Result;
use std::process::Command;

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
        // 1. Get number of cores
        let cores_output = Command::new("sysctl").args(["-n", "hw.ncpu"]).output()?;
        let cores: f32 = String::from_utf8_lossy(&cores_output.stdout)
            .trim()
            .parse()
            .unwrap_or(1.0);

        // 2. Get total CPU usage from ps
        // %cpu is the sum of %cpu for all processes. On macOS, this is per-core (e.g. 800% for 8 cores)
        let ps_output = Command::new("ps").args(["-A", "-o", "%cpu"]).output()?;

        let stdout = String::from_utf8_lossy(&ps_output.stdout);
        let mut total_cpu: f32 = 0.0;
        for line in stdout.lines().skip(1) {
            // Skip " %CPU" header
            if let Ok(val) = line.trim().parse::<f32>() {
                total_cpu += val;
            }
        }

        let load = (total_cpu / cores).round() as u8;
        // Clamp to 100
        let load = if load > 100 { 100 } else { load };

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
            .icon_color(CATPUCCIN_MOCHA.red.clone())
            .background_color(CATPUCCIN_MOCHA.surface0.clone())
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
