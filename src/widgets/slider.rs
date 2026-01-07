use bevy::{prelude::*, ui::InteractionDisabled, ui_widgets::observe};
use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::{consts::*, events::*, utils::*, on_mouse_out};
use super::*;

/// Marker component for `slider`.
#[derive(Component)]
pub struct MakaraSlider;

/// Component to store data related to `slider`.
#[derive(Component, Default)]
pub struct MakaraSliderData {
    pub value: f32,
    pub range_start: f32,
    pub range_end: f32,
    pub step: f32
}

/// Marker component for slider thumb.
#[derive(Component)]
pub struct MakaraSliderThumb;

#[derive(Component)]
pub struct ThumbPositionAdjusted(pub bool);

/// A struct used to mutate components attached to `slider` widget.
pub struct SliderWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub thumb_style: WidgetStyle<'a>,
    pub commands: &'a mut Commands<'w, 's>
}

impl<'a, 'w, 's> SliderWidget<'a, 'w, 's> {
    pub fn set_value(&mut self, value: f32) {
        self.commands.trigger(SetSliderValue {
            entity: self.entity,
            value
        });
    }
}

/// `slider` system param.
#[derive(SystemParam)]
pub struct SliderQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraSlider>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), With<MakaraSlider>>,
    pub style: StyleQuery<'w, 's, With<MakaraSlider>>,
    pub thumb_style: StyleQuery<'w, 's,
        (With<MakaraSliderThumb>, Without<MakaraSlider>)
    >,
    pub children: Query<'w, 's, &'static Children>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for SliderQuery<'w, 's> {
     type WidgetView<'a> = SliderWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let SliderQuery { id: _, class, style, thumb_style, children, commands } = self;

        let mut thumb_entity = None;

        let children_list = children.get(entity).ok()?;

        for child in children_list {
            if thumb_entity.is_none() && thumb_style.query.get(*child).is_ok() {
                thumb_entity = Some(*child);
                break;
            }
        }

        // 2. BORROW PHASE: Now that the loop is over, we borrow specifically what we need.
        if let Some(thumb_entity) = thumb_entity {
            // Fetch the parent styles
            let style_bundle = style.query.get_mut(entity).ok()?;
            let (node, bg, border, radius, shadow, z) = style_bundle;

            // Fetch the children components mutably (only happens once!)
            let thumb_components = thumb_style.query.get_mut(thumb_entity).ok()?;

            return Some(SliderWidget {
                entity,
                class: class.get_mut(entity).ok()?.1.into_inner(),
                style: WidgetStyle {
                    node: node.into_inner(),
                    background_color: bg.into_inner(),
                    border_color: border.into_inner(),
                    border_radius: radius.into_inner(),
                    shadow: shadow.into_inner(),
                    z_index: z.into_inner(),
                },
                thumb_style: WidgetStyle {
                    node: thumb_components.0.into_inner(),
                    background_color: thumb_components.1.into_inner(),
                    border_color: thumb_components.2.into_inner(),
                    border_radius: thumb_components.3.into_inner(),
                    shadow: thumb_components.4.into_inner(),
                    z_index: thumb_components.5.into_inner(),
                },
                commands: commands
            });
        }

        None
    }

    fn find_by_id<'a>(&'a mut self, target_id: &str) -> Option<Self::WidgetView<'a>> {
        let entity = self.id.iter()
            .find(|(_, id)| id.0 == target_id)
            .map(|(e, _)| e)?;

        self.get_components(entity)
    }

    fn find_by_entity<'a>(&'a mut self, target_entity: Entity) -> Option<Self::WidgetView<'a>> {
        self.get_components(target_entity)
    }

    fn find_by_class(&self, target_class: &str) -> Vec<Entity> {
        self.class.iter()
            .filter(|(_, class)| class.0 == target_class)
            .map(|(e, _)| e)
            .collect()
    }
}

/// Bundle for creating slider.
#[derive(Bundle)]
pub struct SliderBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub thumb_style: ContainerStyle,
    pub data: MakaraSliderData,
    pub tooltip_bundle: TooltipBundle
}

