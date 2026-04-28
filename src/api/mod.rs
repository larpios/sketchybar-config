pub mod bar;
pub mod builder;
pub mod components;
pub mod event;
pub mod exec;
pub mod item;
pub mod props;
pub mod types;

use std::process::Command;

use anyhow::{Ok, Result};

use crate::api::bar::Bar;
use crate::api::event::BarEvent;
use crate::api::item::{BarItem, ChildComponent};
use crate::api::props::ComponentPosition;
use crate::api::types::ToSketchybarArgs;

macro_rules! sb {
    ($args:ident) => {
        {
            let output = Command::new("sketchybar").args(&$args).output()?;
            if !output.status.success() {
                eprintln!("Error executing sketchybar command with args: {:?}", $args);
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Ok(())
        }
    };
    ($cmd:expr, $v:ident) => {
        {
            let output = Command::new("sketchybar").arg($cmd).args($v).output()?;
            if !output.status.success() {
                eprintln!("Error executing sketchybar command with args: {}", $cmd);
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Ok(())
        }
    };
    ($($arg:expr),*) => {
        {
            let cmd_args = [$($arg.to_string()),*];
            let output = Command::new("sketchybar").args(&cmd_args).output()?;
            if !output.status.success() {
                eprintln!("Error executing sketchybar command with args: {:?}", cmd_args);
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Ok(())
        }
    };
}

pub fn add_bar(bar: &Bar) -> Result<()> {
    let args: Vec<_> = bar
        .to_sketchybar_args()
        .iter()
        .map(|p| p.to_string())
        .collect();

    sb!("--bar", args)?;

    Ok(())
}

pub fn set_default<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut cmd_args = vec!["--default".to_string()];
    cmd_args.extend(args.into_iter().map(|s| s.as_ref().to_string()));
    sb!(cmd_args)?;
    Ok(())
}

pub fn add_item(item: &BarItem) -> Result<()> {
    sb!("--remove", &item.name)?;

    let mut args = vec![
        "--add".to_string(),
        "item".to_string(),
        item.name.clone(),
        item.props
            .geometry
            .position
            .clone()
            .unwrap_or_default()
            .to_string(),
        "--set".to_string(),
        item.name.clone(),
    ];

    args.extend(
        item.to_sketchybar_args()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    );

    sb!(args)?;

    // Handle children
    for child in &item.children {
        match child {
            ChildComponent::Item(child_item) => {
                let mut c = child_item.clone();
                c.props.geometry.position = Some(ComponentPosition::Popup(item.name.clone()));
                add_item(&c)?;
            }
            ChildComponent::Slider(slider) => {
                add_special_item(
                    "slider",
                    &slider.name,
                    &format!("popup.{}", item.name),
                    slider.as_ref(),
                )?;
            }
            ChildComponent::Space(space) => {
                add_special_item("space", &space.name, &format!("popup.{}", item.name), space)?;
            }
        }
    }

    Ok(())
}

pub fn add_special_item<T: ToSketchybarArgs>(
    kind: &str,
    name: &str,
    parent_or_pos: &str,
    item: &T,
) -> Result<()> {
    // Remove if exists (silently)
    let _ = Command::new("sketchybar")
        .arg("--remove")
        .arg(name)
        .output();

    let mut args = vec![
        "--add".to_string(),
        kind.to_string(),
        name.to_string(),
        parent_or_pos.to_string(),
        "--set".to_string(),
        name.to_string(),
    ];

    args.extend(
        item.to_sketchybar_args()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    );

    sb!(args)?;

    Ok(())
}

pub fn set_item<T: ToSketchybarArgs>(item_name: &str, item: &T) -> Result<()> {
    let mut args = vec!["--set".to_string(), item_name.to_string()];
    args.extend(
        item.to_sketchybar_args()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    );
    sb!(args)?;
    Ok(())
}

pub fn set_args<I, S>(item_name: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut cmd_args = vec!["--set".to_string(), item_name.to_string()];
    cmd_args.extend(args.into_iter().map(|s| s.as_ref().to_string()));
    sb!(cmd_args)?;
    Ok(())
}

pub fn add_event(event: &str) -> Result<()> {
    sb!("--add", "event", event)?;
    Ok(())
}

pub fn subscribe<I, E>(item: &str, events: I) -> Result<()>
where
    I: IntoIterator<Item = E>,
    E: Into<BarEvent>,
{
    let mut cmd_args = vec!["--subscribe".to_string(), item.to_string()];
    cmd_args.extend(events.into_iter().map(|e| e.into().to_string()));
    sb!(cmd_args)?;
    Ok(())
}

pub fn update() -> Result<()> {
    sb!("--update")?;
    Ok(())
}

pub fn trigger_evt<E: Into<BarEvent>>(evt: E) -> Result<()> {
    sb!("--trigger".to_string(), evt.into().to_string())?;
    Ok(())
}

pub fn trigger_evt_with_data(evt: &str, data: &str) -> Result<()> {
    let cmd_args = ["--trigger", evt, &format!("INFO={}", data)];
    let output = Command::new("sketchybar").args(cmd_args).output()?;
    if !output.status.success() {
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}
