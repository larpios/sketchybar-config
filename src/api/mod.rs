pub mod bar;
pub mod builder;
pub mod components;
pub mod event;
pub mod exec;
pub mod item;
pub mod macros;
pub mod props;
pub mod types;

use std::process::Command;

use anyhow::{Ok, Result};

use crate::api::bar::Bar;
use crate::api::components::{Bracket, Space};
use crate::api::event::BarEvent;
use crate::api::item::{BarItem, PopupChild, Slider};
use crate::api::props::ComponentPosition;
use crate::api::types::{RelativePosition, ToSketchybarArgs};

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
            PopupChild::Item(child_item) => {
                let mut c = child_item.clone();
                c.props.geometry.position = Some(ComponentPosition::Popup(item.name.clone()));
                add_item(&c)?;
            }
            PopupChild::Slider(slider) => {
                let mut s = slider.clone();
                s.position = ComponentPosition::Popup(item.name.clone());
                add_slider(&s)?;
            }
        }
    }

    Ok(())
}

pub fn add_bracket(bracket: &Bracket) -> Result<()> {
    let name = bracket.name.as_str();
    let _ = Command::new("sketchybar").args(["--remove", name]).output();

    let mut args = vec!["--add".to_string(), "bracket".to_string(), name.to_string()];
    args.extend(bracket.members.clone());
    args.extend(["--set".to_string(), name.to_string()]);

    args.extend(
        bracket
            .to_sketchybar_args()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    );

    sb!(args)
}

pub fn add_slider(slider: &Slider) -> Result<()> {
    add_special_item(
        "slider",
        slider.name.as_str(),
        Some(slider.position.clone()),
        slider,
    )
}

pub fn add_space(space: &Space) -> Result<()> {
    add_special_item(
        "space",
        space.name.as_str(),
        Some(space.position.clone()),
        space,
    )
}

pub fn add_special_item<T: ToSketchybarArgs, S: AsRef<str>, P: Into<Option<ComponentPosition>>>(
    kind: S,
    name: S,
    parent_or_pos: P,
    item: &T,
) -> Result<()> {
    let name = name.as_ref();
    let kind = kind.as_ref();
    let parent_or_pos = parent_or_pos.into();

    // Remove if exists (silently)
    let _ = Command::new("sketchybar")
        .arg("--remove")
        .arg(name)
        .output();

    let mut args = vec!["--add".to_string(), kind.to_string(), name.to_string()];

    if let Some(pos) = parent_or_pos {
        args.push(pos.to_string());
    }

    args.extend(["--set".to_string(), name.to_string()]);

    args.extend(
        item.to_sketchybar_args()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    );

    sb!(args)
}

pub fn rename_item(item_name: &str, new_name: &str) -> Result<()> {
    sb!("--rename", item_name, new_name)
}

pub fn remove_item(item_name: &str) -> Result<()> {
    sb!("--remove", &item_name)
}

pub fn clone_item(
    item_name: &str,
    new_name: &str,
    rel_pos: Option<RelativePosition>,
) -> Result<()> {
    sb!(
        "--clone",
        item_name,
        new_name,
        rel_pos.unwrap_or_default().to_string()
    )
}

pub fn animate_set_item<T: ToSketchybarArgs>(
    curve: &str,
    duration: u32,
    item_name: &str,
    item: &T,
) -> Result<()> {
    let mut args = vec![
        "--animate".to_string(),
        curve.to_string(),
        duration.to_string(),
        "--set".to_string(),
        item_name.to_string(),
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
