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

### An overview of ECS

Because this library is powered by a game engine which is based on ECS architecture, it's a must to understand how ECS works.

ECS stands for **Entity-Component-System**.

- **Entity**: entities are objects you see when you run your application. A button widget is an entity, 
a text widget is an entity, the list goes on. Each entity consists of one or multiple **components**.

- **Component**: think of component as a tag. It's used to identify different kind of data. In the context 
of **Makara** or **Bevy**, component is just a rust struct or enum. 
  
  For example, you create a button with background color of red and border color of blue in **Makara**.

  ```rust
  // create button with background color of red and border color of blue
  button("Click me")
    .background_color(Color::RED)
    .border_color(COLOR::BLUE)
    .build();
  ```

  Under the hood, the button is just an entity that contains these 
  2 components, [`BackgroundColor`](https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html) 
  and [`BorderColor`](https://docs.rs/bevy/latest/bevy/prelude/struct.BorderColor.html) which looks 
  something like this.

  ```rust
  (
      BackgroundColor(Color::RED),
      BorderColor(Color::BLUE),
      // other components
  )
  ```

  It's then up to the **Bevy** engine to translate it to shading language and render it on your GPU.

- **System**: systems are just rust functions that take specific arguments and run at specific schedule. `Startup` schedule means the
functions run only 1 time at the beginning. `Update` schedule means the functions run every single frame. And 
of course, there are more schedules than just that.

  ```rust
  fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
  }
  
  fn setup(mut commands: Commands) {
      // do something
  }
  ```
  Here, `setup` function is a system that take a special argument, in this case `Commands`. It's being registered
  to run at `Startup` schedule which means it only do something once and that's it.

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
