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
use crate::events::*;

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
        self.id_and_class().class.0 = class.to_string();
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

    fn background_color(mut self, color: Color) -> Self {
        self.container_style().background_color.0 = color;
        self
    }

    fn border_radius(mut self, radius: BorderRadius) -> Self{
        self.container_style().border_radius = radius;
        self
    }

    fn border_color(mut self, color: BorderColor) -> Self {
        self.container_style().border_color = color;
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
pub struct Class(pub String);

#[derive(Bundle, Clone, Default)]
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
        &'static mut BorderRadius,
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
    pub border_radius: &'a mut BorderRadius,
    pub z_index: &'a mut ZIndex,
    pub shadow: &'a mut BoxShadow
}

/// Trait used for widgets system param.
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
    pub border_radius: BorderRadius,
    pub shadow: BoxShadow,
    pub z_index: ZIndex,
}

impl Default for ContainerStyle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            background_color: BackgroundColor::default(),
            border_color: BorderColor::default(),
            border_radius: BorderRadius::default(),
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

/// Custom style for widgets, set via id or class.
#[derive(Debug, Default)]
pub struct Style {
    pub width: Option<Val>,
    pub height: Option<Val>,
    pub display: Option<Display>,
    pub overflow: Option<Overflow>,
    pub left: Option<Val>,
    pub right: Option<Val>,
    pub top: Option<Val>,
    pub bottom: Option<Val>,
    pub align_items: Option<AlignItems>,
    pub justify_items: Option<JustifyItems>,
    pub align_self: Option<AlignSelf>,
    pub justify_self: Option<JustifySelf>,
    pub align_content: Option<AlignContent>,
    pub justify_content: Option<JustifyContent>,
    pub margin: Option<UiRect>,
    pub padding: Option<UiRect>,
    pub border: Option<UiRect>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<Val>,
    pub row_gap: Option<Val>,
    pub column_gap: Option<Val>,
    pub background_color: Option<BackgroundColor>,
    pub border_color: Option<BorderColor>,
    pub border_radius: Option<BorderRadius>,
    pub z_index: Option<ZIndex>,
    pub shadow: Option<BoxShadow>,
    pub color: Option<TextColor>,
    pub font_size: Option<f32>,
    pub layout: Option<TextLayout>
}

impl Style {
    pub fn width(mut self, value: Val) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: Val) -> Self {
        self.height = Some(value);
        self
    }

    pub fn display(mut self, value: Display) -> Self {
        self.display = Some(value);
        self
    }

    pub fn overflow(mut self, value: Overflow) -> Self {
        self.overflow = Some(value);
        self
    }

    pub fn left(mut self, value: Val) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Val) -> Self {
        self.right = Some(value);
        self
    }

    pub fn top(mut self, value: Val) -> Self {
        self.top = Some(value);
        self
    }

    pub fn bottom(mut self, value: Val) -> Self {
        self.bottom = Some(value);
        self
    }

    pub fn align_items(mut self, value: AlignItems) -> Self {
        self.align_items = Some(value);
        self
    }

    pub fn justify_items(mut self, value: JustifyItems) -> Self {
        self.justify_items = Some(value);
        self
    }

    pub fn align_self(mut self, value: AlignSelf) -> Self {
        self.align_self = Some(value);
        self
    }

    pub fn justify_self(mut self, value: JustifySelf) -> Self {
        self.justify_self = Some(value);
        self
    }

    pub fn align_content(mut self, value: AlignContent) -> Self {
        self.align_content = Some(value);
        self
    }

    pub fn justify_content(mut self, value: JustifyContent) -> Self {
        self.justify_content = Some(value);
        self
    }

    pub fn margin(mut self, value: UiRect) -> Self {
        self.margin = Some(value);
        self
    }

    pub fn padding(mut self, value: UiRect) -> Self {
        self.padding = Some(value);
        self
    }

    pub fn border(mut self, value: UiRect) -> Self {
        self.border = Some(value);
        self
    }

    pub fn flex_direction(mut self, value: FlexDirection) -> Self {
        self.flex_direction = Some(value);
        self
    }

    pub fn flex_wrap(mut self, value: FlexWrap) -> Self {
        self.flex_wrap = Some(value);
        self
    }

    pub fn flex_grow(mut self, value: f32) -> Self {
        self.flex_grow = Some(value);
        self
    }

    pub fn flex_shrink(mut self, value: f32) -> Self {
        self.flex_shrink = Some(value);
        self
    }

    pub fn flex_basis(mut self, value: Val) -> Self {
        self.flex_basis = Some(value);
        self
    }

    pub fn row_gap(mut self, value: Val) -> Self {
        self.row_gap = Some(value);
        self
    }

    pub fn column_gap(mut self, value: Val) -> Self {
        self.column_gap = Some(value);
        self
    }

    pub fn background_color(mut self, value: Color) -> Self {
        self.background_color = Some(BackgroundColor(value));
        self
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
