use crate::core::canvas::Canvas;
use crate::core::rng::Rng;

/// Cámara 2D con encuadre, zoom, rotación y vibración (shake / whip)
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub angle: f32,
    pub shake_amp: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            angle: 0.0,
            shake_amp: 0.0,
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn with_shake(mut self, amp: f32) -> Self {
        self.shake_amp = amp;
        self
    }

    /// Aplica las transformaciones de cámara al lienzo centradas en la pantalla
    pub fn apply(&self, canvas: &mut Canvas, seed: u32) {
        let cx = canvas.width as f32 / 2.0;
        let cy = canvas.height as f32 / 2.0;

        let (sx, sy) = if self.shake_amp > 0.0 {
            let mut rng = Rng::new(seed);
            (
                (rng.next_f32() - 0.5) * self.shake_amp,
                (rng.next_f32() - 0.5) * self.shake_amp,
            )
        } else {
            (0.0, 0.0)
        };

        canvas.translate(cx + sx, cy + sy);
        if self.zoom != 1.0 {
            canvas.scale(self.zoom, self.zoom);
        }
        if self.angle != 0.0 {
            canvas.rotate(self.angle);
        }
        canvas.translate(-self.x, -self.y);
    }
}
