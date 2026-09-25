use serde::{Deserialize, Serialize};

/// Representación de color RGBA con componentes en rango [0.0, 1.0].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Parsea un color en formato hexadecimal "#rrggbb" o "#rrggbbaa"
    pub fn hex(hex_str: &str) -> Self {
        let s = hex_str.trim().trim_start_matches('#');
        let (r, g, b, a) = match s.len() {
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                (r, g, b, 255)
            }
            8 => {
                let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
                (r, g, b, a)
            }
            3 => {
                let r = u8::from_str_radix(&s[0..1].repeat(2), 16).unwrap_or(0);
                let g = u8::from_str_radix(&s[1..2].repeat(2), 16).unwrap_or(0);
                let b = u8::from_str_radix(&s[2..3].repeat(2), 16).unwrap_or(0);
                (r, g, b, 255)
            }
            _ => (0, 0, 0, 255),
        };
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha.clamp(0.0, 1.0),
        }
    }

    pub fn to_tiny_skia(&self) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba(self.r, self.g, self.b, self.a)
            .unwrap_or(tiny_skia::Color::BLACK)
    }

    pub fn to_u8_array(&self) -> [u8; 4] {
        [
            (self.r.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.g.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.b.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.a.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }

    /// Interpolación lineal entre dos colores
    pub fn mix(&self, other: Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Multiplicación sustractiva (simulación de sobreimpresión risográfica)
    pub fn multiply(&self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: (self.a * other.a).max(self.a).max(other.a),
        }
    }

    /// Aclara hacia blanco (tint)
    pub fn tint(&self, amount: f32) -> Self {
        self.mix(Color::rgb(1.0, 1.0, 1.0), amount)
    }

    /// Oscurece hacia negro (shade)
    pub fn shade(&self, amount: f32) -> Self {
        self.mix(Color::rgb(0.0, 0.0, 0.0), amount)
    }
}

/// Acabados de superficie según emezac/tools
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinishKind {
    Ink,
    Riso,
    Screen,
    Pencil,
    Doodle,
    Flat,
}

/// Esquema completo de paleta según `references/palettes.md`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub finish: FinishKind,
    pub paper: Color,
    pub paper_band: Option<Color>,
    pub ink: Color,
    pub night: Color,
    pub chalk: Color,
    pub chalk_dim: Color,
    pub guide: Color,
    pub fills: Vec<Color>,
    pub shade: Color,
    pub light: Color,
    pub blush: Color,
    pub accents: Vec<Color>,
    pub inks: Vec<Color>,
}

impl Palette {
    /// Preset 1: Ink on warm paper (estilo 'The life of a fruit fly')
    pub fn paper_ink() -> Self {
        Self {
            name: "paperInk".into(),
            finish: FinishKind::Ink,
            paper: Color::hex("#f3e6cf"),
            paper_band: Some(Color::rgba(1.0, 0.933, 0.784, 0.65)),
            ink: Color::hex("#1e1630"),
            night: Color::hex("#0b0d1f"),
            chalk: Color::hex("#e8ecff"),
            chalk_dim: Color::hex("#8a9acb"),
            guide: Color::rgba(0.274, 0.392, 1.0, 0.55),
            fills: vec![
                Color::hex("#d9a566"),
                Color::hex("#c87d55"),
                Color::hex("#8c5e43"),
                Color::hex("#5c3e2e"),
            ],
            shade: Color::hex("#3a251b"),
            light: Color::hex("#fff7e6"),
            blush: Color::hex("#d95f53"),
            accents: vec![
                Color::hex("#ff3366"),
                Color::hex("#ffcc00"),
                Color::hex("#00ccff"),
                Color::hex("#33ff99"),
            ],
            inks: vec![
                Color::hex("#1e1630"),
                Color::hex("#c8473f"),
                Color::hex("#2b5fb8"),
            ],
        }
    }

    /// Preset 2: Riso halftone prints (estilo 'Riso flipbook')
    pub fn riso_pop() -> Self {
        Self {
            name: "risoPop".into(),
            finish: FinishKind::Riso,
            paper: Color::hex("#f0ece2"),
            paper_band: None,
            ink: Color::hex("#22366b"),
            night: Color::hex("#2a2050"),
            chalk: Color::hex("#ffe800"),
            chalk_dim: Color::hex("#ff85c2"),
            guide: Color::rgba(0.0, 0.47, 0.75, 0.4),
            fills: vec![
                Color::hex("#0078bf"), // Cyan Riso
                Color::hex("#ff48b0"), // Fluorescent Pink Riso
                Color::hex("#ffe800"), // Yellow Riso
                Color::hex("#22366b"), // Federal Blue Riso
            ],
            shade: Color::hex("#1c2b54"),
            light: Color::hex("#fffdf0"),
            blush: Color::hex("#ff48b0"),
            accents: vec![
                Color::hex("#ff48b0"),
                Color::hex("#ffe800"),
                Color::hex("#0078bf"),
                Color::hex("#00ffaa"),
            ],
            inks: vec![
                Color::hex("#0078bf"), // Placa 1: Azul
                Color::hex("#ff48b0"), // Placa 2: Rosa
                Color::hex("#ffe800"), // Placa 3: Amarillo
                Color::hex("#22366b"), // Placa 4: Azul marino
            ],
        }
    }

