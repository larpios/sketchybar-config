use crate::props::types::{ARGB, Font, Property, ToSketchybarArgs};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct BarItem {
    pub name: String,

    pub geometry: Geometry,
    pub icon: Icon,
    pub label: Label,
    pub scripting: Scripting,
    pub text: Text,
}

impl ToSketchybarArgs for BarItem {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = self.geometry.to_sketchybar_args();
        args.extend(self.icon.to_sketchybar_args());
        args.extend(self.label.to_sketchybar_args());
        args.extend(self.scripting.to_sketchybar_args());
        args.extend(self.text.to_sketchybar_args());

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
        vec![
            Property::new("script", &self.script.as_ref().unwrap().to_string()),
            Property::new(
                "click_script",
                &self.click_script.as_ref().unwrap().to_string(),
            ),
            Property::new("update_freq", &self.update_freq.to_string()),
            Property::new("updates", &self.updates.to_string()),
            Property::new(
                "mach_helper",
                &self.mach_helper.as_ref().unwrap().to_string(),
            ),
        ]
    }
}

impl Scripting {
    fn new() -> Self {
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
    fn new() -> Self {
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
    fn new() -> Self {
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
    fn new(name: String, position: ComponentPosition) -> Self {
        Self {
            name,
            geometry: Geometry::new(position),
            icon: Icon::new(),
            label: Label::new(),
            scripting: Scripting::new(),
            text: Text::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    /// If the text is rendered
    pub drawing: bool,
    pub highlight: bool,
    pub color: ARGB,
    pub highlight_color: ARGB,
    pub padding_left: u32,
    pub padding_right: u32,
    pub y_offset: i32,
    pub font: Font,
    pub string: String,
    pub scroll_duration: u32,
    pub max_chars: u32,
    pub width: WidthMode,
    pub align: TextAlignment,
    pub background: Option<BackgroundProperties>,
    pub shadow: Option<ShadowProperties>,
}

impl ToSketchybarArgs for Text {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![
            Property::new("drawing", &self.drawing.to_string()),
            Property::new("highlight", &self.highlight.to_string()),
            Property::new("color", &self.color.to_string()),
            Property::new("highlight_color", &self.highlight_color.to_string()),
            Property::new("padding_left", &self.padding_left.to_string()),
            Property::new("padding_right", &self.padding_right.to_string()),
            Property::new("y_offset", &self.y_offset.to_string()),
            Property::new("font", &self.font.to_string()),
            Property::new("string", &self.string),
            Property::new("scroll_duration", &self.scroll_duration.to_string()),
            Property::new("max_chars", &self.max_chars.to_string()),
            Property::new("width", &self.width.to_string()),
            Property::new("align", &self.align.to_string()),
        ];

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

impl Default for Text {
    fn default() -> Self {
        Self {
            drawing: true,
            highlight: false,
            color: ARGB::default(),
            highlight_color: ARGB::default(),
            padding_left: 0,
            padding_right: 0,
            y_offset: 0,
            font: Font::default(),
            string: String::new(),
            scroll_duration: 0,
            max_chars: 0,
            width: WidthMode::default(),
            align: TextAlignment::Left,
            background: None,
            shadow: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackgroundProperties {
    pub drawing: bool,
    pub color: ARGB,
    pub border_color: ARGB,
    pub border_width: u32,
    pub height: u32,
    pub corner_radius: u32,
    pub padding_left: i32,
    pub padding_right: i32,
    pub y_offset: i32,
    pub x_offset: i32,
    pub clip: f32,
    pub image: Option<ImageProperties>,
    pub shadow: Option<ShadowProperties>,
}

impl ToSketchybarArgs for BackgroundProperties {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        vec![
            Property::new("drawing", &self.drawing.to_string()),
            Property::new("color", &self.color.to_string()),
            Property::new("border_color", &self.border_color.to_string()),
            Property::new("border_width", &self.border_width.to_string()),
            Property::new("height", &self.height.to_string()),
            Property::new("corner_radius", &self.corner_radius.to_string()),
            Property::new("padding_left", &self.padding_left.to_string()),
            Property::new("padding_right", &self.padding_right.to_string()),
            Property::new("y_offset", &self.y_offset.to_string()),
            Property::new("x_offset", &self.x_offset.to_string()),
            Property::new("clip", &self.clip.to_string()),
        ]
    }
}

impl BackgroundProperties {
    fn new() -> Self {
        Self {
            drawing: false,
            color: ARGB::default(),
            border_color: ARGB::default(),
            border_width: 0,
            height: 0,
            corner_radius: 0,
            padding_left: 0,
            padding_right: 0,
            y_offset: 0,
            x_offset: 0,
            clip: 0.0,
            image: None,
            shadow: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageProperties {
    pub drawing: bool,
    pub scale: f32,
    pub border_color: ARGB,
    pub border_width: u32,
    pub corner_radius: u32,
    pub padding_left: i32,
    pub padding_right: i32,
    pub y_offset: i32,
    pub string: ImageType,
    pub shadow: Option<ShadowProperties>,
}

impl ToSketchybarArgs for ImageProperties {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![
            Property::new("drawing", &self.drawing.to_string()),
            Property::new("scale", &self.scale.to_string()),
            Property::new("border_color", &self.border_color.to_string()),
            Property::new("border_width", &self.border_width.to_string()),
            Property::new("corner_radius", &self.corner_radius.to_string()),
            Property::new("padding_left", &self.padding_left.to_string()),
            Property::new("padding_right", &self.padding_right.to_string()),
            Property::new("y_offset", &self.y_offset.to_string()),
            Property::new("string", &self.string.to_string()),
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

impl ImageProperties {
    fn new(string: ImageType) -> Self {
        Self {
            drawing: false,
            scale: 1.0,
            border_color: ARGB::new(0, 0, 0, 0),
            border_width: 0,
            corner_radius: 0,
            padding_left: 0,
            padding_right: 0,
            y_offset: 0,
            string,
            shadow: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShadowProperties {
    pub drawing: bool,
    pub color: ARGB,
    pub angle: u32,
    pub distance: u32,
}

impl ToSketchybarArgs for ShadowProperties {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        vec![
            Property::new("drawing", &self.drawing.to_string()),
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
            color: ARGB::new(255, 0, 0, 0),
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

#[derive(Debug, Clone)]
pub struct Geometry {
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
    pub background: Option<BackgroundProperties>,
}

impl ToSketchybarArgs for Geometry {
    fn to_sketchybar_args(&self) -> Vec<Property> {
        let mut args = vec![
            Property::new("drawing", &self.drawing.to_string()),
            Property::new("position", &self.position.to_string()),
            Property::new("space", &self.space.to_string()),
            Property::new("display", &self.display.to_string()),
            Property::new("ignore_association", &self.ignore_association.to_string()),
            Property::new("y_offset", &self.y_offset.to_string()),
            Property::new("padding_left", &self.padding_left.to_string()),
            Property::new("padding_right", &self.padding_right.to_string()),
            Property::new("width", &self.width.to_string()),
            Property::new("scroll_texts", &self.scroll_texts.to_string()),
            Property::new("blur_radius", &self.blur_radius.to_string()),
        ];

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
    fn new(position: ComponentPosition) -> Self {
        Self {
            drawing: true,
            position,
            space: 0,
            display: DisplayHandle::Id(vec![0]),
            ignore_association: false,
            y_offset: 0,
            padding_left: 0,
            padding_right: 0,
            width: WidthMode::default(),
            scroll_texts: false,
            blur_radius: 0,
            background: None,
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
            Self::Toggle(value) => write!(f, "{}", value),
            Self::WhenShown => write!(f, "when_shown"),
        }
    }
}

impl Default for UpdateMode {
    fn default() -> Self {
        Self::Toggle(true)
    }
}
