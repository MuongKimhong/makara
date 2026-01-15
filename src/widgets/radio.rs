use std::collections::HashMap;

use bevy::{prelude::*, ui::InteractionDisabled, ui_widgets::observe};
use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::utils::update_focus_state_for_widgets_on_click;
use crate::{consts::*, events::*, on_mouse_out};
use super::*;

/// Marker component for radio.
#[derive(Component)]
pub struct MakaraRadio;

/// Marker component for `radio_group`.
#[derive(Component)]
pub struct MakaraRadioGroup;

/// Component used to store state of `radio`.
#[derive(Component)]
pub struct MakaraRadioState(pub bool);

#[derive(Component)]
pub struct MakaraRadioSelected(pub String);

/// Marker component for `radio`'s button.
#[derive(Component)]
pub struct MakaraRadioButton;

/// A struct used to mutate components attached to `radio` widget.
pub struct RadioWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub button_style: WidgetStyle<'a>,
    pub text: ChildText<'a>,
    pub commands: &'a mut Commands<'w, 's>
}

/// `radio` system param.
#[derive(SystemParam)]
pub struct RadioQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraRadio>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), With<MakaraRadio>>,
    pub style: StyleQuery<'w, 's, With<MakaraRadio>>,
    pub button_style: StyleQuery<'w, 's,
        (With<MakaraRadioButton>, Without<MakaraRadio>)
    >,
    pub text: TextQueryAsChild<'w, 's>,
    pub children: Query<'w, 's, &'static Children>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for RadioQuery<'w, 's> {
     type WidgetView<'a> = RadioWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let RadioQuery { id: _, class, style, button_style, text, children, commands } = self;

        let mut text_entity = None;
        let mut btn_entity = None;

        let children_list = children.get(entity).ok()?;

        for child in children_list {
            if text_entity.is_none() && text.query.get(*child).is_ok() {
                text_entity = Some(*child);
            }

            if btn_entity.is_none() && button_style.query.get(*child).is_ok() {
                btn_entity = Some(*child);
            }
        }

        // 2. BORROW PHASE: Now that the loop is over, we borrow specifically what we need.
        if let (Some(t_ent), Some(b_ent)) = (text_entity, btn_entity) {
            // Fetch the parent styles
            let style_bundle = style.query.get_mut(entity).ok()?;
            let (node, bg, border, shadow, z) = style_bundle;

            // Fetch the children components mutably (only happens once!)
            let t_components = text.query.get_mut(t_ent).ok()?;
            let b_style_components = button_style.query.get_mut(b_ent).ok()?;

            return Some(RadioWidget {
                entity,
                class: class.get_mut(entity).ok()?.1.into_inner(),
                style: WidgetStyle {
                    node: node.into_inner(),
                    background_color: bg.into_inner(),
                    border_color: border.into_inner(),
                    shadow: shadow.into_inner(),
                    z_index: z.into_inner(),
                },
                button_style: WidgetStyle {
                    node: b_style_components.0.into_inner(),
                    background_color: b_style_components.1.into_inner(),
                    border_color: b_style_components.2.into_inner(),
                    shadow: b_style_components.3.into_inner(),
                    z_index: b_style_components.4.into_inner(),
                },
                text: ChildText {
                    value: t_components.0.into_inner(),
                    font: t_components.1.into_inner(),
                    layout: t_components.2.into_inner(),
                    color: t_components.3.into_inner(),
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
            .filter(|(_, class)| class.0.split(" ").any(|word| word == target_class))
            .map(|(e, _)| e)
            .collect()
    }
}

/// Bundle for creating `radio`.
#[derive(Bundle)]
pub struct RadioBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub button_style: ContainerStyle,
    pub text_bundle: TextBundle,
    pub state: MakaraRadioState,
    pub tooltip_bundle: TooltipBundle
}

impl Default for RadioBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            shadow: BoxShadow::default(),
            ..default()
        };

        let button_style = ContainerStyle {
            node: Node {
                width: px(15),
                height: px(15),
                border: UiRect::all(px(2)),
                display: Display::Flex,
                margin: UiRect::top(px(1.5)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            border_color: BorderColor::all(LIGHT_THEME_TEXT_COLOR),
            background_color: BackgroundColor(RADIO_UNCHECKED_COLOR),
            shadow: BoxShadow::default(),
            ..default()
        };

        let text_bundle = TextBundle {
            id_class: IdAndClass::default(),
            text: Text::new(""),
            text_style: TextStyle {
                color: TextColor(LIGHT_THEME_TEXT_COLOR),
                font: TextFont::from_font_size(DEFAULT_TEXT_FONT_SIZE),
                ..default()
            },
            style: ContainerStyle {
                node: Node {
                    margin: UiRect {
                        top: px(2),
                        left: px(2.5),
                        ..default()
                    },
                    ..default()
                },
                shadow: BoxShadow::default(),
                ..default()
            }
        };

        let state = MakaraRadioState(false);
        let tooltip_bundle = TooltipBundle::default();
        let id_class = IdAndClass::default();

        Self { style, button_style, text_bundle, state, tooltip_bundle, id_class }
    }
}


