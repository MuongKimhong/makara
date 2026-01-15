//! A container with flex direction set to row.

use bevy::prelude::*;

use crate::{ContainerStyle, SetContainerStyle, Widget, events::*};
use super::*;

/// Marker component for `row`.
#[derive(Component)]
pub struct MakaraRow;

/// A struct used to mutate components attached to `row` widget.
pub struct RowWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub commands: &'a mut Commands<'w, 's>,
    pub child_entities: Vec<Entity>
}

impl<'a, 'w, 's> WidgetChildren for RowWidget<'a, 'w, 's> {
    fn add_child(&mut self, child_bundle: impl Bundle) {
        let child_entity = self.commands.spawn(child_bundle).id();
        self.commands.entity(self.entity).add_child(child_entity);
    }

    fn add_children(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) {
        let mut child_entities = Vec::new();

        for bundle in bundles {
            let child_entity = self.commands.spawn(bundle).id();
            child_entities.push(child_entity);
        }
        self.commands.entity(self.entity).add_children(&child_entities);
    }

    fn insert_at(
        &mut self,
        index: usize,
        bundles: impl IntoIterator<Item = impl Bundle>
    ) {
        let mut child_entities = Vec::new();

        for bundle in bundles {
            let child_entity = self.commands.spawn(bundle).id();
            child_entities.push(child_entity);
        }
        self.commands
            .entity(self.entity)
            .insert_children(index, &child_entities);
    }

    fn insert_first(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) {
        self.insert_at(0, bundles);
    }

    fn insert_last(&mut self, bundles: impl IntoIterator<Item = impl Bundle>) {
        let last_index = self.child_entities.len();
        self.insert_at(last_index, bundles);
    }

    fn remove_at(&mut self, index: usize) {
        if let Some(entity) = self.child_entities.get(index) {
            self.commands.entity(self.entity).detach_child(*entity);
            self.commands.entity(*entity).despawn();
        }
    }

    fn remove_first(&mut self) {
        self.remove_at(0);
    }

    fn remove_last(&mut self) {
        // if list is empty, does nothing.
        if let Some(last_index) = self.child_entities.len().checked_sub(1) {
            self.remove_at(last_index);
        }
    }
}

/// `row` system param.
#[derive(SystemParam)]
pub struct RowQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraRow>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), With<MakaraRow>>,
    pub style: StyleQuery<'w, 's, With<MakaraRow>>,
    pub children: Query<'w, 's, &'static Children, With<MakaraRow>>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> WidgetQuery<'w, 's> for RowQuery<'w, 's> {
    type WidgetView<'a> = RowWidget<'a, 'w, 's> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let RowQuery { id: _, class, style, children, commands } = self;

        let style_bundle = style.query.get_mut(entity).ok()?;
        let (node, bg, border_color, shadow, z_index) = style_bundle;

        let entities = children.get(entity).ok()?
            .iter()
            .map(|e| e)
            .collect::<Vec<Entity>>();

        return Some(RowWidget {
            entity,
            class: class.get_mut(entity).ok()?.1.into_inner(),
            style: WidgetStyle {
                node: node.into_inner(),
                background_color: bg.into_inner(),
                border_color: border_color.into_inner(),
                shadow: shadow.into_inner(),
                z_index: z_index.into_inner(),
            },
            commands: commands,
            child_entities: entities
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
            .filter(|(_, class)| class.0.split(" ").any(|word| word == target_class))
            .map(|(e, _)| e)
            .collect()
    }
}

/// Bundle for creating `row`.
#[derive(Bundle)]
pub struct RowBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle
}

impl Default for RowBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: percent(100),
                height: auto(),
                flex_direction: FlexDirection::Row,
                display: Display::Flex,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            shadow: BoxShadow::default(),
            ..default()
        };

        Self { style, id_class: IdAndClass::default() }
    }
}

impl Widget for RowBundle {
    /// Build `row`.
    fn build(self) -> impl Bundle {
        (
            self.id_class,
            self.style,
            MakaraRow,
        )
    }
}

impl SetContainerStyle for RowBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetIdAndClass for RowBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

pub(crate) fn detect_row_built(
    mut commands: Commands,
    q: Query<Entity, Added<MakaraRow>>
) {
    for entity in q.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

/// Create row widget.
pub fn row() -> RowBundle {
    RowBundle::default()
}
