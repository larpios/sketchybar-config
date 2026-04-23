use crate::api::item::BackgroundProps;
use crate::api::types::{Argb, Property, ToSketchybarArgs};

#[derive(Debug, Clone)]
pub struct Graph {
    pub color: Option<Argb>,
    pub fill_color: Option<Argb>,
    pub line_width: Option<f32>,
    data_points: Vec<f32>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            color: None,
            fill_color: None,
            line_width: None,
            data_points: Vec::new(),
        }
    }

    pub fn push_data_point(&mut self, value: f32) {
        self.data_points.push(value);
    }

    pub fn push_data_points(&mut self, data_points: &[f32]) {
        self.data_points.extend(data_points);
    }
}

impl ToSketchybarArgs for Graph {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];
        if let Some(color) = &self.color {
            args.push(Property::new("graph.color", &color.to_string()));
        }
        if let Some(fill_color) = &self.fill_color {
            args.push(Property::new("graph.fill_color", &fill_color.to_string()));
        }
        if let Some(line_width) = self.line_width {
            args.push(Property::new("graph.line_width", &line_width.to_string()));
        }
        if !self.data_points.is_empty() {
            let data = self
                .data_points
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(",");
            args.push(Property::new("graph.data_points", &data));
        }
        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct Space {
    pub space: Option<u32>,
    pub display: Option<u32>,
}

impl Space {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ToSketchybarArgs for Space {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];
        if let Some(space) = self.space {
            args.push(Property::new("space", &space.to_string()));
        }
        if let Some(display) = self.display {
            args.push(Property::new("display", &display.to_string()));
        }
        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct Alias {
    pub color: Option<Argb>,
    pub scale: Option<f32>,
}

impl Alias {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ToSketchybarArgs for Alias {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];
        if let Some(color) = &self.color {
            args.push(Property::new("alias.color", &color.to_string()));
        }
        if let Some(scale) = self.scale {
            args.push(Property::new("alias.scale", &scale.to_string()));
        }
        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct Bracket {
    pub members: Vec<String>,
    pub background: Option<BackgroundProps>,
}

impl Bracket {
    pub fn new(members: Vec<String>) -> Self {
        Self {
            members,
            background: None,
        }
    }
}

impl ToSketchybarArgs for Bracket {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];
        // Note: members are used during creation (--add bracket name member1 member2)
        // This struct handles properties set via --set
        if let Some(bg) = &self.background {
            args.extend(bg.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("background.{}", p.property);
                p
            }));
        }
        args
    }
}
