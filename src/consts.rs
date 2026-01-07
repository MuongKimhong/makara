//! Constant values used by Makara's widgets.

use bevy::prelude::*;

// For button
pub const LIGHT_BUTTON_BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const DARK_BUTTON_BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const DEFAULT_BUTTON_BORDER_RADIUS: BorderRadius = BorderRadius::all(Val::Px(5.0));

// For checkbox
pub const CHECKBOX_CHECKED_COLOR: Color = Color::srgba(0.0, 0.0, 0.9, 1.0);
pub const CHECKBOX_UNCHECKED_COLOR: Color = Color::NONE;

// For radio
pub const RADIO_CHECKED_COLOR: Color = Color::srgba(0.0, 0.0, 0.9, 1.0);
pub const RADIO_UNCHECKED_COLOR: Color = Color::NONE;

pub const LIGHT_THEME_TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const DARK_THEME_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const DEFAULT_TEXT_FONT_SIZE: f32 = 12.5;

// For root and modal
pub const LIGHT_THEME_BG_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);
pub const DARK_THEME_BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

// For Link
pub const DEFAULT_LINK_COLOR: Color = Color::srgb(0.0, 0.0, 0.6);

// Slider
pub const SLIDER_THUMB_COLOR: Color = Color::srgba(0.0, 0.0, 0.9, 1.0);
pub const LIGHT_SLIDER_BG_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
pub const DARK_SLIDER_BG_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

// Progress bar
pub const LIGHT_PROGRESS_BAR_BG_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
pub const DARK_PROGRESS_BAR_BG_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const DEFAULT_PROGRESS_VALUE_COLOR: Color = Color::srgba(0.0, 0.0, 0.9, 1.0);

// Circular
pub const DEFAULT_CIRCULAR_VALUE_COLOR: Color = Color::srgba(0.0, 0.0, 0.9, 1.0);
pub const LIGHT_CIRCULAR_BG_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
pub const DARK_CIRCULAR_BG_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

// Tooltip
pub const LIGHT_TOOLTIP_BG_COLOR: Color = Color::srgba(0.9, 0.9, 0.9, 0.6);
pub const DARK_TOOLTIP_BG_COLOR: Color = Color::srgba(0.15, 0.15, 0.15, 0.6);
pub const DEFAULT_TOOLTIP_BORDER_RADIUS: BorderRadius = BorderRadius::all(Val::Px(5.0));

// Text input & Text area
pub const LIGHT_CARET_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const DARK_CARET_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const LIGHT_INPUT_BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const DARK_INPUT_BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const SELECTION_COLOR: Color = Color::srgba(0.0, 0.0, 1.0, 0.8);
pub const DEFAULT_INPUT_BORDER_RADIUS: BorderRadius = BorderRadius::all(Val::Px(5.0));
pub const LIGHT_PLACEHOLDER_COLOR: Color = Color::srgb(0.6, 0.6, 0.7);
pub const DARK_PLACEHOLDER_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

// scroll
pub const DEFAULT_SCROLL_HEIGHT: f32 = 15.0;
