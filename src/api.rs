use std::process::Command;

use crate::props::{item::BarItem, types::ToSketchybarArgs};
use anyhow::{Ok, Result};

macro_rules! sb {
    ($args:ident) => {
        Command::new("sketchybar").args($args).status()
    };
    ($($arg:expr),*) => {
        Command::new("sketchybar").args([$($arg),*]).status()
    };
}

pub fn add_bar(bar: &super::props::bar::Bar) -> Result<()> {
    let mut args = vec!["--bar".to_string()];

    args.extend_from_slice(
        &bar.to_sketchybar_args()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    );

    sb!(args)?;

    Ok(())
}

pub fn add_item(item: &BarItem) -> Result<()> {
    sb!(
        "--add",
        "item",
        item.name.as_str(),
        item.geometry.position.to_string().as_str()
    )?;

    let mut set_args = vec!["--set".to_string(), item.name.clone()];

    set_args.extend_from_slice(
        &item
            .to_sketchybar_args()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    );

    sb!(set_args)?;

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
