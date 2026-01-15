use bevy::prelude::*;
use cosmic_text::Edit;
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
    pub active_color: Option<Color>,
    pub background_color: Option<BackgroundColor>,
    pub border_color: Option<BorderColor>,
    pub border_radius: Option<BorderRadius>,
    pub z_index: Option<ZIndex>,
    pub shadow: Option<BoxShadow>,
    pub color: Option<TextColor>,
    pub font_size: Option<f32>,
    pub layout: Option<TextLayout>,

    // specifically for circular
    pub spin_color: Option<Color>,

    // specifically for progress bar
    pub progress_color: Option<Color>,

    // sepcifically for text input
    pub selection_color: Option<Color>,
    pub placeholder_color: Option<Color>
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

    pub fn active_color(mut self, value: impl IntoColor) -> Self {
        self.active_color = Some(value.into_color());
        self
    }

    pub fn background_color(mut self, value: impl IntoColor) -> Self {
        self.background_color = Some(BackgroundColor(value.into_color()));
        self
    }

    pub fn border_color(mut self, value: impl IntoColor) -> Self {
        self.border_color = Some(BorderColor::all(value.into_color()));
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

    pub fn color(mut self, value: impl IntoColor) -> Self {
        self.color = Some(TextColor(value.into_color()));
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

    pub fn spin_color(mut self, value: impl IntoColor) -> Self {
        self.spin_color = Some(value.into_color());
        self
    }

    pub fn progress_color(mut self, value: impl IntoColor) -> Self {
        self.progress_color = Some(value.into_color());
        self
    }

    pub fn placeholder_color(mut self, value: impl IntoColor) -> Self {
        self.placeholder_color = Some(value.into_color());
        self
    }

    pub fn selection_color(mut self, value: impl IntoColor) -> Self {
        self.selection_color = Some(value.into_color());
        self
    }
}

#[derive(Resource)]
pub struct CustomStyle {
    pub id_maps: HashMap<String, Style>,
    pub class_maps: HashMap<String, Style>,
    pub(crate) id_changed: HashSet<String>,
    pub(crate) class_changed: HashSet<String>,
    pub(crate) has_changed: bool,
    pub(crate) can_set_unchanged: bool
}

impl Default for CustomStyle {
    fn default() -> Self {
        Self {
            id_maps: HashMap::new(),
            class_maps: HashMap::new(),
            id_changed: HashSet::new(),
            class_changed: HashSet::new(),
            has_changed: false,
            can_set_unchanged: true,
        }
    }
}

impl CustomStyle {
    pub fn bind_id(&mut self, key: &str, style: Style) {
        self.id_maps.insert(key.to_string(), style);
        self.id_changed.insert(key.to_string());
        self.has_changed = true;

        if let Some(base_id) = key.split("::").next() {
            if base_id != key {
                self.id_changed.insert(base_id.to_string());
            }
        }
    }

    pub fn bind_class(&mut self, key: &str, style: Style) {
        self.class_maps.insert(key.to_string(), style);
        self.class_changed.insert(key.to_string());
        self.has_changed = true;

        if let Some(base_class) = key.split("::").next() {
            if base_class != key {
                self.class_changed.insert(base_class.to_string());
            }
        }
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
    if let Some(v) = custom_style.border_radius { node.border_radius= v; }

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

pub(crate) fn apply_custom_style_to_checkbox(
    mut checkbox_q: CheckboxQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut checkbox) = checkbox_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut checkbox.style, style);
                set_text_style(&mut checkbox.text, style);
            }

            let btn_id = format!("{}::checkbox-button", changed_id);

            if let Some(btn_style) = custom_style.id_maps.get(&btn_id) {
                set_style(&mut checkbox.button_style, btn_style);

                if let Some(active_color) = btn_style.active_color {
                    checkbox.button_active_color.0 = active_color;
                }
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let style = custom_style.class_maps.get(changed_class);

        let btn_class = format!("{}::checkbox-button", changed_class);
        let btn_style = custom_style.class_maps.get(&btn_class);

        let entities = checkbox_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut checkbox) = checkbox_q.find_by_entity(entity) {
                if let Some(style) = style {
                    set_style(&mut checkbox.style, style);
                    set_text_style(&mut checkbox.text, style);
                }

                if let Some(btn_style) = btn_style {
                    set_style(&mut checkbox.button_style, btn_style);

                    if let Some(active_color) = btn_style.active_color {
                        checkbox.button_active_color.0 = active_color;
                    }
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_radio(
    mut radio_q: RadioQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut radio) = radio_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut radio.style, style);
                set_text_style(&mut radio.text, style);
            }

            let btn_id = format!("{}::radio-button", changed_id);

            if let Some(btn_style) = custom_style.id_maps.get(&btn_id) {
                set_style(&mut radio.button_style, btn_style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let style = custom_style.class_maps.get(changed_class);

        let btn_class = format!("{}::radio-button", changed_class);
        let btn_style = custom_style.class_maps.get(&btn_class);

        let entities = radio_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut radio) = radio_q.find_by_entity(entity) {
                if let Some(style) = style {
                    set_style(&mut radio.style, style);
                    set_text_style(&mut radio.text, style);
                }

                if let Some(btn_style) = btn_style {
                    set_style(&mut radio.button_style, btn_style);
                }
            }
        }
    }
}


