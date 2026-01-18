//! Collection of events used by Makara's widgets.

use bevy::prelude::*;
use crate::routers::*;

/// This event fires when a widget is active.
/// This event is used by `checkbox`, `modal`, `dropdown`, `select`, `radio`.
#[derive(EntityEvent, Debug)]
pub struct Active<T> {
    pub entity: Entity,
    pub data: T
}

/// This event fires when a widget is inactive.
/// This event is used by `checkbox`, `modal`, `dropdown`, `select`, `radio`.
#[derive(EntityEvent, Debug)]
pub struct Inactive<T> {
    pub entity: Entity,
    pub data: T
}

/// This event fires when mouse cursor is over a widget.
/// This event is used by all widgets except `row`, `column` and `root`.
#[derive(EntityEvent, Debug)]
pub struct MouseOver {
    pub entity: Entity
}

/// This event fires when mouse cursor is moved out of a widget.
/// This event is used by all widgets except `row`, `column` and `root`.
#[derive(EntityEvent, Debug)]
pub struct MouseOut {
    pub entity: Entity
}

/// This event fires when `scroll` widget is being scrolled.
#[derive(EntityEvent, Debug)]
pub struct Scrolling {
    pub entity: Entity,

    /// Top position of the moving container within `scroll` widget.
    pub position: f32
}

/// This event fires when mouse is pressed on widgets.
/// This event is used by all widgets except `row`, `column` and `root`.
#[derive(EntityEvent, Debug)]
pub struct Clicked {
    pub entity: Entity
}

/// This event fires when value of a widget is changed.
/// This event is used by `progress_bar`, `circular`, `select`,
/// `slider`, `radio_group`, `image` and `text_input`.
#[derive(EntityEvent, Debug)]
pub struct Change<T> {
    pub entity: Entity,
    pub data: T
}

/// This event fires when a widget is spawned/built into UI world.
#[derive(EntityEvent, Debug)]
pub struct WidgetBuilt {
    pub entity: Entity
}

/// Event used to set slider value directly.
/// Requires slider entity and value to be set.
#[derive(EntityEvent, Debug)]
pub struct SetSliderValue {
    pub entity: Entity,
    pub value: f32
}

/// Event used to set checkbox state directly.
/// Requires checkbox entity and state to be set.
#[derive(EntityEvent, Debug)]
pub struct SetCheckboxState {
    pub entity: Entity,
    pub state: bool
}

/// Event used to set radio_group value directly.
/// Requires radio_group entity, radio entity and radio text to be set.
#[derive(EntityEvent, Debug)]
pub struct SetRadioGroupValue {
    /// `radio_group` entity.
    pub entity: Entity,

    /// `radio` entity (child of `radio_group`).
    pub radio_entity: Entity,

    /// `radio` text.
    pub radio_text: String
}


/// Event used to set progress bar value directly.
/// Requires progress bar entity, bar_type and value to be set.
/// `bar_type` can be "indeterminate" or "percentage".
/// If `bar_type` set to "indeterminate", `value` will be ignored,
#[derive(EntityEvent, Debug)]
pub struct SetProgressBarValue {
    pub entity: Entity,
    pub bar_type: String,
    pub value: f32
}

/// Event used to set circular value directly.
/// Requires circular entity, circular_type and value to be set.
/// `circular_type` can be "indeterminate" or "percentage".
/// If `circular_type` set to "indeterminate", `value` will be ignored,
#[derive(EntityEvent, Debug)]
pub struct SetCircularValue {
    pub entity: Entity,
    pub circular_type: String,
    pub value: f32
}

/// Event used to set select value directly.
/// Requires select entity and value to be set.
#[derive(EntityEvent, Debug)]
pub struct SetSelectValue {
    pub entity: Entity,
    pub value: String
}

/// Event used to set text_input value directly.
/// Requires select entity and value to be set.
#[derive(EntityEvent, Debug)]
pub struct SetTextInputValue {
    pub entity: Entity,
    pub value: String
}

/// This event fires when image is in loading state.
/// Used by `image` widget.
#[derive(EntityEvent, Debug)]
pub struct Loading {
    pub entity: Entity
}

/// This event fires when image is loaded.
/// Used by `image` widget.
#[derive(EntityEvent, Debug)]
pub struct Loaded {
    pub entity: Entity
}

/// This event fires when a page is loaded.
/// Used by `root` widget.
#[derive(EntityEvent, Debug)]
pub struct PageLoaded {
    pub entity: Entity,
    pub name: String,
    pub param: Param
}

/// This event fires when current route is changed
#[derive(Event, Debug)]
pub struct RouteChanged {
    pub route: String,
    pub param: Param
}
