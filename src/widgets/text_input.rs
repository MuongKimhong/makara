use bevy::{prelude::*, ui::InteractionDisabled, ui_widgets::observe};
use bevy::window::{CursorIcon, SystemCursorIcon};
use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::Extent3d;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::image::ImageSampler;
use cosmic_text::{
    Attrs, Edit, Weight, Family, Selection,
    Buffer, Metrics, Shaping
};
use std::sync::Arc;

use crate::{consts::*, events::*, utils::*, on_mouse_out};
use super::*;

#[cfg(target_arch = "wasm32")]
use js_sys::Promise;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
pub struct WasmPaste {
    pub text: String,
    pub entity: Entity,
}

/// Async channel for receiving from the clipboard in Wasm
#[cfg(target_arch = "wasm32")]
#[derive(Resource, Clone)]
pub struct WasmPasteAsyncChannel {
    pub tx: crossbeam_channel::Sender<WasmPaste>,
    pub rx: crossbeam_channel::Receiver<WasmPaste>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn write_clipboard_wasm(text: &str) {
    let clipboard = web_sys::window().unwrap().navigator().clipboard();
    let _result = clipboard.write_text(text);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn read_clipboard_wasm() -> Promise {
    let clipboard = web_sys::window().unwrap().navigator().clipboard();
    clipboard.read_text()
}

/// Marker component for `text_input`.
#[derive(Component)]
pub struct MakaraTextInput;

/// Marker component for text input editor (cosmic-text).
#[derive(Component)]
pub struct MakaraTextInputEditor;

/// Marker component for cursor (caret).
#[derive(Component)]
pub struct MakaraTextInputCursor;

/// Component for input editor holds entity of main text input.
/// This component is added inside `detect_new_text_input_added` system.
#[derive(Component)]
pub struct MainTextInputEntity(pub Entity);

/// Component for main input holds entity of the editor.
/// This component is added inside `detect_new_text_input_added` system.
#[derive(Component)]
pub struct TextInputEditorEntity(pub Entity);

/// Component for main input and editor hold entity of the cursor.
/// This component is added inside `detect_new_text_input_added` system.
#[derive(Component)]
pub struct TextInputCursorEntity(pub Entity);

#[derive(Component, Debug, Default)]
pub struct TextInputPlaceholder(pub String);

/// Component used to store value of `text_input`.
#[derive(Component, Debug, Default)]
pub struct TextInputValue(pub String);

/// Component used to store `text_input` style related.
/// `text_input` uses cosmic-text which is completely different from
/// Bevy's Text.
#[derive(Component, Debug)]
pub struct TextInputEditorStyle {
    pub font_size: f32,
    pub scale_factor: f32,
    pub selection_color: Color,
    pub text_color: Color,
    pub placeholder_color: Color
}

impl Default for TextInputEditorStyle {
    fn default() -> Self {
        Self {
            font_size: DEFAULT_TEXT_FONT_SIZE,
            selection_color: SELECTION_COLOR,
            text_color: LIGHT_THEME_TEXT_COLOR,
            placeholder_color: LIGHT_PLACEHOLDER_COLOR,
            scale_factor: 1.0
        }
    }
}

#[derive(Component)]
pub struct CursorBlinkTimer {
    pub timer: Timer,
    pub can_blink: bool
}

impl Default for CursorBlinkTimer {
    fn default() -> Self {
        CursorBlinkTimer {
            timer: Timer::from_seconds(0.55, TimerMode::Repeating),
            can_blink: true
        }
    }
}

/// A struct used to mutate components attached to `text_input` widget.
pub struct TextInputWidget<'a, 'w, 's> {
    pub entity: Entity,
    pub class: &'a mut Class,
    pub style: WidgetStyle<'a>,
    pub caret_style: WidgetStyle<'a>,
    pub editor_style: &'a mut TextInputEditorStyle,
    pub value: &'a mut TextInputValue,
    pub(crate) input_computed: Mut<'a, ComputedNode>,
    pub(crate) commands: &'a mut Commands<'w, 's>,
    pub(crate) ctx: &'a mut MakaraTextEditContext,
    pub(crate) editor: &'a mut TextEditor
}

impl<'a, 'w, 's> TextInputWidget<'a, 'w, 's> {
    pub fn set_value(&mut self, value: &str) {
        self.editor.clear_text_editor(self.ctx);
        self.editor.editor.set_cursor(cosmic_text::Cursor::new(0, 0));
        self.editor.editor.set_selection(cosmic_text::Selection::None);
        self.editor.editor.insert_string(value, None);
        self.editor.editor.shape_as_needed(&mut self.ctx.font_system, true);
        self.value.0 = value.to_string();
        self.editor.editor.set_redraw(true);
        self.commands.trigger(Change {
            entity: self.entity,
            data: value.to_string()
        });
        self.input_computed.set_changed();
    }
}

/// `text_input` system param.
#[derive(SystemParam)]
pub struct TextInputQuery<'w, 's> {
    pub id_class: Query<'w, 's, (Entity, &'static Id, &'static mut Class), With<MakaraTextInput>>,
    pub input_related: Query<'w, 's,  (
        Entity,
        &'static mut ComputedNode,
        &'static TextInputCursorEntity,
        &'static TextInputEditorEntity
    )>,
    pub style: StyleQuery<'w, 's, With<MakaraTextInput>>,
    pub caret_style: StyleQuery<
        'w, 's,
        (With<MakaraTextInputCursor>, Without<MakaraTextInput>)
    >,
    pub editor: Query<'w, 's,
        (&'static mut TextInputEditorStyle, &'static mut TextEditor, &'static mut TextInputValue)
    >,
    pub ctx: ResMut<'w, MakaraTextEditContext>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> TextInputQuery<'w, 's> {
    pub fn id_match(&mut self, target_id: &str) -> bool {
        self.id_class.iter().any(|(_, id, _)| id.0 == target_id)
    }
}

impl<'w, 's> WidgetQuery<'w, 's> for TextInputQuery<'w, 's> {
     type WidgetView<'a> = TextInputWidget<'a, 'w, 's> where Self: 'a;

