use crate::audio::synth::AudioTrack;
use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette};
use crate::core::effects::{bloom_glow, vignette};
use crate::core::schematic::{
    draw_blueprint_header, draw_blueprint_plate, draw_paper_plate, draw_vector_text,
};
use crate::puppet::*;
use crate::timeline::scene::Scene;
use crate::timeline::FilmTimeline;
use std::f32::consts::PI;

// =============================================================================
// PLACAS BASE MAESTRAS: ALTERNANCIA PAPEL GRABADO VS. RAYOS X ALQUÍMICO
// =============================================================================

/// Placa 1: Grabado Costumbrista sobre Papel Cálido
/// Textura de pergamino rayado a 30°, doble fileteado grabado y tipografía colonial
fn plate_grabado_costumbrista(
    canvas: &mut Canvas,
    w: u32,
    h: u32,
    title: &str,
    subtitle: &str,
    seed: usize,
    _tau: f32,
) {
    draw_paper_plate(canvas, w, h, (seed as u32) / 2);

    let wf = w as f32;
    let hf = h as f32;

    // Doble fileteado perimetral de grabado calcográfico
    let margin = 32.0;
    let border_dark = Color::hex("#3A2210");
    let border_gold = Color::hex("#8C6228");

    canvas.stroke_rect(margin, margin, wf - margin * 2.0, hf - margin * 2.0, border_dark, 2.5);
    canvas.stroke_rect(margin + 6.0, margin + 6.0, wf - (margin + 6.0) * 2.0, hf - (margin + 6.0) * 2.0, border_gold, 1.2);

    // Ornamentos en las 4 esquinas
    for &(cx, cy) in &[
        (margin + 6.0, margin + 6.0),
        (wf - margin - 6.0, margin + 6.0),
        (margin + 6.0, hf - margin - 6.0),
        (wf - margin - 6.0, hf - margin - 6.0),
    ] {
        canvas.fill_circle(cx, cy, 5.0, border_gold);
        canvas.stroke_circle(cx, cy, 5.0, border_dark, 1.2);
    }

    // Cabecera tipográfica grabada
    draw_vector_text(canvas, wf * 0.5, 68.0, title, 32.0, border_dark, true);
    if !subtitle.is_empty() {
        draw_vector_text(canvas, wf * 0.5, 102.0, subtitle, 13.5, border_gold, true);
    }
}

/// Placa 2: Rayos X Alquímico / Blueprint de Melquíades
/// Azul marino profundo (#0B1230), retícula de ingeniería 60px, reglas milimétricas y cyan luminoso
fn plate_rayos_x_alquimia(
    canvas: &mut Canvas,
    w: u32,
    h: u32,
    title: &str,
    subtitle: &str,
    seed: usize,
    _tau: f32,
) {
    draw_blueprint_plate(canvas, w, h, seed as u32);
    draw_blueprint_header(canvas, w as f32, title, subtitle);

    let wf = w as f32;
    let hf = h as f32;
    let ruler_col = Color::hex("#80D0FF");

    // Marcas milimétricas grabadas en márgenes laterales
    for side in [48.0, wf - 48.0] {
        for tick in 0..=36 {
            let ty = 140.0 + (tick as f32) * 22.0;
            let tick_len = if tick % 5 == 0 { 16.0 } else { 7.0 };
            let dir = if side < wf * 0.5 { 1.0 } else { -1.0 };
            canvas.stroke_line_fine(side, ty, side + dir * tick_len, ty, ruler_col.with_alpha(0.65), 1.2);
        }
    }

    // Coordenadas cartográficas de Macondo en esquinas
    draw_vector_text(canvas, 110.0, hf - 52.0, "LAT 10° 28' N • LONG 74° 12' W", 10.0, ruler_col.with_alpha(0.70), false);
    draw_vector_text(canvas, wf - 240.0, hf - 52.0, "MACONDO ARCHIVES • SERIE 100", 10.0, ruler_col.with_alpha(0.70), false);
}

