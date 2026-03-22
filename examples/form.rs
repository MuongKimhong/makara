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

    column_!(
        align_items: AlignItems::Center;

        [
            text_input_!(
                "Enter email address",
                width: px(200);

                on: |change: On<Change<String>>, mut user: ResMut<User>, mut txt_q: TextQuery| {
                    if let Some(txt) = txt_q.find_by_id("info-text") {
                        user.email = change.data.clone();
                        txt.text.value.0 = user.get_info();
                    }
                }
            ),

            text_input_!(
                "Enter name",
                margin_top: px(5),
                width: px(200);

                on: |change: On<Change<String>>, mut user: ResMut<User>, mut txt_q: TextQuery| {
                    if let Some(txt) = txt_q.find_by_id("info-text") {
                        user.name = change.data.clone();
                        txt.text.value.0 = user.get_info();
                    }
                }
            ),

            radio_group_!(
                margin_top: px(5);

                on: |change: On<Change<String>>, mut user: ResMut<User>, mut txt_q: TextQuery| {
                    if let Some(txt) = txt_q.find_by_id("info-text") {
                        user.gender = change.data.clone();
                        txt.text.value.0 = user.get_info();
                    }
                };

                [ radio_!("Male"), radio_!("Female") ]
            ),

            file_picker().build(),
        ]
    )
}

fn setup(mut commands: Commands, user: Res<User>) {
    commands.spawn(
        root_!(
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;

            [
                text_!(&user.get_info(), margin_bottom: px(5), id: "info-text"),
                form_container()
            ]
        )
    );
}
