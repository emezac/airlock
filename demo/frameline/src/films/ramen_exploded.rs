//! Film "Ramen Exploded Diagram" (Taller Film)
//! Reproducción fidedigna del diagrama técnico vectorial animado de @99cccc (Arrow 2):
//! "one bowl of ramen, exploded into nine components, drawn as a technical diagram.
//! Every line is a path. Zoom as far as you want — nothing breaks. Watch it draw itself."
//!
//! https://x.com/99cccc/status/2101618191583895693

use crate::audio::{AudioTrack, Waveform};
use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette};
use crate::core::rng::Rng;
use crate::timeline::{FilmTimeline, Scene};
use std::f32::consts::PI;
use tiny_skia::PathBuilder;

// ─────────────────────────────────────────────────────────────────────────────
// Paleta Cromática y Constantes del Diagrama
// ─────────────────────────────────────────────────────────────────────────────

const COLOR_BG: Color = Color { r: 243.0 / 255.0, g: 239.0 / 255.0, b: 233.0 / 255.0, a: 1.0 }; // #F3EFE9
const COLOR_INK: Color = Color { r: 32.0 / 255.0, g: 30.0 / 255.0, b: 27.0 / 255.0, a: 1.0 };    // #201E1B
const COLOR_RED: Color = Color { r: 214.0 / 255.0, g: 49.0 / 255.0, b: 42.0 / 255.0, a: 1.0 };   // #D6312A (Yolk & Bowl stripes)
const COLOR_FILL: Color = Color { r: 250.0 / 255.0, g: 248.0 / 255.0, b: 243.0 / 255.0, a: 1.0 };// #FAF8F3 (Opaque occluder)
const COLOR_BAMBOO_SHADOW: Color = Color { r: 235.0 / 255.0, g: 231.0 / 255.0, b: 222.0 / 255.0, a: 1.0 };
const COLOR_BAMBOO_CAP: Color = Color { r: 228.0 / 255.0, g: 223.0 / 255.0, b: 212.0 / 255.0, a: 1.0 };

const REF_WIDTH: f32 = 720.0;
const REF_HEIGHT: f32 = 900.0;
const TOTAL_DURATION_SECS: f32 = 12.5;

