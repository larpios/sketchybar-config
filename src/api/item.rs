use crate::api::components::Space;
pub use crate::api::types::{
    Argb, ComponentPosition, Font, PopupAlign, Property, SketchyBool, TextAlignment,
    ToSketchybarArgs, ToggleState, WidthMode,
};
use anyhow::Result;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ChildComponent {
    Item(Box<BarItem>),
    Slider(Box<Slider>),
    Space(Space),
}

#[derive(Debug, Clone)]
pub struct BarItem {
    pub name: String,
    pub props: ItemProps,
    pub children: Vec<ChildComponent>,
}

impl BarItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            props: ItemProps::default(),
            children: Vec::new(),
        }
    }

    pub fn add(&self) -> Result<()> {
        crate::api::add_item(self)
    }

    pub fn set(&self) -> Result<()> {
        crate::api::set_item(&self.name, self)
    }

    pub fn subscribe<I, E>(&self, events: I) -> Result<()>
    where
        I: IntoIterator<Item = E>,
        E: Into<crate::api::event::BarEvent>,
    {
        crate::api::subscribe(&self.name, events)
    }

    pub fn add_item(mut self, item: BarItem) -> Self {
        self.children.push(ChildComponent::Item(Box::new(item)));
        self
    }

    pub fn add_space(mut self, space: Space) -> Self {
        self.children.push(ChildComponent::Space(space));
        self
    }

    pub fn add_slider(mut self, slider: Slider) -> Self {
        self.children.push(ChildComponent::Slider(Box::new(slider)));
        self
    }

    pub fn add_child(mut self, child: ChildComponent) -> Self {
        self.children.push(child);
        self
    }
}

impl ToSketchybarArgs for BarItem {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        self.props.to_sketchybar_args()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShadowProps {
    pub drawing: Option<ToggleState>,
    pub color: Option<Argb>,
    pub angle: Option<u32>,
    pub distance: Option<u32>,
}

impl ToSketchybarArgs for ShadowProps {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];
        if let Some(drawing) = self.drawing {
            args.push(Property::new("shadow.drawing", &drawing.to_on_off()));
        }
        if let Some(color) = &self.color {
            args.push(Property::new("shadow.color", &color.to_string()));
        }
        if let Some(angle) = self.angle {
            args.push(Property::new("shadow.angle", &angle.to_string()));
        }
        if let Some(distance) = self.distance {
            args.push(Property::new("shadow.distance", &distance.to_string()));
        }
        args
    }
}

pub trait ItemBuilder: Sized {
    fn item_props_mut(&mut self) -> &mut ItemProps;

    fn apply_if<F>(self, condition: bool, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if condition { f(self) } else { self }
    }

    fn position(mut self, position: ComponentPosition) -> Self {
        self.item_props_mut().geometry.position = Some(position);
        self
    }

