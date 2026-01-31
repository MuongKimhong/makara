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

pub fn get_bulma_bg_colors(class_name: &str) -> Option<(Color, Color)> {
    match class_name {
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
        _ => None,
    }
}

pub(crate) fn process_button_built_in_color_class_bg_only(
    class: &Class,
    bg: &mut BackgroundColor
) {
    for class_name in class.class_list() {
        if let Some((base_color, _)) = get_bulma_bg_colors(class_name.as_str()) {
            bg.0 = base_color;
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
    color: &mut TextColor
) {
    for class_name in class.class_list() {
        if let Some((base_color, _)) = get_bulma_bg_colors(class_name.as_str()) {
            color.0 = base_color;
        }
    }
}