pub(crate) fn apply_custom_style_to_slider(
    mut slider_q: SliderQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        let thumb_id = format!("{}::thumb", changed_id);

        if let Some(mut slider) = slider_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut slider.style, style);
            }

            if let Some(thumb_style) = custom_style.id_maps.get(&thumb_id) {
                set_style(&mut slider.thumb_style, thumb_style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let style = custom_style.class_maps.get(changed_class);

        let thumb_class = format!("{}::thumb", changed_class);
        let thumb_style = custom_style.class_maps.get(&thumb_class);

        let entities = slider_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut slider) = slider_q.find_by_entity(entity) {
                if let Some(style) = style {
                    set_style(&mut slider.style, style);
                }

                if let Some(thumb_style) = thumb_style {
                    set_style(&mut slider.thumb_style, thumb_style);
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_circular(
    mut circular_q: CircularQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut circular) = circular_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut circular.style, style);

                if let Some(color) = style.spin_color {
                    circular.spin_color.0 = color;
                }
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let entities = circular_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut circular) = circular_q.find_by_entity(entity) {
                if let Some(style) = custom_style.class_maps.get(changed_class) {
                    set_style(&mut circular.style, style);

                    if let Some(color) = style.spin_color {
                        circular.spin_color.0 = color;
                    }
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_progress_bar(
    mut progress_q: ProgressBarQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut progress) = progress_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut progress.style, style);

                if let Some(color) = style.progress_color {
                    progress.color.0 = color;
                }
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let entities = progress_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut progress) = progress_q.find_by_entity(entity) {
                if let Some(style) = custom_style.class_maps.get(changed_class) {
                    set_style(&mut progress.style, style);

                    if let Some(color) = style.progress_color {
                        progress.color.0 = color;
                    }
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_root(
    mut root_q: RootQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut root) = root_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut root.style, style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let entities = root_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut root) = root_q.find_by_entity(entity) {
                if let Some(style) = custom_style.class_maps.get(changed_class) {
                    set_style(&mut root.style, style);
                }
            }
        }
    }
}


pub(crate) fn apply_custom_style_to_column(
    mut column_q: ColumnQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut column) = column_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut column.style, style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let entities = column_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut column) = column_q.find_by_entity(entity) {
                if let Some(style) = custom_style.class_maps.get(changed_class) {
                    set_style(&mut column.style, style);
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_row(
    mut row_q: RowQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut row) = row_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut row.style, style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let entities = row_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut row) = row_q.find_by_entity(entity) {
                if let Some(style) = custom_style.class_maps.get(changed_class) {
                    set_style(&mut row.style, style);
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_radio_group(
    mut radio_group_q: RadioGroupQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut rg) = radio_group_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut rg.style, style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let entities = radio_group_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut rg) = radio_group_q.find_by_entity(entity) {
                if let Some(style) = custom_style.class_maps.get(changed_class) {
                    set_style(&mut rg.style, style);
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_dropdown(
    mut dropdown_q: DropdownQuery,
    mut custom_style: ResMut<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }
    let mut should_set_unchanged = true;

    for changed_id in custom_style.id_changed.iter() {
        if !dropdown_q.id_match(changed_id) {
            continue;
        }

        if let Some(mut dropdown) = dropdown_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut dropdown.style, style);
                set_text_style(&mut dropdown.text, style);
            }

            let overlay_id = format!("{}::overlay", changed_id);

            if let Some(overlay_style) = custom_style.id_maps.get(&overlay_id) {
                set_style(&mut dropdown.overlay_style, overlay_style);
            }
            should_set_unchanged = true;
        }
        else {
            should_set_unchanged = false;
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let style = custom_style.class_maps.get(changed_class);

        let overlay_class = format!("{}::overlay", changed_class);
        let overlay_style = custom_style.class_maps.get(&overlay_class);

        let entities = dropdown_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut dropdown) = dropdown_q.find_by_entity(entity) {
                if let Some(style) = style {
                    set_style(&mut dropdown.style, style);
                    set_text_style(&mut dropdown.text, style);
                }

                if let Some(overlay_style) = overlay_style {
                    set_style(&mut dropdown.overlay_style, overlay_style);
                }
            }
        }
    }

    custom_style.can_set_unchanged = should_set_unchanged;
}