// ─────────────────────────────────────────────────────────────────────────────
// Utilidades Geométricas para Trazado Vectorial Progresivo
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct Viewport {
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

impl Viewport {
    fn new(canvas_w: f32, canvas_h: f32) -> Self {
        let scale = canvas_h / REF_HEIGHT;
        let offset_x = (canvas_w - REF_WIDTH * scale) * 0.5;
        let offset_y = 0.0;
        Self { scale, offset_x, offset_y }
    }

    fn pt(&self, x: f32, y: f32) -> (f32, f32) {
        (self.offset_x + x * self.scale, self.offset_y + y * self.scale)
    }

    fn sz(&self, val: f32) -> f32 {
        val * self.scale
    }
}

/// Dibuja una línea punteada progresiva
fn draw_dashed_line(
    canvas: &mut Canvas,
    vp: &Viewport,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    progress: f32,
    dash_len: f32,
    gap_len: f32,
    color: Color,
    width: f32,
) {
    if progress <= 0.0 {
        return;
    }
    let dx = x2 - x1;
    let dy = y2 - y1;
    let total_len = (dx * dx + dy * dy).sqrt();
    if total_len < 0.001 {
        return;
    }
    let target_len = total_len * progress.clamp(0.0, 1.0);

    let unit_x = dx / total_len;
    let unit_y = dy / total_len;

    let mut cur_dist = 0.0;
    while cur_dist < target_len {
        let seg_end = (cur_dist + dash_len).min(target_len);
        let start_x = x1 + unit_x * cur_dist;
        let start_y = y1 + unit_y * cur_dist;
        let end_x = x1 + unit_x * seg_end;
        let end_y = y1 + unit_y * seg_end;

        let (p1x, p1y) = vp.pt(start_x, start_y);
        let (p2x, p2y) = vp.pt(end_x, end_y);
        canvas.stroke_line(p1x, p1y, p2x, p2y, color, vp.sz(width));

        cur_dist += dash_len + gap_len;
    }
}

/// Dibuja una polilínea recortada al porcentaje de progreso `[0.0, 1.0]`
fn draw_partial_polyline(
    canvas: &mut Canvas,
    vp: &Viewport,
    points: &[(f32, f32)],
    progress: f32,
    color: Color,
    stroke_width: f32,
) -> Option<(f32, f32)> {
    if points.len() < 2 || progress <= 0.0 {
        return None;
    }

    let mut seg_lengths = Vec::with_capacity(points.len() - 1);
    let mut total_len = 0.0;
    for i in 0..points.len() - 1 {
        let dx = points[i + 1].0 - points[i].0;
        let dy = points[i + 1].1 - points[i].1;
        let len = (dx * dx + dy * dy).sqrt();
        seg_lengths.push(len);
        total_len += len;
    }

    if total_len < 0.001 {
        return None;
    }

    let target_len = total_len * progress.clamp(0.0, 1.0);
    let mut accumulated = 0.0;
    let mut pb = PathBuilder::new();
    let (p0x, p0y) = vp.pt(points[0].0, points[0].1);
    pb.move_to(p0x, p0y);

    let mut tip_pt = (points[0].0, points[0].1);

    for (i, &len) in seg_lengths.iter().enumerate() {
        if accumulated + len <= target_len {
            accumulated += len;
            let (px, py) = vp.pt(points[i + 1].0, points[i + 1].1);
            pb.line_to(px, py);
            tip_pt = points[i + 1];
        } else {
            let remain = target_len - accumulated;
            let ratio = (remain / len).clamp(0.0, 1.0);
            let interp_x = points[i].0 + (points[i + 1].0 - points[i].0) * ratio;
            let interp_y = points[i].1 + (points[i + 1].1 - points[i].1) * ratio;
            let (px, py) = vp.pt(interp_x, interp_y);
            pb.line_to(px, py);
            tip_pt = (interp_x, interp_y);
            break;
        }
    }

    if let Some(path) = pb.finish() {
        canvas.stroke_path(&path, color, vp.sz(stroke_width));
    }

    Some(tip_pt)
}

/// Dibuja un polígono relleno opaco con contorno dibujado progresivamente
fn draw_partial_polygon(
    canvas: &mut Canvas,
    vp: &Viewport,
    points: &[(f32, f32)],
    progress: f32,
    fill_color: Option<Color>,
    stroke_color: Color,
    stroke_width: f32,
) -> Option<(f32, f32)> {
    if points.len() < 3 || progress <= 0.0 {
        return None;
    }

    if let Some(fc) = fill_color {
        let mut fpb = PathBuilder::new();
        let (p0x, p0y) = vp.pt(points[0].0, points[0].1);
        fpb.move_to(p0x, p0y);
        for pt in &points[1..] {
            let (px, py) = vp.pt(pt.0, pt.1);
            fpb.line_to(px, py);
        }
        fpb.close();
        if let Some(path) = fpb.finish() {
            let alpha = progress.clamp(0.0, 1.0);
            canvas.fill_path(&path, fc.with_alpha(alpha));
        }
    }

    let mut closed_pts = points.to_vec();
    closed_pts.push(points[0]);
    draw_partial_polyline(canvas, vp, &closed_pts, progress, stroke_color, stroke_width)
}

/// Dibuja una elipse orientada con trazo parcial
fn draw_partial_ellipse(
    canvas: &mut Canvas,
    vp: &Viewport,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    angle_rad: f32,
    progress: f32,
    fill_color: Option<Color>,
    stroke_color: Color,
    stroke_width: f32,
) -> Option<(f32, f32)> {
    if progress <= 0.0 {
        return None;
    }

    let steps = 48;
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    let mut pts = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let theta = (i as f32 / steps as f32) * PI * 2.0;
        let lx = rx * theta.cos();
        let ly = ry * theta.sin();
        let gx = cx + lx * cos_a - ly * sin_a;
        let gy = cy + lx * sin_a + ly * cos_a;
        pts.push((gx, gy));
    }

    if let Some(fc) = fill_color {
        let mut fpb = PathBuilder::new();
        let (p0x, p0y) = vp.pt(pts[0].0, pts[0].1);
        fpb.move_to(p0x, p0y);
        for pt in &pts[1..steps] {
            let (px, py) = vp.pt(pt.0, pt.1);
            fpb.line_to(px, py);
        }
        fpb.close();
        if let Some(path) = fpb.finish() {
            let alpha = progress.clamp(0.0, 1.0);
            canvas.fill_path(&path, fc.with_alpha(alpha));
        }
    }

    draw_partial_polyline(canvas, vp, &pts, progress, stroke_color, stroke_width)
}

/// Dibuja un cursor circular sutil (plumilla de dibujo tipo Arrow 2)
fn draw_pen_cursor(canvas: &mut Canvas, vp: &Viewport, tip_x: f32, tip_y: f32) {
    let (cx, cy) = vp.pt(tip_x, tip_y);
    canvas.stroke_circle(cx, cy, vp.sz(2.8), COLOR_INK, vp.sz(1.2));
}

// ─────────────────────────────────────────────────────────────────────────────
// Implementación de las 9 Capas del Ramen
// ─────────────────────────────────────────────────────────────────────────────

