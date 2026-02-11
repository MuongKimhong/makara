use bevy::prelude::*;
use bevy::asset::{io::AssetSourceId, AssetPath};
use std::path::Path;

use crate::widgets::{WidgetFocus, Class};
use crate::colors::*;

pub(crate) fn get_embedded_asset_path(file_path: &'static str) -> AssetPath<'static> {
    // path: relative to embedded_assets dir

    let path = Path::new("makara").join(file_path);
    let source = AssetSourceId::from("embedded");
    AssetPath::from_path(&path).with_source(source).into_owned()
}

pub(crate) fn update_focus_state_for_widgets_on_click(
    clicked_entity: Entity,
    widgets: &mut Query<(Entity, &mut WidgetFocus)>
) {
    for (entity, mut w_focus) in widgets.iter_mut() {
        if entity == clicked_entity {
            w_focus.0 = true;
        }
        else {
            w_focus.0 = false;
        }
    }
}

pub(crate) fn process_built_in_spacing_class(class: &Class, node: &mut Node) {
    for c in class.class_list() {
        let parts: Vec<&str> = c.split('-').collect();
        if parts.len() != 2 { continue; }

        let prefix = parts[0];
        let value_str = parts[1];

        let val = match value_str {
            "0" => px(0.0),
            "1" => px(4.0),
            "2" => px(8.0),
            "3" => px(12.0),
            "4" => px(16.0),
            "5" => px(24.0),
            "6" => px(48.0),
            "auto" => auto(),
            _ => continue,
        };

        match prefix {
            // Margins
            "m"  => node.margin = UiRect::all(val),
            "mt" => node.margin.top = val,
            "mr" => node.margin.right = val,
            "mb" => node.margin.bottom = val,
            "ml" => node.margin.left = val,
            "mx" => { node.margin.left = val; node.margin.right = val; },
            "my" => { node.margin.top = val; node.margin.bottom = val; },

            // Padding
            "p"  => node.padding = UiRect::all(val),
            "pt" => node.padding.top = val,
            "pr" => node.padding.right = val,
            "pb" => node.padding.bottom = val,
            "pl" => node.padding.left = val,
            "px" => { node.padding.left = val; node.padding.right = val; },
            "py" => { node.padding.top = val; node.padding.bottom = val; },

            _ => {}
        }
    }
}

pub(crate) fn process_built_in_alignment_class(class: &Class, node: &mut Node) {
    for c in class.class_list() {
        match c.as_str() {
            // Justify Content (main axis alignment)
            "justify-center" | "justify-content-center" => {
                node.justify_content = JustifyContent::Center;
            },
            "justify-start" | "justify-content-start" => {
                node.justify_content = JustifyContent::Start;
            },
            "justify-end" | "justify-content-end" => {
                node.justify_content = JustifyContent::End;
            },
            "justify-between" | "justify-content-between" => {
                node.justify_content = JustifyContent::SpaceBetween;
            },
            "justify-around" | "justify-content-around" => {
                node.justify_content = JustifyContent::SpaceAround;
            },
            "justify-evenly" | "justify-content-evenly" => {
                node.justify_content = JustifyContent::SpaceEvenly;
            },
            "justify-stretch" | "justify-content-stretch" => {
                node.justify_content = JustifyContent::Stretch;
            },

            // Align Items (cross axis alignment)
            "align-center" | "align-items-center" => {
                node.align_items = AlignItems::Center;
            },
            "align-start" | "align-items-start" => {
                node.align_items = AlignItems::Start;
            },
            "align-end" | "align-items-end" => {
                node.align_items = AlignItems::End;
            },
            "align-stretch" | "align-items-stretch" => {
                node.align_items = AlignItems::Stretch;
            },
            "align-baseline" | "align-items-baseline" => {
                node.align_items = AlignItems::Baseline;
            },

            // Align Content (multi-line alignment)
            "align-content-center" => {
                node.align_content = AlignContent::Center;
            },
            "align-content-start" => {
                node.align_content = AlignContent::Start;
            },
            "align-content-end" => {
                node.align_content = AlignContent::End;
            },
            "align-content-stretch" => {
                node.align_content = AlignContent::Stretch;
            },
            "align-content-between" => {
                node.align_content = AlignContent::SpaceBetween;
            },
            "align-content-around" => {
                node.align_content = AlignContent::SpaceAround;
            },
            "align-content-evenly" => {
                node.align_content = AlignContent::SpaceEvenly;
            },

            _ => {}
        }
    }
}

pub fn get_bulma_bg_colors(class_name: &str) -> Option<(Color, Color)> {
    match class_name {
        "is-light"        => Some((LIGHT_BG, LIGHT_BG_HOVER)),
        "is-dark"         => Some((DARK_BG, DARK_BG_HOVER)),
        "is-primary"      => Some((PRIMARY_BG, PRIMARY_BG_HOVER)),
        "is-primary-dark" => Some((PRIMARY_DARK_BG, PRIMARY_DARK_BG_HOVER)),
        "is-link"         => Some((LINK_BG, LINK_BG_HOVER)),
        "is-link-dark"    => Some((LINK_DARK_BG, LINK_DARK_BG_HOVER)),
        "is-info"         => Some((INFO_BG, INFO_BG_HOVER)),
        "is-info-dark"    => Some((INFO_DARK_BG, INFO_DARK_BG_HOVER)),
        "is-success"      => Some((SUCCESS_BG, SUCCESS_BG_HOVER)),
        "is-success-dark" => Some((SUCCESS_DARK_BG, SUCCESS_DARK_BG_HOVER)),
        "is-warning"      => Some((WARNING_BG, WARNING_BG_HOVER)),
        "is-warning-dark" => Some((WARNING_DARK_BG, WARNING_DARK_BG_HOVER)),
        "is-danger"       => Some((DANGER_BG, DANGER_BG_HOVER)),
        "is-danger-dark"  => Some((DANGER_DARK_BG, DANGER_DARK_BG_HOVER)),
        _ => None,
    }
}

pub fn get_bulma_text_colors(class_name: &str) -> Option<Color> {
    match class_name {
        "is-primary"      => Some(PRIMARY_TEXT),
        "is-primary-dark" => Some(PRIMARY_DARK_TEXT),
        "is-link"         => Some(LINK_TEXT),
        "is-link-dark"    => Some(LINK_DARK_TEXT),
        "is-info"         => Some(INFO_TEXT),
        "is-info-dark"    => Some(INFO_DARK_TEXT),
        "is-success"      => Some(SUCCESS_TEXT),
        "is-success-dark" => Some(SUCCESS_DARK_TEXT),
        "is-warning"      => Some(WARNING_TEXT),
        "is-warning-dark" => Some(WARNING_DARK_TEXT),
        "is-danger"       => Some(DANGER_TEXT),
        "is-danger-dark"  => Some(DANGER_DARK_TEXT),
        "is-light"        => Some(LIGHT_TEXT),
        "is-dark"         => Some(DARK_TEXT),
        _ => None,
    }
}

pub fn get_bulma_placeholder_colors(class_name: &str) -> Option<Color> {
    match class_name {
        "is-primary"      => Some(PRIMARY_PLACEHOLDER),
        "is-primary-dark" => Some(PRIMARY_DARK_PLACEHOLDER),
        "is-link"         => Some(LINK_PLACEHOLDER),
        "is-link-dark"    => Some(LINK_DARK_PLACEHOLDER),
        "is-info"         => Some(INFO_PLACEHOLDER),
        "is-info-dark"    => Some(INFO_DARK_PLACEHOLDER),
        "is-success"      => Some(SUCCESS_PLACEHOLDER),
        "is-success-dark" => Some(SUCCESS_DARK_PLACEHOLDER),
        "is-warning"      => Some(WARNING_PLACEHOLDER),
        "is-warning-dark" => Some(WARNING_DARK_PLACEHOLDER),
        "is-danger"       => Some(DANGER_PLACEHOLDER),
        "is-danger-dark"  => Some(DANGER_DARK_PLACEHOLDER),
        _ => None,
    }
}

pub(crate) fn process_built_in_color(
    class: &Class,
    color: &mut Color
) {
    for class_name in class.class_list() {
        if let Some((base_color, _)) = get_bulma_bg_colors(class_name.as_str()) {
            *color = base_color;
        }
    }
}

pub(crate) fn process_button_built_in_color_class_hover_only(
    class: &Class,
    bg: &mut BackgroundColor
) {
    for class_name in class.class_list() {
        if let Some((_, hover_color)) = get_bulma_bg_colors(class_name.as_str()) {
            bg.0 = hover_color;
        }
    }
}

pub(crate) fn process_text_built_in_color_class(
    class: &Class,
    color: &mut Color
) {
    for class_name in class.class_list() {
        if let Some(base_color) = get_bulma_text_colors(class_name.as_str()) {
            *color = base_color;
        }
    }
}

pub(crate) fn process_placeholder_text_built_in_color_class(
    class: &Class,
    color: &mut Color
) {
    for class_name in class.class_list() {
        if let Some(base_color) = get_bulma_placeholder_colors(class_name.as_str()) {
            *color = base_color;
        }
    }
}
