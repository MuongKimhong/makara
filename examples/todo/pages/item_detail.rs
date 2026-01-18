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
                    text("").id("item-title").font_size(25.0).build(),

                    (
                        button("Back").build(),
                        observe(|_clicked: On<Clicked>, mut router: ResMut<Router>| {
                            router.navigate("home", ());
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
