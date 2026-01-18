use makara::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, (
            setup_home_view,
            setup_sale_view,
            setup_company_view,
            setup_router,
            setup_style
        ))
        // we register route change event listener here, because RouteChanged is
        // a global event.
        .add_observer(on_route_changed)
        .run();
}

fn on_route_changed(route_changed: On<RouteChanged>, mut btn_q: ButtonQuery) {
    let entities = btn_q.find_by_class("nav_button");

    for entity in entities.into_iter() {
        if let Some(btn) = btn_q.find_by_entity(entity) {
            if btn.text.value.0.to_lowercase() == route_changed.route {
                btn.text.color.0 = Color::srgb(0.0, 0.0, 1.0);
            }
            else {
                btn.text.color.0 = LIGHT_THEME_TEXT_COLOR;
            }
        }
    }
}

fn navigation_buttons() -> impl Bundle {
    (
        row()
            .margin_bottom(px(20))
            .justify_content(JustifyContent::Center)
            .build(),

        children![
            (
                button("Home").class("nav_button").build(),
                observe(|_clicked: On<Clicked>, mut router: ResMut<Router>| {
                    router.nagivate("home", ());
                })
            ),
            text(" / ").class("nav_button").build(),
            (
                button("Sale").class("nav_button").build(),
                observe(|_clicked: On<Clicked>, mut router: ResMut<Router>| {
                    router.nagivate("sale", ());
                })
            ),
            text(" / ").class("nav_button").build(),
            (
                button("Company").class("nav_button").build(),
                observe(|_clicked: On<Clicked>, mut router: ResMut<Router>| {
                    router.nagivate("company", ());
                })
            ),
        ]
    )
}

fn setup_home_view(mut commands: Commands) {
    commands.spawn((
        root()
            .route("home")
            .class("root_center")
            .build(),

        children![
            navigation_buttons(),
            text("Welcome to Home view").font_size(25.0).build()
        ]
    ));
}


fn setup_sale_view(mut commands: Commands) {
    commands.spawn((
        root()
            .route("sale")
            .class("root_center")
            .build(),

        children![
            navigation_buttons(),
            text("Sale view is for anything related to sales and business").font_size(25.0).build()
        ]
    ));
}


fn setup_company_view(mut commands: Commands) {
    commands.spawn((
        root()
            .route("company")
            .class("root_center")
            .build(),

        children![
            navigation_buttons(),
            text("This company is all about love.").font_size(25.0).build()
        ]
    ));
}

fn setup_router(mut router: ResMut<Router>) {
    router.register_routes(["home", "sale", "company"]);
    router.default_route("home", ());
}

fn setup_style(mut style: ResMut<CustomStyle>) {
    style.bind_class(
        "nav_button",
        Style::new()
            .background_color(Color::NONE)
            .shadow(BoxShadow::default())
            .font_size(16.0)
    );

    style.bind_class(
        "root_center",
        Style::new()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
    );
}
