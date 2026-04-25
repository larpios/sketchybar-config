pub mod apple;
pub mod battery;
pub mod bluetooth;
pub mod clock;
pub mod cpu;
pub mod media;
pub mod network;
pub mod volume;
pub mod weather;
pub mod workspaces;

use anyhow::Result;

/// Unified trait for all bar items
pub trait Item {
    /// Fetch fresh data for this item
    fn fetch() -> Result<Self>
    where
        Self: Sized;

    /// Update the UI elements with current data
    fn update_items(&self) -> Result<()>;

    /// Initial setup of UI elements
    fn setup(exe_path: &str) -> Result<()>;
}

/// Registry for command-based item updates
pub struct ItemRegistry;

impl ItemRegistry {
    /// Execute an item command if registered. Returns Some if handled.
    pub async fn execute(cmd: &str, args: &[String]) -> Result<Option<()>> {
        match cmd {
            "--update-clock" => Ok(Some(clock::Clock::update_command()?)),
            "--update-weather" => Ok(Some(weather::Weather::update_command()?)),
            "--update-battery" => Ok(Some(battery::Battery::update_command()?)),
            "--update-cpu" => Ok(Some(cpu::Cpu::update_command()?)),
            "--update-network" => Ok(Some(network::Network::update_command()?)),
            "--update-volume" => Ok(Some(volume::update_command()?)),
            "--update-media" => Ok(Some(media::update_command()?)),
            "--update-workspaces" => Ok(Some(workspaces::update_command()?)),
            "--update-bluetooth" => Ok(Some(bluetooth::update().await?)),
            "--update-bluetooth-popup" => Ok(Some(bluetooth::update_popup(false).await?)),
            "--scan-bluetooth" => Ok(Some(bluetooth::update_popup(true).await?)),
            "--toggle-bluetooth-device" => {
                if !args.is_empty() {
                    bluetooth::toggle_device(&args[0]).await?;
                    bluetooth::update_popup(false).await?;
                }
                Ok(Some(()))
            }
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
        battery::Battery::setup(exe_path)?;
        network::Network::setup(exe_path)?;
        bluetooth::setup(exe_path).await?;
        cpu::Cpu::setup(exe_path)?;

        Ok(())
    }
}
