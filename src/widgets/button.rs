//! `button` widget.

use bevy::{prelude::*, ui::InteractionDisabled, ui_widgets::observe};
use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::{consts::*, events::*, on_mouse_out, utils::*, colors::*};
use super::*;

/// Marker component for `button`.
#[derive(Component)]
pub struct MakaraButton;

#[derive(Component)]
pub struct MakaraButtonText;

/// A struct used to mutate components attached to `button` widget.
pub struct ButtonWidget<'a> {
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub text: ChildText<'a>
}

impl<'a> SetText for ButtonWidget<'a> {
    fn set_text(&mut self, text: &str) {
        self.text.value.0 = text.to_string();
    }
}

type IsButtonOnly = (
    (
        With<MakaraButton>,
        Without<MakaraCheckbox>,
        Without<MakaraCheckboxButton>,
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

type IsButtonTextOnly = (
    With<MakaraButtonText>,
    Without<MakaraCheckboxText>,
);

/// `button` system param.
#[derive(SystemParam)]
pub struct ButtonQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraButton>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), IsButtonOnly>,
    pub style: StyleQuery<'w, 's, IsButtonOnly>,
    pub text: TextQueryAsChild<'w, 's, IsButtonTextOnly>,
    pub children: Query<'w, 's, &'static Children>
}

impl<'w, 's> WidgetQuery<'w, 's> for ButtonQuery<'w, 's> {
    type WidgetView<'a> = ButtonWidget<'a> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let ButtonQuery { id: _, class, style, text, children } = self;
        let children_list = children.get(entity).ok()?;

        for child in children_list {
            if text.query.get_mut(*child).is_err() {
                continue;
            }
            let text_comp = text.query.get_mut(*child).unwrap();
            let (text, text_font, text_layout, text_color) = text_comp;

            let style_bundle = style.query.get_mut(entity).ok()?;
            let (node, bg, border_color, shadow, z_index) = style_bundle;

            return Some(ButtonWidget {
                class: class.get_mut(entity).ok()?.1.into_inner(),
                style: WidgetStyle {
                    node: node.into_inner(),
                    background_color: bg.into_inner(),
                    border_color: border_color.into_inner(),
                    shadow: shadow.into_inner(),
                    z_index: z_index.into_inner(),
                },
                text: ChildText {
                    value: text.into_inner(),
                    font: text_font.into_inner(),
                    layout: text_layout.into_inner(),
                    color: text_color.into_inner(),
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

/// Bundle for creating `button`.
#[derive(Bundle)]
pub struct ButtonBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub text_bundle: TextBundle,
    pub tooltip_bundle: TooltipBundle
}

impl Default for ButtonBundle {
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

        let text_bundle = TextBundle::default();
        let tooltip_bundle = TooltipBundle::default();
        let id_class = IdAndClass::default();

        Self { style, text_bundle, tooltip_bundle, id_class }
    }
}

impl ButtonBundle {
    pub fn new(text: &str) -> Self {
        let mut bundle = ButtonBundle::default();
        bundle.text_bundle.text.0 = text.to_string();
        bundle
    }

    /// Replace text style with provided style.
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_bundle.text_style = style;
        self
    }

    /// Build button as disabled.
    pub fn build_as_disabled(self) -> impl Bundle {
        (self.build(), InteractionDisabled)
    }
}

impl Widget for ButtonBundle {
    /// Build `button`.
    fn build(mut self) -> impl Bundle {
        process_built_in_color(
            &self.id_class.class,
            &mut self.style.background_color.0
        );
        process_text_built_in_color_class(
            &self.id_class.class,
            &mut self.text_bundle.text_style.color.0
        );
        process_built_in_spacing_class(&self.id_class.class, &mut self.style.node);

        (
            self.id_class,
            self.style,
            children![
                (self.text_bundle, MakaraText, MakaraButtonText),
                self.tooltip_bundle.build()
            ],
            WidgetFocus(false),
            MakaraButton,
            MakaraWidget,
            observe(on_mouse_click),
            observe(on_button_mouse_over),
            observe(on_button_mouse_out),
            observe(on_mouse_out)
        )
    }
}

impl SetContainerStyle for ButtonBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for ButtonBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for ButtonBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default button (light variant) as default theme is light.
pub fn button(text: &str) -> ButtonBundle {
    let mut button_bundle = ButtonBundle::default();
    button_bundle.text_bundle.text.0 = text.to_string();
    button_bundle
}

pub(crate) fn detect_button_class_change_for_built_in_color(
    mut btns: Query<
        (&Class, &mut BackgroundColor, &mut Node, &Children),
        (IsButtonOnly, Changed<Class>)
    >,
    mut texts: Query<&mut TextColor>
) {
    for (class, mut bg, mut node, children) in btns.iter_mut() {
        for child in children.iter() {
            if let Ok(mut text_color) = texts.get_mut(child) {
                process_built_in_color(class, &mut bg.0);
                process_text_built_in_color_class(class, &mut text_color.0);
                process_built_in_spacing_class(class, &mut node);
                break;
            }
        }
    }
}

fn on_mouse_click(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut widgets: Query<(Entity, &mut WidgetFocus)>
) {
    update_focus_state_for_widgets_on_click(click.entity, &mut widgets);
    commands.trigger(Clicked { entity: click.entity });
    click.propagate(false);
}

fn on_button_mouse_over(
    mut over: On<Pointer<Over>>,
    mut btns: Query<
        (
            Has<InteractionDisabled>,
            &Children,
            &UiTransform,
            &ComputedNode,
            &Class,
            &mut BackgroundColor
        ),
        With<MakaraButton>
    >,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    if let Ok((is_disabled, children, transform, computed, class, mut bg)) = btns.get_mut(over.entity) {
        let cursor_icon = if is_disabled {
            CursorIcon::System(SystemCursorIcon::Default)
        } else {
            CursorIcon::System(SystemCursorIcon::Pointer)
        };

        commands.entity(*window).insert(cursor_icon);
        show_or_hide_tooltip(true, &mut tooltips, Some(computed), Some(transform), children);
        process_button_built_in_color_class_hover_only(&class, &mut bg);
    }

    commands.trigger(MouseOver { entity: over.entity });
    over.propagate(false);
}

fn on_button_mouse_out(
    mut out: On<Pointer<Out>>,
    mut btns: Query<(&mut BackgroundColor, &Class), With<MakaraButton>>
) {
    if let Ok((mut bg, class)) = btns.get_mut(out.entity) {
        process_built_in_color(&class, &mut bg.0);
    }

    out.propagate(false);
}

pub(crate) fn update_button_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut button_q: Query<&mut BackgroundColor, With<MakaraButton>>,
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_bg_color = match makara_theme.theme {
        Theme::Light => LIGHT_BUTTON_BG_COLOR,
        Theme::Dark => DARK_BUTTON_BG_COLOR
    };

    for mut bg_color in button_q.iter_mut() {
        // only react to theme change if color is default
        if bg_color.0 == LIGHT_BUTTON_BG_COLOR ||
           bg_color.0 == DARK_BUTTON_BG_COLOR
        {
            bg_color.0 = new_bg_color;
        }
    }
}

pub(crate) fn detect_button_built(
    mut commands: Commands,
    buttons: Query<Entity, Added<MakaraButton>>
) {
    for entity in buttons.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn can_run_button_systems(btns: Query<&MakaraButton>) -> bool {
    btns.count() > 0
}