pub(crate) fn apply_custom_style_to_select(
    mut select_q: SelectQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }
    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut select) = select_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut select.style, style);
                set_text_style(&mut select.placeholder, style);
            }

            let overlay_id = format!("{}::overlay", changed_id);
            let arrow_id = format!("{}::arrow", changed_id);

            if let Some(overlay_style) = custom_style.id_maps.get(&overlay_id) {
                set_style(&mut select.overlay_style, overlay_style);
            }

            if let Some(arrow_style) = custom_style.id_maps.get(&arrow_id) {
                set_text_style(&mut select.arrow, arrow_style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let style = custom_style.class_maps.get(changed_class);

        let overlay_class = format!("{}::overlay", changed_class);
        let overlay_style = custom_style.class_maps.get(&overlay_class);

        let arrow_class = format!("{}::arrow", changed_class);
        let arrow_style = custom_style.class_maps.get(&arrow_class);

        let entities = select_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut select) = select_q.find_by_entity(entity) {
                if let Some(style) = style {
                    set_style(&mut select.style, style);
                    set_text_style(&mut select.placeholder, style);
                }

                if let Some(overlay_style) = overlay_style {
                    set_style(&mut select.overlay_style, overlay_style);
                }

                if let Some(arrow_style) = arrow_style {
                    set_text_style(&mut select.arrow, arrow_style);
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_text_input(
    mut input_q: TextInputQuery,
    mut custom_style: ResMut<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    let mut should_set_unchanged = None;

    for changed_id in custom_style.id_changed.iter() {
        if !input_q.id_match(changed_id) {
            continue;
        }

        if let Some(mut input) = input_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut input.style, style);

                if let Some(font_size) = style.font_size {
                    input.editor_style.font_size = font_size;
                }

                if let Some(color) = style.color {
                    input.editor_style.text_color = color.0;
                }

                if let Some(color) = style.selection_color {
                    input.editor_style.selection_color = color;
                }

                if let Some(color) = style.placeholder_color {
                    input.editor_style.placeholder_color = color;
                }
            }

            let caret_id = format!("{}::caret", changed_id);
            if let Some(caret_style) = custom_style.id_maps.get(&caret_id) {
                set_style(&mut input.caret_style, caret_style);
            }
            input.editor.editor.set_redraw(true);
            should_set_unchanged = Some(true);
        }
        else {
            should_set_unchanged = Some(false);
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let style = custom_style.class_maps.get(changed_class);

        let caret_class = format!("{}::caret", changed_class);
        let caret_style = custom_style.class_maps.get(&caret_class);

        let entities = input_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut input) = input_q.find_by_entity(entity) {
                if let Some(style) = style {
                    set_style(&mut input.style, style);

                    if let Some(font_size) = style.font_size {
                        input.editor_style.font_size = font_size;
                    }

                    if let Some(color) = style.color {
                        input.editor_style.text_color = color.0;
                    }

                    if let Some(color) = style.selection_color {
                        input.editor_style.selection_color = color;
                    }

                    if let Some(color) = style.placeholder_color {
                        input.editor_style.placeholder_color = color;
                    }
                    input.editor.editor.set_redraw(true);
                }

                if let Some(caret_style) = caret_style {
                    set_style(&mut input.caret_style, caret_style);
                }
            }
        }
    }

    if let Some(flag) = should_set_unchanged {
        custom_style.can_set_unchanged = flag;
    }
}

