pub use crate::api::types::{
    Argb, ComponentPosition, Font, PopupAlign, Property, SketchyBool, TextAlignment,
    ToSketchybarArgs, ToggleState, WidthMode,
};
use std::fmt::Display;

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
    pub scroll_texts: Option<ToggleState>,
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
