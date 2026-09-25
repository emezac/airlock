use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::finishes::wob_path;
use crate::core::primitives::ellipse_points;
use crate::core::rng::Rng;
use std::f32::consts::PI;

/// Dibuja el fondo de placa de papel cálido con franjas diagonales auténticas del referente
pub fn draw_paper_plate(canvas: &mut Canvas, width: u32, height: u32, seed: u32) {
    let mut rng = Rng::new(seed);
    let paper_base = Color::hex("#EFE3C9"); // Base de papel natural
    let stripe_cream = Color::hex("#F2E7CF"); // Franja A
    let stripe_yellow = Color::hex("#EFDCA3"); // Franja B (sol de verano / warm plate)

    canvas.clear(paper_base);

    let w = width as f32;
    let h = height as f32;

    // Franjas diagonales auténticas a -0.52 radianes (-30 grados), ancho 140px
    let angle = -0.52_f32;
    let sw = 140.0_f32;
    let _period = sw * 2.0;
    let half_diag = ((w * w + h * h).sqrt()) * 0.75;
    let cx = w * 0.5;
    let cy = h * 0.5;

    let ca = angle.cos();
    let sa = angle.sin();
    let nx = -sa;
    let ny = ca;

    // Bandas paralelas alternadas
    let mut v = -half_diag;
    let mut stripe_idx = 0;
    while v < half_diag {
        let col = if stripe_idx % 2 == 0 { stripe_cream } else { stripe_yellow };
        let mut pb = tiny_skia::PathBuilder::new();

        let v0 = v;
        let v1 = v + sw;

        // 4 esquinas de la franja proyectadas sobre el ángulo
        let p1x = cx + ca * (-half_diag) + nx * v0;
        let p1y = cy + sa * (-half_diag) + ny * v0;
        let p2x = cx + ca * half_diag + nx * v0;
        let p2y = cy + sa * half_diag + ny * v0;
        let p3x = cx + ca * half_diag + nx * v1;
        let p3y = cy + sa * half_diag + ny * v1;
        let p4x = cx + ca * (-half_diag) + nx * v1;
        let p4y = cy + sa * (-half_diag) + ny * v1;

        pb.move_to(p1x, p1y);
        pb.line_to(p2x, p2y);
        pb.line_to(p3x, p3y);
        pb.line_to(p4x, p4y);
        pb.close();

        if let Some(path) = pb.finish() {
            canvas.fill_path(&path, col);
        }

        v += sw;
        stripe_idx += 1;
    }

    // Grano de papel sutil (300 pequeñas fibras orgánicas)
    let grain_count = 320;
    let grain_col = Color::hex("#2A1C13").with_alpha(0.045);
    for _ in 0..grain_count {
        let gx = rng.next_f32() * w;
        let gy = rng.next_f32() * h;
        let gl = 1.0 + rng.next_f32() * 2.5;
        let ga = (rng.next_f32() - 0.5) * 0.8;
        canvas.stroke_line(gx, gy, gx + gl, gy + gl * ga, grain_col, 0.7);
    }
}

