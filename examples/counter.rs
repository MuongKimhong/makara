use makara::prelude::*;
use bevy::prelude::*;

// Use bevy's resource as an app state.
#[derive(Resource, Default)]
struct Count(i32);

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .insert_resource(Count(0)) // don't forget to insert resource
        .add_systems(Startup, setup)
        .run();
}

fn on_plus_click(_click: On<Clicked>, mut count: ResMut<Count>, mut text_q: TextQuery) {
    if let Some(text_widget) = text_q.find_by_id("#count-text") {
        count.0 += 1;
        text_widget.text.value.0 = format!("Count: {:?}", count.0);
    }
}

fn on_minus_click(_click: On<Clicked>, mut count: ResMut<Count>, mut text_q: TextQuery) {
    if let Some(text_widget) = text_q.find_by_id("#count-text") {
        count.0 -= 1;
        text_widget.text.value.0 = format!("Count: {:?}", count.0);
    }
}

fn setup(mut commands: Commands, count: Res<Count>) {
    // bevy needs this
    commands.spawn(Camera2d);

    commands.spawn((
        root()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            text(&format!("Count: {:?}", count.0))
                .id("#count-text")
                .build(),

            (
                row()
                    .margin_top(px(5))
                    .justify_content(JustifyContent::Center)
                    .build(),

                children![
                    (
                        button("+").margin_right(px(5)).build(),
                        observe(on_plus_click)
                    ),
                    (
                        button("-").margin_left(px(5)).build(),
                        observe(on_minus_click)
                    ),
                ]
            )
        ]
    ));
}
