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

#[macro_export]
macro_rules! property {
    ($property:expr, $value:expr) => {
        Property::new($property, $value)
    };
    ($property:expr, $value:ident) => {
        Property::new($property, stringify!($value))
    };
}

pub trait ToSketchybarArgs {
    fn to_sketchybar_args(&self) -> Vec<Property>;
}

pub trait SketchyBool {
    fn to_on_off(&self) -> String;
}

impl SketchyBool for bool {
    fn to_on_off(&self) -> String {
        if *self { "on" } else { "off" }.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ARGB {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for ARGB {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl ARGB {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }
}

impl FromStr for ARGB {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..2] != "0x" {
            return Err("ARGB must start with 0x".to_string());
        }
        let a = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
        let r = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;
        let g = u8::from_str_radix(&s[6..8], 16).map_err(|e| e.to_string())?;
        let b = u8::from_str_radix(&s[8..10], 16).map_err(|e| e.to_string())?;

        Ok(ARGB::new(a, r, g, b))
    }
}

impl Display for ARGB {
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
    pub type_: FontType,
    pub size: f32,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: "JetBrainsMono Nerd Font".to_string(),
            type_: FontType::Regular,
            size: 14.0,
        }
    }
}

impl Display for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.family, self.type_, self.size)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontType {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

impl Display for FontType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular => write!(f, "Regular"),
            Self::Bold => write!(f, "Bold"),
            Self::Italic => write!(f, "Italic"),
            Self::BoldItalic => write!(f, "BoldItalic"),
        }
    }
}
