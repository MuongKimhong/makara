/// helper functions for text-input and text-area.

use cosmic_text::{
     Editor, Edit, Selection, Action, Motion,
     Shaping, Cursor, LayoutRun
};
use bevy::prelude::*;
use smol_str::SmolStr;

#[cfg(not(target_arch = "wasm32"))]
use arboard::Clipboard;

#[cfg(target_arch = "wasm32")]
use crate::widgets::text_input::{
    write_clipboard_wasm, read_clipboard_wasm,
    WasmPaste, WasmPasteAsyncChannel
};

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
use arboard::{SetExtLinux, LinuxClipboardKind};

#[cfg(target_arch = "wasm32")]
use bevy::tasks::AsyncComputeTaskPool;
#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

use crate::MakaraTextEditContext;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub(crate) enum InputType{
    TextInput,
    TextArea
}

#[derive(Component, Debug)]
pub struct TextEditor {
    pub editor: Editor<'static>,
    pub selected_text: String,
    pub select_start_cursor_index: usize,
    pub was_dragging: bool,

    #[allow(dead_code)]
    pub(crate) input_type: InputType,
}

impl TextEditor {
    pub fn new(editor: Editor<'static>) -> Self {
        TextEditor {
            editor,
            selected_text: "".to_string(),
            select_start_cursor_index: 0,
            was_dragging: false,
            input_type: InputType::TextInput,
        }
    }

    /// clear text-input and text-area.
    pub fn clear_text_editor(&mut self, ctx: &mut MakaraTextEditContext) {
        let attrs = ctx.attrs.clone();

        self.editor.with_buffer_mut(|buffer| {
            buffer.set_text(
                &mut ctx.font_system,
                "",
                &attrs,
                Shaping::Advanced,
                None
            );
        });
    }

    /// reset text-input and text-area to show default placeholder.
    pub fn reset_text_editor(&mut self, ctx: &mut MakaraTextEditContext, placeholder: &str) {
        let attrs = ctx.attrs.clone();

        self.editor.with_buffer_mut(|buffer| {
            buffer.set_text(
                &mut ctx.font_system,
                placeholder,
                &attrs,
                Shaping::Advanced,
                None
            );
        });
    }

    pub fn insert_char(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        value: &mut String,
        ch: &SmolStr
    ) {
        if let Some(c) = ch.chars().next() {
            if value.is_empty() {
                self.clear_text_editor(ctx);
            }
            self.editor.action(&mut ctx.font_system, Action::Insert(c));
            self.editor.with_buffer(|buffer| {
                let full_text: String = buffer
                    .lines
                    .iter()
                    .map(|line| line.text())
                    .collect::<Vec<_>>()
                    .join("\n");

                *value = full_text;
            });
        }
    }

    pub fn insert_space(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        value: &mut String
    ) {
        self.insert_char(ctx, value, &SmolStr::new(" "));
    }

    pub fn delete_text(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        value: &mut String,
        placeholder: &str
    ) {
        self.editor.action(&mut ctx.font_system, Action::Backspace);
        self.editor.set_selection(cosmic_text::Selection::None);
        let full_text = self.editor.with_buffer(|buffer| {
            buffer.lines.iter().map(|line| line.text()).collect::<Vec<_>>().join("\n")
        });

        if full_text != placeholder {
            *value = full_text;
        }
        self.editor.shape_as_needed(&mut ctx.font_system, true);
        self.editor.set_redraw(true);
    }

    pub fn backspace(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        value: &mut String,
        placeholder: &str
    ) {
        self.delete_text(ctx, value, placeholder);

        if !self.selected_text.is_empty() {
            self.selected_text.clear();
        }

        if value.is_empty() {
            self.reset_text_editor(ctx, placeholder);
        }
    }

    /// Get cursor x position in pixel space, relative to text-edit node.
    pub fn logical_cursor_position_x(
        &self,
        scale: f32,
        editor_computed: &ComputedNode
    ) -> f32 {
        let mut x = 0.0;
        let cursor_width = 1.0; // always 1.0

        if let Some(layout_cursor) = self.editor.cursor_position() {
            self.editor.with_buffer(|buffer| {
                let scroll_x = buffer.scroll().horizontal;

                // Calculate the raw logical position
                let raw_cursor_x = (layout_cursor.0 as f32 - scroll_x) / scale;

                x = if editor_computed.size.x - cursor_width >= 0.0 {
                    raw_cursor_x.clamp(0.0, editor_computed.size.x - cursor_width)
                } else {
                    raw_cursor_x.clamp(0.0, editor_computed.size.x)
                };
            });
        }

        x
    }

