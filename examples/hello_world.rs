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

    // spawning root & text widget.
    commands.spawn((
        root()
            .background_color(Color::srgb(1.0, 0.5, 0.5))
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            text("Hello world").font_size(25.0).build()
        ]
    ));
}
