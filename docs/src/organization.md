### Organizing your project

UI hierarchy can become verbosity and hard to keep track of. It's always recommended
to seperate your code into snippets.

For example,
```rust
use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d); // spawn 2D camera as bevy needs this.

    commands.spawn((
        root()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            text("Register your info").build(), 
            input_container(), 
            button_container()
        ]
    ));
}

fn input_container() -> impl Bundle {
    (
        column().build(),
        children![
            text_input("Enter Email").build(),
            text_input("Enter Name").build(), 
            text("Choose gender").build(),
            (
                radio_group().build(),
                children![
                    radio("Male").build(),
                    radio("Female").build(),
                ]
            )
        ]
    )
}

fn button_container() -> impl Bundle {
    (
        row().build(),
        children![
            (
                button("Cancel").build(),
                observe(|_clicked: On<Clicked>| {})
            ),
            (
                button("Submit").build(),
                observe(|_clicked: On<Clicked>| {})
            )
        ]
    )
}
```