/// Capa 1: Alga Nori (Seaweed Sheet)
fn draw_nori_layer(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let t_start = 0.0;
    let t_end = 1.2;
    if t < t_start {
        return;
    }
    let p = ((t - t_start) / (t_end - t_start)).clamp(0.0, 1.0);

    let nori_poly = [
        (356.0, 13.0),
        (433.0, 49.0),
        (356.0, 87.0),
        (272.0, 51.0),
    ];

    let tip = draw_partial_polygon(
        canvas,
        vp,
        &nori_poly,
        p.min(1.0),
        Some(COLOR_FILL),
        COLOR_INK,
        2.0,
    );

    // Textura orgánica granular de alga seca nori rica y densa
    if p > 0.20 {
        let tex_prog = ((p - 0.20) / 0.80).clamp(0.0, 1.0);
        let mut rng = Rng::new(42);

        let count = (380.0 * tex_prog) as usize;
        for _ in 0..count {
            let u: f32 = rng.next_range(0.03, 0.97);
            let v: f32 = rng.next_range(0.03, 0.97);

            let top_edge_x = nori_poly[0].0 + (nori_poly[1].0 - nori_poly[0].0) * u;
            let top_edge_y = nori_poly[0].1 + (nori_poly[1].1 - nori_poly[0].1) * u;
            let bot_edge_x = nori_poly[3].0 + (nori_poly[2].0 - nori_poly[3].0) * u;
            let bot_edge_y = nori_poly[3].1 + (nori_poly[2].1 - nori_poly[3].1) * u;

            let px = top_edge_x + (bot_edge_x - top_edge_x) * v;
            let py = top_edge_y + (bot_edge_y - top_edge_y) * v;

            let len = rng.next_range(2.0, 5.0);
            let angle = rng.next_range(0.0, PI * 2.0);
            let p2x = px + angle.cos() * len;
            let p2y = py + angle.sin() * len;

            let (p1, p2) = (vp.pt(px, py), vp.pt(p2x, p2y));
            canvas.stroke_line(p1.0, p1.1, p2.0, p2.1, COLOR_INK, vp.sz(1.0));

            if rng.next_bool(0.40) {
                let (cx, cy) = vp.pt(px, py);
                canvas.fill_circle(cx, cy, vp.sz(0.85), COLOR_INK);
            }
        }
    }

    if p < 1.0 && t >= t_start {
        if let Some((tx, ty)) = tip {
            draw_pen_cursor(canvas, vp, tx, ty);
        }
    }
}

/// Capa 2: Ajitsuke Tamago (Dos mitades de huevo cocido marinado)
fn draw_tamago_layer(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let t_start = 0.6;
    if t < t_start {
        return;
    }

    let p_left = ((t - 0.6) / 0.9).clamp(0.0, 1.0);
    let p_right = ((t - 1.3) / 1.0).clamp(0.0, 1.0);

    let draw_egg_half = |canvas: &mut Canvas, cx: f32, cy: f32, angle_deg: f32, p: f32, is_right: bool| {
        if p <= 0.0 {
            return;
        }
        let rad = angle_deg.to_radians();

        // Silueta inferior curva suave 3D de la cáscara/clara
        let steps = 24;
        let mut cup_pts = Vec::with_capacity(steps + 1);
        let span_angle = PI * 0.95;
        let start_angle = if !is_right { 0.05 * PI } else { 0.0 * PI };
        for s in 0..=steps {
            let u = s as f32 / steps as f32;
            let a = start_angle + u * span_angle;
            let ex = 36.0 * a.cos();
            let ey = 32.0 * a.sin();
            let rx = cx + ex * rad.cos() - ey * rad.sin();
            let ry = cy + ex * rad.sin() + ey * rad.cos() + 6.0;
            cup_pts.push((rx, ry));
        }

        // Relleno opaco de la clara
        let mut fpb = PathBuilder::new();
        let (p0x, p0y) = vp.pt(cup_pts[0].0, cup_pts[0].1);
        fpb.move_to(p0x, p0y);
        for pt in &cup_pts[1..] {
            let (px, py) = vp.pt(pt.0, pt.1);
            fpb.line_to(px, py);
        }
        fpb.close();
        if let Some(path) = fpb.finish() {
            canvas.fill_path(&path, COLOR_FILL);
        }

        draw_partial_polyline(canvas, vp, &cup_pts, p.min(1.0), COLOR_INK, 1.8);

        // Cara de corte elíptica
        draw_partial_ellipse(
            canvas,
            vp,
            cx,
            cy,
            34.0,
            23.0,
            rad,
            p.min(1.0),
            Some(COLOR_FILL),
            COLOR_INK,
            2.0,
        );

        // Yema central rojo bermellón (#D6312A)
        if p > 0.35 {
            let p_yolk = ((p - 0.35) / 0.65).clamp(0.0, 1.0);
            let tip = draw_partial_ellipse(
                canvas,
                vp,
                cx,
                cy,
                16.0,
                19.5,
                rad,
                p_yolk,
                Some(COLOR_RED),
                COLOR_INK,
                1.8,
            );
            if p_yolk < 1.0 {
                if let Some((tx, ty)) = tip {
                    draw_pen_cursor(canvas, vp, tx, ty);
                }
            }
        }
    };

    draw_egg_half(canvas, 314.0, 147.0, -16.0, p_left, false);
    draw_egg_half(canvas, 394.0, 147.0, 16.0, p_right, true);
}

