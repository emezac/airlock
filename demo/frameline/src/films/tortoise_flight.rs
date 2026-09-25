//! Película Procedural "Tortoise Flight" (A Tortoise Dreams of Flying South)
//! Recrea fielmente el film animado de Kevin Ngo (@kevin_t_ngo):
//! https://x.com/kevin_t_ngo/status/2101057692395884553
//!
//! Historia:
//! "A tortoise dreams of flying south with the geese.
//!  Claude teaches it how. Every frame drawn with procedural cut-paper collage."
//!
//! Duración: 64.0 segundos a 24 fps (1536 fotogramas).
//! Dividida en 5 escenas poéticas y artesanales:
//! 1. The Longing (0-12s, 288 frames)
//! 2. The Attempt & The Meeting (12-22s, 240 frames)
//! 3. The Blueprint & Inspiration (22-34s, 288 frames)
//! 4. Weaving the Patchwork Balloon (34-48s, 336 frames)
//! 5. Flight South with the Geese (48-64s, 384 frames)

use crate::audio::{AudioTrack, Waveform};
use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette};
use crate::core::finishes::draw_paper;
use crate::core::primitives::draw_scribble;
use crate::core::rng::Rng;
use crate::puppet::balloon::{AutumnTree, PatchworkBalloon};
use crate::puppet::claude_spark::ClaudeSpark;
use crate::puppet::goose::GooseFlock;
use crate::puppet::tortoise::TortoisePuppet;
use crate::timeline::{FilmTimeline, Scene};
use std::f32::consts::PI;
use tiny_skia::{PathBuilder, Stroke};

// ─────────────────────────────────────────────────────────────────────────────
// Utilidades de Entorno y Paisaje de Papel Recortado (Cut-Paper Landscape)
// ─────────────────────────────────────────────────────────────────────────────

/// Dibuja el fondo de papel cálido con textura y grano artesanal
fn draw_paper_sky(canvas: &mut Canvas, base_hex: &str, rng: &mut Rng) {
    let mut pal = Palette::paper_ink();
    pal.paper = Color::hex(base_hex);
    draw_paper(canvas, &pal, rng.next_u32());
}

/// Dibuja el sol otoñal con halo suave y rayos punteados de hilo/lápiz
fn draw_warm_sun(canvas: &mut Canvas, x: f32, y: f32, radius: f32, rng: &mut Rng) {
    // Halo exterior sutil
    canvas.fill_circle(x, y, radius * 1.55, Color::rgba(1.0, 0.92, 0.68, 0.20));
    canvas.fill_circle(x, y, radius * 1.25, Color::rgba(0.98, 0.85, 0.50, 0.35));

    // Disco central de papel amarillo sol
    canvas.fill_circle(x, y, radius, Color::hex("#F6C758"));

    // Borde cosido / punteado alrededor del sol
    let num_dashes = 24;
    for i in 0..num_dashes {
        let angle = (i as f32 / num_dashes as f32) * PI * 2.0;
        let r1 = radius + 4.0;
        let r2 = radius + 12.0;
        let p1x = x + angle.cos() * r1;
        let p1y = y + angle.sin() * r1;
        let p2x = x + angle.cos() * r2;
        let p2y = y + angle.sin() * r2;

        let mut pb = PathBuilder::new();
        pb.move_to(p1x, p1y);
        pb.line_to(p2x, p2y);
        if let Some(path) = pb.finish() {
            canvas.stroke_path(&path, Color::hex("#E5A93C"), 2.0);
        }
    }

    // Suaves garabatos de cera
    let scribble_colors = [Color::rgba(0.94, 0.70, 0.20, 0.25)];
    draw_scribble(canvas, x, y, radius * 0.7, &scribble_colors, rng.next_u32());
}

