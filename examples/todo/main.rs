pub mod pages;

pub use pages::*;

use makara::prelude::*;
use bevy::prelude::*;

#[derive(Debug)]
pub struct TodoItem {
    pub title: String,
    pub completed: bool
}

impl TodoItem {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            completed: false
        }
    }
}

// Use bevy's resource as global state
#[derive(Resource, Debug, Default)]
pub struct TodoList {
    pub items: Vec<TodoItem>
}

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, (
            setup_home_page,
            setup_home_styles,
            setup_item_detail_page,
            setup_styles,
            setup_routes
        ))
        .insert_resource(TodoList::default())
        .run();
}

fn setup_styles(mut style: ResMut<CustomStyle>) {
    style.bind_class(
        "page-container",
        Style::new()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
    );
}

fn setup_routes(mut router: ResMut<Router>) {
    router.register_routes(["home", "item-detail"]);
}
