## Styling in Makara

There are 2 ways to provide custom styles to widgets.

1. Style on widgets directly.
2. Attach an `ID` or `Classes` and provide styles via `ID` or `Classes`.

### Direct styling
```rust
fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        root()
            .align_items(AlignItems::Center) // direct styling
            .justify_content(JustifyContent::Center) // direct styling
            .build(),

        children![
            // direct styling for font size
            text("Hello world").font_size(20.0).build()
        ]
    ));
}
```

### With ID and Classes
```rust
fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, (setup, setup_styles)) // add this
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        root().id("root").build(),

        children![
            text("Hello world").id("text-one").class("text").build(),
            text("Hello mom").class("text").build(),
            text("Hello friend").class("text hello-friend").build(),
            text("Hello stranger").class("text").build()
        ]
    ));
}

fn setup_styles(mut style: ResMut<CustomStyle>) {
    // via id
    style.bind_id(
        "root",
        Style::new()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
    );
    
    // via id
    style.bind_id("text-one", Style::new().color("red"));
    
    // via class
    style.bind_class(
        "text",
        Style::new().font_size(20.0)
    );
    
    // via class
    style.bind_class(
        "hello-friend",
        Style::new().color("blue")
    );
}
```
When you run the application:

- `root` will get `AlignItems` and `JustifyContent` styles.
- all `texts` will have `font_size` of `20.0`.
- only one `text` with id `text-one` will have `color` of `red`.
- only one `text` with class `hello-friend` will have `color` of `blue`.
