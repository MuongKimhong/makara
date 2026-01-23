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
            id: "root",
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;

            [
                dropdown_!(
                    "Background color options",
                    margin_top: px(10);

                    [
                        button_!("Red"; on: |_clicked: On<Clicked>, mut root_q: RootQuery| {
                            let root = root_q.find_by_id("root").unwrap();
                            root.style.background_color.set_color("red");
                        }),
                        button_!("Blue"; on: |_clicked: On<Clicked>, mut root_q: RootQuery| {
                            let root = root_q.find_by_id("root").unwrap();
                            root.style.background_color.set_color("blue");
                        })
                    ]
                )
            ]
        )
    );
}
