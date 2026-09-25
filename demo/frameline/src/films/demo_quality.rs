//! Película de Demostración de Alta Calidad Cinematográfica
//! Estándar de Referencia: Kevin Ngo / Claude Opus 5 ("The Life of a Fruit Fly")
//!
//! Demuestra la integración completa de:
//! 1. Gradientes radiales y lineales ricos por fotograma.
//! 2. Iluminación puntual dinámica (`point_light`) con halo y blend modes.
//! 3. Niebla volumétrica de profundidad (`ambient_fog`).
//! 4. Animación orgánica con curvas de easing cúbicas, elásticas y springs.
//! 5. Sistema de partículas vivas con ciclo de vida, física y fade (`ParticlePool`).
//! 6. Desenfoque de movimiento (`motion_blur`) en renderizado de alta velocidad.
//! 7. Tipografía técnica y HUD de microscopía científica integrada.

use crate::audio::{pent_hz, AudioTrack, Waveform};
use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette};
use crate::core::easing::{
    ease_in_out_cubic, ease_in_out_quint, ease_out_cubic, ease_out_elastic, spring,
};
use crate::core::effects::{ambient_fog, point_light, vignette};
use crate::core::finishes::wob_path;
use crate::core::particles::{EmitParams, ParticlePool, ParticleShape, SpreadShape};
use crate::core::primitives::ellipse_points;
use crate::core::schematic::draw_vector_text;
use crate::puppet::FruitFly;
use crate::timeline::{FilmTimeline, Scene};
use std::f32::consts::PI;
use tiny_skia::{BlendMode, PathBuilder};

// ─────────────────────────────────────────────────────────────────────────────
// Utilidades de HUD Científico y Grillas Técnicas
// ─────────────────────────────────────────────────────────────────────────────