impl RadioBundle {
    /// Replace text style with provided style.
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_bundle.text_style = style;
        self
    }

    /// Set checkbox as active.
    pub fn active(mut self) -> Self {
        self.state.0 = true;
        self
    }

    /// Build `radio` as disabled.
    pub fn build_as_disabled(self) -> impl Bundle {
        (self.build(), InteractionDisabled)
    }
}

impl Widget for RadioBundle {
    /// Build `radio`.
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            self.state,
            MakaraRadio,
            MakaraWidget,
            WidgetFocus(false),
            children![
                (self.button_style, MakaraRadioButton),
                (self.text_bundle, MakaraText),
                self.tooltip_bundle.build()
            ],
            observe(on_radio_mouse_over),
            observe(on_mouse_out),
            observe(on_radio_click)
        )
    }
}

impl SetContainerStyle for RadioBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for RadioBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for RadioBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create radio widget. This widget only work when it's a child of `radio_group`.
pub fn radio(text: &str) -> RadioBundle {
    let mut bundle = RadioBundle::default();
    bundle.text_bundle.text.0 = text.to_string();
    bundle
}

fn on_radio_mouse_over(
    mut over: On<Pointer<Over>>,
    mut radios: Query<
        (Has<InteractionDisabled>, &Children, &UiTransform, &ComputedNode),
        With<MakaraRadio>
    >,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    window: Single<Entity, With<Window>>,
) {

    if let Ok((is_disabled, children, transform, computed)) = radios.get_mut(over.entity) {
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

// emit clicked event for radio child.
// emit active & inactive for radio child.
// emit change event for radio group.
fn on_radio_click(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut radio: Query<(&mut MakaraRadioState, &Children, &ChildOf, Entity), With<MakaraRadio>>,
    mut radio_groups: Query<&mut MakaraRadioSelected, With<MakaraRadioGroup>>,
    mut widgets: Query<(Entity, &mut WidgetFocus)>,
    text_q: Query<&Text, With<MakaraText>>
) {
    update_focus_state_for_widgets_on_click(click.entity, &mut widgets);

    let mut radio_group_entity: Option<Entity> = None;
    let mut selected_text = String::from("");

    if let Ok((mut state, children, parent, _)) = radio.get_mut(click.entity) {
        state.0 = !state.0;

        for child in children {
            if let Ok(text) = text_q.get(*child) {
                if let Ok(mut selected) = radio_groups.get_mut(parent.0) {
                    selected.0 = text.0.clone();
                    radio_group_entity = Some(parent.0);
                    selected_text = text.0.clone();

                    commands.trigger(Change {
                        entity: parent.0,
                        data: text.0.clone()
                    });

                    commands.trigger(Active {
                        entity: click.entity,
                        data: text.0.clone()
                    });
                }
            }
        }

        if let Some(group_entity) = radio_group_entity {
            for (mut state, _, parent, entity) in radio.iter_mut() {
                if parent.0 == group_entity && entity != click.entity {
                    state.0 = false;

                    commands.trigger(Inactive {
                        entity,
                        data: selected_text.clone()
                    });
                }
            }
        }

        commands.trigger(Clicked { entity: click.entity });
        click.propagate(false);
    }
}

// emit active & inactive for radio child.
// emit change event for radio group.
fn on_radio_group_value_set(
    value_set: On<SetRadioGroupValue>,
    mut commands: Commands,
    mut radio: Query<(&mut MakaraRadioState, &Children, &ChildOf, Entity), With<MakaraRadio>>,
    mut radio_groups: Query<&mut MakaraRadioSelected, With<MakaraRadioGroup>>,
    text_q: Query<&Text, With<MakaraText>>
) {
    let mut selected_text = String::from("");

    if let Ok((mut state, children, _, _)) = radio.get_mut(value_set.radio_entity) {
        state.0 = !state.0;

        for child in children {
            if let Ok(text) = text_q.get(*child) {
                if let Ok(mut selected) = radio_groups.get_mut(value_set.entity) {
                    selected.0 = text.0.clone();
                    selected_text = text.0.clone();

                    commands.trigger(Change {
                        entity: value_set.entity,
                        data: text.0.clone()
                    });

                    commands.trigger(Active {
                        entity: value_set.entity,
                        data: text.0.clone()
                    });
                }
            }
        }

        for (mut state, _, parent, entity) in radio.iter_mut() {
            if parent.0 == value_set.entity && entity != value_set.radio_entity {
                state.0 = false;

                commands.trigger(Inactive {
                    entity,
                    data: selected_text.clone()
                });
            }
        }
    }
}

pub(crate) fn update_radio_style_on_state_change_system(
    radios: Query<
        (&MakaraRadioState, &Children),
        (With<MakaraRadio>, Or<(Changed<MakaraRadioState>, Added<MakaraRadioState>)>)
    >,
    mut radio_btns: Query<&mut BackgroundColor, With<MakaraRadioButton>>,
) {
    for (state, children) in radios.iter() {
        for child in children {
            if let Ok(mut bg_color) = radio_btns.get_mut(*child) {
                match state.0 {
                    true => bg_color.0 = CHECKBOX_CHECKED_COLOR,
                    false => bg_color.0 = CHECKBOX_UNCHECKED_COLOR
                }
            }
        }
    }
}


/// A struct used to mutate components attached to `radio_group` widget.
pub struct RadioGroupWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub radio_entities: HashMap<Entity, String>, // entity-radio_text
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub commands: &'a mut Commands<'w, 's>,
}

impl<'a,'w, 's> RadioGroupWidget<'a, 'w, 's> {
    /// Set a specific `radio_group` item to active.
    pub fn set_active(&mut self, value: &str) {
        for (radio_entity, radio_text) in self.radio_entities.iter() {
            if radio_text == value {
                self.commands.trigger(SetRadioGroupValue {
                    entity: self.entity,
                    radio_entity: *radio_entity,
                    radio_text: value.to_string()
                });
                break;
            }
        }
    }
}

/// `radio_group` system param.
#[derive(SystemParam)]
pub struct RadioGroupQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraRadioGroup>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), With<MakaraRadioGroup>>,
    pub style: StyleQuery<'w, 's, With<MakaraRadioGroup>>,
    pub radios: Query<'w, 's, &'static Children, With<MakaraRadio>>,
    pub texts: Query<'w, 's, &'static Text>,
    pub children: Query<'w, 's, &'static Children>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for RadioGroupQuery<'w, 's> {
    type WidgetView<'a> = RadioGroupWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let RadioGroupQuery { id: _, class, style, radios, texts, children, commands } = self;

        let mut radio_entity_text_map: HashMap<Entity, String> = HashMap::new();
        let radio_group_children = children.get(entity).ok()?;

        for child in radio_group_children {
            if let Ok(radio_children) = radios.get(*child) {
                for radio_child in radio_children {
                    if let Ok(text) = texts.get(*radio_child) {
                        radio_entity_text_map.insert(*child, text.0.clone());
                        break;
                    }
                }
                break;
            }
        }

        let style_bundle = style.query.get_mut(entity).ok()?;
        let (node, bg, border, shadow, z) = style_bundle;

        return Some(RadioGroupWidget {
            entity,
            class: class.get_mut(entity).ok()?.1.into_inner(),
            style: WidgetStyle {
                node: node.into_inner(),
                background_color: bg.into_inner(),
                border_color: border.into_inner(),
                shadow: shadow.into_inner(),
                z_index: z.into_inner(),
            },
            radio_entities: radio_entity_text_map,
            commands: commands
        });
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

