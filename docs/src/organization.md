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
    commands.spawn(
        root_!(
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;
            
            [
                text_!("Register your info"),
                input_container(),
                button_container()
            ]
        )
    );
}

fn input_container() -> impl Bundle {
    column_!([
        text_input_!("Enter email"),
        text_input_!("Enter name"),
        text_!("Choose gender"),
        radio_group_!([ radio_!("Male"), radio_!("Female") ])
    ])
}

fn button_container() -> impl Bundle {
    row_!([
        button_!("Cancel"; on: |_clicked: On<Clicked>| {}),
        button_!("Submit"; on: |_clicked: On<Clicked>| {})
    ])
}
```
