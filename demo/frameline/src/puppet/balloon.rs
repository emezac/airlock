//! Puppet Artesanal: El Globo Aerostático de Hojas y el Árbol Otoñal (Balloon & Autumn Tree)
//! Estilo: Mosaico de hojas de retazos (*patchwork leaves*), canasta de mimbre tejida y ramas orgánicas.

use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::rng::Rng;
use std::f32::consts::PI;
use tiny_skia::{PathBuilder, Stroke};

#[derive(Clone, Debug)]
pub struct PatchworkBalloon {
    pub cx: f32,
    pub cy: f32,
    pub scale: f32,
    pub sway: f32, // Ángulo de balanceo con el viento en radianes
    pub leaf_assembly_progress: f32, // 0.0 (vacío/plano) a 1.0 (globo completamente tejido)
}

impl Default for PatchworkBalloon {
    fn default() -> Self {
        Self {
            cx: 400.0,
            cy: 400.0,
            scale: 1.0,
            sway: 0.0,
            leaf_assembly_progress: 1.0,
        }
    }
}

impl PatchworkBalloon {
    pub fn new(cx: f32, cy: f32, scale: f32) -> Self {
        Self {
            cx,
            cy,
            scale,
            ..Default::default()
        }
    }

    /// Dibuja el globo aerostático completo con envoltura de hojas, cuerdas y canasta
    pub fn draw(&self, canvas: &mut Canvas) {
        let s = self.scale;
        let p = self.leaf_assembly_progress;
        if p <= 0.01 {
            return;
        }

        canvas.save();
        canvas.translate(self.cx, self.cy);
        canvas.rotate(self.sway);

        let col_outline = Color::hex("#2a1f18");
        let col_basket_fill = Color::hex("#ebe0cb"); // Mimbre claro
        let col_basket_line = Color::hex("#6b5a45");
        let col_collar = Color::hex("#4a6b42");      // Cuello verde tejido

        let balloon_rx = 125.0 * s;
        let balloon_ry = 155.0 * s;

        // 1. Envoltura de Hojas de Mosaico (The Leaf Envelope)
        draw_leaf_mosaic_envelope(canvas, 0.0, -40.0 * s, balloon_rx, balloon_ry, s, p);

        // 2. Cuello inferior del globo (Collar)
        let collar_y = -40.0 * s + balloon_ry * 0.95;
        let collar_w = 48.0 * s * p;
        let collar_h = 16.0 * s * p;
        canvas.fill_rect(-collar_w, collar_y, collar_w * 2.0, collar_h, col_collar);
        canvas.stroke_rect(-collar_w, collar_y, collar_w * 2.0, collar_h, col_outline, 2.0 * s);

        // Trama a cuadros en el cuello
        let grid_step = 8.0 * s;
        let mut gx = -collar_w + grid_step;
        while gx < collar_w {
            canvas.stroke_line(gx, collar_y, gx, collar_y + collar_h, Color::hex("#739669"), 1.2 * s);
            gx += grid_step;
        }

        // 3. Cuerdas de Suspensión (Rigging Ropes)
        let basket_top_y = 135.0 * s;
        let basket_w_top = 110.0 * s;
        let basket_w_bot = 95.0 * s;
        let basket_h = 52.0 * s;

        let rope_alpha = p.min(1.0);
        let col_rope = col_outline.with_alpha(rope_alpha);

        // 4 Cuerdas en ángulo que sostienen la canasta
        canvas.stroke_line(-collar_w * 0.85, collar_y + collar_h, -basket_w_top * 0.85, basket_top_y, col_rope, 1.8 * s);
        canvas.stroke_line(-collar_w * 0.35, collar_y + collar_h, -basket_w_top * 0.35, basket_top_y, col_rope, 1.6 * s);
        canvas.stroke_line(collar_w * 0.35, collar_y + collar_h, basket_w_top * 0.35, basket_top_y, col_rope, 1.6 * s);
        canvas.stroke_line(collar_w * 0.85, collar_y + collar_h, basket_w_top * 0.85, basket_top_y, col_rope, 1.8 * s);

        // 4. Canasta de Mimbre Tejida (Wicker Basket)
        let mut pb_basket = PathBuilder::new();
        pb_basket.move_to(-basket_w_top, basket_top_y);
        pb_basket.line_to(basket_w_top, basket_top_y);
        pb_basket.line_to(basket_w_bot, basket_top_y + basket_h);
        pb_basket.line_to(-basket_w_bot, basket_top_y + basket_h);
        pb_basket.close();

        if let Some(path_b) = pb_basket.finish() {
            canvas.fill_path(&path_b, col_basket_fill);

            // Trama de cestería cruzada (Crosshatch Wicker)
            let b_step = 10.0 * s;
            let mut bx = -basket_w_top;
            while bx <= basket_w_top + basket_h {
                canvas.stroke_line(bx, basket_top_y, bx - basket_h * 0.6, basket_top_y + basket_h, col_basket_line, 1.0 * s);
                canvas.stroke_line(bx - basket_h * 0.6, basket_top_y, bx, basket_top_y + basket_h, col_basket_line, 1.0 * s);
                bx += b_step;
            }

            canvas.stroke_path(&path_b, col_outline, 2.4 * s);
        }

        // Ribete acolchado superior de la canasta
        canvas.fill_rect(-basket_w_top - 4.0 * s, basket_top_y - 6.0 * s, (basket_w_top + 4.0 * s) * 2.0, 10.0 * s, col_collar);
        canvas.stroke_rect(-basket_w_top - 4.0 * s, basket_top_y - 6.0 * s, (basket_w_top + 4.0 * s) * 2.0, 10.0 * s, col_outline, 2.0 * s);

        canvas.restore();
    }