/// Bundle for creating `radio_group`.
#[derive(Bundle)]
pub struct RadioGroupBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
}

impl Default for RadioGroupBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            shadow: BoxShadow::default(),
            ..default()
        };

        Self { style, id_class: IdAndClass::default() }
    }
}

impl RadioGroupBundle {
    /// Set direction of radio item as "row" or "column".
    /// Default is "row";
    pub fn direction(mut self, d: &str) -> Self {
        if d == "column" {
            self.style.node.flex_direction = FlexDirection::Column;
        }
        self
    }
}

impl Widget for RadioGroupBundle {
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            MakaraRadioGroup,
            MakaraRadioSelected("".to_string()),
            observe(on_radio_group_value_set)
        )
    }
}

impl SetContainerStyle for RadioGroupBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetIdAndClass for RadioGroupBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

pub fn radio_group() -> RadioGroupBundle {
    RadioGroupBundle::default()
}

pub(crate) fn update_radio_button_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut radio_btn_q: Query<&mut BorderColor, With<MakaraRadioButton>>,
) {
    if !makara_theme.is_changed() {
        return;
    }

    // use color same as text
    let new_bd_color = match makara_theme.theme {
        Theme::Light => LIGHT_THEME_TEXT_COLOR,
        Theme::Dark => DARK_THEME_TEXT_COLOR
    };

    let default_light = BorderColor::all(LIGHT_THEME_TEXT_COLOR);
    let default_dark = BorderColor::all(DARK_THEME_TEXT_COLOR);

    for mut bd_color in radio_btn_q.iter_mut() {
        // only react to theme change if color is default
        if *bd_color == default_light || *bd_color == default_dark {
            *bd_color = BorderColor::all(new_bd_color);
        }
    }
}

pub(crate) fn detect_radio_built(
    mut commands: Commands,
    q_1: Query<Entity, Added<MakaraRadioGroup>>,
    q_2: Query<Entity, Added<MakaraRadio>>,
) {
    for entity in q_1.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }

    for entity in q_2.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn can_run_radio_systems(q: Query<&MakaraRadio>) -> bool {
    q.count() > 0
}
