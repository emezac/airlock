use crate::audio::synth::{AudioTrack, Waveform};
use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette};
use crate::core::effects::vignette;
use crate::core::rng::Rng;
use crate::core::schematic::*;
use crate::puppet::monarch::*;
use crate::timeline::scene::{FilmTimeline, Scene};
use std::f32::consts::PI;

// =============================================================================
// ESCENA 1: Hero on Milkweed (0.0s - 2.0s, 48 fotogramas)
// =============================================================================
struct HeroOnMilkweedScene {
    palette: Palette,
}

impl Scene for HeroOnMilkweedScene {
    fn name(&self) -> &str {
        "01 Hero on Milkweed"
    }
    fn duration_frames(&self) -> usize {
        48
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;
        let cy = h as f32 * 0.46;

        // Fondo de papel rayado diagonal a 30° (-0.52 rad)
        draw_paper_plate(canvas, w, h, (frame as u32) / 2);

        // Inflorescencia esférica botánica de algodoncillo (Asclepias syriaca) con pedicelos y hojas
        let flower_cy = cy + 140.0;
        draw_botanical_milkweed_ball(canvas, cx, flower_cy, 130.0, h as f32, (frame as u32) / 2);

        // Arcos celestes astronómicos de orientación sobre la heroína
        canvas.stroke_circle(cx, cy - 60.0, 240.0, Color::hex("#EAB530").with_alpha(0.55), 1.6);
        canvas.stroke_circle(cx, cy - 60.0, 320.0, Color::hex("#EAB530").with_alpha(0.35), 1.2);
        // Pequeñas marcas de grado en el arco
        for i in 0..16 {
            let a = PI + (i as f32 / 15.0) * PI;
            let r1 = 312.0;
            let r2 = 328.0;
            canvas.stroke_line(cx + a.cos() * r1, (cy - 60.0) + a.sin() * r1, cx + a.cos() * r2, (cy - 60.0) + a.sin() * r2, Color::hex("#EAB530").with_alpha(0.6), 1.2);
        }

        // Animación de la mariposa:
        // En los primeros 16 fotogramas (tau < 0.35) está posada con las alas cerradas en perfil lateral
        // En el beat 2 (tau >= 0.35) abre las alas súbitamente a envergadura completa G5 de 1000px
        if tau < 0.35 {
            let rest_scale = 1.35;
            draw_monarch_resting(canvas, cx, flower_cy - 100.0, rest_scale, (frame as u32) / 2);
        } else {
            let open_tau = ((tau - 0.35) / 0.65).min(1.0);
            let flap = if open_tau < 0.25 {
                (open_tau / 0.25 * PI).sin() * 0.95
            } else if open_tau < 0.55 {
                ((open_tau - 0.25) / 0.3 * PI).sin() * 0.40
            } else {
                0.0
            };

            // Ondas de choque expansivas amarillas y magenta en el tórax al batir las alas
            if open_tau < 0.50 {
                let burst_p = open_tau / 0.50;
                let br1 = 90.0 + burst_p * 380.0;
                let br2 = 50.0 + burst_p * 260.0;
                let b_alpha = (1.0 - burst_p) * 0.85;
                canvas.stroke_circle(cx, cy - 60.0, br1, Color::hex("#EAB530").with_alpha(b_alpha), 3.0);
                canvas.stroke_circle(cx, cy - 60.0, br2, Color::hex("#E43D8C").with_alpha(b_alpha * 0.9), 2.0);
            }

            let scale = 1.65 + open_tau * 0.05;
            draw_monarch(canvas, cx, cy - 60.0, scale, flap, false, (frame as u32) / 2);
        }

        // Cartela de título técnico dentro del área segura de Shorts
        draw_vector_text(canvas, cx, h as f32 - 140.0, "DANAUS PLEXIPPUS • HEROINE", 16.0, Color::hex("#2A1C13"), true);
        draw_vector_text(canvas, cx, h as f32 - 105.0, "FRAME 001 - ON COMMON MILKWEED (ASCLEPIAS SYRIACA)", 11.0, Color::hex("#5B4331"), true);

        vignette(canvas, 0.18, 1.0);
    }
}

// =============================================================================
// ESCENA 2: Egg Blueprint (2.0s - 4.0s, 48 fotogramas)
// =============================================================================
struct EggBlueprintScene {
    palette: Palette,
}

impl Scene for EggBlueprintScene {
    fn name(&self) -> &str {
        "02 Egg Blueprint"
    }
    fn duration_frames(&self) -> usize {
        48
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;
        let cy = h as f32 * 0.46;

        // Base auténtica de anteproyecto (blueprint plate) en azul marino con rejilla 60px
        draw_wireframe_egg_3d_blueprint(canvas, cx, cy + 20.0, 240.0, 360.0, tau);
        draw_stage_cycle_glyph(canvas, w as f32 - 140.0, 220.0, 42.0, 0);

        draw_vector_text(canvas, cx, h as f32 - 120.0, "STAGE I: EMBRYOGENESIS", 15.0, Color::hex("#EEF0FF"), true);
        draw_vector_text(canvas, cx, h as f32 - 90.0, "CELLULAR DIVISION IN CHORION SHELL", 11.0, Color::hex("#C8C1EF"), true);
    }
}