/// Capa 3: Chashu Pork (Tres rodajas de panceta braseada en espiral)
fn draw_chashu_layer(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let t_start = 1.8;
    if t < t_start {
        return;
    }

    let slices = [
        (298.0, 252.0, 42.0, 39.0, 1.8, 2.7, -0.15),
        (348.0, 250.0, 44.0, 40.0, 2.5, 3.5, 0.05),
        (404.0, 248.0, 46.0, 41.0, 3.3, 4.3, 0.20),
    ];

    for &(cx, cy, rx, ry, s_t, e_t, angle) in &slices {
        if t < s_t {
            continue;
        }
        let p = ((t - s_t) / (e_t - s_t)).clamp(0.0, 1.0);

        let tip = draw_partial_ellipse(
            canvas,
            vp,
            cx,
            cy,
            rx,
            ry,
            angle,
            p.min(1.0),
            Some(COLOR_FILL),
            COLOR_INK,
            2.0,
        );

        if p > 0.25 {
            let p_inner = ((p - 0.25) / 0.75).clamp(0.0, 1.0);

            // Espiral concéntrica 1
            draw_partial_ellipse(
                canvas,
                vp,
                cx + 2.0,
                cy + 1.0,
                rx * 0.78,
                ry * 0.76,
                angle + 0.1,
                p_inner,
                None,
                COLOR_INK,
                1.3,
            );

            // Espiral concéntrica 2
            if p_inner > 0.25 {
                let p2 = ((p_inner - 0.25) / 0.75).clamp(0.0, 1.0);
                draw_partial_ellipse(
                    canvas,
                    vp,
                    cx - 1.0,
                    cy - 2.0,
                    rx * 0.54,
                    ry * 0.52,
                    angle - 0.08,
                    p2,
                    None,
                    COLOR_INK,
                    1.2,
                );
            }

            // Espiral concéntrica 3 (núcleo)
            if p_inner > 0.50 {
                let p3 = ((p_inner - 0.50) / 0.50).clamp(0.0, 1.0);
                draw_partial_ellipse(
                    canvas,
                    vp,
                    cx + 1.0,
                    cy + 1.0,
                    rx * 0.30,
                    ry * 0.28,
                    angle + 0.05,
                    p3,
                    None,
                    COLOR_INK,
                    1.1,
                );
            }

            // Franja de grasa marmoleada suave
            if p_inner > 0.40 {
                let p_fat = ((p_inner - 0.40) / 0.60).clamp(0.0, 1.0);
                let steps = 16;
                let mut fat_pts = Vec::with_capacity(steps + 1);
                for s in 0..=steps {
                    let u = s as f32 / steps as f32;
                    let x = cx - rx * 0.65 + u * rx * 1.3;
                    let wave = (u * PI * 2.0).sin() * 7.0;
                    let y = cy + (u - 0.5) * 8.0 + wave;
                    fat_pts.push((x, y));
                }
                draw_partial_polyline(canvas, vp, &fat_pts, p_fat, COLOR_INK, 1.4);
            }
        }

        if p < 1.0 {
            if let Some((tx, ty)) = tip {
                draw_pen_cursor(canvas, vp, tx, ty);
            }
        }
    }
}

/// Capa 4: Menma (Brotes de Bambú en prismas 3D facetados)
fn draw_menma_layer(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let t_start = 3.8;
    if t < t_start {
        return;
    }

    struct BambooStick {
        top_face: [(f32, f32); 4],
        front_face: [(f32, f32); 4],
        end_face: [(f32, f32); 4],
        s_t: f32,
        e_t: f32,
    }

    let sticks = [
        BambooStick {
            top_face: [(372.0, 344.0), (420.0, 354.0), (414.0, 363.0), (366.0, 353.0)],
            front_face: [(366.0, 353.0), (414.0, 363.0), (414.0, 372.0), (366.0, 362.0)],
            end_face: [(414.0, 363.0), (420.0, 354.0), (420.0, 363.0), (414.0, 372.0)],
            s_t: 3.8,
            e_t: 4.3,
        },
        BambooStick {
            top_face: [(296.0, 346.0), (348.0, 338.0), (352.0, 347.0), (300.0, 355.0)],
            front_face: [(300.0, 355.0), (352.0, 347.0), (352.0, 356.0), (300.0, 364.0)],
            end_face: [(296.0, 346.0), (300.0, 355.0), (300.0, 364.0), (296.0, 355.0)],
            s_t: 4.1,
            e_t: 4.6,
        },
        BambooStick {
            top_face: [(298.0, 354.0), (402.0, 378.0), (398.0, 388.0), (294.0, 364.0)],
            front_face: [(294.0, 364.0), (398.0, 388.0), (398.0, 399.0), (294.0, 375.0)],
            end_face: [(398.0, 388.0), (402.0, 378.0), (402.0, 389.0), (398.0, 399.0)],
            s_t: 4.4,
            e_t: 5.0,
        },
        BambooStick {
            top_face: [(288.0, 368.0), (392.0, 392.0), (388.0, 402.0), (284.0, 378.0)],
            front_face: [(284.0, 378.0), (392.0, 402.0), (392.0, 412.0), (284.0, 388.0)],
            end_face: [(388.0, 402.0), (392.0, 392.0), (392.0, 402.0), (388.0, 412.0)],
            s_t: 4.8,
            e_t: 5.4,
        },
        BambooStick {
            top_face: [(304.0, 382.0), (388.0, 402.0), (384.0, 412.0), (300.0, 392.0)],
            front_face: [(300.0, 392.0), (384.0, 412.0), (384.0, 421.0), (300.0, 401.0)],
            end_face: [(384.0, 412.0), (388.0, 402.0), (388.0, 411.0), (384.0, 421.0)],
            s_t: 5.1,
            e_t: 5.6,
        },
    ];

    for stick in &sticks {
        if t < stick.s_t {
            continue;
        }
        let p = ((t - stick.s_t) / (stick.e_t - stick.s_t)).clamp(0.0, 1.0);

        draw_partial_polygon(
            canvas,
            vp,
            &stick.front_face,
            p,
            Some(COLOR_BAMBOO_SHADOW),
            COLOR_INK,
            1.6,
        );

        let tip = draw_partial_polygon(
            canvas,
            vp,
            &stick.top_face,
            p,
            Some(COLOR_FILL),
            COLOR_INK,
            1.7,
        );

        draw_partial_polygon(
            canvas,
            vp,
            &stick.end_face,
            p,
            Some(COLOR_BAMBOO_CAP),
            COLOR_INK,
            1.5,
        );

        if p > 0.4 {
            let p_grain = ((p - 0.4) / 0.6).clamp(0.0, 1.0);
            for frac in [0.3, 0.6] {
                let p1x = stick.top_face[0].0 + (stick.top_face[3].0 - stick.top_face[0].0) * frac;
                let p1y = stick.top_face[0].1 + (stick.top_face[3].1 - stick.top_face[0].1) * frac;
                let p2x = stick.top_face[1].0 + (stick.top_face[2].0 - stick.top_face[1].0) * frac;
                let p2y = stick.top_face[1].1 + (stick.top_face[2].1 - stick.top_face[1].1) * frac;
                draw_partial_polyline(canvas, vp, &[(p1x, p1y), (p2x, p2y)], p_grain, COLOR_INK, 0.9);
            }
        }

        if p < 1.0 {
            if let Some((tx, ty)) = tip {
                draw_pen_cursor(canvas, vp, tx, ty);
            }
        }
    }
}

