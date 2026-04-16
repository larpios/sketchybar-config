use crate::property;
use crate::props::item::{Icon, Text};
use crate::props::types::{Property, ToSketchybarArgs};
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub load: u8,
}

impl ToSketchybarArgs for Cpu {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let label = format!("{}%", self.load);

        vec![property!("label", &label)]
    }
}

impl Cpu {
    pub fn fetch() -> anyhow::Result<Self> {
        let mut sys = System::new_all();
        sys.refresh_cpu_usage();
        // We need a small sleep to get accurate delta reading
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_cpu_usage();

        let global_cpu = sys.global_cpu_usage();
        let load = global_cpu.round() as u8;

        Ok(Self { load })
    }

    pub fn setup(exe_path: &str) -> anyhow::Result<()> {
        use crate::api;
        use crate::props::item::{BackgroundProps, BarItem, ComponentPosition, ScriptType};
        use crate::themes::CATPUCCIN_MOCHA;

        let mut item = BarItem::new("cpu".to_string(), ComponentPosition::Right);
        item.scripting.update_freq = 2;
        item.scripting.script = Some(ScriptType::String(format!("{} --update-cpu", exe_path)));
        item.icon.icon = Some("".to_string());
        item.icon.props = Some(Text {
            color: Some(CATPUCCIN_MOCHA.red.clone()),
            ..Default::default()
        });
        let mut bg = BackgroundProps::new();
        bg.color = Some(CATPUCCIN_MOCHA.surface0.clone());
        bg.drawing = Some(true);
        item.geometry.background = Some(bg);
        api::add_item(&item)?;
        Ok(())
    }
}
