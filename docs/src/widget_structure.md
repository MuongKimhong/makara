### Understanding the structure of widgets in Makara

In **Makara**, creating a widget can be done by calling its pre-defined macro.
A button can be created using `button_!` and a text can be created using `text_!`.

Each widget macro is splitted into 3 parts:

1. Properties
2. Event handlers
3. Children array

For example,
```rust
scroll_!( 
    // properties
    background_color: "red",
    border_radius: px(5),
    margin: px(5),
    padding: px(10),
    
    // event handlers (if there's any)
    on: |_scrolling: On<Scrolling>| {},
    on: |_built: On<WidgetBuilt>| {},
    
    // children array
    [
        button_!("Click me", background_color: "blue"),
        text_!("This is some text")
    ]
);
```

For event handlers, you can also create functions for it to make it cleaner.
```rust
scroll_!( 
    // properties
    background_color: "red",
    border_radius: px(5),
    margin: px(5),
    padding: px(10),
    
    // event handlers (if there's any)
    on: handle_scrolling,
    on: handle_widget_built,
    
    // children array
    [
        button_!("Click me", background_color: "blue"),
        text_!("This is some text")
    ]
);

fn handle_scrolling(_scrolling: On<Scrolling>) {
    // do something
}

fn handle_widget_built(_built: On<WidgetBuilt>) {
    // do something
}
```
