use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        root_!(
            background_color: Color::srgb(1.0, 0.5, 0.5),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center;

            [
                button_!("Show center modal"; on: |_click: On<Clicked>, mut modal_q: ModalQuery| {
                    if let Some(mut modal) = modal_q.find_by_id("center-modal") {
                        modal.show();
                    }
                }),
                button_!(
                    "Show bottom right modal & without animation";

                    on: |_click: On<Clicked>, mut modal_q: ModalQuery| {
                        if let Some(mut modal) = modal_q.find_by_id("bottom-right-modal") {
                            modal.show();
                        }
                    }
                ),
            ]
        )
    );

    // Modal is independent and not part of root hierachy.
    commands.spawn(
        modal_!(id: "center-modal"; [
            column_!(
                padding: px(10),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center;

                [
                    text_!("This is modal with center position"),
                    button_!(
                        "Hide modal",
                        margin_top: px(10);

                        on: |_click: On<Clicked>, mut modal_q: ModalQuery| {
                            if let Some(mut modal) = modal_q.find_by_id("center-modal") {
                                modal.hide();
                            }
                        }
                    )
                ]
            )
        ])
    );

    commands.spawn(
        modal_!(
            id: "bottom-right-modal",
            position: ModalPosition::BottomRight,
            scale_animation: false;

            [
                column_!(
                    padding: px(10),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center;

                    [
                        text_!("This is modal with bottom right position"),
                        button_!("Hide modal"; on: |_click: On<Clicked>, mut modal_q: ModalQuery| {
                            if let Some(mut modal) = modal_q.find_by_id("bottom-right-modal") {
                                modal.hide();
                            }
                        })
                    ]
                )
            ]
        )
    );
}
