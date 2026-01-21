/// Macro for creating a [`ScrollBundle`].
///
/// # Event Handler
/// * `on_scrolling` - Triggered when the scroll position changes.
///
/// # Example
/// ```rust
/// scroll_!(
///     id: "main_list",
///     height: px(300);
///
///     on_scrolling: |scrolling: On<Scrolling>| {
///         println!("{:?}", ev.position)
///     };
///
///     [ text_!("Item 1"), text_!("Item 2") ]
/// )
/// ```
#[macro_export]
macro_rules! scroll_ {
    (
        // Properties (Methods on the builder)
        $( $prop:ident : $val:expr ),* $(,)?

        // Specific Event Handlers
        $( ; on_scrolling: $scroll_handler:expr )?

        // Children
        $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut s = $crate::widgets::scroll::scroll();
            $( s = s.$prop($val); )*

            (
                s.build(),
                // If the user provided a handler, wrap it in observe()
                $( $crate::prelude::observe($scroll_handler), )?
                $(
                    bevy::prelude::children![ $($child),* ]
                )?
            )
        }
    };
}

/// Macro for creating a Horizontal Container ([`RowBundle`]).
///
/// # Example
/// ```rust
/// row_!(
///     justify_content: JustifyContent::SpaceBetween,
///     align_items: AlignItems::Center;
///
///     [
///         text_!("Left Side"),
///         button_!("Right Side")
///     ]
/// )
/// ```
#[macro_export]
macro_rules! row_ {
    (
        $( $prop:ident : $val:expr ),* $(,)?

        // Children
        $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut s = $crate::widgets::row::row();
            $( s = s.$prop($val); )*

            (
                s.build(),
                $(
                    bevy::prelude::children![ $($child),* ]
                )?
            )
        }
    };
}

/// Macro for creating a Vertical container ([`ColumnBundle`]).
///
/// # Example
/// ```rust
/// column_!(
///     padding: px(10);
///
///     [
///         text_!("Top"),
///         text_!("Middle"),
///         text_!("Bottom")
///     ]
/// )
/// ```
#[macro_export]
macro_rules! column_ {
    (
        $( $prop:ident : $val:expr ),* $(,)?
        $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut s = $crate::widgets::column::column();
            $( s = s.$prop($val); )*

            (
                s.build(),
                $(
                    bevy::prelude::children![ $($child),* ]
                )?
            )
        }
    };
}

/// Macro for creating the base UI Root ([`RootBundle`]).
/// Usually the top-level element in a `commands.spawn()` call.
///
/// # Example
/// ```rust
/// commands.spawn(root_!(
///     width: percent(100),
///     height: percent(100);
///
///     [ text_!("Hello wrold") ]
/// ));
/// ```
#[macro_export]
macro_rules! root_ {
    (
        $( $prop:ident : $val:expr ),* $(,)?
        $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut s = $crate::widgets::root::root();
            $( s = s.$prop($val); )*

            (
                s.build(),
                $(
                    bevy::prelude::children![ $($child),* ]
                )?
            )
        }
    };
}

/// Macro for creating a [`ButtonBundle`].
///
/// # Event Handlers
/// * `on_clicked` - Triggered on mouse click/press.
/// * `on_mouse_over` - Triggered when the cursor enters the button area.
/// * `on_mouse_out` - Triggered when the cursor leaves the button area.
///
/// # Example
/// ```rust
/// button_!(
///     "Click Me",
///     background_color: "red";
///
///     on_clicked: |clicked: On<Clicked>| {
///         println!("Button clicked!")
///     }
/// )
/// ```
#[macro_export]
macro_rules! button_ {
    (
        $text:expr
        $(, $prop:ident : $val:expr )* $(,)?

        $( ; on_clicked: $click_handler:expr $(,)? )?
        $( ; on_mouse_over: $mouse_over_handler:expr $(,)? )?
        $( ; on_mouse_out: $mouse_out_handler:expr $(,)? )?
    ) => {
        {
            // Call the function with the text argument
            let mut b = $crate::widgets::button::button($text);
            $( b = b.$prop($val); )*

            (
                b.build(),
                $( $crate::prelude::observe($click_handler), )?
                $( $crate::prelude::observe($mouse_over_handler), )?
                $( $crate::prelude::observe($mouse_out_handler), )?
            )
        }
    };
}

/// Macro for creating a [`TextBundle`].
///
/// # Example
/// ```rust
/// text_!(
///     "Hello World",
///     font_size: 20.0,
///     color: "white"
/// )
/// ```
#[macro_export]
macro_rules! text_ {
    (
        $text:expr
        $(, $prop:ident : $val:expr )* $(,)?
    ) => {
        {
            let mut b = $crate::widgets::text::text($text);
            $( b = b.$prop($val); )*

            (
                b.build(),
            )
        }
    };
}

/// Macro for creating [`CheckboxBundle`].
///
/// # Event Handlers
/// You can attach observers by using the following keys:
/// * `on_clicked` - Triggered when the checkbox is clicked.
/// * `on_mouse_over` - Triggered when the cursor enters the checkbox.
/// * `on_mouse_out` - Triggered when the cursor leaves.
/// * `on_active` - Triggered when the checkbox becomes checked.
/// * `on_inactive` - Triggered when the checkbox becomes unchecked.
///
/// # Example
/// ```rust
/// commands.spawn(checkbox_!(
///     "Enable Features",
///     id: "feature_toggle";
///
///     on_active: |active: On<Active>| {
///         println!("Enabled!");
///     },
///     on_inactive: |inactive: On<Inactive>| {
///         println!("Disabled");
///     }
/// ));
/// ```
#[macro_export]
macro_rules! checkbox_ {
    (
        $text:expr
        $(, $prop:ident : $val:expr )* $(,)?

        $( ; on_clicked: $click_handler:expr $(,)? )?
        $( ; on_mouse_over: $mouse_over_handler:expr $(,)? )?
        $( ; on_mouse_out: $mouse_out_handler:expr $(,)? )?
        $( ; on_active: $active_handler:expr $(,)? )?
        $( ; on_inactive: $inactive_handler:expr $(,)? )?
    ) => {
        {
            let mut b = $crate::widgets::checkbox::checkbox($text);
            $( b = b.$prop($val); )*

            (
                b.build(),
                $( $crate::prelude::observe($click_handler), )?
                $( $crate::prelude::observe($mouse_over_handler), )?
                $( $crate::prelude::observe($mouse_out_handler), )?
                $( $crate::prelude::observe($active_handler), )?
                $( $crate::prelude::observe($inactive_handler), )?
            )
        }
    };
}
