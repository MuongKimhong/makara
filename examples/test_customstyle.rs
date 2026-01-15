use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, (setup, setup_style).chain())
        .run();
}

fn setup(mut commands: Commands) {
    // spawning root & text widget.

    commands.spawn((
        root()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            (
                column().id("column").build(),
                children![
                    text("Text one").id("text-one").class("text").build(),
                    text("Text two").class("text").build(),

                    button("button 1").id("btn-one").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),
                    button("button 2").class("button").build(),

                    circular().id("circular-one").class("circular").build(),
                    circular().class("circular").build(),

                    progress_bar().id("bar-one").class("bar").build(),
                    progress_bar().class("bar").build(),

                    (
                        dropdown("Dropdown").id("dropdown").build(),
                        children![
                            button("dropdown option 1").class("button").build(),
                            button("dropdown option 2").class("button").build(),
                        ]
                    ),

                    slider(0.0, 100.0).class("slider").build()
                ]
            ),
            (
                row().class("row").build(),
                children![
                    text_input("Input one").id("text-input").class("input").build(),
                    text_input("Input two").class("input").build(),

                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                    checkbox("choose something").class("checkbox").build(),
                ]
            )
        ]
    ));
}

fn setup_style(mut custom_style: ResMut<CustomStyle>) {
    custom_style.bind_id(
        "text-one",
        Style::new().color(Color::srgb(1.0, 0.0, 0.0))
    );

    custom_style.bind_class(
        "text",
        Style::new().font_size(100.0)
    );

    custom_style.bind_id(
        "btn-one",
        Style::new().color(Color::srgb(1.0, 0.0, 0.0))
    );

    custom_style.bind_class(
        "button",
        Style::new()
            .background_color(Color::srgb(1.0, 0.0, 0.0))
            .font_size(40.0)
    );

    custom_style.bind_class(
        "checkbox",
        Style::new()
            .background_color(Color::srgb(1.0, 0.0, 0.0))
            .font_size(40.0)
    );

    custom_style.bind_id(
        "circular-one",
        Style::new()
            .spin_color(Color::srgba(1.0, 0.0, 0.0, 0.0))
    );

    custom_style.bind_class(
        "circular",
        Style::new()
            .width(px(50))
    );

    custom_style.bind_id(
        "bar-one",
        Style::new()
            .progress_color(Color::srgb(1.0, 0.0, 0.0))
    );

    custom_style.bind_class(
        "bar",
        Style::new()
            .background_color(Color::srgb(0.0, 1.0, 0.0))
    );

    custom_style.bind_id(
        "dropdown",
        Style::new()
            .padding(UiRect::all(px(10)))
    );

    custom_style.bind_class(
        "slider::thumb",
        Style::new()
            .background_color(Color::srgb(0.0, 1.0, 0.0))
    );

    custom_style.bind_id(
        "text-input",
        Style::new()
            .padding(UiRect::all(px(20)))
    );

    custom_style.bind_class(
        "input",
        Style::new()
            .background_color(Color::srgb(0.0, 1.0, 0.0))
    );
}
