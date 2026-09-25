//! Puppet Artesanal: La Chispa de Claude (Claude Spark Puppet)
//! Representa la chispa emblemática de Anthropic de 8 puntas en color terracota/coral,
//! con ojos tiernos, animación de flotación y capacidad de trazar planos arquitectónicos.

use crate::core::canvas::Canvas;
use crate::core::color::Color;
use std::f32::consts::PI;
use tiny_skia::{PathBuilder, Stroke};

#[derive(Clone, Debug)]
pub struct ClaudeSpark {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub angle: f32,
    pub blink: f32,       // 0.0 (abierto) a 1.0 (cerrado)
    pub is_happy: bool,   // Si los ojos forman arcos sonrientes
    pub alpha: f32,
    pub time: f32,        // Tiempo para la flotación suave
}

impl Default for ClaudeSpark {
    fn default() -> Self {
        Self {
            x: 500.0,
            y: 400.0,
            scale: 1.0,
            angle: 0.0,
            blink: 0.0,
            is_happy: true,
            alpha: 1.0,
            time: 0.0,
        }
    }
}

impl ClaudeSpark {
    pub fn new(x: f32, y: f32, scale: f32) -> Self {
        Self {
            x,
            y,
            scale,
            ..Default::default()
        }
    }

    /// Actualiza la física de flotación orgánica
    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    /// Dibuja la chispa de Claude con su rostro amigable
    pub fn draw(&self, canvas: &mut Canvas) {
        if self.alpha <= 0.01 {
            return;
        }

        let s = self.scale;
        // Flotación suave con respiración y leve cabeceo
        let hover_y = self.y + (self.time * 4.5).sin() * 5.5 * s;
        let tilt = self.angle + (self.time * 3.0).sin() * 0.08;

        canvas.save();
        canvas.translate(self.x, hover_y);
        canvas.rotate(tilt);

        let col_spark = Color::hex("#d96951").with_alpha(self.alpha); // Terracota / Coral Anthropic
        let col_outline = Color::hex("#2a1f18").with_alpha(self.alpha);
        let col_eye = Color::hex("#1e1612").with_alpha(self.alpha);
        let col_white = Color::hex("#ffffff").with_alpha(self.alpha);

        let lobes = 8;
        let r_inner = 14.0 * s;
        let r_outer = 32.0 * s;
        let lobe_width = 7.0 * s;

        // 1. Trazar la silueta icónica de la chispa de Anthropic (8 rayos redondeados)
        let mut pb = PathBuilder::new();
        for i in 0..lobes {
            let a = (i as f32 / lobes as f32) * PI * 2.0;
            let a_next = ((i as f32 + 0.5) / lobes as f32) * PI * 2.0;

            let px_tip = a.cos() * r_outer;
            let py_tip = a.sin() * r_outer;

            let perp_x = -a.sin() * lobe_width;
            let perp_y = a.cos() * lobe_width;

            let px_valley = a_next.cos() * r_inner;
            let py_valley = a_next.sin() * r_inner;

            if i == 0 {
                pb.move_to(px_tip + perp_x, py_tip + perp_y);
            }

            // Lóbulo con cabeza redondeada
            pb.cubic_to(
                px_tip + perp_x * 0.8, py_tip + perp_y * 0.8,
                px_tip - perp_x * 0.8, py_tip - perp_y * 0.8,
                px_tip - perp_x, py_tip - perp_y,
            );
            // Valle interior entre lóbulos
            pb.line_to(px_valley, py_valley);
        }
        pb.close();

        if let Some(path) = pb.finish() {
            canvas.fill_path(&path, col_spark);
            canvas.stroke_path(&path, col_outline, 2.0 * s);
        }

        // 2. Rostro amigable y expresivo en el centro
        let eye_dist = 6.0 * s;
        let eye_y = -2.0 * s;

        if self.blink > 0.85 {
            // Parpadeo feliz (dos arcos sonrientes)
            canvas.stroke_line(-eye_dist - 3.0 * s, eye_y, -eye_dist + 3.0 * s, eye_y, col_eye, 1.8 * s);
            canvas.stroke_line(eye_dist - 3.0 * s, eye_y, eye_dist + 3.0 * s, eye_y, col_eye, 1.8 * s);
        } else if self.is_happy {
            // Ojos redondos grandes y curiosos
            let eye_r = 3.2 * s;
            canvas.fill_circle(-eye_dist, eye_y, eye_r, col_eye);
            canvas.fill_circle(eye_dist, eye_y, eye_r, col_eye);

            // Brillo blanco en los ojos
            canvas.fill_circle(-eye_dist + 1.0 * s, eye_y - 1.0 * s, 1.0 * s, col_white);
            canvas.fill_circle(eye_dist + 1.0 * s, eye_y - 1.0 * s, 1.0 * s, col_white);
        }

        // Sonrisa curvada pequeña
        let mut pb_mouth = PathBuilder::new();
        pb_mouth.move_to(-3.5 * s, 4.0 * s);
        pb_mouth.cubic_to(-1.5 * s, 6.5 * s, 1.5 * s, 6.5 * s, 3.5 * s, 4.0 * s);
        if let Some(path_mouth) = pb_mouth.finish() {
            canvas.stroke_path(&path_mouth, col_eye, 1.4 * s);
        }

        canvas.restore();
    }

