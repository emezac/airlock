//! Puppet Artesanal: La Tortuga Soñadora (Tortoise Puppet)
//! Estilo: Collage de papel recortado y telas patchwork (*cut-paper / riso / fabric*),
//! inspirado en la obra de Kevin Ngo (@kevin_t_ngo).

use crate::core::canvas::Canvas;
use crate::core::color::Color;
use std::f32::consts::PI;
use tiny_skia::PathBuilder;


#[derive(Clone, Debug)]
pub struct TortoisePuppet {
    pub cx: f32,
    pub cy: f32,
    pub scale: f32,
    pub neck_extension: f32, // 0.0 (retraído) a 1.0 (totalmente extendido mirando al cielo)
    pub neck_angle: f32,     // Ángulo del cuello en radianes (-0.2 a 0.8)
    pub eye_blink: f32,      // 0.0 (ojo abierto) a 1.0 (parpadeo cerrado)
    pub walk_phase: f32,     // Fase del ciclo de caminata (0.0 .. 2*PI)
    pub is_walking: bool,
    pub has_feather: bool,   // Si sostiene la pluma de ganso en el pico
    pub in_basket: bool,     // Si está posada dentro de la canasta del globo
    pub mouth_open: f32,     // 0.0 (cerrada con sonrisa) a 1.0 (abierta de asombro)
}

impl Default for TortoisePuppet {
    fn default() -> Self {
        Self {
            cx: 400.0,
            cy: 700.0,
            scale: 1.0,
            neck_extension: 0.8,
            neck_angle: 0.35, // Mirando hacia arriba
            eye_blink: 0.0,
            walk_phase: 0.0,
            is_walking: false,
            has_feather: false,
            in_basket: false,
            mouth_open: 0.0,
        }
    }
}

impl TortoisePuppet {
    pub fn new(cx: f32, cy: f32, scale: f32) -> Self {
        Self {
            cx,
            cy,
            scale,
            ..Default::default()
        }
    }

