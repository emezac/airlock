use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::finishes::wob_path;
use crate::core::rng::Rng;
use std::f32::consts::PI;

/// Genera los puntos perimetrales de una elipse para trazar con `wob`
pub fn ellipse_points(cx: f32, cy: f32, rx: f32, ry: f32, count: usize) -> Vec<[f32; 2]> {
    let mut pts = Vec::with_capacity(count);
    for i in 0..count {
        let theta = (i as f32 / count as f32) * PI * 2.0;
        pts.push([cx + rx * theta.cos(), cy + ry * theta.sin()]);
    }
    pts
}

/// Genera los puntos de un rectángulo con esquinas redondeadas
pub fn round_rect_points(x: f32, y: f32, w: f32, h: f32, r: f32) -> Vec<[f32; 2]> {
    let r = r.min(w / 2.0).min(h / 2.0);
    let mut pts = Vec::new();
    let arc_steps = 6;

    // Esquina superior derecha
    for i in 0..=arc_steps {
        let a = -PI * 0.5 + (i as f32 / arc_steps as f32) * (PI * 0.5);
        pts.push([x + w - r + r * a.cos(), y + r + r * a.sin()]);
    }
    // Esquina inferior derecha
    for i in 0..=arc_steps {
        let a = 0.0 + (i as f32 / arc_steps as f32) * (PI * 0.5);
        pts.push([x + w - r + r * a.cos(), y + h - r + r * a.sin()]);
    }
    // Esquina inferior izquierda
    for i in 0..=arc_steps {
        let a = PI * 0.5 + (i as f32 / arc_steps as f32) * (PI * 0.5);
        pts.push([x + r + r * a.cos(), y + h - r + r * a.sin()]);
    }
    // Esquina superior izquierda
    for i in 0..=arc_steps {
        let a = PI + (i as f32 / arc_steps as f32) * (PI * 0.5);
        pts.push([x + r + r * a.cos(), y + r + r * a.sin()]);
    }

    pts
}

/// Líneas de construcción técnica (construction lines)
pub fn draw_construction(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    radius: f32,
    seed: u32,
    color: Color,
    alpha: f32,
) {
    let mut rng = Rng::new(seed);
    let col = color.with_alpha(alpha);

    // Círculo guía
    let pts = ellipse_points(cx, cy, radius, radius, 36);
    if let Some(path) = wob_path(&pts, 1.2, seed, true) {
        canvas.stroke_path(&path, col, 0.8);
    }

    // Cruces axiales
    let extend = radius * 1.35;
    canvas.stroke_line(cx - extend, cy, cx + extend, cy, col, 0.7);
    canvas.stroke_line(cx, cy - extend, cx, cy + extend, col, 0.7);

    // Rayos radiales a 45 grados aleatorios
    let rays = 2 + (rng.next_f32() * 3.0) as usize;
    for _i in 0..rays {
        let angle = rng.next_f32() * PI * 2.0;
        let r_len = radius * (0.8 + rng.next_f32() * 0.5);
        let x2 = cx + angle.cos() * r_len;
        let y2 = cy + angle.sin() * r_len;
        canvas.stroke_line(cx, cy, x2, y2, col.with_alpha(alpha * 0.6), 0.6);

        // Pequeño marcador en la punta
        canvas.fill_circle(x2, y2, 2.0, col);
    }
}

/// Rejilla hexagonal para ojos de insecto o indicadores de alta tecnología
pub fn draw_hex_lattice(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    radius: f32,
    seed: u32,
    color: Color,
) {
    let cell_size = 5.5;
    let mut rng = Rng::new(seed);
    let col = color.with_alpha(0.85);

    let rows = (radius / cell_size) as i32;
    for q in -rows..=rows {
        for r in -rows..=rows {
            let px = cx + cell_size * (3.0_f32.sqrt() * q as f32 + 3.0_f32.sqrt() / 2.0 * r as f32);
            let py = cy + cell_size * (1.5 * r as f32);

            let dist = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
            if dist < radius {
                let dot_r = (cell_size * 0.38) * (1.0 - dist / radius * 0.3);
                // Jitter sutil para apariencia orgánica
                let jx = (rng.next_f32() - 0.5) * 0.6;
                let jy = (rng.next_f32() - 0.5) * 0.6;
                canvas.fill_circle(px + jx, py + jy, dot_r, col);
            }
        }
    }
}

/// Garabato energético acentuado (scribble)
pub fn draw_scribble(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    radius: f32,
    colors: &[Color],
    seed: u32,
) {
    if colors.is_empty() {
        return;
    }
    let mut rng = Rng::new(seed);
    let loops = 4;

    for (k, &col) in colors.iter().enumerate().take(3) {
        let mut pts = Vec::new();
        let step_count = 20;
        for i in 0..step_count {
            let t = i as f32 / step_count as f32;
            let theta = t * PI * 2.0 * loops as f32;
            let r = radius * (0.3 + t * 0.7);
            let x = cx + theta.cos() * r + (rng.next_f32() - 0.5) * 6.0;
            let y = cy + theta.sin() * r + (rng.next_f32() - 0.5) * 6.0;
            pts.push([x, y]);
        }
        if let Some(path) = wob_path(&pts, 3.0, seed + (k as u32) * 13, false) {
            canvas.stroke_path(&path, col.with_alpha(0.75), 2.2);
        }
    }
}
