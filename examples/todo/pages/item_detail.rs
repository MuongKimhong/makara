use bevy::prelude::*;
use makara::prelude::*;

fn on_back_btn_click(_clicked: On<Clicked>, mut router: ResMut<Router>) {
    router.navigate("home", ());
}

fn on_mark_complete_btn_click(_clicked: On<Clicked>, mut router: ResMut<Router>, mut btn_q: ButtonQuery) {
    let Some((_, param)) = router.get_current_route() else { return };
    let Some(target_title) = param.get("title") else { return };

    for entity in btn_q.find_by_class("item-btn") {
        let Some(btn) = btn_q.find_by_entity(entity) else { continue };

        if btn.text.value.0 != *target_title {
            continue;
        }

        btn.class.0 = "item-btn completed".to_string();
        router.navigate("home", ());
        return;
    }
}

pub fn setup_item_detail_page(mut commands: Commands) {
    commands.spawn(
        root_!(
            route: "item-detail", class: "page-container";

            on: |page_loaded: On<PageLoaded>, mut text_q: TextQuery| {
                if let Some(title) = page_loaded.param.get("title") {
                    if let Some(text) = text_q.find_by_id("item-title") {
                        text.text.value.0 = title.to_string();
                    }
                }
            };

            [
                column_!(class: "page-container"; [
                    text_!("", id: "item-title", font_size: 25.0, margin_bottom: px(15)),
                    button_!("Back", margin_bottom: px(10); on: on_back_btn_click),
                    button_!("Mark as completed"; on: on_mark_complete_btn_click)
                ])
            ]
        )
    );
}