    /// Renderiza la tortuga completa con todas sus articulaciones y patchwork
    pub fn draw(&self, canvas: &mut Canvas) {
        let s = self.scale;
        let cx = self.cx;
        let cy = self.cy;

        // Paleta de colores cálidos de retazos textiles / papel
        let col_skin = Color::hex("#95a76e");      // Verde salvia/oliva suave
        let col_skin_dark = Color::hex("#778754"); // Sombra de piel
        let col_outline = Color::hex("#2a2016");   // Trazado tinta sepia profunda

        // 1. Cola posterior (si no está en canasta)
        if !self.in_basket {
            let tail_x = cx - 125.0 * s;
            let tail_y = cy - 8.0 * s;
            let mut pb = PathBuilder::new();
            pb.move_to(tail_x, tail_y - 12.0 * s);
            pb.line_to(tail_x - 30.0 * s, tail_y + 4.0 * s);
            pb.line_to(tail_x + 6.0 * s, tail_y + 10.0 * s);
            pb.close();
            if let Some(path) = pb.finish() {
                canvas.fill_path(&path, col_skin_dark);
                canvas.stroke_path(&path, col_outline, 2.0 * s);
            }
        }

        // 2. Patas traseras y delanteras (articuladas con ciclo de marcha si camina)
        if !self.in_basket {
            let walk = if self.is_walking { self.walk_phase } else { 0.0 };

            // Pata trasera (lejana)
            let leg_b_ang = (walk + PI).sin() * 0.22;
            let leg_b_x = cx - 70.0 * s + (walk + PI).cos() * 8.0 * s;
            let leg_b_y = cy + 10.0 * s;
            draw_leg(canvas, leg_b_x, leg_b_y, s, leg_b_ang, col_skin_dark, col_outline);

            // Pata delantera (cercana)
            let leg_f_ang = walk.sin() * 0.25;
            let leg_f_x = cx + 55.0 * s + walk.cos() * 10.0 * s;
            let leg_f_y = cy + 10.0 * s;
            draw_leg(canvas, leg_f_x, leg_f_y, s, leg_f_ang, col_skin, col_outline);
        } else {
            // Patitas apoyadas con ternura sobre el borde de la canasta
            let paw_x = cx + 38.0 * s;
            let paw_y = cy + 8.0 * s;
            canvas.fill_circle(paw_x, paw_y, 9.0 * s, col_skin);
            canvas.stroke_circle(paw_x, paw_y, 9.0 * s, col_outline, 2.0 * s);
            canvas.fill_circle(paw_x + 14.0 * s, paw_y + 1.0 * s, 8.0 * s, col_skin);
            canvas.stroke_circle(paw_x + 14.0 * s, paw_y + 1.0 * s, 8.0 * s, col_outline, 1.8 * s);
        }

        // 3. Cuello y Cabeza (articulados y expresivos)
        let neck_base_x = cx + 85.0 * s;
        let neck_base_y = cy - 25.0 * s;

        // Longitud del cuello modulada por neck_extension
        let neck_len = (75.0 + 45.0 * self.neck_extension) * s;
        let n_ang = -self.neck_angle - 0.45; // Orientación diagonal hacia arriba-derecha

        let head_center_x = neck_base_x + n_ang.cos() * neck_len;
        let head_center_y = neck_base_y + n_ang.sin() * neck_len;

        // Trazar cuello como un tubo orgánico ahusado
        let mut pb_neck = PathBuilder::new();
        let perp_x = -n_ang.sin();
        let perp_y = n_ang.cos();
        let w_base = 22.0 * s;
        let w_top = 18.0 * s;

        pb_neck.move_to(neck_base_x - perp_x * w_base, neck_base_y - perp_y * w_base);
        pb_neck.line_to(head_center_x - perp_x * w_top, head_center_y - perp_y * w_top);
        pb_neck.line_to(head_center_x + perp_x * w_top, head_center_y + perp_y * w_top);
        pb_neck.line_to(neck_base_x + perp_x * w_base, neck_base_y + perp_y * w_base);
        pb_neck.close();

        if let Some(path_neck) = pb_neck.finish() {
            canvas.fill_path(&path_neck, col_skin);
            canvas.stroke_path(&path_neck, col_outline, 2.2 * s);

            // Líneas de arrugas sutiles en el cuello
            for i in 1..4 {
                let t = i as f32 / 4.0;
                let nx = neck_base_x + (head_center_x - neck_base_x) * t;
                let ny = neck_base_y + (head_center_y - neck_base_y) * t;
                let nw = w_base + (w_top - w_base) * t;
                canvas.stroke_line(
                    nx - perp_x * (nw * 0.7),
                    ny - perp_y * (nw * 0.7),
                    nx + perp_x * (nw * 0.7),
                    ny + perp_y * (nw * 0.7),
                    col_skin_dark,
                    1.4 * s,
                );
            }
        }

        // 4. Cabeza
        let head_rot = n_ang + 0.35;
        let h_rad_x = 34.0 * s;
        let h_rad_y = 26.0 * s;

        // Cabeza ovoide
        canvas.save();
        canvas.translate(head_center_x, head_center_y);
        canvas.rotate(head_rot);

        let mut pb_head = PathBuilder::new();
        pb_head.push_circle(0.0, 0.0, h_rad_y);
        // Alargamiento hacia el hocico
        pb_head.move_to(0.0, -h_rad_y);
        pb_head.cubic_to(h_rad_x * 0.9, -h_rad_y * 0.7, h_rad_x * 1.1, 0.0, h_rad_x * 0.9, h_rad_y * 0.8);
        pb_head.line_to(0.0, h_rad_y);
        pb_head.close();

        if let Some(path_head) = pb_head.finish() {
            canvas.fill_path(&path_head, col_skin);
            canvas.stroke_path(&path_head, col_outline, 2.2 * s);
        }

        // Ojo grande expresivo
        let eye_x = 4.0 * s;
        let eye_y = -8.0 * s;
        let eye_r = 8.5 * s;

        if self.eye_blink > 0.85 {
            // Ojo cerrado feliz (arco)
            canvas.stroke_line(eye_x - 7.0 * s, eye_y, eye_x + 7.0 * s, eye_y, col_outline, 2.5 * s);
        } else {
            // Esclerótica blanca
            canvas.fill_circle(eye_x, eye_y, eye_r, Color::hex("#ffffff"));
            canvas.stroke_circle(eye_x, eye_y, eye_r, col_outline, 1.8 * s);

            // Pupila orientada hacia arriba y al frente (mirando a los gansos / cielo)
            let pup_x = eye_x + 3.0 * s;
            let pup_y = eye_y - 2.5 * s;
            let pup_r = 4.8 * s;
            canvas.fill_circle(pup_x, pup_y, pup_r, Color::hex("#2a2016"));

            // Brillo blanco en la pupila
            canvas.fill_circle(pup_x + 1.6 * s, pup_y - 1.6 * s, 1.8 * s, Color::hex("#ffffff"));
        }

        // Sonrisa curvada en el pico
        let snout_tip_x = h_rad_x * 0.95;
        let snout_tip_y = 6.0 * s;
        let mut pb_smile = PathBuilder::new();
        pb_smile.move_to(snout_tip_x, snout_tip_y);
        pb_smile.cubic_to(
            snout_tip_x - 10.0 * s,
            snout_tip_y + 3.0 * s,
            snout_tip_x - 18.0 * s,
            snout_tip_y + 1.0 * s,
            snout_tip_x - 24.0 * s,
            snout_tip_y - 3.0 * s,
        );
        if let Some(path_smile) = pb_smile.finish() {
            canvas.stroke_path(&path_smile, col_outline, 2.0 * s);
        }

        // 5. Pluma de Ganso sostenida en el pico (si has_feather está activo)
        if self.has_feather {
            draw_feather(canvas, snout_tip_x + 2.0 * s, snout_tip_y - 2.0 * s, s);
        }

        canvas.restore();

        // 6. Caparazón Patchwork / Mosaico (The Carapace)
        draw_patchwork_shell(canvas, cx, cy, s, self.in_basket);

        // 7. Pata delantera reposando en el borde si está en la canasta
        if self.in_basket {
            let claw_x = cx + 80.0 * s;
            let claw_y = cy + 12.0 * s;
            canvas.fill_circle(claw_x, claw_y, 14.0 * s, col_skin);
            canvas.stroke_circle(claw_x, claw_y, 14.0 * s, col_outline, 2.0 * s);
            for i in 0..3 {
                let tx = claw_x + (i as f32 * 5.0 - 5.0) * s;
                let ty = claw_y + 10.0 * s;
                canvas.fill_circle(tx, ty, 3.2 * s, Color::hex("#f0e6d2"));
                canvas.stroke_circle(tx, ty, 3.2 * s, col_outline, 1.2 * s);
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Sub-funciones de Dibujo: Patas, Pluma y Caparazón Mosaico
// ─────────────────────────────────────────────────────────────────────────────

fn draw_leg(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    scale: f32,
    angle: f32,
    fill_col: Color,
    outline_col: Color,
) {
    let s = scale;
    canvas.save();
    canvas.translate(x, y);
    canvas.rotate(angle);

    let mut pb = PathBuilder::new();
    let w_top = 22.0 * s;
    let w_bot = 28.0 * s;
    let h = 48.0 * s;

    pb.move_to(-w_top, 0.0);
    pb.line_to(w_top, 0.0);
    pb.line_to(w_bot, h);
    pb.line_to(-w_bot, h);
    pb.close();

    if let Some(path) = pb.finish() {
        canvas.fill_path(&path, fill_col);
        canvas.stroke_path(&path, outline_col, 2.2 * s);

        // Uñas redondeadas en la pezuña
        for i in 0..3 {
            let nx = -w_bot + 8.0 * s + i as f32 * 10.0 * s;
            let ny = h - 2.0 * s;
            canvas.fill_circle(nx, ny, 4.0 * s, Color::hex("#ebe0cb"));
            canvas.stroke_circle(nx, ny, 4.0 * s, outline_col, 1.4 * s);
        }
    }

    canvas.restore();
}

fn draw_feather(canvas: &mut Canvas, tip_x: f32, tip_y: f32, scale: f32) {
    let s = scale;
    canvas.save();
    canvas.translate(tip_x, tip_y);
    canvas.rotate(-0.35); // Pluma inclinada hacia arriba

    let col_quill = Color::hex("#f0ede6");
    let col_vane_dark = Color::hex("#3a3835"); // Puntas gris carbón
    let col_vane_light = Color::hex("#7a756d"); // Gris medio

    // Raquis / eje central de la pluma
    canvas.stroke_line(0.0, 0.0, 48.0 * s, 0.0, col_quill, 1.8 * s);

    // Vexilo superior e inferior
    let mut pb_vane = PathBuilder::new();
    pb_vane.move_to(8.0 * s, 0.0);
    pb_vane.cubic_to(20.0 * s, -12.0 * s, 36.0 * s, -10.0 * s, 48.0 * s, 0.0);
    pb_vane.cubic_to(36.0 * s, 8.0 * s, 20.0 * s, 10.0 * s, 8.0 * s, 0.0);
    pb_vane.close();

    if let Some(path) = pb_vane.finish() {
        canvas.fill_path(&path, col_vane_light);
        canvas.stroke_path(&path, col_vane_dark, 1.5 * s);
    }

    canvas.restore();
}

fn draw_patchwork_shell(canvas: &mut Canvas, cx: f32, cy: f32, scale: f32, in_basket: bool) {
    let s = scale;
    let col_line = Color::hex("#2a1f18");

    // Paleta de colores otoñales de retazos (Patchwork Scutes)
    let p_olive = Color::hex("#5c6d3d");
    let p_brown_dark = Color::hex("#52331c");
    let p_amber = Color::hex("#cc8e35");
    let p_ochre = Color::hex("#deb356");
    let p_terracotta = Color::hex("#9e472a");
    let p_tan = Color::hex("#b89260");

    let rx = 120.0 * s;
    let ry = 80.0 * s;
    let shell_base_y = cy + 12.0 * s;

    // 1. Cúpula general del caparazón (fondo base)
    let mut pb_dome = PathBuilder::new();
    pb_dome.move_to(cx - rx, shell_base_y);
    pb_dome.cubic_to(
        cx - rx * 0.95, shell_base_y - ry * 1.35,
        cx + rx * 0.95, shell_base_y - ry * 1.35,
        cx + rx, shell_base_y
    );
    pb_dome.line_to(cx - rx, shell_base_y);
    pb_dome.close();

    let path_dome = pb_dome.finish();
    if let Some(ref pd) = path_dome {
        canvas.fill_path(pd, p_brown_dark);
    }

    // 2. Mosaico de placas (Scutes) en 3 hileras
    // Hilera Superior (3 placas redondeadas)
    let scutes_top = [
        (cx - 70.0 * s, shell_base_y - 65.0 * s, 36.0 * s, 26.0 * s, p_olive),
        (cx - 10.0 * s, shell_base_y - 75.0 * s, 42.0 * s, 28.0 * s, p_amber),
        (cx + 50.0 * s, shell_base_y - 65.0 * s, 38.0 * s, 26.0 * s, p_terracotta),
    ];
    for (sx, sy, sw, sh, col) in scutes_top {
        draw_scute(canvas, sx, sy, sw, sh, col, col_line, 2.0 * s);
    }

    // Hilera Media (4 placas centrales grandes)
    let scutes_mid = [
        (cx - 85.0 * s, shell_base_y - 40.0 * s, 40.0 * s, 32.0 * s, p_ochre),
        (cx - 40.0 * s, shell_base_y - 46.0 * s, 44.0 * s, 36.0 * s, p_brown_dark),
        (cx + 8.0 * s, shell_base_y - 46.0 * s, 44.0 * s, 36.0 * s, p_ochre),
        (cx + 55.0 * s, shell_base_y - 40.0 * s, 40.0 * s, 32.0 * s, p_olive),
    ];
    for (sx, sy, sw, sh, col) in scutes_mid {
        draw_scute(canvas, sx, sy, sw, sh, col, col_line, 2.2 * s);
    }

    // Hilera Inferior (6 placas marginales rectangulares)
    let scutes_bot = [
        (cx - 95.0 * s, shell_base_y - 12.0 * s, 30.0 * s, 20.0 * s, p_terracotta),
        (cx - 60.0 * s, shell_base_y - 12.0 * s, 34.0 * s, 20.0 * s, p_amber),
        (cx - 22.0 * s, shell_base_y - 12.0 * s, 36.0 * s, 20.0 * s, p_tan),
        (cx + 18.0 * s, shell_base_y - 12.0 * s, 36.0 * s, 20.0 * s, p_ochre),
        (cx + 58.0 * s, shell_base_y - 12.0 * s, 34.0 * s, 20.0 * s, p_olive),
        (cx + 90.0 * s, shell_base_y - 12.0 * s, 26.0 * s, 20.0 * s, p_brown_dark),
    ];
    for (sx, sy, sw, sh, col) in scutes_bot {
        draw_scute(canvas, sx, sy, sw, sh, col, col_line, 2.0 * s);
    }

    // 3. Contorno general nítido de la cúpula
    if let Some(ref pd) = path_dome {
        canvas.stroke_path(pd, col_line, 3.2 * s);
    }

    // 4. Franja base del plastrón (borde inferior)
    if !in_basket {
        let mut pb_base = PathBuilder::new();
        pb_base.move_to(cx - rx - 4.0 * s, shell_base_y);
        pb_base.line_to(cx + rx + 4.0 * s, shell_base_y);
        pb_base.line_to(cx + rx * 0.85, shell_base_y + 12.0 * s);
        pb_base.line_to(cx - rx * 0.85, shell_base_y + 12.0 * s);
        pb_base.close();
        if let Some(path_base) = pb_base.finish() {
            canvas.fill_path(&path_base, Color::hex("#dfcf9f"));
            canvas.stroke_path(&path_base, col_line, 2.5 * s);
        }
    }
}

fn draw_scute(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    fill_col: Color,
    line_col: Color,
    line_width: f32,
) {
    let mut pb = PathBuilder::new();
    if let Some(rect) = tiny_skia::Rect::from_xywh(x, y, w, h) {
        pb.push_rect(rect);
        if let Some(path) = pb.finish() {
            canvas.fill_path(&path, fill_col);
            canvas.stroke_path(&path, line_col, line_width);
        }
    }
}