    /// Dibuja la pared frontal y ribete de la canasta de mimbre por encima del personaje
    pub fn draw_basket_front(&self, canvas: &mut Canvas) {
        let s = self.scale;
        let p = self.leaf_assembly_progress;
        if p <= 0.01 {
            return;
        }

        canvas.save();
        canvas.translate(self.cx, self.cy);
        canvas.rotate(self.sway);

        let col_outline = Color::hex("#2a1f18");
        let col_collar = Color::hex("#4a6b42");
        let col_basket_line = Color::hex("#6b5a45");
        let col_basket_fill = Color::hex("#ebe0cb");

        let basket_top_y = 135.0 * s;
        let basket_w_top = 110.0 * s;
        let basket_w_bot = 95.0 * s;
        let basket_h = 52.0 * s;

        let mut pb_basket = PathBuilder::new();
        pb_basket.move_to(-basket_w_top, basket_top_y);
        pb_basket.line_to(basket_w_top, basket_top_y);
        pb_basket.line_to(basket_w_bot, basket_top_y + basket_h);
        pb_basket.line_to(-basket_w_bot, basket_top_y + basket_h);
        pb_basket.close();

        if let Some(path_b) = pb_basket.finish() {
            canvas.fill_path(&path_b, col_basket_fill);

            let b_step = 10.0 * s;
            let mut bx = -basket_w_top;
            while bx <= basket_w_top + basket_h {
                canvas.stroke_line(bx, basket_top_y, bx - basket_h * 0.6, basket_top_y + basket_h, col_basket_line, 1.0 * s);
                canvas.stroke_line(bx - basket_h * 0.6, basket_top_y, bx, basket_top_y + basket_h, col_basket_line, 1.0 * s);
                bx += b_step;
            }
            canvas.stroke_path(&path_b, col_outline, 2.4 * s);
        }

        canvas.fill_rect(-basket_w_top - 4.0 * s, basket_top_y - 6.0 * s, (basket_w_top + 4.0 * s) * 2.0, 10.0 * s, col_collar);
        canvas.stroke_rect(-basket_w_top - 4.0 * s, basket_top_y - 6.0 * s, (basket_w_top + 4.0 * s) * 2.0, 10.0 * s, col_outline, 2.0 * s);

        canvas.restore();
    }
}