     fn get_components<'a>(&'a mut self, entity: Entity) -> Option<Self::WidgetView<'a>> {
        let TextInputQuery {
            id_class,
            input_related,
            style,
            caret_style,
            editor,
            ctx,
            commands
        } = self;

        let (_, _, class) = id_class.get_mut(entity).ok()?;

        let input_related_bundle = input_related.get_mut(entity).ok()?;
        let (_, computed, cursor_entity, editor_entity) = input_related_bundle;

        let (node, bg, border, shadow, z) = style.query.get_mut(entity).ok()?;

        let caret_bundle = caret_style.query.get_mut(cursor_entity.0).ok()?;
        let (ca_node, ca_bg, ca_border, ca_shadow, ca_z) = caret_bundle;

        let editor_bundle = editor.get_mut(editor_entity.0).ok()?;
        let (input_editor_style, input_editor, input_value) = editor_bundle;

        return Some(TextInputWidget {
            entity,
            class: class.into_inner(),
            style: WidgetStyle {
                node: node.into_inner(),
                background_color: bg.into_inner(),
                border_color: border.into_inner(),
                shadow: shadow.into_inner(),
                z_index: z.into_inner(),
            },
            caret_style: WidgetStyle {
                node: ca_node.into_inner(),
                background_color: ca_bg.into_inner(),
                border_color: ca_border.into_inner(),
                shadow: ca_shadow.into_inner(),
                z_index: ca_z.into_inner()
            },
            value: input_value.into_inner(),
            editor: input_editor.into_inner(),
            editor_style: input_editor_style.into_inner(),
            ctx: ctx,
            commands: commands,
            input_computed: computed
        });
    }

    fn find_by_id<'a>(&'a mut self, target_id: &str) -> Option<Self::WidgetView<'a>> {
        let entity = self.id_class.iter()
            .find(|(_, id, _)| id.0 == target_id)
            .map(|(e, _, _)| e)?;

        self.get_components(entity)
    }

    fn find_by_entity<'a>(&'a mut self, target_entity: Entity) -> Option<Self::WidgetView<'a>> {
        self.get_components(target_entity)
    }

    fn find_by_class(&self, target_class: &str) -> Vec<Entity> {
        self.id_class.iter()
            .filter(|(_, _, class)| class.0.split(" ").any(|word| word == target_class))
            .map(|(e, _, _)| e)
            .collect()
    }
}