    fn drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut().geometry.drawing = Some(drawing);
        self
    }

    fn width(mut self, width: u32) -> Self {
        self.item_props_mut().geometry.width = Some(WidthMode::Value(width));
        self
    }

    fn y_offset(mut self, offset: i32) -> Self {
        self.item_props_mut().geometry.y_offset = Some(offset);
        self
    }

    fn padding_left(mut self, padding: u32) -> Self {
        self.item_props_mut().geometry.padding_left = Some(padding);
        self
    }

    fn padding_right(mut self, padding: u32) -> Self {
        self.item_props_mut().geometry.padding_right = Some(padding);
        self
    }

    fn icon(mut self, icon: &str) -> Self {
        self.item_props_mut().icon.icon = Some(icon.to_string());
        self
    }

    fn icon_props(mut self, props: Text) -> Self {
        self.item_props_mut().icon.props = Some(props);
        self
    }

    fn icon_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .icon
            .props
            .get_or_insert_default()
            .color = Some(color);
        self
    }

    fn icon_font(mut self, font: Font) -> Self {
        self.item_props_mut()
            .icon
            .props
            .get_or_insert_default()
            .font = Some(font);
        self
    }

    fn icon_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut()
            .icon
            .props
            .get_or_insert_default()
            .drawing = Some(drawing);
        self
    }

    fn icon_shadow_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut()
            .icon
            .props
            .get_or_insert_default()
            .shadow
            .get_or_insert_default()
            .drawing = Some(drawing);
        self
    }

    fn icon_shadow_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .icon
            .props
            .get_or_insert_default()
            .shadow
            .get_or_insert_default()
            .color = Some(color);
        self
    }

    fn label(mut self, label: &str) -> Self {
        self.item_props_mut().label.label = Some(label.to_string());
        self
    }

    fn label_props(mut self, props: Text) -> Self {
        self.item_props_mut().label.props = Some(props);
        self
    }

    fn label_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .label
            .props
            .get_or_insert_default()
            .color = Some(color);
        self
    }

    fn label_font(mut self, font: Font) -> Self {
        self.item_props_mut()
            .label
            .props
            .get_or_insert_default()
            .font = Some(font);
        self
    }

    fn label_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut()
            .label
            .props
            .get_or_insert_default()
            .drawing = Some(drawing);
        self
    }

    fn label_shadow_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut()
            .label
            .props
            .get_or_insert_default()
            .shadow
            .get_or_insert_default()
            .drawing = Some(drawing);
        self
    }

    fn label_shadow_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .label
            .props
            .get_or_insert_default()
            .shadow
            .get_or_insert_default()
            .color = Some(color);
        self
    }

    fn scroll_texts(mut self, scroll: bool) -> Self {
        self.item_props_mut().geometry.scroll_texts = Some(scroll);
        self
    }

    fn script(mut self, script: &str) -> Self {
        self.item_props_mut().scripting.script = Some(ScriptType::String(script.to_string()));
        self
    }

    fn click_script(mut self, script: &str) -> Self {
        self.item_props_mut().scripting.click_script = Some(ScriptType::String(script.to_string()));
        self
    }

    fn update_freq(mut self, freq: u32) -> Self {
        self.item_props_mut().scripting.update_freq = Some(freq);
        self
    }

    fn updates(mut self, mode: UpdateMode) -> Self {
        self.item_props_mut().scripting.updates = Some(mode);
        self
    }

    fn background(mut self, props: BackgroundProps) -> Self {
        self.item_props_mut().geometry.background = Some(props);
        self
    }

    fn background_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .color = Some(color);
        self
    }

    fn background_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .drawing = Some(drawing);
        self
    }

    fn background_corner_radius(mut self, radius: u32) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .corner_radius = Some(radius);
        self
    }

    fn background_border_width(mut self, width: u32) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .border_width = Some(width);
        self
    }

    fn background_border_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .border_color = Some(color);
        self
    }

    fn background_height(mut self, height: u32) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .height = Some(height);
        self
    }

    fn background_blur_radius(mut self, radius: u32) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .blur_radius = Some(radius);
        self
    }

    fn background_clip(mut self, clip: bool) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .clip = Some(clip);
        self
    }

    fn background_shadow_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .shadow
            .get_or_insert_default()
            .drawing = Some(drawing);
        self
    }

    fn background_shadow_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .shadow
            .get_or_insert_default()
            .color = Some(color);
        self
    }

    fn background_image(mut self, image: ImageType) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .image
            .get_or_insert_default()
            .image = Some(image);
        self
    }

    fn background_image_scale(mut self, scale: f32) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .image
            .get_or_insert_default()
            .scale = Some(scale);
        self
    }

    fn background_image_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .image
            .get_or_insert_default()
            .drawing = Some(drawing);
        self
    }

    fn background_image_corner_radius(mut self, radius: u32) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .image
            .get_or_insert_default()
            .corner_radius = Some(radius);
        self
    }

    fn background_image_blur_radius(mut self, radius: u32) -> Self {
        self.item_props_mut()
            .geometry
            .background
            .get_or_insert_default()
            .image
            .get_or_insert_default()
            .blur_radius = Some(radius);
        self
    }

    fn popup(mut self, props: PopupProps) -> Self {
        self.item_props_mut().popup = Some(props);
        self
    }

    fn popup_blur_radius(mut self, radius: u32) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .blur_radius = Some(radius);
        self
    }

    fn popup_align(mut self, align: PopupAlign) -> Self {
        self.item_props_mut().popup.get_or_insert_default().align = Some(align);
        self
    }

    fn popup_topmost(mut self, topmost: ToggleState) -> Self {
        self.item_props_mut().popup.get_or_insert_default().topmost = Some(topmost);
        self
    }

    fn popup_horizontal(mut self, horizontal: ToggleState) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .horizontal = Some(horizontal);
        self
    }

    fn popup_y_offset(mut self, offset: i32) -> Self {
        self.item_props_mut().popup.get_or_insert_default().y_offset = Some(offset);
        self
    }

    fn popup_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut().popup.get_or_insert_default().drawing = Some(drawing);
        self
    }

    fn popup_background_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .background
            .get_or_insert_default()
            .color = Some(color);
        self
    }

    fn popup_background_corner_radius(mut self, radius: u32) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .background
            .get_or_insert_default()
            .corner_radius = Some(radius);
        self
    }

    fn popup_background_blur_radius(mut self, radius: u32) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .background
            .get_or_insert_default()
            .blur_radius = Some(radius);
        self
    }

    fn popup_background_border_width(mut self, width: u32) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .background
            .get_or_insert_default()
            .border_width = Some(width);
        self
    }

    fn popup_background_border_color(mut self, color: Argb) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .background
            .get_or_insert_default()
            .border_color = Some(color);
        self
    }

    fn popup_background_image(mut self, image: ImageType) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .background
            .get_or_insert_default()
            .image
            .get_or_insert_default()
            .image = Some(image);
        self
    }

    fn popup_background_image_blur_radius(mut self, radius: u32) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .background
            .get_or_insert_default()
            .image
            .get_or_insert_default()
            .blur_radius = Some(radius);
        self
    }

    fn popup_background_image_drawing(mut self, drawing: ToggleState) -> Self {
        self.item_props_mut()
            .popup
            .get_or_insert_default()
            .background
            .get_or_insert_default()
            .image
            .get_or_insert_default()
            .drawing = Some(drawing);
        self
    }

    fn text(mut self, props: Text) -> Self {
        self.item_props_mut().text = Some(props);
        self
    }

    fn text_align(mut self, align: TextAlignment) -> Self {
        self.item_props_mut().text.get_or_insert_default().align = Some(align);
        self
    }
}

