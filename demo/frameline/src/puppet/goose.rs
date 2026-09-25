//! Puppet Artesanal: Ganso Canadiense y Bandada en "V" (Goose Puppet)
//! Estilo: Collage de papel recortado y siluetas articuladas con batido de alas orgánico.

use crate::core::canvas::Canvas;
use crate::core::color::Color;
use std::f32::consts::PI;
use tiny_skia::{PathBuilder, Stroke};

#[derive(Clone, Debug)]
pub struct GoosePuppet {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub flap_phase: f32, // Fase del aleteo (0.0 .. 2*PI)
    pub angle: f32,      // Ángulo de cabeceo/vuelo
    pub facing_left: bool,
}

impl GoosePuppet {
    pub fn new(x: f32, y: f32, scale: f32, flap_phase: f32) -> Self {
        Self {
            x,
            y,
            scale,
            flap_phase,
            angle: -0.05,
            facing_left: false,
        }
    }

    /// Dibuja un ganso canadiense en vuelo con alas articuladas
    pub fn draw(&self, canvas: &mut Canvas) {
        let s = self.scale;
        canvas.save();
        canvas.translate(self.x, self.y);
        if self.facing_left {
            canvas.scale(-1.0, 1.0);
        }
        canvas.rotate(self.angle);

        let col_black = Color::hex("#1e1c1a");      // Cuello, cabeza y cola negra
        let col_white = Color::hex("#f8f7f2");      // Mancha blanca en la mejilla
        let col_body = Color::hex("#8a7f70");       // Cuerpo pardo grisáceo
        let col_body_light = Color::hex("#cfc7b8"); // Pecho y vientre claro
        let col_wing = Color::hex("#6e6456");       // Ala primaria
        let col_outline = Color::hex("#1c1815");

        // Ángulo de aleteo: onda senoidal suave [-0.75, +0.75] radianes
        let flap = self.flap_phase.sin();

        // 1. Ala lejana (far wing) en segundo plano
        let far_wing_ang = -flap * 0.65 - 0.2;
        draw_wing(canvas, -8.0 * s, -6.0 * s, s, far_wing_ang, col_wing.with_alpha(0.88), col_outline, true);

        // 2. Cola y vientre
        let mut pb_body = PathBuilder::new();
        pb_body.move_to(-38.0 * s, -2.0 * s); // Punta cola
        pb_body.cubic_to(-25.0 * s, -14.0 * s, 10.0 * s, -14.0 * s, 25.0 * s, -4.0 * s);
        pb_body.cubic_to(18.0 * s, 14.0 * s, -15.0 * s, 16.0 * s, -38.0 * s, -2.0 * s);
        pb_body.close();

        if let Some(path_body) = pb_body.finish() {
            canvas.fill_path(&path_body, col_body);
            canvas.stroke_path(&path_body, col_outline, 1.8 * s);
        }

        // Vientre claro recortado
        let mut pb_belly = PathBuilder::new();
        pb_belly.move_to(-20.0 * s, 4.0 * s);
        pb_belly.cubic_to(-5.0 * s, 14.0 * s, 12.0 * s, 12.0 * s, 22.0 * s, 2.0 * s);
        pb_belly.cubic_to(10.0 * s, 4.0 * s, -10.0 * s, 4.0 * s, -20.0 * s, 4.0 * s);
        pb_belly.close();
        if let Some(path_belly) = pb_belly.finish() {
            canvas.fill_path(&path_belly, col_body_light);
        }

        // 3. Cuello estirado hacia adelante y cabeza negra aerodinámica
        let mut pb_neck = PathBuilder::new();
        pb_neck.move_to(18.0 * s, -4.0 * s);
        pb_neck.cubic_to(32.0 * s, -6.0 * s, 48.0 * s, -7.0 * s, 65.0 * s, -7.0 * s); // Cabeza
        pb_neck.line_to(76.0 * s, -6.0 * s); // Pico
        pb_neck.line_to(65.0 * s, -3.0 * s);
        pb_neck.cubic_to(48.0 * s, -1.0 * s, 32.0 * s, 3.0 * s, 18.0 * s, 5.0 * s);
        pb_neck.close();

        if let Some(path_neck) = pb_neck.finish() {
            canvas.fill_path(&path_neck, col_black);
            canvas.stroke_path(&path_neck, col_outline, 1.8 * s);
        }

        // 4. Parche blanco icónico en la mejilla (Canada goose white cheek patch)
        let mut pb_patch = PathBuilder::new();
        pb_patch.move_to(56.0 * s, -8.0 * s);
        pb_patch.line_to(64.0 * s, -4.0 * s);
        pb_patch.line_to(58.0 * s, -2.0 * s);
        pb_patch.close();
        if let Some(path_patch) = pb_patch.finish() {
            canvas.fill_path(&path_patch, col_white);
        }

        // 5. Ojo pequeño
        canvas.fill_circle(62.0 * s, -6.5 * s, 1.2 * s, Color::hex("#ffffff"));

        // 6. Ala cercana (near wing) en primer plano
        let near_wing_ang = -flap * 0.85;
        draw_wing(canvas, 0.0, -4.0 * s, s, near_wing_ang, col_wing, col_outline, false);

        canvas.restore();
    }
}

