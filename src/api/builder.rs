use crate::api::props::*;

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

    fn label_max_chars(mut self, num_chars: u32) -> Self {
        self.item_props_mut()
            .label
            .props
            .get_or_insert_default()
            .max_chars = Some(num_chars);
        self
    }

    fn scroll_texts(mut self, scroll: ToggleState) -> Self {
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
