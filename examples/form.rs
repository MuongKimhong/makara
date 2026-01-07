use makara::prelude::*;
use bevy::prelude::*;

// User bevy's resource as app state.
// You can have as many resource as you want!
#[derive(Resource)]
struct User {
    email: String,
    name: String,
    gender: String
}

impl Default for User {
    fn default() -> Self {
        Self {
            email: "unknown".to_string(),
            name: "unknown".to_string(),
            gender: "unknown".to_string(),
        }
    }
}

impl User {
    fn get_info(&self) -> String {
        format!(
            "Email: {}, Name: {}, Gender: {}",
            self.email,
            self.name,
            self.gender
        )
    }
}

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .insert_resource(User::default())
        .add_systems(Startup, setup)
        .run();
}

fn form_container() -> impl Bundle {
    (
        column().align_items(AlignItems::Center).build(),
        children![
            (
                text_input("Enter email address").width(px(200)).build(),
                observe(|change: On<Change<String>>, mut user: ResMut<User>, mut txt_q: TextQuery| {
                    if let Some(txt) = txt_q.find_by_id("info-text") {
                        user.email = change.data.clone();
                        txt.text.value.0 = user.get_info();
                    }
                })
            ),

            (
                text_input("Enter name").margin_top(px(5)).width(px(200)).build(),
                observe(|change: On<Change<String>>, mut user: ResMut<User>, mut txt_q: TextQuery| {
                    if let Some(txt) = txt_q.find_by_id("info-text") {
                        user.name = change.data.clone();
                        txt.text.value.0 = user.get_info();
                    }
                })
            ),

            (
                radio_group().margin_top(px(5)).build(),
                children![
                    radio("Male").build(),
                    radio("Female").build(),
                ],
                observe(|change: On<Change<String>>, mut user: ResMut<User>, mut txt_q: TextQuery| {
                    if let Some(txt) = txt_q.find_by_id("info-text") {
                        user.gender = change.data.clone();
                        txt.text.value.0 = user.get_info();
                    }
                })
            )
        ]
    )
}

fn setup(mut commands: Commands, user: Res<User>) {
    // bevy needs this
    commands.spawn(Camera2d);

    // spawning root & text widget.
    commands.spawn((
        root()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            text(&user.get_info()).margin_bottom(px(5)).id("info-text").build(),
            form_container()
        ]
    ));
}