    /// Preset 3: Flat screen prints (estilo 'Paper boat')
    pub fn screen_sea() -> Self {
        Self {
            name: "screenSea".into(),
            finish: FinishKind::Screen,
            paper: Color::hex("#e8e6db"),
            paper_band: None,
            ink: Color::hex("#051630"),
            night: Color::hex("#1a1c2e"),
            chalk: Color::hex("#e8c84a"),
            chalk_dim: Color::hex("#738ca6"),
            guide: Color::rgba(0.04, 0.31, 0.51, 0.35),
            fills: vec![
                Color::hex("#0a5083"),
                Color::hex("#1e78a6"),
                Color::hex("#e8c84a"),
                Color::hex("#d95f38"),
            ],
            shade: Color::hex("#051630"),
            light: Color::hex("#ffffff"),
            blush: Color::hex("#d95f38"),
            accents: vec![
                Color::hex("#e8c84a"),
                Color::hex("#d95f38"),
                Color::hex("#1e78a6"),
                Color::hex("#ffffff"),
            ],
            inks: vec![
                Color::hex("#0a5083"),
                Color::hex("#051630"),
                Color::hex("#e8c84a"),
            ],
        }
    }

    /// Preset 4: Graphite minimalism (estilo 'Personal website')
    pub fn pencil_minimal() -> Self {
        Self {
            name: "pencilMinimal".into(),
            finish: FinishKind::Pencil,
            paper: Color::hex("#f4efe4"),
            paper_band: None,
            ink: Color::hex("#201f1b"),
            night: Color::hex("#27251f"),
            chalk: Color::hex("#ded9ce"),
            chalk_dim: Color::hex("#918e84"),
            guide: Color::rgba(0.4, 0.4, 0.35, 0.3),
            fills: vec![
                Color::hex("#d9d1c1"),
                Color::hex("#b8b09f"),
                Color::hex("#8a8a55"),
                Color::hex("#5c594f"),
            ],
            shade: Color::hex("#201f1b"),
            light: Color::hex("#faf7f0"),
            blush: Color::hex("#c2a18a"),
            accents: vec![
                Color::hex("#8a8a55"),
                Color::hex("#5c594f"),
                Color::hex("#201f1b"),
                Color::hex("#e6deb8"),
            ],
            inks: vec![
                Color::hex("#201f1b"),
                Color::hex("#8a8a55"),
            ],
        }
    }

    /// Preset 5: Doodles on cut-out photos of real objects
    pub fn doodle_pastel(pastel_name: &str) -> Self {
        let paper = match pastel_name.to_lowercase().as_str() {
            "mint" => Color::hex("#d3e6d9"),
            "butter" => Color::hex("#efe4b3"),
            "sky" => Color::hex("#d2dee8"),
            "cream" => Color::hex("#ebe5d4"),
            "peach" => Color::hex("#eeccb4"),
            "lilac" => Color::hex("#ded4e9"),
            "sand" => Color::hex("#c9b07e"),
            "night" => Color::hex("#383750"),
            _ => Color::hex("#efd2d1"), // Rose por defecto
        };
        Self {
            name: format!("doodlePastel_{}", pastel_name),
            finish: FinishKind::Doodle,
            paper,
            paper_band: None,
            ink: Color::hex("#23202b"),
            night: Color::hex("#2c2f5e"),
            chalk: Color::hex("#ffffff"),
            chalk_dim: Color::hex("#a2a6d4"),
            guide: Color::rgba(0.91, 0.31, 0.36, 0.4),
            fills: vec![
                Color::hex("#e8505b"),
                Color::hex("#455a64"),
                Color::hex("#f3c68f"),
                Color::hex("#7aa5d2"),
            ],
            shade: Color::hex("#1b1822"),
            light: Color::hex("#ffffff"),
            blush: Color::hex("#e8505b"),
            accents: vec![
                Color::hex("#ffffff"), // Gouache blanco
                Color::hex("#e8505b"),
                Color::hex("#ffcc00"),
                Color::hex("#23202b"),
            ],
            inks: vec![
                Color::hex("#23202b"),
                Color::hex("#e8505b"),
            ],
        }
    }

    /// Preset 6: Blueprint interlude (chalk on navy)
    pub fn blueprint_night() -> Self {
        Self {
            name: "blueprintNight".into(),
            finish: FinishKind::Ink,
            paper: Color::hex("#0b0d1f"),
            paper_band: None,
            ink: Color::hex("#e8ecff"),
            night: Color::hex("#0b0d1f"),
            chalk: Color::hex("#7fe7ff"),
            chalk_dim: Color::hex("#3a7b99"),
            guide: Color::rgba(0.2, 0.5, 0.8, 0.35),
            fills: vec![
                Color::hex("#121736"),
                Color::hex("#19224d"),
                Color::hex("#212e66"),
            ],
            shade: Color::hex("#070814"),
            light: Color::hex("#7fe7ff"),
            blush: Color::hex("#5c7cfa"),
            accents: vec![
                Color::hex("#7fe7ff"),
                Color::hex("#e8ecff"),
                Color::hex("#38d9a9"),
                Color::hex("#ffd43b"),
            ],
            inks: vec![
                Color::hex("#e8ecff"),
                Color::hex("#7fe7ff"),
            ],
        }
    }
}
