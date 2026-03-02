# Widgets

**Makara** provides a comprehensive set of built-in widgets for building modern GUI applications. Each widget comes with styling options, event handling capabilities, and built-in class support for rapid development.

## Container Widgets

Container widgets manage layout and can contain child elements.

### Root

The `root_!` widget is the starting point for your UI hierarchy and must be spawned by commands. It supports alignment utility classes.

**Basic Usage:**
```rust
commands.spawn(root_!([
    text_!("Hello World")
]));
```

**With styling and alignment:**
```rust
commands.spawn(
    root_!(
        background_color: Color::srgb(0.1, 0.1, 0.2),
        class: "justify-center align-center p-4",
        
        [
            column_!([
                text_!("Welcome to Makara!"),
                button_!("Get Started")
            ])
        ]
    )
);
```

**With custom properties:**
```rust
commands.spawn(
    root_!(
        width: percent(100),
        height: percent(100),
        id: "main-container",
        class: "justify-between align-stretch",
        
        [/* children */]
    )
);
```

### Column

The `column_!` widget arranges children vertically in a flexbox column layout.

**Basic Usage:**
```rust
column_!([
    text_!("Item 1"),
    text_!("Item 2"),
    text_!("Item 3")
])
```

**With alignment and spacing:**
```rust
column_!(
    class: "justify-center align-center p-4 m-2",
    background_color: "white",
    
    [
        text_!("Centered Column", font_size: 24.0),
        button_!("Action Button", class: "is-primary mt-3"),
        text_!("Footer text", class: "mt-2")
    ]
)
```

**With event handling:**
```rust
column_!(
    on: |_built: On<WidgetBuilt>| {
        println!("Column widget built!");
    },
    
    [/* children */]
)
```

### Row

The `row_!` widget arranges children horizontally in a flexbox row layout.

**Basic Usage:**
```rust
row_!([
    text_!("Left"),
    text_!("Center"), 
    text_!("Right")
])
```

**Navigation bar example:**
```rust
row_!(
    class: "justify-between align-center p-3",
    background_color: Color::srgb(0.2, 0.3, 0.8),
    
    [
        text_!("MyApp", font_size: 20.0, color: Color::WHITE),
        row_!(
            class: "justify-end align-center",
            
            [
                button_!("Home", class: "mr-2"),
                button_!("About", class: "mr-2"),
                button_!("Contact")
            ]
        )
    ]
)
```

### Scroll

The `scroll_!` widget creates a scrollable container for content that exceeds the container size.

**Basic Usage:**
```rust
scroll_!([
    text_!("Line 1"),
    text_!("Line 2"),
    text_!("Line 3"),
    // ... many more lines
])
```

**Styled scroll container:**
```rust
scroll_!(
    width: px(300),
    height: px(200),
    class: "p-3",
    background_color: Color::srgb(0.95, 0.95, 0.95),
    
    [
        column_!(
            class: "justify-start align-stretch",
            
            [
                text_!("Scrollable Content", font_size: 18.0, class: "mb-2"),
                text_!("This is line 1"),
                text_!("This is line 2"),
                // ... more content
            ]
        )
    ]
)
```

**With scroll event handling:**
```rust
scroll_!(
    on: |scrolling: On<Scrolling>| {
        println!("User is scrolling!");
    },
    
    [/* scrollable content */]
)
```

## Interactive Widgets

### Button

The `button_!` widget provides clickable interactions with built-in hover states.

**Basic Usage:**
```rust
button_!("Click Me")
```

**With styling:**
```rust
button_!("Submit", class: "is-primary p-3 m-2")
```

**With event handling:**
```rust
button_!(
    "Save Data";
    on: |clicked: On<Clicked>| {
        println!("Save button clicked!");
    },
    on: |over: On<MouseOver>| {
        println!("Mouse over save button");
    },
    on: |out: On<MouseOut>| {
        println!("Mouse left save button");  
    }
)
```

**Complete example with styling and events:**
```rust
button_!(
    "Delete Item",
    id: "delete-btn",
    class: "is-danger p-2 m-1",
    on: |clicked: On<Clicked>| {
        // Handle delete action
    },
    on: |built: On<WidgetBuilt>| {
        println!("Delete button ready!");
    }
)
```

### Text Input

The `text_input_!` widget provides text entry with placeholder support and theming.

**Basic Usage:**
```rust
text_input_!("Enter your name...")
```

**With styling and validation:**
```rust
text_input_!(
    "Enter email address...",
    class: "is-primary p-2",
    width: px(300),
    id: "email-input"
)
```