fn draw_scientific_hud(
    canvas: &mut Canvas,
    title: &str,
    code: &str,
    scale_label: &str,
    frame: usize,
    opacity: f32,
) {
    if opacity <= 0.01 {
        return;
    }
    let w = canvas.width as f32;
    let h = canvas.height as f32;
    let hud_color = Color::hex("#64ffda").with_alpha(0.35 * opacity);
    let hud_dim = Color::hex("#233554").with_alpha(0.40 * opacity);
    let hud_text = Color::hex("#ccd6f6").with_alpha(0.75 * opacity);

    // 1. Marco perimetral y marcas de encuadre en las 4 esquinas
    let pad = 40.0;
    let tick = 24.0;
    // Top-Left
    canvas.stroke_line(pad, pad, pad + tick, pad, hud_color, 1.2);
    canvas.stroke_line(pad, pad, pad, pad + tick, hud_color, 1.2);
    // Top-Right
    canvas.stroke_line(w - pad, pad, w - pad - tick, pad, hud_color, 1.2);
    canvas.stroke_line(w - pad, pad, w - pad, pad + tick, hud_color, 1.2);
    // Bottom-Left
    canvas.stroke_line(pad, h - pad, pad + tick, h - pad, hud_color, 1.2);
    canvas.stroke_line(pad, h - pad, pad, h - pad - tick, hud_color, 1.2);
    // Bottom-Right
    canvas.stroke_line(w - pad, h - pad, w - pad - tick, h - pad, hud_color, 1.2);
    canvas.stroke_line(w - pad, h - pad, w - pad, h - pad - tick, hud_color, 1.2);

    // 2. Líneas sutiles de retícula
    let step = 120.0;
    let mut x = pad * 2.0;
    while x < w - pad * 2.0 {
        canvas.stroke_line(x, pad + 10.0, x, pad + 16.0, hud_dim, 0.8);
        canvas.stroke_line(x, h - pad - 16.0, x, h - pad - 10.0, hud_dim, 0.8);
        x += step;
    }

    // 3. Cruz central micrométrica
    let cx = w * 0.5;
    let cy = h * 0.5;
    let cross = 18.0;
    canvas.stroke_line(cx - cross, cy, cx - 6.0, cy, hud_color, 1.0);
    canvas.stroke_line(cx + 6.0, cy, cx + cross, cy, hud_color, 1.0);
    canvas.stroke_line(cx, cy - cross, cx, cy - 6.0, hud_color, 1.0);
    canvas.stroke_line(cx, cy + 6.0, cx, cy + cross, hud_color, 1.0);

    // Círculo concéntrico de mira
    let pts = ellipse_points(cx, cy, 38.0, 38.0, 32);
    if let Some(path) = wob_path(&pts, 0.5, 999, true) {
        canvas.stroke_path(&path, hud_dim, 0.75);
    }

    // 4. Tipografía y metadatos técnicos en pantalla
    draw_vector_text(canvas, pad + 12.0, pad + 16.0, title, 14.0, hud_text, false);
    draw_vector_text(canvas, pad + 12.0, pad + 38.0, code, 11.0, hud_color, false);

    let frame_str = format!("FRM {:04}", frame);
    draw_vector_text(canvas, w - pad - 140.0, pad + 16.0, &frame_str, 12.0, hud_color, false);
    draw_vector_text(canvas, w - pad - 140.0, pad + 38.0, scale_label, 10.0, hud_text, false);
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 1: "I. Embryo in Dew / Génesis Microscópica" (96 fotogramas = 4.0s)
// ─────────────────────────────────────────────────────────────────────────────

pub struct EmbryoScene {
    pub duration: usize,
    pub palette: Palette,
}

impl Scene for EmbryoScene {
    fn name(&self) -> &str {
        "I. Embryo in Dew (Génesis Microscópica)"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let w = canvas.width as f32;
        let h = canvas.height as f32;
        let cx = w * 0.5;
        let cy = h * 0.5;

        // 1. Fondo atmosférico de laboratorio con gradiente radial profundo
        let bg_center = Color::hex("#0c192c");
        let bg_mid = Color::hex("#050e18");
        let bg_edge = Color::hex("#020509");
        canvas.fill_radial_gradient(
            cx,
            cy,
            w * 0.85,
            &[
                (0.0, bg_center),
                (0.55, bg_mid),
                (1.0, bg_edge),
            ],
        );

        // 2. Luz puntual móvil que recorre el huevo como iluminación microscópica
        let light_angle = tau * PI * 2.0;
        let light_x = cx + light_angle.cos() * 180.0;
        let light_y = cy + light_angle.sin() * 140.0;
        point_light(
            canvas,
            light_x,
            light_y,
            420.0,
            Color::hex("#64ffda").with_alpha(0.6),
            0.85,
        );

        // Halo secundario cálido (foco halógeno sub-etapa)
        point_light(
            canvas,
            cx - 80.0,
            cy - 60.0,
            300.0,
            Color::hex("#f6d365").with_alpha(0.4),
            0.6,
        );

        // 3. Partículas microscópicas / esporas en suspensión (Pool procedural)
        let mut pool = ParticlePool::with_seed((frame as u32).wrapping_mul(73) + 11);
        pool.emit(&EmitParams {
            cx,
            cy,
            count: 36,
            spread: SpreadShape::Rect { w: w * 0.7, h: h * 0.7 },
            speed_min: 6.0,
            speed_max: 28.0,
            life_min: 2.0,
            life_max: 5.0,
            size_min: 1.5,
            size_max: 4.5,
            size_end: 0.5,
            color_start: Color::hex("#64ffda").with_alpha(0.55),
            color_end: Color::hex("#0c192c").with_alpha(0.0),
            gravity_y: -4.0,
            shape: ParticleShape::Circle,
            blend: BlendMode::Screen,
            ..Default::default()
        });
        pool.update(tau * 2.5);
        pool.draw(canvas);

        // 4. El huevo (Drosophila Embryo) con volumen orgánico y cáscara multicapa
        let pulse = (tau * PI * 6.0).sin() * 4.0;
        let breath = ease_in_out_cubic(tau);
        let egg_rx = 110.0 + pulse * 0.5 + breath * 12.0;
        let egg_ry = 185.0 + pulse + breath * 18.0;

        canvas.save();
        canvas.translate(cx, cy);
        let egg_tilt = (tau * PI * 1.5).sin() * 0.08;
        canvas.rotate(egg_tilt);

        // A. Resplandor exterior de la membrana vitelina
        let outer_glow = Color::hex("#64ffda").with_alpha(0.18);
        canvas.fill_radial_gradient(
            0.0,
            0.0,
            egg_ry * 1.4,
            &[
                (0.0, outer_glow),
                (0.65, outer_glow.with_alpha(0.06)),
                (1.0, Color::rgba(0.0, 0.0, 0.0, 0.0)),
            ],
        );

        // B. Capa base de la cáscara del huevo (óvalo de Bezier con gradiente)
        let egg_pts = ellipse_points(0.0, 0.0, egg_rx, egg_ry, 48);
        let mut pb = PathBuilder::new();
        for (i, p) in egg_pts.iter().enumerate() {
            if i == 0 {
                pb.move_to(p[0], p[1]);
            } else {
                pb.line_to(p[0], p[1]);
            }
        }
        pb.close();

        if let Some(egg_path) = pb.finish() {
            let egg_core = Color::hex("#112240").with_alpha(0.92);
            let egg_amber = Color::hex("#d4af37").with_alpha(0.75);
            let egg_highlight = Color::hex("#e2e8f0").with_alpha(0.85);

            canvas.fill_path_linear_gradient(
                &egg_path,
                -egg_rx * 0.7,
                -egg_ry * 0.8,
                egg_rx * 0.8,
                egg_ry * 0.8,
                &[
                    (0.0, egg_highlight),
                    (0.35, egg_amber),
                    (0.75, egg_core),
                    (1.0, Color::hex("#0a192f")),
                ],
            );

            // Borde celular membranoso
            if let Some(stroke) = wob_path(&egg_pts, 0.8, (frame as u32) + 42, true) {
                canvas.stroke_path(&stroke, Color::hex("#64ffda").with_alpha(0.7), 2.2);
            }
        }

        // C. Mitosis / Núcleos embrionarios en división dentro del huevo
        let cell_count = 5;
        for c in 0..cell_count {
            let offset_t = tau + c as f32 * 0.2;
            let cell_y = -egg_ry * 0.55 + (c as f32 / (cell_count - 1) as f32) * egg_ry * 1.1;
            let cell_x = (offset_t * PI * 4.0).sin() * 22.0;
            let cell_radius = 16.0 + (offset_t * PI * 3.0).cos() * 5.0;

            let nuc_pts = ellipse_points(cell_x, cell_y, cell_radius, cell_radius * 0.85, 24);
            let mut pb_nuc = PathBuilder::new();
            for (i, p) in nuc_pts.iter().enumerate() {
                if i == 0 {
                    pb_nuc.move_to(p[0], p[1]);
                } else {
                    pb_nuc.line_to(p[0], p[1]);
                }
            }
            pb_nuc.close();
            if let Some(np) = pb_nuc.finish() {
                canvas.fill_path(&np, Color::hex("#64ffda").with_alpha(0.45));
                if let Some(ns) = wob_path(&nuc_pts, 0.5, (frame as u32) + c as u32, true) {
                    canvas.stroke_path(&ns, Color::rgba(1.0, 1.0, 1.0, 0.65), 1.2);
                }
            }
        }

        // D. Especular superficial cáustico
        let spec_pts = ellipse_points(-egg_rx * 0.45, -egg_ry * 0.45, egg_rx * 0.3, egg_ry * 0.2, 24);
        if let Some(spec_stroke) = wob_path(&spec_pts, 0.6, 888, true) {
            canvas.stroke_path(&spec_stroke, Color::rgba(1.0, 1.0, 1.0, 0.65), 2.4);
        }

        canvas.restore();

        // 5. Niebla ambiental para volumen y profundidad
        ambient_fog(canvas, h * 0.65, Color::hex("#0a192f"), 0.35);

        // 6. HUD Científico
        let ease_opacity = ease_out_cubic(tau.min(0.2) / 0.2);
        draw_scientific_hud(
            canvas,
            "01 // DROSOPHILA EMBRYOGENESIS",
            "MOD: CONFOCAL FLUORESCENCE 488nm",
            "DIAMETER: 0.52mm",
            frame,
            ease_opacity,
        );

        // 7. Viñeteado cinematográfico
        vignette(canvas, 0.65, 1.4);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 2: "II. Metamorphosis & Organogenesis" (96 fotogramas = 4.0s)
// ─────────────────────────────────────────────────────────────────────────────

pub struct MetamorphosisScene {
    pub duration: usize,
    pub palette: Palette,
}

impl Scene for MetamorphosisScene {
    fn name(&self) -> &str {
        "II. Metamorphosis & Organogenesis"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let w = canvas.width as f32;
        let h = canvas.height as f32;
        let cx = w * 0.5;
        let cy = h * 0.5;

        // 1. Fondo biológico con gradiente lineal rico
        let bg_top = Color::hex("#1a1423");
        let bg_mid = Color::hex("#13111c");
        let bg_bot = Color::hex("#09070d");
        canvas.fill_linear_gradient(
            0.0,
            0.0,
            0.0,
            h,
            &[
                (0.0, bg_top),
                (0.45, bg_mid),
                (1.0, bg_bot),
            ],
        );

        // 2. Luz puntual biológica (Golden Chrysalis Glow)
        let glow_intensity = 0.8 + (tau * PI * 4.0).sin() * 0.25;
        point_light(
            canvas,
            cx,
            cy,
            500.0,
            Color::hex("#f6ad55").with_alpha(0.55),
            glow_intensity,
        );

        // 3. Sistema de partículas: Chispas de energía metamórfica
        let mut sparks = ParticlePool::with_seed((frame as u32).wrapping_mul(91) + 7);
        sparks.emit(&EmitParams {
            cx,
            cy,
            count: 48,
            spread: SpreadShape::Disc { radius: 90.0 },
            speed_min: 15.0,
            speed_max: 65.0,
            life_min: 1.2,
            life_max: 3.5,
            size_min: 2.0,
            size_max: 5.5,
            size_end: 0.5,
            color_start: Color::hex("#fbd38d").with_alpha(0.75),
            color_end: Color::hex("#dd6b20").with_alpha(0.0),
            gravity_y: -12.0,
            shape: ParticleShape::Star,
            blend: BlendMode::Screen,
            ..Default::default()
        });
        sparks.update(tau * 3.0);
        sparks.draw(canvas);

        // 4. Formación de la Crisálida / Pupario con interpolación elástica
        let deform = ease_out_elastic(tau.min(0.85) / 0.85);
        let chrysalis_scale = 1.0 + deform * 0.45;

        canvas.save();
        canvas.translate(cx, cy);
        canvas.scale(chrysalis_scale, chrysalis_scale);

        // Segmentos quitinosos de la crisálida
        for seg in 0..7 {
            let seg_y = -90.0 + (seg as f32) * 28.0;
            let seg_w = (1.0 - ((seg as f32 - 3.0) / 4.0).powi(2)).max(0.2) * 55.0;
            let seg_h = 16.0;

            let pts = ellipse_points(0.0, seg_y, seg_w, seg_h, 32);
            let mut pb = PathBuilder::new();
            for (i, p) in pts.iter().enumerate() {
                if i == 0 {
                    pb.move_to(p[0], p[1]);
                } else {
                    pb.line_to(p[0], p[1]);
                }
            }
            pb.close();

            if let Some(path) = pb.finish() {
                let chitin_amber = Color::hex("#dd6b20").with_alpha(0.85);
                let chitin_dark = Color::hex("#7b341e").with_alpha(0.95);
                canvas.fill_path_linear_gradient(
                    &path,
                    -seg_w,
                    seg_y,
                    seg_w,
                    seg_y,
                    &[
                        (0.0, chitin_dark),
                        (0.35, chitin_amber),
                        (0.7, Color::hex("#f6e05e").with_alpha(0.85)),
                        (1.0, chitin_dark),
                    ],
                );
                if let Some(stroke) = wob_path(&pts, 1.2, (frame as u32) + seg as u32 * 5, true) {
                    canvas.stroke_path(&stroke, Color::hex("#2d3748"), 1.8);
                }
            }
        }

        // Disco imaginal alar translucido naciendo por los costados
        let wing_bud_alpha = (tau * 1.5).clamp(0.0, 0.85);
        let wing_pts = ellipse_points(60.0, -10.0, 42.0 * deform, 70.0 * deform, 32);
        let mut pb_w = PathBuilder::new();
        for (i, p) in wing_pts.iter().enumerate() {
            if i == 0 {
                pb_w.move_to(p[0], p[1]);
            } else {
                pb_w.line_to(p[0], p[1]);
            }
        }
        pb_w.close();
        if let Some(wp) = pb_w.finish() {
            canvas.fill_path(&wp, Color::hex("#63b3ed").with_alpha(wing_bud_alpha * 0.5));
            if let Some(ws) = wob_path(&wing_pts, 0.9, 555, true) {
                canvas.stroke_path(&ws, Color::hex("#90cdfa").with_alpha(wing_bud_alpha), 1.4);
            }
        }

        canvas.restore();

        // 5. Calina / Niebla de fondo
        ambient_fog(canvas, h * 0.7, Color::hex("#2a1b3d"), 0.4);

        // 6. HUD
        draw_scientific_hud(
            canvas,
            "02 // IMAGINAL DISC EVAGINATION",
            "STAGE: PUPA P6 (ECDYSONE PEAK)",
            "CHITINIZATION: 94.2%",
            frame,
            0.9,
        );

        vignette(canvas, 0.6, 1.3);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 3: "III. Wing Expansion & Eclosion" (96 fotogramas = 4.0s)
// ─────────────────────────────────────────────────────────────────────────────

pub struct WingExpansionScene {
    pub duration: usize,
    pub palette: Palette,
}

impl Scene for WingExpansionScene {
    fn name(&self) -> &str {
        "III. Wing Expansion & Eclosion"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let w = canvas.width as f32;
        let h = canvas.height as f32;
        let cx = w * 0.5;
        let cy = h * 0.5;

        // 1. Fondo atmosférico twilight cálido
        canvas.fill_radial_gradient(
            cx,
            cy,
            w * 0.75,
            &[
                (0.0, Color::hex("#172a3a")),
                (0.6, Color::hex("#09141d")),
                (1.0, Color::hex("#02080d")),
            ],
        );

        // 2. Foco de luz principal cenital
        let light_y = cy - 250.0 + (tau * PI * 2.0).sin() * 30.0;
        point_light(
            canvas,
            cx,
            light_y,
            460.0,
            Color::hex("#90cdfa").with_alpha(0.65),
            0.95,
        );

        // 3. Eclosión del Puppet FruitFly con expansión de alas usando Spring
        let wing_spread = spring(tau, 1.0, 120.0, 14.0);
        let fly_scale = 1.6 + ease_out_cubic(tau) * 0.3;

        let fly = FruitFly {
            x: cx,
            y: cy + 20.0,
            scale: fly_scale,
            wing_flap: wing_spread * 1.8,
            tilt: (tau * PI * 3.0).sin() * 0.04,
        };

        // Renderizado del puppet
        let seed = (frame as u32) + 1;
        fly.draw(canvas, self.palette(), seed, true);

        // 4. Detalle de venas alares con gradiente iridiscente
        canvas.save();
        canvas.translate(cx, cy + 20.0);
        canvas.scale(fly_scale, fly_scale);

        let vein_flow = (tau * 12.0) % 1.0;
        let stops = [
            (0.0, Color::hex("#63b3ed").with_alpha(0.1)),
            (vein_flow.clamp(0.0, 1.0), Color::hex("#f6e05e").with_alpha(0.85)),
            (1.0, Color::hex("#63b3ed").with_alpha(0.2)),
        ];
        canvas.fill_radial_gradient(0.0, -30.0, 120.0, &stops);

        canvas.restore();

        // 5. Partículas de escamas y gotas de rocío eclosionado
        let mut drops = ParticlePool::with_seed((frame as u32) * 5 + 3);
        drops.emit(&EmitParams {
            cx,
            cy: cy - 40.0,
            count: 32,
            spread: SpreadShape::Rect { w: 220.0, h: 120.0 },
            speed_min: 10.0,
            speed_max: 40.0,
            life_min: 1.5,
            life_max: 4.0,
            size_min: 2.0,
            size_max: 5.0,
            size_end: 0.5,
            color_start: Color::hex("#81e6d9").with_alpha(0.65),
            color_end: Color::hex("#172a3a").with_alpha(0.0),
            gravity_y: 8.0,
            shape: ParticleShape::Petal,
            blend: BlendMode::Screen,
            ..Default::default()
        });
        drops.update(tau * 2.0);
        drops.draw(canvas);

        // 6. Niebla ambiental
        ambient_fog(canvas, h * 0.75, Color::hex("#0d1b2a"), 0.35);

        // 7. HUD
        draw_scientific_hud(
            canvas,
            "03 // WING EXPANSION & SCLEROTIZATION",
            "HEMOLYMPH PRESSURE: 1.8 kPa",
            "EXPANSION: 14.2 um/s",
            frame,
            0.95,
        );

        vignette(canvas, 0.65, 1.4);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ESCENA 4: "IV. Kinetic Flight / El Ascenso Cinético" (96 fotogramas = 4.0s)
// ─────────────────────────────────────────────────────────────────────────────

pub struct KineticFlightScene {
    pub duration: usize,
    pub palette: Palette,
}

impl Scene for KineticFlightScene {
    fn name(&self) -> &str {
        "IV. Kinetic Flight / El Ascenso Cinético"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // 1. Fondo dinámico cinético con gradiente angular/lineal
        let bg_grad_top = Color::hex("#0a2540");
        let bg_grad_bot = Color::hex("#040d1a");
        canvas.fill_linear_gradient(
            0.0,
            0.0,
            w,
            h,
            &[
                (0.0, bg_grad_top),
                (0.5, Color::hex("#071629")),
                (1.0, bg_grad_bot),
            ],
        );

        // 2. Trayectoria cinemática en curva S usando ease_in_out_cubic
        let curve_t = ease_in_out_cubic(tau);
        let fly_x = w * 0.15 + curve_t * (w * 0.7);
        let flight_wave = (tau * PI * 5.0).sin() * 90.0;
        let fly_y = h * 0.68 - curve_t * (h * 0.38) + flight_wave;
        let fly_tilt = ((tau * PI * 5.0).cos() * 0.35) - 0.25;

        // 3. Fuente de luz puntual de seguimiento que acompaña a la mosca en vuelo
        point_light(
            canvas,
            fly_x,
            fly_y,
            380.0,
            Color::hex("#64ffda").with_alpha(0.65),
            1.1,
        );

        // 4. Estela de viento y partículas cinéticas de estela (`Streak`)
        let mut contrail = ParticlePool::with_seed((frame as u32).wrapping_mul(101) + 29);
        contrail.emit(&EmitParams {
            cx: fly_x - 30.0,
            cy: fly_y + 10.0,
            count: 24,
            spread: SpreadShape::Disc { radius: 25.0 },
            speed_min: 40.0,
            speed_max: 120.0,
            life_min: 0.6,
            life_max: 1.8,
            size_min: 2.0,
            size_max: 6.0,
            size_end: 0.5,
            color_start: Color::hex("#f6e05e").with_alpha(0.7),
            color_end: Color::hex("#0a2540").with_alpha(0.0),
            gravity_y: 4.0,
            shape: ParticleShape::Streak,
            blend: BlendMode::Screen,
            ..Default::default()
        });
        contrail.update(tau * 1.5);
        contrail.draw(canvas);

        // 5. Puppet FruitFly en vuelo a alta frecuencia de aleteo (200 Hz simulado)
        let wing_rapid_flap = tau * 48.0;
        let fly = FruitFly {
            x: fly_x,
            y: fly_y,
            scale: 2.1,
            wing_flap: wing_rapid_flap,
            tilt: fly_tilt,
        };
        let seed = (frame as u32) + 7;
        fly.draw(canvas, self.palette(), seed, false);

        // 6. Título final de producción cinematográfica (fade-in en la segunda mitad)
        if tau > 0.45 {
            let title_tau = ((tau - 0.45) / 0.55).clamp(0.0, 1.0);
            let title_alpha = ease_in_out_quint(title_tau);
            let title_color = Color::hex("#ccd6f6").with_alpha(title_alpha * 0.95);
            let sub_color = Color::hex("#64ffda").with_alpha(title_alpha * 0.85);

            draw_vector_text(
                canvas,
                w * 0.5,
                h * 0.82,
                "TALLER FILM // PROCEDURAL CINEMA",
                18.0,
                title_color,
                true,
            );
            draw_vector_text(
                canvas,
                w * 0.5,
                h * 0.86,
                "INSPIRED BY THE LIFE OF A FRUIT FLY - KEVIN T. NGO",
                11.0,
                sub_color,
                true,
            );
        }

        // 7. Niebla ambiental y HUD
        ambient_fog(canvas, h * 0.55, Color::hex("#061826"), 0.3);

        draw_scientific_hud(
            canvas,
            "04 // HIGH-SPEED AERODYNAMIC ASCENT",
            "WINGBEAT FREQ: 218 Hz",
            "REYNOLDS NUM: 140",
            frame,
            0.9,
        );

        vignette(canvas, 0.7, 1.45);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Generador del Film Completo y Soundtrack Científico Procedural
// ─────────────────────────────────────────────────────────────────────────────

/// Construye la película "Demo Quality: The Life of a Fruit Fly" (16 segundos)
/// configurada con `motion_blur = 0.22` para máxima fluidez orgánica.
pub fn create_demo_quality_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();
    timeline.fps = 24;
    timeline.on_twos = false; // Render a 24 fps continuos para máxima riqueza fluida
    timeline.motion_blur = 0.22; // 22% de acumulación de doble buffer para desenfoque orgánico

    let scene_frames = 24 * 4; // 4 segundos (96 fotogramas) por escena = 16 segundos en total
    let palette = Palette::paper_ink();

    timeline.add_scene(EmbryoScene {
        duration: scene_frames,
        palette: palette.clone(),
    });
    timeline.add_scene(MetamorphosisScene {
        duration: scene_frames,
        palette: palette.clone(),
    });
    timeline.add_scene(WingExpansionScene {
        duration: scene_frames,
        palette: palette.clone(),
    });
    timeline.add_scene(KineticFlightScene {
        duration: scene_frames,
        palette,
    });

    // Banda Sonora Procedural: Minimal Ambient Biomusic con sintetizador
    let total_secs = (scene_frames * 4) as f32 / 24.0;
    let mut audio = AudioTrack::new(total_secs, 44100);

    // 1. Drone sub-bass atmosférico continuo
    audio.add_note(0.0, total_secs, 65.4, Waveform::Sine, 0.18, 0.0);
    audio.add_note(0.0, total_secs, 130.8, Waveform::Sine, 0.08, 0.0);

    // 2. Pulsos rítmicos de arpegio pentatónico simulando escaneo microscópico
    let bpm = 120.0;
    let beat_len = 60.0 / bpm;
    let total_beats = (total_secs / beat_len) as usize;

    for b in 0..total_beats {
        let t = b as f32 * beat_len;
        let step = (b * 2) % 5;
        let freq = pent_hz(1.0, step, 220.0);

        let tone_len = 0.35;
        let vol = if b % 4 == 0 { 0.12 } else { 0.06 };
        let pan = if b % 2 == 0 { -0.35 } else { 0.35 };
        audio.add_note(t, tone_len, freq, Waveform::Triangle, vol, pan);

        // Clic de percusión sutil cada 2 tiempos
        if b % 2 == 0 {
            audio.add_note(t, 0.03, 1800.0, Waveform::Square, 0.03, 0.0);
        }
    }

    // 3. Glissando ascendente en la escena 4 (despegue)
    let takeoff_start = scene_frames as f32 * 3.0 / 24.0; // 12.0s
    let takeoff_dur = 3.5;
    let mut step_idx = 0;
    while (step_idx as f32 * 0.1) < takeoff_dur {
        let st = takeoff_start + step_idx as f32 * 0.1;
        let progress = (step_idx as f32 * 0.1) / takeoff_dur;
        let freq = 180.0 + progress * 520.0;
        audio.add_note(st, 0.12, freq, Waveform::Sine, 0.10 * (0.5 + progress * 0.5), 0.0);
        step_idx += 1;
    }

    (timeline, audio)
}
