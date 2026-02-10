//! A scrollable container with flex direction set to column.

use bevy::input::mouse::{MouseWheel, MouseScrollUnit};
use bevy::ui_widgets::observe;

use crate::{events::*, consts::*, utils::*};
use super::*;

/// Internal resource used to track scroll.
/// Only entity inside this resource can be scrolled
#[derive(Resource, Default)]
pub struct CanBeScrolled {
    pub entity: Option<Entity>
}

/// Marker component for `scroll`.
#[derive(Component)]
pub struct MakaraScroll;

/// Marker component for `scroll` moving panel.
/// The panel that will actually move up and down when scroll
#[derive(Component)]
pub struct MakaraScrollMovePanel;

/// Marker component for `scroll`'s bar.
#[derive(Component)]
pub struct MakaraScrollbar;

/// Component used to store entity of move panel.
/// This component will be inserted into `scroll` in `detect_scroll_children_added` system.
#[derive(Component)]
pub struct ScrollMovePanelEntity(pub Entity);

/// Component used to store entity of `scroll`.
/// This component will be inserted into move panel and bar in `detect_scroll_children_added` system.
#[derive(Component)]
pub struct ScrollEntity(pub Entity);

/// Component used to store entity of scroll bar.
/// This component will be inserted into `scroll` and move panel in `detect_scroll_children_added` system.
#[derive(Component)]
pub struct ScrollBarEntity(pub Entity);

/// Component used to keep track of scrolling.
/// Used with `scroll`'s move panel.
#[derive(Component)]
pub struct MakaraScrollList {
    pub position: f32,
    pub scroll_height: f32
}

impl Default for MakaraScrollList {
    fn default() -> Self {
        Self {
            position: 0.0,
            scroll_height: DEFAULT_SCROLL_HEIGHT
        }
    }
}

#[derive(Component)]
pub struct ScrollChildrenNeedTransfer;

#[derive(Component)]
pub struct TempMovePanelStyle(pub ContainerStyle);

#[derive(Component)]
pub struct TempScrollBarStyle(pub ContainerStyle);

/// A struct used to mutate components attached to `scroll` widget.
pub struct ScrollWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub panel_entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub bar_style: WidgetStyle<'a>,
    pub child_entities: Vec<Entity>, // child entities of move panel, not scroll.
    pub commands: &'a mut Commands<'w, 's>
}

impl<'a, 'w, 's> WidgetChildren for ScrollWidget<'a, 'w, 's> {
    fn add_child(&mut self, child_bundle: impl Bundle) {
        let child_entity = self.commands.spawn(child_bundle).id();
        self.commands.entity(self.panel_entity).add_child(child_entity);
    }

    fn add_children(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) {
        let mut child_entities = Vec::new();

        for bundle in bundles {
            let child_entity = self.commands.spawn(bundle).id();
            child_entities.push(child_entity);
        }
        self.commands.entity(self.panel_entity).add_children(&child_entities);
    }

    fn insert_at(
        &mut self,
        index: usize,
        bundles: impl IntoIterator<Item = impl Bundle>
    ) {
        let mut child_entities = Vec::new();

        for bundle in bundles {
            let child_entity = self.commands.spawn(bundle).id();
            child_entities.push(child_entity);
        }
        self.commands
            .entity(self.panel_entity)
            .insert_children(index, &child_entities);
    }

    fn insert_first(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) {
        self.insert_at(0, bundles);
    }

    fn insert_last(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) {
        let last_index = self.child_entities.len();
        self.insert_at(last_index, bundles);
    }

    fn remove_at(&mut self, index: usize) {
        if let Some(entity) = self.child_entities.get(index) {
            self.commands.entity(self.panel_entity).detach_child(*entity);
            self.commands.entity(*entity).despawn();
        }
    }

    fn remove_first(&mut self) {
        self.remove_at(0);
    }

    fn remove_last(&mut self) {
        // if list is empty, does nothing.
        if let Some(last_index) = self.child_entities.len().checked_sub(1) {
            self.remove_at(last_index);
        }
    }
}

