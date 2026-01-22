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
    commands.spawn(text_!("This is underneath Text", color: Color::srgb(1.0, 1.0, 1.0)));

    commands.spawn(
        root_!(
            id: "root",
            background_color: Color::srgba(1.0, 0.5, 0.5, 1.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;

            [
                text_!("Adjust background color alpha with slider"),

                row_!(
                    justify_content: JustifyContent::Center,
                    margin_top: px(15);

                    [
                        text_!("0.0"),
                        slider_!(
                            min: 0.0, max: 1.0, step: 0.01, value: 1.0, margin_x: px(5);
                            on: on_slider_value_change
                        ),
                        text_!("1.0"),
                    ]
                )
            ]
        )
    );
}

fn on_slider_value_change(change: On<Change<f32>>, mut root_q: RootQuery) {
    if let Some(root) = root_q.find_by_id("root") {
        root.style.background_color.0 = Color::srgba(1.0, 0.5, 0.5, change.data);
    }
}
