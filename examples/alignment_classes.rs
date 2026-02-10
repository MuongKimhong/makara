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
            background_color: Color::srgb(0.9, 0.9, 0.9),
            class: "justify-center align-center p-4";
            
            [
                // Main container
                column_!(
                    class: "justify-start align-stretch",
                    width: percent(90),
                    height: percent(90),
                    background_color: Color::WHITE;
                    
                    [
                        // Header
                        row_!(
                            class: "justify-center align-center p-3 mb-4",
                            background_color: Color::srgb(0.2, 0.3, 0.8);
                            
                            [
                                text_!(
                                    "Container Alignment Classes Demo",
                                    font_size: 24.0,
                                    color: Color::WHITE
                                )
                            ]
                        ),
                        
                        // Info text
                        row_!(
                            class: "justify-center align-center mb-4";
                            [
                                text_!(
                                    "Alignment classes work on: root, row, column, scroll containers",
                                    font_size: 16.0,
                                    color: Color::srgb(0.5, 0.5, 0.5)
                                )
                            ]
                        ),
                        
                        // Justify Content Examples
                        column_!(
                            class: "mb-4";
                            
                            [
                                text_!("Justify Content Examples:", font_size: 18.0, class: "mb-2"),
                                
                                // justify-start
                                row_!(
                                    class: "justify-start align-center p-2 mb-2",
                                    background_color: Color::srgb(0.95, 0.95, 1.0);
                                    
                                    [
                                        text_!("Start", color: Color::srgb(0.8, 0.2, 0.2)),
                                        text_!("Middle", color: Color::srgb(0.2, 0.6, 0.2)),
                                        text_!("End", color: Color::srgb(0.2, 0.2, 0.8))
                                    ]
                                ),
                                
                                // justify-center
                                row_!(
                                    class: "justify-center align-center p-2 mb-2",
                                    background_color: Color::srgb(0.95, 1.0, 0.95);
                                    
                                    [
                                        text_!("Start", color: Color::srgb(0.8, 0.2, 0.2)),
                                        text_!("Middle", color: Color::srgb(0.2, 0.6, 0.2)),
                                        text_!("End", color: Color::srgb(0.2, 0.2, 0.8))
                                    ]
                                ),
                                
                                // justify-end
                                row_!(
                                    class: "justify-end align-center p-2 mb-2",
                                    background_color: Color::srgb(1.0, 0.95, 0.95);
                                    
                                    [
                                        text_!("Start", color: Color::srgb(0.8, 0.2, 0.2)),
                                        text_!("Middle", color: Color::srgb(0.2, 0.6, 0.2)),
                                        text_!("End", color: Color::srgb(0.2, 0.2, 0.8))
                                    ]
                                ),
                                
                                // justify-between
                                row_!(
                                    class: "justify-between align-center p-2 mb-2",
                                    background_color: Color::srgb(1.0, 1.0, 0.95);
                                    
                                    [
                                        text_!("Start", color: Color::srgb(0.8, 0.2, 0.2)),
                                        text_!("Middle", color: Color::srgb(0.2, 0.6, 0.2)),
                                        text_!("End", color: Color::srgb(0.2, 0.2, 0.8))
                                    ]
                                )
                            ]
                        ),
                        
                        // Align Items Examples
                        column_!(
                            class: "mb-4";
                            
                            [
                                text_!("Align Items Examples:", font_size: 18.0, class: "mb-2"),
                                
                                row_!(
                                    class: "justify-around";
                                    
                                    [
                                        // align-start
                                        column_!(
                                            class: "align-start justify-center p-2 mr-2",
                                            width: px(120),
                                            height: px(100),
                                            background_color: Color::srgb(0.95, 0.95, 1.0);
                                            
                                            [
                                                text_!("align-start", font_size: 12.0),
                                                text_!("Content", font_size: 10.0, color: Color::srgb(0.6, 0.6, 0.6))
                                            ]
                                        ),
                                        
                                        // align-center
                                        column_!(
                                            class: "align-center justify-center p-2 mr-2",
                                            width: px(120),
                                            height: px(100),
                                            background_color: Color::srgb(0.95, 1.0, 0.95);
                                            
                                            [
                                                text_!("align-center", font_size: 12.0),
                                                text_!("Content", font_size: 10.0, color: Color::srgb(0.6, 0.6, 0.6))
                                            ]
                                        ),
                                        
                                        // align-end
                                        column_!(
                                            class: "align-end justify-center p-2",
                                            width: px(120),
                                            height: px(100),
                                            background_color: Color::srgb(1.0, 0.95, 0.95);
                                            
                                            [
                                                text_!("align-end", font_size: 12.0),
                                                text_!("Content", font_size: 10.0, color: Color::srgb(0.6, 0.6, 0.6))
                                            ]
                                        )
                                    ]
                                )
                            ]
                        ),
                        
                        // Combined Examples
                        column_!(
                            [
                                text_!("Combined Alignment Examples:", font_size: 18.0, class: "mb-2"),
                                
                                row_!(
                                    class: "justify-between";
                                    
                                    [
                                        // Top-left
                                        column_!(
                                            class: "justify-start align-start p-3",
                                            width: px(150),
                                            height: px(80),
                                            background_color: Color::srgb(0.9, 0.9, 1.0);
                                            
                                            [
                                                text_!("Top-Left", font_size: 14.0),
                                                text_!("🔝⬅️", font_size: 12.0)
                                            ]
                                        ),
                                        
                                        // Center-center
                                        column_!(
                                            class: "justify-center align-center p-3",
                                            width: px(150),
                                            height: px(80),
                                            background_color: Color::srgb(0.9, 1.0, 0.9);
                                            
                                            [
                                                text_!("Center", font_size: 14.0),
                                                text_!("🎯", font_size: 12.0)
                                            ]
                                        ),
                                        
                                        // Bottom-right
                                        column_!(
                                            class: "justify-end align-end p-3",
                                            width: px(150),
                                            height: px(80),
                                            background_color: Color::srgb(1.0, 0.9, 0.9);
                                            
                                            [
                                                text_!("Bottom-Right", font_size: 14.0),
                                                text_!("🔽➡️", font_size: 12.0)
                                            ]
                                        ),
                                
                                        // Scroll container example
                                        text_!("Scroll Container Example:", font_size: 18.0, class: "mb-2 mt-4"),
                                
                                        scroll_!(
                                            width: percent(100),
                                            height: px(120),
                                            background_color: Color::srgb(0.95, 0.95, 0.95),
                                            class: "justify-center align-center p-3";
                                    
                                            [
                                                column_!(
                                                    class: "justify-start align-center";
                                                    [
                                                        text_!("Scrollable content with alignment", font_size: 14.0),
                                                        text_!("Line 2 - scroll to see more", font_size: 12.0, color: Color::srgb(0.6, 0.6, 0.6)),
                                                        text_!("Line 3 - centered alignment", font_size: 12.0, color: Color::srgb(0.6, 0.6, 0.6)),
                                                        text_!("Line 4 - in scroll container", font_size: 12.0, color: Color::srgb(0.6, 0.6, 0.6)),
                                                        text_!("Line 5 - more content below", font_size: 12.0, color: Color::srgb(0.6, 0.6, 0.6)),
                                                        text_!("Line 6 - keep scrolling", font_size: 12.0, color: Color::srgb(0.6, 0.6, 0.6))
                                                    ]
                                                )
                                            ]
                                        )
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