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

use anyhow::Result;

/// Registry for command-based item updates
pub struct ItemRegistry;

impl ItemRegistry {
    /// Execute an item command if registered. Returns Some if handled.
    pub async fn execute(cmd: &str, _args: &[String]) -> Result<Option<()>> {
        match cmd {
            "--update-clock" => Ok(Some(clock::Clock::update_command()?)),
            "--update-weather" => Ok(Some(weather::Weather::update_command()?)),
            "--update-battery" => Ok(Some(battery::Battery::update_command()?)),
            "--update-cpu" => Ok(Some(cpu::Cpu::update_command()?)),
            "--update-keyboard-layout" => {
                Ok(Some(keyboard_layout::KeyboardLayout::update_command()?))
            }
            "--update-network" => Ok(Some(network::Network::update_command()?)),
            "--update-volume" => Ok(Some(volume::update_command()?)),
            "--update-media" => Ok(Some(media::update_command()?)),
            "--update-workspaces" => Ok(Some(workspaces::update_command()?)),
            "--update-bluetooth" => Ok(Some(bluetooth::update().await?)),
            _ => Ok(None),
        }
    }

    /// Setup all items
    pub async fn setup_all(exe_path: &str) -> Result<()> {
        // Left items
        apple::setup()?;
        workspaces::setup(exe_path)?;
        media::setup(exe_path)?;

        // Right items (first added is rightmost)
        clock::Clock::setup(exe_path)?;
        weather::Weather::setup(exe_path)?;
        volume::setup(exe_path)?;
        keyboard_layout::KeyboardLayout::setup(exe_path)?;
        battery::Battery::setup(exe_path)?;
        network::Network::setup(exe_path)?;
        bluetooth::setup(exe_path).await?;
        cpu::Cpu::setup(exe_path)?;

        Ok(())
    }
}
