//! Collection of built-in widgets provided by Makara.

pub mod button;
pub mod text;
pub mod row;
pub mod column;
pub mod modal;
pub mod checkbox;
pub mod root;
pub mod radio;
pub mod dropdown;
pub mod select;
pub mod link;
pub mod slider;
pub mod progress_bar;
pub mod circular;
pub mod tooltip;
pub mod text_input;
pub mod text_edit;
pub mod scroll;
pub mod image;

pub use button::*;
pub use text::*;
pub use row::*;
pub use column::*;
pub use modal::*;
pub use checkbox::*;
pub use root::*;
pub use radio::*;
pub use dropdown::*;
pub use select::*;
pub use link::*;
pub use slider::*;
pub use progress_bar::*;
pub use circular::*;
pub use tooltip::*;
pub use text_input::*;
pub use text_edit::*;
pub use scroll::*;
pub use image::*;

use bevy::prelude::*;
use bevy::window::{CursorIcon, SystemCursorIcon};
use bevy::ecs::query::QueryFilter;
use bevy::ecs::system::SystemParam;
use cosmic_text::{FontSystem, SwashCache, Attrs};
use crate::{events::*, colors::IntoColor};

pub trait Widget {
    fn build(self) -> impl Bundle;
}

/// Trait for setting tooltip and its style.
pub trait SetToolTip: Sized {
    fn set_tooltip(&mut self) -> &mut TooltipBundle;

    /// Enable and set tooltip set.
    fn tooltip(mut self, text: &str) -> Self {
        self.set_tooltip().use_tooltip = UseTooltip(true);
        self.set_tooltip().text = TooltipText(text.to_string());
        self.set_tooltip().text_bundle.text.0 = text.to_string();
        self
    }

    /// Set tooltip background style.
    fn tooltip_style(mut self, style: ContainerStyle) -> Self {
        self.set_tooltip().style = style;
        self
    }

    /// Set tooltip text style.
    fn tooltip_text_style(mut self, text_style: TextStyle) -> Self {
        self.set_tooltip().text_bundle.text_style = text_style;
        self
    }

    /// Set tooltip position.
    /// Available positions: `left`, `right`, `top`, `bottom`, `center`.
    /// Default is `center`.
    fn tooltip_position(mut self, pos: &str) -> Self {
        match pos {
            "left" => self.set_tooltip().position = TooltipPosition::Left,
            "right" => self.set_tooltip().position = TooltipPosition::Right,
            "top" => self.set_tooltip().position = TooltipPosition::Top,
            "bottom" => self.set_tooltip().position = TooltipPosition::Bottom,
            _ => self.set_tooltip().position = TooltipPosition::Center,
        }
        self
    }
}

pub trait SetIdAndClass: Sized {
    fn id_and_class(&mut self) -> &mut IdAndClass;

    fn id(mut self, id: &str) -> Self {
        self.id_and_class().id.0 = id.to_string();
        self
    }

    fn class(mut self, class: &str) -> Self {
        self.id_and_class().class.value = class.to_string();
        self.id_and_class().class.changed = true;
        self
    }
}

pub trait SetContainerStyle: Sized {
    fn container_style(&mut self) -> &mut ContainerStyle;

    fn node(mut self, node: Node) -> Self {
        self.container_style().node = node;
        self
    }

    fn width(mut self, value: Val) -> Self {
        self.container_style().node.width = value;
        self
    }

    fn height(mut self, value: Val) -> Self {
        self.container_style().node.height = value;
        self
    }

    fn align_items(mut self, ai: AlignItems) -> Self {
        self.container_style().node.align_items = ai;
        self
    }

    fn align_content(mut self, ac: AlignContent) -> Self {
        self.container_style().node.align_content = ac;
        self
    }

    fn justify_content(mut self, jc: JustifyContent) -> Self {
        self.container_style().node.justify_content = jc;
        self
    }

    fn margin(mut self, value: Val) -> Self {
        self.container_style().node.margin = UiRect::all(value);
        self
    }

    fn margin_x(mut self, value: Val) -> Self {
        self.container_style().node.margin.left = value;
        self.container_style().node.margin.right = value;
        self
    }

    fn margin_y(mut self, value: Val) -> Self {
        self.container_style().node.margin.top = value;
        self.container_style().node.margin.bottom = value;
        self
    }

    fn margin_top(mut self, value: Val) -> Self {
        self.container_style().node.margin.top = value;
        self
    }

    fn margin_right(mut self, value: Val) -> Self {
        self.container_style().node.margin.right = value;
        self
    }

    fn margin_bottom(mut self, value: Val) -> Self {
        self.container_style().node.margin.bottom = value;
        self
    }

    fn margin_left(mut self, value: Val) -> Self {
        self.container_style().node.margin.left = value;
        self
    }

    fn padding(mut self, value: Val) -> Self {
        self.container_style().node.padding = UiRect::all(value);
        self
    }

    fn padding_x(mut self, value: Val) -> Self {
        self.container_style().node.padding.left = value;
        self.container_style().node.padding.right = value;
        self
    }