// =============================================================================
// ESCENA 3: Egg Hatch (4.0s - 5.5s, 36 fotogramas)
// =============================================================================
struct EggHatchScene {
    palette: Palette,
}

impl Scene for EggHatchScene {
    fn name(&self) -> &str {
        "03 Egg Hatch"
    }
    fn duration_frames(&self) -> usize {
        36
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;

        // Fondo de papel rayado cálido
        draw_paper_plate(canvas, w, h, (frame as u32) / 2);

        // Envés de la hoja botánica gigante de algodoncillo ocupando el tercio superior
        let leaf_base_y = 520.0;
        draw_giant_leaf_underside(canvas, w as f32, leaf_base_y, (frame as u32) / 2);

        // Huevo suspendido bajo la hoja botánica en modo papel
        let egg_cy = 880.0;
        draw_ribbed_egg_chorion(canvas, cx, egg_cy, 220.0, 320.0, false);

        // Oruga de primer estadio eclosionando y masticando el cascarón
        draw_egg_hatch_caterpillar(canvas, cx, egg_cy, tau, (frame as u32) / 2);

        // Lupa técnica enfocando la eclosión
        draw_magnifying_loupe(canvas, cx + 220.0, egg_cy + 100.0, 75.0, "ECLOSION 10X", Color::hex("#C8A47A"));

        draw_vector_text(canvas, cx, h as f32 - 130.0, "FIRST MEAL: THE EGGSHELL", 15.0, Color::hex("#2A1C13"), true);
        draw_vector_text(canvas, cx, h as f32 - 95.0, "INSTAR 1 LARVA CONSUMES THE NUTRIENT CHORION", 11.0, Color::hex("#5B4331"), true);

        vignette(canvas, 0.18, 1.0);
    }
}

// =============================================================================
// ESCENA 4: Caterpillar Climbing (5.5s - 8.0s, 60 fotogramas)
// =============================================================================
struct CaterpillarClimbingScene {
    palette: Palette,
}

impl Scene for CaterpillarClimbingScene {
    fn name(&self) -> &str {
        "04 Caterpillar Climbing"
    }
    fn duration_frames(&self) -> usize {
        60
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.54;

        draw_paper_plate(canvas, w, h, (frame as u32) / 2);

        // Placa botánica completa: tallo vertical, 3 pares de hojas opuestas con mordiscos, oruga y regla
        draw_instar_stem_botanical_plate(canvas, cx, w as f32, h as f32, tau, (frame as u32) / 2);

        draw_vector_text(canvas, cx - 120.0, h as f32 - 120.0, "FOUR MOLTS UP THE STEM (INSTAR 1-5)", 15.0, Color::hex("#2A1C13"), true);
        draw_vector_text(canvas, cx - 120.0, h as f32 - 90.0, "SEQUESTERING CARDENOLIDES FOR CHEMICAL DEFENSE", 10.0, Color::hex("#5B4331"), true);

        vignette(canvas, 0.18, 1.0);
    }
}

// =============================================================================
// ESCENA 5: Instar Ladder (8.0s - 9.5s, 36 fotogramas)
// =============================================================================
struct InstarLadderScene {
    palette: Palette,
}

impl Scene for InstarLadderScene {
    fn name(&self) -> &str {
        "05 Instar Ladder"
    }
    fn duration_frames(&self) -> usize {
        36
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.48;

        draw_blueprint_plate(canvas, w, h, (frame as u32) / 2);
        draw_blueprint_header(canvas, w as f32, "SCHEMATIC G2: LARVAL INSTARS & TRACHEAL SYSTEM", "EXPONENTIAL MASS GAIN 2000X");
        draw_stage_cycle_glyph(canvas, w as f32 - 140.0, 220.0, 42.0, 1);

        let stages = [
            ("INSTAR I", "3 mm", 0.32, h as f32 * 0.22),
            ("INSTAR II", "7 mm", 0.48, h as f32 * 0.38),
            ("INSTAR III", "15 mm", 0.68, h as f32 * 0.54),
            ("INSTAR IV", "28 mm", 0.92, h as f32 * 0.70),
            ("INSTAR V", "45 mm", 1.25, h as f32 * 0.86),
        ];

        let visible = ((tau * 5.5) as usize).min(5);
        for (i, &(lbl, sz, sc, y_pos)) in stages.iter().enumerate().take(visible) {
            draw_caterpillar(canvas, cx, y_pos, sc, 0.0, (frame as u32) / 2);
            draw_vector_text(canvas, 100.0, y_pos - 8.0, lbl, 13.0, Color::hex("#EEF0FF"), false);
            draw_vector_text(canvas, 100.0, y_pos + 14.0, sz, 10.0, Color::hex("#C8C1EF"), false);

            // Cota técnica
            draw_dimension_bracket(canvas, cx - 180.0 * sc, y_pos - 25.0 * sc, cx + 180.0 * sc, y_pos - 25.0 * sc, sz, Color::hex("#C8C1EF"), 16.0);

            // Estadio 5 con discos imaginales
            if i == 4 {
                canvas.fill_circle(cx + 90.0, y_pos, 8.0, Color::hex("#FF3D98"));
                canvas.stroke_circle(cx + 90.0, y_pos, 18.0, Color::hex("#FF3D98"), 1.4);
                draw_node_callout(canvas, cx + 90.0, y_pos, "WING DISC", "IMAGINAL BUD", Color::hex("#FF3D98"), true);
            }
        }
    }
}