impl ItemBuilder for BarItem {
    fn item_props_mut(&mut self) -> &mut ItemProps {
        &mut self.props
    }
}

#[derive(Debug, Clone, Default)]
pub struct ItemProps {
    pub geometry: Geometry,
    pub icon: Icon,
    pub label: Label,
    pub scripting: Scripting,
    pub text: Option<Text>,
    pub popup: Option<PopupProps>,
}

impl ToSketchybarArgs for ItemProps {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = self.geometry.to_sketchybar_args();
        args.extend(self.icon.to_sketchybar_args());
        args.extend(self.label.to_sketchybar_args());
        args.extend(self.scripting.to_sketchybar_args());

        if let Some(text) = &self.text {
            args.extend(text.to_sketchybar_args());
        }

        if let Some(popup) = &self.popup {
            args.extend(popup.to_sketchybar_args());
        }

        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct Scripting {
    pub script: Option<ScriptType>,
    pub click_script: Option<ScriptType>,
    pub update_freq: Option<u32>,
    pub updates: Option<UpdateMode>,
    pub mach_helper: Option<String>,
}

impl ToSketchybarArgs for Scripting {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];

        if let Some(update_freq) = self.update_freq {
            args.push(Property::new("update_freq", &update_freq.to_string()));
        }
        if let Some(updates) = self.updates {
            args.push(Property::new("updates", &updates.to_string()));
        }
        if let Some(script) = &self.script {
            args.push(Property::new("script", &script.to_string()));
        }
        if let Some(click_script) = &self.click_script {
            args.push(Property::new("click_script", &click_script.to_string()));
        }
        if let Some(mach_helper) = &self.mach_helper {
            args.push(Property::new("mach_helper", mach_helper));
        }

        args
    }
}

#[derive(Debug, Clone)]
pub enum ScriptType {
    Path(String),
    String(String),
}

impl Display for ScriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptType::Path(path) => write!(f, "{}", path),
            ScriptType::String(script) => write!(f, "{}", script),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Icon {
    pub icon: Option<String>,
    pub props: Option<Text>,
}

impl ToSketchybarArgs for Icon {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];
        if let Some(icon) = &self.icon {
            args.push(Property::new("icon", icon));
        }
        if let Some(props) = &self.props {
            args.extend(
                props
                    .to_sketchybar_args()
                    .into_iter()
                    .map(|mut p| {
                        p.property = format!("icon.{}", p.property);
                        p
                    })
                    .collect::<Vec<Property>>(),
            );
        }

        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct Label {
    pub label: Option<String>,
    pub props: Option<Text>,
}