/// Dibuja el fondo técnico de cianotipo / anteproyecto (Blueprint Plate)
/// Ajustado exactamente al estándar Kevin Ngo: fondo navy `#0B1230`, cuadrícula 60px `#3A4A86`
pub fn draw_blueprint_plate(canvas: &mut Canvas, width: u32, height: u32, _seed: u32) {
    let navy_base = Color::hex("#0B1230"); // Base azul marino profundo
    let grid_color = Color::hex("#3A4A86").with_alpha(0.35); // Rejilla 60px
    let subgrid_color = Color::hex("#18234D").with_alpha(0.22); // Rejilla fina 20px
    let lavender_line = Color::hex("#C8C1EF").with_alpha(0.55); // Líneas de cota y registro

    canvas.clear(navy_base);

    let w = width as f32;
    let h = height as f32;

    // Sub-cuadrícula fina cada 20px
    let sub_step = 20.0;
    let mut x = 0.0;
    while x <= w {
        canvas.stroke_line(x, 0.0, x, h, subgrid_color, 0.4);
        x += sub_step;
    }
    let mut y = 0.0;
    while y <= h {
        canvas.stroke_line(0.0, y, w, y, subgrid_color, 0.4);
        y += sub_step;
    }

    // Cuadrícula principal de ingeniería cada 60px
    let grid_step = 60.0;
    let mut gx = 0.0;
    while gx <= w {
        canvas.stroke_line(gx, 0.0, gx, h, grid_color, 0.8);
        gx += grid_step;
    }
    let mut gy = 0.0;
    while gy <= h {
        canvas.stroke_line(0.0, gy, w, gy, grid_color, 0.8);
        gy += grid_step;
    }

    // Cruces de registro y marcas de esquina
    let margin = 50.0;
    let cross = 14.0;
    let corners = [
        (margin, margin),
        (w - margin, margin),
        (margin, h - margin),
        (w - margin, h - margin),
        (w * 0.5, margin),
        (w * 0.5, h - margin),
    ];
    for (cx, cy) in corners {
        canvas.stroke_line(cx - cross, cy, cx + cross, cy, lavender_line, 1.0);
        canvas.stroke_line(cx, cy - cross, cx, cy + cross, lavender_line, 1.0);
        canvas.stroke_circle(cx, cy, 3.0, lavender_line, 0.8);
    }

    // Regla de marcas milimétricas laterales
    let mut ry = margin;
    let mut tick_idx = 0;
    while ry <= h - margin {
        let is_major = tick_idx % 4 == 0;
        let tick_len = if is_major { 14.0 } else { 7.0 };
        let col = if is_major { lavender_line } else { grid_color };
        canvas.stroke_line(margin * 0.5, ry, margin * 0.5 + tick_len, ry, col, 1.0);
        canvas.stroke_line(w - margin * 0.5 - tick_len, ry, w - margin * 0.5, ry, col, 1.0);
        ry += 15.0;
        tick_idx += 1;
    }
}

/// Dibuja corchetes de cota técnica con líneas de extensión y marcas de cota
pub fn draw_dimension_bracket(
    canvas: &mut Canvas,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    label: &str,
    color: Color,
    offset: f32,
) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let nx = -dy / len;
    let ny = dx / len;

    // Puntos desplazados
    let bx1 = x1 + nx * offset;
    let by1 = y1 + ny * offset;
    let bx2 = x2 + nx * offset;
    let by2 = y2 + ny * offset;

    // Líneas de extensión
    let ext_color = color.with_alpha(color.a * 0.6);
    canvas.stroke_line(x1, y1, bx1 + nx * 4.0, by1 + ny * 4.0, ext_color, 0.7);
    canvas.stroke_line(x2, y2, bx2 + nx * 4.0, by2 + ny * 4.0, ext_color, 0.7);

    // Línea de dimensión principal
    canvas.stroke_line(bx1, by1, bx2, by2, color, 0.9);

    // Marcas en los extremos (ticks diagonales a 45 grados)
    let tick_s = 6.0;
    canvas.stroke_line(bx1 - tick_s, by1 - tick_s, bx1 + tick_s, by1 + tick_s, color, 1.2);
    canvas.stroke_line(bx2 - tick_s, by2 - tick_s, bx2 + tick_s, by2 + tick_s, color, 1.2);

    // Etiqueta en el centro
    let mx = (bx1 + bx2) * 0.5 + nx * 14.0;
    let my = (by1 + by2) * 0.5 + ny * 14.0;
    draw_vector_text(canvas, mx, my, label, 11.0, color, true);
}