// =============================================================================
// ESCENA 01: El Río de Piedras Prehistóricas (Grabado Continental)
// =============================================================================
pub struct RioPiedrasPrehistoricasScene { pub palette: Palette }

impl Scene for RioPiedrasPrehistoricasScene {
    fn name(&self) -> &str { "01 Río de Piedras Prehistóricas" }
    fn duration_frames(&self) -> usize { 194 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_grabado_costumbrista(canvas, w, h, "M A C O N D O", "CIEN ANOS DE SOLEDAD • OSCAR CHAVEZ", frame, tau);

        // Serranía montañosa al fondo con aguafuerte
        let mc = Color::hex("#5A7B82").with_alpha(0.60);
        let mut pb = tiny_skia::PathBuilder::new();
        pb.move_to(0.0, hf * 0.54);
        pb.cubic_to(wf * 0.18, hf * 0.28, wf * 0.35, hf * 0.40, wf * 0.52, hf * 0.32);
        pb.cubic_to(wf * 0.70, hf * 0.22, wf * 0.85, hf * 0.38, wf, hf * 0.35);
        pb.line_to(wf, hf * 0.54);
        pb.close();
        if let Some(p) = pb.finish() {
            canvas.fill_path(&p, mc);
            canvas.stroke_path_fine(&p, Color::hex("#2A444C"), 2.0);
        }

        // Río de aguas diáfanas en perspectiva
        let river_top = hf * 0.50;
        for row in 0..14 {
            let ry = river_top + row as f32 * 26.0;
            let mut wp = tiny_skia::PathBuilder::new();
            for s in 0..=48 {
                let sx = (s as f32 / 48.0) * wf;
                let phase = tau * 4.0 * PI + sx * 0.007 + row as f32 * 0.8;
                let sy = ry + phase.sin() * 7.0;
                if s == 0 { wp.move_to(sx, sy); } else { wp.line_to(sx, sy); }
            }
            if let Some(wp) = wp.finish() {
                canvas.stroke_path_fine(&wp, Color::hex("#1D7287").with_alpha(0.55), 2.5);
            }
        }

        // 5 Grandes piedras pulidas blancas como huevos prehistóricos en primer plano inferior
        let rocks = [
            (wf * 0.10, hf * 0.78, 140.0, 95.0, -0.18_f32),
            (wf * 0.30, hf * 0.84, 185.0, 125.0, 0.12_f32),
            (wf * 0.52, hf * 0.76, 210.0, 140.0, -0.06_f32),
            (wf * 0.74, hf * 0.82, 175.0, 115.0, 0.22_f32),
            (wf * 0.92, hf * 0.79, 150.0, 100.0, -0.14_f32),
        ];

        for (rx, ry, rw, rh, rot) in rocks {
            canvas.save();
            canvas.translate(rx, ry);
            canvas.rotate(rot);

            // Sombra suave en el lecho del río
            canvas.fill_circle(0.0, rh * 0.25, rw * 0.52, Color::hex("#143540").with_alpha(0.40));

            // Piedra de granito pulido
            let mut rock_pb = tiny_skia::PathBuilder::new();
            rock_pb.move_to(-rw * 0.5, 0.0);
            rock_pb.cubic_to(-rw * 0.5, -rh * 0.55, rw * 0.5, -rh * 0.55, rw * 0.5, 0.0);
            rock_pb.cubic_to(rw * 0.5, rh * 0.55, -rw * 0.5, rh * 0.55, -rw * 0.5, 0.0);
            rock_pb.close();

            if let Some(rp) = rock_pb.finish() {
                canvas.fill_path(&rp, Color::hex("#EFECE6"));
                canvas.stroke_path_fine(&rp, Color::hex("#686055"), 3.0);

                // Brillo especular cáustico
                let mut hl_pb = tiny_skia::PathBuilder::new();
                hl_pb.move_to(-rw * 0.32, -rh * 0.25);
                hl_pb.cubic_to(-rw * 0.10, -rh * 0.42, rw * 0.20, -rh * 0.38, rw * 0.32, -rh * 0.15);
                if let Some(hl) = hl_pb.finish() {
                    canvas.stroke_path_fine(&hl, Color::WHITE.with_alpha(0.85), 4.5);
                }
            }
            canvas.restore();
        }

        // Mariposas amarillas cruzando el horizonte
        for b in 0..7 {
            let bx = wf * (0.06 + b as f32 * 0.14) + (tau * 8.0 + b as f32 * 2.0).cos() * 60.0;
            let by = hf * 0.34 + (tau * 10.0 * PI + b as f32 * 1.8).sin() * 45.0;
            let flap = ((tau * 32.0 * PI + b as f32 * 1.7).sin()).abs();
            draw_realistic_yellow_butterfly(canvas, bx, by, 0.90, flap);
        }

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "LOS CIEN ANOS DE MACONDO SUENAN, SUENAN EN EL AIRE...", 20.0, Color::hex("#3A2210"), true);
        vignette(canvas, 0.14, 1.0);
    }
}