impl ToSketchybarArgs for Label {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];
        if let Some(label) = &self.label {
            args.push(Property::new("label", label));
        }
        if let Some(props) = &self.props {
            args.extend(
                props
                    .to_sketchybar_args()
                    .into_iter()
                    .map(|mut p| {
                        p.property = format!("label.{}", p.property);
                        p
                    })
                    .collect::<Vec<Property>>(),
            );
        }

        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct Text {
    pub drawing: Option<ToggleState>,
    pub highlight: Option<ToggleState>,
    pub color: Option<Argb>,
    pub highlight_color: Option<Argb>,
    pub padding_left: Option<u32>,
    pub padding_right: Option<u32>,
    pub y_offset: Option<i32>,
    pub font: Option<Font>,
    pub scroll_duration: Option<f32>,
    pub max_chars: Option<u32>,
    pub width: Option<WidthMode>,
    pub align: Option<TextAlignment>,
    pub background: Option<BackgroundProps>,
    pub shadow: Option<ShadowProps>,
}

impl ToSketchybarArgs for Text {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];

        if let Some(drawing) = self.drawing {
            args.push(Property::new("drawing", &drawing.to_on_off()));
        }
        if let Some(highlight) = self.highlight {
            args.push(Property::new("highlight", &highlight.to_on_off()));
        }
        if let Some(color) = &self.color {
            args.push(Property::new("color", &color.to_string()));
        }
        if let Some(highlight_color) = &self.highlight_color {
            args.push(Property::new(
                "highlight_color",
                &highlight_color.to_string(),
            ));
        }
        if let Some(padding_left) = self.padding_left {
            args.push(Property::new("padding_left", &padding_left.to_string()));
        }
        if let Some(padding_right) = self.padding_right {
            args.push(Property::new("padding_right", &padding_right.to_string()));
        }
        if let Some(y_offset) = self.y_offset {
            args.push(Property::new("y_offset", &y_offset.to_string()));
        }
        if let Some(font) = &self.font {
            args.push(Property::new("font", &font.to_string()));
        }
        if let Some(scroll_duration) = self.scroll_duration {
            args.push(Property::new(
                "scroll_duration",
                &scroll_duration.to_string(),
            ));
        }
        if let Some(max_chars) = self.max_chars {
            args.push(Property::new("max_chars", &max_chars.to_string()));
        }
        if let Some(width) = self.width {
            args.push(Property::new("width", &width.to_string()));
        }
        if let Some(align) = &self.align {
            args.push(Property::new("align", &align.to_string()));
        }

        if let Some(background) = &self.background {
            args.extend(background.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("background.{}", p.property);
                p
            }));
        }

        if let Some(shadow) = &self.shadow {
            args.extend(shadow.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("shadow.{}", p.property);
                p
            }));
        }

        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct BackgroundProps {
    pub drawing: Option<ToggleState>,
    pub color: Option<Argb>,
    pub border_color: Option<Argb>,
    pub border_width: Option<u32>,
    pub height: Option<u32>,
    pub corner_radius: Option<u32>,
    pub padding_left: Option<i32>,
    pub padding_right: Option<i32>,
    pub y_offset: Option<i32>,
    pub x_offset: Option<i32>,
    pub blur_radius: Option<u32>,
    pub clip: Option<bool>,
    pub image: Option<ImageProps>,
    pub shadow: Option<ShadowProps>,
}

