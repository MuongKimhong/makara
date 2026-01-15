//! `image` widget.

use bevy::prelude::*;
use bevy::ui_widgets::observe;
use bevy::asset::LoadState;
use std::collections::HashMap;

use crate::{events::*};
use super::*;

#[derive(Resource, Debug, Default)]
pub(crate) struct ImageHandleMap {
    pub(crate) maps: HashMap<Entity, Handle<Image>>
}

/// Marker component for `image`.
#[derive(Component)]
pub struct MakaraImage;

#[derive(Component, Default)]
pub struct ImagePath(pub String);

#[derive(Component, Default)]
pub struct ImageLoadStateNeedsCheck(pub bool);

/// A struct used to mutate components attached to `image` widget.
pub struct ImageWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub image_node: &'a mut ImageNode,
    pub image_path: &'a mut ImagePath,
    pub need_check: &'a mut ImageLoadStateNeedsCheck,
    pub asset_server: &'a AssetServer,
    pub(crate) handle_maps: &'a mut ImageHandleMap,
    pub commands: &'a mut Commands<'w, 's>
}

impl<'a, 'w, 's> ImageWidget<'a, 'w, 's> {
    /// Set new path for image
    pub fn set_path(&mut self, new_path: String) {
        let new_handle: Handle<Image> = self.asset_server.load(&new_path);
        self.handle_maps.maps.insert(self.entity, new_handle.clone());
        self.image_node.image = new_handle;
        self.image_path.0 = new_path.to_string();
        self.need_check.0 = true;

        self.commands.trigger(Change {
            entity: self.entity,
            data: new_path.to_string()
        });

        self.commands.trigger(Loading {
            entity: self.entity
        });
    }
}

/// `image` system param.
#[derive(SystemParam)]
pub struct ImageQuery<'w, 's> {
    pub image_related: Query<
        'w, 's,
        (
            Entity,
            &'static Id,
            &'static mut Class,
            &'static mut ImageNode,
            &'static mut ImagePath,
            &'static mut ImageLoadStateNeedsCheck
        ),
        With<MakaraImage>
    >,
    pub style: StyleQuery<'w, 's, With<MakaraImage>>,
    pub asset_server: Res<'w, AssetServer>,
    pub(crate) handle_maps: ResMut<'w, ImageHandleMap>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for ImageQuery<'w, 's> {
    type WidgetView<'a> = ImageWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let ImageQuery { image_related, style, asset_server, handle_maps, commands } = self;

        let (_, _, class, image_node, image_path, need_check) = image_related.get_mut(entity).ok()?;

        let style_bundle = style.query.get_mut(entity).ok()?;
        let (node, bg, border_color, shadow, z_index) = style_bundle;

        return Some(ImageWidget {
            entity,
            class: class.into_inner(),
            style: WidgetStyle {
                node: node.into_inner(),
                background_color: bg.into_inner(),
                border_color: border_color.into_inner(),
                shadow: shadow.into_inner(),
                z_index: z_index.into_inner(),
            },
            need_check: need_check.into_inner(),
            image_node: image_node.into_inner(),
            image_path: image_path.into_inner(),
            asset_server: asset_server,
            handle_maps: handle_maps,
            commands: commands
        });
    }

    fn find_by_id<'a>(&'a mut self, target_id: &str) -> Option<Self::WidgetView<'a>> {
        let entity = self.image_related.iter()
            .find(|(_, id, _, _, _, _)| id.0 == target_id)
            .map(|(e, _, _, _, _, _)| e)?;

        self.get_components(entity)
    }

    fn find_by_entity<'a>(&'a mut self, target_entity: Entity) -> Option<Self::WidgetView<'a>> {
        self.get_components(target_entity)
    }

    fn find_by_class(&self, target_class: &str) -> Vec<Entity> {
        self.image_related.iter()
            .filter(|(_, _, class, _, _, _)| class.0.split(" ").any(|word| word == target_class))
            .map(|(e, _, _, _, _, _)| e)
            .collect()
    }
}

#[derive(Bundle)]
pub struct ImageBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub image_node: ImageNode,
    pub image_path: ImagePath,
    pub tooltip_bundle: TooltipBundle
}

impl Default for ImageBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            shadow: BoxShadow::default(),
            ..default()
        };

        let id_class = IdAndClass::default();
        let image_path = ImagePath::default();
        let tooltip_bundle = TooltipBundle::default();
        Self { id_class, style, image_path, tooltip_bundle, image_node: ImageNode::default() }
    }
}

impl Widget for ImageBundle {
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            self.image_path,
            self.image_node,
            MakaraImage,
            MakaraWidget,
            ImageLoadStateNeedsCheck::default(),
            observe(on_image_mouse_over),
            observe(on_mouse_out),
            observe(on_image_mouse_click)
        )
    }
}

impl SetContainerStyle for ImageBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for ImageBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for ImageBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create image widget. `path` can be http url or local path.
pub fn image(path: &str) -> ImageBundle {
    let mut bundle = ImageBundle::default();
    bundle.image_path.0 = path.to_string();
    bundle
}

fn on_image_mouse_over(
    mut over: On<Pointer<Over>>,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    images: Query<
        (&Children, &UiTransform, &ComputedNode),
        With<MakaraImage>
    >,
) {
    if let Ok((children, transform, computed)) = images.get(over.entity) {
        show_or_hide_tooltip(true, &mut tooltips, Some(computed), Some(transform), children);
    }
    commands.trigger(MouseOver {
        entity: over.entity,
    });
    over.propagate(false);
}

fn on_image_mouse_click(
    mut click: On<Pointer<Click>>,
    mut commands: Commands
) {
    commands.trigger(Clicked {
        entity: click.entity
    });
    click.propagate(false);
}

pub(crate) fn detect_new_image_added(
    mut images: Query<
        (Entity, &mut ImageNode, &mut ImageLoadStateNeedsCheck, &ImagePath),
        Added<MakaraImage>
    >,
    mut commands: Commands,
    mut handle_maps: ResMut<ImageHandleMap>,
    asset_server: Res<AssetServer>
) {
    for (entity, mut image_node, mut need_check, image_path) in images.iter_mut() {
        if image_path.0.is_empty() {
            continue;
        }

        let handle: Handle<Image> = asset_server.load(&image_path.0);
        handle_maps.maps.insert(entity, handle.clone());
        image_node.image = handle;
        commands.trigger(Loading { entity });
        need_check.0 = true;
    }
}

pub(crate) fn track_image_loading_state(
    handle_maps: Res<ImageHandleMap>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut images: Query<(Entity, &mut ImageLoadStateNeedsCheck)>,
) {
    for (entity, mut need_check) in images.iter_mut() {
        if !need_check.0 {
            continue;
        }

        if let Some(handle) = handle_maps.maps.get(&entity) {
            match asset_server.load_state(handle) {
                LoadState::Loaded => {
                    commands.trigger(Loaded { entity });
                    need_check.0 = false;
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn detect_image_built(
    mut commands: Commands,
    images: Query<Entity, Added<MakaraImage>>
) {
    for entity in images.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn can_run_image_systems(q: Query<&MakaraImage>) -> bool {
    q.count() > 0
}
