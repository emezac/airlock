use crate::core::canvas::Canvas;
use crate::core::color::Palette;
use crate::core::finishes::{draw_crayon, draw_surface, wob_path};
use tiny_skia::PathBuilder;

/// Títere del barco de papel de origami (personaje icónico de Kevin Ngo)
pub struct PaperBoat {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub bob_angle: f32,
}

impl PaperBoat {
    pub fn new(x: f32, y: f32, scale: f32) -> Self {
        Self {
            x,
            y,
            scale,
            bob_angle: 0.0,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, palette: &Palette, seed: u32) {
        canvas.save();
        canvas.translate(self.x, self.y);
        canvas.rotate(self.bob_angle);
        canvas.scale(self.scale, self.scale);

        // 1. Casco: Cara izquierda del barco
        let hull_left = [
            [-90.0, 10.0],
            [-30.0, 50.0],
            [0.0, 50.0],
            [0.0, 10.0],
        ];
        let mut pb = PathBuilder::new();
        pb.move_to(-90.0, 10.0);
        pb.line_to(-30.0, 50.0);
        pb.line_to(0.0, 50.0);
        pb.line_to(0.0, 10.0);
        pb.close();
        if let Some(path) = pb.finish() {
            let col = palette.fills.get(0).copied().unwrap_or(palette.paper);
            canvas.fill_path(&path, col);
            draw_surface(canvas, &path, [-90.0, 10.0, 90.0, 40.0], palette, seed + 1);
        }
        if let Some(stroke_path) = wob_path(&hull_left, 1.8, seed + 2, true) {
            canvas.stroke_path(&stroke_path, palette.ink, 1.8);
        }

        // 2. Casco: Cara derecha del barco (ligeramente más en sombra)
        let hull_right = [
            [0.0, 10.0],
            [0.0, 50.0],
            [40.0, 50.0],
            [90.0, 10.0],
        ];
        let mut pb = PathBuilder::new();
        pb.move_to(0.0, 10.0);
        pb.line_to(0.0, 50.0);
        pb.line_to(40.0, 50.0);
        pb.line_to(90.0, 10.0);
        pb.close();
        if let Some(path) = pb.finish() {
            let col = palette.fills.get(1).copied().unwrap_or(palette.shade);
            canvas.fill_path(&path, col);
            draw_surface(canvas, &path, [0.0, 10.0, 90.0, 40.0], palette, seed + 3);
        }
        if let Some(stroke_path) = wob_path(&hull_right, 1.8, seed + 4, true) {
            canvas.stroke_path(&stroke_path, palette.ink, 1.8);
        }

        // 3. Vela mayor (triángulo izquierdo)
        let sail_left = [
            [0.0, 5.0],
            [-60.0, 5.0],
            [0.0, -80.0],
        ];
        let mut pb = PathBuilder::new();
        pb.move_to(0.0, 5.0);
        pb.line_to(-60.0, 5.0);
        pb.line_to(0.0, -80.0);
        pb.close();
        if let Some(path) = pb.finish() {
            let col = palette.paper.tint(0.5);
            canvas.fill_path(&path, col);
            draw_surface(canvas, &path, [-60.0, -80.0, 60.0, 85.0], palette, seed + 5);
        }
        if let Some(stroke_path) = wob_path(&sail_left, 1.6, seed + 6, true) {
            canvas.stroke_path(&stroke_path, palette.ink, 1.6);
        }

        // 4. Vela menor (triángulo derecho)
        let sail_right = [
            [0.0, 5.0],
            [0.0, -70.0],
            [45.0, 5.0],
        ];
        let mut pb = PathBuilder::new();
        pb.move_to(0.0, 5.0);
        pb.line_to(0.0, -70.0);
        pb.line_to(45.0, 5.0);
        pb.close();
        if let Some(path) = pb.finish() {
            let col = palette.fills.get(2).copied().unwrap_or(palette.light);
            canvas.fill_path(&path, col);
            draw_surface(canvas, &path, [0.0, -70.0, 45.0, 75.0], palette, seed + 7);
        }
        if let Some(stroke_path) = wob_path(&sail_right, 1.6, seed + 8, true) {
            canvas.stroke_path(&stroke_path, palette.ink, 1.6);
        }

        // 5. Ondas de agua en la base estilo crayón / rizo
        let ripple1 = [[-110.0, 58.0], [-40.0, 56.0], [30.0, 60.0], [100.0, 57.0]];
        let ripple2 = [[-80.0, 68.0], [-10.0, 70.0], [70.0, 67.0]];
        let ripple_col = palette.inks.first().copied().unwrap_or(palette.ink);

        draw_crayon(canvas, &ripple1, ripple_col, 2.2, seed + 9, false);
        draw_crayon(canvas, &ripple2, ripple_col, 1.8, seed + 10, false);

        canvas.restore();
    }
}