// =============================================================================
// ESCENA 6: J-Hang & Pupation (9.5s - 11.5s, 48 fotogramas)
// =============================================================================
struct JHangPupationScene {
    palette: Palette,
}

impl Scene for JHangPupationScene {
    fn name(&self) -> &str {
        "06 J-Hang & Pupation"
    }
    fn duration_frames(&self) -> usize {
        48
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;

        draw_paper_plate(canvas, w, h, (frame as u32) / 2);

        // Rama gnarled de madera rugosa superior a lo ancho de la pantalla
        let branch_underside = 300.0_f32;
        let branch_top = 235.0_f32;
        draw_gnarled_branch_twig(canvas, w as f32, branch_underside, branch_top);

        // Botón de seda blanca anclado en la rama
        canvas.fill_circle(cx, branch_underside + 6.0, 11.0, Color::hex("#FBF6EA"));
        canvas.stroke_circle(cx, branch_underside + 6.0, 11.0, Color::hex("#2A1C13"), 1.2);

        // Sistema de péndulo y transportador de gravedad con plomada
        draw_pendulum_protractor_system(canvas, cx, branch_underside, 1500.0, tau);

        if tau < 0.48 {
            // Curvatura auténtica en "J"
            let j_pts = [
                (cx, branch_underside + 12.0),
                (cx, branch_underside + 80.0),
                (cx + 8.0, branch_underside + 160.0),
                (cx + 28.0, branch_underside + 230.0),
                (cx + 72.0, branch_underside + 270.0),
                (cx + 115.0, branch_underside + 240.0),
            ];
            for (idx, &(px, py)) in j_pts.iter().enumerate() {
                let r = 24.0 - (idx as f32) * 1.8;
                canvas.fill_circle(px, py, r, Color::hex("#221A15"));
                canvas.fill_circle(px, py, r * 0.8, Color::hex("#E6BE3A"));
                canvas.fill_circle(px, py, r * 0.52, Color::hex("#F1ECDF"));
                canvas.fill_circle(px, py, r * 0.26, Color::hex("#221A15"));
            }
            // Tentáculos colgantes
            canvas.stroke_line(cx + 115.0, branch_underside + 240.0, cx + 145.0, branch_underside + 210.0, Color::hex("#221A15"), 4.0);

            draw_vector_text(canvas, cx, h as f32 - 120.0, "THE J-HANG • CUTICLE SOFTENING", 15.0, Color::hex("#2A1C13"), true);
        } else {
            // Crisálida de jade formada con borde dorado
            let scale = 1.75;
            draw_chrysalis(canvas, cx, branch_underside + 220.0, scale, 0.0, false, (frame as u32) / 2);

            draw_vector_text(canvas, cx, h as f32 - 120.0, "PUPATION • JADE CHRYSALIS FORMED", 15.0, Color::hex("#2A1C13"), true);
        }

        vignette(canvas, 0.18, 1.0);
    }
}

// =============================================================================
// ESCENA 7: Inside Chrysalis (11.5s - 13.0s, 36 fotogramas)
// =============================================================================
struct InsideChrysalisScene {
    palette: Palette,
}

impl Scene for InsideChrysalisScene {
    fn name(&self) -> &str {
        "07 Inside Chrysalis"
    }
    fn duration_frames(&self) -> usize {
        36
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.54;
        let cy = h as f32 * 0.46;

        draw_blueprint_plate(canvas, w, h, (frame as u32) / 2);
        draw_blueprint_header(canvas, w as f32, "SCHEMATIC G3: HISTOLYSIS & IMAGINAL DISCS", "METAMORPHOSIS • NOT MELTED");
        draw_stage_cycle_glyph(canvas, w as f32 - 140.0, 220.0, 42.0, 2);

        // Contorno de la crisálida en blueprint
        draw_chrysalis(canvas, cx, cy - 40.0, 1.8, 0.0, true, (frame as u32) / 2);

        // 5 nodos diagramáticos a la izquierda conectados por curvas finas de lavanda
        let node_y_start = 320.0;
        let node_x = 160.0;
        let targets = [
            ("HEAD & OPTIC LOBE", cx - 20.0, cy - 100.0),
            ("ANTENNAL IMAGINAL DISC", cx + 25.0, cy - 90.0),
            ("FLIGHT THORACIC MUSCLES", cx, cy - 10.0),
            ("FOREWING BUD (G5)", cx - 45.0, cy + 60.0),
            ("HINDWING BUD (G5)", cx + 45.0, cy + 90.0),
        ];

        for (i, &(lbl, tx, ty)) in targets.iter().enumerate() {
            let ny = node_y_start + (i as f32) * 110.0;
            // Nodo circular
            canvas.stroke_circle(node_x, ny, 20.0, Color::hex("#C8C1EF"), 1.4);
            canvas.fill_circle(node_x, ny, 5.0, Color::hex("#FF3D98"));

            // Curva de conexión
            let mut c_pb = tiny_skia::PathBuilder::new();
            c_pb.move_to(node_x + 20.0, ny);
            c_pb.cubic_to((node_x + tx) * 0.5, ny, (node_x + tx) * 0.6, ty, tx, ty);
            if let Some(cp) = c_pb.finish() {
                canvas.stroke_path(&cp, Color::hex("#C8C1EF").with_alpha(0.5), 1.0);
            }

            // Punto diana pulsante
            let pulse = (tau * PI * 6.0 + i as f32).sin().abs() * 3.0;
            canvas.fill_circle(tx, ty, 6.0 + pulse, Color::hex("#FF3D98"));
            canvas.stroke_circle(tx, ty, 14.0 + pulse, Color::hex("#EEF0FF"), 1.2);

            draw_vector_text(canvas, node_x + 30.0, ny - 5.0, lbl, 9.5, Color::hex("#EEF0FF"), false);
        }

        draw_vector_text(canvas, cx, h as f32 - 120.0, "HISTOLYSIS & ORGANOGENESIS", 15.0, Color::hex("#EEF0FF"), true);
        draw_vector_text(canvas, cx, h as f32 - 90.0, "IMAGINAL CLUSTERS BUILD ADULT WINGS & MUSCLES", 11.0, Color::hex("#C8C1EF"), true);
    }
}

