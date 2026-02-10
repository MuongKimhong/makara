# Alignment Utility Classes

Makara provides a comprehensive set of alignment utility classes that allow you to control the layout and positioning of widgets without writing custom styles. These classes follow CSS flexbox conventions and work similarly to utility frameworks like Tailwind CSS.

## Justify Content Classes

Justify content classes control the alignment of child elements along the **main axis** (horizontal for rows, vertical for columns).

| Class Name | Alternative | Effect |
|-----------|-------------|---------|
| `justify-start` | `justify-content-start` | Align children to the start of the main axis |
| `justify-center` | `justify-content-center` | Center children along the main axis |
| `justify-end` | `justify-content-end` | Align children to the end of the main axis |
| `justify-between` | `justify-content-between` | Distribute children with equal space between them |
| `justify-around` | `justify-content-around` | Distribute children with equal space around them |
| `justify-evenly` | `justify-content-evenly` | Distribute children with equal space around and between them |
| `justify-stretch` | `justify-content-stretch` | Stretch children to fill the main axis |

## Align Items Classes

Align items classes control the alignment of child elements along the **cross axis** (vertical for rows, horizontal for columns).

| Class Name | Alternative | Effect |
|-----------|-------------|---------|
| `align-start` | `align-items-start` | Align children to the start of the cross axis |
| `align-center` | `align-items-center` | Center children along the cross axis |
| `align-end` | `align-items-end` | Align children to the end of the cross axis |
| `align-stretch` | `align-items-stretch` | Stretch children to fill the cross axis |
| `align-baseline` | `align-items-baseline` | Align children along their baseline |

## Align Content Classes

Align content classes control the alignment of wrapped lines when there are multiple rows/columns of content.

| Class Name | Effect |
|-----------|---------|
| `align-content-start` | Align wrapped lines to the start |
| `align-content-center` | Center wrapped lines |
| `align-content-end` | Align wrapped lines to the end |
| `align-content-stretch` | Stretch wrapped lines to fill the container |
| `align-content-between` | Distribute wrapped lines with equal space between them |
| `align-content-around` | Distribute wrapped lines with equal space around them |
| `align-content-evenly` | Distribute wrapped lines with equal space around and between them |

## Usage Examples

### Basic Centering

```rust
// Center content both horizontally and vertically
root_!(
    class: "justify-center align-center";
    [ text_!("Centered content") ]
)
```

### Button Layout

```rust
// Distribute buttons evenly across a row
row_!(
    class: "justify-between align-center";
    [
        button_!("Cancel"),
        button_!("Submit")
    ]
)
```

### Card Layout

```rust
// Create a vertically centered card with content aligned to start
column_!(
    class: "justify-center align-start p-4";
    [
        text_!("Card Title", font_size: 18.0),
        text_!("Card content goes here..."),
        button_!("Action", class: "mt-3")
    ]
)
```

### Navigation Bar

```rust
// Create a navigation bar with logo on left, menu on right
row_!(
    class: "justify-between align-center p-3";
    [
        text_!("Logo", font_size: 20.0),
        row_!(
            class: "justify-end align-center";
            [
                button_!("Home", class: "mr-2"),
                button_!("About", class: "mr-2"),
                button_!("Contact")
            ]
        )
    ]
)
```

### Grid-like Layout

```rust
// Create a grid of items with even spacing
column_!(
    class: "justify-start align-stretch";
    [
        row_!(
            class: "justify-around align-center mb-4";
            [
                button_!("Item 1"),
                button_!("Item 2"),
                button_!("Item 3")
            ]
        ),
        row_!(
            class: "justify-around align-center";
            [
                button_!("Item 4"),
                button_!("Item 5"),
                button_!("Item 6")
            ]
        )
    ]
)
```

## Combining with Other Classes

Alignment classes work seamlessly with other built-in utility classes:

```rust
// Combine alignment with spacing and color classes
column_!(
    class: "justify-center align-center p-4 m-2";
    background_color: Color::WHITE;
    [
        text_!("Welcome!", class: "mb-3", font_size: 24.0),
        button_!("Get Started", class: "is-primary")
    ]
)
```

## Responsive Considerations

While Makara doesn't have built-in responsive breakpoints like web CSS frameworks, you can dynamically change classes based on window size or other conditions in your Bevy systems:

```rust
fn adjust_layout_system(
    mut commands: Commands,
    windows: Query<&Window>,
    mut widgets: Query<(Entity, &mut Class), With<SomeWidget>>
) {
    if let Ok(window) = windows.get_single() {
        for (entity, mut class) in widgets.iter_mut() {
            if window.width() < 600.0 {
                // Mobile layout
                class.set("justify-center align-center".to_string());
            } else {
                // Desktop layout
                class.set("justify-between align-center".to_string());
            }
        }
    }
}
```

## Supported Widgets

Alignment utility classes are supported on **container widgets** that manage child element layout:

- `root_!()` - Main application container
- `row_!()` - Horizontal flex container
- `column_!()` - Vertical flex container  
- `scroll_!()` - Scrollable container

Other widgets like `button_!()`, `text_!()`, `image_!()`, etc. have fixed internal layouts and don't support alignment classes since they're not intended to be containers for user-defined child elements.

## Performance Notes

- Alignment classes are processed during widget build and when the class attribute changes
- No runtime overhead once the layout is computed
- Classes are parsed once and cached until the class attribute is modified
- Bevy's layout system handles the actual positioning efficiently

## Migration from Manual Styling

If you were previously setting alignment properties manually:

```rust
// Old way
root_!(
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center;
    [ /* children */ ]
)

// New way with utility classes
root_!(
    class: "justify-center align-center";
    [ /* children */ ]
)
```

The utility class approach provides the same functionality with more concise syntax and better consistency across your application.