impl ToSketchybarArgs for BackgroundProps {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];

        if let Some(drawing) = self.drawing {
            args.push(Property::new("drawing", &drawing.to_on_off()));
        }
        if let Some(color) = &self.color {
            args.push(Property::new("color", &color.to_string()));
        }
        if let Some(border_color) = &self.border_color {
            args.push(Property::new("border_color", &border_color.to_string()));
        }
        if let Some(border_width) = self.border_width {
            args.push(Property::new("border_width", &border_width.to_string()));
        }
        if let Some(height) = self.height {
            args.push(Property::new("height", &height.to_string()));
        }
        if let Some(corner_radius) = self.corner_radius {
            args.push(Property::new("corner_radius", &corner_radius.to_string()));
        }
        if let Some(padding_left) = self.padding_left {
            args.push(Property::new("padding_left", &padding_left.to_string()));
        }
        if let Some(padding_right) = self.padding_right {
            args.push(Property::new("padding_right", &padding_right.to_string()));
        }
        if let Some(y_offset) = self.y_offset {
            args.push(Property::new("y_offset", &y_offset.to_string()));
        }
        if let Some(x_offset) = self.x_offset {
            args.push(Property::new("x_offset", &x_offset.to_string()));
        }
        if let Some(blur_radius) = self.blur_radius {
            args.push(Property::new("blur_radius", &blur_radius.to_string()));
        }
        if let Some(clip) = self.clip {
            args.push(Property::new("clip", &clip.to_on_off()));
        }

        if let Some(image) = &self.image {
            args.extend(image.to_sketchybar_args().into_iter().map(|mut p| {
                if p.property.is_empty() {
                    p.property = "image".to_string();
                } else {
                    p.property = format!("image.{}", p.property);
                }
                p
            }));
        }

        if let Some(shadow) = &self.shadow {
            args.extend(shadow.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("shadow.{}", p.property);
                p
            }));
        }

        args
    }
}

#[derive(Default, Debug, Clone)]
pub struct ImageProps {
    pub drawing: Option<ToggleState>,
    pub scale: Option<f32>,
    pub blur_radius: Option<u32>,
    pub border_color: Option<Argb>,
    pub border_width: Option<u32>,
    pub corner_radius: Option<u32>,
    pub padding_left: Option<i32>,
    pub padding_right: Option<i32>,
    pub y_offset: Option<i32>,
    pub image: Option<ImageType>,
    pub shadow: Option<ShadowProps>,
}

impl ToSketchybarArgs for ImageProps {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];

        if let Some(image) = &self.image {
            args.push(Property::new("", image.to_string().as_str()));
        }

        if let Some(drawing) = self.drawing {
            args.push(Property::new("drawing", &drawing.to_on_off()));
        }
        if let Some(scale) = self.scale {
            args.push(Property::new("scale", &scale.to_string()));
        }
        if let Some(blur_radius) = self.blur_radius {
            args.push(Property::new("blur_radius", &blur_radius.to_string()));
        }
        if let Some(border_color) = &self.border_color {
            args.push(Property::new("border_color", &border_color.to_string()));
        }
        if let Some(border_width) = self.border_width {
            args.push(Property::new("border_width", &border_width.to_string()));
        }
        if let Some(corner_radius) = self.corner_radius {
            args.push(Property::new("corner_radius", &corner_radius.to_string()));
        }
        if let Some(padding_left) = self.padding_left {
            args.push(Property::new("padding_left", &padding_left.to_string()));
        }
        if let Some(padding_right) = self.padding_right {
            args.push(Property::new("padding_right", &padding_right.to_string()));
        }
        if let Some(y_offset) = self.y_offset {
            args.push(Property::new("y_offset", &y_offset.to_string()));
        }

        if let Some(shadow) = &self.shadow {
            args.extend(shadow.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("shadow.{}", p.property);
                p
            }));
        }

        args
    }
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Path(String),
    AppBundleId(String),
    AppName(String),
    MediaArtwork,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(path) => write!(f, "{}", path),
            Self::AppBundleId(bundle_id) => write!(f, "app.{}", bundle_id),
            Self::AppName(app_name) => write!(f, "app.{}", app_name),
            Self::MediaArtwork => write!(f, "media.artwork"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Geometry {
    pub drawing: Option<ToggleState>,
    pub position: Option<ComponentPosition>,
    pub space: Option<u32>,
    pub display: Option<DisplayHandle>,
    pub ignore_association: Option<ToggleState>,
    pub y_offset: Option<i32>,
    pub padding_left: Option<u32>,
    pub padding_right: Option<u32>,
    pub width: Option<WidthMode>,
    pub blur_radius: Option<u32>,
    pub scroll_texts: Option<bool>,
    pub background: Option<BackgroundProps>,
}

impl ToSketchybarArgs for Geometry {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];

        if let Some(drawing) = self.drawing {
            args.push(Property::new("drawing", &drawing.to_on_off()));
        }
        if let Some(position) = &self.position {
            args.push(Property::new("position", &position.to_string()));
        }
        if let Some(ignore_association) = self.ignore_association {
            args.push(Property::new(
                "ignore_association",
                &ignore_association.to_on_off(),
            ));
        }
        if let Some(scroll_texts) = self.scroll_texts {
            args.push(Property::new("scroll_texts", &scroll_texts.to_on_off()));
        }
        if let Some(y_offset) = self.y_offset {
            args.push(Property::new("y_offset", &y_offset.to_string()));
        }
        if let Some(padding_left) = self.padding_left {
            args.push(Property::new("padding_left", &padding_left.to_string()));
        }
        if let Some(padding_right) = self.padding_right {
            args.push(Property::new("padding_right", &padding_right.to_string()));
        }
        if let Some(width) = &self.width {
            args.push(Property::new("width", &width.to_string()));
        }
        if let Some(blur_radius) = self.blur_radius {
            args.push(Property::new("blur_radius", &blur_radius.to_string()));
        }

        if let Some(space) = self.space {
            args.push(Property::new("space", &space.to_string()));
        }

        if let Some(display) = &self.display {
            args.push(Property::new("display", &display.to_string()));
        }

        if let Some(background) = &self.background {
            args.extend(background.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("background.{}", p.property);
                p
            }));
        }

        args
    }
}