/// Círculos guía concéntricos con marcas de grado
pub fn draw_concentric_guides(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    r_inner: f32,
    r_outer: f32,
    color: Color,
    seed: u32,
) {
    let rings = [r_inner, (r_inner + r_outer) * 0.5, r_outer];
    for &r in &rings {
        let pts = ellipse_points(cx, cy, r, r, 48);
        if let Some(path) = wob_path(&pts, 1.0, seed + (r as u32), true) {
            canvas.stroke_path(&path, color.with_alpha(color.a * 0.65), 0.8);
        }
    }

    // Cruces principales
    let ext = r_outer * 1.15;
    canvas.stroke_line(cx - ext, cy, cx + ext, cy, color.with_alpha(color.a * 0.5), 0.7);
    canvas.stroke_line(cx, cy - ext, cx, cy + ext, color.with_alpha(color.a * 0.5), 0.7);

    // Ticks perimetrales cada 30 grados
    for i in 0..12 {
        let angle = (i as f32) * PI / 6.0;
        let x1 = cx + angle.cos() * (r_outer - 8.0);
        let y1 = cy + angle.sin() * (r_outer - 8.0);
        let x2 = cx + angle.cos() * (r_outer + 8.0);
        let y2 = cy + angle.sin() * (r_outer + 8.0);
        canvas.stroke_line(x1, y1, x2, y2, color, 0.9);
    }
}

/// Dibuja una brújula solar / indicador de navegación celestial (Sun Compass)
pub fn draw_sun_compass(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    radius: f32,
    sun_angle_deg: f32,
    heading_deg: f32,
    color: Color,
    seed: u32,
) {
    draw_concentric_guides(canvas, cx, cy, radius * 0.45, radius, color, seed);

    let sun_rad = sun_angle_deg.to_radians();
    let head_rad = heading_deg.to_radians();

    // Rayo hacia el Sol (dorado / ámbar)
    let sun_color = Color::hex("#ffd040");
    let sx = cx + sun_rad.cos() * (radius * 1.25);
    let sy = cy + sun_rad.sin() * (radius * 1.25);
    canvas.stroke_line(cx, cy, sx, sy, sun_color, 1.8);
    canvas.fill_circle(sx, sy, 7.0, sun_color);
    for i in 0..8 {
        let a = (i as f32) * PI * 0.25;
        let rx1 = sx + a.cos() * 9.0;
        let ry1 = sy + a.sin() * 9.0;
        let rx2 = sx + a.cos() * 15.0;
        let ry2 = sy + a.sin() * 15.0;
        canvas.stroke_line(rx1, ry1, rx2, ry2, sun_color, 1.2);
    }
    draw_vector_text(canvas, sx + 20.0, sy - 8.0, "SOLAR AZIMUTH", 10.0, sun_color, false);

    // Vector de rumbo migratorio (Rosa/Magenta hacia el Sur)
    let head_color = Color::hex("#ff3d9b");
    let hx = cx + head_rad.cos() * (radius * 1.15);
    let hy = cy + head_rad.sin() * (radius * 1.15);
    canvas.stroke_line(cx, cy, hx, hy, head_color, 2.2);

    // Flecha de rumbo
    let arrow_sz = 14.0;
    let wing_ang1 = head_rad + PI * 0.85;
    let wing_ang2 = head_rad - PI * 0.85;
    canvas.stroke_line(hx, hy, hx + wing_ang1.cos() * arrow_sz, hy + wing_ang1.sin() * arrow_sz, head_color, 2.0);
    canvas.stroke_line(hx, hy, hx + wing_ang2.cos() * arrow_sz, hy + wing_ang2.sin() * arrow_sz, head_color, 2.0);
    draw_vector_text(canvas, hx + 18.0, hy + 6.0, "FLIGHT HEADING 210° (SSW)", 10.0, head_color, false);

    // Arco de compensación de ángulo
    let arc_steps = 24;
    let arc_r = radius * 0.7;
    let start_a = sun_rad.min(head_rad);
    let end_a = sun_rad.max(head_rad);
    let mut arc_pb = tiny_skia::PathBuilder::new();
    for i in 0..=arc_steps {
        let a = start_a + (end_a - start_a) * (i as f32 / arc_steps as f32);
        let px = cx + a.cos() * arc_r;
        let py = cy + a.sin() * arc_r;
        if i == 0 {
            arc_pb.move_to(px, py);
        } else {
            arc_pb.line_to(px, py);
        }
    }
    if let Some(path) = arc_pb.finish() {
        canvas.stroke_path(&path, Color::hex("#00f0ff").with_alpha(0.8), 1.0);
    }
    draw_vector_text(canvas, cx + ((start_a + end_a) * 0.5).cos() * (arc_r - 20.0), cy + ((start_a + end_a) * 0.5).sin() * (arc_r - 20.0), "CIRCADIAN ANGLE", 9.0, Color::hex("#00f0ff"), true);
}

