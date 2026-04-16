use crate::props::types::{Argb, Font, Property, ToSketchybarArgs, SketchyBool};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct BarItem {
    pub name: String,

    pub geometry: Geometry,
    pub icon: Icon,
    pub label: Label,
    pub scripting: Scripting,
    pub text: Option<Text>,
    pub popup: Option<PopupProperties>,
}

impl ToSketchybarArgs for BarItem {
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
    pub update_freq: u32,
    pub updates: UpdateMode,
    pub mach_helper: Option<String>,
}

impl ToSketchybarArgs for Scripting {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![
            Property::new("update_freq", &self.update_freq.to_string()),
            Property::new("updates", &self.updates.to_string()),
        ];

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

impl Scripting {
    pub fn new() -> Self {
        Self {
            script: None,
            click_script: None,
            update_freq: 0,
            updates: UpdateMode::default(),
            mach_helper: None,
        }
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

#[derive(Debug, Clone)]
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
                    .iter_mut()
                    .map(|p| {
                        p.property = format!("icon.{}", p.property);
                        p.clone()
                    })
                    .collect::<Vec<Property>>(),
            );
        }

        args
    }
}

impl Icon {
    pub fn new() -> Self {
        Self {
            icon: None,
            props: None,
        }
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    pub label: Option<String>,
    pub props: Option<Text>,
}

impl Label {
    pub fn new() -> Self {
        Self {
            label: None,
            props: None,
        }
    }
}

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
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
                    .iter_mut()
                    .map(|p| {
                        p.property = format!("label.{}", p.property);
                        p.clone()
                    })
                    .collect::<Vec<Property>>(),
            );
        }

        args
    }
}

impl BarItem {
    pub fn new(name: String, position: ComponentPosition) -> Self {
        Self {
            name,
            geometry: Geometry::new(position),
            icon: Icon::new(),
            label: Label::new(),
            scripting: Scripting::new(),
            text: None,
            popup: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Text {
    /// If the text is rendered
    pub drawing: Option<bool>,
    pub highlight: Option<bool>,
    pub color: Option<Argb>,
    pub highlight_color: Option<Argb>,
    pub padding_left: Option<u32>,
    pub padding_right: Option<u32>,
    pub y_offset: Option<i32>,
    pub font: Option<Font>,
    pub scroll_duration: Option<u32>,
    pub max_chars: Option<u32>,
    pub width: Option<WidthMode>,
    pub align: Option<TextAlignment>,
    pub background: Option<BackgroundProps>,
    pub shadow: Option<ShadowProperties>,
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
            args.push(Property::new("highlight_color", &highlight_color.to_string()));
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
            args.push(Property::new("scroll_duration", &scroll_duration.to_string()));
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
            args.extend(background.to_sketchybar_args().iter_mut().map(|p| {
                p.property = format!("background.{}", p.property);
                p.clone()
            }));
        }

        if let Some(shadow) = &self.shadow {
            args.extend(shadow.to_sketchybar_args().iter_mut().map(|p| {
                p.property = format!("shadow.{}", p.property);
                p.clone()
            }));
        }

        args
    }
}

#[derive(Debug, Clone, Default)]
pub struct BackgroundProps {
    pub drawing: Option<bool>,
    pub color: Option<Argb>,
    pub border_color: Option<Argb>,
    pub border_width: Option<u32>,
    pub height: Option<u32>,
    pub corner_radius: Option<u32>,
    pub padding_left: Option<i32>,
    pub padding_right: Option<i32>,
    pub y_offset: Option<i32>,
    pub x_offset: Option<i32>,
    pub clip: Option<f32>,
    pub image: Option<ImageProps>,
    pub shadow: Option<ShadowProperties>,
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
        if let Some(clip) = self.clip {
            args.push(Property::new("clip", &clip.to_string()));
        }

        if let Some(image) = &self.image {
            args.extend(image.to_sketchybar_args().iter_mut().map(|p| {
                if p.property.is_empty() {
                    p.property = "image".to_string();
                } else {
                    p.property = format!("image.{}", p.property);
                }
                p.clone()
            }));
        }

        if let Some(shadow) = &self.shadow {
            args.extend(shadow.to_sketchybar_args().iter_mut().map(|p| {
                p.property = format!("shadow.{}", p.property);
                p.clone()
            }));
        }

        args
    }
}

