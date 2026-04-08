use bevy::{prelude::*, ui::InteractionDisabled, ui_widgets::observe};
use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::utils::update_focus_state_for_widgets_on_click;
use crate::{consts::*, events::*, utils::*, on_mouse_out};
use super::*;

/// Marker component for `select`.
#[derive(Component)]
pub struct MakaraSelect;

/// Component used to store state of `select`.
#[derive(Component)]
pub struct MakaraSelectState(pub bool);

/// Marker component for `select`'s choice.
#[derive(Component)]
pub struct MakaraSelectChoice(pub String);

#[derive(Component)]
pub struct MakaraSelectChoices(pub Vec<String>);

/// Marker component for select overlay.
#[derive(Component)]
pub struct MakaraSelectOverlay;

#[derive(Component)]
pub struct MakaraSelectSelected(pub String);

#[derive(Component)]
pub struct MakaraSelectEntity(pub Entity);

#[derive(Component)]
pub struct MakaraSelectPlaceholder(pub String);

#[derive(Component, Default)]
pub struct SelectPlaceholderNodeResized(pub bool);

/// Use this component to store placeholder text entity.
/// This component is used by main select widget and placeholder wrapper.
#[derive(Component)]
pub struct SelectPlaceholderTextEntity(pub Entity);

/// Marker component for arrow.
#[derive(Component)]
pub struct SelectArrow;

/// Marker component for placeholder.
#[derive(Component)]
pub struct SelectPlaceholder;

#[derive(Component, Default)]
pub(crate) struct SelectItemsAdded {
    pub items_added: bool,
    pub overlay_resized: bool
}

/// A struct used to mutate components attached to `select` widget.
pub struct SelectWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub overlay_style: WidgetStyle<'a>,
    pub placeholder: ChildText<'a>,
    pub arrow: ChildText<'a>,
    pub commands: &'a mut Commands<'w, 's>
}

impl<'a, 'w, 's> SetText for SelectWidget<'a, 'w, 's> {
    fn set_text(&mut self, text: &str) {
        self.placeholder.value.0 = text.to_string();
    }
}

impl<'a, 'w, 's> SelectWidget<'a, 'w, 's> {
    pub fn set_value(&mut self, value: &str) {
        self.commands.trigger(SetSelectValue {
            entity: self.entity,
            value: value.to_string()
        });
    }
}

/// `select` system param.
#[derive(SystemParam)]
pub struct SelectQuery<'w, 's> {
    pub select_related: Query<
        'w, 's,
        (Entity, &'static Id, &'static mut Class, &'static Children),
        With<MakaraSelect>
    >,
    pub style: StyleQuery<'w, 's, With<MakaraSelect>>,
    pub overlay_style: StyleQuery<'w, 's,
        (With<MakaraSelectOverlay>, Without<MakaraSelect>)
    >,
    pub children: Query<'w, 's, &'static Children>,
    pub arrow_text: TextQueryAsChild<'w, 's, With<SelectArrow>>,
    pub placeholder_text: TextQueryAsChild<'w, 's,
        (With<SelectPlaceholder>, Without<SelectArrow>)
    >,
    pub placeholder_node: Query<'w, 's, &'static SelectPlaceholderNodeResized>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for SelectQuery<'w, 's> {
     type WidgetView<'a> = SelectWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let SelectQuery {
            select_related, style, overlay_style, children, commands,
            arrow_text, placeholder_text, placeholder_node
        } = self;

        let select_related_bundle = select_related.get_mut(entity).ok()?;
        let (entity, _, class, select_children) = select_related_bundle;


        let mut pl_text_entity = None;

        for child in select_children {
            if let Ok(_) = placeholder_node.get(*child) {
                let placeholder_node_children = children.get(*child).ok()?;
                for sub_child in placeholder_node_children {
                    if let Ok(_) = placeholder_text.query.get(*sub_child) {
                        pl_text_entity = Some(*sub_child);
                        break;
                    }
                }
                break;
            }
        }

        let mut overlay_entity = None;
        let mut arrow_entity = None;

        let children_list = children.get(entity).ok()?;
        for child in children_list {
            if overlay_entity.is_none() && overlay_style.query.get(*child).is_ok() {
                overlay_entity = Some(*child);
            }

            if arrow_entity.is_none() && arrow_text.query.get(*child).is_ok() {
                arrow_entity = Some(*child);
            }
        }

