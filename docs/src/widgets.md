**Makara** provides built-in widgets that are needed for building GUI applications including
**button**, **modal**, **text input** and more. There will be more important widgets added
to **Makara** in upcoming new versions.

### Root
`root_!` is a starting point for UI hierarchy, so it needs to be spawned by commands.
```rust
commands.spawn(root_!());
```
For more detail, see [RootBundle](https://docs.rs/makara/latest/makara/widgets/root/struct.RootBundle.html), 
[RootQuery](https://docs.rs/makara/latest/makara/widgets/root/struct.RootQuery.html), 
[RootWidget](https://docs.rs/makara/latest/makara/widgets/root/struct.RootWidget.html).

### Text
```rust
text_!("Hello world");
```
For more detail, see [TextBundle](https://docs.rs/makara/latest/makara/widgets/text/struct.TextBundle.html), 
[TextQuery](https://docs.rs/makara/latest/makara/widgets/text/struct.TextQuery.html), 
[TextWidget](https://docs.rs/makara/latest/makara/widgets/text/struct.TextWidget.html).

### Button 
```rust
button_!("Click me");
```
With event listeners
```rust
button_!(
    "Click me";
    
    on: |clicked: On<Clicked>| {}; 
    on: |over: On<MouseOver>| {}; 
    on: |out: On<MouseOut>| {}; 
    on: |built: On<WidgetBuilt>| {}
);
```
For more detail, see [ButtonBundle](https://docs.rs/makara/latest/makara/widgets/button/struct.ButtonBundle.html), 
[ButtonQuery](https://docs.rs/makara/latest/makara/widgets/button/struct.ButtonQuery.html), 
[ButtonWidget](https://docs.rs/makara/latest/makara/widgets/button/struct.ButtonWidget.html).

### Checkbox
```rust
checkbox_!("Check me");
```
With event listeners
```rust
checkbox_!(
    "Check me";
    
    on: |clicked: On<Clicked>| {}; 
    on: |over: On<MouseOver>| {}; 
    on: |out: On<MouseOut>| {}; 
    on: |active: On<Active<String>>| {}; 
    on: |inactive: On<Inactive<String>>| {}; 
    on: |built: On<WidgetBuilt>| {}
);
```
For more detail, see [CheckboxBundle](https://docs.rs/makara/latest/makara/widgets/checkbox/struct.CheckboxBundle.html), 
[CheckboxQuery](https://docs.rs/makara/latest/makara/widgets/checkbox/struct.CheckboxQuery.html), 
[CheckboxWidget](https://docs.rs/makara/latest/makara/widgets/checkbox/struct.CheckboxWidget.html).

### Circular
```rust
circular_!();
```
With event listeners
```rust
circular_!(
    on: |over: On<MouseOver>| {}; 
    on: |out: On<MouseOut>| {}; 
    on: |change: On<Change<f32>>| {}; 
    on: |built: On<WidgetBuilt>| {}
);
```
For more detail, see [CircularBundle](https://docs.rs/makara/latest/makara/widgets/circular/struct.CircularBundle.html), 
[CircularQuery](https://docs.rs/makara/latest/makara/widgets/circular/struct.CircularQuery.html), 
[CircularWidget](https://docs.rs/makara/latest/makara/widgets/circular/struct.CircularWidget.html).

### Progress Bar
```rust
progress_bar_!();
```
With event listeners
```rust
progress_bar_!(
    on: |over: On<MouseOver>| {};
    on: |out: On<MouseOut>| {};
    on: |change: On<Change<f32>>| {};
    on: |built: On<WidgetBuilt>| {}
);
```
For more detail, see [ProgressBarBundle](https://docs.rs/makara/latest/makara/widgets/progress_bar/struct.ProgressBarBundle.html), 
[ProgressBarQuery](https://docs.rs/makara/latest/makara/widgets/progress_bar/struct.ProgressBarQuery.html), 
[ProgressBarWidget](https://docs.rs/makara/latest/makara/widgets/progress_bar/struct.ProgressBarWidget.html).

### Column
```rust
column_!([
    text_!("Item 1"),
    text_!("Item 2")
]);
```
With event listener
```rust
column_!(
    on: |built: On<WidgetBuilt>| {};
    [ text_!("Item 1") ]
);
```
For more detail, see [ColumnBundle](https://docs.rs/makara/latest/makara/widgets/column/struct.ColumnBundle.html), 
[ColumnQuery](https://docs.rs/makara/latest/makara/widgets/column/struct.ColumnQuery.html), 
[ColumnWidget](https://docs.rs/makara/latest/makara/widgets/column/struct.ColumnWidget.html).

### Row
```rust
row_!([
    text_!("Left"),
    text_!("Right")
]);
```
With event listener
```rust
row_!(
    on: |built: On<WidgetBuilt>| {};
    [ text_!("Item 1") ]
);
```
For more detail, see [RowBundle](https://docs.rs/makara/latest/makara/widgets/row/struct.RowBundle.html), 
[RowQuery](https://docs.rs/makara/latest/makara/widgets/row/struct.RowQuery.html), 
[RowWidget](https://docs.rs/makara/latest/makara/widgets/row/struct.RowWidget.html).

### Link 
```rust
link_!("https://rust-lang.org/");
```
With event listeners
```rust
link_!(
    "https://rust-lang.org/";

    on: |clicked: On<Clicked>| {};
    on: |over: On<MouseOver>| {};
    on: |out: On<MouseOut>| {};
    on: |built: On<WidgetBuilt>| {}
);
```
For more detail, see [LinkBundle](https://docs.rs/makara/latest/makara/widgets/link/struct.LinkBundle.html), 
[LinkQuery](https://docs.rs/makara/latest/makara/widgets/link/struct.LinkQuery.html), 
[LinkWidget](https://docs.rs/makara/latest/makara/widgets/link/struct.LinkWidget.html).

### Dropdown 
```rust
dropdown_!(
    "Click me to show option";
    [
        button_!("Sign In"),
        button_!("Sign Up"),
        button_!("About us")
    ]
);
```
With event listeners
```rust
dropdown_!(
    "Click me to show option";

    on: |clicked: On<Clicked>| {};
    on: |over: On<MouseOver>| {};
    on: |out: On<MouseOut>| {};
    on: |built: On<WidgetBuilt>| {};

    [
        button_!("Sign In"),
        button_!("Sign Up")
    ]
);
```
For more detail, see [DropdownBundle](https://docs.rs/makara/latest/makara/widgets/dropdown/struct.DropdownBundle.html), 
[DropdownQuery](https://docs.rs/makara/latest/makara/widgets/dropdown/struct.DropdownQuery.html), 
[DropdownWidget](https://docs.rs/makara/latest/makara/widgets/dropdown/struct.DropdownWidget.html).

### Image 
```rust
// path or url
image_!("path/to/image.png");
```
With event listeners
```rust
image_!(
    "path/to/image.png";
    
    on: |clicked: On<Clicked>| {};
    on: |over: On<MouseOver>| {};
    on: |out: On<MouseOut>| {};
    on: |built: On<WidgetBuilt>| {}
);
```
For more detail, see [ImageBundle](https://docs.rs/makara/latest/makara/widgets/image/struct.ImageBundle.html), 
[ImageQuery](https://docs.rs/makara/latest/makara/widgets/image/struct.ImageQuery.html), 
[ImageWidget](https://docs.rs/makara/latest/makara/widgets/image/struct.ImageWidget.html).

### Modal
Modal is independent and doesn't need to be part of `root` widget.
```rust
command.spawn(
    modal_!([
        column_!([
            text_!("Hello world"),
            button_!("Close modal")
        ])
    ])
);
```
With event listeners
```rust
modal_!(
    on: |active: On<Active>| {};
    on: |inactive: On<Inactive>| {};
    on: |built: On<WidgetBuilt>| {};

    [
        column_!([
            text_!("Hello world"),
            button_!("Close modal")
        ])
    ]
);
```
For more detail, see [ModalBundle](https://docs.rs/makara/latest/makara/widgets/modal/struct.ModalBundle.html), 
[ModalQuery](https://docs.rs/makara/latest/makara/widgets/modal/struct.ModalQuery.html), 
[ModalWidget](https://docs.rs/makara/latest/makara/widgets/modal/struct.ModalWidget.html).

### Radio Group & Radio
`radio_group` needs `radio` as its item.
```rust
radio_group_!([
    radio_!("Pay by Card"),
    radio_!("Pay by Cash")
]);
```
With event listeners
```rust
radio_group_!(
    on: |clicked: On<Clicked>| {};
    on: |over: On<MouseOver>| {};
    on: |out: On<MouseOut>| {};
    on: |change: On<Change<String>>| {};
    on: |built: On<WidgetBuilt>| {};

    [
        radio_!(
            "Pay by Card";
            on: |active: On<Active>| {}; 
            on: |inactive: On<Inactive>| {}
        ),
        radio_!("Pay by Cash")
    ]
);
```
For more detail, see [RadioGroupBundle](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioGroupBundle.html), 
[RadioGroupQuery](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioGroupQuery.html), 
[RadioGroupWidget](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioGroupWidget.html),
[RadioBundle](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioBundle.html), 
[RadioQuery](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioQuery.html), 
[RadioWidget](https://docs.rs/makara/latest/makara/widgets/radio/struct.RadioWidget.html).

### Scroll
```rust
scroll_!([
    text_!("Hello world"),
    text_!("Hello there")
]);
```
With event listener
```rust
scroll_!(
    on: |scrolling: On<Scrolling>| {};
    [
        text_!("Hello world"),
        text_!("Hello there")
    ]
);
```
For more detail, see [ScrollBundle](https://docs.rs/makara/latest/makara/widgets/scroll/struct.ScrollBundle.html), 
[ScrollQuery](https://docs.rs/makara/latest/makara/widgets/scroll/struct.ScrollQuery.html), 
[ScrollWidget](https://docs.rs/makara/latest/makara/widgets/scroll/struct.ScrollWidget.html).

### Select 
```rust
select_!("Click me to show option", choices: &["Cash", "Card", "Afterpay"]);    
```
With event listeners
```rust
select_!(
    "Click me to show option", 
    choices: &["Cash", "Card", "Afterpay"];
    
    on: |clicked: On<Clicked>| {};
    on: |over: On<MouseOver>| {};
    on: |out: On<MouseOut>| {};
    on: |built: On<WidgetBuilt>| {};
    on: |active: On<Active>| {};
    on: |inactive: On<Inactive>| {};
    on: |change: On<Change<String>>| {}
);
```
For more detail, see [SelectBundle](https://docs.rs/makara/latest/makara/widgets/select/struct.SelectBundle.html), 
[SelectQuery](https://docs.rs/makara/latest/makara/widgets/select/struct.SelectQuery.html), 
[SelectWidget](https://docs.rs/makara/latest/makara/widgets/select/struct.SelectWidget.html).

### Slider 
```rust
// Takes range-start & range-end as arguments.
// In this case start at 0.0, end at 100.0 .
slider_!(min: 0.0, max: 100.0);
```
With event listeners
```rust
slider_!(
    min: 0.0, max: 100.0;
    
    on: |clicked: On<Clicked>| {};
    on: |over: On<MouseOver>| {};
    on: |out: On<MouseOut>| {};
    on: |built: On<WidgetBuilt>| {}; 
    on: |change: On<Change<f32>>| {}
);
```
For more detail, see [SliderBundle](https://docs.rs/makara/latest/makara/widgets/slider/struct.SliderBundle.html), 
[SliderQuery](https://docs.rs/makara/latest/makara/widgets/slider/struct.SliderQuery.html), 
[SliderWidget](https://docs.rs/makara/latest/makara/widgets/slider/struct.SliderWidget.html).

### Text Input
```rust
text_input_!("Enter something");
```
With event listeners
```rust
text_input_!(
    "Enter something";
    
    on: |clicked: On<Clicked>| {};
    on: |over: On<MouseOver>| {};
    on: |out: On<MouseOut>| {};
    on: |built: On<WidgetBuilt>| {}; 
    on: |change: On<Change<String>>| {}
);
```
For more detail, see [TextInputBundle](https://docs.rs/makara/latest/makara/widgets/text_input/struct.TextInputBundle.html), 
[TextInputQuery](https://docs.rs/makara/latest/makara/widgets/text_input/struct.TextInputQuery.html), 
[TextInputWidget](https://docs.rs/makara/latest/makara/widgets/text_input/struct.TextInputWidget.html).