/// Dibuja un cuadro de llamada o nodo anotador (Callout / Node Glyph)
pub fn draw_node_callout(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    label: &str,
    sublabel: &str,
    color: Color,
    dir_right: bool,
) {
    let sign = if dir_right { 1.0 } else { -1.0 };
    // Punto central con anillo
    canvas.fill_circle(x, y, 4.0, color);
    canvas.stroke_circle(x, y, 9.0, color, 1.0);

    // Línea directriz en quiebre técnico
    let arm_x = x + sign * 45.0;
    let arm_y = y - 25.0;
    let end_x = arm_x + sign * 110.0;

    canvas.stroke_line(x, y, arm_x, arm_y, color, 1.0);
    canvas.stroke_line(arm_x, arm_y, end_x, arm_y, color, 1.0);

    // Textos de la etiqueta
    let text_x = if dir_right { arm_x + 8.0 } else { arm_x - 8.0 };
    draw_vector_text(canvas, text_x, arm_y - 8.0, label, 11.0, color, !dir_right);
    draw_vector_text(canvas, text_x, arm_y + 14.0, sublabel, 8.5, color.with_alpha(color.a * 0.75), !dir_right);
}

/// Punteado estocástico para sombreado orgánico de volumen (Stippling)
pub fn draw_stipple_area(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    count: usize,
    color: Color,
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    for _ in 0..count {
        // Distribución elíptica con densidad concentrada hacia el centro o borde
        let u = rng.next_f32();
        let theta = rng.next_f32() * PI * 2.0;
        let r = u.sqrt(); // Distribución uniforme en área
        let px = cx + rx * r * theta.cos();
        let py = cy + ry * r * theta.sin();

        let dot_r = 0.6 + rng.next_f32() * 1.1;
        let alpha = color.a * (0.35 + rng.next_f32() * 0.55);
        canvas.fill_circle(px, py, dot_r, color.with_alpha(alpha));
    }
}

/// Characters the vector font can draw (letters are drawn upper case).
pub const VECTOR_CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .:-/°(),'!?";

/// Dibuja texto vectorial técnico sin requerir fuentes externas (ver VECTOR_CHARSET)
pub fn draw_vector_text(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    text: &str,
    size: f32,
    color: Color,
    align_center: bool,
) {
    let stroke_w = (size * 0.12).clamp(0.8, 2.5);
    let char_w = size * 0.62;
    let char_h = size;
    let spacing = size * 0.22;

    let chars: Vec<char> = text.chars().collect();
    let total_w = chars.len() as f32 * (char_w + spacing) - spacing;

    let start_x = if align_center { x - total_w * 0.5 } else { x };

    for (i, &ch) in chars.iter().enumerate() {
        let cx = start_x + (i as f32) * (char_w + spacing);
        let cy = y;

        draw_single_char(canvas, cx, cy, char_w, char_h, ch, color, stroke_w);
    }
}

