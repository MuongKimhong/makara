use bevy::prelude::*;
use bevy::platform::collections::HashMap;
use bevy::ui::FocusPolicy;

use crate::{ContainerStyle, SetContainerStyle, MakaraTheme, Theme};
use crate::{events::*, consts::*};
use super::*;

/// Resource used to store states, show and hide modals.
#[derive(Resource, Debug, Default)]
pub struct MakaraModalState {
    pub states: HashMap<String, bool>,
    pub state_changed_id: Option<String>,
    pub state_changed: bool,
}

impl MakaraModalState {
    /// Show modal using its id.
    pub fn show(&mut self, id: &str) {
        self.states.insert(id.to_string(), true);
        self.state_changed_id = Some(id.to_string());
        self.state_changed = true;
    }

    /// Hide modal using its id.
    pub fn hide(&mut self, id: &str) {
        self.states.insert(id.to_string(), false);
        self.state_changed_id = Some(id.to_string());
        self.state_changed = true;
    }

    pub(crate) fn reset(&mut self) {
        self.state_changed_id = None;
        self.state_changed = false;
    }

    /// Get state of modal.
    ///
    /// Return `false` if provided modal id doesn't exist.
    pub fn get_state(&self, id: &str) -> bool{
        if let Some(state) = self.states.get(id) {
            return *state;
        }

        false
    }
}

/// Position of modals.
#[derive(Debug, Default)]
pub enum ModalPosition {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    #[default]
    Center
}

/// Marker component for modal backdrop.
#[derive(Component)]
pub struct MakaraModalBackdrop;

#[derive(Component)]
pub struct BackdropNeedsChildrenTransfer;

/// Marker component for modal.
#[derive(Component, Default)]
pub struct MakaraModal;

/// Component used to keep track of modal animation progress.
/// `0.0` mean fully hidden, `1.0` mean fully visible.
#[derive(Component, Default, Clone)]
pub struct ModalAnimationProgress(pub f32);

/// Component used to store modal animation speed.
#[derive(Component, Default, Clone)]
pub struct ModalScaleAnimationSpeed(pub f32);

/// Component used to define if modal has scale animation or not.
#[derive(Component, Default, Clone)]
pub struct ModalHasScaleAnimation(pub bool);

#[derive(Bundle, Default, Clone)]
pub struct ModalAnimation {
    pub has_scale_animation: ModalHasScaleAnimation,
    pub scale_animation_speed: ModalScaleAnimationSpeed,
    pub animation_progress: ModalAnimationProgress
}

/// Temperary component to hold style of modal.
#[derive(Component)]
pub(crate) struct TempModalStyle(pub ContainerStyle);

#[derive(Component)]
pub(crate) struct TempModalAnimation(pub ModalAnimation);

#[derive(Component)]
pub(crate) struct TemModalIdAndClass(pub IdAndClass);

/// A struct used to mutate components attached to `modal` widget.
pub struct ModalWidget<'a> {
    pub id: &'a Id,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub backdrop_style: WidgetStyle<'a>,
    pub(crate) scale_animation: &'a mut ModalHasScaleAnimation,
    pub(crate) scale_animation_speed: &'a mut ModalScaleAnimationSpeed,
    pub(crate) modal_state: &'a mut MakaraModalState
}

impl<'a> ModalWidget<'a> {
    /// Show modal.
    pub fn show(&mut self) {
        self.modal_state.show(&self.id.0);
    }

    /// Hide modal.
    pub fn hide(&mut self) {
        self.modal_state.hide(&self.id.0);
    }

    /// Define if modal has animation or not.
    pub fn scale_animation(&mut self, has_animation: bool) {
        self.scale_animation.0 = has_animation;
    }

    /// Set modal animation speed.
    pub fn animation_speed(&mut self, speed: f32) {
        self.scale_animation_speed.0 = speed;
    }
}

/// `modal` system param.
#[derive(SystemParam)]
pub struct ModalQuery<'w, 's> {
    pub modal_related: Query<
        'w, 's,
        (
            Entity,
            &'static Id,
            &'static ChildOf,
            &'static mut Class,
            &'static mut ModalScaleAnimationSpeed,
            &'static mut ModalHasScaleAnimation
        ),
    >,
    pub style: StyleQuery<'w, 's, With<MakaraModal>>,
    pub backdrop_style: StyleQuery<'w, 's,
        (With<MakaraModalBackdrop>, Without<MakaraModal>)
    >,
    pub modal_state: ResMut<'w, MakaraModalState>
}

impl<'w, 's> WidgetQuery<'w, 's> for ModalQuery<'w, 's> {
     type WidgetView<'a> = ModalWidget<'a> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let ModalQuery { modal_related, style, backdrop_style, modal_state } = self;

        let modal_related_bundle = modal_related.get_mut(entity).ok()?;
        let (_, id, parent, class, animation_speed, has_animation) = modal_related_bundle;