// =============================================================================
// ESCENA 8: Twelve Days Arc (13.0s - 16.0s, 72 fotogramas)
// =============================================================================
struct TwelveDaysArcScene {
    palette: Palette,
}

impl Scene for TwelveDaysArcScene {
    fn name(&self) -> &str {
        "08 Twelve Days Arc"
    }
    fn duration_frames(&self) -> usize {
        72
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;
        let cy = h as f32 * 0.44;

        draw_paper_plate(canvas, w, h, (frame as u32) / 2);

        // Ventana arqueada celestial, alféizar y hoja en perspectiva con reloj solar
        draw_celestial_window_chrysalis_days(canvas, cx, cy, w as f32, h as f32, tau);

        // Rama superior
        let branch_underside = 300.0_f32;
        let branch_top = 235.0_f32;
        draw_gnarled_branch_twig(canvas, w as f32, branch_underside, branch_top);

        // Crisálida madurando: en los días 9 a 12 se oscurece y revela las alas naranjas
        let wing_reveal = if tau < 0.6 { 0.0 } else { ((tau - 0.6) / 0.4).min(1.0) };
        draw_chrysalis(canvas, cx, branch_underside + 220.0, 1.8, wing_reveal, false, (frame as u32) / 2);

        let curr_day = ((tau * 12.0) as usize).min(11);
        let day_label = format!("DAY {:02} OF 12 • WING CASES VISIBLE", curr_day + 1);
        draw_vector_text(canvas, cx, h as f32 - 120.0, &day_label, 15.0, Color::hex("#2A1C13"), true);

        vignette(canvas, 0.18, 1.0);
    }
}

// =============================================================================
// ESCENA 9: Eclosion (16.0s - 18.0s, 48 fotogramas)
// =============================================================================
struct EclosionScene {
    palette: Palette,
}

impl Scene for EclosionScene {
    fn name(&self) -> &str {
        "09 Eclosion"
    }
    fn duration_frames(&self) -> usize {
        48
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;
        let cy = h as f32 * 0.42;

        draw_paper_plate(canvas, w, h, (frame as u32) / 2);

        // Rama gnarled superior
        let branch_underside = 300.0_f32;
        let branch_top = 235.0_f32;
        draw_gnarled_branch_twig(canvas, w as f32, branch_underside, branch_top);

        // Cáscara transparente vacía colgada arriba con fisura de eclosión
        let mut shell_pb = tiny_skia::PathBuilder::new();
        shell_pb.move_to(cx, branch_underside + 10.0);
        shell_pb.cubic_to(cx + 45.0, branch_underside + 50.0, cx + 35.0, branch_underside + 130.0, cx, branch_underside + 160.0);
        shell_pb.cubic_to(cx - 35.0, branch_underside + 130.0, cx - 45.0, branch_underside + 50.0, cx, branch_underside + 10.0);
        if let Some(sp) = shell_pb.finish() {
            canvas.fill_path(&sp, Color::hex("#FBF6EA").with_alpha(0.45));
            canvas.stroke_path(&sp, Color::hex("#8A735C"), 1.8);
            // Hendidura de eclosión
            canvas.stroke_line(cx, branch_underside + 40.0, cx, branch_underside + 140.0, Color::hex("#2A1C13"), 2.0);
        }

        // Mariposa adulta recién emergida colgando hacia abajo
        let expand = 0.50 + tau * 0.50;
        let monarch_y = branch_underside + 260.0;
        canvas.save();
        canvas.translate(cx, monarch_y);
        canvas.scale(expand, expand);
        draw_monarch(canvas, 0.0, 0.0, 1.35, 0.70 - tau * 0.35, false, (frame as u32) / 2);
        canvas.restore();

        // Gota de meconio rojo-marrón cayendo
        canvas.fill_circle(cx, monarch_y + 110.0 + tau * 60.0, 5.5, Color::hex("#8E2B22"));

        // Lupa circular mostrando la probóscide enrollada cerrándose
        let loupe_x = 220.0;
        let loupe_y = h as f32 * 0.58;
        draw_magnifying_loupe(canvas, loupe_x, loupe_y, 75.0, "COILED PROBOSCIS", Color::hex("#C8A47A"));
        // Espiral de probóscide dentro de la lupa
        let mut pr_pb = tiny_skia::PathBuilder::new();
        pr_pb.move_to(loupe_x, loupe_y - 20.0);
        for s in 0..30 {
            let t = s as f32 / 30.0;
            let a = t * PI * 3.5;
            let r = 24.0 * (1.0 - t * 0.7);
            pr_pb.line_to(loupe_x + a.cos() * r * 0.4, loupe_y + a.sin() * r);
        }
        if let Some(pp) = pr_pb.finish() {
            canvas.stroke_path(&pp, Color::hex("#2A1C13"), 2.2);
        }

        draw_dimension_bracket(canvas, cx - 180.0, h as f32 - 180.0, cx + 180.0, h as f32 - 180.0, "FULL EXPANSION 20 MIN", Color::hex("#2A1C13"), 15.0);
        draw_vector_text(canvas, cx, h as f32 - 120.0, "ECLOSION: PUMPING HEMOLYMPH", 15.0, Color::hex("#2A1C13"), true);
        draw_vector_text(canvas, cx, h as f32 - 90.0, "WINGS EXPAND TO FLIGHT SURFACE AREA", 11.0, Color::hex("#5B4331"), true);

        vignette(canvas, 0.18, 1.0);
    }
}

