use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use super::*;

/// Custom style for widgets, set via id or class.
#[derive(Debug, Default)]
pub struct Style {
    pub width: Option<Val>,
    pub height: Option<Val>,
    pub display: Option<Display>,
    pub overflow: Option<Overflow>,
    pub left: Option<Val>,
    pub right: Option<Val>,
    pub top: Option<Val>,
    pub bottom: Option<Val>,
    pub align_items: Option<AlignItems>,
    pub justify_items: Option<JustifyItems>,
    pub align_self: Option<AlignSelf>,
    pub justify_self: Option<JustifySelf>,
    pub align_content: Option<AlignContent>,
    pub justify_content: Option<JustifyContent>,
    pub margin: Option<UiRect>,
    pub padding: Option<UiRect>,
    pub border: Option<UiRect>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<Val>,
    pub row_gap: Option<Val>,
    pub column_gap: Option<Val>,
    pub background_color: Option<BackgroundColor>,
    pub border_color: Option<BorderColor>,
    pub border_radius: Option<BorderRadius>,
    pub z_index: Option<ZIndex>,
    pub shadow: Option<BoxShadow>,
    pub color: Option<TextColor>,
    pub font_size: Option<f32>,
    pub layout: Option<TextLayout>
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, value: Val) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: Val) -> Self {
        self.height = Some(value);
        self
    }

    pub fn display(mut self, value: Display) -> Self {
        self.display = Some(value);
        self
    }

    pub fn overflow(mut self, value: Overflow) -> Self {
        self.overflow = Some(value);
        self
    }

    pub fn left(mut self, value: Val) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Val) -> Self {
        self.right = Some(value);
        self
    }

    pub fn top(mut self, value: Val) -> Self {
        self.top = Some(value);
        self
    }

    pub fn bottom(mut self, value: Val) -> Self {
        self.bottom = Some(value);
        self
    }

    pub fn align_items(mut self, value: AlignItems) -> Self {
        self.align_items = Some(value);
        self
    }

    pub fn justify_items(mut self, value: JustifyItems) -> Self {
        self.justify_items = Some(value);
        self
    }

    pub fn align_self(mut self, value: AlignSelf) -> Self {
        self.align_self = Some(value);
        self
    }

    pub fn justify_self(mut self, value: JustifySelf) -> Self {
        self.justify_self = Some(value);
        self
    }

    pub fn align_content(mut self, value: AlignContent) -> Self {
        self.align_content = Some(value);
        self
    }

    pub fn justify_content(mut self, value: JustifyContent) -> Self {
        self.justify_content = Some(value);
        self
    }

    pub fn margin(mut self, value: UiRect) -> Self {
        self.margin = Some(value);
        self
    }

    pub fn padding(mut self, value: UiRect) -> Self {
        self.padding = Some(value);
        self
    }

    pub fn border(mut self, value: UiRect) -> Self {
        self.border = Some(value);
        self
    }

    pub fn flex_direction(mut self, value: FlexDirection) -> Self {
        self.flex_direction = Some(value);
        self
    }

    pub fn flex_wrap(mut self, value: FlexWrap) -> Self {
        self.flex_wrap = Some(value);
        self
    }

    pub fn flex_grow(mut self, value: f32) -> Self {
        self.flex_grow = Some(value);
        self
    }

    pub fn flex_shrink(mut self, value: f32) -> Self {
        self.flex_shrink = Some(value);
        self
    }

    pub fn flex_basis(mut self, value: Val) -> Self {
        self.flex_basis = Some(value);
        self
    }

    pub fn row_gap(mut self, value: Val) -> Self {
        self.row_gap = Some(value);
        self
    }

    pub fn column_gap(mut self, value: Val) -> Self {
        self.column_gap = Some(value);
        self
    }

    pub fn background_color(mut self, value: Color) -> Self {
        self.background_color = Some(BackgroundColor(value));
        self
    }

    pub fn border_color(mut self, value: Color) -> Self {
        self.border_color = Some(BorderColor::all(value));
        self
    }

    pub fn z_index(mut self, value: i32) -> Self {
        self.z_index = Some(ZIndex(value));
        self
    }

    pub fn shadow(mut self, value: ShadowStyle) -> Self {
        self.shadow = Some(BoxShadow(vec![value]));
        self
    }

    pub fn color(mut self, value: Color) -> Self {
        self.color = Some(TextColor(value));
        self
    }

    pub fn font_size(mut self, value: f32) -> Self {
        self.font_size = Some(value);
        self
    }

    pub fn layout(mut self, value: TextLayout) -> Self {
        self.layout = Some(value);
        self
    }
}

