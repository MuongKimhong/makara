use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        root_!(
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;

            [
                button_!("Add 1 text to scroll container"; on: on_add_1_button_click),
                button_!("Add 3 text to scroll container"; on: on_add_3_button_click),
                button_!("Remove last item from container"; on: on_remove_button_click),
                scroll_!(
                    id: "scroll-container",
                    align_items: AlignItems::Center,
                    height: px(500),
                    width: px(500),
                    margin_top: px(10);

                    on: |scroll: On<Scrolling>| {
                        println!("scrolling top position {:?}", scroll.position);
                    }
                )
            ]
        )
    );
}

fn on_add_1_button_click(_clicked: On<Clicked>, mut scrolls: ScrollQuery) {
    if let Some(mut scroll) = scrolls.find_by_id("scroll-container") {
        scroll.add_child(
            text_!("hello world!", font_size: 15.0)
        );
    }
}

fn on_add_3_button_click(_clicked: On<Clicked>, mut scrolls: ScrollQuery) {
    if let Some(mut scroll) = scrolls.find_by_id("scroll-container") {
        scroll.add_children([
            text_!("Roses are red", font_size: 30.0),
            text_!("Violets are blue", font_size: 30.0),
            text_!("I Love You", font_size: 30.0)
        ]);
    }
}

fn on_remove_button_click(_clicked: On<Clicked>, mut scrolls: ScrollQuery) {
    if let Some(mut scroll) = scrolls.find_by_id("scroll-container") {
        scroll.remove_last();
    }
}
