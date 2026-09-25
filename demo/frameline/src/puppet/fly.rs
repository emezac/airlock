use crate::core::canvas::Canvas;
use crate::core::color::Palette;
use crate::core::effects::{draw_xray_fly, XRayParams};
use crate::core::finishes::{draw_surface, wob_path};
use crate::core::primitives::{draw_construction, draw_hex_lattice, draw_scribble, ellipse_points};
use tiny_skia::PathBuilder;
use std::f32::consts::PI;

/// Títere de la mosca de la fruta ('The life of a fruit fly')
pub struct FruitFly {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub wing_flap: f32,
    pub tilt: f32,
}

impl FruitFly {
    pub fn new(x: f32, y: f32, scale: f32) -> Self {
        Self {
            x,
            y,
            scale,
            wing_flap: 0.0,
            tilt: 0.0,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, palette: &Palette, seed: u32, show_guides: bool) {
        canvas.save();
        canvas.translate(self.x, self.y);
        canvas.rotate(self.tilt);
        canvas.scale(self.scale, self.scale);

        // 1. Líneas de construcción técnica detrás
        if show_guides {
            draw_construction(canvas, 0.0, 0.0, 110.0, seed + 1, palette.guide, 0.45);
        }

        // 2. Patas (detrás del cuerpo)
        let leg_pts = [
            [-30.0, -10.0], [-60.0, -25.0], [-80.0, -15.0],
            [30.0, -10.0], [60.0, -25.0], [80.0, -15.0],
            [-25.0, 15.0], [-55.0, 35.0], [-75.0, 45.0],
            [25.0, 15.0], [55.0, 35.0], [75.0, 45.0],
        ];
        for i in (0..leg_pts.len()).step_by(3) {
            let pts = &leg_pts[i..i+3];
            if let Some(path) = wob_path(pts, 1.5, seed + 2 + i as u32, false) {
                canvas.stroke_path(&path, palette.ink, 1.4);
            }
        }

        // 3. Abdomen (elipse inferior alargada)
        let abd_pts = ellipse_points(0.0, 35.0, 26.0, 42.0, 28);
        let mut pb = PathBuilder::new();
        for (i, p) in abd_pts.iter().enumerate() {
            if i == 0 { pb.move_to(p[0], p[1]); } else { pb.line_to(p[0], p[1]); }
        }
        pb.close();
        if let Some(path) = pb.finish() {
            let fill = palette.fills.get(1).copied().unwrap_or(palette.paper);
            canvas.fill_path(&path, fill);
            draw_surface(canvas, &path, [-26.0, -7.0, 52.0, 84.0], palette, seed + 10);
            if let Some(stroke) = wob_path(&abd_pts, 1.6, seed + 11, true) {
                canvas.stroke_path(&stroke, palette.ink, 1.8);
            }
        }

        // 4. Tórax (elipse central)
        let thx_pts = ellipse_points(0.0, -10.0, 22.0, 20.0, 24);
        let mut pb = PathBuilder::new();
        for (i, p) in thx_pts.iter().enumerate() {
            if i == 0 { pb.move_to(p[0], p[1]); } else { pb.line_to(p[0], p[1]); }
        }
        pb.close();
        if let Some(path) = pb.finish() {
            let fill = palette.fills.get(0).copied().unwrap_or(palette.shade);
            canvas.fill_path(&path, fill);
            draw_surface(canvas, &path, [-22.0, -30.0, 44.0, 40.0], palette, seed + 20);
            if let Some(stroke) = wob_path(&thx_pts, 1.5, seed + 21, true) {
                canvas.stroke_path(&stroke, palette.ink, 1.8);
            }
        }

        // Garabato acentuado de energía sobre el tórax
        draw_scribble(canvas, 0.0, -10.0, 18.0, &palette.accents, seed + 25);

        // 5. Cabeza y ojos
        let head_pts = ellipse_points(0.0, -38.0, 18.0, 14.0, 20);
        let mut pb = PathBuilder::new();
        for (i, p) in head_pts.iter().enumerate() {
            if i == 0 { pb.move_to(p[0], p[1]); } else { pb.line_to(p[0], p[1]); }
        }
        pb.close();
        if let Some(path) = pb.finish() {
            canvas.fill_path(&path, palette.ink.tint(0.2));
            if let Some(stroke) = wob_path(&head_pts, 1.4, seed + 30, true) {
                canvas.stroke_path(&stroke, palette.ink, 1.6);
            }
        }

        // Ojos de celosía hexagonal
        draw_hex_lattice(canvas, -13.0, -40.0, 10.0, seed + 35, palette.blush);
        draw_hex_lattice(canvas, 13.0, -40.0, 10.0, seed + 36, palette.blush);

        // 6. Alas translúcidas con batimiento
        let wing_angle = (self.wing_flap * PI * 2.0).sin() * 0.45;

        // Ala izquierda
        canvas.save();
        canvas.translate(-14.0, -12.0);
        canvas.rotate(-0.5 + wing_angle);
        let w1 = ellipse_points(-45.0, -10.0, 48.0, 18.0, 24);
        let mut pb = PathBuilder::new();
        for (i, p) in w1.iter().enumerate() {
            if i == 0 { pb.move_to(p[0], p[1]); } else { pb.line_to(p[0], p[1]); }
        }
        pb.close();
        if let Some(path) = pb.finish() {
            canvas.fill_path(&path, palette.light.with_alpha(0.55));
            if let Some(stroke) = wob_path(&w1, 1.4, seed + 40, true) {
                canvas.stroke_path(&stroke, palette.ink.with_alpha(0.8), 1.2);
            }
        }
        canvas.restore();

        // Ala derecha
        canvas.save();
        canvas.translate(14.0, -12.0);
        canvas.rotate(0.5 - wing_angle);
        let w2 = ellipse_points(45.0, -10.0, 48.0, 18.0, 24);
        let mut pb = PathBuilder::new();
        for (i, p) in w2.iter().enumerate() {
            if i == 0 { pb.move_to(p[0], p[1]); } else { pb.line_to(p[0], p[1]); }
        }
        pb.close();
        if let Some(path) = pb.finish() {
            canvas.fill_path(&path, palette.light.with_alpha(0.55));
            if let Some(stroke) = wob_path(&w2, 1.4, seed + 41, true) {
                canvas.stroke_path(&stroke, palette.ink.with_alpha(0.8), 1.2);
            }
        }
        canvas.restore();

        canvas.restore();
    }

    /// Dibuja la mosca completa + capa X-Ray superpuesta con `reveal` (0.0–1.0).
    ///
    /// `reveal = 0.0` → solo la mosca normal.  
    /// `reveal = 1.0` → la mosca + esqueleto X-Ray a plena intensidad.
    pub fn draw_with_xray(
        &self,
        canvas: &mut Canvas,
        palette: &Palette,
        seed: u32,
        show_guides: bool,
        reveal: f32,
    ) {
        // Primero dibujamos la mosca normal
        self.draw(canvas, palette, seed, show_guides);

        // Luego superponemos la capa X-Ray en coordenadas de mundo
        if reveal > 0.01 {
            let params = XRayParams {
                cx: self.x,
                cy: self.y,
                scale: self.scale,
                reveal,
                // usa el chalk de la paleta para armonía de color
                bone_color: palette.chalk,
                seed: seed + 100,
            };
            draw_xray_fly(canvas, &params);
        }
    }
}
