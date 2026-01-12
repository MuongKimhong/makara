use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, (setup, setup_style).chain())
        .run();
}

fn setup(mut commands: Commands) {
    // spawning root & text widget.
    commands.spawn((
        root()
            .background_color(Color::srgb(1.0, 0.5, 0.5))
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            text("Hello world").font_size(25.0).build(),

            checkbox("Option 1").id("checkbox").build()
        ]
    ));
}

fn setup_style(mut custom_style: ResMut<CustomStyle>) {
    custom_style.bind_id(
        "checkbox",
        Style::new()
            .border(UiRect::all(px(5)))
            .border_color(Color::srgb(1.0, 0.0, 0.0))
    );

    custom_style.bind_id(
        "checkbox::checkbox-button",
        Style::new()
            .border_color(Color::srgb(1.0, 0.0, 0.0))
            .active_color(Color::srgb(0.0, 1.0, 0.0))
    );
}
