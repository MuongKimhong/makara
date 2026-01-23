//! `checkbox` widget.

use bevy::{prelude::*, ui::InteractionDisabled, ui_widgets::observe};
use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::utils::update_focus_state_for_widgets_on_click;
use crate::{consts::*, events::*, on_mouse_out};
use super::*;

/// Marker component for `checkbox`.
#[derive(Component)]
pub struct MakaraCheckbox;

/// Component used to store state of a `checkbox`.
#[derive(Component)]
pub struct MakaraCheckboxState(pub bool);

/// Marker component for `checkbox` button.
#[derive(Component)]
pub struct MakaraCheckboxButton;

#[derive(Component)]
pub struct MakaraCheckboxText;

#[derive(Component)]
pub struct CheckboxButtonActiveColor(pub Color);

/// A struct used to mutate components attached to `checkbox` widget.
pub struct CheckboxWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub button_style: WidgetStyle<'a>,
    pub button_active_color: &'a mut CheckboxButtonActiveColor,
    pub text: ChildText<'a>,
    pub commands: &'a mut Commands<'w, 's>
}

impl<'a, 'w, 's> SetText for CheckboxWidget<'a, 'w, 's> {
    fn set_text(&mut self, text: &str) {
        self.text.value.0 = text.to_string();
    }
}

impl<'a, 'w, 's> CheckboxWidget<'a, 'w, 's> {
    /// Set state of a `checkbox` to active.
    pub fn set_active(&mut self) {
        self.commands.trigger(SetCheckboxState {
            entity: self.entity,
            state: true
        });
    }

    /// Set state of a `checkbox` to inactive.
    pub fn set_inactive(&mut self) {
        self.commands.trigger(SetCheckboxState {
            entity: self.entity,
            state: false
        });
    }
}

type IsCheckboxOnly = (
    (
        With<MakaraCheckbox>,
        Without<MakaraCheckboxButton>,
        Without<MakaraButton>,
        Without<MakaraColumn>,
        Without<MakaraRow>,
        Without<MakaraRoot>,
        Without<MakaraCircular>,
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
        Without<MakaraScroll>,
        Without<MakaraScrollbar>,
        Without<MakaraTextInput>,
        Without<MakaraTextInputCursor>,
        Without<MakaraSlider>,
        Without<MakaraSliderThumb>,
        Without<MakaraSelect>,
        Without<MakaraSelectOverlay>,
    )
);

type IsCheckboxButtonOnly = (
    (
        With<MakaraCheckboxButton>,
        Without<MakaraCheckbox>,
        Without<MakaraButton>,
        Without<MakaraColumn>,
        Without<MakaraRow>,
        Without<MakaraRoot>,
        Without<MakaraCircular>,
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
        Without<MakaraScroll>,
        Without<MakaraScrollbar>,
        Without<MakaraTextInput>,
        Without<MakaraTextInputCursor>,
        Without<MakaraSlider>,
        Without<MakaraSliderThumb>,
        Without<MakaraSelect>,
        Without<MakaraSelectOverlay>,
    )
);

type IsCheckboxTextOnly = (
    With<MakaraCheckboxText>,
    Without<MakaraButtonText>
);

/// `checkbox` system param.
#[derive(SystemParam)]
pub struct CheckboxQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraCheckbox>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), IsCheckboxOnly>,
    pub style: StyleQuery<'w, 's, IsCheckboxOnly>,
    pub button_style: StyleQuery<'w, 's, IsCheckboxButtonOnly>,
    pub button_active_color: Query<'w, 's, &'static mut CheckboxButtonActiveColor>,
    pub text: TextQueryAsChild<'w, 's, IsCheckboxTextOnly>,
    pub children: Query<'w, 's, &'static Children>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for CheckboxQuery<'w, 's> {
     type WidgetView<'a> = CheckboxWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let CheckboxQuery {
            id: _, class, style, button_style, button_active_color, text, children, commands
        } = self;

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
            let active_color = button_active_color.get_mut(b_ent).ok()?;

            return Some(CheckboxWidget {
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
                button_active_color: active_color.into_inner(),
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

/// Bundle for creating `checkbox`.
#[derive(Bundle)]
pub struct CheckboxBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub button_style: ContainerStyle,
    pub button_active_color: CheckboxButtonActiveColor,
    pub text_bundle: TextBundle,
    pub state: MakaraCheckboxState,
    pub tooltip_bundle: TooltipBundle
}

impl Default for CheckboxBundle {
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
                border_radius: BorderRadius::all(px(2.5)),
                ..default()
            },
            border_color: BorderColor::all(LIGHT_THEME_TEXT_COLOR),
            background_color: BackgroundColor(CHECKBOX_UNCHECKED_COLOR),
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

        let state = MakaraCheckboxState(false);
        let tooltip_bundle = TooltipBundle::default();
        let id_class = IdAndClass::default();
        let button_active_color = CheckboxButtonActiveColor(CHECKBOX_CHECKED_COLOR);

        Self { style, button_style, text_bundle, state, tooltip_bundle, id_class, button_active_color }
    }
}

