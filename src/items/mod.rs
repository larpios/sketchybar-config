pub mod apple;
pub mod battery;
pub mod bluetooth;
pub mod clock;
pub mod cpu;
pub mod keyboard_layout;
pub mod media;
pub mod network;
pub mod volume;
pub mod weather;
pub mod workspaces;

use crate::events::Event;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SketchybarItem: Send + Sync {
    async fn setup(&self, exe_path: &str) -> Result<()>;
    async fn spawn_background_task(&self, _bus: tokio::sync::broadcast::Receiver<Event>) {}
}

pub fn all_items() -> Vec<Box<dyn SketchybarItem>> {
    vec![
        Box::new(apple::Apple),
        Box::new(workspaces::Workspaces),
        Box::new(media::Media),
        Box::new(clock::Clock),
        Box::new(weather::Weather),
        Box::new(volume::Volume),
        Box::new(keyboard_layout::KeyboardLayout),
        Box::new(battery::Battery),
        Box::new(network::Network),
        Box::new(bluetooth::Bluetooth),
        Box::new(cpu::Cpu),
    ]
}

pub async fn handle_command(cmd: &str) -> Result<Option<()>> {
    match cmd {
        "--update-clock" => Ok(Some(clock::Clock::update_command()?)),
        "--update-weather" => Ok(Some(weather::Weather::update_command()?)),
        "--update-battery" => Ok(Some(battery::Battery::update_command()?)),
        "--update-cpu" => Ok(Some(cpu::Cpu::update_command()?)),
        "--update-keyboard-layout" => {
            Ok(Some(keyboard_layout::KeyboardLayout::update_command(None)?))
        }
        "--update-network" => Ok(Some(network::Network::update_command()?)),
        "--update-volume" => Ok(Some(volume::Volume::update_command()?)),
        "--update-media" => Ok(Some(media::Media::update_command()?)),
        "--update-workspaces" => Ok(Some(workspaces::Workspaces::update_command()?)),
        "--update-bluetooth" => Ok(Some(bluetooth::Bluetooth::update().await?)),
        _ => Ok(None),
    }
}
