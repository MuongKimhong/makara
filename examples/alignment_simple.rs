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
            background_color: Color::srgb(0.1, 0.1, 0.2),
            class: "justify-center align-center";
            
            [
                column_!(
                    width: percent(90),
                    height: percent(90),
                    background_color: Color::WHITE,
                    class: "justify-start align-center p-4";
                    
                    [
                        // Title
                        text_!(
                            "Container Alignment Classes",
                            font_size: 28.0,
                            color: Color::srgb(0.2, 0.2, 0.8),
                            class: "mb-4"
                        ),
                        
                        text_!(
                            "Alignment classes work on container widgets: root, row, column, scroll",
                            font_size: 16.0,
                            color: Color::srgb(0.5, 0.5, 0.5),
                            class: "mb-4"
                        ),
                        
                        // Row with justify-center
                        row_!(
                            width: percent(100),
                            height: px(60),
                            background_color: Color::srgb(0.95, 0.95, 1.0),
                            class: "justify-center align-center mb-3";
                            
                            [
                                text_!("row with justify-center", color: Color::srgb(0.3, 0.3, 0.7))
                            ]
                        ),
                        
                        // Row with justify-between
                        row_!(
                            width: percent(100),
                            height: px(60),
                            background_color: Color::srgb(0.95, 1.0, 0.95),
                            class: "justify-between align-center p-3 mb-3";
                            
                            [
                                text_!("Left", color: Color::srgb(0.2, 0.6, 0.2)),
                                text_!("justify-between", color: Color::srgb(0.2, 0.6, 0.2)),
                                text_!("Right", color: Color::srgb(0.2, 0.6, 0.2))
                            ]
                        ),
                        
                        // Column with align-center
                        column_!(
                            width: percent(100),
                            height: px(120),
                            background_color: Color::srgb(1.0, 0.95, 0.95),
                            class: "justify-center align-center p-3 mb-3";
                            
                            [
                                text_!("column with align-center", color: Color::srgb(0.7, 0.2, 0.2)),
                                text_!("Content is centered", color: Color::srgb(0.5, 0.5, 0.5))
                            ]
                        ),
                        
                        // Scroll container example
                        scroll_!(
                            width: percent(100),
                            height: px(100),
                            background_color: Color::srgb(0.95, 0.95, 0.95),
                            class: "justify-center align-center p-2";
                            
                            [
                                column_!(
                                    class: "justify-start align-center";
                                    [
                                        text_!("scroll container with alignment", color: Color::srgb(0.4, 0.4, 0.4)),
                                        text_!("Line 2", color: Color::srgb(0.4, 0.4, 0.4)),
                                        text_!("Line 3", color: Color::srgb(0.4, 0.4, 0.4)),
                                        text_!("Line 4", color: Color::srgb(0.4, 0.4, 0.4)),
                                        text_!("Line 5", color: Color::srgb(0.4, 0.4, 0.4))
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        )
    );
}