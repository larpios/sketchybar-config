use crate::api::props::*;

pub trait ItemBuilder: Sized {
    fn item_props_mut(&mut self) -> &mut ItemProps;

    fn apply_if<F>(self, condition: bool, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if condition { f(self) } else { self }
    }

    fn geometry<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Geometry) -> Geometry,
    {
        self.item_props_mut().geometry = f(std::mem::take(&mut self.item_props_mut().geometry));
        self
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
        self.item_props_mut().geometry.width = Some(WidthMode::Fixed(width));
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

    fn icon_props<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Text) -> Text,
    {
        self.item_props_mut().icon.props = Some(f(self
            .item_props_mut()
            .icon
            .props
            .take()
            .unwrap_or_default()));
        self
    }

    fn label(mut self, label: &str) -> Self {
        self.item_props_mut().label.label = Some(label.to_string());
        self
    }

    fn label_props<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Text) -> Text,
    {
        self.item_props_mut().label.props = Some(f(self
            .item_props_mut()
            .label
            .props
            .take()
            .unwrap_or_default()));
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

    fn background<F>(mut self, f: F) -> Self
    where
        F: FnOnce(BackgroundProps) -> BackgroundProps,
    {
        self.item_props_mut().geometry.background = Some(f(self
            .item_props_mut()
            .geometry
            .background
            .take()
            .unwrap_or_default()));
        self
    }

    fn popup<F>(mut self, f: F) -> Self
    where
        F: FnOnce(PopupProps) -> PopupProps,
    {
        self.item_props_mut().popup =
            Some(f(self.item_props_mut().popup.take().unwrap_or_default()));
        self
    }

    fn text<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Text) -> Text,
    {
        self.item_props_mut().text = Some(f(self.item_props_mut().text.take().unwrap_or_default()));
        self
    }
}
