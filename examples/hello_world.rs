use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // spawning root & text widget.
    commands.spawn(
        root_!(
            background_color: Color::srgb(1.0, 0.5, 0.5),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;

            [ text_!("Hello World", font_size: 25.0) ]
        )
    );
}