type IsScrollOnly = (
    (
        With<MakaraScroll>,
        Without<MakaraCheckbox>,
        Without<MakaraCheckboxButton>,
        Without<MakaraCircular>,
        Without<MakaraColumn>,
        Without<MakaraRoot>,
        Without<MakaraButton>,
        Without<MakaraDropdown>,
        Without<MakaraDropdownOverlay>,
        Without<MakaraImage>,
        Without<MakaraLink>,
        Without<MakaraModal>,
        Without<MakaraModalBackdrop>,
    ),
    (
        Without<MakaraProgressBar>,
        Without<MakaraRadio>,
        Without<MakaraRadioGroup>,
        Without<MakaraRow>,
        Without<MakaraScrollbar>,
        Without<MakaraTextInput>,
        Without<MakaraTextInputCursor>,
        Without<MakaraSlider>,
        Without<MakaraSliderThumb>,
        Without<MakaraSelect>,
        Without<MakaraSelectOverlay>,
    )
);

type IsScrollBarOnly = (
    (
        With<MakaraScrollbar>,
        Without<MakaraCheckbox>,
        Without<MakaraCheckboxButton>,
        Without<MakaraCircular>,
        Without<MakaraColumn>,
        Without<MakaraRoot>,
        Without<MakaraButton>,
        Without<MakaraDropdown>,
        Without<MakaraDropdownOverlay>,
        Without<MakaraImage>,
        Without<MakaraLink>,
        Without<MakaraModal>,
        Without<MakaraModalBackdrop>,
    ),
    (
        Without<MakaraProgressBar>,
        Without<MakaraRadio>,
        Without<MakaraRadioGroup>,
        Without<MakaraRow>,
        Without<MakaraScroll>,
        Without<MakaraTextInput>,
        Without<MakaraTextInputCursor>,
        Without<MakaraSlider>,
        Without<MakaraSliderThumb>,
        Without<MakaraSelect>,
        Without<MakaraSelectOverlay>,
    )
);

/// `scroll` system param.
#[derive(SystemParam)]
pub struct ScrollQuery<'w, 's> {
    pub id_class: Query<
        'w, 's,
        (Entity, &'static Id, &'static mut Class),
        IsScrollOnly
    >,
    pub scroll_related: Query<'w, 's,
        (
            Entity,
            &'static ScrollBarEntity,
            &'static ScrollMovePanelEntity
        ),
        IsScrollOnly
    >,
    pub style: StyleQuery<'w, 's, IsScrollOnly>,
    pub bar_style: StyleQuery<'w, 's, IsScrollBarOnly>,
    pub children: Query<'w, 's, Option<&'static Children>, With<MakaraScrollMovePanel>>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> ScrollQuery<'w, 's> {
    pub fn id_match(&mut self, target_id: &str) -> bool {
        self.id_class.iter().any(|(_, id, _)| id.0 == target_id)
    }
}

impl<'w, 's> WidgetQuery<'w, 's> for ScrollQuery<'w, 's> {
    type WidgetView<'a> = ScrollWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let ScrollQuery {
            id_class, scroll_related, style, bar_style, children, commands
        } = self;

        let (_, _, class) = id_class.get_mut(entity).ok()?;
        let (_, bar_entity, panel_entity) = scroll_related.get_mut(entity).ok()?;

        let bar_bundle = bar_style.query.get_mut(bar_entity.0).ok()?;
        let (b_node, b_bg, b_border_color, b_shadow, b_z) = bar_bundle;

        let style_bundle = style.query.get_mut(entity).ok()?;
        let (node, bg, border_color, shadow, z_index) = style_bundle;

        let mut entities: Vec<Entity> = Vec::new();
        {
            let children = children.get(panel_entity.0).ok()?;
            if let Some(children) = children {
                entities = children.iter().map(|e| e).collect();
            }
        }

        return Some(ScrollWidget {
            entity,
            panel_entity: panel_entity.0,
            class: class.into_inner(),
            style: WidgetStyle {
                node: node.into_inner(),
                background_color: bg.into_inner(),
                border_color: border_color.into_inner(),
                shadow: shadow.into_inner(),
                z_index: z_index.into_inner(),
            },
            bar_style: WidgetStyle {
                node: b_node.into_inner(),
                background_color: b_bg.into_inner(),
                border_color: b_border_color.into_inner(),
                shadow: b_shadow.into_inner(),
                z_index: b_z.into_inner(),
            },
            child_entities: entities,
            commands: commands
        });
    }

    fn find_by_id<'a>(&'a mut self, target_id: &str) -> Option<Self::WidgetView<'a>> {
        let entity = self.id_class.iter()
            .find(|(_, id, _)| id.0 == target_id)
            .map(|(e, _, _)| e)?;

        self.get_components(entity)
    }

    fn find_by_entity<'a>(&'a mut self, target_entity: Entity) -> Option<Self::WidgetView<'a>> {
        self.get_components(target_entity)
    }

    fn find_by_class(&self, target_class: &str) -> Vec<Entity> {
        self.id_class.iter()
            .filter(|(_, _, class)| class.0.split(" ").any(|word| word == target_class))
            .map(|(e, _, _)| e)
            .collect()
    }
}

