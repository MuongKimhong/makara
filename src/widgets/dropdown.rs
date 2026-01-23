use bevy::{prelude::*, ui::InteractionDisabled, ui_widgets::observe};
use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::utils::update_focus_state_for_widgets_on_click;
use crate::{consts::*, events::*, on_mouse_out};
use super::*;

/// Marker component for `dropdown`.
#[derive(Component)]
pub struct MakaraDropdown;

/// Marker component for dropdown overlay.
#[derive(Component)]
pub struct MakaraDropdownOverlay;

#[derive(Component)]
pub struct MakaraDropdownOverlayStyle(pub ContainerStyle);

#[derive(Component)]
pub struct MakaraDropdownTextBundle(pub TextBundle);

#[derive(Component)]
pub struct MakaraDropdownTooltipBundle(pub TooltipBundle);

/// Marker component for dropdown item.
#[derive(Component)]
pub struct MakaraDropdownItem;

/// Component used to store state of dropdown.
#[derive(Component)]
pub struct MakaraDropdownState(pub bool);

#[derive(Resource, Default)]
pub(crate) struct DropdownOverlayAndTextAdded(pub bool);

/// A struct used to mutate components attached to `dropdown` widget.
pub struct DropdownWidget<'a> {
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub overlay_style: WidgetStyle<'a>,
    pub text: ChildText<'a>
}

impl<'a> SetText for DropdownWidget<'a> {
    fn set_text(&mut self, text: &str) {
        self.text.value.0 = text.to_string();
    }
}

/// `dropdown` system param.
#[derive(SystemParam)]
pub struct DropdownQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraDropdown>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), With<MakaraDropdown>>,
    pub style: StyleQuery<'w, 's, With<MakaraDropdown>>,
    pub overlay_style: StyleQuery<
        'w, 's, (With<MakaraDropdownOverlay>, Without<MakaraDropdown>)
    >,
    pub text: TextQueryAsChild<'w, 's>,
    pub children: Query<'w, 's, &'static Children>
}

impl<'w, 's> DropdownQuery<'w, 's> {
    pub fn id_match(&self, target_id: &str) -> bool {
        self.id.iter().any(|(_, id)| id.0 == target_id)
    }
}

impl<'w, 's> WidgetQuery<'w, 's> for DropdownQuery<'w, 's> {
     type WidgetView<'a> = DropdownWidget<'a> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let DropdownQuery { id: _, class, style, overlay_style, text, children } = self;

        let mut text_entity = None;
        let mut overlay_entity = None;

        let children_list = children.get(entity).ok()?;

        for child in children_list {
            if text_entity.is_none() && text.query.get(*child).is_ok() {
                text_entity = Some(*child);
            }

            if overlay_entity.is_none() && overlay_style.query.get(*child).is_ok() {
                overlay_entity = Some(*child);
            }
        }

        if let (Some(t_ent), Some(o_ent)) = (text_entity, overlay_entity) {
            // Fetch the parent styles
            let style_bundle = style.query.get_mut(entity).ok()?;
            let (node, bg, border, shadow, z) = style_bundle;

            // Fetch the children components mutably (only happens once!)
            let t_components = text.query.get_mut(t_ent).ok()?;
            let o_style_components = overlay_style.query.get_mut(o_ent).ok()?;

            return Some(DropdownWidget {
                class: class.get_mut(entity).ok()?.1.into_inner(),
                style: WidgetStyle {
                    node: node.into_inner(),
                    background_color: bg.into_inner(),
                    border_color: border.into_inner(),
                    shadow: shadow.into_inner(),
                    z_index: z.into_inner(),
                },
                overlay_style: WidgetStyle {
                    node: o_style_components.0.into_inner(),
                    background_color: o_style_components.1.into_inner(),
                    border_color: o_style_components.2.into_inner(),
                    shadow: o_style_components.3.into_inner(),
                    z_index: o_style_components.4.into_inner(),
                },
                text: ChildText {
                    value: t_components.0.into_inner(),
                    font: t_components.1.into_inner(),
                    layout: t_components.2.into_inner(),
                    color: t_components.3.into_inner()
                }
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
            .filter(|(_, class)| class.0.split(" ").any(|word| word == target_class))
            .map(|(e, _)| e)
            .collect()
    }
}

/// Bundle for creating `dropdown`.
#[derive(Bundle)]
pub struct DropdownBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub overlay_style: ContainerStyle,
    pub state: MakaraDropdownState,
    pub text_bundle: TextBundle,
    pub tooltip_bundle: TooltipBundle
}

impl Default for DropdownBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                padding: UiRect::axes(px(8), px(5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: DEFAULT_BUTTON_BORDER_RADIUS,
                ..default()
            },
            background_color: BackgroundColor(LIGHT_BUTTON_BG_COLOR),
            ..default()
        };