        // 2. BORROW PHASE: Now that the loop is over, we borrow specifically what we need.
        if let (Some(overlay_entity), Some(arrow_entity), Some(pl_entity)) = (overlay_entity, arrow_entity, pl_text_entity) {
            let style_bundle = style.query.get_mut(entity).ok()?;
            let (node, bg, border, shadow, z) = style_bundle;

            let overlay_components = overlay_style.query.get_mut(overlay_entity).ok()?;
            let arrow_components = arrow_text.query.get_mut(arrow_entity).ok()?;
            let placeholder_components = placeholder_text.query.get_mut(pl_entity).ok()?;

            return Some(SelectWidget {
                entity,
                class: class.into_inner(),
                style: WidgetStyle {
                    node: node.into_inner(),
                    background_color: bg.into_inner(),
                    border_color: border.into_inner(),
                    shadow: shadow.into_inner(),
                    z_index: z.into_inner(),
                },
                overlay_style: WidgetStyle {
                    node: overlay_components.0.into_inner(),
                    background_color: overlay_components.1.into_inner(),
                    border_color: overlay_components.2.into_inner(),
                    shadow: overlay_components.3.into_inner(),
                    z_index: overlay_components.4.into_inner(),
                },
                placeholder: ChildText {
                    value: placeholder_components.0.into_inner(),
                    font: placeholder_components.1.into_inner(),
                    layout: placeholder_components.2.into_inner(),
                    color: placeholder_components.3.into_inner(),
                },
                arrow: ChildText {
                    value: arrow_components.0.into_inner(),
                    font: arrow_components.1.into_inner(),
                    layout: arrow_components.2.into_inner(),
                    color: arrow_components.3.into_inner(),
                },
                commands: commands
            });
        }

        None
    }

    fn find_by_id<'a>(&'a mut self, target_id: &str) -> Option<Self::WidgetView<'a>> {
        let entity = self.select_related.iter()
            .find(|(_, id, _, _)| {
                id.0 == target_id
            })
            .map(|(e, _, _, _)| e)?;

        self.get_components(entity)
    }

    fn find_by_entity<'a>(&'a mut self, target_entity: Entity) -> Option<Self::WidgetView<'a>> {
        self.get_components(target_entity)
    }

    fn find_by_class(&self, target_class: &str) -> Vec<Entity> {
        self.select_related.iter()
            .filter(|(_, _, class, _)| class.value.split(" ").any(|word| word == target_class))
            .map(|(e, _, _, _)| e)
            .collect()
    }
}

/// Bundle for creating `select`.
#[derive(Bundle)]
pub struct SelectBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub text_bundle: TextBundle,
    pub arrow_text_bundle: TextBundle,
    pub overlay_style: ContainerStyle,
    pub state: MakaraSelectState,
    pub choices: MakaraSelectChoices,
    pub placeholder: MakaraSelectPlaceholder,
    pub tooltip_bundle: TooltipBundle
}

impl Default for SelectBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                padding: UiRect::axes(px(8), px(5)),
                justify_content: JustifyContent::SpaceBetween,
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

        let text_bundle = TextBundle {
            id_class: IdAndClass::default(),
            text: Text::new(""),
            text_style: TextStyle {
                color: TextColor(LIGHT_THEME_TEXT_COLOR),
                font: TextFont::from_font_size(DEFAULT_TEXT_FONT_SIZE),
                layout: TextLayout::new_with_no_wrap()
            },
            style: ContainerStyle {
                shadow: BoxShadow::default(),
                ..default()
            }
        };

        let arrow_text_bundle = TextBundle {
            id_class: IdAndClass::default(),
            text: Text::new("v"),
            text_style: TextStyle {
                color: TextColor(LIGHT_THEME_TEXT_COLOR),
                font: TextFont::from_font_size(DEFAULT_TEXT_FONT_SIZE),
                ..default()
            },
            style: ContainerStyle {
                node: Node {
                    margin: UiRect::left(px(5)),
                    ..default()
                },
                shadow: BoxShadow::default(),
                ..default()
            }
        };

        let state = MakaraSelectState(false);
        let choices = MakaraSelectChoices(Vec::new());
        let placeholder = MakaraSelectPlaceholder("".to_string());
        let tooltip_bundle = TooltipBundle::default();

        Self {
            id_class: IdAndClass::default(),
            style,
            overlay_style,
            text_bundle,
            state,
            choices,
            arrow_text_bundle,
            placeholder,
            tooltip_bundle
        }
    }
}