    /// Get click position of buffer.
    pub fn get_buffer_click_position(
        &mut self,
        local_pos: (f32, f32),
        scale: f32
    ) -> (f32, f32) {
        let mut buffer_click_x = 0.0;
        let mut buffer_click_y = 0.0;

        self.editor.with_buffer(|buffer| {
            let scroll_x = buffer.scroll().horizontal;
            let scroll_y = buffer.scroll().vertical;

            // Convert to physical coordinates
            buffer_click_x = (local_pos.0 * scale) + scroll_x;
            buffer_click_y = (local_pos.1 * scale) + scroll_y;
        });

        (buffer_click_x, buffer_click_y)
    }

    /// Set buffer cursor to specific position using bevy mouse position on click.
    pub fn set_buffer_cursor_position_on_click(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        local_pos: (f32, f32), // local (x, y) relative to ui input node
        scale: f32,
    ) {
        let buffer_click = self.get_buffer_click_position(local_pos, scale);

        self.editor.action(
            &mut ctx.font_system,
            Action::Click {
                x: buffer_click.0 as i32,
                y: buffer_click.1 as i32,
            }
        );
        self.editor.shape_as_needed(&mut ctx.font_system, true);
    }

    /// Set buffer cursor to specific position using bevy mouse position on drag.
    pub fn set_buffer_cursor_position_on_drag(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        local_pos: (f32, f32), // local (x, y) relative to ui input node
        scale: f32,
    ) {
        let buffer_click = self.get_buffer_click_position(local_pos, scale);

        self.editor.action(
            &mut ctx.font_system,
            Action::Drag {
                x: buffer_click.0 as i32,
                y: buffer_click.1 as i32,
            }
        );
        self.editor.shape_as_needed(&mut ctx.font_system, true);
    }

    pub fn arrow_left(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        value: &String
    ) {
        if value.is_empty() {
            return;
        }
        if !self.selected_text.is_empty() {
            self.editor.action(&mut ctx.font_system, Action::Motion(Motion::BufferStart));
            self.escape();
            return;
        }
        self.editor.action(&mut ctx.font_system, Action::Motion(Motion::Left));
    }

    pub fn arrow_right(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        value: &String
    ) {
        if value.is_empty() {
            return;
        }
        if !self.selected_text.is_empty() {
            self.editor.action(&mut ctx.font_system, Action::Motion(Motion::BufferEnd));
            self.escape();
            return;
        }
        self.editor.action(&mut ctx.font_system, Action::Motion(Motion::Right));
    }

    pub fn escape(&mut self) {
        if !self.selected_text.is_empty() {
            self.editor.set_selection(Selection::None);
            self.selected_text.clear();
            self.editor.with_buffer_mut(|buffer| {
                buffer.set_redraw(true);
            });
        }
    }

    pub fn select_all(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        value: &mut String,
        editor_computed: &ComputedNode
    ) {
        if value.is_empty() {
            return;
        }

        self.editor.action(&mut ctx.font_system, Action::Motion(Motion::BufferEnd));
        self.editor.set_selection(Selection::Normal(Cursor::new(0, 0)));
        self.editor.with_buffer_mut(|buffer| {
            buffer.shape_until_scroll(&mut ctx.font_system, true);

            let text_width = buffer
                .layout_runs()
                .map(|run| run.line_w)
                .fold(0.0, f32::max);

            let mut scroll = buffer.scroll();
            let max_scroll = (text_width - editor_computed.size.x).max(0.0);

            scroll.horizontal = max_scroll;
            buffer.set_scroll(scroll);
            buffer.shape_until_scroll(&mut ctx.font_system, true);
        });
        self.selected_text = value.clone();
    }