impl Default for SliderBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: px(100.0),
                height: px(8),
                position_type: PositionType::Relative,
                flex_shrink: 0.0,
                flex_grow: 0.0,
                ..default()
            },
            background_color: BackgroundColor(LIGHT_SLIDER_BG_COLOR),
            border_radius: BorderRadius::MAX,
            shadow: BoxShadow::default(),
            ..default()
        };

        let thumb_style = ContainerStyle {
            node: Node {
                width: px(15),
                height: px(15),
                position_type: PositionType::Absolute,
                left: percent(0),
                top: px(0),
                ..default()
            },
            background_color: BackgroundColor(SLIDER_THUMB_COLOR),
            border_radius: BorderRadius::MAX,
            shadow: BoxShadow::default(),
            ..default()
        };

        let data = MakaraSliderData::default();
        let tooltip_bundle = TooltipBundle::default();
        let id_class = IdAndClass::default();

        Self { style, thumb_style, data, tooltip_bundle, id_class }
    }
}

impl SliderBundle {
    /// Set default value for slider.
    pub fn value(mut self, value: f32) -> Self {
        self.data.value = value;
        self
    }

    /// Set step for slider thumb move. Default is `1.0`.
    pub fn step(mut self, step: f32) -> Self {
        self.data.step = step;
        self
    }

    /// Build slider as disabled.
    pub fn build_as_disabled(self) -> impl Bundle {
        (self.build(), InteractionDisabled)
    }
}

impl Widget for SliderBundle {
    /// Build slider.
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            self.data,
            MakaraSlider,
            MakaraWidget,
            WidgetFocus(false),
            children![
                (
                    self.thumb_style,
                    MakaraSliderThumb,
                    ThumbPositionAdjusted(false),
                    observe(on_slider_thumb_mouse_over),
                    observe(on_mouse_out),
                    observe(on_slider_thumb_mouse_drag),
                    observe(on_thumb_mouse_click)
                ),
                self.tooltip_bundle.build()
            ],
            observe(on_slider_mouse_over),
            observe(on_mouse_out),
            observe(on_slider_value_set)
        )
    }
}

impl SetContainerStyle for SliderBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for SliderBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for SliderBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default slider.
pub fn slider(range_start: f32, range_end: f32) -> SliderBundle {
    let mut bundle = SliderBundle::default();
    bundle.data.range_start = range_start;
    bundle.data.range_end = range_end;
    bundle.data.step = 1.0;
    bundle
}

fn on_thumb_mouse_click(
    mut click: On<Pointer<Click>>,
    mut widgets: Query<(Entity, &mut WidgetFocus)>
) {
    update_focus_state_for_widgets_on_click(click.entity, &mut widgets);
    click.propagate(false);
}

fn on_slider_mouse_over(
    mut over: On<Pointer<Over>>,
    mut sliders: Query<
        (Has<InteractionDisabled>, &Children, &UiTransform, &ComputedNode),
        With<MakaraSlider>
    >,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    window: Single<Entity, With<Window>>,
) {
    if let Ok((is_disabled, children, transform, computed)) = sliders.get_mut(over.entity) {
        let cursor_icon = if is_disabled {
            CursorIcon::System(SystemCursorIcon::Default)
        } else {
            CursorIcon::System(SystemCursorIcon::Pointer)
        };

        commands.entity(*window).insert(cursor_icon);
        show_or_hide_tooltip(true, &mut tooltips, Some(computed), Some(transform), children);
    }

    commands.trigger(MouseOver { entity: over.entity });
    over.propagate(false);
}

fn on_slider_thumb_mouse_over(
    mut over: On<Pointer<Over>>,
    mut thumbs: Query<Has<InteractionDisabled>, With<MakaraSliderThumb>>,
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    if let Ok(is_disabled) = thumbs.get_mut(over.entity) {
        let cursor_icon = if is_disabled {
            CursorIcon::System(SystemCursorIcon::Default)
        } else {
            CursorIcon::System(SystemCursorIcon::Pointer)
        };

        commands.entity(*window).insert(cursor_icon);
    }

    commands.trigger(MouseOver { entity: over.entity });
    over.propagate(false);
}