// =============================================================================
// ESCENA 10: Wing Hydraulics (18.0s - 19.5s, 36 fotogramas)
// =============================================================================
struct WingHydraulicsScene {
    palette: Palette,
}

impl Scene for WingHydraulicsScene {
    fn name(&self) -> &str {
        "10 Wing Hydraulics"
    }
    fn duration_frames(&self) -> usize {
        36
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;
        let cy = h as f32 * 0.46;

        draw_blueprint_plate(canvas, w, h, (frame as u32) / 2);
        draw_blueprint_header(canvas, w as f32, "SCHEMATIC G4: WING HEMOLYMPH HYDRAULICS", "PRESSURE 2.4 kPa • VENATION NET");
        draw_stage_cycle_glyph(canvas, w as f32 - 140.0, 220.0, 42.0, 3);

        // Adulto en silueta técnica en blueprint
        draw_monarch(canvas, cx, cy, 1.6, 0.0, true, (frame as u32) / 2);

        // Puntos y pulsos de hemolinfa que recorren los canales venosos
        let flow_dist = tau * 320.0;
        let angles = [0.12 * PI, 0.30 * PI, 0.50 * PI, 0.70 * PI, 0.88 * PI];
        for &a in &angles {
            for &side in &[-1.0_f32, 1.0_f32] {
                let px = cx + side * a.cos() * flow_dist;
                let py = cy - a.sin() * flow_dist * 0.85;
                canvas.fill_circle(px, py, 6.0, Color::hex("#BFEFF5"));
                canvas.stroke_circle(px, py, 12.0, Color::hex("#C8C1EF"), 1.2);
            }
        }

        // Círculo satélite de sección transversal de vena
        let loupe_x = 180.0;
        let loupe_y = h as f32 * 0.42;
        draw_magnifying_loupe(canvas, loupe_x, loupe_y, 70.0, "VEIN CROSS SECTION", Color::hex("#C8C1EF"));
        canvas.stroke_circle(loupe_x, loupe_y, 35.0, Color::hex("#BFEFF5"), 2.0);
        canvas.stroke_circle(loupe_x, loupe_y, 15.0, Color::hex("#EEF0FF"), 1.5);

        draw_dimension_bracket(canvas, cx - 240.0, cy - 220.0, cx + 240.0, cy - 220.0, "WINGSPAN 100 mm", Color::hex("#C8C1EF"), 25.0);
        draw_vector_text(canvas, cx, h as f32 - 120.0, "PRIMARY VENATION: SC, R1-R5, M1-M3, CU, 2A", 15.0, Color::hex("#EEF0FF"), true);
    }
}

// =============================================================================
// ESCENA 11: Scale Mosaic Macro (19.5s - 21.5s, 48 fotogramas)
// =============================================================================
struct ScaleMosaicMacroScene {
    palette: Palette,
}

impl Scene for ScaleMosaicMacroScene {
    fn name(&self) -> &str {
        "11 Scale Mosaic Macro"
    }
    fn duration_frames(&self) -> usize {
        48
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;

        // Mosaico masivo macroscópico de escamas individuales festoneadas
        draw_massive_scale_mosaic(canvas, w as f32, h as f32, tau, (frame as u32) / 2);

        draw_dimension_bracket(canvas, cx - 120.0, h as f32 * 0.5 + 300.0, cx + 120.0, h as f32 * 0.5 + 300.0, "50 µm SCALE", Color::hex("#FBF6EA"), 18.0);
        draw_vector_text(canvas, cx, h as f32 - 110.0, "CHITIN SHINGLE MOSAIC • STRUCTURAL RIDGES", 15.0, Color::hex("#FBF6EA"), true);

        vignette(canvas, 0.22, 1.0);
    }
}

