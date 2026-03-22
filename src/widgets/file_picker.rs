//! `file picker` widget.

use bevy::prelude::*;
use bevy::ui_widgets::observe;
use std::thread;
use crossbeam_channel::{Receiver, unbounded};
use std::path::PathBuf;

use crate::{events::*, utils::*};
use super::*;

/// Result of a file picker widget.
#[derive(Debug)]
pub struct FilePickerResult {
    /// Path of selected file.
    pub path: Option<PathBuf>,
    /// Entity of the widget.
    pub entity: Entity
}

impl FilePickerResult {
    pub fn new(path: Option<PathBuf>, entity: Entity) -> Self {
        Self {
            path,
            entity
        }
    }
}

/// use channel receiver to get the result of picked file path.
#[derive(Resource, Default)]
pub struct FilePickerState {
    pub receiver: Option<Receiver<FilePickerResult>>,
}

/// Marker component for `file_picker`.
#[derive(Component)]
pub struct MakaraFilePicker;

#[derive(Component)]
pub struct MakaraFilePickerButton;

/// Hold entity of main picker, used inside file picker button and display text.
#[derive(Component)]
pub struct MainFilePickerEntity(pub Entity);

#[derive(Component)]
pub struct MakaraFilePickerDisplayText;

/// Bundle for creating file picker.
pub struct FilePickerBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub button_bundle: ButtonBundle,
    pub text_bundle: TextBundle,
}

impl Default for FilePickerBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: auto(),
                height: auto(),
                padding: UiRect::all(px(5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                border_radius: BorderRadius::all(px(5)),
                ..default()
            },
            shadow: BoxShadow::default(),
            ..default()
        };

        let mut text_bundle = TextBundle::default();
        text_bundle.text_style.layout.linebreak = LineBreak::NoWrap;
        text_bundle.text.0 = "No file chosen".to_string();

        let mut button_bundle = ButtonBundle::new("Choose file");
        button_bundle.style.node.margin = UiRect::left(px(5));

        let id_class = IdAndClass::default();

        Self {
            style,
            button_bundle,
            text_bundle,
            id_class
        }
    }
}

impl FilePickerBundle {
    /// Replace text style with provided style.
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_bundle.text_style = style;
        self
    }

    pub fn button_text(mut self, text: &str) -> Self {
        self.button_bundle.text_bundle.text.0 = text.to_string();
        self
    }

    pub fn default_display_text(mut self, text: &str) -> Self {
        self.text_bundle.text.0 = text.to_string();
        self
    }
}

impl Widget for FilePickerBundle {
    fn build(mut self) -> impl Bundle {
        process_text_built_in_color_class(
            &self.id_class.class,
            &mut self.text_bundle.text_style.color.0
        );
        process_built_in_spacing_class(&self.id_class.class, &mut self.style.node);

        (
            self.id_class,
            self.style,
            children![
                (self.text_bundle, MakaraFilePickerDisplayText, MakaraText),
                (
                    self.button_bundle.build(),
                    MakaraFilePickerButton,
                    observe(on_file_picker_button_click)
                )
            ],
            MakaraFilePicker,
        )
    }
}

impl SetContainerStyle for FilePickerBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetIdAndClass for FilePickerBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}

pub fn file_picker() -> FilePickerBundle {
    FilePickerBundle::default()
}

pub(crate) fn detect_file_picker_built(
    mut commands: Commands,
    pickers: Query<Entity, Added<MakaraFilePicker>>,
    picker_children: Query<(Entity, &Children), Added<Children>>
) {
    for entity in pickers.iter() {
        commands.trigger(WidgetBuilt {
            entity
        });
    }

    for (entity, children) in picker_children.iter() {
        for child in children {
            commands.entity(*child).insert(MainFilePickerEntity(entity));
        }
    }
}

fn on_file_picker_button_click(
    mut click: On<Pointer<Click>>,
    mut file_picker_state: ResMut<FilePickerState>,
    picker_buttons: Query<&MainFilePickerEntity, With<MakaraFilePickerButton>>
) {
    click.propagate(false);

    if file_picker_state.receiver.is_some() {
        return;
    }

    if let Ok(picker_entity) = picker_buttons.get(click.entity) {
        let (sender, receiver) = unbounded();
        file_picker_state.receiver = Some(receiver);

        let picker_entity = picker_entity.0.clone();

        thread::spawn(move || {
            let file_path = rfd::FileDialog::new()
                .set_directory("/")
                .pick_file();

            let _ = sender.send(FilePickerResult::new(file_path, picker_entity));
        });
    }
}

fn handle_receive_result_success(
    text_q: &mut Query<(&mut Text, &MainFilePickerEntity), With<MakaraFilePickerDisplayText>>,
    commands: &mut Commands,
    entity: Entity,
    path: &PathBuf
) {
    for (mut text, picker_entity) in text_q.iter_mut() {
        if picker_entity.0 != entity {
            continue;
        }
        let file_name = path.file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string());

        if file_name.is_some() {
            text.0 = file_name.unwrap();
        }
        else {
            text.0 = "Unknown".to_string();
        }

        commands.trigger(Change {
            entity: picker_entity.0,
            data: path.clone().display().to_string()
        });
    }
}

pub fn handle_file_picker_result_received_system(
    mut file_picker_state: ResMut<FilePickerState>,
    mut text_q: Query<(&mut Text, &MainFilePickerEntity), With<MakaraFilePickerDisplayText>>,
    mut commands: Commands
) {
    if let Some(receiver) = &file_picker_state.receiver {
        if let Ok(result) = receiver.try_recv() {
            match result.path {
                Some(path) => handle_receive_result_success(
                    &mut text_q,
                    &mut commands,
                    result.entity,
                    &path
                ),
                None => {}
            }
            file_picker_state.receiver = None;
        }
    }
}