fn draw_single_char(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    c: char,
    color: Color,
    sw: f32,
) {
    let c = c.to_ascii_uppercase();
    let r = x + w;
    let b = y + h;
    let my = y + h * 0.5;
    let mx = x + w * 0.5;

    match c {
        'A' => {
            canvas.stroke_line(x, b, mx, y, color, sw);
            canvas.stroke_line(mx, y, r, b, color, sw);
            canvas.stroke_line(x + w * 0.2, my, r - w * 0.2, my, color, sw);
        }
        'B' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, y, r - w * 0.2, y, color, sw);
            canvas.stroke_line(r - w * 0.2, y, r, (y + my) * 0.5, color, sw);
            canvas.stroke_line(r, (y + my) * 0.5, x, my, color, sw);
            canvas.stroke_line(x, my, r, (my + b) * 0.5, color, sw);
            canvas.stroke_line(r, (my + b) * 0.5, r - w * 0.2, b, color, sw);
            canvas.stroke_line(r - w * 0.2, b, x, b, color, sw);
        }
        'C' => {
            canvas.stroke_line(r, y, x, y, color, sw);
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
        }
        'D' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, y, r - w * 0.3, y, color, sw);
            canvas.stroke_line(r - w * 0.3, y, r, my, color, sw);
            canvas.stroke_line(r, my, r - w * 0.3, b, color, sw);
            canvas.stroke_line(r - w * 0.3, b, x, b, color, sw);
        }
        'E' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(x, my, r - w * 0.2, my, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
        }
        'F' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(x, my, r - w * 0.2, my, color, sw);
        }
        'G' => {
            canvas.stroke_line(r, y, x, y, color, sw);
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
            canvas.stroke_line(r, b, r, my, color, sw);
            canvas.stroke_line(r, my, mx, my, color, sw);
        }
        'H' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(r, y, r, b, color, sw);
            canvas.stroke_line(x, my, r, my, color, sw);
        }
        'I' => {
            canvas.stroke_line(mx, y, mx, b, color, sw);
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
        }
        'J' => {
            canvas.stroke_line(r, y, r, b - h * 0.2, color, sw);
            canvas.stroke_line(r, b - h * 0.2, mx, b, color, sw);
            canvas.stroke_line(mx, b, x, b - h * 0.2, color, sw);
        }
        'K' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, my, r, y, color, sw);
            canvas.stroke_line(x, my, r, b, color, sw);
        }
        'L' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
        }
        'M' => {
            canvas.stroke_line(x, b, x, y, color, sw);
            canvas.stroke_line(x, y, mx, my, color, sw);
            canvas.stroke_line(mx, my, r, y, color, sw);
            canvas.stroke_line(r, y, r, b, color, sw);
        }
        'N' => {
            canvas.stroke_line(x, b, x, y, color, sw);
            canvas.stroke_line(x, y, r, b, color, sw);
            canvas.stroke_line(r, b, r, y, color, sw);
        }
        'O' | '0' => {
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, b, color, sw);
            canvas.stroke_line(r, b, x, b, color, sw);
            canvas.stroke_line(x, b, x, y, color, sw);
        }
        'P' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, my, color, sw);
            canvas.stroke_line(r, my, x, my, color, sw);
        }
        'Q' => {
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, b, color, sw);
            canvas.stroke_line(r, b, x, b, color, sw);
            canvas.stroke_line(x, b, x, y, color, sw);
            canvas.stroke_line(mx, my, r + w * 0.1, b + h * 0.1, color, sw);
        }
        'R' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, my, color, sw);
            canvas.stroke_line(r, my, x, my, color, sw);
            canvas.stroke_line(x, my, r, b, color, sw);
        }
        'S' => {
            canvas.stroke_line(r, y, x, y, color, sw);
            canvas.stroke_line(x, y, x, my, color, sw);
            canvas.stroke_line(x, my, r, my, color, sw);
            canvas.stroke_line(r, my, r, b, color, sw);
            canvas.stroke_line(r, b, x, b, color, sw);
        }
        'T' => {
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(mx, y, mx, b, color, sw);
        }
        'U' => {
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
            canvas.stroke_line(r, b, r, y, color, sw);
        }
        'V' => {
            canvas.stroke_line(x, y, mx, b, color, sw);
            canvas.stroke_line(mx, b, r, y, color, sw);
        }
        'W' => {
            canvas.stroke_line(x, y, x + w * 0.25, b, color, sw);
            canvas.stroke_line(x + w * 0.25, b, mx, my, color, sw);
            canvas.stroke_line(mx, my, r - w * 0.25, b, color, sw);
            canvas.stroke_line(r - w * 0.25, b, r, y, color, sw);
        }
        'X' => {
            canvas.stroke_line(x, y, r, b, color, sw);
            canvas.stroke_line(r, y, x, b, color, sw);
        }
        'Y' => {
            canvas.stroke_line(x, y, mx, my, color, sw);
            canvas.stroke_line(r, y, mx, my, color, sw);
            canvas.stroke_line(mx, my, mx, b, color, sw);
        }
        'Z' => {
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, x, b, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
        }
        '1' => {
            canvas.stroke_line(mx - w * 0.2, y + h * 0.2, mx, y, color, sw);
            canvas.stroke_line(mx, y, mx, b, color, sw);
            canvas.stroke_line(mx - w * 0.3, b, mx + w * 0.3, b, color, sw);
        }
        '2' => {
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, my, color, sw);
            canvas.stroke_line(r, my, x, b, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
        }
        '3' => {
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, b, color, sw);
            canvas.stroke_line(x, my, r, my, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
        }
        '4' => {
            canvas.stroke_line(x, y, x, my, color, sw);
            canvas.stroke_line(x, my, r, my, color, sw);
            canvas.stroke_line(r - w * 0.25, y, r - w * 0.25, b, color, sw);
        }
        '5' => {
            canvas.stroke_line(r, y, x, y, color, sw);
            canvas.stroke_line(x, y, x, my, color, sw);
            canvas.stroke_line(x, my, r, my, color, sw);
            canvas.stroke_line(r, my, r, b, color, sw);
            canvas.stroke_line(r, b, x, b, color, sw);
        }
        '6' => {
            canvas.stroke_line(r, y, x, y, color, sw);
            canvas.stroke_line(x, y, x, b, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
            canvas.stroke_line(r, b, r, my, color, sw);
            canvas.stroke_line(r, my, x, my, color, sw);
        }
        '7' => {
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, mx, b, color, sw);
        }
        '8' => {
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, b, color, sw);
            canvas.stroke_line(r, b, x, b, color, sw);
            canvas.stroke_line(x, b, x, y, color, sw);
            canvas.stroke_line(x, my, r, my, color, sw);
        }
        '9' => {
            canvas.stroke_line(x, my, r, my, color, sw);
            canvas.stroke_line(x, y, x, my, color, sw);
            canvas.stroke_line(x, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, b, color, sw);
            canvas.stroke_line(x, b, r, b, color, sw);
        }
        '.' => {
            canvas.fill_circle(mx, b - sw, sw * 1.2, color);
        }
        ':' => {
            canvas.fill_circle(mx, my - h * 0.2, sw * 1.1, color);
            canvas.fill_circle(mx, my + h * 0.2, sw * 1.1, color);
        }
        '-' => {
            canvas.stroke_line(x, my, r, my, color, sw);
        }
        ',' => {
            canvas.fill_circle(mx, b - sw, sw * 1.2, color);
            canvas.stroke_line(mx, b - sw, mx - w * 0.15, b + h * 0.15, color, sw);
        }
        '\'' => {
            canvas.stroke_line(mx, y, mx, y + h * 0.25, color, sw);
        }
        '!' => {
            canvas.stroke_line(mx, y, mx, b - h * 0.3, color, sw);
            canvas.fill_circle(mx, b - sw, sw * 1.2, color);
        }
        '?' => {
            canvas.stroke_line(x, y + h * 0.15, x + w * 0.2, y, color, sw);
            canvas.stroke_line(x + w * 0.2, y, r, y, color, sw);
            canvas.stroke_line(r, y, r, my - h * 0.1, color, sw);
            canvas.stroke_line(r, my - h * 0.1, mx, my, color, sw);
            canvas.stroke_line(mx, my, mx, b - h * 0.3, color, sw);
            canvas.fill_circle(mx, b - sw, sw * 1.2, color);
        }
        '/' => {
            canvas.stroke_line(x, b, r, y, color, sw);
        }
        '°' => {
            canvas.stroke_circle(mx, y + h * 0.2, w * 0.25, color, sw * 0.8);
        }
        '(' => {
            canvas.stroke_line(r, y, x, my, color, sw);
            canvas.stroke_line(x, my, r, b, color, sw);
        }
        ')' => {
            canvas.stroke_line(x, y, r, my, color, sw);
            canvas.stroke_line(r, my, x, b, color, sw);
        }
        _ => {}
    }
}