    fn padding_y(mut self, value: Val) -> Self {
        self.container_style().node.padding.top = value;
        self.container_style().node.padding.bottom = value;
        self
    }

    fn padding_top(mut self, value: Val) -> Self {
        self.container_style().node.padding.top = value;
        self
    }

    fn padding_bottom(mut self, value: Val) -> Self {
        self.container_style().node.padding.bottom = value;
        self
    }

    fn padding_left(mut self, value: Val) -> Self {
        self.container_style().node.padding.left = value;
        self
    }

    fn padding_right(mut self, value: Val) -> Self {
        self.container_style().node.padding.right = value;
        self
    }

    fn border(mut self, value: Val) -> Self {
        self.container_style().node.border = UiRect::all(value);
        self
    }

    fn border_top(mut self, value: Val) -> Self {
        self.container_style().node.border.top = value;
        self
    }

    fn border_bottom(mut self, value: Val) -> Self {
        self.container_style().node.border.bottom = value;
        self
    }

    fn border_left(mut self, value: Val) -> Self {
        self.container_style().node.border.left = value;
        self
    }

    fn border_right(mut self, value: Val) -> Self {
        self.container_style().node.border.right = value;
        self
    }

    fn background_color(mut self, color: impl IntoColor) -> Self {
        self.container_style().background_color.0 = color.into_color();
        self
    }

    fn border_radius(mut self, radius: BorderRadius) -> Self{
        self.container_style().node.border_radius = radius;
        self
    }

    fn border_color(mut self, color: impl IntoColor) -> Self {
        self.container_style().border_color = BorderColor::all(color.into_color());
        self
    }

    fn border_top_color(mut self, color: impl IntoColor) -> Self {
        self.container_style().border_color.top = color.into_color();
        self
    }

    fn border_bottom_color(mut self, color: impl IntoColor) -> Self {
        self.container_style().border_color.bottom = color.into_color();
        self
    }

    fn border_left_color(mut self, color: impl IntoColor) -> Self {
        self.container_style().border_color.left = color.into_color();
        self
    }

    fn border_right_color(mut self, color: impl IntoColor) -> Self {
        self.container_style().border_color.right = color.into_color();
        self
    }

    fn shadow(mut self, shadow: ShadowStyle) -> Self {
        self.container_style().shadow = BoxShadow(vec![shadow]);
        self
    }

    fn no_shadow(mut self) -> Self {
        self.container_style().shadow = BoxShadow::default();
        self
    }

    fn style(mut self, style: ContainerStyle) -> Self {
        *self.container_style() = style;
        self
    }
}

/// Component used to store id for a widget.
#[derive(Component, Debug, Default, PartialEq, Eq, Clone)]
pub struct Id(pub String);

/// Component used to store class for a widget.
#[derive(Component, Debug, Default, PartialEq, Eq, Clone)]
pub struct Class {
    pub value: String,
    pub changed: bool
}

impl Class {
    /// Replace current class with new class.
    ///
    /// You need to use this method to set new class for new class to be effective.
    pub fn set_class(&mut self, class: &str) {
        self.value = class.to_string();
        self.changed = true;
    }

    /// Adds a class (or multiple space-separated classes) to the widget.
    pub fn add_class(&mut self, class: &str) {
        let mut classes: Vec<_> = self.value.split_whitespace().collect();

        for new_class in class.split_whitespace() {
            if !classes.contains(&new_class) {
                classes.push(new_class);
            }
        }

        self.value = classes.join(" ");
        self.changed = true;
    }

    /// Removes a specific class (or multiple) from the widget.
    pub fn remove_class(&mut self, class: &str) {
        let to_remove: Vec<_> = class.split_whitespace().collect();

        let classes: Vec<_> = self.value
            .split_whitespace()
            .filter(|c| !to_remove.contains(c))
            .collect();

        self.value = classes.join(" ");
        self.changed = true
    }

    /// Returns an iterator over the individual class names.
    ///
    /// Useful for checking specific styles or logic conditions.
    pub fn class_list(&self) -> Vec<String> {
        self.value
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    /// Checks if the widget contains a specific class.
    pub fn has_class(&self, class: &str) -> bool {
        self.value.split_whitespace().any(|c| c == class)
    }
}

#[derive(Bundle, Clone, Default, Debug)]
pub struct IdAndClass {
    pub id: Id,
    pub class: Class
}

/// System param for based styling for all widgets.
#[derive(SystemParam)]
pub struct StyleQuery<'w, 's, F: QueryFilter + 'static = ()> {
    pub query: Query<'w, 's, (
        &'static mut Node,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        &'static mut BoxShadow,
        &'static mut ZIndex,
    ), F>,
}

/// A struct used to mutate styling components for text.
/// This struct is only used within other widgets.
pub struct ChildText<'a> {
    pub value: &'a mut Text,
    pub font: &'a mut TextFont,
    pub layout: &'a mut TextLayout,
    pub color: &'a mut TextColor
}

