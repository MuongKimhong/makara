//! Tooltip widget. This widget can only be used as part of other widgets.

use bevy::prelude::*;

use crate::{ContainerStyle, MakaraTheme, Theme, Widget};
use crate::{consts::*, events::*, text::*};

/// Marker component for tooltip.
#[derive(Component)]
pub struct MakaraTooltip;

#[derive(Component, Clone)]
pub struct UseTooltip(pub bool);

#[derive(Component, Clone)]
pub struct TooltipText(pub String);

#[derive(Component, Default, Clone)]
pub enum TooltipPosition {
    #[default]
    Center,
    Left,
    Right,
    Top,
    Bottom
}

#[derive(Bundle, Clone)]
pub struct TooltipBundle {
    pub text: TooltipText,
    pub style: ContainerStyle,
    pub text_bundle: TextBundle,
    pub use_tooltip: UseTooltip,
    pub position: TooltipPosition
}

impl Default for TooltipBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                padding: UiRect::all(px(5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                display: Display::None,
                ..default()
            },
            background_color: BackgroundColor(LIGHT_TOOLTIP_BG_COLOR),
            border_radius: DEFAULT_TOOLTIP_BORDER_RADIUS,
            ..default()
        };

        let text_bundle = TextBundle::default();

        Self {
            style,
            text_bundle,
            text: TooltipText("".to_string()),
            use_tooltip: UseTooltip(false),
            position: TooltipPosition::default()
        }
    }
}

impl Widget for TooltipBundle {
    fn build(self) -> impl Bundle {
        (
            self.style,
            self.use_tooltip,
            self.text,
            self.position,
            MakaraTooltip,
            GlobalZIndex(8),
            children![
                (self.text_bundle, MakaraText)
            ]
        )
    }
}

pub(crate) fn update_tooltip_style_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut q: Query<&mut BackgroundColor, With<MakaraTooltip>>,
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_bg_color = match makara_theme.theme {
        Theme::Light => LIGHT_TOOLTIP_BG_COLOR,
        Theme::Dark => DARK_TOOLTIP_BG_COLOR
    };

    for mut bg_color in q.iter_mut() {
        // only react to theme change if color is default
        if bg_color.0 == LIGHT_TOOLTIP_BG_COLOR || bg_color.0 == DARK_TOOLTIP_BG_COLOR {
            bg_color.0 = new_bg_color;
        }
    }
}

pub(crate) fn detect_tooltip_built(
    mut commands: Commands,
    q: Query<Entity, Added<MakaraTooltip>>
) {
    for entity in q.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn show_or_hide_tooltip(
    show: bool,
    tooltip_q: &mut Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    parent_computed: Option<&ComputedNode>,
    parent_transform: Option<&UiTransform>,
    children: &Children // all child entities of tooltip's parent
) {
    for child in children {
        if let Ok((mut node, child_computed, position, use_tooltip)) = tooltip_q.get_mut(*child) {
            if !use_tooltip.0 {
                break;
            }

            if !show {
                node.display = Display::None;
                break;
            }

            if parent_transform.is_none() && parent_computed.is_none() {
                break;
            }

            let computed = parent_computed.unwrap();
            let transform = parent_transform.unwrap();

            let (Val::Px(trans_x), Val::Px(trans_y)) = (transform.translation.x, transform.translation.y)
            else {
                break;
            };

            if child_computed.size.x > 0.0 && child_computed.size.y > 0.0 {
                let width_diff = (child_computed.size.x * child_computed.inverse_scale_factor) -
                                 (computed.size.x * computed.inverse_scale_factor);

                let height_diff = (child_computed.size.y * child_computed.inverse_scale_factor) -
                                  (computed.size.y * computed.inverse_scale_factor);

                let offset = 5.0;

                match *position {
                    TooltipPosition::Center => {
                        node.left = px(trans_x + ((computed.size.x * computed.inverse_scale_factor) / 2.0));
                        node.top = px(trans_y + ((computed.size.y * computed.inverse_scale_factor) / 2.0));
                    }
                    TooltipPosition::Bottom => {
                        node.top = px(trans_y + (computed.size.y * computed.inverse_scale_factor) + offset);
                        node.left = px(trans_x - (width_diff / 2.0));
                    }
                    TooltipPosition::Top => {
                        node.top = px(
                            trans_y -
                            (child_computed.size.y * child_computed.inverse_scale_factor) -
                            offset
                        );
                        node.left = px(trans_x - (width_diff / 2.0));
                    }
                    TooltipPosition::Left => {
                         node.top = px(trans_y - (height_diff / 2.0));
                         node.left = px(
                             trans_x -
                             (child_computed.size.x * child_computed.inverse_scale_factor) -
                             offset
                         );
                    }
                    TooltipPosition::Right => {
                        node.top = px(trans_y - (height_diff / 2.0));
                        node.left = px(trans_x + (computed.size.x * computed.inverse_scale_factor) + offset);
                    }
                }
            }
            node.display = Display::default();
            break;
        }
    }
}
