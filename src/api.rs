use std::process::Command;

use crate::props::{item::BarItem, types::ToSketchybarArgs};
use anyhow::{Ok, Result};

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
    ($($arg:expr),*) => {
        {
            let output = Command::new("sketchybar").args([$($arg),*]).output()?;
            if !output.status.success() {
                eprintln!("Error executing sketchybar command with args: {:?}", [$($arg),*]);
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Ok(())
        }
    };
}

pub fn add_bar(bar: &super::props::bar::Bar) -> Result<()> {
    let mut args = vec!["--bar".to_string()];

    args.extend(
        bar.to_sketchybar_args()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    );

    sb!(args)?;

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
    // Remove if exists (silently)
    let _ = Command::new("sketchybar")
        .arg("--remove")
        .arg(&item.name)
        .output();

    let mut args = vec![
        "--add".to_string(),
        "item".to_string(),
        item.name.clone(),
        item.geometry
            .position
            .unwrap_or(crate::props::item::ComponentPosition::Left)
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

pub fn subscribe<I, S>(item: &str, events: I) -> Result<()> 
where 
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut cmd_args = vec!["--subscribe".to_string(), item.to_string()];
    cmd_args.extend(events.into_iter().map(|s| s.as_ref().to_string()));
    sb!(cmd_args)?;
    Ok(())
}

pub fn update() -> Result<()> {
    sb!("--update")?;
    Ok(())
}

pub fn trigger_evt(evt: &str) -> Result<()> {
    sb!("--trigger", evt)?;
    Ok(())
}
