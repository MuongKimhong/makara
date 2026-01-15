use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, (setup, setup_style).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        root()
            .id("root")
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            (
                dropdown("Background color options").id("hello").margin_top(px(10)).build(),
                children![
                    (
                        button("Red").build(),
                        observe(|_clicked: On<Clicked>, mut root_q: RootQuery| {
                            let root = root_q.find_by_id("root").unwrap();
                            root.style.background_color.0 = Color::srgb(1.0, 0.0, 0.0);
                        })
                    ),
                    (
                        button("Blue").build(),
                        observe(|_clicked: On<Clicked>, mut root_q: RootQuery| {
                            let root = root_q.find_by_id("root").unwrap();
                            root.style.background_color.0 = Color::srgb(0.0, 0.0, 1.0);
                        })
                    )
                ]
            )
        ]
    ));
}

fn setup_style(mut custom_style: ResMut<CustomStyle>) {
    custom_style.bind_id(
        "hello",
        Style::new()
            .background_color(Color::srgb(1.0, 0.0, 0.0))
    );

    custom_style.bind_id(
        "hello::overlay",
        Style::new()
            .background_color(Color::srgb(0.0, 1.0, 0.0))
    );
}