        let overlay_style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                justify_content: JustifyContent::Stretch,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                display: Display::None,
                border_radius: DEFAULT_BUTTON_BORDER_RADIUS,
                ..default()
            },
            background_color: BackgroundColor(LIGHT_BUTTON_BG_COLOR),
            ..default()
        };

        let text_bundle = TextBundle::default();
        let state = MakaraDropdownState(false);
        let tooltip_bundle = TooltipBundle::default();
        let id_class = IdAndClass::default();

        Self { style, overlay_style, text_bundle, state, tooltip_bundle, id_class }
    }
}

impl DropdownBundle {
    /// Replace text style with provided style.
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_bundle.text_style = style;
        self
    }

    /// Replace overlay style with provided style
    pub fn overlay_style(mut self, style: ContainerStyle) -> Self {
        self.overlay_style = style;
        self
    }

    /// Build dropdown as disabled.
    pub fn build_as_disabled(self) -> impl Bundle {
        (self.build(), InteractionDisabled)
    }
}

impl Widget for DropdownBundle {
    /// Build `dropdown`.
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            self.state,
            MakaraDropdown,
            MakaraWidget,
            MakaraDropdownOverlayStyle(self.overlay_style),
            MakaraDropdownTextBundle(self.text_bundle),
            MakaraDropdownTooltipBundle(self.tooltip_bundle),
            WidgetFocus(false),
            observe(on_dropdown_mouse_over),
            observe(on_mouse_out),
            observe(on_dropdown_click)
        )
    }
}

impl SetContainerStyle for DropdownBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for DropdownBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for DropdownBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default dropdown (light variant) as default theme is light.
pub fn dropdown(text: &str) -> DropdownBundle {
    let mut bundle = DropdownBundle::default();
    bundle.text_bundle.text.0 = format!("{text} v");
    bundle
}