impl CheckboxBundle {
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

    /// Set color for button when active
    pub fn button_active_color(mut self, color: Color) -> Self {
        self.button_active_color.0 = color;
        self
    }

    /// Build `checkbox` as disabled.
    pub fn build_as_disabled(self) -> impl Bundle {
        (self.build(), InteractionDisabled)
    }
}

impl Widget for CheckboxBundle {
    /// Build `checkbox`.
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            self.state,
            MakaraCheckbox,
            MakaraWidget,
            WidgetFocus(false),
            children![
                (
                    self.button_style,
                    self.button_active_color,
                    MakaraCheckboxButton
                ),
                (self.text_bundle, MakaraText, MakaraCheckboxText),
                self.tooltip_bundle.build()
            ],
            observe(on_checkbox_mouse_over),
            observe(on_checkbox_state_set),
            observe(on_mouse_out),
            observe(on_checkbox_click)
        )
    }
}

impl SetContainerStyle for CheckboxBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for CheckboxBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for CheckboxBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

pub fn checkbox(text: &str) -> CheckboxBundle {
    let mut bundle = CheckboxBundle::default();
    bundle.text_bundle.text.0 = text.to_string();
    bundle
}

fn on_checkbox_mouse_over(
    mut over: On<Pointer<Over>>,
    mut checkboxs: Query<
        (Has<InteractionDisabled>, &Children, &UiTransform, &ComputedNode),
        With<MakaraCheckbox>
    >,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
) {

    if let Ok((is_disabled, children, transform, computed)) = checkboxs.get_mut(over.entity) {
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

fn on_checkbox_click(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut checkboxs: Query<(&mut MakaraCheckboxState, &Children), With<MakaraCheckbox>>,
    mut widgets: Query<(Entity, &mut WidgetFocus)>,
    text_q: Query<&Text, With<MakaraText>>
) {
    update_focus_state_for_widgets_on_click(click.entity, &mut widgets);

    if let Ok((mut state, children)) = checkboxs.get_mut(click.entity) {
        state.0 = !state.0;

        for child in children {
            if let Ok(text) = text_q.get(*child) {
               match state.0 {
                    true => commands.trigger(Active {
                        entity: click.entity,
                        data: text.0.clone()
                    }),
                    false => commands.trigger(Inactive {
                        entity: click.entity,
                        data: text.0.clone()
                    })
                }
            }
        }

        commands.trigger(Clicked { entity: click.entity });
        click.propagate(false);
    }
}

fn on_checkbox_state_set(
    set_state: On<SetCheckboxState>,
    mut commands: Commands,
    mut checkboxs: Query<(&mut MakaraCheckboxState, &Children), With<MakaraCheckbox>>,
    text_q: Query<&Text, With<MakaraText>>
) {
    if let Ok((mut state, children)) = checkboxs.get_mut(set_state.entity) {
        state.0 = set_state.state;

        for child in children {
            if let Ok(text) = text_q.get(*child) {
               match state.0 {
                    true => commands.trigger(Active {
                        entity: set_state.entity,
                        data: text.0.clone()
                    }),
                    false => commands.trigger(Inactive {
                        entity: set_state.entity,
                        data: text.0.clone()
                    })
                }
            }
        }
    }
}

pub(crate) fn update_checkbox_style_on_state_change_system(
    checkboxs: Query<
        (&MakaraCheckboxState, &Children),
        (With<MakaraCheckbox>, Or<(Changed<MakaraCheckboxState>, Added<MakaraCheckboxState>)>)
    >,
    mut checkbox_btns: Query<
        (&mut BackgroundColor, &CheckboxButtonActiveColor),
        With<MakaraCheckboxButton>
    >,
) {
    for (state, children) in checkboxs.iter() {
        for child in children {
            if let Ok((mut bg_color, active_color)) = checkbox_btns.get_mut(*child) {
                match state.0 {
                    true => bg_color.0 = active_color.0.clone(),
                    false => bg_color.0 = CHECKBOX_UNCHECKED_COLOR
                }
            }
        }
    }
}

pub(crate) fn update_checkbox_button_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut checkbox_btn_q: Query<&mut BorderColor, With<MakaraCheckboxButton>>,
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

    for mut bd_color in checkbox_btn_q.iter_mut() {
        // only react to theme change if color is default
        if *bd_color == default_light || *bd_color == default_dark {
            *bd_color = BorderColor::all(new_bd_color);
        }
    }
}

pub(crate) fn detect_checkbox_built(
    mut commands: Commands,
    checkboxs: Query<Entity, Added<MakaraCheckbox>>
) {
    for entity in checkboxs.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn can_run_checkbox_systems(q: Query<&MakaraCheckbox>) -> bool {
    q.count() > 0
}
