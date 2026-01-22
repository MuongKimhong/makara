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

fn spawn_default_items(commands: &mut Commands, items: &Vec<TodoItem>) -> Vec<Entity> {
    let mut item_entities: Vec<Entity> = Vec::new();

    for item in items.iter() {
        let title = item.title.clone();

        let entity = commands
            .spawn(
                button_!(&item.title, class: "item-btn"; on: move |_clicked: On<Clicked>, mut router: ResMut<Router>| {
                    router.navigate("item-detail", Param::new().value("title", &title.clone()));
                })
            )
            .id();

        item_entities.push(entity);
    }
    item_entities
}

pub fn setup_home_page(
    mut commands: Commands,
    mut todo_list: ResMut<TodoList>,
) {
    todo_list.items = create_default_items();

    let item_entities = spawn_default_items(&mut commands, &todo_list.items);

    let items_container = commands.spawn(scroll_!(id: "items-container"))
        .add_children(&item_entities)
        .id();

    commands.spawn(
        root_!(route: "home", class: "page-container"; [
            text_!("To Do", font_size: 25.0)
        ])
    )
    .add_child(items_container);
}

pub fn setup_home_styles(mut style: ResMut<CustomStyle>) {
    style.bind_id(
        "items-container",
        Style::new()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .width(px(400))
            .height(px(400))
            .padding(px(5))
    );

    style.bind_class(
        "item-btn",
        Style::new()
            .no_shadow()
            .border(px(2))
            .width(percent(100))
            .border_color("blue")
            .background_color("transparent")
            .padding(px(20))
            .margin_y(px(10))
            .font_size(16.0)
    );

    style.bind_class(
        "completed",
        Style::new()
            .border_color("green")
    );
}