    pub fn copy_text(&mut self, value: &String) {
        if self.selected_text.trim().is_empty() {
            return;
        }

        if value.is_empty() {
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut ctx = Clipboard::new().unwrap();
            #[cfg(target_os = "linux")]
            ctx.set().clipboard(LinuxClipboardKind::Clipboard).text(self.selected_text.clone()).unwrap();
            #[cfg(not(target_os = "linux"))]
            ctx.set_text(self.selected_text.clone()).unwrap();
        }

        #[cfg(target_arch = "wasm32")]
        {
            write_clipboard_wasm(&self.selected_text);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn paste_text(
        &mut self,
        ctx: &mut MakaraTextEditContext,
        value: &mut String,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut clipboard_ctx = Clipboard::new().unwrap();
            let copied_text = clipboard_ctx.get_text().ok();

            if let Some(text) = copied_text {
                if value.is_empty() {
                    self.clear_text_editor(ctx);
                }
                self.editor.insert_string(&text, None);
            }

            self.editor.with_buffer(|buffer| {
                let full_text: String = buffer
                    .lines
                    .iter()
                    .map(|line| line.text())
                    .collect::<Vec<_>>()
                    .join("\n");

                *value = full_text;
            });
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn paste_text_wasm(
        &mut self,
        entity: Entity,
        wasm_channel: &WasmPasteAsyncChannel
    ) {
        let tx = wasm_channel.tx.clone();
        let _task = AsyncComputeTaskPool::get().spawn(async move {
            let promise = read_clipboard_wasm();

            let result = JsFuture::from(promise).await;

            if let Ok(js_text) = result {
                if let Some(text) = js_text.as_string() {
                    let _ = tx.try_send(WasmPaste { text, entity });
                }
            }
        });
    }
}

pub(crate) fn is_ctrl_a_pressed(
    keys: &Res<ButtonInput<KeyCode>>,
    keycode: KeyCode
) -> bool {
    #[cfg(target_os = "macos")]
    let modifier = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);

    #[cfg(not(target_os = "macos"))]
    let modifier = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    modifier && matches!(keycode, KeyCode::KeyA)
}

pub(crate) fn is_ctrl_c_pressed(
    keys: &Res<ButtonInput<KeyCode>>,
    keycode: KeyCode
) -> bool {
    #[cfg(target_os = "macos")]
    let modifier = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);

    #[cfg(not(target_os = "macos"))]
    let modifier = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    modifier && matches!(keycode, KeyCode::KeyC)
}

pub(crate) fn is_ctrl_v_pressed(
    keys: &Res<ButtonInput<KeyCode>>,
    keycode: KeyCode
) -> bool {
    #[cfg(target_os = "macos")]
    let modifier = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);

    #[cfg(not(target_os = "macos"))]
    let modifier = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    modifier && matches!(keycode, KeyCode::KeyV)
}

pub(crate) fn blend_glyph_pixel(
    canvas: &mut [u8],
    dst_idx: usize,
    color: [u8; 4],
    alpha: u8,
) {
    let a = alpha as f32 / 255.0;

    // Check if background is empty (Alpha < 5)
    if canvas[dst_idx + 3] < 5 {
        canvas[dst_idx] = color[0];
        canvas[dst_idx + 1] = color[1];
        canvas[dst_idx + 2] = color[2];
        canvas[dst_idx + 3] = alpha;
    }
    else {
        let bg_r = canvas[dst_idx] as f32;
        let bg_g = canvas[dst_idx + 1] as f32;
        let bg_b = canvas[dst_idx + 2] as f32;
        let bg_a = canvas[dst_idx + 3] as f32 / 255.0;

        // Use Porter-Duff 'Over' Operator to make text render properly
        // when text is being selected (selection).
        // Ref: https://ssp.impulsetrain.com/porterduff.html
        let out_a = a + bg_a * (1.0 - a);
        if out_a > 0.0 {
            canvas[dst_idx] = ((color[0] as f32 * a + bg_r * bg_a * (1.0 - a)) / out_a) as u8;
            canvas[dst_idx + 1] = ((color[1] as f32 * a + bg_g * bg_a * (1.0 - a)) / out_a) as u8;
            canvas[dst_idx + 2] = ((color[2] as f32 * a + bg_b * bg_a * (1.0 - a)) / out_a) as u8;
            canvas[dst_idx + 3] = (out_a * 255.0) as u8;
        }
    }
}

pub(crate) fn draw_selection_background_single_line(
    canvas: &mut [u8],
    select_anchor: &Cursor,
    cursor: &Cursor,
    run: &LayoutRun<'_>,
    scroll: (f32, f32), // (x, y)
    bound: (u32, u32), // (width, height)
    selection_color: &[u8; 4]
) {
    let (start, end) = if select_anchor.line < cursor.line ||
        (select_anchor.line == cursor.line && select_anchor.index < cursor.index)
    {
        (select_anchor, cursor)
    } else {
        (cursor, select_anchor)
    };

    if let Some((x_left, x_width)) = run.highlight(*start, *end) {
        let x_start = (x_left - scroll.0) as i32;
        let x_end = (x_left + x_width - scroll.0) as i32;

        let draw_x_start = x_start.max(0);
        let draw_x_end = x_end.min(bound.0 as i32);

        for x in draw_x_start..draw_x_end {
            for y in 0..bound.1 as i32 { // from top to bottom
                let dst_idx = ((y as u32 * bound.0 + x as u32) * 4) as usize;
                canvas[dst_idx] = selection_color[0];
                canvas[dst_idx + 1] = selection_color[1];
                canvas[dst_idx + 2] = selection_color[2];
                canvas[dst_idx + 3] = selection_color[3];
            }
        }
    }
}