// =============================================================================
// ESCENA 12: Sun Compass Navigation (21.5s - 23.5s, 48 fotogramas)
// =============================================================================
struct SunCompassScene {
    palette: Palette,
}

impl Scene for SunCompassScene {
    fn name(&self) -> &str {
        "12 Sun Compass Navigation"
    }
    fn duration_frames(&self) -> usize {
        48
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;
        let cy = h as f32 * 0.44;

        draw_blueprint_plate(canvas, w, h, (frame as u32) / 2);
        draw_blueprint_header(canvas, w as f32, "SCHEMATIC G5: TIME-COMPENSATED SUN COMPASS", "ANTENNAL CLOCK • MERIDIAN AZIMUTH");
        draw_stage_cycle_glyph(canvas, w as f32 - 140.0, 220.0, 42.0, 3);

        // Transportador celeste circular de 360 grados
        let compass_r = 260.0;
        canvas.stroke_circle(cx, cy, compass_r, Color::hex("#C8C1EF"), 1.8);
        canvas.stroke_circle(cx, cy, compass_r * 0.75, Color::hex("#C8C1EF").with_alpha(0.35), 1.0);
        for i in 0..36 {
            let a = (i as f32 / 36.0) * PI * 2.0;
            let is_maj = i % 3 == 0;
            let r1 = compass_r;
            let r2 = compass_r - if is_maj { 20.0 } else { 10.0 };
            canvas.stroke_line(cx + a.cos() * r1, cy + a.sin() * r1, cx + a.cos() * r2, cy + a.sin() * r2, Color::hex("#C8C1EF"), if is_maj { 1.6 } else { 0.9 });
        }

        // Rayo de azimut solar magenta disparando a través del retículo
        let sun_deg = 50.0 + tau * 45.0;
        let sun_rad = sun_deg * PI / 180.0;
        let heading_rad = 215.0 * PI / 180.0; // Rumbo SO hacia México

        canvas.stroke_line(cx, cy, cx + sun_rad.cos() * (compass_r + 40.0), cy + sun_rad.sin() * (compass_r + 40.0), Color::hex("#FF3D98"), 3.0);
        canvas.fill_circle(cx + sun_rad.cos() * (compass_r + 40.0), cy + sun_rad.sin() * (compass_r + 40.0), 9.0, Color::hex("#FF3D98"));

        // Vector de rumbo de vuelo corregido por reloj circadiano
        canvas.stroke_line(cx, cy, cx + heading_rad.cos() * compass_r, cy + heading_rad.sin() * compass_r, Color::hex("#EEF0FF"), 3.0);

        // Cabeza y antenas de la monarca en el centro
        canvas.fill_circle(cx, cy, 28.0, Color::hex("#C8C1EF").with_alpha(0.8));
        canvas.stroke_circle(cx, cy, 28.0, Color::hex("#EEF0FF"), 1.8);
        // Antenas
        canvas.stroke_line(cx - 15.0, cy - 10.0, cx - 65.0, cy - 85.0, Color::hex("#EEF0FF"), 2.5);
        canvas.stroke_line(cx + 15.0, cy - 10.0, cx + 65.0, cy - 85.0, Color::hex("#EEF0FF"), 2.5);
        canvas.fill_circle(cx - 65.0, cy - 85.0, 7.0, Color::hex("#FFF3DC"));
        canvas.fill_circle(cx + 65.0, cy - 85.0, 7.0, Color::hex("#FFF3DC"));

        draw_vector_text(canvas, cx, h as f32 - 120.0, "CIRCADIAN COMPENSATED CELESTIAL STEERING", 15.0, Color::hex("#EEF0FF"), true);
        draw_vector_text(canvas, cx, h as f32 - 90.0, "ANTENNAL CLOCK INTEGRATES SOLAR AZIMUTH", 11.0, Color::hex("#C8C1EF"), true);
    }
}

// =============================================================================
// ESCENA 13: Continental Flyways (23.5s - 26.0s, 60 fotogramas)
// =============================================================================
struct ContinentalFlywaysScene {
    palette: Palette,
}

impl Scene for ContinentalFlywaysScene {
    fn name(&self) -> &str {
        "13 Continental Flyways"
    }
    fn duration_frames(&self) -> usize {
        60
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;
        let cy = h as f32 * 0.45;

        // Mapa cartográfico histórico de Norteamérica con líneas de costa, Grandes Lagos y rutas migratorias
        draw_north_america_migration_map(canvas, cx, cy, 1.22, tau);

        // Mariposas planeando a lo largo de las rutas
        for i in 0..6 {
            let t = (tau + i as f32 * 0.16) % 1.0;
            let mx = cx + (1.0 - t) * 120.0 - t * 90.0;
            let my = (cy - 340.0) + t * 680.0;
            draw_monarch(canvas, mx, my, 0.28, (tau * PI * 8.0 + i as f32).sin() * 0.4, false, (frame as u32) / 2 + i as u32);
        }

        draw_vector_text(canvas, cx, h as f32 - 110.0, "CONTINENTAL FLYWAYS • 4,000 KM JOURNEY", 15.0, Color::hex("#2A1C13"), true);
        draw_vector_text(canvas, cx, h as f32 - 80.0, "EASTERN POPULATION CONVERGENCE TO MICHOACAN", 11.0, Color::hex("#5B4331"), true);

        vignette(canvas, 0.20, 1.0);
    }
}

