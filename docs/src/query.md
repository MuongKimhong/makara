Each widget type has its own query and blueprint.
Once widgets are built, you can query any widgets within systems using 
its corresponding query struct.

For example, let's say we have a text and a button. Once the button is clicked,
we want to update the text.

```rust
column_!([
    text_!("Hello world", id: "my-text"),
    button_!("Click me"; on: |_clicked: On<Clicked>, mut text_q: TextQuery| {
        if let Some(text) = text_q.find_by_id("my-text") {
            text.text.value.0 = "Hello Mom".to_string();
        }
    })
]);
```

In the example above, once the button is clicked, we query text widgets using `TextQuery`.
We then find the specific widget using its `id`. This is similar to how `javascript` finds
HTML elements within the **DOM**.

Of course, you can query a widget using **class** or **entity** instead of **id**. Querying
with **class** is useful if you want to fetch multiple widgets at same time, but requires
you to write extra code.

```rust
// change all text colors when button is clicked.
column_!([
    text_!("Hello world", class: "my-text"),
    text_!("Hello mom", class: "my-text"),
    text_!("Hello friend", class: "my-text"),
    text_!("Hello stranger", class: "my-text"),
    
    button_!("Click me"; on: |_clicked: On<Clicked>, mut text_q: TextQuery| {
        // find_by_class returns list of entity
        let text_entities = text_q.find_by_class("my-text");
    
        // iterate thru the list
        for entity in text_entities.into_iter() {
            if let Some(text) = text_q.find_by_entity(entity) {
                text.text.color.0 = Color::RED;
            }
        }
    })
]);
```

For more information about widget query and its blueprint, see [widgets](./widgets.md).