#[derive(Resource, Default)]
pub struct CustomStyle {
    pub id_maps: HashMap<String, Style>,
    pub class_maps: HashMap<String, Style>,
    pub(crate) id_changed: HashSet<String>,
    pub(crate) class_changed: HashSet<String>,
    pub(crate) has_changed: bool
}

impl CustomStyle {
    pub fn bind_id(&mut self, key: &str, style: Style) {
        self.id_maps.insert(key.to_string(), style);
        self.id_changed.insert(key.to_string());
        self.has_changed = true;
    }

    pub fn bind_class(&mut self, key: &str, style: Style) {
        self.class_maps.insert(key.to_string(), style);
        self.class_changed.insert(key.to_string());
        self.has_changed = true;
    }
}

fn set_style(widget_style: &mut WidgetStyle, custom_style: &Style) {
    // 1. Background, Border, Shadow
    if let Some(bg) = custom_style.background_color {
        *widget_style.background_color = bg;
    }
    if let Some(bc) = custom_style.border_color {
        *widget_style.border_color = bc;
    }
    if let Some(br) = custom_style.border_radius {
        *widget_style.border_radius = br;
    }
    if let Some(zi) = custom_style.z_index {
        *widget_style.z_index = zi;
    }
    if let Some(sh) = &custom_style.shadow {
        *widget_style.shadow = sh.clone();
    }

    // 2. Node Properties
    let node = &mut *widget_style.node;

    if let Some(v) = custom_style.width { node.width = v; }
    if let Some(v) = custom_style.height { node.height = v; }
    if let Some(v) = custom_style.display { node.display = v; }
    if let Some(v) = custom_style.overflow { node.overflow = v; }

    // Positioning
    if let Some(v) = custom_style.left { node.left = v; }
    if let Some(v) = custom_style.right { node.right = v; }
    if let Some(v) = custom_style.top { node.top = v; }
    if let Some(v) = custom_style.bottom { node.bottom = v; }

    // Flexbox Alignment
    if let Some(v) = custom_style.align_items { node.align_items = v; }
    if let Some(v) = custom_style.justify_items { node.justify_items = v; }
    if let Some(v) = custom_style.align_self { node.align_self = v; }
    if let Some(v) = custom_style.justify_self { node.justify_self = v; }
    if let Some(v) = custom_style.align_content { node.align_content = v; }
    if let Some(v) = custom_style.justify_content { node.justify_content = v; }

    // Spacing
    if let Some(v) = custom_style.margin { node.margin = v; }
    if let Some(v) = custom_style.padding { node.padding = v; }
    if let Some(v) = custom_style.border { node.border = v; }

    // Flex
    if let Some(v) = custom_style.flex_direction { node.flex_direction = v; }
    if let Some(v) = custom_style.flex_wrap { node.flex_wrap = v; }
    if let Some(v) = custom_style.flex_grow { node.flex_grow = v; }
    if let Some(v) = custom_style.flex_shrink { node.flex_shrink = v; }
    if let Some(v) = custom_style.flex_basis { node.flex_basis = v; }
    if let Some(v) = custom_style.row_gap { node.row_gap = v; }
    if let Some(v) = custom_style.column_gap { node.column_gap = v; }
}

fn set_text_style(text_style: &mut ChildText, custom_style: &Style) {
    if let Some(c) = custom_style.color {
        *text_style.color = c;
    }

    if let Some(l) = custom_style.layout {
        *text_style.layout = l;
    }

    if let Some(size) = custom_style.font_size {
        *text_style.font = TextFont::from_font_size(size);
    }
}

pub(crate) fn apply_custom_style_to_button(
    mut button_q: ButtonQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut btn) = button_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut btn.style, style);
                set_text_style(&mut btn.text, style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        if let Some(style) = custom_style.class_maps.get(changed_class) {
            let entities = button_q.find_by_class(changed_class);

            for entity in entities.into_iter() {
                if let Some(mut btn) = button_q.find_by_entity(entity) {
                    set_style(&mut btn.style, style);
                    set_text_style(&mut btn.text, style);
                }
            }
        }
    }
}