// =============================================================================
// ESCENA 14: Fall Flight Swarm (26.0s - 28.5s, 60 fotogramas)
// =============================================================================
struct FallFlightSwarmScene {
    palette: Palette,
}

impl Scene for FallFlightSwarmScene {
    fn name(&self) -> &str {
        "14 Fall Flight Swarm"
    }
    fn duration_frames(&self) -> usize {
        60
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;

        // Paisaje panorámico de migración con colinas, valle y río de cientos de mariposas
        draw_migration_panoramic_landscape(canvas, w as f32, h as f32, tau, (frame as u32) / 2);

        draw_vector_text(canvas, cx, h as f32 - 110.0, "FALL FLIGHT • GLIDING ON THERMAL CURRENTS", 15.0, Color::hex("#2A1C13"), true);
        draw_vector_text(canvas, cx, h as f32 - 80.0, "SOARING HUNDREDS OF KILOMETERS PER DAY", 11.0, Color::hex("#5B4331"), true);

        vignette(canvas, 0.18, 1.0);
    }
}

// =============================================================================
// ESCENA 15: Oyamel Fir Forest (28.5s - 30.5s, 48 fotogramas)
// =============================================================================
struct OyamelForestScene {
    palette: Palette,
}

impl Scene for OyamelForestScene {
    fn name(&self) -> &str {
        "15 Oyamel Fir Forest"
    }
    fn duration_frames(&self) -> usize {
        48
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, _tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;

        // Cielo nocturno crepuscular profundo (`#2F2748`)
        canvas.clear(Color::hex("#2F2748"));

        // Fases lunares alineadas arriba
        draw_moon_phases(canvas, cx, 140.0, w as f32);

        // Santuario de abetos sagrados Oyamel con racimos densos durmientes y niebla
        draw_oyamel_winter_roost(canvas, cx, h as f32 * 0.5, w as f32, h as f32, (frame as u32) / 2);

        draw_vector_text(canvas, cx, h as f32 - 120.0, "OYAMEL FIR SANCTUARY • SIERRA CHINCUA", 15.0, Color::hex("#FBF6EA"), true);
        draw_vector_text(canvas, cx, h as f32 - 90.0, "OVERWINTERING COLONIES • CLUSTERED DORMANCY", 11.0, Color::hex("#CFC6E0"), true);

        vignette(canvas, 0.28, 1.0);
    }
}

// =============================================================================
// ESCENA 16: Closing Cycle (30.5s - 32.0s, 36 fotogramas)
// =============================================================================
struct ClosingCycleScene {
    palette: Palette,
}

impl Scene for ClosingCycleScene {
    fn name(&self) -> &str {
        "16 Closing Cycle"
    }
    fn duration_frames(&self) -> usize {
        36
    }
    fn palette(&self) -> &Palette {
        &self.palette
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let (w, h) = (canvas.width, canvas.height);
        let cx = w as f32 * 0.5;
        let cy = h as f32 * 0.44;

        // Malla alámbrica 3D del huevo cerrando el ciclo en plano azul profundo
        draw_wireframe_egg_3d_blueprint(canvas, cx, cy, 240.0, 360.0, tau);

        // Wordmark oficial en minúscula según art-bible.md regla 9
        draw_vector_text(canvas, cx, 1470.0, "m o n a r c h", 44.0, Color::hex("#C8C1EF"), true);

        draw_vector_text(canvas, cx, h as f32 - 120.0, "THE FOUR GENERATIONS OF THE MONARCH", 15.0, Color::hex("#EEF0FF"), true);
        draw_vector_text(canvas, cx, h as f32 - 90.0, "ONE SPECIES • ONE CONTINENT • ETERNAL LOOP", 11.0, Color::hex("#C8C1EF"), true);
    }
}

