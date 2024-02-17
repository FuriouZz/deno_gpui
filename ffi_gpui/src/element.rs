use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    RgbHex(u32),
    RgbaHex(u32),
    Rgb {
        /// The red component of the color, in the range 0.0 to 1.0
        r: f32,
        /// The green component of the color, in the range 0.0 to 1.0
        g: f32,
        /// The blue component of the color, in the range 0.0 to 1.0
        b: f32,
    },
    Rgba {
        /// The red component of the color, in the range 0.0 to 1.0
        r: f32,
        /// The green component of the color, in the range 0.0 to 1.0
        g: f32,
        /// The blue component of the color, in the range 0.0 to 1.0
        b: f32,
        /// The alpha component of the color, in the range 0.0 to 1.0
        a: f32,
    },
    Hsl {
        /// Hue, in a range from 0 to 1
        h: f32,

        /// Saturation, in a range from 0 to 1
        s: f32,

        /// Lightness, in a range from 0 to 1
        l: f32,
    },
    Hsla {
        /// Hue, in a range from 0 to 1
        h: f32,

        /// Saturation, in a range from 0 to 1
        s: f32,

        /// Lightness, in a range from 0 to 1
        l: f32,

        /// Alpha, in a range from 0 to 1
        a: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Len {
    DefiniteAbsolutePixels(f32),
    DefiniteAbsoluteRems(f32),
    DefiniteFraction(f32),
    Auto(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JSElement {
    Div {
        z_index: Option<u16>,
        full: Option<bool>,
        relative: Option<bool>,
        absolute: Option<bool>,
        block: Option<bool>,
        flex: Option<bool>,
        visible: Option<bool>,
        invisible: Option<bool>,
        overflow_hidden: Option<bool>,
        overflow_x_hidden: Option<bool>,
        overflow_y_hidden: Option<bool>,
        cursor: Option<bool>,
        cursor_default: Option<bool>,
        cursor_pointer: Option<bool>,
        cursor_text: Option<bool>,
        cursor_move: Option<bool>,
        cursor_not_allowed: Option<bool>,
        cursor_context_menu: Option<bool>,
        cursor_crosshair: Option<bool>,
        cursor_vertical_text: Option<bool>,
        cursor_alias: Option<bool>,
        cursor_copy: Option<bool>,
        cursor_no_drop: Option<bool>,
        cursor_grab: Option<bool>,
        cursor_grabbing: Option<bool>,
        bg: Option<Color>,
        justify_center: Option<bool>,
        items_center: Option<bool>,
        shadow_lg: Option<bool>,
        border: Option<bool>,
        border_color: Option<Color>,
        text_xl: Option<bool>,
        text_color: Option<Color>,
        size: Option<Len>,
        children: Option<Vec<JSElement>>,
    },
    Text(String),
}

impl Render for JSElement {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.element()
    }
}

impl Into<Hsla> for Color {
    fn into(self) -> Hsla {
        match self {
            Color::RgbHex(hex) => rgb(hex).into(),
            Color::RgbaHex(hex) => rgba(hex).into(),
            Color::Rgb { r, g, b } => Rgba { r, g, b, a: 1.0 }.into(),
            Color::Rgba { r, g, b, a } => Rgba { r, b, g, a }.into(),
            Color::Hsl { h, s, l } => hsla(h, s, l, 1.0),
            Color::Hsla { h, s, l, a } => hsla(h, s, l, a),
        }
    }
}

impl Into<Fill> for Color {
    fn into(self) -> Fill {
        let c: Hsla = self.into();
        Fill::Color(c)
    }
}

impl Into<Length> for Len {
    fn into(self) -> Length {
        match self {
            Len::Auto(..) => Length::Auto,
            Len::DefiniteFraction(frac) => Length::Definite(DefiniteLength::Fraction(frac)),
            Len::DefiniteAbsolutePixels(px) => {
                Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(px))))
            }
            Len::DefiniteAbsoluteRems(rems) => {
                Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Rems(Rems(rems))))
            }
        }
    }
}

impl JSElement {
    pub fn element(&self) -> AnyElement {
        match self {
            JSElement::Div {
                flex,
                bg,
                justify_center,
                items_center,
                shadow_lg,
                border,
                border_color,
                text_xl,
                text_color,
                children,
                size,
                ..
            } => {
                let mut d = div();

                if let Some(true) = flex {
                    d = d.flex();
                }

                if let Some(color) = bg {
                    d = d.bg(color.clone());
                }

                if let Some(true) = justify_center {
                    d = d.justify_center();
                }

                if let Some(true) = items_center {
                    d = d.items_center();
                }

                if let Some(true) = shadow_lg {
                    d = d.shadow_lg();
                }

                if let Some(true) = border {
                    d = d.border();
                }

                if let Some(color) = border_color {
                    d = d.border_color(color.clone());
                }

                if let Some(true) = text_xl {
                    d = d.text_xl();
                }

                if let Some(color) = text_color {
                    d = d.text_color(color.clone());
                }

                if let Some(children) = children {
                    d = d.children(children.iter().map(|el| el.element()));
                }

                if let Some(len) = size {
                    d = d.size(len.clone());
                }

                d.into_any_element()
            }
            JSElement::Text(text) => text.clone().into_any_element(),
        }
    }
}