/// A struct used to mutate styling components for widgets.
pub struct WidgetStyle<'a> {
    pub node: &'a mut Node,
    pub background_color: &'a mut BackgroundColor,
    pub border_color: &'a mut BorderColor,
    pub z_index: &'a mut ZIndex,
    pub shadow: &'a mut BoxShadow
}

/// Trait used for widgets system param.
///
/// # Navigation Patterns
///
/// When using widget queries in event handlers that may navigate, use the `navigate!` macro
/// to automatically exit the function after navigation:
///
/// ```rust
/// fn on_button_click(
///     _click: On<Clicked>,
///     mut text_q: TextQuery,
///     mut router: ResMut<Router>
/// ) {
///     if let Some(mut text) = text_q.find_by_id("status") {
///         text.set_text("Navigating...");
///     }
///
///     navigate!(router, "next-page", ()); // Function exits here automatically
///     // This code will never execute
/// }
/// ```
///
/// This prevents common issues where loops or code continue executing after navigation:
///
/// ```rust
/// fn mark_items_complete(
///     _click: On<Clicked>,
///     mut btn_q: ButtonQuery,
///     mut router: ResMut<Router>
/// ) {
///     for entity in btn_q.find_by_class("item-btn") {
///         if let Some(mut btn) = btn_q.find_by_entity(entity) {
///             if btn.text.value.0 == "target_item" {
///                 btn.class.add_class("completed");
///                 navigate!(router, "home", ()); // Exits function and loop
///             }
///         }
///     }
/// }
/// ```
pub trait WidgetQuery<'w, 's> {
    /// The specific view struct this query returns (Ex, ButtonWidget, TextWidget).
    type WidgetView<'a> where Self: 'a;

    /// Method to find widget by ID.
    fn find_by_id<'a>(&'a mut self, target_id: &str) -> Option<Self::WidgetView<'a>>;

    /// Method to find widget by Entity.
    fn find_by_entity<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>>;

    /// Method to find entities that match with provided classes.
    fn find_by_class(&self, target_class: &str) -> Vec<Entity>;

    /// Get related components of an entity.
    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>>;
}

/// Trait used for widgets that accept child/children.
pub trait WidgetChildren {
    /// Add a single child to a widget.
    fn add_child(&mut self, child_bundle: impl Bundle);

    /// Add multiple children to a widget.
    fn add_children(&mut self, bundles: impl IntoIterator<Item = impl Bundle>);

    /// Insert children at given index.
    fn insert_at(
        &mut self,
        index: usize,
        bundles: impl IntoIterator<Item = impl Bundle>
    );

    /// Insert children at the beginning.
    fn insert_first(&mut self, bundles: impl IntoIterator<Item = impl Bundle>);

    /// Insert children at the end.
    fn insert_last(&mut self, bundles: impl IntoIterator<Item = impl Bundle>);

    /// Remove a child at given index. Does nothing if index out of bound.
    /// This will remove the child from hierachy and UI world.
    fn remove_at(&mut self, index: usize);

    /// Remove the first child.
    /// This will remove the child from hierachy and UI world.
    fn remove_first(&mut self);

    /// Remove the last child.
    /// This will remove the child from hierachy and UI world.
    fn remove_last(&mut self);
}

#[derive(Component)]
pub(crate) struct MakaraWidget;

/// Component used to store focus state of a widget.
#[derive(Component)]
pub struct WidgetFocus(pub bool);

/// Style for all container box. Example, button, row, column ..
#[derive(Bundle, Clone)]
pub struct ContainerStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub shadow: BoxShadow,
    pub z_index: ZIndex,
}

impl Default for ContainerStyle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            background_color: BackgroundColor::default(),
            border_color: BorderColor::default(),
            z_index: ZIndex::default(),
            shadow: BoxShadow::new(
                Color::BLACK.with_alpha(0.8),
                px(0.0),
                px(1.0),
                px(1.0),
                px(2.0),
            ),
        }
    }
}

/// Type of theme for makara application.
#[derive(Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Light,
    Dark
}

/// Resource used to track current theme.
#[derive(Resource, Default)]
pub struct MakaraTheme {
    pub theme: Theme
}

impl MakaraTheme {
    /// Change current theme to the provided one.
    pub fn change_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}

/// Resource used to store related data for cosmic-text.
#[derive(Resource)]
pub struct MakaraTextEditContext {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub attrs: Attrs<'static>
}

impl Default for MakaraTextEditContext {
    fn default() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            attrs: Attrs::new()
        }
    }
}

// mouse out observer for widget
pub(crate) fn on_mouse_out(
    mut out: On<Pointer<Out>>,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    widgets: Query<&Children, With<MakaraWidget>>,
    window: Single<Entity, With<Window>>,
) {
    if let Ok(children) = widgets.get(out.entity) {
        show_or_hide_tooltip(false, &mut tooltips, None, None, children);
    }

    commands.entity(*window).insert(CursorIcon::System(SystemCursorIcon::Default));
    commands.trigger(MouseOut { entity: out.entity });
    out.propagate(false);
}