// =============================================================================
// ESCENA 02: Las Trompetas de Gabriel (Rayos X Alquímico)
// =============================================================================
pub struct TrompetasGabrielScene { pub palette: Palette }

impl Scene for TrompetasGabrielScene {
    fn name(&self) -> &str { "02 Trompetas de Gabriel" }
    fn duration_frames(&self) -> usize { 123 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_rayos_x_alquimia(canvas, w, h, "HERALDIC CLARION RESONANCE • GABRIEL", "ACOUSTIC PRESSURE HARMONIC MATRIX • 440 Hz (A4)", frame, tau);

        // Dos trompetas heráldicas simétricas a escala monumental (2.40x)
        // Trompeta izquierda apuntando hacia el centro
        draw_master_trumpet_heraldic(canvas, wf * 0.22, hf * 0.52, 2.40, -0.15, tau, true);

        // Trompeta derecha apuntando hacia el centro (espejo)
        canvas.save();
        canvas.translate(wf * 0.78, hf * 0.52);
        canvas.scale(-1.0, 1.0);
        draw_master_trumpet_heraldic(canvas, 0.0, 0.0, 2.40, -0.15, tau, true);
        canvas.restore();

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "TROMPETAS, TROMPETAS LO ANUNCIAN...", 22.0, Color::hex("#FFD700"), true);
        bloom_glow(canvas, 0.25, Color::hex("#F6C432"));
        vignette(canvas, 0.18, 1.0);
    }
}

// =============================================================================
// ESCENA 03: Don José Arcadio al Castaño (Grabado de Selva y Madera)
// =============================================================================
pub struct JoseArcadioCastanoScene { pub palette: Palette }

impl Scene for JoseArcadioCastanoScene {
    fn name(&self) -> &str { "03 Don José Arcadio al Castaño" }
    fn duration_frames(&self) -> usize { 221 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_grabado_costumbrista(canvas, w, h, "DON JOSE ARCADIO AL CASTANO", "ENCADENADO A MACONDO SUENA • EL PATRIARCA", frame, tau);

        // El castaño ancestral monumental a escala completa con Don José Arcadio encadenado
        draw_master_castano_ancestral(canvas, wf * 0.50, hf * 0.50, wf, hf, tau);

        // Re-estampar cabecera colonial para contraste prístino
        draw_vector_text(canvas, wf * 0.5, 68.0, "DON JOSE ARCADIO AL CASTANO", 32.0, Color::hex("#3A2210"), true);
        draw_vector_text(canvas, wf * 0.5, 102.0, "ENCADENADO A MACONDO SUENA • EL PATRIARCA", 13.5, Color::hex("#8C6228"), true);

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "ENCADENADO A MACONDO SUENA DON JOSE ARCADIO...", 20.0, Color::hex("#2C4215"), true);
        vignette(canvas, 0.16, 1.0);
    }
}

