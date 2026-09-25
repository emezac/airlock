use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::finishes::wob_path;
use tiny_skia::{Pixmap, PixmapPaint, Transform};

/// Representa un objeto real recortado con anclajes para trazar doodles encima o alrededor
pub struct CutoutPhoto {
    pub pixmap: Pixmap,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotation: f32,
}

impl CutoutPhoto {
    pub fn from_png_bytes(bytes: &[u8], x: f32, y: f32, scale: f32) -> anyhow::Result<Self> {
        let pixmap = Pixmap::decode_png(bytes)?;
        Ok(Self {
            pixmap,
            x,
            y,
            scale,
            rotation: 0.0,
        })
    }

    /// Dibuja la foto recortada sobre el lienzo
    pub fn draw(&self, canvas: &mut Canvas) {
        let pw = self.pixmap.width() as f32;
        let ph = self.pixmap.height() as f32;

        let transform = Transform::identity()
            .pre_translate(self.x, self.y)
            .pre_rotate(self.rotation.to_degrees())
            .pre_scale(self.scale, self.scale)
            .pre_translate(-pw / 2.0, -ph / 2.0);

        let paint = PixmapPaint::default();
        canvas.pixmap.draw_pixmap(
            0,
            0,
            self.pixmap.as_ref(),
            &paint,
            transform,
            None,
        );
    }

    /// Dibuja un trazo de plumilla (brush pen) o gouache blanco interactuando con el objeto
    pub fn draw_doodle_stroke(
        &self,
        canvas: &mut Canvas,
        relative_points: &[[f32; 2]],
        color: Color,
        width: f32,
        seed: u32,
    ) {
        canvas.save();
        canvas.translate(self.x, self.y);
        canvas.rotate(self.rotation);
        canvas.scale(self.scale, self.scale);

        if let Some(path) = wob_path(relative_points, 2.0, seed, false) {
            canvas.stroke_path(&path, color, width);
        }

        canvas.restore();
    }
}
