use bevy::prelude::*;
use makara::prelude::*;

pub fn setup_item_detail_page(mut commands: Commands) {
    commands.spawn((
        root()
            .route("item-detail")
            .class("page-container")
            .build(),

        children![
            (
                column()
                    .class("page-container")
                    .build(),

                children![
                    text("")
                        .id("item-title")
                        .font_size(25.0)
                        .margin_bottom(px(15))
                        .build(),

                    (
                        button("Back").margin_bottom(px(10)).build(),
                        observe(|_clicked: On<Clicked>, mut router: ResMut<Router>| {
                            router.navigate("home", ());
                        })
                    ),
                    (
                        button("Mark as completed").build(),
                        observe(|_clicked: On<Clicked>, mut router: ResMut<Router>, mut btn_q: ButtonQuery| {
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
                        })
                    )
                ]
            )
        ],

        observe(|page_loaded: On<PageLoaded>, mut text_q: TextQuery| {
            if let Some(title) = page_loaded.param.get("title") {
                if let Some(text) = text_q.find_by_id("item-title") {
                    text.text.value.0 = title.to_string();
                }
            }
        })
    ));
}