// =============================================================================
// ESCENA 04: La Tristeza de Aureliano (Taller Orfebre en Rayos X)
// =============================================================================
pub struct TristezaAurelianoScene { pub palette: Palette }

impl Scene for TristezaAurelianoScene {
    fn name(&self) -> &str { "04 La Tristeza de Aureliano" }
    fn duration_frames(&self) -> usize { 73 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_rayos_x_alquimia(canvas, w, h, "TALLER DE PLATERIA • CNEL. AURELIANO BUENDIA", "ORFEBRERIA ARTICULADA (24 VERTEBRAS) & EL CUATRO", frame, tau);

        // Cuatro llanero monumental a la izquierda (escala grande para llenar el cuadro)
        draw_master_cuatro(canvas, wf * 0.24, hf * 0.52, 2.50, 0.32, tau);

        // Taller orfebre y Pescadito de oro monumental en rayos X con 24 vértebras y engranajes suizos
        draw_master_pescadito_oro_alquimia(canvas, wf * 0.74, hf * 0.50, 1.65, tau);

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "LA TRISTEZA DE AURELIANO, EL CUATRO...", 22.0, Color::hex("#FFD700"), true);
        vignette(canvas, 0.16, 1.0);
    }
}

// =============================================================================
// ESCENA 05: La Belleza de Remedios (Luz Celestial y Violín)
// =============================================================================
pub struct BellezaRemediosScene { pub palette: Palette }

impl Scene for BellezaRemediosScene {
    fn name(&self) -> &str { "05 La Belleza de Remedios" }
    fn duration_frames(&self) -> usize { 149 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_grabado_costumbrista(canvas, w, h, "LA BELLEZA DE REMEDIOS", "ASCENSION EN SABANAS DE BRAMANTE • VIOLINES CANTAN", frame, tau);

        // Resplandor celestial en abanico ampliado
        for r in 0..20 {
            let rx = (r as f32) * (wf / 19.0);
            canvas.stroke_line_fine(rx, hf, wf * 0.38, 0.0, Color::hex("#F6C432").with_alpha(0.18), 14.0);
        }

        // Remedios la Bella ascendiendo con sábanas monumentales (escala equilibrada 1.95x)
        draw_master_remedios_ascension(canvas, wf * 0.32, hf * 0.52, 1.95, tau);

        // Violín monumental a la derecha en diagonal (voluta y clavijas 100% en encuadre)
        draw_master_violin(canvas, wf * 0.76, hf * 0.52, 2.60, -0.32, tau);

        // Re-estampar cabecera celestial
        draw_vector_text(canvas, wf * 0.5, 68.0, "LA BELLEZA DE REMEDIOS", 32.0, Color::hex("#3A2210"), true);
        draw_vector_text(canvas, wf * 0.5, 102.0, "ASCENSION EN SABANAS DE BRAMANTE • VIOLINES CANTAN", 13.5, Color::hex("#8C6228"), true);

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "LA BELLEZA DE REMEDIOS, VIOLINES, VIOLINES CANTAN...", 20.0, Color::hex("#3A2210"), true);
        vignette(canvas, 0.14, 1.0);
    }
}

// =============================================================================
// ESCENA 06: Las Pasiones de Amaranta (Mecánica Textil en Rayos X)
// =============================================================================
pub struct PasionesAmarantaScene { pub palette: Palette }

impl Scene for PasionesAmarantaScene {
    fn name(&self) -> &str { "06 Las Pasiones de Amaranta" }
    fn duration_frames(&self) -> usize { 151 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_rayos_x_alquimia(canvas, w, h, "LAS PASIONES DE AMARANTA", "EL TELAR DEL SUDARIO • MECANICA TEXTIL & GUITARRA", frame, tau);

        // Guitarra española a la izquierda en diagonal armónica — escala monumental
        draw_master_cuatro(canvas, wf * 0.23, hf * 0.52, 2.40, 0.30, tau);

        // Bastidor de mortaja y telar de encaje en rayos X a la derecha — más grande
        draw_master_bastidor_mortaja_alquimia(canvas, wf * 0.73, hf * 0.50, wf * 0.52, hf * 0.82, tau, frame as u32);

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "Y EL AMOR DE AMARANTA LLORA POR LAS ESQUINAS...", 22.0, Color::hex("#FFD700"), true);
        vignette(canvas, 0.16, 1.0);
    }
}

