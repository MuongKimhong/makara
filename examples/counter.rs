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
    commands.spawn(
        root_!(
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;

            [
                text_!(&format!("Count: {:?}", count.0), id: "#count-text"),

                row_!(
                    justify_content: JustifyContent::Center,
                    margin_top: px(5);

                    [
                        button_!("+", margin_right: px(5); on: on_plus_click),
                        button_!("-", margin_left: px(5); on: on_minus_click),
                    ]
                )
            ]
        )
    );
}