#[derive(Debug, Clone)]
pub enum DisplayHandle {
    Id(Vec<u32>),
    Active,
}

impl Display for DisplayHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(
                f,
                "{}",
                id.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Active => write!(f, "active"),
        }
    }
}

impl Default for DisplayHandle {
    fn default() -> Self {
        Self::Id(vec![0])
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UpdateMode {
    Toggle(bool),
    WhenShown,
}

impl Display for UpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toggle(value) => write!(f, "{}", if *value { "on" } else { "off" }),
            Self::WhenShown => write!(f, "when_shown"),
        }
    }
}

impl Default for UpdateMode {
    fn default() -> Self {
        Self::Toggle(true)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PopupProps {
    pub drawing: Option<ToggleState>,
    pub horizontal: Option<ToggleState>,
    pub topmost: Option<ToggleState>,
    pub height: Option<u32>,
    pub blur_radius: Option<u32>,
    pub y_offset: Option<i32>,
    pub align: Option<PopupAlign>,
    pub background: Option<BackgroundProps>,
}

impl ToSketchybarArgs for PopupProps {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![];

        if let Some(topmost) = self.topmost {
            args.push(Property::new("popup.topmost", &topmost.to_on_off()));
        }

        if let Some(align) = self.align {
            args.push(Property::new("popup.align", &align.to_string()));
        }

        if let Some(drawing) = &self.drawing {
            args.push(Property::new("popup.drawing", &drawing.to_on_off()));
        }

        if let Some(y_offset) = self.y_offset {
            args.push(Property::new("popup.y_offset", &y_offset.to_string()));
        }

        if let Some(blur_radius) = self.blur_radius {
            args.push(Property::new("popup.blur_radius", &blur_radius.to_string()));
        }

        if let Some(horizontal) = self.horizontal {
            args.push(Property::new("popup.horizontal", &horizontal.to_on_off()));
        }

        if let Some(background) = &self.background {
            args.extend(background.to_sketchybar_args().into_iter().map(|mut p| {
                p.property = format!("popup.background.{}", p.property);
                p
            }));
        }

        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct Slider {
    pub name: String,
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
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn set(&self) -> Result<()> {
        crate::api::set_item(&self.name, self)
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

    pub fn knob_color(mut self, color: Argb) -> Self {
        self.knob_props.get_or_insert_default().color = Some(color);
        self
    }

    pub fn knob_highlight(mut self, highlight: ToggleState) -> Self {
        self.knob_props.get_or_insert_default().highlight = Some(highlight);
        self
    }

    pub fn knob_font(mut self, font: Font) -> Self {
        self.knob_props.get_or_insert_default().font = Some(font);
        self
    }

    pub fn highlight_color(mut self, color: Argb) -> Self {
        self.highlight_color = Some(color);
        self
    }

    pub fn slider_background_color(mut self, color: Argb) -> Self {
        self.background.get_or_insert_default().color = Some(color);
        self
    }

    pub fn slider_background_height(mut self, height: u32) -> Self {
        self.background.get_or_insert_default().height = Some(height);
        self
    }

    pub fn slider_background_corner_radius(mut self, radius: u32) -> Self {
        self.background.get_or_insert_default().corner_radius = Some(radius);
        self
    }

    pub fn slider_background_drawing(mut self, drawing: ToggleState) -> Self {
        self.background.get_or_insert_default().drawing = Some(drawing);
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
