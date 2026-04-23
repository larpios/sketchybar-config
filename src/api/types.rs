use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct Property {
    pub property: String,
    pub value: String,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.property, self.value)
    }
}

impl Property {
    pub fn new(property: &str, value: &str) -> Self {
        Self {
            property: property.to_string(),
            value: value.to_string(),
        }
    }
}

pub trait ToSketchybarArgs {
    fn to_sketchybar_args(&self) -> Vec<Property>;
}

pub trait SketchyBool {
    fn to_on_off(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    On,
    Off,
    Toggle,
}

impl Display for ToggleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::Toggle => write!(f, "toggle"),
        }
    }
}

impl SketchyBool for ToggleState {
    fn to_on_off(&self) -> String {
        self.to_string()
    }
}

impl SketchyBool for bool {
    fn to_on_off(&self) -> String {
        if *self { "on" } else { "off" }.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Argb {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Argb {
    fn default() -> Self {
        Self::transparent()
    }
}

impl Argb {
    pub fn transparent() -> Self {
        Self {
            a: 0,
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn black() -> Self {
        Self {
            a: 255,
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn from_u32(hex: u32) -> Self {
        Self {
            a: ((hex >> 24) & 0xff) as u8,
            r: ((hex >> 16) & 0xff) as u8,
            g: ((hex >> 8) & 0xff) as u8,
            b: (hex & 0xff) as u8,
        }
    }
}

impl FromStr for Argb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..2] != "0x" {
            return Err("ARGB must start with 0x".to_string());
        }
        let a = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
        let r = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;
        let g = u8::from_str_radix(&s[6..8], 16).map_err(|e| e.to_string())?;
        let b = u8::from_str_radix(&s[8..10], 16).map_err(|e| e.to_string())?;

        Ok(Argb { a, r, g, b })
    }
}

impl Display for Argb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "0x{:02x}{:02x}{:02x}{:02x}",
            self.a, self.r, self.g, self.b
        )
    }
}

#[derive(Debug, Clone)]
pub struct Font {
    pub family: String,
    pub style: FontStyle,
    pub size: f32,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: "JetBrainsMono Nerd Font".to_string(),
            style: FontStyle::Regular,
            size: 14.0,
        }
    }
}

impl Display for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.family, self.style, self.size)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular => write!(f, "Regular"),
            Self::Bold => write!(f, "Bold"),
            Self::Italic => write!(f, "Italic"),
            Self::BoldItalic => write!(f, "BoldItalic"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl Display for TextAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Center => write!(f, "center"),
            Self::Right => write!(f, "right"),
        }
    }
}

#[derive(Default, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComponentPosition {
    #[default]
    Left,
    Right,
    Center,
    Popup(String),
}

impl Display for ComponentPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Center => write!(f, "center"),
            Self::Popup(name) => write!(f, "popup.{}", name),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PopupAlign {
    #[default]
    Left,
    Right,
    Center,
}

impl Display for PopupAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Center => write!(f, "center"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum WidthMode {
    Value(u32),
    #[default]
    Dyanmic,
}

impl Display for WidthMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{}", value),
            Self::Dyanmic => write!(f, "dynamic"),
        }
    }
}