/// Bundle for creating `scroll`.
pub struct ScrollBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub move_panel_style: ContainerStyle,
    pub scroll_bar: ContainerStyle,
}

impl Default for ScrollBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                height: Val::Percent(50.0),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            shadow: BoxShadow::default(),
            ..default()
        };

        let move_panel_style = ContainerStyle {
            node: Node {
                width: percent(100.0),
                position_type: PositionType::Absolute,
                left: px(0.0),
                top: px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                height: auto(),
                padding: UiRect {
                    left: px(0.0),
                    right: px(0.0),
                    top: px(2.0),
                    bottom: px(2.0)
                },
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            shadow: BoxShadow::default(),
            ..default()
        };

        let scroll_bar = ContainerStyle {
            node: Node {
                width: px(8),
                height: px(10),
                position_type: PositionType::Absolute,
                right: px(0),
                top: px(0),
                border_radius: BorderRadius::all(px(8)),
                // display: Display::None,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
            shadow: BoxShadow::default(),
            ..default()
        };

        Self { style, move_panel_style, scroll_bar, id_class: IdAndClass::default() }
    }
}

impl Widget for ScrollBundle {
    /// Build `scroll`.
    fn build(mut self) -> impl Bundle {
        process_built_in_spacing_class(&self.id_class.class, &mut self.style.node);
        process_built_in_alignment_class(&self.id_class.class, &mut self.style.node);
        (
            self.id_class,
            self.style,
            TempMovePanelStyle(self.move_panel_style),
            TempScrollBarStyle(self.scroll_bar),
            MakaraScroll,
            ScrollChildrenNeedTransfer,
            observe(on_mouse_over)
        )
    }
}

impl ScrollBundle {
    /// Set custom style for scroll bar.
    /// Warning: certain styles can break how scroll bar behave.
    /// The only recommended styles to change are width, height, background_color and border_radius.
    pub fn bar_style(mut self, f: impl FnOnce(&mut ContainerStyle)) -> Self {
        f(&mut self.scroll_bar);
        self
    }
}

impl SetContainerStyle for ScrollBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetIdAndClass for ScrollBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

fn on_mouse_over(
    mut over: On<Pointer<Over>>,
    mut can_be_scrolled: ResMut<CanBeScrolled>
) {
    can_be_scrolled.entity = Some(over.entity);
    over.propagate(false);
}

