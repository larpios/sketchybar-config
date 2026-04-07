use std::{fmt::Display, process::Command};

pub fn add_item(item: &BarItem) {
    Command::new("sketchybar")
        .args(["--add", item.to_string().as_str()])
        .status()
        .unwrap();
    
}

#[derive(Debug, Clone)]
pub struct BarItem {
    /// If the item should be drawn into the bar
    pub drawing: bool,
    /// Position of the item in the bar
    pub position: ComponentPosition,
    /// Spaces to show this item on
    pub space: u32,
    pub display: DisplayHandle,
    pub ignore_association: bool,
    pub y_offset: i32,
    pub padding_left: i32,
    pub padding_right: i32,
    pub width: WidthMode,
    pub scroll_texts: bool,
    pub blur_radius: u32,

    pub icon: &'static str,
    pub label: &'static str,

    pub script: Option<&'static str>,
    pub click_script: Option<&'static str>,
    pub update_freq: u32,
    pub updates: UpdateMode,
    pub mach_helper: Option<&'static str>,
}


#[derive(Debug, Clone)]
pub enum UpdateMode {
    Toggle(bool),
    WhenShown,
}

impl Default for UpdateMode {
    fn default() -> Self {
        Self::Toggle(true)
    }
}

impl BarItem {
    fn new(position: ComponentPosition) -> Self {
        Self {
            drawing: true,
            // Not supposed to have a default value.
            position,
            space: 0,
            display: DisplayHandle::Id(vec![0]),
            ignore_association: false,
            y_offset: 0,
            padding_left: 0,
            padding_right: 0,
            width: WidthMode::Dyanmic,
            scroll_texts: false,
            blur_radius: 0,
        }
    }
    pub fn padding_left(mut self, padding_left: i32) -> Self {
        self.padding_left = padding_left;
        self
    }

    pub fn padding_right(mut self, padding_right: i32) -> Self {
        self.padding_right = padding_right;
        self
    }

    pub fn y_offset(mut self, y_offset: i32) -> Self {
        self.y_offset = y_offset;
        self
    }

    pub fn space(mut self, space: u32) -> Self {
        self.space = space;
        self
    }

    pub fn ignore_association(mut self, ignore_association: bool) -> Self {
        self.ignore_association = ignore_association;
        self
    }

    pub fn drawing(mut self, drawing: bool) -> Self {
        self.drawing = drawing;
        self
    }

    pub fn position(mut self, position: ComponentPosition) -> Self {
        self.position = position;
        self
    }

    pub fn display(mut self, display: DisplayHandle) -> Self {
        self.display = display;
        self
    }

    pub fn width(mut self, width: WidthMode) -> Self {
        self.width = width;
        self
    }

    pub fn scroll_texts(mut self, scroll_texts: bool) -> Self {
        self.scroll_texts = scroll_texts;
        self
    }

    pub fn blur_radius(mut self, blur_radius: u32) -> Self {
        self.blur_radius = blur_radius;
        self
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum WidthMode {
    Value(u32),
    #[default]
    Dyanmic,
}

#[derive(Debug, Clone)]
pub enum DisplayHandle {
    Id(Vec<u32>),
    Active,
}

impl Default for DisplayHandle {
    fn default() -> Self {
        Self::Id(vec![0])
    }
}

/// Item position in bar
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComponentPosition {
    Left,
    Right,
    Center,
}

impl Display for ComponentPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Center => write!(f, "center"),
        }
    }
}

/// Item position in bar
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ItemPosition {
    Left,
    Right,
    Center,
    Q,
    E,
}

impl Display for ItemPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Center => write!(f, "center"),
            Self::Q => write!(f, "q"),
            Self::E => write!(f, "e"),
        }
    }
}
