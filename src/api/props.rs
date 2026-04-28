pub use crate::api::types::{
    Argb, ComponentPosition, Font, PopupAlign, Property, SketchyBool, TextAlignment,
    ToSketchybarArgs, ToggleState, WidthMode,
};
use std::fmt::Display;

impl Argb {
    pub fn into_background(self) -> BackgroundProps {
        BackgroundProps {
            color: Some(self),
            ..Default::default()
        }
    }

    pub fn into_text(self) -> Text {
        Text {
            color: Some(self),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShadowProps {
    pub drawing: Option<ToggleState>,
    pub color: Option<Argb>,
    pub angle: Option<u32>,
    pub distance: Option<u32>,
}

impl ShadowProps {
    pub fn drawing(mut self, drawing: ToggleState) -> Self {
        self.drawing = Some(drawing);
        self
    }
    pub fn color(mut self, color: Argb) -> Self {
        self.color = Some(color);
        self
    }
    pub fn angle(mut self, angle: u32) -> Self {
        self.angle = Some(angle);
        self
    }
    pub fn distance(mut self, distance: u32) -> Self {
        self.distance = Some(distance);
        self
    }
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

impl Text {
    pub fn drawing(mut self, drawing: ToggleState) -> Self {
        self.drawing = Some(drawing);
        self
    }
    pub fn highlight(mut self, highlight: ToggleState) -> Self {
        self.highlight = Some(highlight);
        self
    }
    pub fn color(mut self, color: Argb) -> Self {
        self.color = Some(color);
        self
    }
    pub fn highlight_color(mut self, color: Argb) -> Self {
        self.highlight_color = Some(color);
        self
    }
    pub fn padding_left(mut self, padding: u32) -> Self {
        self.padding_left = Some(padding);
        self
    }
    pub fn padding_right(mut self, padding: u32) -> Self {
        self.padding_right = Some(padding);
        self
    }
    pub fn y_offset(mut self, offset: i32) -> Self {
        self.y_offset = Some(offset);
        self
    }
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }
    pub fn scroll_duration(mut self, duration: f32) -> Self {
        self.scroll_duration = Some(duration);
        self
    }
    pub fn max_chars(mut self, max: u32) -> Self {
        self.max_chars = Some(max);
        self
    }
    pub fn width(mut self, width: WidthMode) -> Self {
        self.width = Some(width);
        self
    }
    pub fn align(mut self, align: TextAlignment) -> Self {
        self.align = Some(align);
        self
    }
    pub fn background(mut self, background: BackgroundProps) -> Self {
        self.background = Some(background);
        self
    }
    pub fn shadow(mut self, shadow: ShadowProps) -> Self {
        self.shadow = Some(shadow);
        self
    }
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

impl BackgroundProps {
    pub fn drawing(mut self, drawing: ToggleState) -> Self {
        self.drawing = Some(drawing);
        self
    }
    pub fn color(mut self, color: Argb) -> Self {
        self.color = Some(color);
        self
    }
    pub fn border_color(mut self, color: Argb) -> Self {
        self.border_color = Some(color);
        self
    }
    pub fn border_width(mut self, width: u32) -> Self {
        self.border_width = Some(width);
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
    pub fn corner_radius(mut self, radius: u32) -> Self {
        self.corner_radius = Some(radius);
        self
    }
    pub fn padding_left(mut self, padding: i32) -> Self {
        self.padding_left = Some(padding);
        self
    }
    pub fn padding_right(mut self, padding: i32) -> Self {
        self.padding_right = Some(padding);
        self
    }
    pub fn y_offset(mut self, offset: i32) -> Self {
        self.y_offset = Some(offset);
        self
    }
    pub fn x_offset(mut self, offset: i32) -> Self {
        self.x_offset = Some(offset);
        self
    }
    pub fn blur_radius(mut self, radius: u32) -> Self {
        self.blur_radius = Some(radius);
        self
    }
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }
    pub fn image(mut self, image: ImageProps) -> Self {
        self.image = Some(image);
        self
    }
    pub fn shadow(mut self, shadow: ShadowProps) -> Self {
        self.shadow = Some(shadow);
        self
    }
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

impl ImageProps {
    pub fn drawing(mut self, drawing: ToggleState) -> Self {
        self.drawing = Some(drawing);
        self
    }
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }
    pub fn blur_radius(mut self, radius: u32) -> Self {
        self.blur_radius = Some(radius);
        self
    }
    pub fn border_color(mut self, color: Argb) -> Self {
        self.border_color = Some(color);
        self
    }
    pub fn border_width(mut self, width: u32) -> Self {
        self.border_width = Some(width);
        self
    }
    pub fn corner_radius(mut self, radius: u32) -> Self {
        self.corner_radius = Some(radius);
        self
    }
    pub fn padding_left(mut self, padding: i32) -> Self {
        self.padding_left = Some(padding);
        self
    }
    pub fn padding_right(mut self, padding: i32) -> Self {
        self.padding_right = Some(padding);
        self
    }
    pub fn y_offset(mut self, offset: i32) -> Self {
        self.y_offset = Some(offset);
        self
    }
    pub fn image(mut self, image: ImageType) -> Self {
        self.image = Some(image);
        self
    }
    pub fn shadow(mut self, shadow: ShadowProps) -> Self {
        self.shadow = Some(shadow);
        self
    }
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

impl ImageType {
    pub fn into_props(self) -> ImageProps {
        ImageProps {
            image: Some(self),
            ..Default::default()
        }
    }
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

impl Geometry {
    pub fn drawing(mut self, drawing: ToggleState) -> Self {
        self.drawing = Some(drawing);
        self
    }
    pub fn position(mut self, position: ComponentPosition) -> Self {
        self.position = Some(position);
        self
    }
    pub fn space(mut self, space: u32) -> Self {
        self.space = Some(space);
        self
    }
    pub fn display(mut self, display: DisplayHandle) -> Self {
        self.display = Some(display);
        self
    }
    pub fn ignore_association(mut self, ignore: ToggleState) -> Self {
        self.ignore_association = Some(ignore);
        self
    }
    pub fn y_offset(mut self, offset: i32) -> Self {
        self.y_offset = Some(offset);
        self
    }
    pub fn padding_left(mut self, padding: u32) -> Self {
        self.padding_left = Some(padding);
        self
    }
    pub fn padding_right(mut self, padding: u32) -> Self {
        self.padding_right = Some(padding);
        self
    }
    pub fn width(mut self, width: WidthMode) -> Self {
        self.width = Some(width);
        self
    }
    pub fn blur_radius(mut self, radius: u32) -> Self {
        self.blur_radius = Some(radius);
        self
    }
    pub fn scroll_texts(mut self, scroll: ToggleState) -> Self {
        self.scroll_texts = Some(scroll);
        self
    }
    pub fn background(mut self, background: BackgroundProps) -> Self {
        self.background = Some(background);
        self
    }
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

impl PopupProps {
    pub fn drawing(mut self, drawing: ToggleState) -> Self {
        self.drawing = Some(drawing);
        self
    }
    pub fn horizontal(mut self, horizontal: ToggleState) -> Self {
        self.horizontal = Some(horizontal);
        self
    }
    pub fn topmost(mut self, topmost: ToggleState) -> Self {
        self.topmost = Some(topmost);
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
    pub fn blur_radius(mut self, radius: u32) -> Self {
        self.blur_radius = Some(radius);
        self
    }
    pub fn y_offset(mut self, offset: i32) -> Self {
        self.y_offset = Some(offset);
        self
    }
    pub fn align(mut self, align: PopupAlign) -> Self {
        self.align = Some(align);
        self
    }
    pub fn background<F>(mut self, f: F) -> Self
    where
        F: FnOnce(BackgroundProps) -> BackgroundProps,
    {
        self.background = Some(f(self.background.take().unwrap_or_default()));
        self
    }
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