**With change event handling:**
```rust
text_input_!(
    "Search...";
    on: |change: On<Change<String>>| {
        println!("Search text: {}", change.data);
    };
    on: |clicked: On<Clicked>| {
        println!("Input field focused");
    }
)
```

**Form example:**
```rust
column_!(
    class: "justify-start align-stretch p-4";
    [
        text_!("User Registration", font_size: 20.0, class: "mb-3"),
        text_input_!("Full Name...", class: "mb-2", width: px(300)),
        text_input_!("Email Address...", class: "is-info mb-2", width: px(300)),
        text_input_!("Password...", class: "is-warning mb-3", width: px(300)),
        button_!("Register", class: "is-success")
    ]
)
```

### Checkbox

The `checkbox_!` widget provides boolean toggle functionality.

**Basic Usage:**
```rust
checkbox_!("Accept terms and conditions")
```

**With styling:**
```rust
checkbox_!("Enable notifications", class: "is-primary p-2")
```

**With state change handling:**
```rust
checkbox_!(
    "Subscribe to newsletter";
    on: |active: On<Active<String>>| {
        println!("Checkbox activated: {}", active.data);
    };
    on: |inactive: On<Inactive<String>>| {
        println!("Checkbox deactivated: {}", inactive.data);
    }
)
```

**Settings panel example:**
```rust
column_!(
    class: "justify-start align-start p-4";
    [
        text_!("Settings", font_size: 18.0, class: "mb-3"),
        checkbox_!("Dark mode", class: "is-info mb-2"),
        checkbox_!("Auto-save", class: "is-success mb-2"),  
        checkbox_!("Show tooltips", class: "is-primary mb-2"),
        button_!("Apply Settings", class: "is-primary mt-2")
    ]
)
```

### Radio Group & Radio

Radio widgets provide single-selection from multiple options.

**Basic Usage:**
```rust
radio_group_!([
    radio_!("Option A"),
    radio_!("Option B"),
    radio_!("Option C")
])
```

**Payment method selector:**
```rust
radio_group_!(
    class: "p-3";
    on: |change: On<Change<String>>| {
        println!("Selected payment method: {}", change.data);
    };
    [
        radio_!("Credit Card", class: "is-primary mb-2"),
        radio_!("PayPal", class: "is-info mb-2"),
        radio_!("Bank Transfer", class: "is-success mb-2"),
        radio_!("Cash on Delivery", class: "is-warning")
    ]
)
```

**With individual radio event handling:**
```rust
radio_group_!([
    radio_!(
        "Small Size";
        on: |active: On<Active>| {
            println!("Small size selected");
        }
    ),
    radio_!("Medium Size"),
    radio_!("Large Size")
])
```

### Select

The `select_!` widget provides a dropdown selection interface.

**Basic Usage:**
```rust
select_!("Choose country...", choices: &["USA", "Canada", "Mexico"])
```

**With styling and events:**
```rust
select_!(
    "Select payment method...",
    choices: &["Credit Card", "Debit Card", "PayPal"],
    class: "is-primary p-2",
    width: px(250);
    on: |change: On<Change<String>>| {
        println!("Selected: {}", change.data);
    };
    on: |active: On<Active>| {
        println!("Dropdown opened");
    };
    on: |inactive: On<Inactive>| {
        println!("Dropdown closed");
    }
)
```

**Form integration:**
```rust
column_!(
    class: "justify-start align-stretch p-4";
    [
        text_!("Shipping Information"),
        select_!("Country...", choices: &["USA", "Canada", "UK"], class: "mb-2"),
        select_!("State/Province...", choices: &["CA", "NY", "TX"], class: "mb-2"),
        button_!("Continue", class: "is-success")
    ]
)
```

### Dropdown

The `dropdown_!` widget creates a button that shows/hides child content.

**Basic Usage:**
```rust
dropdown_!(
    "Menu";
    [
        button_!("Profile"),
        button_!("Settings"),
        button_!("Logout")
    ]
)
```

**Navigation dropdown:**
```rust
dropdown_!(
    "Products",
    class: "is-primary";
    on: |clicked: On<Clicked>| {
        println!("Products dropdown toggled");
    };
    [
        button_!("Laptops", class: "mb-1"),
        button_!("Phones", class: "mb-1"),
        button_!("Tablets", class: "mb-1"),
        button_!("Accessories")
    ]
)
```

### Slider

The `slider_!` widget provides numeric input via draggable control.

**Basic Usage:**
```rust
slider_!(min: 0.0, max: 100.0)
```

