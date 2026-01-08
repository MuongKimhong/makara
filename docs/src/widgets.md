**Makara** provides built-in widgets that are needed for building GUI applications including
**button**, **modal**, **text input** and more. There will be more important widgets added
to **Makara** in upcoming new versions.

### Root
```rust
root().build()
```
For more detail, see [RootBundle](https://docs.rs/makara/latest/makara/widgets/root/struct.RootBundle.html), 
[RootQuery](https://docs.rs/makara/latest/makara/widgets/root/struct.RootQuery.html), 
[RootWidget](https://docs.rs/makara/latest/makara/widgets/root/struct.RootWidget.html).

### Text
```rus
text("Hi Mom!").build()
```
For more detail, see [TextBundle](https://docs.rs/makara/latest/makara/widgets/text/struct.TextBundle.html), 
[TextQuery](https://docs.rs/makara/latest/makara/widgets/text/struct.TextQuery.html), 
[TextWidget](https://docs.rs/makara/latest/makara/widgets/text/struct.TextWidget.html).

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
For more detail, see [ButtonBundle](https://docs.rs/makara/latest/makara/widgets/button/struct.ButtonBundle.html), 
[ButtonQuery](https://docs.rs/makara/latest/makara/widgets/button/struct.ButtonQuery.html), 
[ButtonWidget](https://docs.rs/makara/latest/makara/widgets/button/struct.ButtonWidget.html).

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
For more detail, see [CheckboxBundle](https://docs.rs/makara/latest/makara/widgets/checkbox/struct.CheckboxBundle.html), 
[CheckboxQuery](https://docs.rs/makara/latest/makara/widgets/checkbox/struct.CheckboxQuery.html), 
[CheckboxWidget](https://docs.rs/makara/latest/makara/widgets/checkbox/struct.CheckboxWidget.html).

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
For more detail, see [CircularBundle](https://docs.rs/makara/latest/makara/widgets/circular/struct.CircularBundle.html), 
[CircularQuery](https://docs.rs/makara/latest/makara/widgets/circular/struct.CircularQuery.html), 
[CircularWidget](https://docs.rs/makara/latest/makara/widgets/circular/struct.CircularWidget.html).

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
For more detail, see [ProgressBarBundle](https://docs.rs/makara/latest/makara/widgets/progress_bar/struct.ProgressBarBundle.html), 
[ProgressBarQuery](https://docs.rs/makara/latest/makara/widgets/progress_bar/struct.ProgressBarQuery.html), 
[ProgressBarWidget](https://docs.rs/makara/latest/makara/widgets/progress_bar/struct.ProgressBarWidget.html).

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
For more detail, see [ColumnBundle](https://docs.rs/makara/latest/makara/widgets/column/struct.ColumnBundle.html), 
[ColumnQuery](https://docs.rs/makara/latest/makara/widgets/column/struct.ColumnQuery.html), 
[ColumnWidget](https://docs.rs/makara/latest/makara/widgets/column/struct.ColumnWidget.html).

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
For more detail, see [RowBundle](https://docs.rs/makara/latest/makara/widgets/row/struct.RowBundle.html), 
[RowQuery](https://docs.rs/makara/latest/makara/widgets/row/struct.RowQuery.html), 
[RowWidget](https://docs.rs/makara/latest/makara/widgets/row/struct.RowWidget.html).

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
For more detail, see [LinkBundle](https://docs.rs/makara/latest/makara/widgets/link/struct.LinkBundle.html), 
[LinkQuery](https://docs.rs/makara/latest/makara/widgets/link/struct.LinkQuery.html), 
[LinkWidget](https://docs.rs/makara/latest/makara/widgets/link/struct.LinkWidget.html).

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
For more detail, see [DropdownBundle](https://docs.rs/makara/latest/makara/widgets/dropdown/struct.DropdownBundle.html), 
[DropdownQuery](https://docs.rs/makara/latest/makara/widgets/dropdown/struct.DropdownQuery.html), 
[DropdownWidget](https://docs.rs/makara/latest/makara/widgets/dropdown/struct.DropdownWidget.html).

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
For more detail, see [ImageBundle](https://docs.rs/makara/latest/makara/widgets/image/struct.ImageBundle.html), 
[ImageQuery](https://docs.rs/makara/latest/makara/widgets/image/struct.ImageQuery.html), 
[ImageWidget](https://docs.rs/makara/latest/makara/widgets/image/struct.ImageWidget.html).

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
For more detail, see [ModalBundle](https://docs.rs/makara/latest/makara/widgets/modal/struct.ModalBundle.html), 
[ModalQuery](https://docs.rs/makara/latest/makara/widgets/modal/struct.ModalQuery.html), 
[ModalWidget](https://docs.rs/makara/latest/makara/widgets/modal/struct.ModalWidget.html).

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
For more detail, see [RadioGroupBundle](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioGroupBundle.html), 
[RadioGroupQuery](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioGroupQuery.html), 
[RadioGroupWidget](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioGroupWidget.html),
[RadioBundle](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioBundle.html), 
[RadioQuery](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioQuery.html), 
[RadioWidget](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioWidget.html).

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
For more detail, see [ScrollBundle](https://docs.rs/makara/latest/makara/widgets/scroll/struct.ScrollBundle.html), 
[ScrollQuery](https://docs.rs/makara/latest/makara/widgets/scroll/struct.ScrollQuery.html), 
[ScrollWidget](https://docs.rs/makara/latest/makara/widgets/scroll/struct.ScrollWidget.html).

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
For more detail, see [SelectBundle](https://docs.rs/makara/latest/makara/widgets/select/struct.SelectBundle.html), 
[SelectQuery](https://docs.rs/makara/latest/makara/widgets/select/struct.SelectQuery.html), 
[SelectWidget](https://docs.rs/makara/latest/makara/widgets/select/struct.SelectWidget.html).

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
For more detail, see [SliderBundle](https://docs.rs/makara/latest/makara/widgets/slider/struct.SliderBundle.html), 
[SliderQuery](https://docs.rs/makara/latest/makara/widgets/slider/struct.SliderQuery.html), 
[SliderWidget](https://docs.rs/makara/latest/makara/widgets/slider/struct.SliderWidget.html).

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
For more detail, see [TextInputBundle](https://docs.rs/makara/latest/makara/widgets/text_input/struct.TextInputBundle.html), 
[TextInputQuery](https://docs.rs/makara/latest/makara/widgets/text_input/struct.TextInputQuery.html), 
[TextInputWidget](https://docs.rs/makara/latest/makara/widgets/text_input/struct.TextInputWidget.html).