fn on_dropdown_mouse_over(
    mut over: On<Pointer<Over>>,
    mut dropdowns: Query<
        (Has<InteractionDisabled>, &Children, &UiTransform, &ComputedNode),
        With<MakaraDropdown>
    >,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    window: Single<Entity, With<Window>>,
) {

    if let Ok((is_disabled, children, transform, computed)) = dropdowns.get_mut(over.entity) {
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

fn on_dropdown_click(
    mut click: On<Pointer<Click>>,
    mut dropdown_q: Query<&mut MakaraDropdownState>,
    mut widgets: Query<(Entity, &mut WidgetFocus)>,
    mut commands: Commands
) {
    update_focus_state_for_widgets_on_click(click.entity, &mut widgets);

    if let Ok(mut state) = dropdown_q.get_mut(click.entity) {
        state.0 = !state.0;

        match state.0 {
            true => commands.trigger(Active {
                entity: click.entity,
                data: true
            }),
            false => commands.trigger(Inactive {
                entity: click.entity,
                data: false
            })
        }
    }
    commands.trigger(Clicked { entity: click.entity });
    click.propagate(false);
}

pub(crate) fn show_and_hide_dropdown_overlay_on_state_change_system(
    dropdown_q: Query<
        (&MakaraDropdownState, &ComputedNode, &UiTransform, &Children),
        Or<(Changed<MakaraDropdownState>, Added<MakaraDropdownState>)>
    >,
    mut overlay_q: Query<&mut Node, With<MakaraDropdownOverlay>>
) {
    for (state, computed_node, dropdown_transform, children) in dropdown_q.iter() {
        for child in children.iter() {
            if let Ok(mut node) = overlay_q.get_mut(child) {
                match state.0 {
                    true  => {
                        node.display = Display::default();

                        let (Val::Px(x), Val::Px(y)) = (dropdown_transform.translation.x, dropdown_transform.translation.y)
                        else {
                            break;
                        };

                        node.left = px(x);
                        node.top = px(y + (computed_node.size.y * computed_node.inverse_scale_factor) + 2.0);
                    },
                    false => {
                        node.display = Display::None;
                    }
                }
                break;
            }
        }
    }
}

pub(crate) fn detect_user_provided_children_system(
    mut dropdown_q: Query<
        (
            Entity,
            &Children,
            &mut MakaraDropdownOverlayStyle,
            &MakaraDropdownTextBundle,
            &MakaraDropdownTooltipBundle
        ),
        (With<MakaraDropdown>, Added<Children>)
    >,
    mut commands: Commands,
    mut makara_theme: ResMut<MakaraTheme>,
    mut overlay_and_text_added: ResMut<DropdownOverlayAndTextAdded>,
) {
    if overlay_and_text_added.0 {
        return;
    }

    for (entity, children, mut overlay_style, text_bundle, tooltip_bundle) in dropdown_q.iter_mut() {
        let mut child_entities: Vec<Entity> = Vec::new();

        for child in children.iter() {
            commands
                .entity(child)
                .insert(MakaraDropdownItem)
                .insert(BoxShadow::default())
                .insert(BackgroundColor(Color::NONE))
                .observe(move |
                    mut click: On<Pointer<Click>>,
                    mut dropdown: Query<&mut MakaraDropdownState>,
                | {
                    if let Ok(mut state) = dropdown.get_mut(entity) {
                        state.0 = false;
                    }
                    click.propagate(false);
                });

            child_entities.push(child);
        }

        let dropdown_text = commands.spawn((
            text_bundle.0.clone(),
            MakaraText
        )).id();

        let tooltip = commands.spawn(tooltip_bundle.0.clone().build()).id();

        if overlay_style.0.background_color.0 == LIGHT_BUTTON_BG_COLOR ||
           overlay_style.0.background_color.0 == DARK_BUTTON_BG_COLOR
        {
            overlay_style.0.background_color.0 = match makara_theme.theme {
                Theme::Light => LIGHT_BUTTON_BG_COLOR,
                Theme::Dark => DARK_BUTTON_BG_COLOR
            };
        }

        let overlay = commands.spawn((
            overlay_style.0.clone(),
            MakaraDropdownOverlay,
            GlobalZIndex(9)
        ))
        .add_children(&child_entities)
        .id();

        commands.entity(entity).add_children(&[overlay, dropdown_text, tooltip]);
        makara_theme.set_changed();
    }

    overlay_and_text_added.0 = true;
}

pub(crate) fn detect_dropdown_overlay_added(
    overlay_added: Res<DropdownOverlayAndTextAdded>,
    dropdown_q: Query<&ComputedNode, With<MakaraDropdown>>,
    mut overlay_q: Query<(&mut Node, &ChildOf, &ComputedNode), With<MakaraDropdownOverlay>>,
) {
    if !overlay_added.0 {
        return;
    }

    for (mut node, parent, overlay_computed_node) in overlay_q.iter_mut() {
        if let Ok(dropdown_computed) = dropdown_q.get(parent.0) {
            if overlay_computed_node.size.x < dropdown_computed.size.x {
                node.width = px(dropdown_computed.size.x * dropdown_computed.inverse_scale_factor);
            }
        }
    }
}

pub(crate) fn update_dropdown_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut dropdown_q: Query<&mut BackgroundColor, (With<MakaraDropdown>, Without<MakaraDropdownOverlay>)>,
    mut overlay_q: Query<&mut BackgroundColor, (With<MakaraDropdownOverlay>, Without<MakaraDropdown>)>
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_bg_color = match makara_theme.theme {
        Theme::Light => LIGHT_BUTTON_BG_COLOR,
        Theme::Dark => DARK_BUTTON_BG_COLOR
    };

    for mut bg_color in dropdown_q.iter_mut() {
        // only react to theme change if color is default
        if bg_color.0 == LIGHT_BUTTON_BG_COLOR || bg_color.0 == DARK_BUTTON_BG_COLOR {
            bg_color.0 = new_bg_color;
        }
    }

    for mut bg_color in overlay_q.iter_mut() {
        // only react to theme change if color is default
        if bg_color.0 == LIGHT_BUTTON_BG_COLOR || bg_color.0 == DARK_BUTTON_BG_COLOR {
            bg_color.0 = new_bg_color;
        }
    }
}

pub(crate) fn detect_dropdown_built(
    mut commands: Commands,
    q: Query<Entity, Added<MakaraDropdown>>
) {
    for entity in q.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn can_run_dropdown_systems(q: Query<&MakaraDropdown>) -> bool {
    q.count() > 0
}