fn draw_wing(
    canvas: &mut Canvas,
    origin_x: f32,
    origin_y: f32,
    scale: f32,
    flap_angle: f32,
    wing_col: Color,
    line_col: Color,
    is_far: bool,
) {
    let s = scale;
    canvas.save();
    canvas.translate(origin_x, origin_y);
    canvas.rotate(flap_angle);

    let len = if is_far { 52.0 * s } else { 58.0 * s };
    let span = 20.0 * s;

    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 0.0);
    // Arco superior del borde de ataque del ala
    pb.cubic_to(-len * 0.2, -span * 1.3, -len * 0.7, -span * 1.2, -len, -span * 0.2);
    // Plumas primarias aserradas en el borde de fuga
    pb.line_to(-len * 0.85, 0.0);
    pb.line_to(-len * 0.70, span * 0.25);
    pb.line_to(-len * 0.45, span * 0.35);
    pb.cubic_to(-len * 0.2, span * 0.3, -len * 0.1, span * 0.15, 0.0, 0.0);
    pb.close();

    if let Some(path) = pb.finish() {
        canvas.fill_path(&path, wing_col);
        canvas.stroke_path(&path, line_col, 1.6 * s);

        // Líneas de plumas de vuelo
        for i in 1..4 {
            let t = i as f32 / 4.0;
            let fx = -len * t;
            let fy1 = -span * 0.6 * t;
            let fy2 = span * 0.2 * t;
            canvas.stroke_line(fx, fy1, fx, fy2, line_col.with_alpha(0.4), 1.0 * s);
        }
    }

    canvas.restore();
}

// ─────────────────────────────────────────────────────────────────────────────
// Bandada en Formación en "V" (Flock)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct GooseFlock {
    pub geese: Vec<GoosePuppet>,
}

impl GooseFlock {
    /// Crea una bandada de N gansos volando en formación en "V"
    pub fn new_v_formation(apex_x: f32, apex_y: f32, base_scale: f32, count: usize, facing_left: bool) -> Self {
        let mut geese = Vec::with_capacity(count);

        // Ganso líder en el vértice
        let mut lead = GoosePuppet::new(apex_x, apex_y, base_scale, 0.0);
        lead.facing_left = facing_left;
        geese.push(lead);

        // Brazos superior e inferior de la "V"
        let dir = if facing_left { 1.0 } else { -1.0 };
        let spacing_x = 58.0 * base_scale * dir;
        let spacing_y = 38.0 * base_scale;

        for i in 1..count {
            let arm_idx = (i + 1) / 2;
            let is_top = i % 2 == 1;
            let sign = if is_top { -1.0 } else { 1.0 };

            let gx = apex_x + arm_idx as f32 * spacing_x;
            let gy = apex_y + sign * arm_idx as f32 * spacing_y + (arm_idx as f32 * 0.15).sin() * 8.0;
            let scale = base_scale * (1.0 - arm_idx as f32 * 0.04).max(0.70);
            let phase = arm_idx as f32 * 0.52; // Desfase rítmico armónico entre gansos

            let mut g = GoosePuppet::new(gx, gy, scale, phase);
            g.facing_left = facing_left;
            geese.push(g);
        }

        Self { geese }
    }

    /// Actualiza la posición y animación de aleteo de toda la bandada
    pub fn update(&mut self, dt: f32, speed_x: f32, flap_speed: f32) {
        for g in &mut self.geese {
            g.x += speed_x * dt;
            g.flap_phase = (g.flap_phase + flap_speed * dt) % (PI * 2.0);
        }
    }

    /// Dibuja toda la bandada
    pub fn draw(&self, canvas: &mut Canvas) {
        for g in &self.geese {
            g.draw(canvas);
        }
    }
}