/// Bundle for creating `text_input`.
#[derive(Bundle)]
pub struct TextInputBundle {
    pub id_class: IdAndClass,
    pub style: ContainerStyle,
    pub editor_style: TextInputEditorStyle,
    pub caret_style: ContainerStyle,
    pub tooltip_bundle: TooltipBundle,
    pub placeholder: TextInputPlaceholder,
}

impl Default for TextInputBundle {
    fn default() -> Self {
        let style = ContainerStyle {
            node: Node {
                width: px(100.0),
                height: auto(),
                padding: UiRect::all(px(4.0)),
                border_radius: DEFAULT_INPUT_BORDER_RADIUS,
                ..default()
            },
            background_color: BackgroundColor(LIGHT_INPUT_BG_COLOR),
            ..default()
        };

        let caret_style = ContainerStyle {
            node: Node {
                width: px(1.0),
                height: percent(80.0),
                left: px(0.0),
                top: percent(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(LIGHT_CARET_COLOR),
            shadow: BoxShadow::default(),
            ..default()
        };

        let editor_style = TextInputEditorStyle::default();
        let tooltip_bundle = TooltipBundle::default();
        let placeholder = TextInputPlaceholder::default();
        let id_class = IdAndClass::default();

        Self { style, editor_style, caret_style, tooltip_bundle, placeholder, id_class }
    }
}

impl TextInputBundle {
    /// Set color for caret
    pub fn caret_color(mut self, color: Color) -> Self {
        self.caret_style.background_color.0 = color;
        self
    }

    /// Set color for selection
    pub fn selection_color(mut self, color: Color) -> Self {
        self.editor_style.selection_color = color;
        self
    }

    /// Set color for placeholder text
    pub fn placeholder_color(mut self, color: Color) -> Self {
        self.editor_style.placeholder_color = color;
        self
    }

    /// Build `text_input` as disabled.
    pub fn build_as_disabled(self) -> impl Bundle {
        (self.build(), InteractionDisabled)
    }
}

impl Widget for TextInputBundle {
    /// Build `text_input`
    fn build(mut self) -> impl Bundle {
        process_built_in_spacing_class(&self.id_class.class, &mut self.style.node);
        (
            self.id_class,
            self.style,
            MakaraTextInput,
            WidgetFocus(false),
            children![
                (
                    self.editor_style,
                    self.placeholder,
                    Node::default(),
                    TextInputValue::default(),
                    MakaraTextInputEditor,
                    children![
                        (
                            self.caret_style,
                            Visibility::Visible,
                            CursorBlinkTimer::default(),
                            MakaraTextInputCursor
                        )
                    ]
                ),
                self.tooltip_bundle.build()
            ],
            observe(on_mouse_out),
            observe(on_mouse_over),
            observe(on_mouse_click),
            observe(on_mouse_drag),
            observe(on_mouse_drag_start),
        )
    }
}

impl SetContainerStyle for TextInputBundle {
    fn container_style(&mut self) -> &mut ContainerStyle {
        &mut self.style
    }
}

impl SetToolTip for TextInputBundle {
    fn set_tooltip(&mut self) -> &mut TooltipBundle {
        &mut self.tooltip_bundle
    }
}

impl SetIdAndClass for TextInputBundle {
    fn id_and_class(&mut self) -> &mut IdAndClass {
        &mut self.id_class
    }
}


/// Create default text input (light variant) as default theme is light.
pub fn text_input(placeholder: &str) -> TextInputBundle {
    let mut bundle = TextInputBundle::default();
    bundle.placeholder.0 = placeholder.to_string();
    bundle
}

fn on_mouse_click(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut widgets: Query<(Entity, &mut WidgetFocus)>,
    mut ctx: ResMut<MakaraTextEditContext>,
    mut editors: Query<(
        &UiGlobalTransform,
        &ComputedNode,
        &TextInputEditorStyle,
        &TextInputValue,
        &mut TextEditor
    )>,
    inputs: Query<&TextInputEditorEntity, With<MakaraTextInput>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>
) {
    update_focus_state_for_widgets_on_click(click.entity, &mut widgets);
    commands.trigger(Clicked { entity: click.entity });
    click.propagate(false);

    if !mouse_button.just_released(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };

    if let Ok(editor_entity) = inputs.get(click.entity) {
        if let Ok((trans, computed, editor_style, input_value, mut input_edit)) = editors.get_mut(editor_entity.0) {
            if input_value.0.is_empty() {
                return;
            }

            if input_edit.was_dragging {
                input_edit.was_dragging = false;
                return;
            }

            let min = trans.translation - (computed.size() * computed.inverse_scale_factor()) / 2.0;
            let max = trans.translation + (computed.size() * computed.inverse_scale_factor()) / 2.0;

            if cursor_pos.x >= min.x && cursor_pos.x <= max.x &&
               cursor_pos.y >= min.y && cursor_pos.y <= max.y
            {
                // Calculate local position relative to ui node
                let local_x = cursor_pos.x - min.x;
                let local_y = cursor_pos.y - min.y;

                input_edit.set_buffer_cursor_position_on_click(
                    &mut ctx,
                    (local_x, local_y),
                    editor_style.scale_factor
                );
            }
        }
    }
}

fn on_mouse_drag(
    mut drag: On<Pointer<Drag>>,
    mut ctx: ResMut<MakaraTextEditContext>,
    mut editors: Query<(
        &UiGlobalTransform,
        &ComputedNode,
        &TextInputEditorStyle,
        &TextInputValue,
        &mut TextEditor
    )>,
    inputs: Query<&TextInputEditorEntity, With<MakaraTextInput>>,
    windows: Query<&Window>
) {
    drag.propagate(false);

    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };

    if let Ok(editor_entity) = inputs.get(drag.entity) {
        if let Ok((trans, computed, editor_style, input_value, mut input_edit)) = editors.get_mut(editor_entity.0) {
            if input_value.0.is_empty() {
                return;
            }

            let min = trans.translation - (computed.size() * computed.inverse_scale_factor()) / 2.0;
            let max = trans.translation + (computed.size() * computed.inverse_scale_factor()) / 2.0;

            if cursor_pos.x >= min.x && cursor_pos.x <= max.x &&
               cursor_pos.y >= min.y && cursor_pos.y <= max.y
            {
                // Calculate local position relative to ui node
                let local_x = cursor_pos.x - min.x;
                let local_y = cursor_pos.y - min.y;

                input_edit.was_dragging = true;
                input_edit.set_buffer_cursor_position_on_drag(
                    &mut ctx,
                    (local_x, local_y),
                    editor_style.scale_factor
                );

                if let Some(selected_text) = input_edit.editor.copy_selection() {
                    input_edit.selected_text = selected_text;
                }
            }
        }
    }
}

fn on_mouse_drag_start(
    mut drag: On<Pointer<DragStart>>,
    mut ctx: ResMut<MakaraTextEditContext>,
    mut editors: Query<(
        &UiGlobalTransform,
        &ComputedNode,
        &TextInputEditorStyle,
        &TextInputValue,
        &mut TextEditor
    )>,
    inputs: Query<&TextInputEditorEntity, With<MakaraTextInput>>,
    windows: Query<&Window>
) {
    drag.propagate(false);

    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };

