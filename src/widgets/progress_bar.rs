use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::{prelude::*, ui_widgets::observe};

use crate::{ContainerStyle, MakaraWidget, MakaraTheme, SetToolTip, SetContainerStyle, Theme, Widget};
use crate::{consts::*, events::*, on_mouse_out, utils::*};
use super::*;

/// UI material for `progress_bar`.
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    u_time: Vec4,
    #[uniform(1)]
    u_color: Vec4,
    #[uniform(2)]
    u_blend: Vec4,
    #[uniform(3)]
    u_size: Vec4,
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/progress_bar.wgsl").into()
    }
}

/// Marker component for `progress_bar`.
#[derive(Component)]
pub struct MakaraProgressBar;

/// Marker component for inner bar.
#[derive(Component)]
pub struct MakaraProgressValue;

/// Type of `progress_bar`.
#[derive(Component)]
pub enum ProgressbarType {
    Indeterminate,
    Percent(f32),
}

/// Component used to store color of inner bar.
#[derive(Component)]
pub struct ProgressValueColor(pub Color);

/// A struct used to mutate components attached to `progress_bar` widget.
pub struct ProgressBarWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub color: &'a mut ProgressValueColor,
    pub commands: &'a mut Commands<'w, 's>
}

impl<'a, 'w, 's> ProgressBarWidget<'a, 'w, 's> {
    /// Turn `progress_bar` into indeterminate mode.
    pub fn set_indeterminate(&mut self) {
        self.commands.trigger(SetProgressBarValue {
            entity: self.entity,
            bar_type: "indeterminate".to_string(),
            value: 0.0
        });
    }

    /// Turn `progress_bar` into percentage mode.
    pub fn set_percentage(&mut self, percent: f32) {
        self.commands.trigger(SetProgressBarValue {
            entity: self.entity,
            bar_type: "percentage".to_string(),
            value: percent
        });
    }
}

/// `progress_bar` system param.
#[derive(SystemParam)]
pub struct ProgressBarQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraProgressBar>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), With<MakaraProgressBar>>,
    pub style: StyleQuery<'w, 's, With<MakaraProgressBar>>,
    pub value_color: Query<'w, 's, &'static mut ProgressValueColor>,
    pub children: Query<'w, 's, &'static Children>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for ProgressBarQuery<'w, 's> {
    type WidgetView<'a> = ProgressBarWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let ProgressBarQuery { id: _, class, style, value_color, children, commands } = self;
        let children_list = children.get(entity).ok()?;

        for child in children_list {
            if value_color.get_mut(*child).is_err() {
                continue;
            }
            let style_bundle = style.query.get_mut(entity).ok()?;
            let (node, bg, border_color, border_radius, shadow, z_index) = style_bundle;

            return Some(ProgressBarWidget {
                entity,
                class: class.get_mut(entity).ok()?.1.into_inner(),
                style: WidgetStyle {
                    node: node.into_inner(),
                    background_color: bg.into_inner(),
                    border_color: border_color.into_inner(),
                    border_radius: border_radius.into_inner(),
                    shadow: shadow.into_inner(),
                    z_index: z_index.into_inner(),
                },
                color: value_color.get_mut(*child).unwrap().into_inner(),
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

/// Bundle for creating `progress_bar`.
#[derive(Bundle)]
pub struct ProgressBarBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub value_style: ContainerStyle,
    pub bar_type: ProgressbarType,
    pub value_color: ProgressValueColor,
    pub tooltip_bundle: TooltipBundle
}

impl Default for ProgressBarBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: px(100),
                height: px(10),
                padding: UiRect::all(px(0)),
                ..default()
            },
            border_radius: BorderRadius::MAX,
            background_color: BackgroundColor(LIGHT_PROGRESS_BAR_BG_COLOR),
            shadow: BoxShadow::default(),
            ..default()
        };

        let value_style = ContainerStyle {
            node: Node {
                width: percent(100.0),
                height: percent(100.0),
                border: UiRect::all(px(0.0)),
                margin: UiRect::all(px(0.0)),
                ..default()
            },
            border_radius: BorderRadius::MAX,
            shadow: BoxShadow::default(),
            ..default()
        };

        let bar_type = ProgressbarType::Indeterminate;
        let value_color = ProgressValueColor(DEFAULT_PROGRESS_VALUE_COLOR);
        let tooltip_bundle = TooltipBundle::default();

        Self {
            id_class: IdAndClass::default(),
            style,
            value_style,
            bar_type,
            value_color,
            tooltip_bundle
        }
    }
}

impl ProgressBarBundle {
    /// Set `progress_bar` to indeterminate mode.
    pub fn as_indeterminate(mut self) -> Self {
        self.bar_type = ProgressbarType::Indeterminate;
        self
    }

    /// Set `progress_bar` to percentage mode.
    pub fn as_percentage(mut self, percent_value: f32) -> Self {
        self.bar_type = ProgressbarType::Percent(percent_value.clamp(0.0, 100.0));
        self.value_style.node.width = percent(percent_value);
        self
    }

    /// Set progress color. Default color will be used if this method is not called.
    pub fn color(mut self, color: Color) -> Self {
        self.value_color.0 = color;
        self
    }
}