impl BackgroundProps {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct ImageProps {
    pub drawing: bool,
    pub scale: f32,
    pub border_color: Argb,
    pub border_width: u32,
    pub corner_radius: u32,
    pub padding_left: i32,
    pub padding_right: i32,
    pub y_offset: i32,
    pub image: ImageType,
    pub shadow: Option<ShadowProperties>,
}

impl ToSketchybarArgs for ImageProps {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![
            Property::new("", &self.image.to_string()),
            Property::new("drawing", &self.drawing.to_on_off()),
            Property::new("scale", &self.scale.to_string()),
            Property::new("border_color", &self.border_color.to_string()),
            Property::new("border_width", &self.border_width.to_string()),
            Property::new("corner_radius", &self.corner_radius.to_string()),
            Property::new("padding_left", &self.padding_left.to_string()),
            Property::new("padding_right", &self.padding_right.to_string()),
            Property::new("y_offset", &self.y_offset.to_string()),
        ];

        if let Some(shadow) = &self.shadow {
            args.extend(shadow.to_sketchybar_args().iter_mut().map(|p| {
                p.property = format!("shadow.{}", p.property);
                p.clone()
            }));
        }

        args
    }
}

impl ImageProps {
    pub fn new(image: ImageType) -> Self {
        Self {
            drawing: false,
            scale: 1.0,
            border_color: Argb::transparent(),
            border_width: 0,
            corner_radius: 0,
            padding_left: 0,
            padding_right: 0,
            y_offset: 0,
            image,
            shadow: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShadowProperties {
    pub drawing: bool,
    pub color: Argb,
    pub angle: u32,
    pub distance: u32,
}

impl ToSketchybarArgs for ShadowProperties {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        vec![
            Property::new("drawing", &self.drawing.to_on_off()),
            Property::new("color", &self.color.to_string()),
            Property::new("angle", &self.angle.to_string()),
            Property::new("distance", &self.distance.to_string()),
        ]
    }
}

impl Default for ShadowProperties {
    fn default() -> Self {
        Self {
            drawing: false,
            color: Argb::black(),
            angle: 30,
            distance: 5,
        }
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

#[derive(Debug, Clone, Default)]
pub struct Geometry {
    /// If the item should be drawn into the bar
    pub drawing: Option<bool>,
    /// Position of the item in the bar
    pub position: Option<ComponentPosition>,
    /// Spaces to show this item on
    pub space: Option<u32>,
    pub display: Option<DisplayHandle>,
    pub ignore_association: Option<bool>,
    pub y_offset: Option<i32>,
    pub padding_left: Option<i32>,
    pub padding_right: Option<i32>,
    pub width: Option<WidthMode>,
    pub scroll_texts: Option<bool>,
    pub blur_radius: Option<u32>,
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
            args.push(Property::new("ignore_association", &ignore_association.to_on_off()));
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
        if let Some(scroll_texts) = self.scroll_texts {
            args.push(Property::new("scroll_texts", &scroll_texts.to_on_off()));
        }
        if let Some(blur_radius) = self.blur_radius {
            args.push(Property::new("blur_radius", &blur_radius.to_string()));
        }

        if let Some(space) = self.space {
            if space != 0 {
                args.push(Property::new("space", &space.to_string()));
            }
        }

        if let Some(display) = &self.display {
            if let DisplayHandle::Id(ids) = display {
                if !ids.is_empty() && ids[0] != 0 {
                    args.push(Property::new("display", &display.to_string()));
                }
            } else {
                args.push(Property::new("display", &display.to_string()));
            }
        }

        if let Some(background) = &self.background {
            args.extend(background.to_sketchybar_args().iter_mut().map(|p| {
                p.property = format!("background.{}", p.property);
                p.clone()
            }));
        }

        args
    }
}

impl Geometry {
    pub fn new(position: ComponentPosition) -> Self {
        Self {
            position: Some(position),
            ..Default::default()
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct PopupProperties {
    pub align: PopupAlign,
    pub background: Option<BackgroundProps>,
    pub drawing: Option<String>, // 'on', 'off', or 'toggle'
    pub blur_radius: Option<u32>,
    pub horizontal: Option<bool>,
}

impl ToSketchybarArgs for PopupProperties {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![Property::new("popup.align", &self.align.to_string())];

        if let Some(drawing) = &self.drawing {
            args.push(Property::new("popup.drawing", drawing));
        }

        if let Some(blur_radius) = self.blur_radius {
            args.push(Property::new("popup.blur_radius", &blur_radius.to_string()));
        }

        if let Some(horizontal) = self.horizontal {
            args.push(Property::new("popup.horizontal", &horizontal.to_on_off()));
        }

        if let Some(background) = &self.background {
            args.extend(background.to_sketchybar_args().iter_mut().map(|p| {
                p.property = format!("popup.background.{}", p.property);
                p.clone()
            }));
        }

        args
    }
}

impl Default for PopupProperties {
    fn default() -> Self {
        Self {
            align: PopupAlign::Center,
            background: None,
            drawing: None,
            blur_radius: None,
            horizontal: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PopupAlign {
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