// Transfer user provided children to move panel instead.
pub(crate) fn detect_scroll_children_added(
    mut commands: Commands,
    scrolls: Query<
        (Entity, &Node, Option<&Children>, &TempMovePanelStyle, &TempScrollBarStyle),
        With<ScrollChildrenNeedTransfer>
    >,
) {
    for (scroll_entity, scroll_node, scroll_children, panel_style, bar_style) in scrolls.iter() {
        let mut new_panel_style = panel_style.0.clone();
        new_panel_style.node.align_items = scroll_node.align_items;
        new_panel_style.node.justify_content = scroll_node.justify_content;

        let panel_entity = commands.spawn((
            new_panel_style,
            MakaraScrollMovePanel,
            MakaraScrollList::default(),
            ScrollEntity(scroll_entity),
        ))
        .id();

        let bar_entity = commands.spawn((
            bar_style.0.clone(),
            MakaraScrollbar,
            ScrollEntity(scroll_entity),
            Visibility::Hidden,
            observe(on_scrollbar_drag),
            observe(on_scrollbar_mouse_over),
            observe(on_mouse_out)
        ))
        .id();

        if let Some(children) = scroll_children {
            let child_entities = children.iter().map(|e| e).collect::<Vec<Entity>>();
            commands.entity(panel_entity).add_children(&child_entities);
        }

        commands.entity(panel_entity).insert(ScrollBarEntity(bar_entity));
        commands
            .entity(scroll_entity)
            .remove::<ScrollChildrenNeedTransfer>()
            .insert((ScrollMovePanelEntity(panel_entity), ScrollBarEntity(bar_entity)))
            .add_children(&[panel_entity, bar_entity]);
    }
}

pub(crate) fn detect_scroll_height_change(
    scrolls: Query<
        (&ComputedNode, &ScrollMovePanelEntity, &ScrollBarEntity),
        Changed<ComputedNode>
    >,
    panels: Query<&ComputedNode, With<MakaraScrollMovePanel>>,
    mut bars: Query<(&mut Visibility, &mut Node), IsScrollBarOnly>
) {
    for (scroll_computed, panel_entity, bar_entity) in scrolls.iter() {
        if let Ok(panel_computed) = panels.get(panel_entity.0) {
            if let Ok((mut vis, mut bar_node)) = bars.get_mut(bar_entity.0) {
                let scroll_height = scroll_computed.size().y * scroll_computed.inverse_scale_factor();
                let panel_height = panel_computed.size().y * panel_computed.inverse_scale_factor();

                if panel_height <= scroll_height {
                    *vis = Visibility::Hidden;
                }
                else {
                    let bar_height = (scroll_height / panel_height) * scroll_height;
                    bar_node.height = px(bar_height);
                    *vis = Visibility::Visible;
                }
            }
        }
    }
}

pub(crate) fn detect_move_panel_height_change(
    panels: Query<
        (&ComputedNode, &ScrollEntity),
        (With<MakaraScrollMovePanel>, Changed<ComputedNode>)
    >,
    scrolls: Query<(&ComputedNode, &ScrollBarEntity)>,
    mut bars: Query<(&mut Visibility, &mut Node), IsScrollBarOnly>
) {
    for (panel_computed, scroll_entity) in panels.iter() {
        if let Ok((scroll_computed, bar_entity)) = scrolls.get(scroll_entity.0) {
            if let Ok((mut vis, mut bar_node)) = bars.get_mut(bar_entity.0) {
                let scroll_height = scroll_computed.size().y * scroll_computed.inverse_scale_factor();
                let panel_height = panel_computed.size().y * panel_computed.inverse_scale_factor();

                if panel_height <= scroll_height {
                    *vis = Visibility::Hidden;
                }
                else {
                    let bar_height = (scroll_height / panel_height) * scroll_height;
                    bar_node.height = px(bar_height);
                    *vis = Visibility::Visible;
                }
            }
        }
    }
}

fn on_scrollbar_drag(
    drag: On<Pointer<Drag>>,
    scrolls: Query<(&ComputedNode, &ScrollMovePanelEntity)>,
    mut bars: Query<(&mut Node, &ComputedNode, &ChildOf), Without<MakaraScrollList>>,
    mut panels: Query<(&mut Node, &ComputedNode, &mut MakaraScrollList)>,
    mut commands: Commands,
) {
    let Ok((mut bar_node, bar_computed, bar_parent)) = bars.get_mut(drag.entity) else {
        return
    };

    let Ok((scroll_computed, panel_entity)) = scrolls.get(bar_parent.0) else {
        return
    };

    let Ok((mut panel_node, panel_computed, mut scroll_list)) = panels.get_mut(panel_entity.0) else {
        return
    };

    let scale = bar_computed.inverse_scale_factor();
    let container_height = scroll_computed.size().y * scale;
    let panel_height = panel_computed.size().y * scale;
    let bar_height = bar_computed.size().y * scale;

    let max_scroll = (panel_height - container_height).max(0.0);
    let track_space = (container_height - bar_height).max(1.0); // Avoid div by zero

    let current_bar_top = if let Val::Px(p) = bar_node.top {
        p
    }
    else {
        0.0
    };

    let new_bar_top = (current_bar_top + drag.delta.y).clamp(0.0, track_space);
    bar_node.top = Val::Px(new_bar_top);

    // Calculate the ratio 0.0 to 1.0
    let scroll_ratio = new_bar_top / track_space;

    scroll_list.position = -(scroll_ratio * max_scroll);
    panel_node.top = Val::Px(scroll_list.position);

    commands.trigger(Scrolling {
        entity: bar_parent.0,
        position: scroll_list.position,
    });
}

