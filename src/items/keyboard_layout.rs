use crate::items::Item;
use anyhow::Result;
use std::env;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct KeyboardLayout {
    pub name: String,
}

impl Item for KeyboardLayout {
    fn fetch() -> Result<Self> {
        KeyboardLayout::fetch()
    }

    fn update_items(&self) -> Result<()> {
        KeyboardLayout::update_items(self)
    }

    fn setup(exe_path: &str) -> Result<()> {
        KeyboardLayout::setup(exe_path)
    }
}

// For bundle IDs like "com.apple.inputmethod.Korean.2SetKorean"
// or "com.apple.keylayout.Ukrainian" passed via $INFO from the watcher.
fn source_id_to_code(id: &str) -> String {
    if id.contains("Japanese") || id.contains("Kotoeri") {
        return "JA".into();
    }
    if id.contains("Korean") {
        return "KO".into();
    }
    if id.contains("SCIM") || id.contains("Chinese") {
        return "ZH".into();
    }
    if id.contains("Vietnamese") {
        return "VI".into();
    }
    // Last non-empty component is the layout name (e.g. "Ukrainian")
    let name = id.split('.').rfind(|s| !s.is_empty()).unwrap_or(id);
    keyboard_name_to_code(name)
}

// For raw layout names like "U.S.", "Ukrainian", "Finnish"
// (from PlistBuddy's KeyboardLayout Name field).
fn keyboard_name_to_code(name: &str) -> String {
    match name {
        n if n.starts_with("U.S") || n.starts_with("ABC") || n.starts_with("British")
            || n.starts_with("Australian") || n.starts_with("Canadian") =>
        {
            "EN".into()
        }
        n if n.starts_with("Ukrainian") => "UA".into(),
        n if n.starts_with("Russian") => "RU".into(),
        n if n.starts_with("German") || n.starts_with("Austrian") => "DE".into(),
        n if n.starts_with("French") => "FR".into(),
        n if n.starts_with("Spanish") => "ES".into(),
        n if n.starts_with("Finnish") => "FI".into(),
        n if n.starts_with("Polish") => "PL".into(),
        n if n.starts_with("Swedish") => "SV".into(),
        n if n.starts_with("Norwegian") => "NO".into(),
        n if n.starts_with("Danish") => "DA".into(),
        n if n.starts_with("Italian") => "IT".into(),
        n if n.starts_with("Portuguese") => "PT".into(),
        n if n.starts_with("Dutch") => "NL".into(),
        n => n.chars().take(2).collect::<String>().to_uppercase(),
    }
}

fn fetch_via_plist() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let plist = format!(
        "{}/Library/Preferences/com.apple.HIToolbox.plist",
        home
    );
    let output = Command::new("/usr/libexec/PlistBuddy")
        .args(["-c", "Print :AppleInputSourceHistory:0", &plist])
        .output();
    let Ok(output) = output else {
        return "??".to_string();
    };
    let text = String::from_utf8_lossy(&output.stdout);

    // Input Mode entry (CJK IMEs)
    if let Some(mode) = text
        .lines()
        .find(|l| l.trim_start().starts_with("Input Mode"))
        .and_then(|l| l.split_once('='))
        .map(|(_, v)| v.trim())
    {
        return source_id_to_code(mode);
    }

    // Plain keyboard layout entry
    text.lines()
        .find(|l| l.trim_start().starts_with("KeyboardLayout Name"))
        .and_then(|l| l.split_once('='))
        .map(|(_, v)| keyboard_name_to_code(v.trim()))
        .unwrap_or_else(|| "??".to_string())
}

impl KeyboardLayout {
    pub fn fetch() -> Result<Self> {
        Ok(Self {
            name: fetch_via_plist(),
        })
    }

    pub fn update_command() -> Result<()> {
        // When triggered by sketchybar, $INFO carries the bundle ID read in
        // the watcher process. Sketchybar passes it as command-line arg INFO=value.
        let name = match std::env::var("INFO") {
            Ok(info) if !info.is_empty() => source_id_to_code(&info),
            _ => fetch_via_plist(),
        };
        Self::update_items(&KeyboardLayout { name })
    }

    pub fn update_items(data: &Self) -> Result<()> {
        use crate::api::item::{BarItem, ItemBuilder};

        BarItem::new("keyboard_layout")
            .label(&data.name)
            .set()?;

        Ok(())
    }

    pub fn setup(exe_path: &str) -> Result<()> {
        use crate::api::event::BarEvent;
        use crate::api::item::{BarItem, ComponentPosition, ItemBuilder, ToggleState};
        use crate::themes::CATPUCCIN_MOCHA;

        let item = BarItem::new("keyboard_layout")
            .position(ComponentPosition::Right)
            .script(&format!("{} --update-keyboard-layout", exe_path))
            .icon("󰌌")
            .icon_color(CATPUCCIN_MOCHA.lavender)
            .background_color(CATPUCCIN_MOCHA.surface0)
            .background_drawing(ToggleState::On);

        item.add()?;
        crate::api::add_event("keyboard_layout_change")?;
        item.subscribe([BarEvent::Custom("keyboard_layout_change".to_string())])?;

        let data = Self::fetch()?;
        Self::update_items(&data)?;

        Ok(())
    }
}
