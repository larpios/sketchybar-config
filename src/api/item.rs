pub use crate::api::builder::ItemBuilder;
pub use crate::api::props::*;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum PopupChild {
    Item(Box<BarItem>),
    Slider(Box<Slider>),
}

impl From<BarItem> for PopupChild {
    fn from(item: BarItem) -> Self {
        Self::Item(Box::new(item))
    }
}

impl From<Slider> for PopupChild {
    fn from(slider: Slider) -> Self {
        Self::Slider(Box::new(slider))
    }
}

#[derive(Debug, Clone)]
pub struct BarItem {
    pub name: String,
    pub props: ItemProps,
    pub children: Vec<PopupChild>,
}

impl BarItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            props: Default::default(),
            children: Vec::new(),
        }
    }

    pub fn new_with_pos(name: &str, position: ComponentPosition) -> Self {
        Self {
            name: name.to_string(),
            props: ItemProps {
                geometry: Geometry {
                    position: Some(position),
                    ..Default::default()
                },
                ..Default::default()
            },
            children: Vec::new(),
        }
    }

    pub fn add(&self) -> Result<()> {
        crate::api::add_item(self)
    }

    pub fn set(&self) -> Result<()> {
        crate::api::set_item(&self.name, self)
    }

    pub fn animate_set(&self, curve: &str, duration: u32) -> Result<()> {
        crate::api::animate_set_item(curve, duration, &self.name, self)
    }

    pub fn subscribe<I, E>(&self, events: I) -> Result<()>
    where
        I: IntoIterator<Item = E>,
        E: Into<crate::api::event::BarEvent>,
    {
        crate::api::subscribe(&self.name, events)
    }

    pub fn with_children<I: IntoIterator<Item = PopupChild>>(mut self, children: I) -> Self {
        self.children.extend(children);
        self
    }

    pub fn add_item(mut self, item: BarItem) -> Self {
        self.children.push(PopupChild::Item(Box::new(item)));
        self
    }

    pub fn add_slider(mut self, slider: Slider) -> Self {
        self.children.push(PopupChild::Slider(Box::new(slider)));
        self
    }

    pub fn add_child(mut self, child: PopupChild) -> Self {
        self.children.push(child);
        self
    }
}

impl ToSketchybarArgs for BarItem {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        self.props.to_sketchybar_args()
    }
}

impl ItemBuilder for BarItem {
    fn item_props_mut(&mut self) -> &mut ItemProps {
        &mut self.props
    }
}

#[derive(Debug, Clone, Default)]
pub struct Slider {
    pub name: String,
    pub position: ComponentPosition,
    pub width: Option<u32>,
    pub percentage: Option<u32>,
    pub highlight_color: Option<Argb>,
    pub knob: Option<String>,
    pub knob_props: Option<Text>,
    pub background: Option<BackgroundProps>,
    pub item: Option<ItemProps>,
}

impl Slider {
    pub fn new(name: &str) -> Self {
        Self::new_with_pos(name, ComponentPosition::Left)
    }

    pub fn new_with_pos(name: &str, position: ComponentPosition) -> Self {
        Self {
            name: name.to_string(),
            position,
            ..Default::default()
        }
    }

    pub fn set(&self) -> Result<()> {
        crate::api::set_item(&self.name, self)
    }

    pub fn animate_set(&self, curve: &str, duration: u32) -> Result<()> {
        crate::api::animate_set_item(curve, duration, &self.name, self)
    }

    pub fn slider_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn percentage(mut self, percentage: u32) -> Self {
        self.percentage = Some(percentage);
        self
    }

    pub fn knob(mut self, knob: &str) -> Self {
        self.knob = Some(knob.to_string());
        self
    }

    pub fn knob_props<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Text) -> Text,
    {
        self.knob_props = Some(f(self.knob_props.take().unwrap_or_default()));
        self
    }

    pub fn highlight_color(mut self, color: Argb) -> Self {
        self.highlight_color = Some(color);
        self
    }

    pub fn slider_background<F>(mut self, f: F) -> Self
    where
        F: FnOnce(BackgroundProps) -> BackgroundProps,
    {
        self.background = Some(f(self.background.take().unwrap_or_default()));
        self
    }
}

impl ItemBuilder for Slider {
    fn item_props_mut(&mut self) -> &mut ItemProps {
        self.item.get_or_insert_default()
    }
}

impl ToSketchybarArgs for Slider {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];
        if let Some(width) = self.width {
            args.push(Property::new("slider.width", &width.to_string()));
        }
        if let Some(percentage) = self.percentage {
            args.push(Property::new("slider.percentage", &percentage.to_string()));
        }
        if let Some(color) = &self.highlight_color {
            args.push(Property::new("slider.highlight_color", &color.to_string()));
        }
        if let Some(knob) = &self.knob {
            args.push(Property::new("slider.knob", &knob.to_string()));
        }
        if let Some(knob_props) = &self.knob_props {
            args.extend(knob_props.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("slider.knob.{}", p.property);
                p
            }));
        }
        if let Some(bg) = &self.background {
            args.extend(bg.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("slider.background.{}", p.property);
                p
            }));
        }
        if let Some(item) = &self.item {
            args.extend(item.to_sketchybar_args());
        }
        args
    }
}
