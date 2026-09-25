use crate::core::color::Color;
use tiny_skia::{
    FillRule, GradientStop, LineCap, LineJoin, Paint, Path, PathBuilder,
    Pixmap, Point, SpreadMode, Stroke, Transform,
};

/// Estado guardado de la pila de transformaciones y estilos (save / restore)
#[derive(Clone, Debug)]
struct CanvasState {
    transform: Transform,
    global_alpha: f32,
    blend_mode: tiny_skia::BlendMode,
    clip_mask: Option<tiny_skia::Mask>,
}

/// Lienzo 2D acelerado por CPU basado en tiny-skia con semántica tipo Canvas2D
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixmap: Pixmap,
    current_transform: Transform,
    global_alpha: f32,
    blend_mode: tiny_skia::BlendMode,
    current_clip: Option<tiny_skia::Mask>,
    states: Vec<CanvasState>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixmap = Pixmap::new(width, height).expect("No se pudo asignar memoria para Pixmap");
        Self {
            width,
            height,
            pixmap,
            current_transform: Transform::identity(),
            global_alpha: 1.0,
            blend_mode: tiny_skia::BlendMode::SourceOver,
            current_clip: None,
            states: Vec::new(),
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.pixmap.fill(color.to_tiny_skia());
    }

    pub fn save(&mut self) {
        self.states.push(CanvasState {
            transform: self.current_transform,
            global_alpha: self.global_alpha,
            blend_mode: self.blend_mode,
            clip_mask: self.current_clip.clone(),
        });
    }

    pub fn restore(&mut self) {
        if let Some(state) = self.states.pop() {
            self.current_transform = state.transform;
            self.global_alpha = state.global_alpha;
            self.blend_mode = state.blend_mode;
            self.current_clip = state.clip_mask;
        }
    }

    pub fn reset_transform(&mut self) {
        self.current_transform = Transform::identity();
    }

    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.current_transform = self.current_transform.pre_translate(tx, ty);
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.current_transform = self.current_transform.pre_scale(sx, sy);
    }

    pub fn rotate(&mut self, radians: f32) {
        self.current_transform = self.current_transform.pre_rotate(radians.to_degrees());
    }

    pub fn set_global_alpha(&mut self, alpha: f32) {
        self.global_alpha = alpha.clamp(0.0, 1.0);
    }

    pub fn set_blend_mode(&mut self, mode: tiny_skia::BlendMode) {
        self.blend_mode = mode;
    }

    pub fn clip_path(&mut self, path: &Path) {
        if let Some(ref mut mask) = self.current_clip {
            mask.intersect_path(
                path,
                FillRule::Winding,
                true,
                self.current_transform,
            );
        } else {
            let mut mask = tiny_skia::Mask::new(self.width, self.height)
                .expect("Error al crear máscara de recorte");
            mask.fill_path(
                path,
                FillRule::Winding,
                true,
                self.current_transform,
            );
            self.current_clip = Some(mask);
        }
    }

    pub fn fill_path(&mut self, path: &Path, color: Color) {
        let mut paint = Paint::default();
        let skia_color = color.with_alpha(color.a * self.global_alpha).to_tiny_skia();
        paint.set_color(skia_color);
        paint.blend_mode = self.blend_mode;
        paint.anti_alias = true;

        let mask_ref = self.current_clip.as_ref();
        self.pixmap.fill_path(
            path,
            &paint,
            FillRule::Winding,
            self.current_transform,
            mask_ref,
        );
    }

    pub fn stroke_path(&mut self, path: &Path, color: Color, width: f32) {
        let mut paint = Paint::default();
        let skia_color = color.with_alpha(color.a * self.global_alpha).to_tiny_skia();
        paint.set_color(skia_color);
        paint.blend_mode = self.blend_mode;
        paint.anti_alias = true;

        let stroke = Stroke {
            width,
            miter_limit: 4.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            dash: None,
        };

        let mask_ref = self.current_clip.as_ref();
        self.pixmap.stroke_path(
            path,
            &paint,
            &stroke,
            self.current_transform,
            mask_ref,
        );
    }

    /// Trazo de precisión tipo aguafuerte / grabado en cobre (extremos planos/cuadrados, esquinas afiladas Miter).
    /// Evita el engrosamiento abultado de extremos redondos en líneas de alta resolución.
    pub fn stroke_path_fine(&mut self, path: &Path, color: Color, width: f32) {
        let mut paint = Paint::default();
        let skia_color = color.with_alpha(color.a * self.global_alpha).to_tiny_skia();
        paint.set_color(skia_color);
        paint.blend_mode = self.blend_mode;
        paint.anti_alias = true;

        let stroke = Stroke {
            width,
            miter_limit: 20.0,
            line_cap: LineCap::Square,
            line_join: LineJoin::Miter,
            dash: None,
        };

        let mask_ref = self.current_clip.as_ref();
        self.pixmap.stroke_path(
            path,
            &paint,
            &stroke,
            self.current_transform,
            mask_ref,
        );
    }

    /// Dibuja una línea directa entre dos puntos con extremos vivos de precisión
    pub fn stroke_line_fine(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, width: f32) {
        let mut pb = PathBuilder::new();
        pb.move_to(x1, y1);
        pb.line_to(x2, y2);
        if let Some(path) = pb.finish() {
            self.stroke_path_fine(&path, color, width);
        }
    }

    /// Dibuja una línea directa entre dos puntos
    pub fn stroke_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, width: f32) {
        let mut pb = PathBuilder::new();
        pb.move_to(x1, y1);
        pb.line_to(x2, y2);
        if let Some(path) = pb.finish() {
            self.stroke_path(&path, color, width);
        }
    }

    /// Dibuja un círculo relleno
    pub fn fill_circle(&mut self, cx: f32, cy: f32, r: f32, color: Color) {
        let mut pb = PathBuilder::new();
        pb.push_circle(cx, cy, r);
        if let Some(path) = pb.finish() {
            self.fill_path(&path, color);
        }
    }

    /// Dibuja un círculo con contorno
    pub fn stroke_circle(&mut self, cx: f32, cy: f32, r: f32, color: Color, width: f32) {
        let mut pb = PathBuilder::new();
        pb.push_circle(cx, cy, r);
        if let Some(path) = pb.finish() {
            self.stroke_path(&path, color, width);
        }
    }

    /// Dibuja un rectángulo relleno
    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let (rx, rw) = if w < 0.0 { (x + w, -w) } else { (x, w) };
        let (ry, rh) = if h < 0.0 { (y + h, -h) } else { (y, h) };
        if let Some(rect) = tiny_skia::Rect::from_xywh(rx, ry, rw, rh) {
            let mut pb = PathBuilder::new();
            pb.push_rect(rect);
            if let Some(path) = pb.finish() {
                self.fill_path(&path, color);
            }
        }
    }

    /// Dibuja un rectángulo con contorno
    pub fn stroke_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color, width: f32) {
        let (rx, rw) = if w < 0.0 { (x + w, -w) } else { (x, w) };
        let (ry, rh) = if h < 0.0 { (y + h, -h) } else { (y, h) };
        if let Some(rect) = tiny_skia::Rect::from_xywh(rx, ry, rw, rh) {
            let mut pb = PathBuilder::new();
            pb.push_rect(rect);
            if let Some(path) = pb.finish() {
                self.stroke_path(&path, color, width);
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    // Gradientes
    // ─────────────────────────────────────────────────────────────────────

    /// Rellena un rectángulo completo con un gradiente lineal.
    ///
    /// * `x1, y1` – punto de inicio del gradiente
    /// * `x2, y2` – punto de fin del gradiente
    /// * `stops`  – lista de `(posición [0..1], Color)`
    pub fn fill_linear_gradient(
        &mut self,
        x1: f32, y1: f32,
        x2: f32, y2: f32,
        stops: &[(f32, Color)],
    ) {
        if stops.len() < 2 { return; }
        let gradient_stops: Vec<GradientStop> = stops
            .iter()
            .filter_map(|(pos, c)| {
                tiny_skia::Color::from_rgba(c.r, c.g, c.b, c.a * self.global_alpha)
                    .map(|sc| GradientStop::new(pos.clamp(0.0, 1.0), sc))
            })
            .collect();

        if let Some(shader) = tiny_skia::LinearGradient::new(
            Point::from_xy(x1, y1),
            Point::from_xy(x2, y2),
            gradient_stops,
            SpreadMode::Pad,
            self.current_transform,
        ) {
            let mut paint = Paint::default();
            paint.shader = shader;
            paint.blend_mode = self.blend_mode;
            paint.anti_alias = true;

            let w = self.width as f32;
            let h = self.height as f32;
            if let Some(rect) = tiny_skia::Rect::from_xywh(0.0, 0.0, w, h) {
                self.pixmap.fill_rect(rect, &paint, Transform::identity(), self.current_clip.as_ref());
            }
        }
    }

    /// Rellena un rectángulo completo con un gradiente radial.
    ///
    /// * `cx, cy` – centro del gradiente
    /// * `radius` – radio del gradiente en px
    /// * `stops`  – lista de `(posición [0..1], Color)`
    pub fn fill_radial_gradient(
        &mut self,
        cx: f32, cy: f32,
        radius: f32,
        stops: &[(f32, Color)],
    ) {
        if stops.len() < 2 { return; }
        let gradient_stops: Vec<GradientStop> = stops
            .iter()
            .filter_map(|(pos, c)| {
                tiny_skia::Color::from_rgba(c.r, c.g, c.b, c.a * self.global_alpha)
                    .map(|sc| GradientStop::new(pos.clamp(0.0, 1.0), sc))
            })
            .collect();

        if let Some(shader) = tiny_skia::RadialGradient::new(
            Point::from_xy(cx, cy),
            Point::from_xy(cx, cy),
            radius.max(1.0),
            gradient_stops,
            SpreadMode::Pad,
            self.current_transform,
        ) {
            let mut paint = Paint::default();
            paint.shader = shader;
            paint.blend_mode = self.blend_mode;
            paint.anti_alias = true;

            let w = self.width as f32;
            let h = self.height as f32;
            if let Some(rect) = tiny_skia::Rect::from_xywh(0.0, 0.0, w, h) {
                self.pixmap.fill_rect(rect, &paint, Transform::identity(), self.current_clip.as_ref());
            }
        }
    }

    /// Rellena un path con un gradiente lineal.
    pub fn fill_path_linear_gradient(
        &mut self,
        path: &Path,
        x1: f32, y1: f32,
        x2: f32, y2: f32,
        stops: &[(f32, Color)],
    ) {
        if stops.len() < 2 { return; }
        let gradient_stops: Vec<GradientStop> = stops
            .iter()
            .filter_map(|(pos, c)| {
                tiny_skia::Color::from_rgba(c.r, c.g, c.b, c.a * self.global_alpha)
                    .map(|sc| GradientStop::new(pos.clamp(0.0, 1.0), sc))
            })
            .collect();

        if let Some(shader) = tiny_skia::LinearGradient::new(
            Point::from_xy(x1, y1),
            Point::from_xy(x2, y2),
            gradient_stops,
            SpreadMode::Pad,
            self.current_transform,
        ) {
            let mut paint = Paint::default();
            paint.shader = shader;
            paint.blend_mode = self.blend_mode;
            paint.anti_alias = true;
            self.pixmap.fill_path(
                path,
                &paint,
                FillRule::Winding,
                self.current_transform,
                self.current_clip.as_ref(),
            );
        }
    }

    /// Mezcla `other` sobre este canvas con `alpha` de acumulación.
    /// Usado para motion blur: `composite_over(prev, 0.25)` retiene 25% del frame anterior.
    pub fn composite_over(&mut self, other: &Canvas, alpha: f32) {
        let alpha = alpha.clamp(0.0, 1.0);
        let a_byte = (alpha * 255.0).round() as u8;
        let src = other.pixmap.data();
        let dst = self.pixmap.data_mut();
        let len = src.len().min(dst.len());

        for i in (0..len).step_by(4) {
            // Mezcla alfa ponderada sin depender de blend mode de skia
            let sr = src[i]   as u32;
            let sg = src[i+1] as u32;
            let sb = src[i+2] as u32;
            let sa = src[i+3] as u32;

            let dr = dst[i]   as u32;
            let dg = dst[i+1] as u32;
            let db = dst[i+2] as u32;
            let da = dst[i+3] as u32;

            let inv = 255 - a_byte as u32;
            dst[i]   = ((sr * a_byte as u32 + dr * inv) / 255).min(255) as u8;
            dst[i+1] = ((sg * a_byte as u32 + dg * inv) / 255).min(255) as u8;
            dst[i+2] = ((sb * a_byte as u32 + db * inv) / 255).min(255) as u8;
            dst[i+3] = ((sa * a_byte as u32 + da * inv) / 255).min(255) as u8;
        }
    }

    /// Dibuja una imagen RGBA sobre el lienzo redimensionándola si es necesario
    pub fn draw_rgba_image(&mut self, img: &image::RgbaImage) {
        if img.width() == self.width && img.height() == self.height {
            self.pixmap.data_mut().copy_from_slice(img.as_raw());
        } else {
            let resized = image::imageops::resize(
                img,
                self.width,
                self.height,
                image::imageops::FilterType::Triangle,
            );
            self.pixmap.data_mut().copy_from_slice(resized.as_raw());
        }
    }

    /// Dibuja un archivo de imagen (JPEG o PNG) directamente sobre el lienzo
    pub fn draw_image_file(&mut self, path: &str) -> anyhow::Result<()> {
        let img = image::open(path)?.to_rgba8();
        self.draw_rgba_image(&img);
        Ok(())
    }

    /// Guarda la imagen como PNG
    pub fn save_png(&self, path: &str) -> anyhow::Result<()> {
        self.pixmap.save_png(path).map_err(|e| anyhow::anyhow!("{:?}", e))
    }

    /// Codifica en memoria a bytes PNG
    pub fn encode_png(&self) -> anyhow::Result<Vec<u8>> {
        self.pixmap.encode_png().map_err(|e| anyhow::anyhow!("{:?}", e))
    }

    /// Retorna el slice de bytes RGBA crudos para alimentar directamente la tubería de video
    pub fn data(&self) -> &[u8] {
        self.pixmap.data()
    }
}