// =============================================================================
// COMPOSICIÓN DE LA BANDA SONORA (120 BPM SINCRONIZADA A 32 SEGUNDOS)
// =============================================================================
pub fn generate_monarch_soundtrack() -> AudioTrack {
    let mut track = AudioTrack::new(32.0, 48000);
    let bpm = 120.0;
    let beat_dur = 60.0 / bpm; // 0.5 segundos por beat

    // Progresión armónica de pads orquestales suaves (Am, F, C, G, Dm, Em, F, C)
    let chord_sections = [
        (0.0, 4.0, 220.0, 329.63, 101),
        (4.0, 8.0, 174.61, 261.63, 102),
        (8.0, 12.0, 130.81, 196.00, 103),
        (12.0, 16.0, 196.00, 293.66, 104),
        (16.0, 20.0, 146.83, 220.00, 105),
        (20.0, 24.0, 164.81, 246.94, 106),
        (24.0, 28.0, 174.61, 261.63, 107),
        (28.0, 32.0, 130.81, 261.63, 108),
    ];

    for (start, end, f_low, f_high, seed) in chord_sections {
        track.add_ambient_pad(start, end - start, f_low, f_high, 0.05, seed);
    }

    // Melodía cristalina de celesta con envolvente suave en cada corte de compás
    let melody_notes = [
        // Frase 1: El despertar (Hero / Egg)
        (0.0, 523.25, 0.0), (0.5, 659.25, 0.2), (1.0, 783.99, -0.2), (1.5, 1046.50, 0.0),
        (2.0, 880.00, 0.3), (3.0, 659.25, -0.3), (4.0, 587.33, 0.0), (5.5, 659.25, 0.2),
        // Frase 2: El crecimiento (Caterpillar / Chrysalis)
        (8.0, 783.99, -0.2), (9.5, 880.00, 0.0), (11.5, 1046.50, 0.3), (13.0, 987.77, -0.3),
        (14.0, 783.99, 0.0), (15.0, 659.25, 0.2), (16.0, 523.25, -0.2), (17.5, 659.25, 0.0),
        // Frase 3: La transformación y vuelo (Wings / Navigation)
        (18.0, 783.99, 0.3), (19.5, 1046.50, -0.3), (21.5, 1174.66, 0.0), (23.5, 987.77, 0.2),
        (24.0, 880.00, -0.2), (25.0, 783.99, 0.0), (26.0, 659.25, 0.3), (27.5, 587.33, -0.3),
        // Frase 4: Santuario y Cierre (Oyamel / Cycle)
        (28.5, 523.25, 0.0), (29.5, 659.25, 0.2), (30.5, 783.99, -0.2), (31.0, 1046.50, 0.0),
    ];

    for (t, freq, pan) in melody_notes {
        track.add_note(t, 0.65, freq, Waveform::Sine, 0.08, pan);
    }

    // Acentos de bajo sub y ráfagas de ruido filtradas en los cortes de escena
    let scene_cuts = [
        0.0, 2.0, 4.0, 5.5, 8.0, 9.5, 11.5, 13.0,
        16.0, 18.0, 19.5, 21.5, 23.5, 26.0, 28.5, 30.5,
    ];

    for (idx, &cut_t) in scene_cuts.iter().enumerate() {
        // Tono grave tipo bombo acústico
        track.add_note(cut_t, 0.30, 65.41, Waveform::Triangle, 0.08, 0.0);
        // Chasquido percusivo filtrado
        track.add_noise_burst(cut_t, 0.06, 0.05, 500 + idx as u32);
    }

    // Pulsos suaves de compás a 120 bpm
    let mut beat_t = 0.0;
    let mut b_count = 0;
    while beat_t < 31.8 {
        let is_downbeat = b_count % 4 == 0;
        let gain = if is_downbeat { 0.04 } else { 0.02 };
        track.add_noise_burst(beat_t, 0.025, gain, 1000 + b_count);
        beat_t += beat_dur;
        b_count += 1;
    }

    track.normalize_peak(0.80);
    track
}

// =============================================================================
// ENVOLTORIO DE ALTA FIDELIDAD (AUTHENTIC MASTER SCENE)
// =============================================================================
fn try_draw_cached_frame(canvas: &mut Canvas, frame: usize) -> bool {
    let frame_idx = (frame + 1).clamp(1, 768);
    let path = format!("assets/monarch/frame_{:04}.jpg", frame_idx);
    if std::path::Path::new(&path).exists() {
        if canvas.draw_image_file(&path).is_ok() {
            return true;
        }
    }
    false
}

struct AuthenticMasterScene<S: Scene> {
    inner: S,
}

impl<S: Scene> Scene for AuthenticMasterScene<S> {
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn duration_frames(&self) -> usize {
        self.inner.duration_frames()
    }
    fn palette(&self) -> &Palette {
        self.inner.palette()
    }
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        if try_draw_cached_frame(canvas, frame) {
            return;
        }
        self.inner.render(canvas, tau, frame);
    }
}

// =============================================================================
// FUNCIÓN PRINCIPAL DE CONSTRUCCIÓN DEL FILM
// =============================================================================
pub fn create_monarch_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();
    timeline.fps = 24;
    timeline.on_twos = true;

    let def_pal = Palette::paper_ink();

    timeline.add_scene(AuthenticMasterScene { inner: HeroOnMilkweedScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: EggBlueprintScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: EggHatchScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: CaterpillarClimbingScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: InstarLadderScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: JHangPupationScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: InsideChrysalisScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: TwelveDaysArcScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: EclosionScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: WingHydraulicsScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: ScaleMosaicMacroScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: SunCompassScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: ContinentalFlywaysScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: FallFlightSwarmScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: OyamelForestScene { palette: def_pal.clone() } });
    timeline.add_scene(AuthenticMasterScene { inner: ClosingCycleScene { palette: def_pal } });

    let soundtrack = generate_monarch_soundtrack();

    (timeline, soundtrack)
}

