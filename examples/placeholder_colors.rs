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
            background_color: Color::srgb(0.95, 0.95, 0.95),
            class: "justify-center align-center p-4";
            
            [
                column_!(
                    width: percent(80),
                    background_color: Color::WHITE,
                    class: "justify-start align-center p-4";
                    
                    [
                        text_!(
                            "Placeholder Colors Test",
                            font_size: 28.0,
                            color: Color::srgb(0.2, 0.2, 0.2),
                            class: "mb-4"
                        ),
                        
                        text_!(
                            "Test how placeholder text looks with different color classes",
                            font_size: 16.0,
                            color: Color::srgb(0.5, 0.5, 0.5),
                            class: "mb-4"
                        ),

                        text_!(
                            "Light Theme Placeholders",
                            font_size: 20.0,
                            color: Color::srgb(0.3, 0.3, 0.3),
                            class: "mb-3"
                        ),
                        
                        // Primary placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-primary placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your primary text...",
                                    class: "is-primary",
                                    width: px(300)
                                )
                            ]
                        ),
                        
                        // Link placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-link placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your link text...",
                                    class: "is-link",
                                    width: px(300)
                                )
                            ]
                        ),
                        
                        // Info placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-info placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your info text...",
                                    class: "is-info",
                                    width: px(300)
                                )
                            ]
                        ),
                        
                        // Success placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-success placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your success text...",
                                    class: "is-success",
                                    width: px(300)
                                )
                            ]
                        ),
                        
                        // Warning placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-warning placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your warning text...",
                                    class: "is-warning",
                                    width: px(300)
                                )
                            ]
                        ),
                        
                        // Danger placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-danger placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your danger text...",
                                    class: "is-danger",
                                    width: px(300)
                                )
                            ]
                        ),
                        
                        // Default placeholder for comparison
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("Default (no class) placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your default text...",
                                    width: px(300)
                                )
                            ]
                        ),

                        text_!(
                            "Dark Theme Placeholders",
                            font_size: 20.0,
                            color: Color::srgb(0.3, 0.3, 0.3),
                            class: "mb-3 mt-4"
                        ),

                        // Dark Primary placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-primary-dark placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your primary dark text...",
                                    class: "is-primary-dark",
                                    width: px(300)
                                )
                            ]
                        ),

                        // Dark Link placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-link-dark placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your link dark text...",
                                    class: "is-link-dark",
                                    width: px(300)
                                )
                            ]
                        ),

                        // Dark Info placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-info-dark placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your info dark text...",
                                    class: "is-info-dark",
                                    width: px(300)
                                )
                            ]
                        ),

                        // Dark Success placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-success-dark placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your success dark text...",
                                    class: "is-success-dark",
                                    width: px(300)
                                )
                            ]
                        ),

                        // Dark Warning placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-warning-dark placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your warning dark text...",
                                    class: "is-warning-dark",
                                    width: px(300)
                                )
                            ]
                        ),

                        // Dark Danger placeholder
                        column_!(
                            class: "align-start mb-3";
                            [
                                text_!("is-danger-dark placeholder:", font_size: 14.0, class: "mb-1"),
                                text_input_!(
                                    "Enter your danger dark text...",
                                    class: "is-danger-dark",
                                    width: px(300)
                                )
                            ]
                        ),
                        
                        text_!(
                            "Type in any field to see how the placeholder disappears and styled text appears",
                            font_size: 12.0,
                            color: Color::srgb(0.6, 0.6, 0.6),
                            class: "mt-4"
                        )
                    ]
                )
            ]
        )
    );
}