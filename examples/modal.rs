use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        root()
            .background_color(Color::srgb(1.0, 0.5, 0.5))
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            (
                button("Show center modal").build(),
                observe(|_click: On<Clicked>, mut modal_q: ModalQuery| {
                    if let Some(mut modal) = modal_q.find_by_id("center-modal") {
                        modal.show();
                    }
                })
            ),
            (
                button("Show bottom right modal & without animation").build(),
                observe(|_click: On<Clicked>, mut modal_q: ModalQuery| {
                    if let Some(mut modal) = modal_q.find_by_id("bottom-right-modal") {
                        modal.show();
                    }
                })
            )
        ]
    ));

    // Modal is independent and not part of root hierachy.
    commands.spawn((
        modal().id("center-modal").build(),

        // wrap modal content in a column widget.
        children![
            (
                column()
                    .padding(px(10))
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center)
                    .build(),

                children![
                    text("This is modal with center position").build(),

                    (
                        button("Hide modal").margin_top(px(10)).build(),
                        observe(|_click: On<Clicked>, mut modal_q: ModalQuery| {
                            if let Some(mut modal) = modal_q.find_by_id("center-modal") {
                                modal.hide();
                            }
                        })
                    )
                ]
            )
        ]
    ));

    commands.spawn((
        modal()
            .id("bottom-right-modal")
            .position(ModalPosition::BottomRight)
            .scale_animation(false)
            .build(),

        // wrap modal content in a column widget.
        children![
            (
                column()
                    .padding(px(10))
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center)
                    .build(),

                children![
                    text("This is modal with bottom right position").build(),

                    (
                        button("Hide modal").margin_top(px(10)).build(),
                        observe(|_click: On<Clicked>, mut modal_q: ModalQuery| {
                            if let Some(mut modal) = modal_q.find_by_id("bottom-right-modal") {
                                modal.hide();
                            }
                        })
                    )
                ]
            )
        ]
    ));
}