// =============================================================================
// ESCENA 07: El Embrujo de Melquíades y el Oboe (Rayos X Alquímico)
// =============================================================================
pub struct EmbrujoMelquiadesScene { pub palette: Palette }

impl Scene for EmbrujoMelquiadesScene {
    fn name(&self) -> &str { "07 El Embrujo de Melquíades" }
    fn duration_frames(&self) -> usize { 156 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_rayos_x_alquimia(canvas, w, h, "LABORATORIO ALQUIMICO • MELQUIADES", "ASTROLABIO, ESFERA ARMILAR & EL OBOE", frame, tau);

        // Oboe monumental reconocible a la izquierda (llaves francesas, tudel y caña doble, pabellón acampanado)
        draw_master_oboe_monumental(canvas, wf * 0.25, hf * 0.52, 1.20, -0.22, tau);

        // Esfera armilar y astrolabio monumental de Melquíades a la derecha
        draw_master_astrolabio_armilar_alquimia(canvas, wf * 0.74, hf * 0.50, 1.35, tau);

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "Y EL OBOE DE MELQUIADES RUEDA POR LAS CALLEJUELAS...", 22.0, Color::hex("#FFD700"), true);
        vignette(canvas, 0.16, 1.0);
    }
}

// =============================================================================
// ESCENA 08: Úrsula y la Soledad (Reloj de Arena Colosal)
// =============================================================================
pub struct UrsulaSoledadScene { pub palette: Palette }

impl Scene for UrsulaSoledadScene {
    fn name(&self) -> &str { "08 Úrsula y la Soledad" }
    fn duration_frames(&self) -> usize { 156 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_grabado_costumbrista(canvas, w, h, "URSULA Y LA SOLEDAD", "EL RELOJ DE ARENA COLOSAL • EL TIEMPO EN CARRUSEL", frame, tau);

        // Reloj de arena colosal centrado (1.85x que ocupa el 80% de la altura)
        draw_master_colossal_hourglass(canvas, wf * 0.50, hf * 0.50, 1.85, tau);

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "EL TIEMPO QUE DA VUELTAS COMO UN CARRUSEL...", 20.0, Color::hex("#3A2210"), true);
        vignette(canvas, 0.16, 1.0);
    }
}

// =============================================================================
// ESCENA 09: El Galeón Encallado en la Manigua (Arqueología Naval en Rayos X)
// =============================================================================
pub struct GaleonEncalladoScene { pub palette: Palette }

impl Scene for GaleonEncalladoScene {
    fn name(&self) -> &str { "09 El Galeón Encallado" }
    fn duration_frames(&self) -> usize { 192 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_rayos_x_alquimia(canvas, w, h, "ARQUEOLOGIA NAVAL • GALEON DEL SIGLO XVI", "ENCALLADO A 12 KM DEL MAR EN MEDIO DE LA SELVA VIRGEN", frame, tau);

        // Galeón naval en rayos X a escala monumental luminosa con velamen, cuadernas y lianas
        draw_master_galeon_naval_blueprint(canvas, wf * 0.50, hf * 0.52, wf * 0.88, hf * 0.72, tau);

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "EL GALEON ESPANOL VARADO EN LA MANIGUA...", 22.0, Color::hex("#FFD700"), true);
        vignette(canvas, 0.16, 1.0);
    }
}

// =============================================================================
// ESCENA 10: Mauricio Babilonia en el Platanal Auténtico (Plantación Tropical)
// =============================================================================
pub struct MariposasMauricioScene { pub palette: Palette }