/// Dibuja la envoltura de hojas de mosaico con una cascada de escamas de hojas estampadas
fn draw_leaf_mosaic_envelope(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    scale: f32,
    progress: f32,
) {
    let s = scale;
    let col_outline = Color::hex("#2a1f18");

    // Silueta aerodinámica de pera del globo
    let mut pb_balloon = PathBuilder::new();
    let steps = 48;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let ang = -PI * 0.5 + t * PI * 2.0;
        let pear = 1.0 - 0.32 * (ang.sin() + 1.0) * 0.5;
        let px = cx + ang.cos() * rx * pear;
        let py = cy + ang.sin() * ry;
        if i == 0 {
            pb_balloon.move_to(px, py);
        } else {
            pb_balloon.line_to(px, py);
        }
    }
    pb_balloon.close();

    let balloon_path = pb_balloon.finish();
    if let Some(ref path_b) = balloon_path {
        canvas.fill_path(path_b, Color::hex("#d8c6a5").with_alpha(progress));
    }

    // Paleta de retazos de hojas (Kevin Ngo leaf tapestry)
    let leaf_palette = [
        Color::hex("#3a5a40"), // Verde pino profundo
        Color::hex("#588157"), // Verde salvia
        Color::hex("#a3b18a"), // Verde té claro
        Color::hex("#bc6c25"), // Ocre canela
        Color::hex("#dda15e"), // Mostaza dorado
        Color::hex("#e07a5f"), // Terracota cálido
        Color::hex("#b84c36"), // Rojo ladrillo
        Color::hex("#3d5a80"), // Azul pizarra
        Color::hex("#98c1d9"), // Azul glaciar
        Color::hex("#4a4e69"), // Índigo grisáceo
    ];

    // Rejilla de hojas en disposición concéntrica / escalonada
    let rows = 9;
    let max_leaves = (rows * 8) as f32;
    let leaves_to_draw = (max_leaves * progress).ceil() as usize;
    let mut drawn_count = 0;

    let mut rng = Rng::new(4207);

    for r in 0..rows {
        let v_norm = r as f32 / (rows - 1) as f32; // 0.0 arriba a 1.0 abajo
        let row_y = cy - ry * 0.85 + v_norm * ry * 1.70;
        let row_rad = (1.0 - ((v_norm - 0.35) * 1.3).powi(2)).max(0.1).sqrt() * rx * 0.92;

        let leaves_in_row = (4.0 + row_rad / (18.0 * s)).round() as usize;
        let x_spacing = (row_rad * 2.0) / (leaves_in_row.max(1) as f32);

        for c in 0..leaves_in_row {
            if drawn_count >= leaves_to_draw {
                break;
            }

            let leaf_x = cx - row_rad + (c as f32 + 0.5) * x_spacing;
            let leaf_y = row_y + ((c % 2) as f32 * 6.0 - 3.0) * s;

            let col_idx = (rng.next_u32() as usize) % leaf_palette.len();
            let col = leaf_palette[col_idx];
            let leaf_scale = (16.0 + rng.next_f32() * 6.0) * s;

            draw_stylized_leaf(canvas, leaf_x, leaf_y, leaf_scale, col, col_outline, rng.next_f32() * 0.4 - 0.2);
            drawn_count += 1;
        }
    }

    // Contorno exterior nítido de la envoltura
    if let Some(ref path_b) = balloon_path {
        canvas.stroke_path(path_b, col_outline, 2.8 * s);
    }

    // Cúpula superior (capuchón dorado)
    let cap_r = 28.0 * s * progress;
    canvas.fill_circle(cx, cy - ry + 4.0 * s, cap_r, Color::hex("#f0c345"));
    canvas.stroke_circle(cx, cy - ry + 4.0 * s, cap_r, col_outline, 2.0 * s);
}

/// Dibuja una hoja individual con silueta almendrada, nervadura central y patrón
fn draw_stylized_leaf(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    line_col: Color,
    angle: f32,
) {
    canvas.save();
    canvas.translate(x, y);
    canvas.rotate(angle);

    let w = size * 0.55;
    let h = size;

    let mut pb = PathBuilder::new();
    pb.move_to(0.0, -h * 0.5);
    // Lado derecho curvado
    pb.cubic_to(w, -h * 0.25, w, h * 0.25, 0.0, h * 0.5);
    // Lado izquierdo curvado
    pb.cubic_to(-w, h * 0.25, -w, -h * 0.25, 0.0, -h * 0.5);
    pb.close();

    if let Some(path) = pb.finish() {
        canvas.fill_path(&path, color);
        canvas.stroke_path(&path, line_col, 1.4);

        // Nervadura central
        canvas.stroke_line(0.0, -h * 0.45, 0.0, h * 0.45, line_col.with_alpha(0.6), 1.1);

        // Nervaduras secundarias diagonales
        canvas.stroke_line(0.0, -h * 0.15, w * 0.45, -h * 0.05, line_col.with_alpha(0.4), 0.9);
        canvas.stroke_line(0.0, -h * 0.15, -w * 0.45, -h * 0.05, line_col.with_alpha(0.4), 0.9);
        canvas.stroke_line(0.0, h * 0.15, w * 0.45, h * 0.25, line_col.with_alpha(0.4), 0.9);
        canvas.stroke_line(0.0, h * 0.15, -w * 0.45, h * 0.25, line_col.with_alpha(0.4), 0.9);
    }

    canvas.restore();
}

