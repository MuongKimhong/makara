use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::{prelude::*, ui_widgets::observe};

use crate::{consts::*, events::*, utils::*, on_mouse_out};
use super::*;

/// Material for `circular`.
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct CircularMaterial {
    #[uniform(0)]
    u_color: Vec4,
    #[uniform(1)]
    u_time: Vec4,
    #[uniform(2)]
    u_percent: f32,
    #[uniform(3)]
    u_type: f32,
    #[uniform(4)]
    u_background_color: Vec4
}

impl UiMaterial for CircularMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/circular.wgsl").into()
    }
}

/// Marker component for `circular`.
#[derive(Component)]
pub struct MakaraCircular;

/// This color is used in shader file as background ring.
#[derive(Component)]
pub struct CircularBackgroundColor(pub Color);

/// Type of circular.
#[derive(Component)]
pub enum CircularType {
    /// A non-stop spinning effect.
    Indeterminate,

    /// A ring is circled at provided value. 100% means full ring.
    Percent(f32)
}

/// Component used to store color of `circular`.
#[derive(Component)]
pub struct CircularColor(pub Color);

/// A struct used to mutate components attached to `circular` widget.
pub struct CircularWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub spin_color: &'a mut CircularColor,
    pub arc_color: &'a mut CircularBackgroundColor,
    pub commands: &'a mut Commands<'w, 's>
}

impl<'a, 'w, 's> CircularWidget<'a, 'w, 's> {
    /// Turn `circular` into indeterminate mode.
    pub fn set_indeterminate(&mut self) {
        self.commands.trigger(SetCircularValue {
            entity: self.entity,
            circular_type: "indeterminate".to_string(),
            value: 0.0
        });
    }

    /// Turn `circular` into percentage mode.
    pub fn set_percentage(&mut self, percent: f32) {
        self.commands.trigger(SetCircularValue {
            entity: self.entity,
            circular_type: "percentage".to_string(),
            value: percent
        });
    }
}

type IsCircularOnly = (
    (
        With<MakaraCircular>,
        Without<MakaraCheckbox>,
        Without<MakaraCheckboxButton>,
        Without<MakaraColumn>,
        Without<MakaraRow>,
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

/// `circular` system param.
#[derive(SystemParam)]
pub struct CircularQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraCircular>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), IsCircularOnly>,
    pub style: StyleQuery<'w, 's, IsCircularOnly>,
    pub custom_style: Query<
        'w, 's,
        (&'static mut CircularColor, &'static mut CircularBackgroundColor)
    >,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for CircularQuery<'w, 's> {
    type WidgetView<'a> = CircularWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let CircularQuery { id: _, class, style, custom_style, commands } = self;

        let style_bundle = style.query.get_mut(entity).ok()?;
        let (node, bg, border_color, shadow, z_index) = style_bundle;

        let custom_style = custom_style.get_mut(entity).ok()?;
        let (cir_color, cir_bg_color) = custom_style;

        return Some(CircularWidget {
            entity,
            class: class.get_mut(entity).ok()?.1.into_inner(),
            style: WidgetStyle {
                node: node.into_inner(),
                background_color: bg.into_inner(),
                border_color: border_color.into_inner(),
                shadow: shadow.into_inner(),
                z_index: z_index.into_inner(),
            },
            spin_color: cir_color.into_inner(),
            arc_color: cir_bg_color.into_inner(),
            commands: commands
        });
    }

    fn find_by_id<'a>(&'a mut self, target_id: &str) -> Option<Self::WidgetView<'a>> {
        let entity = self.id.iter()
            .find(|(_, id)| id.0 == target_id)
            .map(|(e, _)| e)?;

        self.get_components(entity)
    }

    fn find_by_entity<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        self.get_components(entity)
    }

    fn find_by_class(&self, target_class: &str) -> Vec<Entity> {
        self.class.iter()
            .filter(|(_, class)| class.0.split(" ").any(|word| word == target_class))
            .map(|(e, _)| e)
            .collect()
    }
}

/// Bundle for creating `circular`.
#[derive(Bundle)]
pub struct CircularBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub circular_type: CircularType,
    pub circular_color: CircularColor,
    pub tooltip_bundle: TooltipBundle
}

impl Default for CircularBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: px(50),
                height: px(50),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                ..default()
            },
            shadow: BoxShadow::default(),
            ..default()
        };

        let circular_type = CircularType::Indeterminate;
        let circular_color = CircularColor(DEFAULT_CIRCULAR_VALUE_COLOR);
        let tooltip_bundle = TooltipBundle::default();
        let id_class = IdAndClass::default();

        Self { style, circular_type, circular_color, tooltip_bundle, id_class }
    }
}

impl CircularBundle {
    /// Set percentage for circular.
    ///
    /// If this method is called after mode, it will override the circular type.
    pub fn percent(mut self, percent: f32) -> Self {
        self.circular_type = CircularType::Percent(percent);
        self
    }

    /// Set mode for circular, either "indeterminate" or "percentage".
    /// Default is "indeterminate".
    pub fn mode(mut self, mode: &str) -> Self {
        if mode.trim() == "indeterminate" {
            self.circular_type = CircularType::Indeterminate
        }
        else {
            self.circular_type = CircularType::Percent(0.0);
        }
        self
    }