**Volume control example:**
```rust
column_!(
    class: "justify-start align-center p-4";
    [
        text_!("Volume Control", class: "mb-2"),
        slider_!(
            min: 0.0, 
            max: 100.0,
            class: "is-primary p-2";
            on: |change: On<Change<f32>>| {
                println!("Volume: {:.1}%", change.data);
            }
        )
    ]
)
```

**Settings panel with multiple sliders:**
```rust
column_!(
    class: "justify-start align-stretch p-4";
    [
        text_!("Audio Settings", font_size: 18.0, class: "mb-3"),
        
        column_!(class: "mb-2"; [
            text_!("Master Volume"),
            slider_!(min: 0.0, max: 100.0, class: "is-primary")
        ]),
        
        column_!(class: "mb-2"; [
            text_!("Music Volume"),
            slider_!(min: 0.0, max: 100.0, class: "is-info")
        ]),
        
        column_!(class: "mb-2"; [
            text_!("Effects Volume"), 
            slider_!(min: 0.0, max: 100.0, class: "is-success")
        ])
    ]
)
```

## Display Widgets

### Text

The `text_!` widget displays styled text content.

**Basic Usage:**
```rust
text_!("Hello World")
```

**With styling:**
```rust
text_!("Welcome!", font_size: 24.0, color: Color::srgb(0.2, 0.6, 0.8))
```

**With classes:**
```rust
text_!("Error message", class: "is-danger", font_size: 16.0)
```

**Typography example:**
```rust
column_!(
    class: "justify-start align-start p-4";
    [
        text_!("Main Heading", font_size: 32.0, class: "mb-3"),
        text_!("Subheading", font_size: 20.0, class: "is-info mb-2"),
        text_!("Regular paragraph text with some content.", font_size: 14.0, class: "mb-2"),
        text_!("Small caption text", font_size: 12.0, class: "is-secondary")
    ]
)
```

### Image

The `image_!` widget displays images from file paths or URLs.

**Basic Usage:**
```rust
image_!("assets/logo.png")
```

**With sizing:**
```rust
image_!(
    "assets/hero-banner.jpg",
    width: px(400),
    height: px(200)
)
```

**With events:**
```rust
image_!(
    "assets/profile.png";
    on: |clicked: On<Clicked>| {
        println!("Profile image clicked");
    };
    on: |over: On<MouseOver>| {
        println!("Hovering over image");
    }
)
```

**Image gallery example:**
```rust
row_!(
    class: "justify-around align-center p-4";
    [
        image_!("assets/thumb1.jpg", width: px(150), height: px(150)),
        image_!("assets/thumb2.jpg", width: px(150), height: px(150)),
        image_!("assets/thumb3.jpg", width: px(150), height: px(150))
    ]
)
```

### Link

The `link_!` widget creates clickable links that open in the browser.

**Basic Usage:**
```rust
link_!("https://rust-lang.org/")
```

**With custom text:**
```rust
link_!("Visit Rust Website", url: "https://rust-lang.org/")
```

**With styling and events:**
```rust
link_!(
    "Documentation", 
    url: "https://docs.rs/makara",
    class: "is-primary";
    on: |clicked: On<Clicked>| {
        println!("Documentation link clicked");
    }
)
```

**Footer links example:**
```rust
row_!(
    class: "justify-center align-center p-3";
    [
        link_!("Privacy Policy", url: "https://example.com/privacy", class: "mr-4"),
        link_!("Terms of Service", url: "https://example.com/terms", class: "mr-4"),
        link_!("Contact Us", url: "https://example.com/contact")
    ]
)
```

## Progress & Loading Widgets

### Progress Bar

The `progress_bar_!` widget shows linear progress indication.

**Basic Usage:**
```rust
progress_bar_!()
```

**With styling:**
```rust
progress_bar_!(
    class: "is-primary p-2",
    width: px(300)
)
```

**With progress tracking:**
```rust
progress_bar_!(
    class: "is-success";
    on: |change: On<Change<f32>>| {
        println!("Progress: {:.1}%", change.data * 100.0);
    }
)
```

**Loading interface example:**
```rust
column_!(
    class: "justify-center align-center p-4";
    [
        text_!("Loading...", class: "mb-2"),
        progress_bar_!(class: "is-info", width: px(250)),
        text_!("Please wait while we process your request", 
               font_size: 12.0, class: "mt-2")
    ]
)
```

### Circular

The `circular_!` widget shows circular/spinner progress indication.

**Basic Usage:**
```rust
circular_!()
```