impl SelectBundle {
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

    /// Build select as disabled.
    pub fn build_as_disabled(self) -> impl Bundle {
        (self.build(), InteractionDisabled)
    }
}

impl Widget for SelectBundle {
    /// Build `select`.
    fn build(mut self) -> impl Bundle {
        process_built_in_spacing_class(&self.id_class.class, &mut self.style.node);
        process_built_in_color(
            &self.id_class.class,
            &mut self.style.background_color.0
        );
        process_built_in_color(
            &self.id_class.class,
            &mut self.overlay_style.background_color.0
        );
        process_text_built_in_color_class(
            &self.id_class.class,
            &mut self.text_bundle.text_style.color.0
        );
        process_text_built_in_color_class(
            &self.id_class.class,
            &mut self.arrow_text_bundle.text_style.color.0
        );
        process_placeholder_text_built_in_color_class(
            &self.id_class.class,
            &mut self.text_bundle.text_style.color.0
        );

        (
            self.id_class,
            self.style,
            self.state,
            self.choices,
            self.placeholder,
            MakaraSelect,
            MakaraWidget,
            MakaraSelectSelected("".to_string()),
            SelectItemsAdded {
                items_added: false,
                overlay_resized: true
            },
            WidgetFocus(false),
            children![
                (self.overlay_style, MakaraSelectOverlay, GlobalZIndex(9)),
                (
                    Node {
                        width: percent(100.0),
                        overflow: Overflow::clip_x(),
                        ..default()
                    },
                    SelectPlaceholderNodeResized::default(),
                    children![
                        (self.text_bundle, MakaraText, SelectPlaceholder)
                    ]
                ),
                (self.arrow_text_bundle, MakaraText, SelectArrow),
                self.tooltip_bundle.build()
            ],
            observe(on_select_value_set),
            observe(on_select_mouse_over),
            observe(on_mouse_out),
            observe(on_select_click),
        )
    }
}

impl SetContainerStyle for SelectBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for SelectBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for SelectBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default select (light variant) as default theme is light.
pub fn select(placeholder: &str, choices: &[&str]) -> SelectBundle {
    let mut bundle = SelectBundle::default();
    bundle.text_bundle.text.0 = format!("{placeholder}");
    bundle.text_bundle.text_style.color.0 = LIGHT_PLACEHOLDER_COLOR;
    bundle.arrow_text_bundle.text_style.color.0 = LIGHT_PLACEHOLDER_COLOR;
    bundle.choices.0 = choices.iter().map(|s| s.to_string()).collect();
    bundle.placeholder.0 = placeholder.to_string();
    bundle
}

pub(crate) fn detect_select_class_change_for_built_in(
    mut selects: Query<
        (&Class, &Children, &SelectPlaceholderTextEntity, &mut Node, &mut BackgroundColor),
        (With<MakaraSelect>, Changed<Class>)
    >,
    mut overlays: Query<
        &mut BackgroundColor,
        (With<MakaraSelectOverlay>, Without<MakaraSelect>)
    >,
    mut texts: Query<&mut TextColor, With<MakaraText>>
) {
    for (class, children, pl_entity, mut node, mut bg) in selects.iter_mut() {
        for child in children.iter() {
            if let Ok(mut overlay_bg) = overlays.get_mut(child) {
                process_built_in_color(class, &mut overlay_bg.0);
            }

            if let Ok(mut arrow_color) = texts.get_mut(child) {
                process_text_built_in_color_class(class, &mut arrow_color.0);
            }
        }

        if let Ok(mut pl_color) = texts.get_mut(pl_entity.0) {
            process_text_built_in_color_class(class, &mut pl_color.0);
        }
        process_built_in_spacing_class(class, &mut node);
        process_built_in_color( class, &mut bg.0);
    }
}

