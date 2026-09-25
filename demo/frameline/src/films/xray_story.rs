//! Film "X-Ray Story" (18 segundos, 6 escenas × 3 seg)
//!
//! Demuestra toda la riqueza visual del motor usando los efectos de `core::effects`:
//!
//! | Escena | Efecto principal             |
//! |--------|------------------------------|
//! | 1      | Pixel Dissolve (aparición)   |
//! | 2      | Vista exterior + Vignette    |
//! | 3      | X-Ray gradual + Bloom        |
//! | 4      | Chromatic Aberration         |
//! | 5      | VHS Glitch + Scanlines       |
//! | 6      | Film Burn + Vignette cierre  |

use crate::audio::{pent_hz, AudioTrack, Waveform};
use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette};
use crate::core::effects::{
    bloom_glow, chromatic_aberration, film_burn, pixel_dissolve,
    scanlines, smoothstep, vhs_glitch, vignette,
};
use crate::core::finishes::{draw_paper, draw_grain};
use crate::puppet::FruitFly;
use crate::timeline::{FilmTimeline, Scene};
use std::f32::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Escena 1: Aparición mágica por Pixel Dissolve
// ─────────────────────────────────────────────────────────────────────────────

pub struct DissolveInScene {
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for DissolveInScene {
    fn name(&self) -> &str { "1. Aparición por Dithering" }
    fn duration_frames(&self) -> usize { self.duration }
    fn palette(&self) -> &Palette { &self.palette }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = frame as u32 + 1;
        draw_paper(canvas, &self.palette, seed);

        let w = canvas.width as f32;
        let h = canvas.height as f32;

        let fly = FruitFly {
            x: w * 0.5,
            y: h * 0.5,
            scale: 1.6,
            wing_flap: tau * 1.5,
            tilt: (tau * PI).sin() * 0.04,
        };
        fly.draw(canvas, &self.palette, seed, false);

        // El dissolve revela la imagen de 0 → 1 a lo largo de tau
        let reveal = smoothstep(0.0, 0.9, tau);
        if reveal < 1.0 {
            pixel_dissolve(canvas, reveal, self.palette.paper, seed + 200);
        }

        // Viñeta suave permanente
        vignette(canvas, 0.35, 0.5);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Escena 2: Vista exterior con viñeta y grano de película
// ─────────────────────────────────────────────────────────────────────────────

pub struct ObservationScene {
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for ObservationScene {
    fn name(&self) -> &str { "2. Observación Exterior" }
    fn duration_frames(&self) -> usize { self.duration }
    fn palette(&self) -> &Palette { &self.palette }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = frame as u32 + 1;
        draw_paper(canvas, &self.palette, seed);

        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // Líneas de texto anotativo: etiquetas como en un atlas de biología
        draw_annotation_lines(canvas, w, h, &self.palette, tau, seed);

        let fly = FruitFly {
            x: w * 0.5,
            y: h * 0.5,
            scale: 1.8 + (tau * PI * 2.0).sin() * 0.05,
            wing_flap: tau * 2.0,
            tilt: (tau * PI * 2.0).sin() * 0.03,
        };
        fly.draw(canvas, &self.palette, seed, true);

        // Grano de película encima de todo
        let grain_path = full_frame_path(w, h);
        if let Some(path) = grain_path {
            draw_grain(canvas, &path, [0.0, 0.0, w, h], 300, self.palette.ink, 0.04, seed + 50, 2.0);
        }

        vignette(canvas, 0.45, 0.4);
    }
}

/// Dibuja líneas de anotación biológica con etiquetas punteadas
fn draw_annotation_lines(canvas: &mut Canvas, w: f32, h: f32, palette: &Palette, tau: f32, seed: u32) {
    let cx = w * 0.5;
    let cy = h * 0.5;
    let col = palette.guide.with_alpha(0.3 + (tau * PI * 2.0).sin() * 0.08);
    let width = 0.8;

    // Línea → tórax
    canvas.stroke_line(cx + 28.0, cy - 10.0, w * 0.78, cy - 50.0, col, width);
    canvas.fill_circle(cx + 28.0, cy - 10.0, 2.5, col);

    // Línea → ojo derecho
    canvas.stroke_line(cx + 18.0, cy - 42.0, w * 0.82, cy - 120.0, col, width);
    canvas.fill_circle(cx + 18.0, cy - 42.0, 2.0, col);

    // Línea → ala
    canvas.stroke_line(cx + 60.0, cy - 20.0, w * 0.80, h * 0.25, col, width);
    canvas.fill_circle(cx + 60.0, cy - 20.0, 2.0, col);

    // Horizontal de medición
    let bar_y = cy + 100.0;
    canvas.stroke_line(cx - 26.0, bar_y, cx + 26.0, bar_y, col, width);
    canvas.stroke_line(cx - 26.0, bar_y - 4.0, cx - 26.0, bar_y + 4.0, col, width);
    canvas.stroke_line(cx + 26.0, bar_y - 4.0, cx + 26.0, bar_y + 4.0, col, width);

    let _ = (seed, w, h); // silence unused warnings
}

// ─────────────────────────────────────────────────────────────────────────────
// Escena 3: Revelación X-Ray + Bloom
// ─────────────────────────────────────────────────────────────────────────────

pub struct XRayRevealScene {
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for XRayRevealScene {
    fn name(&self) -> &str { "3. Revelación X-Ray con Bloom" }
    fn duration_frames(&self) -> usize { self.duration }
    fn palette(&self) -> &Palette { &self.palette }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = frame as u32 + 1;
        // Fondo nocturno (Blueprint) para que el X-Ray luzca más dramático
        draw_paper(canvas, &self.palette, seed);

        let w = canvas.width as f32;
        let h = canvas.height as f32;

        let fly = FruitFly {
            x: w * 0.5,
            y: h * 0.5,
            scale: 1.7,
            wing_flap: tau * 2.0,
            tilt: 0.0,
        };

        // El X-Ray se revela progresivamente: smoothstep de 0.1 a 0.8 en tau
        let xray_reveal = smoothstep(0.1, 0.8, tau);
        fly.draw_with_xray(canvas, &self.palette, seed, false, xray_reveal);

        // Bloom aumenta junto con el X-Ray
        let bloom = smoothstep(0.2, 0.9, tau) * 0.55;
        bloom_glow(canvas, bloom, self.palette.chalk.with_alpha(1.0));

        vignette(canvas, 0.55, 0.35);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Escena 4: Velocidad extrema — Chromatic Aberration
// ─────────────────────────────────────────────────────────────────────────────

pub struct ChromaticFlightScene {
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for ChromaticFlightScene {
    fn name(&self) -> &str { "4. Vuelo a Velocidad — Aberración Cromática" }
    fn duration_frames(&self) -> usize { self.duration }
    fn palette(&self) -> &Palette { &self.palette }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = frame as u32 + 1;
        draw_paper(canvas, &self.palette, seed);

        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // Trayectoria S-curve: vuelo veloz a través de la pantalla
        let x = w * (0.08 + tau * 0.84);
        let y = h * 0.5 + (tau * PI * 3.0).sin() * 150.0;
        let tilt = (tau * PI * 3.0).cos() * 0.5;

        let fly = FruitFly {
            x,
            y,
            scale: 1.9 + (tau * PI * 6.0).sin() * 0.1,
            wing_flap: tau * 30.0, // Batimiento muy rápido
            tilt,
        };
        fly.draw(canvas, &self.palette, seed, false);

        // Rastros de movimiento: copias semitransparentes en posiciones previas
        for k in 1..=3 {
            let t_prev = (tau - k as f32 * 0.025).max(0.0);
            let xp = w * (0.08 + t_prev * 0.84);
            let yp = h * 0.5 + (t_prev * PI * 3.0).sin() * 150.0;
            let trail_fly = FruitFly { x: xp, y: yp, scale: 1.9, wing_flap: t_prev * 30.0, tilt };
            canvas.set_global_alpha(0.12 - k as f32 * 0.03);
            trail_fly.draw(canvas, &self.palette, seed + k as u32, false);
            canvas.set_global_alpha(1.0);
        }

        // Aberración cromática que crece con la velocidad (máximo en mitad del trayecto)
        let ca_intensity = ((-((tau - 0.5) * 2.0).powi(2) + 1.0) * 14.0) as i32;
        chromatic_aberration(canvas, ca_intensity.max(0));

        // Leve bloom de velocidad
        bloom_glow(canvas, 0.2, Color::hex("#ffffff"));
        vignette(canvas, 0.4, 0.5);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Escena 5: Archivo / memoria dañada — VHS Glitch + Scanlines
// ─────────────────────────────────────────────────────────────────────────────

pub struct VhsMemoryScene {
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for VhsMemoryScene {
    fn name(&self) -> &str { "5. Archivo VHS — Memoria Dañada" }
    fn duration_frames(&self) -> usize { self.duration }
    fn palette(&self) -> &Palette { &self.palette }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = frame as u32 + 1;
        draw_paper(canvas, &self.palette, seed);

        let w = canvas.width as f32;
        let h = canvas.height as f32;

        let fly = FruitFly {
            x: w * 0.5 + (tau * PI * 7.0).sin() * 8.0, // leve vibración
            y: h * 0.5,
            scale: 1.6,
            wing_flap: tau * 1.8,
            tilt: (tau * PI * 7.0).cos() * 0.02,
        };

        // X-Ray parcial como si fuera una tomografía VHS
        fly.draw_with_xray(canvas, &self.palette, seed, false, 0.45);

        // Glitch más intenso al inicio y al final de la escena
        let glitch_phase = if tau < 0.2 || tau > 0.75 { 1.0 } else { 0.3 };
        let glitch_px = (glitch_phase * 22.0) as i32;
        vhs_glitch(canvas, glitch_px, 6, seed + 300);

        // Scanlines constantes
        scanlines(canvas, 6, 0.12);

        // Leve degradación de color: desaturación simulada con velo gris
        let overlay_alpha = 0.08;
        let desaturation_path = full_frame_path(w, h);
        if let Some(path) = desaturation_path {
            canvas.fill_path(&path, Color::rgba(0.5, 0.5, 0.5, overlay_alpha));
        }

        vignette(canvas, 0.5, 0.35);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Escena 6: Cierre cinematográfico — Film Burn + Vignette intensa
// ─────────────────────────────────────────────────────────────────────────────

pub struct FilmBurnCloseScene {
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for FilmBurnCloseScene {
    fn name(&self) -> &str { "6. Cierre — Quemado de Película" }
    fn duration_frames(&self) -> usize { self.duration }
    fn palette(&self) -> &Palette { &self.palette }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = frame as u32 + 1;
        draw_paper(canvas, &self.palette, seed);

        let w = canvas.width as f32;
        let h = canvas.height as f32;

        let fly = FruitFly {
            x: w * 0.5,
            y: h * 0.5,
            scale: 1.6 - tau * 0.3, // se encoge al final
            wing_flap: tau * 0.5,   // alas lentas → reposo
            tilt: 0.0,
        };
        fly.draw(canvas, &self.palette, seed, false);

        // Quemado que aumenta progresivamente hasta consumir la imagen
        let burn_tau = smoothstep(0.3, 1.0, tau);
        film_burn(canvas, burn_tau, Color::hex("#ff8c00"));

        // Viñeta que se intensifica hacia el negro final
        let vignette_strength = 0.4 + burn_tau * 0.55;
        vignette(canvas, vignette_strength, 0.3);

        // En los últimos instantes: pixel dissolve inverso (desaparición)
        if tau > 0.82 {
            let dissolve_tau = 1.0 - smoothstep(0.82, 1.0, tau);
            pixel_dissolve(canvas, dissolve_tau, self.palette.paper, seed + 500);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Función constructora del film completo
// ─────────────────────────────────────────────────────────────────────────────

/// Construye el film "X-Ray Story" de 18 segundos con audio procedural suave.
pub fn create_xray_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();
    let fps = 24;
    let scene_frames = fps * 3; // 3 segundos por escena

    // Escena 1: Pixel Dissolve (papel cálido)
    timeline.add_scene(DissolveInScene {
        palette: Palette::paper_ink(),
        duration: scene_frames,
    });

    // Escena 2: Observación biológica (tinta sobre papel)
    timeline.add_scene(ObservationScene {
        palette: Palette::pencil_minimal(),
        duration: scene_frames,
    });

    // Escena 3: Revelación X-Ray (blueprint nocturno)
    timeline.add_scene(XRayRevealScene {
        palette: Palette::blueprint_night(),
        duration: scene_frames,
    });

    // Escena 4: Vuelo cromatizado (riso pop)
    timeline.add_scene(ChromaticFlightScene {
        palette: Palette::riso_pop(),
        duration: scene_frames,
    });

    // Escena 5: Archivo VHS (blueprint + efecto analógico)
    timeline.add_scene(VhsMemoryScene {
        palette: Palette::blueprint_night(),
        duration: scene_frames,
    });

    // Escena 6: Cierre de película (papel cálido)
    timeline.add_scene(FilmBurnCloseScene {
        palette: Palette::paper_ink(),
        duration: scene_frames,
    });

    // ── Banda sonora procedural sincronizada ────────────────────────────────
    let total_secs = timeline.total_duration_secs(); // 18 s
    let mut audio = AudioTrack::new(total_secs, 48000);

    // Pad ambiental de fondo: drone muy suave durante toda la película
    audio.add_ambient_pad(0.0, total_secs, pent_hz(-1.0, 0, 110.0), pent_hz(-1.0, 2, 110.0), 0.05, 1);

    // Eventos en cada corte de escena (cada 3 segundos)
    for i in 0..6usize {
        let t = i as f32 * 3.0;

        // Percusión suave en el corte (gain muy reducido)
        audio.add_noise_burst(t, 0.12, 0.08, i as u32 + 20);

        // Acorde pentatónico diferente por escena
        let f1 = pent_hz(0.0, i * 2,     220.0);
        let f2 = pent_hz(0.0, i * 2 + 2, 220.0);
        let f3 = pent_hz(1.0, i * 2 + 1, 220.0);

        audio.add_note(t,        1.6, f1, Waveform::Triangle, 0.10, -0.25);
        audio.add_note(t + 0.12, 1.4, f2, Waveform::Sine,     0.08,  0.25);
        audio.add_note(t + 0.25, 2.0, f3, Waveform::Saw,      0.06,  0.0);

        // Pulso rítmico a 1.0 y 2.0 dentro de cada escena
        audio.add_note(t + 1.0, 0.4, pent_hz(0.0, i + 1, 220.0), Waveform::Sine, 0.07, -0.15);
        audio.add_note(t + 2.0, 0.4, pent_hz(0.0, i + 3, 220.0), Waveform::Sine, 0.07,  0.15);

        // Escena 3 (X-Ray reveal): añadir un pad extra de tensión
        if i == 2 {
            audio.add_ambient_pad(t, 3.0, pent_hz(1.0, 0, 220.0), pent_hz(1.0, 4, 220.0), 0.07, 77);
        }

        // Escena 5 (VHS): ruido de tracking analógico
        if i == 4 {
            for k in 0..5u32 {
                audio.add_noise_burst(t + k as f32 * 0.55, 0.08, 0.05, k + 88);
            }
        }
    }

    (timeline, audio)
}

// ─────────────────────────────────────────────────────────────────────────────
// Utilidades internas
// ─────────────────────────────────────────────────────────────────────────────

/// Crea un path que cubre todo el canvas (para operaciones de grano / overlay)
fn full_frame_path(w: f32, h: f32) -> Option<tiny_skia::Path> {
    use crate::core::primitives::round_rect_points;
    let pts = round_rect_points(0.0, 0.0, w, h, 0.0);
    let mut pb = tiny_skia::PathBuilder::new();
    for (i, p) in pts.iter().enumerate() {
        if i == 0 { pb.move_to(p[0], p[1]); } else { pb.line_to(p[0], p[1]); }
    }
    pb.close();
    pb.finish()
}
