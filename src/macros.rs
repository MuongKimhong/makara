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
    ([ $($child:expr),* $(,)? ]) => { $crate::scroll_!(; [ $($child),* ]) };
    (on: $handler:expr $(; on: $more:expr)* $(; [ $($child:expr),* $(,)? ])?) => { $crate::scroll_!(; on: $handler $(; on: $more)* $(; [ $($child),* ])?) };
    ($( $prop:ident : $val:expr ),* $(; on: $handler:expr )* $(; [ $($child:expr),* $(,)? ])?) => {
        {
            let mut s = $crate::widgets::scroll::scroll();
            $( s = s.$prop($val); )*
            (s.build(), $( $crate::prelude::observe($handler), )* $( ::bevy::prelude::children![ $($child),* ] )?)
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
    ([ $($child:expr),* $(,)? ]) => { $crate::row_!(; [ $($child),* ]) };
    (on: $handler:expr $(; on: $more:expr)* $(; [ $($child:expr),* $(,)? ])?) => { $crate::row_!(; on: $handler $(; on: $more)* $(; [ $($child),* ])?) };
    ($( $prop:ident : $val:expr ),* $(; on: $handler:expr )* $(; [ $($child:expr),* $(,)? ])?) => {
        {
            let mut s = $crate::widgets::row::row();
            $( s = s.$prop($val); )*
            (s.build(), $( $crate::prelude::observe($handler), )* $( ::bevy::prelude::children![ $($child),* ] )?)
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
    ([ $($child:expr),* $(,)? ]) => { $crate::column_!(; [ $($child),* ]) };
    (on: $handler:expr $(; on: $more:expr)* $(; [ $($child:expr),* $(,)? ])?) => { $crate::column_!(; on: $handler $(; on: $more)* $(; [ $($child),* ])?) };
    ($( $prop:ident : $val:expr ),* $(; on: $handler:expr )* $(; [ $($child:expr),* $(,)? ])?) => {
        {
            let mut s = $crate::widgets::column::column();
            $( s = s.$prop($val); )*
            (s.build(), $( $crate::prelude::observe($handler), )* $( ::bevy::prelude::children![ $($child),* ] )?)
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
    ([ $($child:expr),* $(,)? ]) => { $crate::root_!(; [ $($child),* ]) };
    (on: $handler:expr $(; on: $more:expr)* $(; [ $($child:expr),* $(,)? ])?) => { $crate::root_!(; on: $handler $(; on: $more)* $(; [ $($child),* ])?) };
    ($( $prop:ident : $val:expr ),* $(; on: $handler:expr )* $(; [ $($child:expr),* $(,)? ])?) => {
        {
            let mut s = $crate::widgets::root::root();
            $( s = s.$prop($val); )*
            (s.build(), $( $crate::prelude::observe($handler), )* $( ::bevy::prelude::children![ $($child),* ] )?)
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
    ($text:expr $(, $prop:ident : $val:expr )* $(; on: $handler:expr )* $(; [ $($child:expr),* $(,)? ])?) => {
        {
            let mut b = $crate::widgets::button::button($text);
            $( b = b.$prop($val); )*
            (b.build(), $( $crate::prelude::observe($handler), )* $( ::bevy::prelude::children![ $($child),* ] )?)
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
    ($text:expr $(, $prop:ident : $val:expr )* $(; on: $handler:expr )*) => {
        {
            let mut t = $crate::widgets::text::text($text);
            $( t = t.$prop($val); )*
            (t.build(), $( $crate::prelude::observe($handler), )*)
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

/// Macro for creating [`CircularBundle`].
///
/// # Event Handlers
/// * `MouseOver`
/// * `MouseOut`
/// * `Change<T>`
///
/// # Example
/// ```rust
/// circular_!(
///     color: "blue",
///     percent: 23.0;
///
///     on: |change: On<Change<f32>>| {
///         println!("at {:?} percent", change.data);
///     }
/// );
/// ```
#[macro_export]
macro_rules! circular_ {
    (
        $( $prop:ident : $val:expr ),* $(,)?

        // Events
        $( ; on: $handler:expr )*
    ) => {
        {
            let mut c = $crate::widgets::circular::circular();
            $( c = c.$prop($val); )*

            (
                c.build(),
                $( $crate::prelude::observe($handler), )*
            )
        }
    };
}

/// Macro for creating [`ProgressBarBundle`].
///
/// # Event Handlers
/// * `MouseOver`
/// * `MouseOut`
/// * `Change<T>`
///
/// # Example
/// ```rust
/// progress_bar_!(
///     color: "blue",
///     percent: 23.0;
///
///     on: |change: On<Change<f32>>| {
///         println!("at {:?} percent", change.data);
///     }
/// );
/// ```
#[macro_export]
macro_rules! progress_bar_ {
    (
        $( $prop:ident : $val:expr ),* $(,)?

        $( ; on: $handler:expr )*
    ) => {
        {
            let mut c = $crate::widgets::progress_bar::progress_bar();
            $( c = c.$prop($val); )*

            (
                c.build(),
                $( $crate::prelude::observe($handler), )*
            )
        }
    };
}

/// Macro for creating [`DropdownBundle`].
///
/// # Event Handlers
/// * `Clicked`
/// * `MouseOver`
/// * `MouseOut`
/// * `Active<T>`
/// * `Inactive<T>`
///
/// # Example
/// ```rust
/// dropdown_!(
///     "Click me to show option";
///
///     on: |active: On<Active<bool>>| {
///         println!("options shown");
///     };
///
///     [
///         button_!("Option 1"),
///         button_!("Option 2"),
///         button_!("Option 3"),
///     ]
/// );
/// ```
#[macro_export]
macro_rules! dropdown_ {
    ($text:expr $(, $prop:ident : $val:expr )* $(; on: $handler:expr )* $(; [ $($child:expr),* $(,)? ])?) => {
        {
            let mut d = $crate::widgets::dropdown::dropdown($text);
            $( d = d.$prop($val); )*
            (d.build(), $( $crate::prelude::observe($handler), )* $( ::bevy::prelude::children![ $($child),* ] )?)
        }
    };
}

/// Macro for creating a [`ImageBundle`].
///
/// # Event Handlers
/// * `Clicked`
/// * `MouseOver`
/// * `MouseOut`
///
/// # Example
/// ```rust
/// image_!(
///     "dog.png";
///
///     on: |clicked: On<Clicked>| {
///         println!("image clicked!")
///     }
/// );
/// ```
#[macro_export]
macro_rules! image_ {
    ($path:expr $(, $prop:ident : $val:expr )* $(; on: $handler:expr )*) => {
        {
            let mut i = $crate::widgets::image::image($path);
            $( i = i.$prop($val); )*
            (i.build(), $( $crate::prelude::observe($handler), )*)
        }
    };
}

/// Macro for creating a [`LinkBundle`].
///
/// # Event Handlers
/// * `Clicked`
/// * `MouseOver`
/// * `MouseOut`
///
/// # Example
/// ```rust
/// link_!(
///     "https://github.com/MuongKimhong/makara",
///     font_size: 20.0,
///     color: "white"
/// )
/// ```
#[macro_export]
macro_rules! link_ {
    ($path:expr $(, $prop:ident : $val:expr )* $(; on: $handler:expr )*) => {
        {
            let mut i = $crate::widgets::link::link($path);
            $( i = i.$prop($val); )*
            (i.build(), $( $crate::prelude::observe($handler), )*)
        }
    };
}

/// Macro for creating a [`SliderBundle`].
///
/// # Event Handlers
/// * `MouseOver`
/// * `MouseOut`
/// * `Change<T>
///
/// # Example
/// ```rust
/// slider_!(
///     min: 0.0, max: 10.0;
///
///     on: |change: On<Change<f32>>| {
///         println!("value {:?}", change.data);
///     }
/// )
/// ```
#[macro_export]
macro_rules! slider_ {
    (min: $min:expr, max: $max:expr $(, $prop:ident : $val:expr )* $(; on: $handler:expr )*) => {
        {
            let mut i = $crate::widgets::slider::slider($min, $max);
            $( i = i.$prop($val); )*
            (i.build(), $( $crate::prelude::observe($handler), )*)
        }
    };
}

/// Macro for creating [`RadioBundle`].
///
/// # Event Handlers
/// * `Clicked`
/// * `MouseOver`
/// * `MouseOut`
/// * `Active<T>`
/// * `Inactive<T>`
///
/// # Example
/// ```rust
/// radio_!(
///     "Option 1";
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
macro_rules! radio_ {
    (
        $text:expr
        $(, $prop:ident : $val:expr )* $(,)?

        $( ; on: $handler:expr )* $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut c = $crate::widgets::radio::radio($text);
            $( c = c.$prop($val); )*

            (
                c.build(),
                $( $crate::prelude::observe($handler), )*
            )
        }
    };
}

#[macro_export]
macro_rules! radio_group_ {
    // Only children (no semicolon, no properties)
    // radio_group_!([ child1 ])
    ( [ $($child:expr),* $(,)? ] ) => {
        {
            let s = $crate::widgets::radio::radio_group();
            (s.build(), ::bevy::prelude::children![ $($child),* ])
        }
    };

    // No properties, but starts with 'on:'
    // radio_group_!(on: my_sys ; [ child1 ])
    ( on: $handler:expr $( ; on: $more_handlers:expr )* $( ; [ $($child:expr),* $(,)? ] )? ) => {
        {
            let s = $crate::widgets::radio::radio_group();
            (
                s.build(),
                $crate::prelude::observe($handler),
                $( $crate::prelude::observe($more_handlers), )*
                $( ::bevy::prelude::children![ $($child),* ] )?
            )
        }
    };

    // Properties, then optional observers and children
    // radio_group_!(spacing: 10.0 ; on: my_sys ; [ child1 ])
    (
        $prop:ident : $val:expr $(, $rest_prop:ident : $rest_val:expr )* $(,)?
        $( ; on: $handler:expr )*
        $( ; [ $($child:expr),* $(,)? ] )?
    ) => {
        {
            let mut s = $crate::widgets::radio::radio_group();
            s = s.$prop($val);
            $( s = s.$rest_prop($rest_val); )*

            (
                s.build(),
                $( $crate::prelude::observe($handler), )*
                $( ::bevy::prelude::children![ $($child),* ] )?
            )
        }
    };
}

/// Macro for creating a [`TextInputBundle`].
///
/// # Event Handlers
/// * `Clicked`
/// * `MouseOver`
/// * `MouseOut`
/// * `Change<T>`
///
/// # Example
/// ```rust
/// text_input_!(
///     "Enter your name";
///
///     on: |change: On<Change<String>>| {
///         println!("text input value {:?}", change.data);
///     }
/// );
/// ```
#[macro_export]
macro_rules! text_input_ {
    ($text:expr $(, $prop:ident : $val:expr )* $(; on: $handler:expr )*) => {
        {
            let mut b = $crate::widgets::text_input::text_input($text);
            $( b = b.$prop($val); )*

            (
                b.build(),
                $( $crate::prelude::observe($handler), )*
            )
        }
    };
}

/// Macro for creating a [`SelectBundle`].
///
/// # Event Handlers
/// * `Clicked`
/// * `MouseOver`
/// * `MouseOut`
/// * `Change<T>`
///
/// # Example
/// ```rust
/// select_!(
///     "Select a choice",
///     choices: &["Choice 1", "Choice 2", "Choice 2"]
///
///     on: |change: On<Change<String>>| {
///         println!("select value {:?}", change.data);
///     }
/// );
/// ```
#[macro_export]
macro_rules! select_ {
    // 1. Placeholder + Choices + Optional Props + Observers
    (
        $placeholder:expr,
        choices: $choices:expr
        $(, $prop:ident : $val:expr )* $(,)?
        $( ; on: $handler:expr )* ) => {
        {
            let mut i = $crate::widgets::select::select($placeholder, $choices);
            $( i = i.$prop($val); )*

            (
                i.build(),
                $( $crate::prelude::observe($handler), )*
            )
        }
    };

    // 2. Placeholder + Choices + Observers (Skipping props)
    (
        $placeholder:expr,
        choices: $choices:expr
        ; on: $handler:expr $( ; on: $more:expr )*
    ) => {
        $crate::select_!($placeholder, choices: $choices ; on: $handler $( ; on: $more )*)
    };
}

/// Macro for creating a [`ModalBundle`]. ID is required for modal to make it works.
///
/// Modal need to be spawned independently, as it's not part of UI heirarchy.
///
/// # Event Handlers
/// * `Active<T>`
/// * `Inactive<T>`
///
/// # Example
/// ```rust
/// commands.spawn(modal_!(
///     id: "my-modal"
///
///     on: |active: On<Active<String>>| {
///         println!("modal with id {:?} is active", active.data);
///     };
///
///     [ text_!("This is modal content") ]
/// ));
/// ```
#[macro_export]
macro_rules! modal_ {
    ([ $($child:expr),* $(,)? ]) => {
        $crate::modal_!(; [ $($child),* ])
    };
    (on: $handler:expr $(; on: $more:expr)* $(; [ $($child:expr),* $(,)? ])?) => {
        $crate::modal_!(; on: $handler $(; on: $more)* $(; [ $($child),* ])?)
    };
    ($( $prop:ident : $val:expr ),* $(; on: $handler:expr )* $(; [ $($child:expr),* $(,)? ])?) => {
        {
            let mut s = $crate::widgets::modal::modal();
            $( s = s.$prop($val); )*
            (s.build(), $( $crate::prelude::observe($handler), )* $( ::bevy::prelude::children![ $($child),* ] )?)
        }
    };
}