fn on_select_mouse_over(
    mut over: On<Pointer<Over>>,
    mut selects: Query<
        (Has<InteractionDisabled>, &Children, &UiTransform, &ComputedNode),
        With<MakaraSelect>
    >,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    window: Single<Entity, With<Window>>,
) {

    if let Ok((is_disabled, children, transform, computed)) = selects.get_mut(over.entity) {
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

fn on_select_click(
    mut click: On<Pointer<Click>>,
    mut widgets: Query<(Entity, &mut WidgetFocus)>,
    mut commands: Commands
) {
    update_focus_state_for_widgets_on_click(click.entity, &mut widgets);
    commands.trigger(Clicked { entity: click.entity });
    click.propagate(false);
}

pub(crate) fn update_select_state_on_focus_change(
    mut commands: Commands,
    mut select_q: Query<
        (Entity, &WidgetFocus, &mut MakaraSelectState),
        (With<MakaraSelect>, Changed<WidgetFocus>)
    >,
) {
    for (entity, focus, mut state) in select_q.iter_mut() {
        if focus.0 && state.0 {
            state.0 = false;
        }
        else {
            state.0 = focus.0;
        }

        match state.0 {
            true => commands.trigger(Active {
                entity,
                data: true
            }),
            false => commands.trigger(Inactive {
                entity,
                data: false
            })
        }
    }
}

pub(crate) fn update_select_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut select_q: Query<&mut BackgroundColor, (With<MakaraSelect>, Without<MakaraSelectOverlay>)>,
    mut overlay_q: Query<&mut BackgroundColor, (With<MakaraSelectOverlay>, Without<MakaraSelect>)>
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_bg_color = match makara_theme.theme {
        Theme::Light => LIGHT_BUTTON_BG_COLOR,
        Theme::Dark => DARK_BUTTON_BG_COLOR
    };

    for mut bg_color in select_q.iter_mut() {
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

pub(crate) fn detect_select_placeholder_added(
    mut placeholder: Query<
        (Entity, &mut Node, &mut SelectPlaceholderNodeResized, &ComputedNode, &Children, &ChildOf),
        With<SelectPlaceholderNodeResized>
    >,
    mut commands: Commands,
    selects: Query<(&SelectPlaceholderTextEntity, &Id, &Class), With<MakaraSelect>>,
    text: Query<Entity, With<MakaraText>>
) {
    for (pl_entity, mut node, mut resized, computed, children, parent) in placeholder.iter_mut() {
        if selects.get(parent.0).is_err() {
            for child in children {
                if let Ok(entity) = text.get(*child) {
                    commands.entity(pl_entity).insert(SelectPlaceholderTextEntity(entity));
                    commands.entity(parent.0).insert(SelectPlaceholderTextEntity(entity));
                    break;
                }
            }
        }

        if !resized.0 && computed.size.x > 0.0 {
            node.width = px(computed.size.x * computed.inverse_scale_factor());
            node.overflow = Overflow::hidden_x();
            resized.0 = true;
        }
    }
}

pub(crate) fn detect_select_items_added_and_overlay_resized(
    mut select_q: Query<
        (Entity, &mut SelectItemsAdded, &mut MakaraSelectChoices, &ComputedNode, &Children, &Class),
        With<MakaraSelect>
    >,
    mut overlay_q: Query<(&mut Node, &ComputedNode), With<MakaraSelectOverlay>>,
    mut commands: Commands,
    mut makara_theme: ResMut<MakaraTheme>,
) {
    for (entity, mut items_added, mut choices, select_computed, children, class) in select_q.iter_mut() {
        for child in children {
            if let Ok((mut node, overlay_computed)) = overlay_q.get_mut(*child) {
                if !items_added.items_added {
                    choices.0.insert(0, "-/-".to_string());

                    commands.entity(*child).with_children(|p| {
                        for choice in &choices.0 {
                            let mut btn = ButtonBundle::new(choice);
                            btn.text_bundle.text_style.layout = TextLayout::new_with_no_wrap();
                            btn.text_bundle.style.node.overflow = Overflow::clip_x();
                            btn.style.background_color.0 = Color::NONE;
                            btn.style.node.justify_content = JustifyContent::Start;
                            btn.style.shadow = BoxShadow::default();

                            if choice == "-/-" {
                                btn.style.node.margin.top = px(2.5);
                            }

                            process_text_built_in_color_class(
                                class,
                                &mut btn.text_bundle.text_style.color.0
                            );

                            p.spawn((
                                btn.build(),
                                MakaraSelectChoice(choice.clone()),
                                MakaraSelectEntity(entity),
                                observe(on_select_choice_click)
                            ));
                        }
                    });
                    items_added.items_added = true;
                    items_added.overlay_resized = false;
                    makara_theme.set_changed();
                }

                if !items_added.overlay_resized {
                    if overlay_computed.size.x < select_computed.size.x {
                        node.width = px(select_computed.size.x * select_computed.inverse_scale_factor());
                        items_added.overlay_resized = true;
                    }
                }
            }
        }
    }
}

fn on_select_choice_click(
    mut click: On<Pointer<Click>>,
    mut select: Query<(&mut MakaraSelectState, &mut Node, &MakaraSelectPlaceholder, &ComputedNode, &Children)>,
    mut text: Query<(&mut Text, &mut TextColor), (With<MakaraText>, Without<SelectArrow>)>,
    mut commands: Commands,
    pl_node: Query<&SelectPlaceholderTextEntity, With<SelectPlaceholderNodeResized>>,
    select_choice: Query<(&MakaraSelectChoice, &MakaraSelectEntity), With<MakaraSelectChoice>>,
) {
    if let Ok((choice, select_entity)) = select_choice.get(click.entity) {
        if let Ok((
            mut state,
            mut node,
            pl,
            select_computed,
            select_children
        )) = select.get_mut(select_entity.0) {
            state.0 = false;

            for select_child in select_children {
                if let Ok(pl_text_entity) = pl_node.get(*select_child) {
                    if let Ok((mut text, mut text_color)) = text.get_mut(pl_text_entity.0) {
                        if choice.0 == "-/-" {
                            text.0 = format!("{}", pl.0);
                            text_color.0 = LIGHT_PLACEHOLDER_COLOR;
                        } else {
                            text.0 = format!("{}", choice.0);
                            text_color.0 = LIGHT_THEME_TEXT_COLOR;
                        }
                        node.width = px(select_computed.size.x * select_computed.inverse_scale_factor());
                        break;
                    }
                }
            }

            let choice_to_emit = if choice.0 == "-/-" {
                "".to_string()
            } else {
                choice.0.clone()
            };

            commands.trigger(Change {
                entity: select_entity.0,
                data: choice_to_emit
            });
        }
    }
    click.propagate(false);
}

fn on_select_value_set(
    value_set: On<SetSelectValue>,
    mut select: Query<(
        &mut MakaraSelectState,
        &mut Node,
        &MakaraSelectPlaceholder,
        &ComputedNode,
        &Children,
        &SelectPlaceholderTextEntity
    )>,
    mut text: Query<&mut Text, (With<MakaraText>, Without<SelectArrow>)>,
    mut commands: Commands,
    overlays: Query<&Children, With<MakaraSelectOverlay>>,
    select_choice: Query<&MakaraSelectChoice>,
) {
    if let Ok((
        mut state,
        mut node,
        pl,
        select_computed,
        select_children,
        pl_text_entity
    )) = select.get_mut(value_set.entity) {
        state.0 = false;

        for select_child in select_children {
            if let Ok(overlay_children) = overlays.get(*select_child) {
                for overlay_child in overlay_children {
                    if let Ok(choice) = select_choice.get(*overlay_child) {
                        if choice.0 != value_set.value {
                            continue;
                        }

                        if let Ok(mut text) = text.get_mut(pl_text_entity.0) {
                            if choice.0 == "-/-" {
                                text.0 = format!("{}", pl.0);
                            } else {
                                text.0 = format!("{}", choice.0);
                            }
                            node.width = px(select_computed.size.x * select_computed.inverse_scale_factor());

                            commands.trigger(Change {
                                entity: value_set.entity,
                                data: value_set.value.clone()
                            });
                            break;
                        }
                    }
                }
                break;
            }
        }
    }
}

pub(crate) fn show_and_hide_select_overlay_on_state_change_system(
    select_q: Query<
        (&MakaraSelectState, &ComputedNode, &UiTransform, &Children),
        Or<(Changed<MakaraSelectState>, Added<MakaraSelectState>)>
    >,
    mut overlay_q: Query<&mut Node, With<MakaraSelectOverlay>>
) {
    for (state, computed_node, dropdown_transform, children) in select_q.iter() {
        for child in children.iter() {
            if let Ok(mut node) = overlay_q.get_mut(child) {
                match state.0 {
                    true  => {
                        node.display = Display::default();

                        let (Val::Px(x), Val::Px(y)) = (
                            dropdown_transform.translation.x,
                            dropdown_transform.translation.y
                        )
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


pub(crate) fn can_run_select_systems(q: Query<&MakaraSelect>) -> bool {
    q.count() > 0
}
