use anyhow::{Ok, Result};
use sketchybarrc::api;
use sketchybarrc::api::bar::Bar;
use sketchybarrc::api::bar::BarPosition;
use sketchybarrc::themes::CATPUCCIN_MOCHA;
use std::env;
use tokio::sync::broadcast;

use sketchybarrc::daemon;
use sketchybarrc::events::Event;
use sketchybarrc::items;
use sketchybarrc::watcher;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Run as persistent event-loop daemon
        if args[1] == "--daemon" {
            let (bus, _) = broadcast::channel::<Event>(100);

            // Instantiate all items
            let items = items::all_items();

            // Spawn background tasks for items that need them
            for item in &items {
                item.spawn_background_task(bus.subscribe()).await;
            }

            let bus_clone = bus.clone();
            tokio::spawn(async move {
                if let Err(e) = daemon::run(bus_clone).await {
                    eprintln!("[daemon] error: {e}");
                }
            });

            return watcher::watch(bus);
        }
        // Send a command to the running daemon
        if args[1] == "--send" {
            if let Some(json) = args.get(2)
                && let Err(e) = daemon::send(json).await
            {
                eprintln!("[send] daemon unavailable: {e}");
            }
            return Ok(());
        }

        // Handle all other commands through registry
        if items::handle_command(&args[1]).await?.is_some() {
            return Ok(());
        }
    }

    // Gracefully stop the previous daemon if it's running
    let _ = daemon::stop().await;

    // Use sysinfo to find and kill old background processes
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_all();

    let current_pid = sysinfo::get_current_pid().ok();

    for (pid, process) in sys.processes() {
        if Some(*pid) == current_pid {
            continue;
        }

        let cmd = process.cmd();
        let cmd_str = cmd.iter().map(|s| s.to_string_lossy()).collect::<Vec<_>>();

        // Match our binary and ensure it's a background process
        let is_our_binary = cmd_str
            .first()
            .is_some_and(|arg| arg.contains("sketchybarrc"));
        let is_watcher = cmd_str.iter().any(|arg| arg == "--watcher");
        let is_daemon = cmd_str.iter().any(|arg| arg == "--daemon");

        if is_our_binary && (is_watcher || is_daemon) {
            process.kill();
        }
    }

    // Small delay to ensure sockets/resources are freed
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Initialize bar
    let _ = std::process::Command::new("sketchybar")
        .args(["--bar", "hidden=off"])
        .status();
    let _ = std::process::Command::new("sketchybar")
        .args(["--remove", "/.*/"])
        .status();

    let bar = Bar {
        color: CATPUCCIN_MOCHA.base,
        position: BarPosition::Top,
        height: 28,
        margin: 8,
        y_offset: 4,
        corner_radius: 21,
        border_width: 2,
        border_color: CATPUCCIN_MOCHA.surface1,
        padding_left: 8,
        padding_right: 8,
        notch_width: 200,
        ..Default::default()
    };

    api::add_bar(&bar)?;

    // Set defaults from nushell config
    api::set_default([
        "icon.font=JetBrainsMono Nerd Font:Regular:14.0",
        &format!("icon.color={}", CATPUCCIN_MOCHA.text),
        "label.font=JetBrainsMono Nerd Font:Regular:12.0",
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

    // Start background daemon
    let _ = std::process::Command::new(&exe_path)
        .arg("--daemon")
        .spawn();

    // Setup all items
    for item in items::all_items() {
        item.setup(&exe_path).await?;
    }

    api::update()?;
    api::trigger_evt("workspace_change")?;

    Ok(())
}