/// Dibuja una cordillera de montañas de papel cortado en capas
fn draw_mountain_ranges(canvas: &mut Canvas, w: f32, h: f32, horizon_y: f32, seed_offset: u32) {
    let mut rng = Rng::new(seed_offset);

    // Capa 1: Montañas lejanas en bruma lavanda/azul grisáceo (#BDCCD6)
    let mut pb1 = PathBuilder::new();
    pb1.move_to(0.0, horizon_y - 80.0);
    let steps1 = 12;
    for i in 1..=steps1 {
        let x = (i as f32 / steps1 as f32) * w;
        let peak_h = 90.0 + (i as f32 * 1.7).sin().abs() * 110.0;
        let mid_x = x - (w / steps1 as f32) * 0.5;
        let mid_y = horizon_y - peak_h;
        pb1.quad_to(mid_x, mid_y, x, horizon_y - 70.0 + (i as f32 * 0.8).cos() * 20.0);
    }
    pb1.line_to(w, h);
    pb1.line_to(0.0, h);
    pb1.close();
    if let Some(path) = pb1.finish() {
        canvas.fill_path(&path, Color::hex("#C6D3DC"));
    }

    // Capa 2: Colinas intermedias verde salvia / oliva claro (#98AB8F)
    let mut pb2 = PathBuilder::new();
    pb2.move_to(0.0, horizon_y + 10.0);
    let steps2 = 10;
    for i in 1..=steps2 {
        let x = (i as f32 / steps2 as f32) * w;
        let peak_h = 50.0 + ((i as f32 * 2.3).sin() * 0.5 + 0.5) * 80.0;
        let mid_x = x - (w / steps2 as f32) * 0.5;
        let mid_y = horizon_y - peak_h + 30.0;
        pb2.quad_to(mid_x, mid_y, x, horizon_y + 15.0 + (i as f32 * 1.2).sin() * 15.0);
    }
    pb2.line_to(w, h);
    pb2.line_to(0.0, h);
    pb2.close();
    if let Some(path) = pb2.finish() {
        canvas.fill_path(&path, Color::hex("#9CAD91"));

        // Pespuntes blancos a lo largo del perfil de la colina intermedia
        let stroke = Stroke {
            width: 2.0,
            dash: tiny_skia::StrokeDash::new(vec![6.0, 5.0], 0.0),
            ..Default::default()
        };
        let paint = tiny_skia::Paint {
            shader: tiny_skia::Shader::SolidColor(Color::rgba(1.0, 1.0, 1.0, 0.5).to_tiny_skia()),
            anti_alias: true,
            ..Default::default()
        };
        canvas.pixmap.stroke_path(&path, &paint, &stroke, tiny_skia::Transform::identity(), None);
    }

    // Capa 3: Pradera ondulada en primer plano (ocre mostaza cálido #C89F52 y verde musgo #869966)
    let mut pb3 = PathBuilder::new();
    pb3.move_to(0.0, horizon_y + 90.0);
    pb3.quad_to(w * 0.35, horizon_y + 40.0, w * 0.7, horizon_y + 80.0);
    pb3.quad_to(w * 0.88, horizon_y + 100.0, w, horizon_y + 60.0);
    pb3.line_to(w, h);
    pb3.line_to(0.0, h);
    pb3.close();
    if let Some(path) = pb3.finish() {
        canvas.fill_path(&path, Color::hex("#C2A264"));
    }

    // Capa 4: Cresta frontal más cercana (verde suave #7C9160)
    let mut pb4 = PathBuilder::new();
    pb4.move_to(0.0, horizon_y + 140.0);
    pb4.quad_to(w * 0.45, horizon_y + 110.0, w * 0.85, horizon_y + 160.0);
    pb4.line_to(w, horizon_y + 150.0);
    pb4.line_to(w, h);
    pb4.line_to(0.0, h);
    pb4.close();
    if let Some(path) = pb4.finish() {
        canvas.fill_path(&path, Color::hex("#7D9362"));

        // Costuras en la cresta
        let stroke = Stroke {
            width: 2.5,
            dash: tiny_skia::StrokeDash::new(vec![7.0, 6.0], 0.0),
            ..Default::default()
        };
        let paint = tiny_skia::Paint {
            shader: tiny_skia::Shader::SolidColor(Color::hex("#5A6D44").to_tiny_skia()),
            anti_alias: true,
            ..Default::default()
        };
        canvas.pixmap.stroke_path(&path, &paint, &stroke, tiny_skia::Transform::identity(), None);
    }

    // Unas pocas briznas de hierba estilizadas de papel
    for i in 0..18 {
        let gx = (i as f32 * 63.0 + 35.0) % w;
        let gy = horizon_y + 145.0 + (i as f32 * 11.0) % 80.0;
        let mut b = PathBuilder::new();
        b.move_to(gx, gy);
        b.quad_to(gx + 4.0, gy - 16.0, gx + 8.0, gy - 24.0);
        if let Some(path) = b.finish() {
            canvas.stroke_path(&path, Color::hex("#5F7246"), 2.0);
        }
    }

    let _ = rng.next_f32();
}