/// Capa 5: Negi (Rodajas concéntricas de cebollino verde)
fn draw_negi_layer(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let t_start = 5.2;
    if t < t_start {
        return;
    }

    let scallions = [
        (356.0, 436.0, 9.0, 5.5, 0.15, 5.2),
        (340.0, 428.0, 8.5, 5.0, -0.20, 5.3),
        (374.0, 432.0, 9.5, 5.5, 0.25, 5.35),
        (322.0, 438.0, 8.0, 4.5, -0.10, 5.4),
        (390.0, 439.0, 9.0, 5.0, 0.18, 5.45),
        (308.0, 442.0, 7.5, 4.5, -0.30, 5.5),
        (406.0, 443.0, 8.5, 5.0, 0.22, 5.55),
        (292.0, 448.0, 8.0, 5.0, -0.15, 5.6),
        (422.0, 448.0, 8.0, 4.8, 0.35, 5.65),
        (276.0, 455.0, 7.0, 4.2, -0.25, 5.7),
        (436.0, 454.0, 7.5, 4.5, 0.40, 5.75),
        (332.0, 448.0, 8.5, 5.0, 0.05, 5.8),
        (362.0, 446.0, 9.0, 5.2, -0.08, 5.85),
        (382.0, 452.0, 8.0, 4.8, 0.12, 5.9),
        (312.0, 458.0, 7.5, 4.4, -0.18, 5.95),
        (402.0, 458.0, 8.0, 4.6, 0.28, 6.0),
        (348.0, 456.0, 9.0, 5.2, 0.10, 6.05),
        (368.0, 460.0, 8.5, 5.0, -0.12, 6.1),
        (330.0, 464.0, 8.0, 4.8, 0.02, 6.15),
        (390.0, 466.0, 7.8, 4.5, 0.16, 6.2),
        (355.0, 420.0, 8.2, 4.8, 0.20, 6.25),
        (370.0, 422.0, 7.8, 4.5, -0.15, 6.3),
        (336.0, 418.0, 8.0, 4.6, 0.10, 6.35),
        (386.0, 426.0, 8.5, 5.0, 0.25, 6.4),
    ];

    for &(cx, cy, rx, ry, angle, s_t) in &scallions {
        if t < s_t {
            continue;
        }
        let dur = 0.45;
        let p = ((t - s_t) / dur).clamp(0.0, 1.0);

        let tip = draw_partial_ellipse(
            canvas,
            vp,
            cx,
            cy,
            rx,
            ry,
            angle,
            p,
            Some(COLOR_FILL),
            COLOR_INK,
            1.5,
        );

        if p > 0.3 {
            let p_in = ((p - 0.3) / 0.7).clamp(0.0, 1.0);
            draw_partial_ellipse(
                canvas,
                vp,
                cx,
                cy,
                rx * 0.55,
                ry * 0.55,
                angle,
                p_in,
                Some(COLOR_BG),
                COLOR_INK,
                1.3,
            );
        }

        if p < 1.0 {
            if let Some((tx, ty)) = tip {
                draw_pen_cursor(canvas, vp, tx, ty);
            }
        }
    }
}