pub(crate) fn apply_custom_style_to_scroll(
    mut scroll_q: ScrollQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut scroll) = scroll_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut scroll.style, style);
            }

            let bar_id = format!("{}::scroll-bar", changed_id);

            if let Some(bar_style) = custom_style.id_maps.get(&bar_id) {
                set_style(&mut scroll.bar_style, bar_style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let style = custom_style.class_maps.get(changed_class);

        let bar_class = format!("{}::scroll-bar", changed_class);
        let bar_style = custom_style.class_maps.get(&bar_class);

        let entities = scroll_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut scroll) = scroll_q.find_by_entity(entity) {
                if let Some(style) = style {
                    set_style(&mut scroll.style, style);
                }

                if let Some(bar_style) = bar_style {
                    set_style(&mut scroll.bar_style, bar_style);
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_image(
    mut image_q: ImageQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut row) = image_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut row.style, style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        let entities = image_q.find_by_class(changed_class);

        for entity in entities.into_iter() {
            if let Some(mut image) = image_q.find_by_entity(entity) {
                if let Some(style) = custom_style.class_maps.get(changed_class) {
                    set_style(&mut image.style, style);
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_link(
    mut link_q: LinkQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut link) = link_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut link.style, style);
                set_text_style(&mut link.text, style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        if let Some(style) = custom_style.class_maps.get(changed_class) {
            let entities = link_q.find_by_class(changed_class);

            for entity in entities.into_iter() {
                if let Some(mut link) = link_q.find_by_entity(entity) {
                    set_style(&mut link.style, style);
                    set_text_style(&mut link.text, style);
                }
            }
        }
    }
}

pub(crate) fn apply_custom_style_to_text(
    mut text_q: TextQuery,
    custom_style: Res<CustomStyle>
) {
    if !custom_style.has_changed {
        return;
    }

    for changed_id in custom_style.id_changed.iter() {
        if let Some(mut text) = text_q.find_by_id(changed_id) {
            if let Some(style) = custom_style.id_maps.get(changed_id) {
                set_style(&mut text.style, style);
                set_text_style(&mut text.text, style);
            }
        }
    }

    for changed_class in custom_style.class_changed.iter() {
        if let Some(style) = custom_style.class_maps.get(changed_class) {
            let entities = text_q.find_by_class(changed_class);

            for entity in entities.into_iter() {
                if let Some(mut text) = text_q.find_by_entity(entity) {
                    set_style(&mut text.style, style);
                    set_text_style(&mut text.text, style);
                }
            }
        }
    }
}

// pub(crate) fn apply_custom_style_to_modal(
//     mut modal_q: ModalQuery,
//     custom_style: Res<CustomStyle>
// ) {
//     if !custom_style.has_changed {
//         return;
//     }

//     for changed_id in custom_style.id_changed.iter() {
//         if !modal_q.id_match(changed_id) {
//             continue;
//         }

//         if let Some(mut modal) = modal_q.find_by_id(changed_id) {
//             if let Some(style) = custom_style.id_maps.get(changed_id) {
//                 set_style(&mut modal.style, style);
//             }

//             let backdrop_id = format!("{}::backdrop", changed_id);

//             if let Some(backdrop_style) = custom_style.id_maps.get(&backdrop_id) {
//                 set_style(&mut modal.backdrop_style, backdrop_style);
//             }
//         }
//         else {
//         }
//     }

//     for changed_class in custom_style.class_changed.iter() {
//         if let Some(style) = custom_style.class_maps.get(changed_class) {
//             let backdrop_class = format!("{}::backdrop", changed_class);
//             let backdrop_style = custom_style.class_maps.get(&backdrop_class);

//             let entities = modal_q.find_by_class(changed_class);

//             for entity in entities.into_iter() {
//                 if let Some(mut modal) = modal_q.find_by_entity(entity) {
//                     set_style(&mut modal.style, style);

//                     if let Some(backdrop_style) = backdrop_style {
//                         set_style(&mut modal.backdrop_style, backdrop_style);
//                     }
//                 }
//             }
//         }
//     }
// }

pub(crate) fn set_style_unchanged(mut custom_style: ResMut<CustomStyle>) {
    if custom_style.has_changed && custom_style.can_set_unchanged {
        custom_style.has_changed = false;
        custom_style.can_set_unchanged = true;
        custom_style.id_changed.clear();
        custom_style.class_changed.clear();
    }
}
