//! Makara is a bevy UI simplifer. Its main goal is to make it easy to build
//! high performance GUI application using bevy engine.
//!
//! # Example
//! ```rust,ignore
//! fn on_button_click(click: On<Clicked>, mut text_q: TextQuery) {
//!    if let Some(text) = text_q.find_by_id("my-text") {
//!        text.text.value.0 = "Hello mars!".to_string();
//!    }
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn((
//!        root().build(),
//!        children![
//!            text("Hello earth").id("my-text").build(),
//!            (
//!                button("Press me").build(),
//!                observe(on_button_click)
//!            )
//!        ]
//!    ));
//! }
//! ```

pub mod widgets;
pub mod consts;
pub mod events;
pub mod utils;
pub mod styles;
pub mod colors;
pub mod routers;

#[macro_use]
pub mod macros;

pub mod prelude {
    use bevy::prelude::*;
    use bevy::asset::embedded_asset;
    use bevy::asset::io::web::WebAssetPlugin;
    use std::collections::HashSet;

    pub use crate::widgets::*;
    pub use crate::consts::*;
    pub use crate::events::*;
    pub use crate::styles::*;
    pub use crate::colors::*;
    pub use crate::routers::*;
    pub use bevy::ui_widgets::observe;

    pub use crate::scroll_;
    pub use crate::row_;
    pub use crate::column_;
    pub use crate::root_;
    pub use crate::button_;
    pub use crate::text_;

    /// Schedule for this plugin to run at.
    /// Default is `AtUpdate`.
    #[derive(Default)]
    pub enum RunSchedule {
        AtPreUpdate,
        AtPostUpdate,
        #[default]
        AtUpdate
    }

    /// Resource use to keep track of custom font.
    #[derive(Resource, Default)]
    pub struct CustomFont {
        pub font_path: String,
        pub font_handle: Option<Handle<Font>>
    }

    /// Plugin for all makara.
    #[derive(Default)]
    pub struct MakaraPlugin {
        pub run_schedule: RunSchedule,
        pub font_path: String
    }

    impl MakaraPlugin {
        /// Set running schedule for makara widgets.
        pub fn run_at(run_schedule: RunSchedule) -> Self {
            Self {
                run_schedule,
                ..default()
            }
        }

        /// Set custom font.
        pub fn set_font(mut self, font_path: &str) -> Self {
            self.font_path = font_path.to_string();
            self
        }
    }

    impl Plugin for MakaraPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(DefaultPlugins.set(WebAssetPlugin {
                silence_startup_warning: false,
            }));
            embedded_asset!(app, "embedded_assets/progress_bar.wgsl");
            embedded_asset!(app, "embedded_assets/circular.wgsl");

            app.add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default());
            app.add_plugins(UiMaterialPlugin::<CircularMaterial>::default());

            app.insert_resource(MakaraTheme::default());
            app.insert_resource(MakaraModalState::default());
            app.insert_resource(MakaraTextEditContext::default());
            app.insert_resource(DropdownOverlayAndTextAdded::default());
            app.insert_resource(CanBeScrolled::default());
            app.insert_resource(ImageHandleMap::default());
            app.insert_resource(CustomStyle::default());
            app.insert_resource(Router::default());
            app.insert_resource(CustomFont {
                font_path: self.font_path.clone(),
                font_handle: None
            });

            #[cfg(target_arch = "wasm32")]
            {
                let (tx, rx) = crossbeam_channel::bounded::<WasmPaste>(1);
                app.insert_resource(WasmPasteAsyncChannel { tx, rx });
            }

            let systems = (
                // btn
                (
                    update_button_style_on_theme_change_system,
                    detect_button_built
                )
                .run_if(can_run_button_systems),

                // text
                (
                    update_text_color_on_theme_change_system,
                    detect_text_built
                )
                .run_if(can_run_text_systems),

                // root
                update_root_style_on_theme_change_system.run_if(can_run_root_systems),

                // checkbox
                (
                    update_checkbox_button_style_on_theme_change_system,
                    update_checkbox_style_on_state_change_system,
                    detect_checkbox_built
                )
                .run_if(can_run_checkbox_systems),

                // modal
                (
                    handle_show_and_hide_modals_system,
                    update_modal_style_on_theme_change_system,
                    detect_modal_children_added,
                    detect_modal_built
                )
                .run_if(can_run_modal_systems),

                // radio
                (
                    update_radio_button_style_on_theme_change_system,
                    update_radio_style_on_state_change_system,
                    detect_radio_built
                )
                .run_if(can_run_radio_systems),

                // dropdown
                (
                    update_dropdown_style_on_theme_change_system,
                    show_and_hide_dropdown_overlay_on_state_change_system,
                    detect_user_provided_children_system,
                    detect_dropdown_overlay_added,
                    detect_dropdown_built
                )
                .run_if(can_run_dropdown_systems),

                // select
                (
                    update_select_style_on_theme_change_system,
                    show_and_hide_select_overlay_on_state_change_system,
                    detect_select_items_added_and_overlay_resized,
                    detect_select_placeholder_added
                )
                .run_if(can_run_select_systems),

                // slider
                (
                    update_slider_style_on_theme_change_system,
                    detect_slider_thumb_added,
                    detect_slider_built
                )
                .run_if(can_run_slider_systems),

                // progress bar
                (
                    update_progress_bar_style_on_theme_change_system,
                    update_progress_bar_material_u_time,
                    detect_progress_value_added,
                    detect_progress_bar_built,
                )
                .run_if(can_run_progress_bar_systems),

                // circular
                (
                    detect_circular_added,
                    update_circular_material_u_time,
                    update_circular_style_on_theme_change_system,
                )
                .run_if(can_run_circular_systems),

                // text input
                (
                    update_text_input_style_on_theme_change,
                    detect_new_text_input_added,
                    update_text_input_render,
                    handle_text_input_typing,
                    handle_cursor_blink,

                    #[cfg(target_arch = "wasm32")]
                    {
                        on_wasm_paste
                    }
                )
                .run_if(can_run_text_input_systems),

                // scroll
                (
                    detect_scroll_built,
                    detect_scroll_children_added,
                    detect_move_panel_height_change,
                    handle_scrolling
                )
                .run_if(can_run_scroll_systems),

                // image
                (
                    track_image_loading_state,
                    detect_new_image_added,
                    detect_image_built
                )
                .run_if(can_run_image_systems),

                update_tooltip_style_on_theme_change_system,
                detect_tooltip_built,

                detect_column_built,
                detect_row_built,
                detect_root_built,
                detect_link_built
            );

            app.add_systems(Startup, setup);
            app.add_systems(Update, (
                detect_makara_text_added,
                detect_widget_class_change,
                check_unique_id,
                handle_match_route_to_root
            ));
            app.add_systems(
                Update,
                (
                    apply_custom_style_to_button,
                    apply_custom_style_to_checkbox,
                    apply_custom_style_to_circular,
                    apply_custom_style_to_row,
                    apply_custom_style_to_column,
                    apply_custom_style_to_dropdown,
                    apply_custom_style_to_image,
                    apply_custom_style_to_link,
                    // apply_custom_style_to_modal,
                    apply_custom_style_to_progress_bar,
                    apply_custom_style_to_root,
                    apply_custom_style_to_text,
                    apply_custom_style_to_slider,
                    apply_custom_style_to_radio,
                    apply_custom_style_to_radio_group,
                    apply_custom_style_to_select,
                    apply_custom_style_to_text_input.after(apply_custom_style_to_dropdown),
                    apply_custom_style_to_scroll.after(apply_custom_style_to_text_input),

                    set_style_unchanged
                        .after(apply_custom_style_to_button)
                        .after(apply_custom_style_to_checkbox)
                        .after(apply_custom_style_to_circular)
                        .after(apply_custom_style_to_column)
                        .after(apply_custom_style_to_row)
                        .after(apply_custom_style_to_dropdown)
                        .after(apply_custom_style_to_image)
                        .after(apply_custom_style_to_link)
                        // .after(apply_custom_style_to_modal)
                        .after(apply_custom_style_to_progress_bar)
                        .after(apply_custom_style_to_root)
                        .after(apply_custom_style_to_scroll)
                        .after(apply_custom_style_to_text)
                        .after(apply_custom_style_to_slider)
                        .after(apply_custom_style_to_radio)
                        .after(apply_custom_style_to_radio_group)
                        .after(apply_custom_style_to_select)
                        .after(apply_custom_style_to_text_input)
                )
            );

            match self.run_schedule {
                RunSchedule::AtUpdate => { app.add_systems(Update, systems); },
                RunSchedule::AtPreUpdate => { app.add_systems(PreUpdate, systems); },
                RunSchedule::AtPostUpdate => { app.add_systems(PostUpdate, systems); },
            }
        }
    }

    fn setup(
        mut commands: Commands,
        mut custom_font: ResMut<CustomFont>,
        asset_server: Res<AssetServer>
    ) {
        commands.spawn(Camera2d);

        if !custom_font.font_path.is_empty() {
            let handle: Handle<Font> = asset_server.load(&custom_font.font_path);
            custom_font.font_handle = Some(handle);
        }
    }

    fn detect_makara_text_added(
        mut texts: Query<&mut TextFont, Added<MakaraText>>,
        custom_font: Res<CustomFont>
    ) {
        for mut text_font in texts.iter_mut() {
            if let Some(handle) = &custom_font.font_handle {
                text_font.font = handle.clone();
            }
        }
    }

    fn check_unique_id(
        all_ids: Query<&Id>,
        new_ids: Query<Entity, Added<Id>>
    ) {
        if new_ids.is_empty() {
            return;
        }

        let mut seen = HashSet::new();

        for id in all_ids.iter() {
            if id.0.is_empty() {
                continue;
            }

            if !seen.insert(&id.0) {
                panic!("Duplicate ID: '{}'. ID must be unique!", id.0);
            }
        }
    }
}

pub use prelude::*;