    if let Ok(editor_entity) = inputs.get(drag.entity) {
        if let Ok((trans, computed, editor_style, input_value, mut input_edit)) = editors.get_mut(editor_entity.0) {
            if input_value.0.is_empty() {
                return;
            }

            let min = trans.translation - (computed.size() * computed.inverse_scale_factor()) / 2.0;
            let max = trans.translation + (computed.size() * computed.inverse_scale_factor()) / 2.0;

            if cursor_pos.x >= min.x && cursor_pos.x <= max.x &&
               cursor_pos.y >= min.y && cursor_pos.y <= max.y
            {
                // Calculate local position relative to ui node
                let local_x = cursor_pos.x - min.x;
                let local_y = cursor_pos.y - min.y;

                input_edit.set_buffer_cursor_position_on_click(
                    &mut ctx,
                    (local_x, local_y),
                    editor_style.scale_factor
                );
            }
        }
    }
}

fn on_mouse_over(
    mut over: On<Pointer<Over>>,
    mut commands: Commands,
    mut tooltips: Query<
        (&mut Node, &ComputedNode, &TooltipPosition, &UseTooltip),
        With<MakaraTooltip>
    >,
    mut inputs: Query<
        (Has<InteractionDisabled>, &Children, &UiTransform, &ComputedNode),
        With<MakaraTextInput>
    >,
    window: Single<Entity, With<Window>>,
) {
    if let Ok((is_disabled, children, transform, computed)) = inputs.get_mut(over.entity) {
        let cursor_icon = if is_disabled {
            CursorIcon::System(SystemCursorIcon::Default)
        } else {
            CursorIcon::System(SystemCursorIcon::Text)
        };

        commands.entity(*window).insert(cursor_icon);
        show_or_hide_tooltip(true, &mut tooltips, Some(computed), Some(transform), children);
    }
    commands.trigger(MouseOver { entity: over.entity });
    over.propagate(false);
}