**With styling:**
```rust
circular_!(
    class: "is-primary p-3",
    width: px(50),
    height: px(50)
)
```

**Loading spinner example:**
```rust
column_!(
    class: "justify-center align-center p-4";
    [
        circular_!(class: "is-info mb-3"),
        text_!("Processing...", class: "is-info")
    ]
)
```

## Modal

The `modal_!` widget creates overlay dialogs and is independent of the root widget hierarchy.

**Basic Usage:**
```rust
commands.spawn(
    modal_!([
        column_!([
            text_!("Are you sure?"),
            button_!("Confirm")
        ])
    ])
);
```

**Confirmation dialog:**
```rust
modal_!(
    on: |active: On<Active>| {
        println!("Modal opened");
    };
    on: |inactive: On<Inactive>| {
        println!("Modal closed"); 
    };
    [
        column_!(
            class: "justify-center align-center p-4",
            background_color: Color::WHITE,
            width: px(300),
            height: px(200);
            [
                text_!("Delete Confirmation", font_size: 18.0, class: "mb-3"),
                text_!("Are you sure you want to delete this item?", class: "mb-4"),
                row_!(
                    class: "justify-center align-center";
                    [
                        button_!("Cancel", class: "is-secondary mr-2"),
                        button_!("Delete", class: "is-danger")
                    ]
                )
            ]
        )
    ]
)
```

**Settings modal:**
```rust
modal_!([
    column_!(
        class: "justify-start align-stretch p-4",
        background_color: Color::WHITE,
        width: px(400),
        height: px(500);
        [
            text_!("Settings", font_size: 24.0, class: "mb-4"),
            
            checkbox_!("Enable notifications", class: "mb-2"),
            checkbox_!("Auto-save documents", class: "mb-2"),
            
            text_!("Theme", class: "mb-1"),
            select_!("Choose theme...", choices: &["Light", "Dark", "Auto"], class: "mb-4"),
            
            row_!(
                class: "justify-end align-center";
                [
                    button_!("Cancel", class: "mr-2"),
                    button_!("Save", class: "is-primary")
                ]
            )
        ]
    )
])
```

## Built-in Classes

All widgets support built-in utility classes:

### Color Classes
- `is-primary`, `is-primary-dark`
- `is-link`, `is-link-dark`  
- `is-info`, `is-info-dark`
- `is-success`, `is-success-dark`
- `is-warning`, `is-warning-dark`
- `is-danger`, `is-danger-dark`

### Spacing Classes (Container widgets only)
- Margin: `m-0` to `m-6`, `mt-2`, `mr-3`, `mb-1`, `ml-4`, `mx-2`, `my-3`
- Padding: `p-0` to `p-6`, `pt-2`, `pr-3`, `pb-1`, `pl-4`, `px-2`, `py-3`

### Alignment Classes (Container widgets only)
- Justify: `justify-start`, `justify-center`, `justify-end`, `justify-between`, `justify-around`, `justify-evenly`
- Align: `align-start`, `align-center`, `align-end`, `align-stretch`, `align-baseline`

## Complete Example

Here's a complete application example using multiple widgets:

```rust
use makara::prelude::*;
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(
        root_!(
            class: "justify-center align-center p-4",
            background_color: Color::srgb(0.9, 0.9, 0.9);
            [
                column_!(
                    class: "justify-start align-stretch p-4",
                    background_color: Color::WHITE,
                    width: px(500);
                    [
                        // Header
                        text_!("User Profile", font_size: 24.0, class: "mb-4"),
                        
                        // Profile image
                        row_!(
                            class: "justify-center align-center mb-4";
                            [
                                image_!("assets/avatar.png", width: px(100), height: px(100))
                            ]
                        ),
                        
                        // Form fields
                        text_input_!("Full Name...", class: "mb-2"),
                        text_input_!("Email...", class: "is-info mb-2"),
                        
                        // Preferences
                        text_!("Preferences", font_size: 16.0, class: "mb-2"),
                        checkbox_!("Email notifications", class: "mb-1"),
                        checkbox_!("Dark mode", class: "mb-3"),
                        
                        // Theme selection
                        select_!("Theme...", choices: &["Light", "Dark", "Auto"], class: "mb-4"),
                        
                        // Actions
                        row_!(
                            class: "justify-end align-center";
                            [
                                button_!("Cancel", class: "mr-2"),
                                button_!("Save Profile", class: "is-primary")
                            ]
                        )
                    ]
                )
            ]
        )
    );
}
```

For detailed API documentation, visit the [Rust docs](https://docs.rs/makara/latest/makara/).
