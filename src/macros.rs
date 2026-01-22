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
        // Properties
        $( $prop:ident : $val:expr ),* $(,)?

        // '*' means "0 or more"
        $( ; on: $handler:expr )*

        // '?' means "0 or 1"
        $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut s = $crate::widgets::scroll::scroll();
            $( s = s.$prop($val); )*

            (
                s.build(),
                $( $crate::prelude::observe($handler), )*

                $(
                    ::bevy::prelude::children![ $($child),* ]
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
/// * `Clicked`
/// * `MouseOver`
/// * `MouseOut`
///
/// # Example
/// ```rust
/// button_!(
///     "Click Me",
///     background_color: "red";
///
///     on: |clicked: On<Clicked>| {
///         println!("Button clicked!")
///     }
/// );
/// ```
#[macro_export]
macro_rules! button_ {
    (
        $text:expr
        $(, $prop:ident : $val:expr )* $(,)?

        $( ; on: $handler:expr )* $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut b = $crate::widgets::button::button($text);
            $( b = b.$prop($val); )*

            (
                b.build(),
                $( $crate::prelude::observe($handler), )*
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
            let mut t = $crate::widgets::text::text($text);
            $( t = t.$prop($val); )*

            (
                t.build(),
            )
        }
    };
}

/// Macro for creating [`CheckboxBundle`].
///
/// # Event Handlers
/// `checkbox` emits following events:
/// * `Clicked`
/// * `MouseOver`
/// * `MouseOut`
/// * `Active<T>`
/// * `Inactive<T>`
///
/// # Example
/// ```rust
/// checkbox_!(
///     "Enable Features",
///     id: "feature_toggle";
///
///     on: |active: On<Active<String>>| {
///         println!("Enabled!");
///     },
///     on: |inactive: On<Inactive<String>>| {
///         println!("Disabled");
///     }
/// );
/// ```
#[macro_export]
macro_rules! checkbox_ {
    (
        $text:expr
        $(, $prop:ident : $val:expr )* $(,)?

        $( ; on: $handler:expr )* $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut c = $crate::widgets::checkbox::checkbox($text);
            $( c = c.$prop($val); )*

            (
                c.build(),
                $( $crate::prelude::observe($handler), )*
            )
        }
    };
}
