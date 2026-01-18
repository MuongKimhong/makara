use bevy::prelude::*;
use makara::prelude::*;
use super::*;

fn create_default_items() -> Vec<TodoItem> {
    vec![
        TodoItem::new("Write documentation"),
        TodoItem::new("Provide Examples"),
        TodoItem::new("Provide Live demo via WASM"),
        TodoItem::new("Publish to rust registery"),
    ]
}

pub fn setup_home_page(
    mut commands: Commands,
    mut todo_list: ResMut<TodoList>,
) {
    todo_list.items = create_default_items();

    let mut item_entities: Vec<Entity> = Vec::new();

    for item in todo_list.items.iter() {
        let title = item.title.clone();

        let entity = commands
            .spawn((
                button(&item.title).build(),
                observe(move |_clicked: On<Clicked>, mut router: ResMut<Router>| {
                    router.navigate("item-detail", Param::new().value("title", &title.clone()));
                })
            ))
            .id();

        item_entities.push(entity);
    }

    commands.spawn((
        root()
            .route("home")
            .class("page-container")
            .build(),

        children![
            text("To do").font_size(20.0).build()
        ]
    ))
    .add_children(&item_entities);
}
