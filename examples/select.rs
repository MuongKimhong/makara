use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        root()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            text("").id("selected-choice").build(),

            (
                select(
                    "Select your",
                    &["Ford Falcon XR6", "Ford Mustang GT", "Holden Commodore Z6", "Subaru WRX"]
                )
                .width(px(200))
                .margin_top(px(10))
                .build(),

                observe(|change: On<Change<String>>, mut text_q: TextQuery| {
                    if let Some(text) = text_q.find_by_id("selected-choice") {
                        text.text.value.0 = change.data.clone();
                    }
                })
            )
        ]
    ));
}