impl Widget for ProgressBarBundle {
    /// Build `progress_bar`.
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            MakaraProgressBar,
            MakaraWidget,
            children![
                (
                    self.value_style,
                    self.value_color,
                    self.bar_type,
                    MakaraProgressValue,
                ),
                self.tooltip_bundle.build()
            ],
            observe(on_progress_bar_value_set),
            observe(on_progress_bar_mouse_over),
            observe(on_mouse_out)
        )
    }
}

impl SetContainerStyle for ProgressBarBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for ProgressBarBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for ProgressBarBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default progress bar (light variant) as default theme is light.
pub fn progress_bar() -> ProgressBarBundle {
    let bundle = ProgressBarBundle::default();
    bundle
}

fn on_progress_bar_value_set(
    value_set: On<SetProgressBarValue>,
    bars: Query<&Children, With<MakaraProgressBar>>,
    mut values: Query<(&mut Node, &mut ProgressbarType), With<MakaraProgressValue>>,
    mut commands: Commands
) {
    if let Ok(children) = bars.get(value_set.entity) {
        for child in children {
            if let Ok((mut node, mut bar_type)) = values.get_mut(*child) {
                match value_set.bar_type.as_str() {
                    "indeterminate" => {
                        *bar_type = ProgressbarType::Indeterminate;
                        node.width = percent(100.0);
                    }
                    "percentage" => {
                        *bar_type = ProgressbarType::Percent(value_set.value.clamp(0.0, 100.0));
                        node.width = percent(value_set.value);
                            commands.trigger(Change {
                                entity: value_set.entity,
                                data: value_set.value
                            });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn on_progress_bar_mouse_over(
    mut over: On<Pointer<Over>>,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    bars: Query<
        (&Children, &UiTransform, &ComputedNode),
        With<MakaraProgressBar>
    >,
) {
    if let Ok((children, transform, computed)) = bars.get(over.entity) {
        show_or_hide_tooltip(true, &mut tooltips, Some(computed), Some(transform), children);
    }
    commands.trigger(MouseOver {
        entity: over.entity,
    });
    over.propagate(false);
}

pub(crate) fn update_progress_bar_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut bars: Query<&mut BackgroundColor, With<MakaraProgressBar>>,
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_bg_color = match makara_theme.theme {
        Theme::Light => LIGHT_PROGRESS_BAR_BG_COLOR,
        Theme::Dark => DARK_PROGRESS_BAR_BG_COLOR,
    };

    for mut bg_color in bars.iter_mut() {
        // only react to theme change if color is default
        if bg_color.0 == LIGHT_PROGRESS_BAR_BG_COLOR || bg_color.0 == DARK_PROGRESS_BAR_BG_COLOR {
            bg_color.0 = new_bg_color;
        }
    }
}

pub(crate) fn detect_progress_value_added(
    mut commands: Commands,
    mut progress_materials: ResMut<Assets<ProgressBarMaterial>>,
    bars: Query<&ComputedNode, With<MakaraProgressBar>>,
    values: Query<
        (Entity, &ProgressbarType, &ProgressValueColor, &ChildOf),
        Or<(Added<MakaraProgressValue>, Changed<ComputedNode>)>,
    >,
) {
    for (entity, bar_type, value_color, parent) in values.iter() {
        if let Ok(bar_computed) = bars.get(parent.0) {
            if let Color::Srgba(value) = value_color.0 {
                let blend: f32 = match bar_type {
                    ProgressbarType::Indeterminate => 1.0,
                    ProgressbarType::Percent(_) => 0.0,
                };

                commands
                    .entity(entity)
                    .insert(MaterialNode(progress_materials.add(ProgressBarMaterial {
                        u_time: Vec4::ZERO,
                        u_color: Vec4::new(value.red, value.green, value.blue, 0.0),
                        u_blend: Vec4::new(blend, 0.0, 0.0, 0.0),
                        u_size: Vec4::new(bar_computed.size.x, bar_computed.size.y, 0.0, 0.0),
                    })));
            }
        }
    }
}

pub(crate) fn update_progress_bar_material_u_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
    query: Query<(
        &MaterialNode<ProgressBarMaterial>,
        &ProgressValueColor,
        &ProgressbarType
    )>,
) {
    query.iter().for_each(|(handle, value_color, bar_type)| {
        if let Some(material) = materials.get_mut(handle) {
            match bar_type {
                ProgressbarType::Indeterminate => {
                    material.u_time.x = -time.elapsed_secs();
                    material.u_blend.x = 1.0;
                }
                ProgressbarType::Percent(_percent) => {
                    material.u_time.x = 0.0;
                    material.u_blend.x = 0.0;
                }
            }
            if let Color::Srgba(value) = value_color.0 {
                material.u_color = Vec4::new(value.red, value.green, value.blue, 0.0);
            }
        }
    });
}

pub(crate) fn detect_progress_bar_built(
    mut commands: Commands,
    q: Query<Entity, Added<MakaraProgressBar>>,
) {
    for entity in q.iter() {
        commands.trigger(WidgetBuilt { entity });
    }
}

pub(crate) fn can_run_progress_bar_systems(q: Query<&MakaraProgressBar>) -> bool {
    q.count() > 0
}
