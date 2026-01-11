use makara::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
struct ImageUrlList {
    list: Vec<String>,
    current_index: usize
}

impl ImageUrlList {
    pub fn new() -> Self {
        Self {
            list: vec![
                "https://upload.wikimedia.org/wikipedia/en/5/5f/Original_Doge_meme.jpg".to_string(),
                "dog_meme.jpg".to_string(),
                "cat_meme.jpg".to_string()
            ],
            current_index: 0
        }
    }

    pub fn next(&mut self) {
        if self.list.is_empty() {
            return;
        }

        self.current_index = (self.current_index + 1) % self.list.len();
    }

    pub fn get_current_url(&self) -> Option<String> {
        self.list.get(self.current_index).cloned()
    }
}

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .insert_resource(ImageUrlList::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        root()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            (button("Change Image").build(), observe(on_button_click)),

            (
                image("dog_meme_2.webp")
                    .width(px(200))
                    .id("image")
                    .margin_top(px(10))
                    .build(),

                observe(|_loading: On<Loading>| println!("image is loading")),
                observe(|_loaded: On<Loaded>| println!("image is loaded"))
            )
        ]
    ));
}

fn on_button_click(
    _clicked: On<Clicked>,
    mut image_list: ResMut<ImageUrlList>,
    mut image_q: ImageQuery
) {
    if let Some(mut image) = image_q.find_by_id("image") {
        let new_url = image_list.get_current_url();

        if let Some(url) = new_url {
            image.set_path(url);
            image_list.next();
        }
    }
}