// ─────────────────────────────────────────────────────────────────────────────
// Árbol Otoñal de Retazos (Autumn Tree Puppet)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct AutumnTree {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub leaves_remaining: f32, // 1.0 (lleno de hojas) a 0.0 (ramas desnudas)
}

impl AutumnTree {
    pub fn new(x: f32, y: f32, scale: f32) -> Self {
        Self {
            x,
            y,
            scale,
            leaves_remaining: 1.0,
        }
    }

    /// Dibuja el árbol orgánico con sus ramas y follaje de hojas de retazos
    pub fn draw(&self, canvas: &mut Canvas) {
        let s = self.scale;
        let col_trunk = Color::hex("#2a1f18");

        canvas.save();
        canvas.translate(self.x, self.y);

        // 1. Tronco y ramas principales desnudas
        let mut pb_trunk = PathBuilder::new();
        let tw = 18.0 * s;
        pb_trunk.move_to(-tw * 1.3, 0.0);
        pb_trunk.line_to(-tw * 0.8, -140.0 * s);
        // Rama izquierda
        pb_trunk.line_to(-85.0 * s, -220.0 * s);
        pb_trunk.line_to(-75.0 * s, -225.0 * s);
        pb_trunk.line_to(-tw * 0.2, -150.0 * s);
        // Rama central
        pb_trunk.line_to(10.0 * s, -240.0 * s);
        pb_trunk.line_to(20.0 * s, -240.0 * s);
        pb_trunk.line_to(tw * 0.2, -145.0 * s);
        // Rama derecha
        pb_trunk.line_to(85.0 * s, -205.0 * s);
        pb_trunk.line_to(80.0 * s, -195.0 * s);
        pb_trunk.line_to(tw * 0.8, -135.0 * s);
        pb_trunk.line_to(tw * 1.3, 0.0);
        pb_trunk.close();

        if let Some(path_t) = pb_trunk.finish() {
            canvas.fill_path(&path_t, col_trunk);
        }

        // Ramas secundarias finas
        canvas.stroke_line(-50.0 * s, -180.0 * s, -110.0 * s, -210.0 * s, col_trunk, 3.5 * s);
        canvas.stroke_line(-30.0 * s, -165.0 * s, -60.0 * s, -255.0 * s, col_trunk, 3.2 * s);
        canvas.stroke_line(15.0 * s, -190.0 * s, -15.0 * s, -260.0 * s, col_trunk, 3.0 * s);
        canvas.stroke_line(45.0 * s, -170.0 * s, 60.0 * s, -245.0 * s, col_trunk, 3.2 * s);
        canvas.stroke_line(55.0 * s, -160.0 * s, 115.0 * s, -190.0 * s, col_trunk, 3.0 * s);

        // 2. Hojas de retazos en la copa (si leaves_remaining > 0.0)
        let lr = self.leaves_remaining;
        if lr > 0.01 {
            let mut rng = Rng::new(9912);
            let leaf_palette = [
                Color::hex("#3a5a40"),
                Color::hex("#588157"),
                Color::hex("#dda15e"),
                Color::hex("#bc6c25"),
                Color::hex("#e07a5f"),
                Color::hex("#b84c36"),
                Color::hex("#3d5a80"),
                Color::hex("#98c1d9"),
            ];

            let total_leaves = 75;
            let count = (total_leaves as f32 * lr).round() as usize;

            for i in 0..count {
                let ang = rng.next_f32() * PI * 2.0;
                let rad = rng.next_f32().sqrt() * 115.0 * s;
                let lx = ang.cos() * rad * 1.15;
                let ly = -215.0 * s + ang.sin() * rad * 0.95;

                let col = leaf_palette[(i + rng.next_u32() as usize) % leaf_palette.len()];
                let size = (16.0 + rng.next_f32() * 6.0) * s;
                draw_stylized_leaf(canvas, lx, ly, size, col, col_trunk, rng.next_f32() * PI);
            }
        }

        canvas.restore();
    }
}
