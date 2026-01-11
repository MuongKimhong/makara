use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // put white text under root, so that it's only visible
    // when root background alpha is low.
    commands.spawn(
        text("This is underneath Text")
            .color(Color::srgb(1.0, 1.0, 1.0))
            .build()
    );

    commands.spawn((
        root()
            .id("root")
            .background_color(Color::srgba(1.0, 0.5, 0.5, 1.0))
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            text("Adjust background color alpha with slider").build(),

            (
                row()
                    .margin_top(px(15))
                    .justify_content(JustifyContent::Center)
                    .build(),

                children![
                    text("0.0").build(),
                    (
                        slider(0.0, 1.0)
                            .id("slider")
                            .margin_left(px(5))
                            .margin_right(px(5))
                            .step(0.01)
                            .value(1.0)
                            .build(),

                        observe(on_slider_value_change)
                    ),
                    text("1.0").build()
                ]
            )
        ]
    ));
}

fn on_slider_value_change(change: On<Change<f32>>, mut root_q: RootQuery) {
    if let Some(root) = root_q.find_by_id("root") {
        root.style.background_color.0 = Color::srgba(1.0, 0.5, 0.5, change.data);
    }
}