fn on_slider_thumb_mouse_drag(
    drag: On<Pointer<Drag>>,
    mut sliders: Query<(&mut MakaraSliderData, &ComputedNode, &UiGlobalTransform)>,
    mut thumbs: Query<(&mut Node, &ComputedNode, &ChildOf), With<MakaraSliderThumb>>,
    mut commands: Commands,
) {
    let thumb_entity = drag.entity;

    if let Ok((mut thumb_node, thumb_computed, parent)) = thumbs.get_mut(thumb_entity) {
        if let Ok((mut data, slider_computed, slider_transform)) = sliders.get_mut(parent.0) {

            // 1. Find where the mouse is in "World Space"
            let mouse_x = drag.pointer_location.position.x;

            // 2. Find where the Slider Track starts in "World Space"
            // Bevy's GlobalTransform.translation is the CENTER of the node
            let slider_center_x = slider_transform.translation.x;
            let slider_width = slider_computed.size.x;
            let slider_left_edge = slider_center_x - (slider_width / 2.0);

            // 3. Calculate thumb position relative to track start
            let thumb_width = thumb_computed.size.x;
            let max_left = slider_width - thumb_width;

            // We subtract half the thumb width so the mouse stays in the center of the thumb
            let desired_left = mouse_x - slider_left_edge - (thumb_width / 2.0);
            let clamped_left = desired_left.clamp(0.0, max_left);

            // 4. Update Data (Mapping)
            let percent = clamped_left / max_left;
            let raw_value = data.range_start + percent * (data.range_end - data.range_start);
            let stepped_value = (raw_value / data.step).round() * data.step;
            let final_value = stepped_value.clamp(data.range_start, data.range_end);

            // 5. Apply visuals (Snapping)
            let final_percent = (final_value - data.range_start) / (data.range_end - data.range_start);
            thumb_node.left = Val::Px(final_percent * max_left);

            if data.value != final_value {
                data.value = final_value;
                commands.trigger(Change { entity: parent.0, data: data.value });
            }
        }
    }
}

fn on_slider_value_set(
    value_set: On<SetSliderValue>,
    mut sliders: Query<(&mut MakaraSliderData, &Children, &ComputedNode)>,
    mut thumbs: Query<(&mut Node, &ComputedNode), With<MakaraSliderThumb>>
) {
    if let Ok((mut data, children, slider_computed)) = sliders.get_mut(value_set.entity) {
        if value_set.value < data.range_start || value_set.value > data.range_end {
            return;
        }

        data.value = value_set.value;

        let parent_width = slider_computed.size.x;

        for child in children {
            if let Ok((mut node, thumb_computed)) = thumbs.get_mut(*child) {
                let thumb_width = thumb_computed.size.x;
                let max_left = parent_width - thumb_width;

                // Convert value to percent
                let t = (value_set.value - data.range_start)
                    / (data.range_end - data.range_start);

                // Convert percent → pixel position
                let new_left = t * max_left;

                // Apply pixel position
                node.left = px(new_left);
            }
        }
    }
}

pub(crate) fn update_slider_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut sliders: Query<&mut BackgroundColor, With<MakaraSlider>>,
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_bg_color = match makara_theme.theme {
        Theme::Light => LIGHT_SLIDER_BG_COLOR,
        Theme::Dark => DARK_SLIDER_BG_COLOR
    };

    for mut bg_color in sliders.iter_mut() {
        // only react to theme change if color is default
        if bg_color.0 == LIGHT_SLIDER_BG_COLOR || bg_color.0 == DARK_SLIDER_BG_COLOR {
            bg_color.0 = new_bg_color;
        }
    }
}

// to adjust thumb position
pub(crate) fn detect_slider_thumb_added(
    sliders: Query<(&MakaraSliderData, &ComputedNode), With<MakaraSlider>>,
    mut thumbs: Query<
        (&mut Node, &mut ThumbPositionAdjusted,  &ComputedNode, &ChildOf)
    >
) {
    for (mut node, mut pos_adjusted, thumb_computed, parent) in thumbs.iter_mut() {
        if pos_adjusted.0 || thumb_computed.size.y == 0.0 {
            continue;
        }

        if let Ok((slider_data, slider_computed)) = sliders.get(parent.0) {
            // adjust top position
            let height_diff = thumb_computed.size.y - slider_computed.size.y;
            node.top = px(-(height_diff / 2.0));

            // adjust left position
            let slider_width = slider_computed.size.x;
            let thumb_width = thumb_computed.size.x;
            let max_left = slider_width - thumb_width;

            // Convert value to percent
            let t = (slider_data.value - slider_data.range_start)
                / (slider_data.range_end - slider_data.range_start);

            let new_left = t * max_left;
            node.left = px(new_left);

            pos_adjusted.0 = true;
        }
    }
}

pub(crate) fn detect_slider_built(
    mut commands: Commands,
    sliders: Query<Entity, Added<MakaraSlider>>
) {
    for entity in sliders.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn can_run_slider_systems(q: Query<&MakaraSlider>) -> bool {
    q.count() > 0
}