/// Capa 6: Fideos Ramen (Chijire-men: Nido denso, ondulado y abovedado)
fn draw_noodles_layer(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let t_start = 6.6;
    let _t_end = 9.2;
    if t < t_start {
        return;
    }

    let cx = 356.0;
    let cy = 548.0;
    let nest_rx = 98.0;
    let nest_ry = 50.0;

    let noodle_count = 52;
    let mut rng = Rng::new(2026);

    for i in 0..noodle_count {
        let frac = i as f32 / noodle_count as f32;
        let s_t = t_start + frac * 2.2;
        let e_t = s_t + 0.45;
        if t < s_t {
            continue;
        }
        let p = ((t - s_t) / (e_t - s_t)).clamp(0.0, 1.0);

        // Posición vertical normalizada dentro de la cúpula del nido
        let norm_y = (frac - 0.5) * 2.0; // [-1.0, 1.0]
        let base_y = cy + norm_y * nest_ry * 0.9 + rng.next_range(-3.0, 3.0);

        // Ancho elíptico en esta altura para formar el montículo abovedado
        let chord_ratio = (1.0 - (norm_y * 0.95).powi(2)).max(0.1).sqrt();
        let current_w = nest_rx * chord_ratio;

        let left_x = cx - current_w + rng.next_range(-6.0, 6.0);
        let right_x = cx + current_w + rng.next_range(-6.0, 6.0);

        let segments = 14;
        let mut strand_pts = Vec::with_capacity(segments + 1);
        let amp = rng.next_range(5.0, 10.0);
        let freq = rng.next_range(3.0, 5.0);
        let phase = rng.next_range(0.0, PI * 2.0);

        for s in 0..=segments {
            let u = s as f32 / segments as f32;
            let x = left_x + (right_x - left_x) * u;
            // Curvatura del nido (abombado hacia abajo en los bordes)
            let sag = (u * PI).sin() * 7.0;
            let wave = (u * PI * freq + phase).sin() * amp;
            let y = base_y + sag + wave;
            strand_pts.push((x, y));
        }

        // Cinta opaca protectora debajo para oclusión física de los fideos
        draw_partial_polyline(canvas, vp, &strand_pts, p, COLOR_FILL, 4.5);

        // Línea de tinta de la hebra de fideo
        let tip = draw_partial_polyline(canvas, vp, &strand_pts, p, COLOR_INK, 1.6);

        // Algunos fideos forman bucles característicos en los extremos
        if i % 5 == 0 && p > 0.7 {
            let p_loop = ((p - 0.7) / 0.3).clamp(0.0, 1.0);
            let loop_x = if i % 2 == 0 { left_x - 4.0 } else { right_x + 4.0 };
            draw_partial_ellipse(canvas, vp, loop_x, base_y + 4.0, 6.0, 8.0, 0.2, p_loop, Some(COLOR_FILL), COLOR_INK, 1.5);
        }

        if p < 1.0 && p > 0.05 {
            if let Some((tx, ty)) = tip {
                draw_pen_cursor(canvas, vp, tx, ty);
            }
        }
    }
}

/// Capa 7: Superficie del Caldo (Tonkotsu / Shoyu Soup Disc)
fn draw_broth_layer(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let t_start = 8.8;
    let t_end = 10.2;
    if t < t_start {
        return;
    }

    let p = ((t - t_start) / (t_end - t_start)).clamp(0.0, 1.0);

    let cx = 356.0;
    let cy = 659.0;
    let rx = 114.0;
    let ry = 33.0;

    let tip = draw_partial_ellipse(
        canvas,
        vp,
        cx,
        cy,
        rx,
        ry,
        0.0,
        p.min(1.0),
        Some(COLOR_FILL),
        COLOR_INK,
        2.0,
    );

    if p > 0.35 {
        let p_swirl = ((p - 0.35) / 0.65).clamp(0.0, 1.0);

        // Remolino fluido suave en S (corrientes térmicas del caldo caliente)
        let steps = 24;
        let mut s_pts = Vec::with_capacity(steps + 1);
        for s in 0..=steps {
            let u = s as f32 / steps as f32;
            let x = cx - 55.0 + u * 105.0;
            let y = cy + (u * PI * 2.0).sin() * 7.0 - (u - 0.5) * 4.0;
            s_pts.push((x, y));
        }
        draw_partial_polyline(canvas, vp, &s_pts, p_swirl, COLOR_INK, 1.3);

        if p_swirl > 0.35 {
            let p2 = ((p_swirl - 0.35) / 0.65).clamp(0.0, 1.0);
            let mut s2_pts = Vec::with_capacity(steps + 1);
            for s in 0..=steps {
                let u = s as f32 / steps as f32;
                let x = cx - 40.0 + u * 90.0;
                let y = cy + 10.0 + (u * PI * 1.5).cos() * 5.0;
                s2_pts.push((x, y));
            }
            draw_partial_polyline(canvas, vp, &s2_pts, p2, COLOR_INK, 1.1);
        }
    }

    if p < 1.0 {
        if let Some((tx, ty)) = tip {
            draw_pen_cursor(canvas, vp, tx, ty);
        }
    }
}