impl Scene for MariposasMauricioScene {
    fn name(&self) -> &str { "10 Mariposas de Mauricio" }
    fn duration_frames(&self) -> usize { 240 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_grabado_costumbrista(canvas, w, h, "MAURICIO BABILONIA EN EL PLATANAL", "PLANTACION TROPICAL • HOJAS ANCHAS, RACIMOS & MARIPOSAS", frame, tau);

        // 4 Plataneras botánicas auténticas colosales en el encuadre (cubren el 75% del marco)
        let plantation_trees = [
            (wf * 0.12, hf * 0.90, 1.45, 0.0),
            (wf * 0.36, hf * 0.94, 1.65, 1.2),
            (wf * 0.64, hf * 0.92, 1.55, 2.4),
            (wf * 0.88, hf * 0.88, 1.40, 3.6),
        ];

        for (tx, ty, tscale, tphase) in plantation_trees {
            draw_master_banana_tree(canvas, tx, ty, tscale, tau * 6.0 * PI + tphase);
        }

        // Nube densa de 48 mariposas amarillas revoloteando por toda la plantación
        for b in 0..48 {
            let bx = wf * 0.06 + (b as f32 * 41.0) % (wf * 0.88) + (tau * 10.0 + b as f32 * 1.5).sin() * 45.0;
            let by = hf * 0.20 + (b as f32 * 19.0) % (hf * 0.58) + (tau * 12.0 * PI + b as f32 * 2.2).cos() * 35.0;
            let flap = ((tau * 36.0 * PI + b as f32 * 1.8).sin()).abs();
            draw_realistic_yellow_butterfly(canvas, bx, by, 0.88, flap);
        }

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "MARIPOSAS AMARILLAS, MAURICIO BABILONIA...", 20.0, Color::hex("#3A2210"), true);
        vignette(canvas, 0.14, 1.0);
    }
}

// =============================================================================
// ESCENA 11: Torbellino de Mariposas Liberadas (Rayos X Aerodinámica)
// =============================================================================
pub struct TorbellinoLiberadasScene { pub palette: Palette }

impl Scene for TorbellinoLiberadasScene {
    fn name(&self) -> &str { "11 Torbellino de Mariposas" }
    fn duration_frames(&self) -> usize { 300 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_rayos_x_alquimia(canvas, w, h, "AERODINAMICA DEL ENJAMBRE • PHOEBIS PHILEA", "CORRIENTE TERMICA ASCENDENTE & EL CUATRO SINCOPADO", frame, tau);

        // Cuatro a la izquierda a escala monumental 2.70 para llenar bien el cuadro
        draw_master_cuatro(canvas, wf * 0.23, hf * 0.52, 2.70, 0.12, tau);

        // Vórtice espiral de 72 mariposas anatómicas en la derecha
        for i in 0..72 {
            let t = i as f32 / 72.0;
            let ang = tau * 6.0 * PI + t * 16.0;
            let r = 40.0 + t * 560.0;
            let bx = wf * 0.64 + ang.cos() * r;
            let by = hf * 0.50 + ang.sin() * (r * 0.68);
            let flap = ((tau * 36.0 * PI + i as f32 * 1.7).sin()).abs();
            let bscale = 0.65 + t * 0.90;
            if bx > 0.0 && bx < wf && by > 0.0 && by < hf {
                draw_realistic_yellow_butterfly(canvas, bx, by, bscale, flap);
            }
        }

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "MARIPOSAS AMARILLAS QUE VUELAN LIBERADAS...", 22.0, Color::hex("#FFD700"), true);
        vignette(canvas, 0.16, 1.0);
    }
}

// =============================================================================
// ESCENA 12: Cumbia Sinfónica: Acordeón Hohner y Tren Bananero
// =============================================================================
pub struct PuenteAcordeonMacondoScene { pub palette: Palette }

