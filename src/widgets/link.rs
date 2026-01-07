use bevy::prelude::*;
use bevy::ui_widgets::observe;
use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::utils::update_focus_state_for_widgets_on_click;
use crate::{events::*, consts::*};
use super::*;

/// Marker component for `link`.
#[derive(Component)]
pub struct MakaraLink;

/// A struct used to mutate components attached to `link` widget.
pub struct LinkWidget<'a> {
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub text: ChildText<'a>
}

/// `link` system param.
#[derive(SystemParam)]
pub struct LinkQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraLink>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), With<MakaraLink>>,
    pub style: StyleQuery<'w, 's, With<MakaraLink>>,
    pub text: TextQueryAsChild<'w, 's>,
    pub children: Query<'w, 's, &'static Children>
}

impl<'w, 's> WidgetQuery<'w, 's> for LinkQuery<'w, 's> {
    type WidgetView<'a> = LinkWidget<'a> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let LinkQuery { id: _, class, style, text, children } = self;
        let children_list = children.get(entity).ok()?;

        for child in children_list {
            if text.query.get_mut(*child).is_err() {
                continue;
            }
            let text_comp = text.query.get_mut(*child).unwrap();
            let (text, text_font, text_layout, text_color) = text_comp;

            let style_bundle = style.query.get_mut(entity).ok()?;
            let (node, bg, border_color, border_radius, shadow, z_index) = style_bundle;

            return Some(LinkWidget {
                class: class.get_mut(entity).ok()?.1.into_inner(),
                style: WidgetStyle {
                    node: node.into_inner(),
                    background_color: bg.into_inner(),
                    border_color: border_color.into_inner(),
                    border_radius: border_radius.into_inner(),
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
            .filter(|(_, class)| class.0 == target_class)
            .map(|(e, _)| e)
            .collect()
    }
}

/// Bundle for creating `link`.
#[derive(Bundle)]
pub struct LinkBundle {
    pub id_class: IdAndClass,
    pub text_bundle: TextBundle,
    pub style: ContainerStyle,
    pub tooltip_bundle: TooltipBundle
}

impl Default for LinkBundle {
    fn default() -> Self {
        let mut text_bundle = TextBundle::default();
        text_bundle.text_style.color.0 = DEFAULT_LINK_COLOR;

        let style = ContainerStyle {
            shadow: BoxShadow::default(),
            border_color: BorderColor {
                bottom: DEFAULT_LINK_COLOR,
                ..default()
            },
            node: Node {
                border: UiRect::bottom(px(1)),
                padding: UiRect::bottom(px(-2)),
                ..default()
            },
            ..default()
        };
        let tooltip_bundle = TooltipBundle::default();
        let id_class = IdAndClass::default();

        Self { text_bundle, style, tooltip_bundle, id_class }
    }
}

impl LinkBundle {
    /// Replace text style with provided style.
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_bundle.text_style = style;
        self
    }

    /// Remove underline from `link`.
    pub fn no_underline(mut self) -> Self {
        self.style.border_color.bottom = Color::NONE;
        self
    }

    /// Set color for both text and underline.
    /// If `no_underline` is called before this method, provided color
    /// won't effect underline.
    pub fn color(mut self, color: Color) -> Self {
        self.text_bundle.text_style.color.0 = color;

        if self.style.border_color.bottom != Color::NONE {
            self.style.border_color.bottom = color;
        }
        self
    }
}

impl Widget for LinkBundle {
    /// Build `link`.
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            MakaraLink,
            MakaraWidget,
            WidgetFocus(false),
            children![self.text_bundle, self.tooltip_bundle.build()],
            observe(on_link_click),
            observe(on_link_mouse_over),
            observe(on_mouse_out)
        )
    }
}

impl SetContainerStyle for LinkBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for LinkBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for LinkBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default text (light variant) as default theme is light.
pub fn link(text: &str) -> LinkBundle {
    let mut bundle = LinkBundle::default();
    bundle.text_bundle.text.0 = text.to_string();
    bundle
}

fn on_link_click(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut widgets: Query<(Entity, &mut WidgetFocus)>,
    links: Query<&Children, With<MakaraLink>>,
    link_texts: Query<&Text>
) {
    for children in links.iter() {
        for child in children {
            if let Ok(text) = link_texts.get(*child) {
                let _ = open::that(&text.0);
                break;
            }
        }
    }

    update_focus_state_for_widgets_on_click(click.entity, &mut widgets);
    commands.trigger(Clicked { entity: click.entity });
    click.propagate(false);
}

fn on_link_mouse_over(
    mut over: On<Pointer<Over>>,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    links: Query<
        (&Children, &UiTransform, &ComputedNode),
        With<MakaraLink>
    >,
    window: Single<Entity, With<Window>>,
) {
    if let Ok((children, transform, computed)) = links.get(over.entity) {
        show_or_hide_tooltip(true, &mut tooltips, Some(computed), Some(transform), children);
    }

    let cursor_icon = CursorIcon::System(SystemCursorIcon::Pointer);
    commands.entity(*window).insert(cursor_icon);
    commands.trigger(MouseOver { entity: over.entity });
    over.propagate(false);
}

pub(crate) fn detect_link_built(
    mut commands: Commands,
    q: Query<Entity, Added<MakaraLink>>
) {
    for entity in q.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}
