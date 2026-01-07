use bevy::prelude::*;
use bevy::asset::{io::AssetSourceId, AssetPath};
use std::path::Path;

use crate::widgets::WidgetFocus;

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
