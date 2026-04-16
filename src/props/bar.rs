use crate::props::types::{Argb, Property, ToSketchybarArgs, SketchyBool};
use std::fmt::Display;

pub struct Bar {
    /// Color of the bar
    pub color: Argb,
    /// Color of the bars border
    pub border_color: Argb,
    pub position: BarPosition,
    pub height: i32,
    pub notch_display_height: i32,
    pub margin: i32,
    pub y_offset: i32,
    pub corner_radius: u32,
    pub border_width: u32,
    pub blur_radius: u32,
    pub padding_left: u32,
    pub padding_right: u32,
    pub notch_width: u32,
    pub notch_offset: u32,
    pub display: DisplayMode,
    pub hidden: HiddenMode,
    pub topmost: TopmostMode,
    pub sticky: bool,
    pub font_smoothing: bool,
    pub shadow: bool,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            color: Argb {a: 68,r: 0,g: 0,b: 0 },
            border_color: Argb { a:255, r:255, g: 0, b: 0 },
            position: BarPosition::Top,
            height: 25,
            notch_display_height: 0,
            margin: 0,
            y_offset: 0,
            corner_radius: 0,
            border_width: 0,
            blur_radius: 0,
            padding_left: 0,
            padding_right: 0,
            notch_width: 200,
            notch_offset: 0,
            display: DisplayMode::All,
            hidden: HiddenMode::Toggle(false),
            topmost: TopmostMode::Toggle(false),
            sticky: true,
            font_smoothing: false,
            shadow: false,
        }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self::new()
    }
}

impl ToSketchybarArgs for Bar {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        vec![
            Property::new("color", &self.color.to_string()),
            Property::new("border_color", &self.border_color.to_string()),
            Property::new("position", &self.position.to_string()),
            Property::new("height", &self.height.to_string()),
            Property::new(
                "notch_display_height",
                &self.notch_display_height.to_string(),
            ),
            Property::new("margin", &self.margin.to_string()),
            Property::new("y_offset", &self.y_offset.to_string()),
            Property::new("corner_radius", &self.corner_radius.to_string()),
            Property::new("border_width", &self.border_width.to_string()),
            Property::new("blur_radius", &self.blur_radius.to_string()),
            Property::new("padding_left", &self.padding_left.to_string()),
            Property::new("padding_right", &self.padding_right.to_string()),
            Property::new("notch_width", &self.notch_width.to_string()),
            Property::new("notch_offset", &self.notch_offset.to_string()),
            Property::new("display", &self.display.to_string()),
            Property::new("hidden", &self.hidden.to_string()),
            Property::new("topmost", &self.topmost.to_string()),
            Property::new("sticky", &self.sticky.to_on_off()),
            Property::new("font_smoothing", &self.font_smoothing.to_on_off()),
            Property::new("shadow", &self.shadow.to_on_off()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BarPosition {
    Top,
    Bottom,
}

impl Display for BarPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarPosition::Top => write!(f, "top"),
            BarPosition::Bottom => write!(f, "bottom"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DisplayMode {
    Main,
    All,
    IDs(Vec<u32>),
}

impl Display for DisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayMode::Main => write!(f, "main"),
            DisplayMode::All => write!(f, "all"),
            DisplayMode::IDs(ids) => write!(
                f,
                "{}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HiddenMode {
    Toggle(bool),
    Current,
}

impl Display for HiddenMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HiddenMode::Toggle(value) => write!(f, "{}", if *value { "on" } else { "off" }),
            HiddenMode::Current => write!(f, "current"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopmostMode {
    Toggle(bool),
    Window,
}

impl Display for TopmostMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopmostMode::Toggle(value) => write!(f, "{}", if *value { "on" } else { "off" }),
            TopmostMode::Window => write!(f, "window"),
        }
    }
}