        let backdrop_bundle = backdrop_style.query.get_mut(parent.0).ok()?;
        let (b_node, b_bg, b_border, b_radius, b_shadow, b_z) = backdrop_bundle;

        let style_bundle = style.query.get_mut(entity).ok()?;
        let (node, bg, border, radius, shadow, z) = style_bundle;

        return Some(ModalWidget {
            id: id,
            class: class.into_inner(),
            style: WidgetStyle {
                node: node.into_inner(),
                background_color: bg.into_inner(),
                border_color: border.into_inner(),
                border_radius: radius.into_inner(),
                shadow: shadow.into_inner(),
                z_index: z.into_inner(),
            },
            backdrop_style: WidgetStyle {
                node: b_node.into_inner(),
                background_color: b_bg.into_inner(),
                border_color: b_border.into_inner(),
                border_radius: b_radius.into_inner(),
                shadow: b_shadow.into_inner(),
                z_index: b_z.into_inner(),
            },
            scale_animation: has_animation.into_inner(),
            scale_animation_speed: animation_speed.into_inner(),
            modal_state: modal_state
        });
    }

    fn find_by_id<'a>(&'a mut self, target_id: &str) -> Option<Self::WidgetView<'a>> {
        let entity = self.modal_related.iter()
            .find(|(_, id, _, _, _, _)| id.0 == target_id)
            .map(|(e, _, _, _, _, _)| e)?;

        self.get_components(entity)
    }

    fn find_by_entity<'a>(&'a mut self, target_entity: Entity) -> Option<Self::WidgetView<'a>> {
        self.get_components(target_entity)
    }

    fn find_by_class(&self, target_class: &str) -> Vec<Entity> {
        self.modal_related.iter()
            .filter(|(_, _, _, class, _, _)| class.0.split(" ").any(|word| word == target_class))
            .map(|(e, _, _, _, _, _)| e)
            .collect()
    }
}

/// Bundle for creating `modal`.
#[derive(Bundle)]
pub struct ModalBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub backdrop_style: ContainerStyle,
    pub animation: ModalAnimation
}

impl Default for ModalBundle {
    fn default() -> Self {
        let modal_style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                ..default()
            },
            background_color: BackgroundColor(LIGHT_THEME_BG_COLOR),
            shadow: BoxShadow::default(),
            ..default()
        };

        let backdrop_style = ContainerStyle {
            node: Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                display: Display::None,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            shadow: BoxShadow::default(),
            ..default()
        };

        let animation = ModalAnimation {
            has_scale_animation: ModalHasScaleAnimation(true),
            scale_animation_speed: ModalScaleAnimationSpeed(10.0),
            ..default()
        };


        Self {
            id_class: IdAndClass::default(),
            style: modal_style,
            backdrop_style: backdrop_style,
            animation
        }
    }
}

impl ModalBundle {
    /// Build `modal`.
    pub fn build(self) -> impl Bundle {
        (
            self.backdrop_style,
            FocusPolicy::Block,
            MakaraModalBackdrop,
            BackdropNeedsChildrenTransfer,
            GlobalZIndex(10),
            TempModalAnimation(self.animation),
            TemModalIdAndClass(self.id_class),
            TempModalStyle(self.style)
        )
    }

    /// Set color of backdrop.
    ///
    /// Use Color::None for no color backdrop.
    pub fn backdrop_color(mut self, color: Color) -> Self {
        self.backdrop_style.background_color.0 = color;
        self
    }

    /// Whether to use scale animation or not.
    pub fn scale_animation(mut self, has_animation: bool) -> Self {
        self.animation.has_scale_animation = ModalHasScaleAnimation(has_animation);
        self
    }

    /// Set speed for the scale animation. Default is `10.0`.
    pub fn scale_animation_speed(mut self, speed: f32) -> Self {
        self.animation.scale_animation_speed = ModalScaleAnimationSpeed(speed);
        self
    }

    /// Set position of modal.
    pub fn position(mut self, pos: ModalPosition) -> Self {
        match pos {
            ModalPosition::Center => {
                self.backdrop_style.node.justify_content = JustifyContent::Center;
                self.backdrop_style.node.align_items = AlignItems::Center;
            }
            ModalPosition::Top => {
                self.backdrop_style.node.justify_content = JustifyContent::Center;
                self.backdrop_style.node.align_items = AlignItems::Start;
            }
            ModalPosition::TopLeft => {
                self.backdrop_style.node.justify_content = JustifyContent::Start;
                self.backdrop_style.node.align_items = AlignItems::Start;
            }
            ModalPosition::TopRight => {
                self.backdrop_style.node.justify_content = JustifyContent::End;
                self.backdrop_style.node.align_items = AlignItems::Start;
            }
            ModalPosition::Bottom => {
                self.backdrop_style.node.justify_content = JustifyContent::Center;
                self.backdrop_style.node.align_items = AlignItems::End;
            }
            ModalPosition::BottomLeft => {
                self.backdrop_style.node.justify_content = JustifyContent::Start;
                self.backdrop_style.node.align_items = AlignItems::End;
            }
            ModalPosition::BottomRight => {
                self.backdrop_style.node.justify_content = JustifyContent::End;
                self.backdrop_style.node.align_items = AlignItems::End;
            }
            ModalPosition::Left => {
                self.backdrop_style.node.justify_content = JustifyContent::Start;
                self.backdrop_style.node.align_items = AlignItems::Center;
            }
            ModalPosition::Right => {
                self.backdrop_style.node.justify_content = JustifyContent::End;
                self.backdrop_style.node.align_items = AlignItems::Center;
            }
        }
        self
    }
}