    /// Dibuja los planos arquitectónicos de vuelo trazados con tiza roja punteada en el cielo
    pub fn draw_blueprint(
        &self,
        canvas: &mut Canvas,
        balloon_cx: f32,
        balloon_cy: f32,
        progress: f32, // 0.0 a 1.0 de construcción del plano
    ) {
        if progress <= 0.01 {
            return;
        }

        let s = self.scale;
        let col_bp = Color::hex("#d96951").with_alpha(0.85 * progress.min(1.0));
        let col_arrow = Color::hex("#c2472d").with_alpha(0.90 * progress.min(1.0));

        let b_rx = 110.0 * s;
        let b_ry = 135.0 * s;

        // 1. Envoltura punteada del globo aerostático
        let steps = (40.0 * progress) as usize;
        let mut pb_envelope = PathBuilder::new();
        for i in 0..=steps {
            let t = i as f32 / 40.0;
            let ang = -PI * 0.5 + t * PI * 2.0;

            // Forma de pera aerodinámica (más ancha arriba, cónica abajo)
            let pear_factor = 1.0 - 0.35 * (ang.sin() + 1.0) * 0.5;
            let px = balloon_cx + ang.cos() * b_rx * pear_factor;
            let py = balloon_cy + ang.sin() * b_ry;

            if i == 0 {
                pb_envelope.move_to(px, py);
            } else {
                pb_envelope.line_to(px, py);
            }
        }
        if let Some(path) = pb_envelope.finish() {
            let stroke = Stroke {
                width: 2.2 * s,
                miter_limit: 4.0,
                line_cap: tiny_skia::LineCap::Round,
                line_join: tiny_skia::LineJoin::Round,
                dash: tiny_skia::StrokeDash::new(vec![6.0 * s, 6.0 * s], 0.0),
            };
            canvas.pixmap.stroke_path(&path, &tiny_skia::Paint {
                shader: tiny_skia::Shader::SolidColor(col_bp.to_tiny_skia()),
                blend_mode: tiny_skia::BlendMode::SourceOver,
                anti_alias: true,
                force_hq_pipeline: false,
            }, &stroke, tiny_skia::Transform::identity(), None);
        }

        // 2. Canasta punteada inferior y cuerdas
        if progress > 0.4 {
            let p_basket = ((progress - 0.4) / 0.6).min(1.0);
            let basket_y = balloon_cy + b_ry + 55.0 * s;
            let bw = 65.0 * s * p_basket;
            let bh = 32.0 * s * p_basket;

            canvas.stroke_line(balloon_cx - 45.0 * s, balloon_cy + b_ry, balloon_cx - bw * 0.8, basket_y, col_bp, 1.8 * s);
            canvas.stroke_line(balloon_cx + 45.0 * s, balloon_cy + b_ry, balloon_cx + bw * 0.8, basket_y, col_bp, 1.8 * s);
            canvas.stroke_line(balloon_cx - bw, basket_y, balloon_cx + bw, basket_y, col_bp, 2.0 * s);
            canvas.stroke_line(balloon_cx - bw * 0.85, basket_y + bh, balloon_cx + bw * 0.85, basket_y + bh, col_bp, 2.0 * s);
        }

        // 3. Flecha curva de despegue y trayectoria aerodinámica
        if progress > 0.6 {
            let p_arrow = ((progress - 0.6) / 0.4).min(1.0);
            let arc_steps = (24.0 * p_arrow) as usize;
            let mut pb_arrow = PathBuilder::new();
            for i in 0..=arc_steps {
                let t = i as f32 / 24.0;
                let ax = balloon_cx + 40.0 * s + t * 140.0 * s;
                let ay = balloon_cy + 80.0 * s - (t * PI).sin() * 95.0 * s;
                if i == 0 {
                    pb_arrow.move_to(ax, ay);
                } else {
                    pb_arrow.line_to(ax, ay);
                }
            }
            if let Some(path) = pb_arrow.finish() {
                canvas.stroke_path(&path, col_arrow, 2.8 * s);

                // Cabeza de la flecha
                if p_arrow > 0.95 {
                    let end_x = balloon_cx + 180.0 * s;
                    let end_y = balloon_cy + 80.0 * s;
                    canvas.stroke_line(end_x, end_y, end_x - 14.0 * s, end_y - 12.0 * s, col_arrow, 2.8 * s);
                    canvas.stroke_line(end_x, end_y, end_x - 4.0 * s, end_y - 18.0 * s, col_arrow, 2.8 * s);
                }
            }
        }
    }
}
