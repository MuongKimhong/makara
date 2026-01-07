use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // bevy needs this
    commands.spawn(Camera2d);

    commands.spawn((
        root()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            (
                button("Add 1 text to scroll container").build(),
                observe(on_add_1_button_click)
            ),

            (
                button("Add 3 text to scroll container")
                    .margin_top(px(5))
                    .margin_bottom(px(5))
                    .build(),

                observe(on_add_3_button_click)
            ),

            (
                button("Remove last item from container").build(),
                observe(on_remove_button_click)
            ),

            (
                scroll()
                    .id("scroll-container")
                    .height(px(200))
                    .width(px(500))
                    .align_items(AlignItems::Center)
                    .background_color(Color::srgb(1.0, 0.1, 0.1))
                    .margin_top(px(10))
                    .build(),

                observe(|scroll: On<Scrolling>| {
                    println!("scrolling top position {:?}", scroll.position);
                })
            )
        ]
    ));
}

fn on_add_1_button_click(_clicked: On<Clicked>, mut scrolls: ScrollQuery) {
    if let Some(mut scroll) = scrolls.find_by_id("scroll-container") {
        scroll.add_child(
            text("Hello world!").font_size(15.0).build()
        );
    }
}

fn on_add_3_button_click(_clicked: On<Clicked>, mut scrolls: ScrollQuery) {
    if let Some(mut scroll) = scrolls.find_by_id("scroll-container") {
        scroll.add_children([
            text("Roses are red").font_size(30.0).build(),
            text("Violets are blue").font_size(30.0).build(),
            text("I Love you :)").font_size(30.0).build(),
        ]);
    }
}

fn on_remove_button_click(_clicked: On<Clicked>, mut scrolls: ScrollQuery) {
    if let Some(mut scroll) = scrolls.find_by_id("scroll-container") {
        scroll.remove_last();
    }
}
