### Understanding the structure of widgets in Makara

In **Makara**, creating a widget can be done by calling its pre-defined macro.
A button can be created using `button_!` and a text can be created using `text_!`.

Each macro is splitted into 3 parts with specific orders and the orders are:

1. Properties
2. Event handlers
3. Children

Let's have a look at a few examples.

Button
```rust
button_!(
    "Click me", 
    id: "my-btn", 
    background_color: "blue"; // properties end with ;
    
    on: |clicked: On<Clicked>| { 
        println("Button is clicked"); 
    };
    on: |mouse_over: On<MouseOver>| {
        println!("Mouse is over button");
    }
);

// We don't need ; at the end of properties because we don't have an event handler.
button_!("Click me", border_color: "red");
```

Column
```rust
column_!(
    id: "my-column",
    align_items: AlignItems::Center; // properties end with ;
    
    [
        text_!("Hello world"), 
        text_!("Hello again")
    ]
);

// We don't need ; at the end of properties because we don't have any children.
column_!(id: "my-column", align_items: AlignItems::Center);
```

Scroll
```rust
scroll_!(
    align_items: AlignItems::Center;  // properties end with ;
    
    on: |scrolling; On<Scrolling>| {
        println!("scrolling");
    }; // event handler ends with ;
    
    [
        text_!("Hello world"),
        text_!("Hello mom"),
        text_!("Hello friends")
    ]
);
```

### The rules for using widget macro

1. Always put `;` after properties if there are event handlers or children `[]`.
2. Always put `;` after an event handler if there are more event handlers or children `[]`.