/// Dibuja la franja superior arquitectónica de anteproyecto (header técnico)
pub fn draw_blueprint_header(canvas: &mut Canvas, width: f32, title: &str, subtitle: &str) {
    let lavender = Color::hex("#C8C1EF");
    let white = Color::hex("#EEF0FF");
    let faint = lavender.with_alpha(0.35);

    let y0 = 60.0;
    let y1 = 110.0;
    let margin = 60.0;

    // Líneas horizontales del encabezado
    canvas.stroke_line(margin, y0, width - margin, y0, lavender.with_alpha(0.6), 1.2);
    canvas.stroke_line(margin, y1, width - margin, y1, lavender.with_alpha(0.6), 1.2);

    // Escala métrica superior con marcas de calibración
    let mut x = margin;
    let mut idx = 0;
    while x <= width - margin {
        let is_major = idx % 5 == 0;
        let tick_h = if is_major { 16.0 } else { 8.0 };
        let col = if is_major { white.with_alpha(0.7) } else { faint };
        canvas.stroke_line(x, y0, x, y0 + tick_h, col, 1.0);
        x += 20.0;
        idx += 1;
    }

    // Título y subtítulo en tipografía técnica
    draw_vector_text(canvas, margin + 20.0, y0 + 32.0, title, 13.0, white.with_alpha(0.9), false);
    draw_vector_text(canvas, width - margin - 260.0, y0 + 32.0, subtitle, 10.0, lavender.with_alpha(0.7), false);
}