/// Capa 8: Tazón Donburi Cerámico Tradicional
fn draw_donburi_layer(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let t_start = 9.6;
    let t_end = 11.5;
    if t < t_start {
        return;
    }

    let p = ((t - t_start) / (t_end - t_start)).clamp(0.0, 1.0);

    let rim_left = (228.0, 736.0);
    let rim_right = (478.0, 736.0);
    let foot_left = (304.0, 842.0);
    let foot_right = (404.0, 842.0);
    let foot_base_left = (308.0, 856.0);
    let foot_base_right = (400.0, 856.0);

    // Contorno completo con curvas suaves
    let mut bowl_outline = vec![rim_left];
    let steps = 24;
    for i in 1..steps {
        let u = i as f32 / steps as f32;
        let x = (1.0 - u).powi(2) * rim_left.0 + 2.0 * (1.0 - u) * u * 240.0 + u.powi(2) * foot_left.0;
        let y = (1.0 - u).powi(2) * rim_left.1 + 2.0 * (1.0 - u) * u * 800.0 + u.powi(2) * foot_left.1;
        bowl_outline.push((x, y));
    }
    bowl_outline.push(foot_left);
    bowl_outline.push(foot_base_left);

    for i in 1..steps {
        let u = i as f32 / steps as f32;
        let x = foot_base_left.0 + (foot_base_right.0 - foot_base_left.0) * u;
        let y = foot_base_left.1 + (u * PI).sin() * 3.5;
        bowl_outline.push((x, y));
    }
    bowl_outline.push(foot_base_right);
    bowl_outline.push(foot_right);

    for i in 1..steps {
        let u = i as f32 / steps as f32;
        let x = (1.0 - u).powi(2) * foot_right.0 + 2.0 * (1.0 - u) * u * 466.0 + u.powi(2) * rim_right.0;
        let y = (1.0 - u).powi(2) * foot_right.1 + 2.0 * (1.0 - u) * u * 800.0 + u.powi(2) * rim_right.1;
        bowl_outline.push((x, y));
    }
    bowl_outline.push(rim_right);

    // Relleno opaco del tazón
    let mut fpb = PathBuilder::new();
    let (p0x, p0y) = vp.pt(bowl_outline[0].0, bowl_outline[0].1);
    fpb.move_to(p0x, p0y);
    for pt in &bowl_outline[1..] {
        let (px, py) = vp.pt(pt.0, pt.1);
        fpb.line_to(px, py);
    }
    fpb.close();
    if let Some(path) = fpb.finish() {
        canvas.fill_path(&path, COLOR_FILL);
    }

    // Boca / borde superior del tazón (elipse de apertura)
    let rim_pts = vec![rim_left, rim_right];
    draw_partial_polyline(canvas, vp, &rim_pts, (p * 2.0).min(1.0), COLOR_INK, 2.0);

    let tip = draw_partial_polyline(canvas, vp, &bowl_outline, p.min(1.0), COLOR_INK, 2.0);

    // Franjas rojas decorativas del tazón (#D6312A)
    if p > 0.35 {
        let p_red = ((p - 0.35) / 0.65).clamp(0.0, 1.0);

        let stripe1_left = (235.0, 748.0);
        let stripe1_right = (471.0, 748.0);
        draw_partial_polyline(
            canvas,
            vp,
            &[stripe1_left, stripe1_right],
            p_red,
            COLOR_RED,
            1.8,
        );

        let stripe2_left = (242.0, 756.0);
        let stripe2_right = (464.0, 756.0);
        draw_partial_polyline(
            canvas,
            vp,
            &[stripe2_left, stripe2_right],
            p_red,
            COLOR_RED,
            1.8,
        );
    }

    // Línea de sombra de cerámica artesanal en el lado inferior izquierdo
    if p > 0.65 {
        let p_shadow = ((p - 0.65) / 0.35).clamp(0.0, 1.0);
        let steps = 16;
        let mut shadow_pts = Vec::with_capacity(steps + 1);
        for s in 0..=steps {
            let u = s as f32 / steps as f32;
            let x = 258.0 + u * 45.0;
            let y = 788.0 + u * 46.0 - (u * PI).sin() * 4.0;
            shadow_pts.push((x, y));
        }
        draw_partial_polyline(canvas, vp, &shadow_pts, p_shadow, COLOR_INK, 1.1);
    }

    if p < 1.0 {
        if let Some((tx, ty)) = tip {
            draw_pen_cursor(canvas, vp, tx, ty);
        }
    }
}

