**Makara** provides built-in widgets that are needed for building GUI applications including
**button**, **modal**, **text input** and more. There will be more important widgets added
to **Makara** in upcoming new versions.

### Root
```rust
root().build()
```

### Text
```rust
text("Hi Mom!").build()
```

### Button 
```rust
button("Click me").build()
```
With event listeners
```rust
(
    button("Click me").build(),

    observe(|clicked: On<Clicked>| {}),

    observe(|over: On<MouseOver>| {}),

    observe(|out: On<MouseOut>| {}),

    observe(|built: On<WidgetBuilt>| {}),
)
```

### Checkbox
```rust
checkbox("Check me").build()
```
With event listeners
```rust
(
    checkbox("Check me").build(),

    observe(|clicked: On<Clicked>| {}),

    observe(|over: On<MouseOver>| {}),

    observe(|out: On<MouseOut>| {}),

    observe(|active: On<Active>| {}),

    observe(|inactive: On<Inactive>| {}),

    observe(|built: On<WidgetBuilt>| {}),
)
```

### Circular
```rust
circular().build()
```
With event listeners
```rust
(
    circular().build(),

    observe(|over: On<MouseOver>| {}),

    observe(|out: On<MouseOut>| {}),

    observe(|change: On<Change<f32>>| {}),

    observe(|built: On<WidgetBuilt>| {}),
)
```

### Progress Bar
```rust
progress_bar().build()
```
With event listeners
```rust
(
    progress_bar().build(),

    observe(|over: On<MouseOver>| {}),

    observe(|out: On<MouseOut>| {}),

    observe(|change: On<Change<f32>>| {}),

    observe(|built: On<WidgetBuilt>| {}),
)
```

### Column
```rust
column().build()
```
With event listener
```rust
(
    column().build(),

    observe(|built: On<WidgetBuilt>| {}),
)
```

### Row
```rust
row().build()
```
With event listener
```rust
(
    row().build(),

    observe(|built: On<WidgetBuilt>| {}),
)
```

### Link 
```rust
link("https://rust-lang.org/").build()
```
With event listeners
```rust
(
    link("https://rust-lang.org/").build(),

    observe(|clicked: On<Clicked>| {}),

    observe(|over: On<MouseOver>| {}),

    observe(|out: On<MouseOut>| {}),

    observe(|built: On<WidgetBuilt>| {}),
)
```

### Dropdown 
```rust
(
    dropdown("Click me to show option").build(),
    children![
        button("Sign In").build(),
        button("Sign Up").build(),
        button("About us").build()
    ]
)
```
With event listeners
```rust
(
    dropdown("Click me to show option").build(),
    children![
        button("Sign In").build(),
        button("Sign Up").build(),
        button("About us").build()
    ],
    
    observe(|clicked: On<Clicked>| {}),
    
    observe(|over: On<MouseOver>| {}),
    
    observe(|out: On<MouseOut>| {}),
    
    observe(|built: On<WidgetBuilt>| {})
)
```

### Image 
```rust
// path or url
image("path/to/image.png").build()
```
With event listeners
```rust
(
    image("path/to/image.png").build()
    
    observe(|clicked: On<Clicked>| {}),
    
    observe(|over: On<MouseOver>| {}),
    
    observe(|out: On<MouseOut>| {}),
    
    observe(|built: On<WidgetBuilt>| {})
)
```

### Modal
Modal is independent and doesn't need to be part of `root` widget.
```rust
(
    modal().build(),
    children![
        ( 
            // modal content need to wrapped inside a container
            column().build(),
            children![
                text("Hello world").build(),
                button("Close modal").build()
            ]
        )
    ]
)
```
With event listeners
```rust
(
    modal().build(),
    children![
        ( 
            // modal content
            column().build(),
            children![
                text("Hello world").build(),
                button("Close modal").build()
            ]
        )
    ],
    observe(|active: On<Active>| {}),

    observe(|inactive: On<Inactive>| {}),
        
    observe(|built: On<WidgetBuilt>| {})
)
```

### Radio Group & Radio
`radio_group` needs `radio` as its item.
```rust
(
    radio_group().build(),
    children![
        radio("Pay by Card").build(),
        radio("Pay by Cash").build(), 
    ]
)
```
With event listeners
```rust
(
    radio_group().build(),
    children![
        (
            radio("Pay by Card").build(),
            
            observe(|active: On<Active>| {}), 
            observe(|inactive: On<Inactive>| {})
        ),
        radio("Pay by Cash").build(), 
    ],

    observe(|clicked: On<Clicked>| {}),

    observe(|over: On<MouseOver>| {}),

    observe(|out: On<MouseOut>| {}),

    observe(|change: On<Change<String>>| {}),

    observe(|built: On<WidgetBuilt>| {}),
)
```

### Scroll
```rust
(
    scroll().build(),
    children![
        // scroll content
        text("Hello world").build(),
        text("Hello there").build(),
    ]
)
```
With event listener
```rust
(
    scroll().build(),
    children![
        // scroll content
        text("Hello world").build(),
        text("Hello there").build(),
    ],

    observe(|scrolling: On<Scrolling>| {})
)
```

### Select 
```rust
select("Click me to show option", &["Cash", "Card", "Afterpay"]).build(), 
```
With event listeners
```rust
(
    select("Click me to show option", &["Cash", "Card", "Afterpay"]).build(),
    
    observe(|clicked: On<Clicked>| {}),
    
    observe(|over: On<MouseOver>| {}),
    
    observe(|out: On<MouseOut>| {}),
    
    observe(|built: On<WidgetBuilt>| {}),
    
    observe(|active: On<Active>| {}),
    
    observe(|inactive: On<Inactive>| {}),

    observe(|change: On<Change<String>>| {}),
)
```


### Slider 
```rust
// Takes range-start & range-end as arguments.
// In this case start at 0.0, end at 100.0 .
slider(0.0, 100.0).build()
```
With event listeners
```rust
(
    slider(0.0, 100.0).build()
    
    observe(|clicked: On<Clicked>| {}),
    
    observe(|over: On<MouseOver>| {}),
    
    observe(|out: On<MouseOut>| {}),
    
    observe(|built: On<WidgetBuilt>| {}), 

    observe(|change: On<Change<f32>>| {}),
)
```

### Text Input
```rust
text_input("Enter something").build()
```
With event listeners
```rust
(    
    text_input("Enter something").build(),
    
    observe(|clicked: On<Clicked>| {}),
    
    observe(|over: On<MouseOver>| {}),
    
    observe(|out: On<MouseOut>| {}),
    
    observe(|built: On<WidgetBuilt>| {}), 
    
    observe(|change: On<Change<String>>| {}),
)
```