/// Dibuja el glifo de cuadrante del ciclo biológico (Egg, Larva, Pupa, Imago)
pub fn draw_stage_cycle_glyph(canvas: &mut Canvas, cx: f32, cy: f32, radius: f32, active_stage: usize) {
    let lavender = Color::hex("#C8C1EF");
    let white = Color::hex("#EEF0FF");
    let r = radius;

    // Círculo base tenue
    canvas.stroke_circle(cx, cy, r, lavender.with_alpha(0.2), 1.0);
    canvas.stroke_circle(cx, cy, r * 0.45, lavender.with_alpha(0.15), 1.0);

    // 4 cuadrantes correspondientes a Huevo (0: top), Larva (1: right), Crisálida (2: bottom), Adulto (3: left)
    let stage_names = ["EGG", "LARVA", "PUPA", "IMAGO"];
    for i in 0..4 {
        let a_start = (i as f32) * PI * 0.5 - PI * 0.5 + 0.08;
        let a_end = (i as f32 + 1.0) * PI * 0.5 - PI * 0.5 - 0.08;
        let is_active = i == active_stage;
        let col = if is_active { white } else { lavender.with_alpha(0.3) };
        let sw = if is_active { 3.0 } else { 1.2 };

        // Dibujar arco
        let steps = 12;
        for s in 0..steps {
            let t0 = a_start + (a_end - a_start) * (s as f32 / steps as f32);
            let t1 = a_start + (a_end - a_start) * ((s + 1) as f32 / steps as f32);
            canvas.stroke_line(cx + t0.cos() * r, cy + t0.sin() * r, cx + t1.cos() * r, cy + t1.sin() * r, col, sw);
        }

        // Marcador o punto en el centro del arco
        let mid_a = (a_start + a_end) * 0.5;
        let dot_r = if is_active { 3.5 } else { 1.5 };
        canvas.fill_circle(cx + mid_a.cos() * r, cy + mid_a.sin() * r, dot_r, col);
    }

    // Texto de la fase activa debajo
    if active_stage < 4 {
        draw_vector_text(canvas, cx, cy + r + 24.0, stage_names[active_stage], 9.0, white.with_alpha(0.85), true);
    }
}