pub(crate) fn detect_new_text_input_added(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut input_editors: Query<(
        &mut TextInputEditorStyle,
        Option<&mut TextEditor>,
        &TextInputValue,
        &TextInputPlaceholder,
        &Children
    )>,
    mut ctx: ResMut<MakaraTextEditContext>,
    inputs: Query<
        (Entity, &ComputedNode, &Children),
        (With<MakaraTextInput>, Or<(Added<ComputedNode>, Changed<ComputedNode>)>)
    >,
    cursors: Query<&MakaraTextInputCursor>,
    font_assets: Res<Assets<Font>>,
    windows: Query<&Window>,
) {
    for (input_entity, input_computed, children) in inputs.iter() {
        let window = windows.single();

        if input_computed.size.x == 0.0 || window.is_err() {
            continue;
        }

        let scale_factor = window.unwrap().scale_factor();

        for child in children {
            if let Ok((
                mut editor_style,
                input_editor,
                input_value,
                placeholder,
                editor_children
            )) = input_editors.get_mut(*child) {
                let scaled_size = editor_style.font_size * scale_factor;
                let handle = images.add(Image::default());
                let line_height_physical = (scaled_size * 1.4).round();
                let buffer_width_physical = input_computed.size.x * scale_factor;
                let metrics = Metrics::new(scaled_size, line_height_physical);

                let mut attrs = Attrs::new();
                let mut buffer = Buffer::new(&mut ctx.font_system, metrics);
                let default_handle = Handle::<Font>::default();

                if let Some(font) = font_assets.get(&default_handle) {
                    let data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new((*font.data).clone());

                    let face_ids = ctx.font_system
                        .db_mut()
                        .load_font_source(cosmic_text::fontdb::Source::Binary(data));

                    // Get face ID for Attrs.
                    // ref: https://github.com/bevyengine/bevy/blob/main/crates/bevy_text/src/pipeline.rs#L170
                    if let Some(&face_id) = face_ids.last() {
                        let face = ctx.font_system.db().face(face_id).unwrap().clone();
                        let family_name = face.families[0].0.clone();
                        let family_name: &'static str = Box::leak(family_name.into_boxed_str());

                        attrs = Attrs::new()
                            .family(Family::Name(family_name))
                            .weight(Weight(face.weight.0))
                            .stretch(face.stretch)
                            .style(face.style);

                        ctx.attrs = attrs.clone();
                    }
                }

                if !input_value.0.is_empty() {
                    buffer.set_text(&mut ctx.font_system, &input_value.0, &attrs, Shaping::Advanced, None);
                }
                else if !placeholder.0.is_empty() {
                    buffer.set_text(&mut ctx.font_system, &placeholder.0, &attrs, Shaping::Advanced, None);
                }

                buffer.set_size(&mut ctx.font_system, Some(buffer_width_physical), Some(line_height_physical));
                buffer.set_wrap(&mut ctx.font_system, cosmic_text::Wrap::None);

                let mut editor = cosmic_text::Editor::new(buffer);
                editor.set_redraw(true);

                let logical_width = input_computed.size.x;
                let logical_height = line_height_physical / scale_factor;

                commands.entity(*child).insert((
                    TextEditor::new(editor),
                    Node {
                        width: px(logical_width),
                        height: px(logical_height),
                        ..default()
                    }
                ));

                if input_editor.is_none() {
                    commands.entity(*child).insert(ImageNode::new(handle));
                }

                editor_style.scale_factor = scale_factor;
                commands.entity(*child).insert(MainTextInputEntity(input_entity));
                commands.entity(input_entity).insert(TextInputEditorEntity(*child));

                for editor_child in editor_children {
                    if let Ok(_) = cursors.get(*editor_child) {
                        commands.entity(*child).insert(TextInputCursorEntity(*editor_child));
                        commands.entity(input_entity).insert(TextInputCursorEntity(*editor_child));
                        break;
                    }
                }
            }
        }
    }
}

