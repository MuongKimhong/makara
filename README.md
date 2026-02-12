![Static Badge](https://img.shields.io/badge/Version-0.2.7-blue)
![Static Badge](https://img.shields.io/badge/OS-Mac%20Linux%20Window-orange)
[![static Badge](https://img.shields.io/badge/crate.io-0.2.7-green)](https://crates.io/crates/makara)

<p align="center">
  <img width="180" src="assets/logo.png">
</p>

<h2 align="center">GUI Library built on top of Bevy Game Engine.</h2>

## Getting start
```rust
fn on_button_click(click: On<Clicked>, mut text_q: TextQuery, mut router: ResMut<Router>) {
   if let Some(mut text) = text_q.find_by_id("my-text") {
       text.set_text("Hello mars");
   }
}

fn setup(mut commands: Commands) {    
    commands.spawn(
        root_!(
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;
            
            [
                text_!("Hello earth", id: "my-text"),
                button_!("Click me"; on: on_button_click)
            ]
        )
    );
}
```

## Features
- Built-in widgets including button, modal, text input and more.
- Routing systems.
- Custom styling with ID & Classes similar to HTML/CSS.
- Leverages Bevy's massive parallelism for smooth and efficient rendering.
- High level API and flexible.

## Installation
```
cargo add makara
```

## Run examples
```
cargo run --examples <example_name>
```

## Documentation
- [Rust official API doc](https://docs.rs/makara/latest/makara/)
- [Makara cookbook](https://muongkimhong.github.io/makara/)

## Contributing
Makara needs your contributions. Please see [contributing](https://github.com/MuongKimhong/makara/blob/master/CONTRIBUTING.md).

## Versions
Currently, it supports only bevy 0.17.x onward.

| Bevy     | Makara  |
| -------- | ------- |
| 0.18.x   | 0.2.x   |
| 0.17.x   | 0.1.x   |


## License
Makara is released under the [MIT License](https://opensource.org/licenses/MIT).


> [!WARNING]
> **Makara is new**, many useful features are still missing.