fn on_scrollbar_mouse_over(
    mut over: On<Pointer<Over>>,
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    commands.entity(*window).insert(CursorIcon::System(SystemCursorIcon::Pointer));
    over.propagate(false);
}

pub(crate) fn handle_scrolling(
    mut mouse_wheel: MessageReader<MouseWheel>,
    scrolls: Query<(&ComputedNode, &ScrollMovePanelEntity, &ScrollBarEntity)>,
    mut panels: Query<
        (&mut Node, &ComputedNode, &mut MakaraScrollList),
        With<MakaraScrollMovePanel>
    >,
    mut bars: Query<
        (&mut Node, &ComputedNode),
        (With<MakaraScrollbar>, Without<MakaraScrollMovePanel>)
    >,
    mut commands: Commands,
    can_be_scrolled: Res<CanBeScrolled>,
) {
    for e in mouse_wheel.read() {
        let Some(hovered) = can_be_scrolled.entity else {
            continue;
        };

        let Ok((scroll_computed, panel_entity, bar_entity)) = scrolls.get(hovered) else {
            continue;
        };

        let Ok((mut bar_node, bar_computed)) = bars.get_mut(bar_entity.0) else {
            continue;
        };

        // panel
        if let Ok((mut panel_node, panel_computed, mut scroll_list)) = panels.get_mut(panel_entity.0) {
            let dy = match e.unit {
                MouseScrollUnit::Line => e.y * scroll_list.scroll_height,
                MouseScrollUnit::Pixel => e.y,
            };

            // calculate max scroll
            let panel_height = panel_computed.size().y * panel_computed.inverse_scale_factor();
            let container_height = scroll_computed.size().y * scroll_computed.inverse_scale_factor();
            let max_scroll = (panel_height - container_height).max(0.0);

            scroll_list.position = (scroll_list.position + dy).clamp(-max_scroll, 0.0);
            panel_node.top = px(scroll_list.position);

            commands.trigger(Scrolling {
                entity: hovered,
                position: scroll_list.position
            });

            if max_scroll > 0.0 {
                // Calculate how far down we are (0.0 to 1.0)
                // We use absolute value because scroll_list.position is negative
                let scroll_ratio = scroll_list.position.abs() / max_scroll;

                // The track space is the container height minus the bar's own height
                let bar_height = bar_computed.size().y * bar_computed.inverse_scale_factor();
                let track_space = container_height - bar_height;

                // Set the bar's top position
                bar_node.top = px(scroll_ratio * track_space);
            } else {
                bar_node.top = px(0.0);
            }
        }
    }
}

pub(crate) fn detect_scroll_built(
    mut commands: Commands,
    q: Query<Entity, Added<MakaraScroll>>
) {
    for entity in q.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}


pub(crate) fn detect_scroll_class_change_for_built_in(
    mut scrolls: Query<(&Class, &mut Node), IsScrollOnly>
) {
    for (class, mut node) in scrolls.iter_mut() {
        process_built_in_spacing_class(class, &mut node);
        process_built_in_alignment_class(class, &mut node);
    }
}

/// Create scroll widget.
pub fn scroll() -> ScrollBundle {
    ScrollBundle::default()
}

pub(crate) fn can_run_scroll_systems(q: Query<&MakaraScroll>) -> bool {
    q.count() > 0
}