fn render_editor_to_image(
    input: &mut TextEditor,
    ctx: &mut MakaraTextEditContext,
    input_value: &String,
    editor_style: &TextInputEditorStyle,
    bound_size: &Vec2,
) -> Image {
    // 1. Force cosmic-text to calculate the scroll needed to keep the cursor in view
    // CHANGE: Use 'true' to force an immediate re-shape so the width is current
    input.editor.shape_as_needed(&mut ctx.font_system, true);

    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut size = Extent3d::default();
    let mut canvas = Vec::new();

    let selection = input.editor.selection();
    let cursor = input.editor.cursor();

    if let Some(cursor_pos) = input.editor.cursor_position() {
        input.editor.with_buffer_mut(|buffer| {
            buffer.shape_until_scroll(&mut ctx.font_system, true);

            let cursor_x = cursor_pos.0 as f32;
            let mut scroll = buffer.scroll();
            let view_left = scroll.horizontal;
            let view_right = scroll.horizontal + bound_size.x;

            if cursor_x < view_left {
                scroll.horizontal = cursor_x;
            } else if cursor_x > view_right {
                scroll.horizontal = cursor_x - bound_size.x + 2.0;
            }

            let text_width = buffer
                .layout_runs()
                .map(|run| run.line_w)
                .fold(0.0, f32::max);

            // let mut scroll = buffer.scroll();
            let max_scroll = (text_width - bound_size.x).max(0.0);

            if !input_value.is_empty() {
                // scroll.horizontal = max_scroll;
                scroll.horizontal = scroll.horizontal.clamp(0.0, max_scroll);
                buffer.set_scroll(scroll);
                buffer.shape_until_scroll(&mut ctx.font_system, true);
            }

            // 6. Setup Canvas
            width = bound_size.x as u32;
            height = bound_size.y as u32;
            size = Extent3d { width, height, ..default() };
            canvas = vec![0u8; (width * height * 4) as usize];

            let scroll_x = buffer.scroll().horizontal;
            let scroll_y = buffer.scroll().vertical;
            let text_color = editor_style.text_color.to_srgba().to_u8_array();
            let selection_color = editor_style.selection_color.to_srgba().to_u8_array();
            let placeholder_color = editor_style.placeholder_color.to_srgba().to_u8_array();

            // draw selection backgrounds
            for run in buffer.layout_runs() {
                match selection {
                    Selection::Normal(select_anchor) | Selection::Line(select_anchor) => {
                        draw_selection_background_single_line(
                            &mut canvas,
                            &select_anchor,
                            &cursor,
                            &run,
                            (scroll_x, scroll_y),
                            (width, height),
                            &selection_color
                        );
                    }
                    _ => {}
                }
            }

            // draw glyphs
            for run in buffer.layout_runs() {
                for glyph in run.glyphs {
                    let physical = glyph.physical((0.0, 0.0), 1.0);

                    if let Some(image) = ctx.swash_cache.get_image(&mut ctx.font_system, physical.cache_key) {
                        let glyph_left = (physical.x as f32 - scroll_x) as i32 + image.placement.left;
                        let glyph_top = (run.line_y - scroll_y).round() as i32 + physical.y - image.placement.top;

                        for y in 0..image.placement.height {
                            for x in 0..image.placement.width {
                                let alpha = image.data[(y * image.placement.width + x) as usize];

                                if alpha > 0 {
                                    let canvas_x = glyph_left + x as i32;
                                    let canvas_y = glyph_top + y as i32;

                                    if canvas_x >= 0 && (canvas_x as u32) < width &&
                                        canvas_y >= 0 && (canvas_y as u32) < height {
                                        let dst_idx = ((canvas_y as u32 * width + canvas_x as u32) * 4) as usize;
                                        let color = if input_value.is_empty() { placeholder_color } else { text_color };

                                        blend_glyph_pixel(&mut canvas, dst_idx, color, alpha);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    // 8. Create the Bevy Image
    let mut img = Image::new(
        size,
        bevy::render::render_resource::TextureDimension::D2,
        canvas,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    img.sampler = ImageSampler::nearest();
    img
}

pub(crate) fn handle_text_input_typing(
    mut commands: Commands,
    mut keyboard_ev: MessageReader<KeyboardInput>,
    mut ctx: ResMut<MakaraTextEditContext>,
    mut editors: Query<(
        &mut TextEditor,
        &mut TextInputValue,
        &TextInputPlaceholder,
        &ComputedNode
    )>,
    inputs: Query<(Entity, &WidgetFocus, &TextInputEditorEntity), With<MakaraTextInput>>,
    keys: Res<ButtonInput<KeyCode>>,
    #[cfg(target_arch = "wasm32")] wasm_channel: Option<Res<WasmPasteAsyncChannel>>,
) {
    for (input_entity, w_focus, editor_entity) in inputs.iter() {
        if !w_focus.0 {
            continue;
        }

        if let Ok((mut input_edit, mut value, placeholder, computed)) = editors.get_mut(editor_entity.0) {
            for ev in keyboard_ev.read() {
                 // it's key up? nevermind
                if ev.state == ButtonState::Released {
                    continue;
                }
                let old_value = value.0.clone();

                if is_ctrl_a_pressed(&keys, ev.key_code) {
                    input_edit.select_all(&mut ctx, &mut value.0, computed);
                    continue;
                }
                else if is_ctrl_c_pressed(&keys, ev.key_code) {
                    input_edit.copy_text(&value.0);
                    continue;
                }
                else if is_ctrl_v_pressed(&keys, ev.key_code) {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        input_edit.paste_text(&mut ctx, &mut value.0);

                        if old_value != value.0 {
                            commands.trigger(Change {
                                entity: input_entity,
                                data: value.0.clone()
                            });
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(channel) = wasm_channel.as_deref() {
                            input_edit.paste_text_wasm(input_entity, channel);
                        }
                    }

                    continue;
                }

                match &ev.logical_key {
                    Key::Character(ch) => input_edit.insert_char(&mut ctx, &mut value.0, ch),
                    Key::Space => input_edit.insert_space(&mut ctx, &mut value.0),
                    Key::Backspace => input_edit.backspace(&mut ctx, &mut value.0, &placeholder.0),
                    Key::Escape => input_edit.escape(),
                    Key::ArrowLeft => input_edit.arrow_left(&mut ctx, &value.0),
                    Key::ArrowRight => input_edit.arrow_right(&mut ctx, &value.0),
                    _ => {}
                }

                if old_value != value.0 {
                    commands.trigger(Change {
                        entity: input_entity,
                        data: value.0.clone()
                    });
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn on_wasm_paste(
    channel: Res<WasmPasteAsyncChannel>,
    inputs: Query<(Entity, &WidgetFocus, &TextInputEditorEntity), With<MakaraTextInput>>,
    mut ctx: ResMut<MakaraTextEditContext>,
    mut editors: Query<(&mut TextEditor, &mut TextInputValue)>,
    mut commands: Commands
) {
    let inlet = channel.rx.try_recv();

    match inlet {
        Ok(inlet) => {
            for (input_entity, w_focus, editor_entity) in inputs.iter() {
                if !w_focus.0 {
                    continue;
                }

                if let Ok((mut input_edit, mut value)) = editors.get_mut(editor_entity.0) {
                    let old_value = value.0.clone();

                    if value.0.is_empty() {
                        input_edit.clear_text_editor(&mut ctx);
                    }
                    input_edit.editor.insert_string(&inlet.text, None);
                    input_edit.editor.with_buffer(|buffer| {
                        let full_text: String = buffer
                            .lines
                            .iter()
                            .map(|line| line.text())
                            .collect::<Vec<_>>()
                            .join("\n");

                        value.0 = full_text;
                    });

                    if old_value != value.0 {
                        commands.trigger(Change {
                            entity: input_entity,
                            data: value.0.clone()
                        });
                    }
                }
            }
        }
        Err(_) => {}
    }
}

pub(crate) fn update_text_input_render(
    mut images: ResMut<Assets<Image>>,
    mut ctx: ResMut<MakaraTextEditContext>,
    mut editors: Query<(
        &mut TextEditor,
        &ImageNode,
        &TextInputEditorStyle,
        &TextInputValue,
        &ComputedNode,
        &Children
    )>,
    mut cursor_query: Query<&mut Node, With<MakaraTextInputCursor>>,
) {
    for (mut input, img_node, editor_style, value, computed, children) in editors.iter_mut() {
        // If the UI hasn't calculated a size yet, skip this frame.
        if computed.size.x <= 0.0 || computed.size.y <= 0.0 {
            continue;
        }
        if !input.editor.redraw() {
            continue;
        }
        let new_img = render_editor_to_image(
            &mut input,
            &mut ctx,
            &value.0,
            &editor_style,
            &computed.size
        );

        // update existing image
        if let Some(img) = images.get_mut(&img_node.image) {
            *img = new_img;
        }

        let scale = editor_style.scale_factor;
        let logical_cursor_x = input.logical_cursor_position_x(scale, &computed);

        for &child in children {
            if let Ok(mut cursor_node) = cursor_query.get_mut(child) {
                cursor_node.left = Val::Px(logical_cursor_x);

                // Always show it now, since we are sticking it to the edges
                cursor_node.display = Display::Flex;
            }
        }
        input.editor.set_redraw(false);
    }
}

pub(crate) fn handle_cursor_blink(
    time: Res<Time>,
    inputs: Query<&WidgetFocus, With<MakaraTextInput>>,
    input_editors: Query<&MainTextInputEntity>,
    mut cursors: Query<
        (&mut Visibility, &CursorBlinkTimer, &ChildOf),
        With<MakaraTextInputCursor>
    >
) {
    for (mut vis, _blink_timer, parent) in cursors.iter_mut() {
        if let Ok(input_entity) = input_editors.get(parent.0) {
            if let Ok(w_focus) = inputs.get(input_entity.0) {
                if !w_focus.0 {
                    *vis = Visibility::Hidden;
                    continue;
                }

                if (time.elapsed_secs() * 2.0) as u32 % 2 == 0 {
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
                }
            }
        }
    }
}

// system to update color on theme change for both input and placeholder color, and text color;
pub(crate) fn update_text_input_style_on_theme_change(
    makara_theme: Res<MakaraTheme>,
    mut inputs: Query<
        &mut BackgroundColor,
        (With<MakaraTextInput>, Without<MakaraTextInputCursor>)
    >,
    mut carets: Query<
        &mut BackgroundColor,
        (With<MakaraTextInputCursor>, Without<MakaraTextInput>)
    >,
    mut editors: Query<(&mut TextInputEditorStyle, &mut TextEditor)>
) {
    if !makara_theme.is_changed() {
        return;
    }

    // input bg
    {
        let new_bg_color = match makara_theme.theme {
            Theme::Light => LIGHT_INPUT_BG_COLOR,
            Theme::Dark => DARK_INPUT_BG_COLOR
        };

        for mut bg_color in inputs.iter_mut() {
            // only react to theme change if color is default
            if bg_color.0 == LIGHT_INPUT_BG_COLOR || bg_color.0 == DARK_INPUT_BG_COLOR {
                bg_color.0 = new_bg_color;
            }
        }
    }

    // carets
    {
        let new_caret_color = match makara_theme.theme {
            Theme::Light => LIGHT_CARET_COLOR,
            Theme::Dark => DARK_CARET_COLOR,
        };

        for mut color in carets.iter_mut() {
            if color.0 == LIGHT_CARET_COLOR || color.0 == DARK_CARET_COLOR {
                color.0 = new_caret_color;
            }
        }
    }

    // editors
    {
        let new_text_color = match makara_theme.theme {
            Theme::Light => LIGHT_THEME_TEXT_COLOR,
            Theme::Dark => DARK_THEME_TEXT_COLOR,
        };

        let new_pl_color = match makara_theme.theme {
            Theme::Light => LIGHT_PLACEHOLDER_COLOR,
            Theme::Dark => DARK_PLACEHOLDER_COLOR,
        };

        for (mut editor_style, mut editor) in editors.iter_mut() {
            if editor_style.text_color == LIGHT_THEME_TEXT_COLOR ||
               editor_style.text_color == DARK_THEME_TEXT_COLOR
            {
                editor_style.text_color = new_text_color;
            }

            if editor_style.placeholder_color == LIGHT_PLACEHOLDER_COLOR ||
               editor_style.placeholder_color == DARK_PLACEHOLDER_COLOR
            {
                editor_style.placeholder_color = new_pl_color;
            }
            editor.editor.set_redraw(true);
        }
    }
}

pub(crate) fn can_run_text_input_systems(q: Query<&MakaraTextInput>) -> bool {
    q.count() > 0
}
