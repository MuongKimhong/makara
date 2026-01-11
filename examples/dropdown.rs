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
            .id("root")
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            (
                dropdown("Background color options").margin_top(px(10)).build(),

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