/// Nubes esponjosas de papel recortado con bordes redondeados
fn draw_paper_cloud(canvas: &mut Canvas, cx: f32, cy: f32, scale: f32) {
    let base_color = Color::rgba(1.0, 0.98, 0.96, 0.85);
    let shade_color = Color::rgba(0.88, 0.86, 0.81, 0.55);

    // Sombra suave inferior
    canvas.fill_circle(cx - 35.0 * scale, cy + 4.0 * scale, 28.0 * scale, shade_color);
    canvas.fill_circle(cx, cy + 6.0 * scale, 36.0 * scale, shade_color);
    canvas.fill_circle(cx + 38.0 * scale, cy + 4.0 * scale, 26.0 * scale, shade_color);

    // Círculos blancos que componen la nube
    canvas.fill_circle(cx - 35.0 * scale, cy, 28.0 * scale, base_color);
    canvas.fill_circle(cx, cy, 36.0 * scale, base_color);
    canvas.fill_circle(cx + 38.0 * scale, cy, 26.0 * scale, base_color);
    canvas.fill_circle(cx - 15.0 * scale, cy - 18.0 * scale, 24.0 * scale, base_color);
    canvas.fill_circle(cx + 18.0 * scale, cy - 16.0 * scale, 22.0 * scale, base_color);
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 1: The Longing (0.0s - 12.0s | 288 frames)
// ─────────────────────────────────────────────────────────────────────────────

pub struct TheLongingScene {
    duration: usize,
    palette: Palette,
}

impl TheLongingScene {
    pub fn new(duration: usize) -> Self {
        Self {
            duration,
            palette: Palette::paper_ink(),
        }
    }
}

impl Scene for TheLongingScene {
    fn name(&self) -> &str {
        "Scene 1: The Longing"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let mut rng = Rng::new((frame + 101) as u32);
        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // 1. Fondo cielo de papel crema
        draw_paper_sky(canvas, "#F8F4EA", &mut rng);

        // 2. Sol matutino cálido
        draw_warm_sun(canvas, w * 0.78, h * 0.22, 62.0, &mut rng);

        // 3. Nubes flotantes lentas
        draw_paper_cloud(canvas, w * 0.22 + tau * 30.0, h * 0.16, 1.1);
        draw_paper_cloud(canvas, w * 0.88 - tau * 20.0, h * 0.28, 0.85);

        // 4. Montañas y colinas de papel recortado
        draw_mountain_ranges(canvas, w, h, h * 0.62, 42);

        // 5. Bandada de gansos en formación de "V" surcando el cielo hacia el sur (de derecha a izquierda)
        let geese_x = w * 1.15 - tau * (w * 1.45);
        let geese_y = h * 0.18 + (tau * PI * 2.0).sin() * 25.0;
        let mut flock = GooseFlock::new_v_formation(geese_x, geese_y, 1.05, 7, true);
        for g in &mut flock.geese {
            g.flap_phase = (tau * 24.0 + g.flap_phase) % (PI * 2.0);
        }
        flock.draw(canvas);

        // 6. Tortuga caminando por la colina hacia la cima y contemplando el cielo
        let (tort_x, tort_y, neck_ext, neck_ang, is_walk, walk_ph, blink) = if tau < 0.45 {
            let walk_t = tau / 0.45;
            let tx = w * 0.15 + walk_t * (w * 0.32);
            let ty = h * 0.77 - walk_t * 40.0;
            let n_ext = 0.25 + (tau * 24.0).sin().abs() * 0.1;
            let n_ang = -0.15;
            let w_phase = walk_t * 22.0;
            let b = if (tau * 12.0) % 3.0 < 0.15 { 1.0 } else { 0.0 };
            (tx, ty, n_ext, n_ang, true, w_phase, b)
        } else {
            // Detenida en la cima, cuello extendido al máximo anhelando volar
            let gaze_t = ((tau - 0.45) / 0.55).clamp(0.0, 1.0);
            let tx = w * 0.47;
            let ty = h * 0.73;
            let n_ext = 0.35 + gaze_t * 0.65;
            let n_ang = -0.15 - gaze_t * 0.55;
            let b = if gaze_t > 0.6 && gaze_t < 0.72 { 1.0 } else { 0.0 };
            (tx, ty, n_ext, n_ang, false, 0.0, b)
        };

        let mut tortoise = TortoisePuppet::new(tort_x, tort_y, 1.55);
        tortoise.neck_extension = neck_ext;
        tortoise.neck_angle = neck_ang;
        tortoise.is_walking = is_walk;
        tortoise.walk_phase = walk_ph;
        tortoise.has_feather = true; // Sostiene la pluma de ganso en el pico
        tortoise.eye_blink = blink;
        tortoise.mouth_open = 0.0;
        tortoise.draw(canvas);

        // 7. Briznas de hojas de otoño arrastradas por el viento
        for i in 0..5 {
            let leaf_t = (tau * 1.8 + i as f32 * 0.23) % 1.0;
            let lx = w * (1.0 - leaf_t);
            let ly = h * 0.55 + (leaf_t * 12.0 + i as f32).sin() * 45.0 + leaf_t * 80.0;
            let leaf_color = match i % 3 {
                0 => Color::hex("#E07A5F"),
                1 => Color::hex("#DDA15E"),
                _ => Color::hex("#BC6C25"),
            };
            canvas.fill_circle(lx, ly, 4.0, leaf_color);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 2: The Attempt & The Meeting (12.0s - 22.0s | 240 frames)
// ─────────────────────────────────────────────────────────────────────────────

pub struct TheAttemptScene {
    duration: usize,
    palette: Palette,
}

impl TheAttemptScene {
    pub fn new(duration: usize) -> Self {
        Self {
            duration,
            palette: Palette::paper_ink(),
        }
    }
}

impl Scene for TheAttemptScene {
    fn name(&self) -> &str {
        "Scene 2: The Attempt & Meeting"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let mut rng = Rng::new((frame + 202) as u32);
        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // 1. Cielo y fondo más cercano
        draw_paper_sky(canvas, "#F6F1E5", &mut rng);
        draw_warm_sun(canvas, w * 0.85, h * 0.18, 50.0, &mut rng);
        draw_mountain_ranges(canvas, w, h, h * 0.60, 55);

        // 2. Comportamiento de la tortuga:
        let (hop_y, neck_ext, neck_ang, is_walk, walk_ph, mouth, blink) = if tau < 0.45 {
            let hop_cycle = (tau * 14.0).sin();
            let hy = if hop_cycle > 0.0 { -hop_cycle * 25.0 } else { 0.0 };
            (hy, 0.4, -0.2, true, tau * 20.0, 0.2, 0.0)
        } else if tau < 0.60 {
            let slump_t = (tau - 0.45) / 0.15;
            let n_ext = 0.4 * (1.0 - slump_t * 0.7);
            (0.0, n_ext, 0.1, false, 0.0, 0.0, 0.0)
        } else {
            let meet_t = ((tau - 0.60) / 0.40).clamp(0.0, 1.0);
            let n_ext = 0.15 + meet_t * 0.75;
            let n_ang = 0.1 - meet_t * 0.45;
            let bl = if meet_t > 0.4 && meet_t < 0.55 { 1.0 } else { 0.0 };
            (0.0, n_ext, n_ang, false, 0.0, 0.4 * meet_t, bl)
        };

        let tort_x = w * 0.38;
        let tort_y = h * 0.72 + hop_y;

        // Si estamos en la primera mitad, dibujamos el globito de juguete atado a su cola
        if tau < 0.58 {
            let balloon_pop_t = (1.0 - (tau / 0.58)).clamp(0.0, 1.0);
            let b_scale = 0.65 * balloon_pop_t;
            let b_x = tort_x - 90.0 + (tau * 10.0).sin() * 8.0;
            let b_y = tort_y - 145.0 + hop_y * 0.5;

            // Hilo de bramante
            let mut pb = PathBuilder::new();
            pb.move_to(tort_x - 55.0, tort_y + 10.0);
            pb.quad_to(b_x + 10.0, (tort_y + b_y) * 0.5, b_x, b_y + 40.0 * b_scale);
            if let Some(path) = pb.finish() {
                canvas.stroke_path(&path, Color::hex("#7A5C3D"), 1.8);
            }

            // Globo de fiesta rojo/terracota
            canvas.fill_circle(b_x, b_y, 42.0 * b_scale, Color::hex("#D05244"));
            canvas.fill_circle(b_x - 12.0 * b_scale, b_y - 12.0 * b_scale, 10.0 * b_scale, Color::rgba(1.0, 1.0, 1.0, 0.55));
            canvas.fill_circle(b_x, b_y + 42.0 * b_scale, 6.0 * b_scale, Color::hex("#A8382C"));
        }

        let mut tortoise = TortoisePuppet::new(tort_x, tort_y, 1.65);
        tortoise.neck_extension = neck_ext;
        tortoise.neck_angle = neck_ang;
        tortoise.is_walking = is_walk;
        tortoise.walk_phase = walk_ph;
        tortoise.has_feather = false;
        tortoise.eye_blink = blink;
        tortoise.mouth_open = mouth;
        tortoise.draw(canvas);

        // 3. Aparición de Claude Spark (la chispa de 8 puntas de Anthropic)
        if tau > 0.48 {
            let spark_t = ((tau - 0.48) / 0.52).clamp(0.0, 1.0);
            let spark_x = w * 0.82 - spark_t * (w * 0.20) + (tau * 6.0).sin() * 15.0;
            let spark_y = h * (-0.1) + spark_t * (h * 0.65) + (tau * 4.0).cos() * 12.0;

            let mut spark = ClaudeSpark::new(spark_x, spark_y, 1.35);
            spark.time = tau * 10.0;

            if spark_t > 0.7 {
                let spin_t = (spark_t - 0.7) / 0.3;
                spark.angle = (spin_t * PI * 2.0).sin() * 0.35;
            }

            if spark_t > 0.82 && spark_t < 0.94 {
                spark.blink = 1.0;
            }

            spark.draw(canvas);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 3: The Blueprint & Inspiration (22.0s - 34.0s | 288 frames)
// ─────────────────────────────────────────────────────────────────────────────

pub struct TheBlueprintScene {
    duration: usize,
    palette: Palette,
}

impl TheBlueprintScene {
    pub fn new(duration: usize) -> Self {
        Self {
            duration,
            palette: Palette::paper_ink(),
        }
    }
}

impl Scene for TheBlueprintScene {
    fn name(&self) -> &str {
        "Scene 3: The Blueprint"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let mut rng = Rng::new((frame + 303) as u32);
        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // 1. Fondo cielo y colinas
        draw_paper_sky(canvas, "#F8F4EA", &mut rng);
        draw_warm_sun(canvas, w * 0.15, h * 0.22, 55.0, &mut rng);
        draw_mountain_ranges(canvas, w, h, h * 0.64, 77);

        // 2. El árbol otoñal lleno de hojas listas para transformarse
        let tree = AutumnTree::new(w * 0.82, h * 0.76, 1.55);
        tree.draw(canvas);

        // 3. La tortuga observando con fascinación
        let mut tortoise = TortoisePuppet::new(w * 0.24, h * 0.74, 1.50);
        tortoise.neck_extension = 0.85;
        tortoise.neck_angle = -0.45;
        tortoise.eye_blink = if (tau * 10.0) % 4.0 < 0.2 { 1.0 } else { 0.0 };
        tortoise.mouth_open = 0.5 * (tau * 1.5).min(1.0);
        tortoise.draw(canvas);

        // 4. Claude Spark actuando como arquitecto/diseñador poético
        let spark_base_x = w * 0.50;
        let spark_base_y = h * 0.44;
        let mut spark = ClaudeSpark::new(spark_base_x, spark_base_y, 1.25);
        spark.time = tau * 12.0;

        // Dibuja el plano esquemático del globo en el aire
        let bp_progress = (tau * 1.25).clamp(0.0, 1.0);
        spark.draw_blueprint(canvas, w * 0.50, h * 0.32, bp_progress);

        spark.draw(canvas);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 4: Weaving the Patchwork Balloon (34.0s - 48.0s | 336 frames)
// ─────────────────────────────────────────────────────────────────────────────

pub struct WeavingBalloonScene {
    duration: usize,
    palette: Palette,
}

impl WeavingBalloonScene {
    pub fn new(duration: usize) -> Self {
        Self {
            duration,
            palette: Palette::paper_ink(),
        }
    }
}

impl Scene for WeavingBalloonScene {
    fn name(&self) -> &str {
        "Scene 4: Weaving Balloon"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let mut rng = Rng::new((frame + 404) as u32);
        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // 1. Cielo y montañas
        draw_paper_sky(canvas, "#F7F2E6", &mut rng);
        draw_warm_sun(canvas, w * 0.82, h * 0.20, 52.0, &mut rng);
        draw_mountain_ranges(canvas, w, h, h * 0.62, 88);

        // 2. Árbol otoñal cuyas hojas se desprenden
        let tree_x = w * 0.18;
        let tree_y = h * 0.78;
        let mut tree = AutumnTree::new(tree_x, tree_y, 1.45);
        tree.leaves_remaining = (1.0 - tau * 0.95).max(0.05);
        tree.draw(canvas);

        // 3. Hojas volando en remolino desde el árbol hacia el globo
        let balloon_target_x = w * 0.68;
        let balloon_target_y = h * 0.38;
        for i in 0..16 {
            let leaf_tau = (tau * 2.5 + i as f32 * 0.065) % 1.0;
            let lx = tree_x + leaf_tau * (balloon_target_x - tree_x);
            let ly = (tree_y - 250.0) + leaf_tau * (balloon_target_y - (tree_y - 250.0)) - (leaf_tau * PI).sin() * 95.0;
            let col = match i % 4 {
                0 => Color::hex("#dda15e"),
                1 => Color::hex("#e07a5f"),
                2 => Color::hex("#bc6c25"),
                _ => Color::hex("#588157"),
            };
            canvas.fill_circle(lx, ly, 5.0, col);
        }

        // 4. Globo de retazos ensamblándose progresivamente
        let assembly = (tau * 1.25).clamp(0.0, 1.0);
        let bob_y = (tau * 8.0).sin() * 8.0 * assembly;
        let mut balloon = PatchworkBalloon::new(balloon_target_x, balloon_target_y + bob_y, 1.38);
        balloon.leaf_assembly_progress = assembly;
        balloon.sway = (tau * 5.0).sin() * 0.04;
        balloon.draw(canvas);

        // 5. Claude Spark supervisando el tejido mágico
        let spark_x = balloon_target_x + 110.0 + (tau * 5.0).cos() * 25.0;
        let spark_y = balloon_target_y - 50.0 + (tau * 7.0).sin() * 20.0;
        let mut spark = ClaudeSpark::new(spark_x, spark_y, 1.15);
        spark.time = tau * 12.0;
        spark.draw(canvas);

        // 6. La tortuga:
        if tau < 0.75 {
            let walk_t = tau / 0.75;
            let tx = w * 0.36 + walk_t * (w * 0.28);
            let ty = h * 0.74;
            let mut tortoise = TortoisePuppet::new(tx, ty, 1.40);
            tortoise.neck_extension = 0.45;
            tortoise.is_walking = true;
            tortoise.walk_phase = walk_t * 22.0;
            tortoise.mouth_open = 0.2;
            tortoise.draw(canvas);
        } else {
            // Ya está dentro de la canasta del globo aerostático
            let mut tortoise = TortoisePuppet::new(balloon_target_x + 8.0, balloon_target_y + bob_y + 195.0, 1.15);
            tortoise.in_basket = true;
            tortoise.neck_extension = 0.7;
            tortoise.neck_angle = -0.25;
            tortoise.mouth_open = 0.1;
            tortoise.eye_blink = if (tau * 10.0) % 3.0 < 0.15 { 1.0 } else { 0.0 };
            tortoise.draw(canvas);

            // Capa frontal de la canasta cubriendo la base del caparazón
            balloon.draw_basket_front(canvas);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 5: Flight South with the Geese (48.0s - 64.0s | 384 frames)
// ─────────────────────────────────────────────────────────────────────────────

pub struct FlightSouthScene {
    duration: usize,
    palette: Palette,
}

impl FlightSouthScene {
    pub fn new(duration: usize) -> Self {
        Self {
            duration,
            palette: Palette::paper_ink(),
        }
    }
}

impl Scene for FlightSouthScene {
    fn name(&self) -> &str {
        "Scene 5: Flight South"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let mut rng = Rng::new((frame + 505) as u32);
        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // 1. Cielo dorado de atardecer / Golden hour
        let sky_color = if tau < 0.7 { "#FAF0DE" } else { "#F7E6CE" };
        draw_paper_sky(canvas, sky_color, &mut rng);

        // 2. Gran sol dorado cálido hacia el horizonte sur
        let sun_x = w * 0.75;
        let sun_y = h * 0.32;
        draw_warm_sun(canvas, sun_x, sun_y, 85.0, &mut rng);

        // 3. Capas de nubes de ensueño a diferentes profundidades
        draw_paper_cloud(canvas, w * 0.15 - tau * 40.0, h * 0.22, 1.2);
        draw_paper_cloud(canvas, w * 0.85 - tau * 30.0, h * 0.45, 0.9);
        draw_paper_cloud(canvas, w * 0.45 - tau * 50.0, h * 0.68, 1.4);

        // 4. Montañas en miniatura muy abajo (efecto de gran altitud de vuelo)
        draw_mountain_ranges(canvas, w, h, h * 0.90, 99);

        // 5. Globo aerostático ascendiendo y flotando majestuosamente
        let balloon_x = w * 0.42 + (tau * PI * 1.5).sin() * 45.0;
        let balloon_y = h * 0.58 - tau * (h * 0.28) + (tau * 6.0).sin() * 12.0;

        let mut balloon = PatchworkBalloon::new(balloon_x, balloon_y, 1.45);
        balloon.leaf_assembly_progress = 1.0;
        balloon.sway = 0.05 + (tau * 4.0).sin() * 0.03;
        balloon.draw(canvas);

        // 6. La tortuga asomada a la barandilla de la canasta, cumpliendo su sueño
        let mut tortoise = TortoisePuppet::new(balloon_x + 10.0, balloon_y + 205.0, 1.20);
        tortoise.in_basket = true;
        tortoise.neck_extension = 0.88;
        tortoise.neck_angle = -0.32;
        tortoise.mouth_open = 0.0;
        tortoise.eye_blink = if (tau * 8.0) % 4.0 < 0.15 { 1.0 } else { 0.0 };
        tortoise.draw(canvas);

        // 7. Claude Spark viajando junto a la tortuga posado cerca de la canasta
        let spark_x = balloon_x - 82.0;
        let spark_y = balloon_y + 165.0 + (tau * 6.0).cos() * 5.0;
        let mut spark = ClaudeSpark::new(spark_x, spark_y, 0.95);
        spark.time = tau * 10.0;
        spark.draw(canvas);

        // Pared frontal y ribete de la canasta cubriendo el plastrón inferior
        balloon.draw_basket_front(canvas);

        // 8. La bandada de gansos volando en formación al lado del globo
        let geese_x = balloon_x + 310.0 - tau * 60.0;
        let geese_y = balloon_y - 65.0 + (tau * 5.0).sin() * 18.0;
        let mut flock = GooseFlock::new_v_formation(geese_x, geese_y, 1.10, 6, false);
        for g in &mut flock.geese {
            g.flap_phase = (tau * 26.0 + g.flap_phase) % (PI * 2.0);
        }
        flock.draw(canvas);

        // 9. Destellos de luz y polvillo dorado en el aire
        for i in 0..12 {
            let p_t = (tau * 2.0 + i as f32 * 0.17) % 1.0;
            let px = w * p_t;
            let py = h * 0.2 + (i as f32 * 37.0) % (h * 0.6);
            let alpha = (p_t * PI).sin() * 0.6;
            canvas.fill_circle(px, py, 2.5, Color::rgba(1.0, 0.90, 0.63, alpha));
        }

        // 10. Suave viñeta de calidez al final (tau > 0.85)
        if tau > 0.85 {
            let fade_t = (tau - 0.85) / 0.15;
            canvas.fill_rect(0.0, 0.0, w, h, Color::rgba(0.97, 0.90, 0.81, fade_t * 0.25));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Generador del Film Completo y Banda Sonora Procedural Sincronizada
// ─────────────────────────────────────────────────────────────────────────────

/// Construye la película "Tortoise Flight" con sus 5 escenas y banda sonora procedural
pub fn create_tortoise_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();
    timeline.fps = 24;
    timeline.on_twos = false; // 24 fps fluidos para suavidad de alas, chispas y vuelo
    timeline.motion_blur = 0.0; // 0.0 para conservar la nitidez de papel recortado

    // Escena 1: The Longing (12.0s = 288 frames)
    timeline.add_scene(TheLongingScene::new(288));

    // Escena 2: The Attempt & The Meeting (10.0s = 240 frames)
    timeline.add_scene(TheAttemptScene::new(240));

    // Escena 3: The Blueprint & Inspiration (12.0s = 288 frames)
    timeline.add_scene(TheBlueprintScene::new(288));

    // Escena 4: Weaving the Patchwork Balloon (14.0s = 336 frames)
    timeline.add_scene(WeavingBalloonScene::new(336));

    // Escena 5: Flight South with the Geese (16.0s = 384 frames)
    timeline.add_scene(FlightSouthScene::new(384));

    // Total: 288 + 240 + 288 + 336 + 384 = 1536 frames = 64.0s a 24 fps
    let total_secs = timeline.total_duration_secs();

    // Síntesis de la banda sonora procedural complementaria:
    // Melodía acústica de arpegios cálidos en escala pentatónica y rumor ambiental
    let mut audio = AudioTrack::new(total_secs, 44100);

    // Escala pentatónica mayor en Sol (G major pentatonic): G3, A3, B3, D4, E4, G4, A4, B4, D5
    let scale = [196.0, 220.0, 246.94, 293.66, 329.63, 392.0, 440.0, 493.88, 587.33];

    let mut step = 0;
    let mut t = 0.0;
    while t < total_secs {
        let note_idx = match (step / 4) % 4 {
            0 => [0, 2, 3, 5],
            1 => [1, 3, 4, 6],
            2 => [2, 4, 5, 7],
            _ => [0, 3, 5, 8],
        }[step % 4];

        let freq = scale[note_idx];
        let duration = if step % 8 == 7 { 0.8 } else { 0.4 };
        let vol = 0.08 + (t * 0.1).sin().abs() * 0.04;

        audio.add_note(t, duration, freq, Waveform::Sine, vol, 0.0);
        audio.add_note(t + 0.01, duration * 0.5, freq * 2.0, Waveform::Triangle, vol * 0.25, 0.2);

        t += 0.35;
        step += 1;
    }

    // Pad ambiental cálido de fondo
    audio.add_ambient_pad(0.0, total_secs, 196.0, 293.66, 0.05, 1234);

    (timeline, audio)
}
