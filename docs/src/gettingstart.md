#### IMPORTANT
Makara relies entire on [Bevy](https://bevyengine.org/)'s UI system and ECS. When you use **Makara**,
you're actually using the **Bevy engine**, with **Makara** providing an abstraction for its UI system
to help you build GUI applications with ease.

A solid understanding of **Bevy's ECS** is required. If you're new to **Bevy**,
I recommend checking out [Bevy's quick start guide](https://bevyengine.org/learn/quick-start/introduction/).

### Getting Start

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
            text("Hello world").font_size(20.0).build()
        ]
    ));
}
```

### Understand Makara hierarchy

Just like any other UI frameworks, you need a starting point for your application. 
In **Makara**, you start by spawning a single **root** widget and add other widgets 
as children of **root** except **modal**.

In above example, when you run the application with `cargo run` you will see the text
"Hello world" appears at the center of the screen.

You can make a widget becomes a child of another widget by using `children` macro provided by Bevy.

A widget is an entity. An **entity** is a set of **components**. If you have more than one component for
an entity, you will need to wrap it inside a `tuple`. For example,
```rust
#[derive(Component)]
pub struct HelloWorldComponent;

#[derive(Component)]
pub struct HelloMomComponent;

// spawn an entity with just one component
commands.spawn(HelloWorldComponent);

// spawn an entity with 2 components
commands.spawn((HelloWorldComponent, HelloMomComponent));
```

This also applies to **Makara**'s widgets.
```rust
// no need tuple here, because text doesn't need other components
children![
    text("Hello world").font_size(20.0).build()
]

// wrap button inside a tuple because it needs to observe Clicked event.
children![
    (
        button("Click me").build(),
        observe(|_clicked: On<Clicked>| {
            // do something
        })
    )
]
```

### The **`build`** method

You propaly notice that every time a widget is called, they will ended with `.build()`. To simplify UI 
creation, we use a Builder Pattern that abstracts the complexity of Bevy's ECS.

```rust
// This gives us a blueprint of the button.
// We then can customise this blueprint with what we want.
button("Click me");

// Example, changing background color or attaching an ID.
button("Click").id("my-btn").background_color(Color::BLACK);

// Finally, call .build() method to flatten the blueprint,
// attaches internal identifying tags and finally register
// event listeners and emitters.
button("Click").id("my-btn").background_color(Color::BLACK).build();
```