/// Capa 9: Guías Técnicas y Cotas de Dibujo (Technical Guides & Crosshairs)
fn draw_technical_guides(canvas: &mut Canvas, vp: &Viewport, t: f32) {
    let p_axis = (t / 1.4).clamp(0.0, 1.0);
    draw_dashed_line(
        canvas,
        vp,
        356.0,
        15.0,
        356.0,
        885.0,
        p_axis,
        8.0,
        8.0,
        COLOR_INK,
        1.6,
    );

    if t > 11.0 {
        let p_reg = ((t - 11.0) / 1.0).clamp(0.0, 1.0);
        let alpha = p_reg;
        let reg_color = COLOR_INK.with_alpha(alpha * 0.4);

        let corners = [
            (50.0, 50.0),
            (670.0, 50.0),
            (50.0, 850.0),
            (670.0, 850.0),
        ];

        for &(cx, cy) in &corners {
            let arm = 8.0;
            let (p1x, p1y) = vp.pt(cx - arm, cy);
            let (p2x, p2y) = vp.pt(cx + arm, cy);
            canvas.stroke_line(p1x, p1y, p2x, p2y, reg_color, vp.sz(1.0));

            let (p3x, p3y) = vp.pt(cx, cy - arm);
            let (p4x, p4y) = vp.pt(cx, cy + arm);
            canvas.stroke_line(p3x, p3y, p4x, p4y, reg_color, vp.sz(1.0));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Escena Cinematográfica y Línea de Tiempo
// ─────────────────────────────────────────────────────────────────────────────

pub struct RamenExplodedScene {
    palette: Palette,
}

impl RamenExplodedScene {
    pub fn new() -> Self {
        let mut palette = Palette::paper_ink();
        palette.paper = COLOR_BG;
        palette.ink = COLOR_INK;
        palette.accents = vec![COLOR_RED];
        Self { palette }
    }
}

impl Scene for RamenExplodedScene {
    fn name(&self) -> &str {
        "RamenExplodedDiagram"
    }

    fn duration_frames(&self) -> usize {
        300
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, _frame: usize) {
        canvas.clear(COLOR_BG);

        let vp = Viewport::new(canvas.width as f32, canvas.height as f32);
        let t = tau * TOTAL_DURATION_SECS;

        // 1. Eje Central Discontinuo (de fondo)
        draw_technical_guides(canvas, &vp, t);

        // 2. Capa 8: Tazón Donburi (inferior)
        draw_donburi_layer(canvas, &vp, t);

        // 3. Capa 7: Superficie del Caldo (Soup Disc)
        draw_broth_layer(canvas, &vp, t);

        // 4. Capa 6: Fideos Ramen (Chijire-men)
        draw_noodles_layer(canvas, &vp, t);

        // 5. Capa 5: Cebollino Verde (Negi)
        draw_negi_layer(canvas, &vp, t);

        // 6. Capa 4: Brotes de Bambú (Menma)
        draw_menma_layer(canvas, &vp, t);

        // 7. Capa 3: Panceta Chashu
        draw_chashu_layer(canvas, &vp, t);

        // 8. Capa 2: Huevos Ajitsuke Tamago
        draw_tamago_layer(canvas, &vp, t);

        // 9. Capa 1: Alga Nori (superior)
        draw_nori_layer(canvas, &vp, t);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Generador de la Banda Sonora Procedural (Fallback)
// ─────────────────────────────────────────────────────────────────────────────

fn generate_ramen_audio(duration_secs: f32) -> AudioTrack {
    let mut track = AudioTrack::new(duration_secs, 48000);

    let stroke_events = [
        (0.2, 1.0, 880.0),
        (1.0, 1.2, 1046.0),
        (2.4, 1.4, 1174.0),
        (4.2, 1.2, 1318.0),
        (5.4, 1.2, 1568.0),
        (7.0, 1.8, 1760.0),
        (9.0, 1.0, 1975.0),
        (10.0, 1.4, 2093.0),
    ];

    for &(start_t, dur, f_base) in &stroke_events {
        track.add_note(start_t, dur * 1.5, f_base, Waveform::Sine, 0.08, 0.0);
        track.add_note(start_t + 0.05, dur, f_base * 1.5, Waveform::Triangle, 0.04, 0.2);

        let step_count = (dur * 8.0) as usize;
        for s in 0..step_count {
            let st = start_t + (s as f32) * 0.12;
            let pan = ((s % 2) as f32) * 0.4 - 0.2;
            track.add_note(st, 0.04, 3200.0 + (s as f32 * 120.0), Waveform::Sawtooth, 0.015, pan);
        }
    }

    track.add_note(11.2, 2.5, 440.0, Waveform::Sine, 0.12, 0.0);
    track.add_note(11.2, 2.5, 659.25, Waveform::Sine, 0.09, -0.3);
    track.add_note(11.2, 2.5, 880.0, Waveform::Sine, 0.07, 0.3);
    track.add_note(11.2, 3.0, 220.0, Waveform::Triangle, 0.10, 0.0);

    track
}

// ─────────────────────────────────────────────────────────────────────────────
// Función Constructora Principal del Film
// ─────────────────────────────────────────────────────────────────────────────

pub fn create_ramen_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();
    timeline.fps = 24;
    timeline.on_twos = false;
    timeline.motion_blur = 0.0;

    timeline.add_scene(RamenExplodedScene::new());

    let audio = generate_ramen_audio(TOTAL_DURATION_SECS);

    (timeline, audio)
}