    /// Set circular spinning color. Default color will be used if this method is not called.
    pub fn color(mut self, color: impl IntoColor) -> Self {
        self.circular_color.0 = color.into_color();
        self
    }
}

impl Widget for CircularBundle {
    /// Build `circular`.
    fn build(mut self) -> impl Bundle {
        process_built_in_spacing_class(&self.id_class.class, &mut self.style.node);
        process_built_in_color(&self.id_class.class, &mut self.circular_color.0);

        (
            self.id_class,
            self.style,
            self.circular_color,
            self.circular_type,
            MakaraCircular,
            MakaraWidget,
            CircularBackgroundColor(LIGHT_CIRCULAR_BG_COLOR),
            observe(on_circular_value_set),
            observe(on_circular_mouse_over),
            observe(on_mouse_out),
            children![
                self.tooltip_bundle.build()
            ]
        )
    }
}

impl SetContainerStyle for CircularBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for CircularBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for CircularBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default circular (light variant) as default theme is light.
pub fn circular() -> CircularBundle {
    let bundle = CircularBundle::default();
    bundle
}

fn on_circular_value_set(
    value_set: On<SetCircularValue>,
    mut circulars: Query<&mut CircularType>,
    mut commands: Commands
) {
    if let Ok(mut cir_type) = circulars.get_mut(value_set.entity) {
        match value_set.circular_type.as_str() {
            "indeterminate" => *cir_type = CircularType::Indeterminate,
            "percentage" => {
                *cir_type = CircularType::Percent(value_set.value.clamp(0.0, 100.0));
                commands.trigger(Change {
                    entity: value_set.entity,
                    data: value_set.value.clamp(0.0, 100.0)
                });
            }
            _ => {}
        }
    }
}

fn on_circular_mouse_over(
    mut over: On<Pointer<Over>>,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    circulars: Query<
        (&Children, &UiTransform, &ComputedNode),
        With<MakaraCircular>
    >,
) {
    if let Ok((children, transform, computed)) = circulars.get(over.entity) {
        show_or_hide_tooltip(true, &mut tooltips, Some(computed), Some(transform), children);
    }
    commands.trigger(MouseOver {
        entity: over.entity,
    });
    over.propagate(false);
}

pub(crate) fn detect_circular_added(
    mut commands: Commands,
    mut circular_materials: ResMut<Assets<CircularMaterial>>,
    circulars: Query<
        (Entity, &CircularType, &CircularColor, &CircularBackgroundColor),
        Or<(Added<MakaraCircular>, Changed<ComputedNode>)>
    >
) {
    for (entity, circular_type, color, bg_color) in circulars.iter() {
        if let Color::Srgba(value) = color.0 {
            let (u_type, u_percent) = match circular_type {
                CircularType::Indeterminate => (0.0, 0.0),
                CircularType::Percent(percent) =>  (1.0, *percent),
            };

            if let Color::Srgba(bg_value) = bg_color.0 {
                commands
                    .entity(entity)
                    .insert(
                        MaterialNode(circular_materials.add(CircularMaterial {
                            u_time: Vec4::ZERO,
                            u_color: Vec4::new(value.red, value.green, value.blue, 1.0),
                            u_background_color: Vec4::new(
                                bg_value.red,
                                bg_value.green,
                                bg_value.blue,
                                bg_value.alpha
                            ),
                            u_type,
                            u_percent
                        }))
                    );
            }
        }
    }
}

pub(crate) fn update_circular_material_u_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<CircularMaterial>>,
    query: Query<(&MaterialNode<CircularMaterial>, &CircularType, &CircularColor, &CircularBackgroundColor)>
) {
    query.iter().for_each(|(handle, circular_type, color, bg_color)| {
        if let Some(material) = materials.get_mut(handle) {
            match circular_type {
                CircularType::Indeterminate => {
                    material.u_time.x = -time.elapsed_secs();
                    material.u_type = 0.0;
                }
                CircularType::Percent(value) => {
                    material.u_time.x = 0.0;
                    material.u_percent = *value;
                    material.u_type = 1.0;
                }
            }
            if let Color::Srgba(value) = bg_color.0 {
                material.u_background_color = Vec4::new(
                    value.red, value.green, value.blue, value.alpha
                );
            }

            if let Color::Srgba(value) = color.0 {
                material.u_color = Vec4::new(
                    value.red, value.green, value.blue, value.alpha
                );
            }
        }
    });
}

pub(crate) fn update_circular_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut circulars: Query<(&mut CircularBackgroundColor, &CircularType), With<MakaraCircular>>,
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_bg_color = match makara_theme.theme {
        Theme::Light => LIGHT_CIRCULAR_BG_COLOR,
        Theme::Dark => DARK_CIRCULAR_BG_COLOR,
    };

    for (mut bg_color, cir_type) in circulars.iter_mut() {
        match cir_type {
            CircularType::Percent(_) => {
                // only react to theme change if color is default
                if bg_color.0 == LIGHT_CIRCULAR_BG_COLOR ||
                   bg_color.0 == DARK_CIRCULAR_BG_COLOR
                {
                    bg_color.0 = new_bg_color;
                }
            },
            _ => {}
        }

    }
}

pub(crate) fn can_run_circular_systems(q: Query<&MakaraCircular>) -> bool {
    q.count() > 0
}
