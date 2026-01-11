use bevy::prelude::*;

use crate::{ContainerStyle, SetContainerStyle, MakaraTheme, Theme, Widget};
use crate::{events::*, consts::*};
use super::*;

/// Marker component for text.
#[derive(Component)]
pub struct MakaraText;

/// Text style components.
#[derive(Bundle, Default, Clone)]
pub struct TextStyle {
    pub color: TextColor,
    pub layout: TextLayout,
    pub font: TextFont
}

impl TextStyle {
    /// Create text style from provided color with other default options.
    pub fn from_color(color: Color) -> Self {
        Self {
            color: TextColor(color),
            ..default()
        }
    }

    /// Create text style from provided font size with
    /// other default options.
    pub fn from_font_size(size: f32) -> Self {
        Self {
            font: TextFont::from_font_size(size),
            ..default()
        }
    }
}

/// A struct used to mutate components related to `text`.
pub struct TextWidget<'a> {
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub text: ChildText<'a>
}


#[derive(SystemParam)]
pub struct TextQueryAsChild<'w, 's,  F: QueryFilter + 'static = ()> {
    pub query: Query<'w, 's,
        (
            &'static mut Text,
            &'static mut TextFont,
            &'static mut TextLayout,
            &'static mut TextColor
        ),
        F
    >
}

/// `text` system param.
#[derive(SystemParam)]
pub struct TextQuery<'w, 's> {
    pub id: Query<'w, 's, (Entity, &'static Id), With<MakaraText>>,
    pub class: Query<'w, 's, (Entity, &'static mut Class), With<MakaraText>>,
    pub style: StyleQuery<'w, 's, With<MakaraText>>,
    pub text_related: Query<'w, 's, (
        &'static mut Text,
        &'static mut TextColor,
        &'static mut TextFont,
        &'static mut TextLayout,
    )>
}

impl<'w, 's> WidgetQuery<'w, 's> for TextQuery<'w, 's> {
    type WidgetView<'a> = TextWidget<'a> where Self: 'a;

    fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let TextQuery { id: _, class, style, text_related } = self;
        let style_bundle = style.query.get_mut(entity).ok()?;
        let (node, bg, border_color, border_radius, shadow, z_index) = style_bundle;
        let (text, color, font, layout) = text_related.get_mut(entity).ok()?;

        return Some(TextWidget {
            class: class.get_mut(entity).ok()?.1.into_inner(),
            text: ChildText {
                value: text.into_inner(),
                layout: layout.into_inner(),
                color: color.into_inner(),
                font: font.into_inner(),
            },
            style: WidgetStyle {
                node: node.into_inner(),
                background_color: bg.into_inner(),
                border_color: border_color.into_inner(),
                border_radius: border_radius.into_inner(),
                shadow: shadow.into_inner(),
                z_index: z_index.into_inner(),
            },
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

/// Bundle for creating text.
#[derive(Bundle, Clone)]
pub struct TextBundle {
    pub id_class: IdAndClass,
    pub text: Text,
    pub text_style: TextStyle,
    pub style: ContainerStyle
}

impl Default for TextBundle {
    fn default() -> Self {
        TextBundle {
            id_class: IdAndClass::default(),
            text: Text::new(""),
            text_style: TextStyle {
                color: TextColor(LIGHT_THEME_TEXT_COLOR),
                font: TextFont::from_font_size(DEFAULT_TEXT_FONT_SIZE),
                ..default()
            },
            style: ContainerStyle {
                shadow: BoxShadow::default(),
                ..default()
            }
        }
    }
}

impl TextBundle {
    /// Set text color.
    pub fn color(mut self, color: Color) -> Self {
        self.text_style.color.0 = color;
        self
    }

    /// Set text font size.
    pub fn font_size(mut self, size: f32) -> Self {
        self.text_style.font.font_size = size;
        self
    }

    /// Replace text style with provided style.
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self
    }
}

impl Widget for TextBundle {
    /// Build text.
    fn build(self) -> impl Bundle {
        (self.id_class, self.text_style, self.text, self.style, MakaraText)
    }
}

impl SetContainerStyle for TextBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetIdAndClass for TextBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

/// Create default text (light variant) as default theme is light.
pub fn text(text: &str) -> TextBundle {
    let mut text_bundle = TextBundle::default();
    text_bundle.text.0 = text.to_string();
    text_bundle
}

pub(crate) fn update_text_color_on_theme_change_system(
    makara_theme: Res<MakaraTheme>,
    mut text_q: Query<&mut TextColor, With<MakaraText>>
) {
    if !makara_theme.is_changed() {
        return;
    }

    let new_text_color = match makara_theme.theme {
        Theme::Light => LIGHT_THEME_TEXT_COLOR,
        Theme::Dark => DARK_THEME_TEXT_COLOR
    };

    for mut text_color in text_q.iter_mut() {
        if text_color.0 == LIGHT_THEME_TEXT_COLOR || text_color.0 == DARK_THEME_TEXT_COLOR {
            text_color.0 = new_text_color;
        }
    }
}

pub(crate) fn detect_text_built(
    mut commands: Commands,
    texts: Query<Entity, Added<MakaraText>>
) {
    for entity in texts.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }
}

pub(crate) fn can_run_text_systems(q: Query<&MakaraText>) -> bool {
    q.count() > 0
}