impl SetContainerStyle for ModalBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetIdAndClass for ModalBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default modal.
/// `modal` requires id to be provided in order to be able to show or hide.
pub fn modal() -> ModalBundle {
    let modal_bundle = ModalBundle::default();
    modal_bundle
}

pub(crate) fn detect_modal_children_added(
    mut commands: Commands,
    backdrops: Query<
        (Entity, &Children, &TempModalAnimation, &TemModalIdAndClass, &TempModalStyle),
        (With<BackdropNeedsChildrenTransfer>, Added<Children>)
    >,
) {
    for (backdrop_entity, children, animation, id_class, style) in backdrops.iter() {
        let modal = commands.spawn((
            animation.0.clone(),
            id_class.0.clone(),
            style.0.clone(),
            MakaraModal
        ))
        .id();

        let child_entities: Vec<Entity> = children.iter().map(|e| e).collect();

        // transfer backdrop children (content) to the actual modal
        commands.entity(modal).add_children(&child_entities);

        // put modal as child of backdrop
        commands.entity(backdrop_entity).add_child(modal);
        commands.entity(backdrop_entity).remove::<BackdropNeedsChildrenTransfer>();
    }
}

pub(crate) fn handle_show_and_hide_modals_system(
    mut modal_state: ResMut<MakaraModalState>,
    mut modals: Query<(
        &mut ModalAnimationProgress,
        &mut UiTransform,
        &Id,
        &ModalScaleAnimationSpeed,
        &ModalHasScaleAnimation
    )>,
    mut backdrops: Query<(Entity, &mut Node, &Children), With<MakaraModalBackdrop>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if !modal_state.state_changed {
        return;
    }
    for (entity, mut node, children) in backdrops.iter_mut() {
        for child in children {
            if let Ok((
                mut progress,
                mut transform,
                modal_id,
                speed,
                has_animation
            )) = modals.get_mut(*child) {
                if let Some(id) = &modal_state.state_changed_id {
                    if id != &modal_id.0 {
                        continue;
                    }

                    let state = modal_state.states.get(id);

                    if state.is_none() {
                        continue;
                    }

                    let delta = time.delta_secs() * speed.0;
                    let state = state.unwrap();

                    if has_animation.0 {
                        let old_progress = progress.0;

                        if *state {
                            node.display = Display::default();
                            progress.0 = (progress.0 + delta).min(1.0);
                        } else {
                            progress.0 = (progress.0 - delta).max(0.0);
                        }

                        transform.scale = Vec2::splat(progress.0);

                        if old_progress != progress.0 && (progress.0 == 1.0 || progress.0 == 0.0) {
                            modal_state.reset();

                            if progress.0 == 0.0 {
                                node.display = Display::None;
                                commands.trigger(Inactive {
                                    entity,
                                    data: modal_id.0.clone()
                                });
                            }
                            else {
                                commands.trigger(Active {
                                    entity,
                                    data: modal_id.0.clone()
                                });
                            }
                        }
                    }
                    else {
                        match *state {
                            true => {
                                node.display = Display::default();
                                commands.trigger(Active {
                                    entity,
                                    data: modal_id.0.clone()
                                });
                            }
                            false => {
                                node.display = Display::None;
                                commands.trigger(Inactive {
                                    entity,
                                    data: modal_id.0.clone()
                                });
                            }
                        }
                        modal_state.reset();
                    }
                }
            }
        }
    }
}

pub(crate) fn update_modal_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut root_q: Query<&mut BackgroundColor, With<MakaraModal>>,
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_bg_color = match makara_theme.theme {
        Theme::Light => LIGHT_THEME_BG_COLOR,
        Theme::Dark => DARK_THEME_BG_COLOR
    };

    for mut bg_color in root_q.iter_mut() {
        // only react to theme change if color is default
        if bg_color.0 == LIGHT_THEME_BG_COLOR || bg_color.0 == DARK_THEME_BG_COLOR {
            bg_color.0 = new_bg_color;
        }
    }
}

pub(crate) fn detect_modal_built(
    mut commands: Commands,
    q: Query<Entity, Added<MakaraModal>>
) {
    for entity in q.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn can_run_modal_systems(q: Query<&MakaraModalBackdrop>) -> bool {
    q.count() > 0
}