impl Scene for PuenteAcordeonMacondoScene {
    fn name(&self) -> &str { "12 Puente de Acordeón y Banano" }
    fn duration_frames(&self) -> usize { 432 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_grabado_costumbrista(canvas, w, h, "CUMBIA DE MACONDO", "EL ACORDEON HOHNER Y EL TREN DE LA BANANERA", frame, tau);

        // Acordeón Hohner monumental en la parte superior (escala 2.80 para dominar el cuadro)
        draw_master_accordion(canvas, wf * 0.50, hf * 0.33, 2.80, tau);

        // Locomotora Baldwin 1910 y tren bananero histórico en la parte inferior
        draw_master_locomotora_bananera(canvas, wf * 0.50, hf * 0.74, wf, tau);

        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "CUMBIA SINFONICA DE MACONDO • LA MEMORIA DEL PUEBLO", 20.0, Color::hex("#3A2210"), true);
        vignette(canvas, 0.15, 1.0);
    }
}

// =============================================================================
// ESCENAS 13 A 24: ACTO II (REPRISES CON ALTERNANCIA Y CIERRE CÓSMICO)
// =============================================================================

pub struct SuenoEnElAireRepriseScene { pub palette: Palette }
impl Scene for SuenoEnElAireRepriseScene {
    fn name(&self) -> &str { "13 Sueño en el Aire Reprise" }
    fn duration_frames(&self) -> usize { 194 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        RioPiedrasPrehistoricasScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct TrompetasOroScene { pub palette: Palette }
impl Scene for TrompetasOroScene {
    fn name(&self) -> &str { "14 Trompetas de Oro" }
    fn duration_frames(&self) -> usize { 125 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        TrompetasGabrielScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct JoseArcadioFantasmaScene { pub palette: Palette }
impl Scene for JoseArcadioFantasmaScene {
    fn name(&self) -> &str { "15 Fantasma de José Arcadio" }
    fn duration_frames(&self) -> usize { 221 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        JoseArcadioCastanoScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct AurelianoGuerraScene { pub palette: Palette }
impl Scene for AurelianoGuerraScene {
    fn name(&self) -> &str { "16 Aureliano y sus Guerras" }
    fn duration_frames(&self) -> usize { 73 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        TristezaAurelianoScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct RemediosCosmosScene { pub palette: Palette }
impl Scene for RemediosCosmosScene {
    fn name(&self) -> &str { "17 Remedios en el Cosmos" }
    fn duration_frames(&self) -> usize { 149 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        BellezaRemediosScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct AmarantaGuitarraScene { pub palette: Palette }
impl Scene for AmarantaGuitarraScene {
    fn name(&self) -> &str { "18 Guitarra de Amaranta" }
    fn duration_frames(&self) -> usize { 151 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        PasionesAmarantaScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct MelquiadesProfeciaScene { pub palette: Palette }
impl Scene for MelquiadesProfeciaScene {
    fn name(&self) -> &str { "19 Profecía de Melquíades" }
    fn duration_frames(&self) -> usize { 156 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        EmbrujoMelquiadesScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct UrsulaCienAnosScene { pub palette: Palette }
impl Scene for UrsulaCienAnosScene {
    fn name(&self) -> &str { "20 Úrsula Ciento Quince Años" }
    fn duration_frames(&self) -> usize { 156 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        UrsulaSoledadScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct GaleonEternoScene { pub palette: Palette }
impl Scene for GaleonEternoScene {
    fn name(&self) -> &str { "21 El Galeón Eterno" }
    fn duration_frames(&self) -> usize { 192 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        GaleonEncalladoScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct MauricioMemeScene { pub palette: Palette }
impl Scene for MauricioMemeScene {
    fn name(&self) -> &str { "22 Mauricio y Meme" }
    fn duration_frames(&self) -> usize { 240 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        MariposasMauricioScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

pub struct TorbellinoFinalScene { pub palette: Palette }
impl Scene for TorbellinoFinalScene {
    fn name(&self) -> &str { "23 Torbellino Final" }
    fn duration_frames(&self) -> usize { 300 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        TorbellinoLiberadasScene { palette: self.palette.clone() }.render(canvas, tau, frame);
    }
}

/// Escena 24: El Viento Bíblico / Cierre Cósmico
pub struct CumbiaFinalScene { pub palette: Palette }
impl Scene for CumbiaFinalScene {
    fn name(&self) -> &str { "24 El Viento Bíblico y Cierre" }
    fn duration_frames(&self) -> usize { 478 }
    fn palette(&self) -> &Palette { &self.palette }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let wf = w as f32;
        let hf = h as f32;

        plate_rayos_x_alquimia(canvas, w, h, "EL VIENTO BIBLICO • EPILOGO", "LA PROFECIA CUMPLIDA • PERGAMINOS DE MELQUIADES", frame, tau);

        // Vórtice bíblico monumental con pergaminos volando en 3D
        draw_viento_biblico_monumental(canvas, wf * 0.50, hf * 0.48, wf, hf, tau, frame as u32);

        // Sentencia final de la obra en letras de oro luminosas
        draw_vector_text(canvas, wf * 0.5, hf - 52.0, "NO TENIAN UNA SEGUNDA OPORTUNIDAD SOBRE LA TIERRA", 22.0, Color::hex("#FFD700"), true);
        vignette(canvas, 0.20, 1.0);
    }
}

// =============================================================================
// FUNCIÓN PRINCIPAL DE CONSTRUCCIÓN DEL FILM
// =============================================================================

pub fn create_macondo_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();
    timeline.fps = 24;
    timeline.on_twos = false;

    let def_pal = Palette::paper_ink();

    timeline.add_scene(RioPiedrasPrehistoricasScene { palette: def_pal.clone() });
    timeline.add_scene(TrompetasGabrielScene { palette: def_pal.clone() });
    timeline.add_scene(JoseArcadioCastanoScene { palette: def_pal.clone() });
    timeline.add_scene(TristezaAurelianoScene { palette: def_pal.clone() });
    timeline.add_scene(BellezaRemediosScene { palette: def_pal.clone() });
    timeline.add_scene(PasionesAmarantaScene { palette: def_pal.clone() });
    timeline.add_scene(EmbrujoMelquiadesScene { palette: def_pal.clone() });
    timeline.add_scene(UrsulaSoledadScene { palette: def_pal.clone() });
    timeline.add_scene(GaleonEncalladoScene { palette: def_pal.clone() });
    timeline.add_scene(MariposasMauricioScene { palette: def_pal.clone() });
    timeline.add_scene(TorbellinoLiberadasScene { palette: def_pal.clone() });
    timeline.add_scene(PuenteAcordeonMacondoScene { palette: def_pal.clone() });

    timeline.add_scene(SuenoEnElAireRepriseScene { palette: def_pal.clone() });
    timeline.add_scene(TrompetasOroScene { palette: def_pal.clone() });
    timeline.add_scene(JoseArcadioFantasmaScene { palette: def_pal.clone() });
    timeline.add_scene(AurelianoGuerraScene { palette: def_pal.clone() });
    timeline.add_scene(RemediosCosmosScene { palette: def_pal.clone() });
    timeline.add_scene(AmarantaGuitarraScene { palette: def_pal.clone() });
    timeline.add_scene(MelquiadesProfeciaScene { palette: def_pal.clone() });
    timeline.add_scene(UrsulaCienAnosScene { palette: def_pal.clone() });
    timeline.add_scene(GaleonEternoScene { palette: def_pal.clone() });
    timeline.add_scene(MauricioMemeScene { palette: def_pal.clone() });
    timeline.add_scene(TorbellinoFinalScene { palette: def_pal.clone() });
    timeline.add_scene(CumbiaFinalScene { palette: def_pal });

    let audio = load_macondo_audio();

    (timeline, audio)
}

fn load_macondo_audio() -> AudioTrack {
    AudioTrack::new(215.0, 48000)
}