/// Dibuja un retículo de lupa de aumento (magnifying loupe) con graduaciones
pub fn draw_magnifying_loupe(canvas: &mut Canvas, cx: f32, cy: f32, radius: f32, label: &str, color: Color) {
    // Aro exterior doble
    canvas.stroke_circle(cx, cy, radius, color.with_alpha(0.85), 2.2);
    canvas.stroke_circle(cx, cy, radius + 5.0, color.with_alpha(0.35), 1.0);
    canvas.stroke_circle(cx, cy, radius - 6.0, color.with_alpha(0.2), 0.8);

    // Cruz central de retículo
    let cross = 18.0;
    canvas.stroke_line(cx - cross, cy, cx + cross, cy, color.with_alpha(0.6), 1.0);
    canvas.stroke_line(cx, cy - cross, cx, cy + cross, color.with_alpha(0.6), 1.0);

    // Marcas circulares cada 30 grados
    for i in 0..12 {
        let a = i as f32 * PI / 6.0;
        let r1 = radius - 8.0;
        let r2 = radius - (if i % 3 == 0 { 16.0 } else { 11.0 });
        canvas.stroke_line(cx + a.cos() * r1, cy + a.sin() * r1, cx + a.cos() * r2, cy + a.sin() * r2, color.with_alpha(0.5), 1.0);
    }

    if !label.is_empty() {
        draw_vector_text(canvas, cx, cy + radius + 20.0, label, 10.0, color, true);
    }
}

/// Dibuja las 5 fases lunares en la parte superior (Oyamel winter)
pub fn draw_moon_phases(canvas: &mut Canvas, cx: f32, y: f32, width: f32) {
    let white = Color::hex("#FBF6EA");
    let faint = white.with_alpha(0.25);
    let r = 16.0;
    let spacing = width / 6.0;

    let phases = [0.15, 0.4, 0.98, 0.6, 0.15]; // creciente, cuarto, llena, menguante, creciente

    for (i, &illum) in phases.iter().enumerate() {
        let mx = cx - 2.0 * spacing + (i as f32) * spacing;
        // Círculo lunar tenue base
        canvas.fill_circle(mx, y, r, Color::hex("#2F2748").with_alpha(0.8));
        canvas.stroke_circle(mx, y, r, faint, 1.0);

        if illum > 0.9 {
            // Luna llena
            canvas.fill_circle(mx, y, r - 1.0, white.with_alpha(0.9));
        } else {
            // Creciente / menguante aproximada
            let side = if i < 2 { 1.0 } else { -1.0 };
            let mut pb = tiny_skia::PathBuilder::new();
            pb.move_to(mx, y - r);
            pb.cubic_to(mx + side * r * 1.3, y - r * 0.5, mx + side * r * 1.3, y + r * 0.5, mx, y + r);
            pb.cubic_to(mx + side * r * (illum * 2.0 - 1.0), y + r * 0.5, mx + side * r * (illum * 2.0 - 1.0), y - r * 0.5, mx, y - r);
            pb.close();
            if let Some(p) = pb.finish() {
                canvas.fill_path(&p, white.with_alpha(0.85));
            }
        }
    }
}
