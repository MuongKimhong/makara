use bevy::prelude::*;

pub trait IntoColor {
    fn into_color(self) -> Color;
}

impl IntoColor for Color {
    fn into_color(self) -> Color {
        self
    }
}

impl IntoColor for &str {
    fn into_color(self) -> Color {
        if self.starts_with("#") {
            match Srgba::hex(self) {
                Ok(srgba) => return Color::Srgba(srgba),
                Err(_) => return Color::NONE
            }
        }

        match self.to_lowercase().as_str() {
            "transparent"    => Color::NONE,
            "black"          => Color::BLACK,
            "white"          => Color::WHITE,
            "red"            => Color::srgb(1.0, 0.0, 0.0),
            "lime"           => Color::srgb(0.0, 1.0, 0.0),
            "blue"           => Color::srgb(0.0, 0.0, 1.0),
            "green"          => Color::srgb(0.0, 1.0, 0.0),
            "yellow"         => Color::srgb(1.0, 1.0, 0.0),
            "cyan" | "aqua"  => Color::srgb(0.0, 1.0, 1.0),
            "magenta" | "fuchsia" => Color::srgb(1.0, 0.0, 1.0),
            "maroon"         => Color::srgb(0.5, 0.0, 0.0),
            "purple"         => Color::srgb(0.5, 0.0, 0.5),

            "aliceblue"      => Color::srgb(240.0/255.0, 248.0/255.0, 255.0/255.0),
            "antiquewhite"   => Color::srgb(250.0/255.0, 235.0/255.0, 215.0/255.0),
            "aquamarine"     => Color::srgb(127.0/255.0, 255.0/255.0, 212.0/255.0),
            "azure"          => Color::srgb(240.0/255.0, 255.0/255.0, 255.0/255.0),
            "beige"          => Color::srgb(245.0/255.0, 245.0/255.0, 220.0/255.0),
            "bisque"         => Color::srgb(255.0/255.0, 228.0/255.0, 196.0/255.0),
            "blanchedalmond" => Color::srgb(255.0/255.0, 235.0/255.0, 205.0/255.0),
            "blueviolet"     => Color::srgb(138.0/255.0, 43.0/255.0, 226.0/255.0),
            "brown"          => Color::srgb(165.0/255.0, 42.0/255.0, 42.0/255.0),
            "burlywood"      => Color::srgb(222.0/255.0, 184.0/255.0, 135.0/255.0),
            "cadetblue"      => Color::srgb(95.0/255.0, 158.0/255.0, 160.0/255.0),
            "chartreuse"     => Color::srgb(127.0/255.0, 255.0/255.0, 0.0),
            "chocolate"      => Color::srgb(210.0/255.0, 105.0/255.0, 30.0/255.0),
            "coral"          => Color::srgb(255.0/255.0, 127.0/255.0, 80.0/255.0),
            "cornflowerblue" => Color::srgb(100.0/255.0, 149.0/255.0, 237.0/255.0),
            "cornsilk"       => Color::srgb(255.0/255.0, 248.0/255.0, 220.0/255.0),
            "crimson"        => Color::srgb(220.0/255.0, 20.0/255.0, 60.0/255.0),
            "darkblue"       => Color::srgb(0.0, 0.0, 139.0/255.0),
            "darkcyan"       => Color::srgb(0.0, 139.0/255.0, 139.0/255.0),
            "darkgoldenrod"  => Color::srgb(184.0/255.0, 134.0/255.0, 11.0/255.0),

            "deeppink"       => Color::srgb(255.0/255.0, 20.0/255.0, 147.0/255.0),
            "deepskyblue"    => Color::srgb(0.0, 191.0/255.0, 255.0/255.0),
            "dimgray"        => Color::srgb(105.0/255.0, 105.0/255.0, 105.0/255.0),
            "dodgerblue"     => Color::srgb(30.0/255.0, 144.0/255.0, 255.0/255.0),
            "firebrick"      => Color::srgb(178.0/255.0, 34.0/255.0, 34.0/255.0),
            "floralwhite"    => Color::srgb(255.0/255.0, 250.0/255.0, 240.0/255.0),
            "forestgreen"    => Color::srgb(34.0/255.0, 139.0/255.0, 34.0/255.0),
            "gainsboro"      => Color::srgb(220.0/255.0, 220.0/255.0, 220.0/255.0),
            "ghostwhite"     => Color::srgb(248.0/255.0, 248.0/255.0, 255.0/255.0),
            "gold"           => Color::srgb(1.0, 215.0/255.0, 0.0),
            "goldenrod"      => Color::srgb(218.0/255.0, 165.0/255.0, 32.0/255.0),
            "greenyellow"    => Color::srgb(173.0/255.0, 255.0/255.0, 47.0/255.0),
            "honeydew"       => Color::srgb(240.0/255.0, 255.0/255.0, 240.0/255.0),
            "hotpink"        => Color::srgb(255.0/255.0, 105.0/255.0, 180.0/255.0),
            "indianred"      => Color::srgb(205.0/255.0, 92.0/255.0, 92.0/255.0),
            "indigo"         => Color::srgb(75.0/255.0, 0.0, 130.0/255.0),
            "khaki"          => Color::srgb(240.0/255.0, 230.0/255.0, 140.0/255.0),
            "lavender"       => Color::srgb(230.0/255.0, 230.0/255.0, 250.0/255.0),
            "lawngreen"      => Color::srgb(124.0/255.0, 252.0/255.0, 0.0),
            "lemonchiffon"   => Color::srgb(255.0/255.0, 250.0/255.0, 205.0/255.0),

            "darkgreen"      => Color::srgb(0.0, 100.0/255.0, 0.0),
            "darkolivegreen" => Color::srgb(85.0/255.0, 107.0/255.0, 47.0/255.0),
            "darkseagreen"   => Color::srgb(143.0/255.0, 188.0/255.0, 143.0/255.0),
            "darkturquoise"  => Color::srgb(0.0, 206.0/255.0, 209.0/255.0),
            "emerald"        => Color::srgb(16.0/255.0, 185.0/255.0, 129.0/255.0),
            "lightgreen"     => Color::srgb(144.0/255.0, 238.0/255.0, 144.0/255.0),
            "lightseagreen"  => Color::srgb(32.0/255.0, 178.0/255.0, 170.0/255.0),
            "limegreen"      => Color::srgb(50.0/255.0, 205.0/255.0, 50.0/255.0),
            "mediumaquamarine" => Color::srgb(102.0/255.0, 205.0/255.0, 170.0/255.0),
            "mediumseagreen"   => Color::srgb(60.0/255.0, 179.0/255.0, 113.0/255.0),
            "mediumspringgreen"=> Color::srgb(0.0, 250.0/255.0, 154.0/255.0),
            "mediumturquoise"  => Color::srgb(72.0/255.0, 209.0/255.0, 204.0/255.0),
            "olive"          => Color::srgb(128.0/255.0, 128.0/255.0, 0.0),
            "olivedrab"      => Color::srgb(107.0/255.0, 142.0/255.0, 35.0/255.0),
            "teal"           => Color::srgb(0.0, 128.0/255.0, 128.0/255.0),

            "darkorchid"     => Color::srgb(153.0/255.0, 50.0/255.0, 204.0/255.0),
            "darkpurple"     => Color::srgb(48.0/255.0, 25.0/255.0, 52.0/255.0),
            "darkviolet"     => Color::srgb(148.0/255.0, 0.0, 211.0/255.0),
            "lavenderblush"  => Color::srgb(255.0/255.0, 240.0/255.0, 245.0/255.0),
            "lightpink"      => Color::srgb(255.0/255.0, 182.0/255.0, 193.0/255.0),
            "mediumorchid"   => Color::srgb(186.0/255.0, 85.0/255.0, 211.0/255.0),
            "mediumpurple"   => Color::srgb(147.0/255.0, 112.0/255.0, 219.0/255.0),
            "mediumvioletred"=> Color::srgb(199.0/255.0, 21.0/255.0, 133.0/255.0),
            "orchid"         => Color::srgb(218.0/255.0, 112.0/255.0, 214.0/255.0),
            "palevioletred"  => Color::srgb(219.0/255.0, 112.0/255.0, 147.0/255.0),
            "plum"           => Color::srgb(221.0/255.0, 160.0/255.0, 221.0/255.0),
            "rebeccapurple"  => Color::srgb(102.0/255.0, 51.0/255.0, 153.0/255.0),
            "thistle"        => Color::srgb(216.0/255.0, 191.0/255.0, 216.0/255.0),
            "violet"         => Color::srgb(238.0/255.0, 130.0/255.0, 238.0/255.0),

            "darkslateblue"  => Color::srgb(72.0/255.0, 61.0/255.0, 139.0/255.0),
            "lightblue"      => Color::srgb(173.0/255.0, 216.0/255.0, 230.0/255.0),
            "lightcyan"      => Color::srgb(224.0/255.0, 255.0/255.0, 255.0/255.0),
            "lightskyblue"   => Color::srgb(135.0/255.0, 206.0/255.0, 250.0/255.0),
            "mediumblue"     => Color::srgb(0.0, 0.0, 205.0/255.0),
            "mediumslateblue"=> Color::srgb(123.0/255.0, 104.0/255.0, 238.0/255.0),
            "midnightblue"   => Color::srgb(25.0/255.0, 25.0/255.0, 112.0/255.0),
            "navy"           => Color::srgb(0.0, 0.0, 128.0/255.0),
            "powderblue"     => Color::srgb(176.0/255.0, 224.0/255.0, 230.0/255.0),
            "royalblue"      => Color::srgb(65.0/255.0, 105.0/255.0, 225.0/255.0),

            "darkgray"       => Color::srgb(169.0/255.0, 169.0/255.0, 169.0/255.0),
            "darkslategray"  => Color::srgb(47.0/255.0, 79.0/255.0, 79.0/255.0),
            "gray" | "grey"  => Color::srgb(128.0/255.0, 128.0/255.0, 128.0/255.0),
            "lightgray"      => Color::srgb(211.0/255.0, 211.0/255.0, 211.0/255.0),
            "lightslategray" => Color::srgb(119.0/255.0, 136.0/255.0, 153.0/255.0),
            "silver"         => Color::srgb(192.0/255.0, 192.0/255.0, 192.0/255.0),
            "slategray"      => Color::srgb(112.0/255.0, 128.0/255.0, 144.0/255.0),
            "snow"           => Color::srgb(255.0/255.0, 250.0/255.0, 250.0/255.0),
            "ivory"          => Color::srgb(255.0/255.0, 255.0/255.0, 240.0/255.0),
            "whitepulse"     => Color::srgb(0.95, 0.95, 0.95),

            "lightgoldenrodyellow" => Color::srgb(250.0/255.0, 250.0/255.0, 210.0/255.0),
            "lightyellow"     => Color::srgb(255.0/255.0, 255.0/255.0, 224.0/255.0),
            "lightsalmon"     => Color::srgb(255.0/255.0, 160.0/255.0, 122.0/255.0),
            "lightcoral"      => Color::srgb(240.0/255.0, 128.0/255.0, 128.0/255.0),
            "lightsteelblue"  => Color::srgb(176.0/255.0, 196.0/255.0, 222.0/255.0),
            "palegreen"       => Color::srgb(152.0/255.0, 251.0/255.0, 152.0/255.0),
            "paleturquoise"   => Color::srgb(175.0/255.0, 238.0/255.0, 238.0/255.0),
            "papayawhip"      => Color::srgb(255.0/255.0, 239.0/255.0, 213.0/255.0),
            "peachpuff"       => Color::srgb(255.0/255.0, 218.0/255.0, 185.0/255.0),

            "mistyrose"       => Color::srgb(255.0/255.0, 228.0/255.0, 225.0/255.0),
            "moccasin"        => Color::srgb(255.0/255.0, 228.0/255.0, 181.0/255.0),
            "navajowhite"     => Color::srgb(255.0/255.0, 222.0/255.0, 173.0/255.0),
            "oldlace"         => Color::srgb(253.0/255.0, 245.0/255.0, 230.0/255.0),
            "peru"            => Color::srgb(205.0/255.0, 133.0/255.0, 63.0/255.0),
            "rosybrown"       => Color::srgb(188.0/255.0, 143.0/255.0, 143.0/255.0),
            "saddlebrown"     => Color::srgb(139.0/255.0, 69.0/255.0, 19.0/255.0),
            "salmon"          => Color::srgb(250.0/255.0, 128.0/255.0, 114.0/255.0),
            "sandybrown"      => Color::srgb(244.0/255.0, 164.0/255.0, 96.0/255.0),
            "seagreen"        => Color::srgb(46.0/255.0, 139.0/255.0, 87.0/255.0),
            "seashell"        => Color::srgb(255.0/255.0, 245.0/255.0, 238.0/255.0),
            "sienna"          => Color::srgb(160.0/255.0, 82.0/255.0, 45.0/255.0),
            "steelblue"       => Color::srgb(70.0/255.0, 130.0/255.0, 180.0/255.0),
            "tan"             => Color::srgb(210.0/255.0, 180.0/255.0, 140.0/255.0),
            "wheat"           => Color::srgb(245.0/255.0, 222.0/255.0, 179.0/255.0),
            "whitesmoke"      => Color::srgb(245.0/255.0, 245.0/255.0, 245.0/255.0),
            "yellowgreen"     => Color::srgb(154.0/255.0, 205.0/255.0, 50.0/255.0),

            "slate-50"  => Color::srgb(0.97, 0.98, 1.0),
            "slate-100" => Color::srgb(0.95, 0.96, 0.98),
            "slate-200" => Color::srgb(0.89, 0.91, 0.94),
            "slate-300" => Color::srgb(0.79, 0.83, 0.88),
            "slate-400" => Color::srgb(0.58, 0.65, 0.73),
            "slate-500" => Color::srgb(0.39, 0.45, 0.55),
            "slate-600" => Color::srgb(0.28, 0.33, 0.41),
            "slate-700" => Color::srgb(0.2, 0.23, 0.29),
            "slate-800" => Color::srgb(0.12, 0.15, 0.19),
            "slate-900" => Color::srgb(0.06, 0.07, 0.1),

            "sky-100" => Color::srgb(0.88, 0.95, 1.0),
            "sky-300" => Color::srgb(0.49, 0.82, 0.99),
            "sky-500" => Color::srgb(0.02, 0.61, 0.91),
            "sky-700" => Color::srgb(0.01, 0.41, 0.62),
            "sky-900" => Color::srgb(0.05, 0.18, 0.28),

            "rose-100" => Color::srgb(1.0, 0.95, 0.96),
            "rose-300" => Color::srgb(1.0, 0.65, 0.71),
            "rose-500" => Color::srgb(0.96, 0.26, 0.38),
            "rose-700" => Color::srgb(0.75, 0.1, 0.22),
            "rose-900" => Color::srgb(0.31, 0.05, 0.11),

            "emerald-100" => Color::srgb(0.82, 0.97, 0.91),
            "emerald-500" => Color::srgb(0.06, 0.72, 0.5),
            "emerald-900" => Color::srgb(0.02, 0.18, 0.13),

            _ => Color::NONE,
        }
    }
}

/// Helper trait to allow .set() on &mut BackgroundColor, &mut BorderColor and &mut TextColor
pub trait SetColor {
    fn set(&mut self, color: impl IntoColor);
}

impl SetColor for BackgroundColor {
    fn set(&mut self, color: impl IntoColor) {
        self.0 = color.into_color();
    }
}

impl SetColor for TextColor {
    fn set(&mut self, color: impl IntoColor) {
        self.0 = color.into_color();
    }
}

impl SetColor for BorderColor {
    fn set(&mut self, color: impl IntoColor) {
        let color = color.into_color();
        self.top = color;
        self.bottom = color;
        self.left = color;
        self.right = color;
    }
}

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::srgb(r, g, b)
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::srgba(r, g, b, a)
}
