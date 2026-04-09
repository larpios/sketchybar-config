use anyhow::{Ok, Result};
use sketchybarrc::api;
use sketchybarrc::props::bar::Bar;
use sketchybarrc::props::bar::BarPosition;
use sketchybarrc::themes::CATPUCCIN_MOCHA;
use std::env;

use sketchybarrc::items::{apple, battery, bluetooth, clock, cpu, media, network, volume, weather, workspaces};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--update-weather" => {
                let weather_data = weather::Weather::fetch()?;
                api::set_item("weather", &weather_data)?;
                return Ok(());
            }
            "--update-battery" => {
                let battery_data = battery::Battery::fetch()?;
                api::set_item("battery", &battery_data)?;
                return Ok(());
            }
            "--update-bluetooth" => {
                return bluetooth::update();
            }
            "--update-clock" => {
                let clock_data = clock::Clock::fetch()?;
                api::set_item("clock", &clock_data)?;

                // Update popups
                api::set_args("clock.date", &[&format!("label={}", clock_data.full_date)])?;
                api::set_args("clock.utc", &[&format!("label={}", clock_data.utc_time)])?;

                return Ok(());
            }
            "--update-cpu" => {
                let cpu_data = cpu::Cpu::fetch()?;
                api::set_item("cpu", &cpu_data)?;
                return Ok(());
            }
            "--update-network" => {
                let network_data = network::Network::fetch()?;
                api::set_item("network", &network_data)?;

                // Set network popup info
                api::set_args("network.ip", &[&format!("label={}", network_data.ip)])?;
                api::set_args(
                    "network.device",
                    &[&format!("label={}", network_data.device)],
                )?;
                return Ok(());
            }
            "--update-workspaces" => {
                return workspaces::update();
            }
            "--update-media" => {
                return media::update();
            }
            "--update-volume" => {
                return volume::update();
            }
            _ => {}
        }
    }

    // Clear bar and items
    let _ = std::process::Command::new("sketchybar").args(["--bar", "hidden=off"]).status();
    let _ = std::process::Command::new("sketchybar").args(["--remove", "/.*/"]).status();

    let bar = Bar {
        color: CATPUCCIN_MOCHA.base.clone(),
        position: BarPosition::Top,
        height: 28,
        margin: 8,
        y_offset: 4,
        corner_radius: 21,
        border_width: 2,
        border_color: CATPUCCIN_MOCHA.surface1.clone(),
        padding_left: 8,
        padding_right: 8,
        notch_width: 200,
        ..Default::default()
    };

    api::add_bar(&bar)?;

    // Set defaults from nushell config
    api::set_default(&[
        &format!("icon.font=JetBrainsMono Nerd Font:Regular:14.0"),
        &format!("icon.color={}", CATPUCCIN_MOCHA.text),
        &format!("label.font=JetBrainsMono Nerd Font:Regular:12.0"),
        &format!("label.color={}", CATPUCCIN_MOCHA.text),
        "padding_left=4",
        "padding_right=4",
        "icon.padding_left=6",
        "icon.padding_right=6",
        "label.padding_left=6",
        "label.padding_right=6",
        &format!("background.color={}", CATPUCCIN_MOCHA.surface0),
        "background.corner_radius=9",
        "background.height=20",
        &format!("icon.highlight_color={}", CATPUCCIN_MOCHA.mauve),
        &format!("label.highlight_color={}", CATPUCCIN_MOCHA.mauve),
    ])?;

    let exe_path = env::current_exe()?.to_string_lossy().to_string();

    // Left items (added left to right)
    apple::setup(&exe_path)?;
    workspaces::setup(&exe_path)?;

    // Center items
    media::setup(&exe_path)?;

    // Right items (first added is rightmost, so add Clock first, then Volume, etc.)
    clock::Clock::setup(&exe_path)?;
    volume::setup(&exe_path)?;
    battery::Battery::setup(&exe_path)?;
    network::Network::setup(&exe_path)?;
    bluetooth::setup(&exe_path)?;
    cpu::Cpu::setup(&exe_path)?;
    weather::Weather::setup(&exe_path)?;

    api::update()?;
    api::trigger_evt("workspace_change")?;

    Ok(())
}
