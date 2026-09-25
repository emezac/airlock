use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::rng::Rng;
use crate::core::schematic::{
    draw_blueprint_header, draw_dimension_bracket, draw_magnifying_loupe, draw_node_callout,
    draw_vector_text,
};
use crate::puppet::macondo::{
    draw_g5_yellow_butterfly, draw_pescadito_oro, draw_reloj_arena_ursula, draw_yellow_butterfly,
};
use tiny_skia::BlendMode;
use std::f32::consts::PI;

// =============================================================================
// COMPONENTES MONUMENTALES Y RAYOS X (X-RAY) DE ALTA PRECISIÓN PARA MACONDO
// =============================================================================

/// 1. Rayos X y corte de orfebrería de alta precisión del Pececito de Oro de Aureliano
/// Revela la columna vertebral articulada de 24 vértebras de oro, engranajes microscópicos y cotas milimétricas.
pub fn draw_xray_pescadito_oro(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let gold_line = Color::hex("#FFDF70");
    let cyan_glow = Color::hex("#00F5D4");
    let ruby_red = Color::hex("#FF0055");
    let faint_grid = Color::hex("#3A4A86").with_alpha(0.40);
    let ruler_col = Color::hex("#80D0FF");

    canvas.save();
    canvas.translate(cx, cy);

    // Escalas verticales milimétricas de taller en los márgenes izquierdo y derecho
    for side in [-480.0, 480.0] {
        canvas.stroke_line_fine(side, -600.0, side, 600.0, ruler_col.with_alpha(0.6), 1.5);
        for tick in 0..=40 {
            let ty = -600.0 + (tick as f32) * 30.0;
            let tick_len = if tick % 5 == 0 { 18.0 } else { 8.0 };
            let dir = if side < 0.0 { 1.0 } else { -1.0 };
            canvas.stroke_line_fine(side, ty, side + dir * tick_len, ty, ruler_col.with_alpha(0.7), 1.2);
            if tick % 5 == 0 {
                let mm = tick * 3;
                let val_str = format!("{:02}", mm);
                draw_vector_text(canvas, side + dir * 32.0, ty + 4.0, &val_str, 9.0, ruler_col.with_alpha(0.8), true);
            }
        }
    }

    // Retículo de calibración de taller orfebre
    for i in -5..=5 {
        let gx = (i as f32) * 65.0 * scale;
        canvas.stroke_line_fine(gx, -240.0 * scale, gx, 240.0 * scale, faint_grid, 0.7);
    }
    for j in -4..=4 {
        let gy = (j as f32) * 65.0 * scale;
        canvas.stroke_line_fine(-360.0 * scale, gy, 360.0 * scale, gy, faint_grid, 0.7);
    }

    // Dibujo del pez central a escala monumental
    draw_pescadito_oro(canvas, 0.0, 0.0, scale * 1.6, 0.5);

    // Activamos BlendMode::Screen para que el esqueleto de rayos X emita luz espectral
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);

    // Columna vertebral axial: 24 vértebras articuladas
    let vertebra_count = 24;
    let spine_len = 220.0 * scale;
    for v in 0..vertebra_count {
        let t = v as f32 / (vertebra_count - 1) as f32;
        let vx = -spine_len * 0.5 + t * spine_len;
        let wave = (tau * 12.0 * PI + t * 4.0).sin() * (5.0 * scale * t);
        let vy = wave;

        // Vértebra
        canvas.fill_circle(vx, vy, 4.0 * scale, cyan_glow);
        canvas.stroke_circle(vx, vy, 6.5 * scale, gold_line, 1.2 * scale);

        // Costillas articuladas espinosas dorsales y ventrales
        if v % 2 == 0 && v > 2 && v < vertebra_count - 2 {
            let rib_h = (1.0 - (t - 0.45).abs() * 1.8).max(0.2) * 44.0 * scale;
            // Costilla dorsal
            canvas.stroke_line_fine(vx, vy, vx - 5.0 * scale, vy - rib_h, cyan_glow.with_alpha(0.85), 1.4 * scale);
            // Costilla ventral
            canvas.stroke_line_fine(vx, vy, vx - 5.0 * scale, vy + rib_h, cyan_glow.with_alpha(0.85), 1.4 * scale);
        }
    }

    // Engranajes microscópicos de relojería suiza en el vientre del pez
    let gear_cx = -20.0 * scale;
    let gear_cy = 8.0 * scale;
    let gear_r = 24.0 * scale;
    canvas.stroke_circle(gear_cx, gear_cy, gear_r, gold_line, 1.6 * scale);
    canvas.fill_circle(gear_cx, gear_cy, 4.5 * scale, ruby_red);
    let teeth = 14;
    for t in 0..teeth {
        let a = (t as f32 / teeth as f32) * PI * 2.0 + tau * 4.0 * PI;
        let p1x = gear_cx + a.cos() * (gear_r - 2.5 * scale);
        let p1y = gear_cy + a.sin() * (gear_r - 2.5 * scale);
        let p2x = gear_cx + a.cos() * (gear_r + 6.0 * scale);
        let p2y = gear_cy + a.sin() * (gear_r + 6.0 * scale);
        canvas.stroke_line_fine(p1x, p1y, p2x, p2y, gold_line, 1.8 * scale);
    }

    // Ojo de rubí con haz de dispersión óptica
    let eye_x = 75.0 * scale * 1.5;
    let eye_y = -12.0 * scale * 1.5;
    canvas.fill_circle(eye_x, eye_y, 9.0 * scale, ruby_red);
    canvas.stroke_circle(eye_x, eye_y, 18.0 * scale, gold_line, 1.5 * scale);
    for r in 0..8 {
        let a = (r as f32 / 8.0) * PI * 2.0 + tau * 3.0;
        canvas.stroke_line_fine(eye_x, eye_y, eye_x + a.cos() * 45.0 * scale, eye_y + a.sin() * 45.0 * scale, ruby_red.with_alpha(0.6), 1.4 * scale);
    }

    // PANEL SUPERIOR DE DESPIECE ANATÓMICO (EXPLODED VERTEBRA VIEW)
    let top_panel_y = -340.0;
    canvas.stroke_rect(-320.0, top_panel_y - 80.0, 640.0, 150.0, faint_grid, 1.0);
    draw_vector_text(canvas, -300.0, top_panel_y - 92.0, "FIG. 2B: EXPLODED VERTEBRAL HINGE (0.35 mm PIN)", 11.0, gold_line, false);
    // Eslabón despiezado
    canvas.stroke_circle(-120.0, top_panel_y, 35.0, gold_line, 2.0);
    canvas.stroke_circle(120.0, top_panel_y, 35.0, gold_line, 2.0);
    canvas.stroke_line_fine(-85.0, top_panel_y, 85.0, top_panel_y, cyan_glow, 3.0); // Pasador
    canvas.fill_circle(0.0, top_panel_y, 7.0, ruby_red);
    draw_dimension_bracket(canvas, -85.0, top_panel_y + 45.0, 85.0, top_panel_y + 45.0, "0.35 mm GOLD ALLOY PIN", Color::hex("#C8C1EF"), 14.0);

    // PANEL INFERIOR DE MICROMECÁNICA HOROLÓGICA (BALANCE & ESCAPEMENT)
    let bot_panel_y = 350.0;
    canvas.stroke_rect(-320.0, bot_panel_y - 75.0, 640.0, 150.0, faint_grid, 1.0);
    draw_vector_text(canvas, -300.0, bot_panel_y - 88.0, "FIG. 3A: ANCHOR ESCAPEMENT & SPIRAL HAIRSPRING", 11.0, cyan_glow, false);
    // Rueda de volante con espiral
    let bal_r = 45.0;
    canvas.stroke_circle(0.0, bot_panel_y, bal_r, gold_line, 2.2);
    for sp in 0..30 {
        let a = (sp as f32) * 0.35 + tau * 6.0 * PI;
        let r = 5.0 + (sp as f32) * 1.2;
        let px = a.cos() * r;
        let py = bot_panel_y + a.sin() * r;
        if sp == 0 {
            canvas.fill_circle(px, py, 3.0, cyan_glow);
        }
    }
    // Paleta de áncora
    canvas.stroke_line_fine(-60.0, bot_panel_y - 20.0, -35.0, bot_panel_y - 40.0, ruby_red, 3.0);
    canvas.stroke_line_fine(60.0, bot_panel_y - 20.0, 35.0, bot_panel_y - 40.0, ruby_red, 3.0);

    canvas.restore(); // Termina BlendMode::Screen

    // Lupa técnica 10X enfocando la orfebrería de las escamas
    let loupe_x = -240.0 * scale;
    let loupe_y = -120.0 * scale;
    draw_magnifying_loupe(canvas, loupe_x, loupe_y, 75.0 * scale, "ARTICULATION 10X", Color::hex("#FFD700"));
    canvas.stroke_circle(loupe_x - 14.0 * scale, loupe_y, 28.0 * scale, Color::hex("#FFD700"), 2.0 * scale);
    canvas.stroke_circle(loupe_x + 14.0 * scale, loupe_y, 28.0 * scale, Color::hex("#00F5D4"), 2.0 * scale);
    canvas.fill_circle(loupe_x, loupe_y, 5.0 * scale, Color::hex("#FF0055"));

    // Cotas técnicas
    draw_dimension_bracket(
        canvas,
        -spine_len * 0.9,
        90.0 * scale,
        spine_len * 0.9,
        90.0 * scale,
        "74 mm • 24 VERTEBRAL JOINTS",
        Color::hex("#C8C1EF"),
        26.0 * scale,
    );

    // Callouts esquemáticos
    draw_node_callout(canvas, eye_x, eye_y - 25.0 * scale, "FACETED RUBY OCULUS", "PIGEON BLOOD • 0.35 ct", Color::hex("#FFDF70"), true);
    draw_node_callout(canvas, gear_cx, gear_cy + 40.0 * scale, "SPRING ESCAPEMENT", "SWISS BRASS BALANCE WHEEL", Color::hex("#00F5D4"), false);

    canvas.restore();
}

/// 2. Fanfarria de Trompetas Heráldicas en Rayos X Acústicos
/// Dos trompetas colosales de 900px que cruzan el encuadre diagonalmente con corte resonador y frentes de onda en Hz.
pub fn draw_xray_trompetas_acustica(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    width: f32,
    tau: f32,
) {
    let gold = Color::hex("#FFD700");
    let amber = Color::hex("#E5A020");
    let wave_cyan = Color::hex("#00E5FF");
    let banner_crimson = Color::hex("#A81C28");

    // Trompetas cruzadas heráldicas a pantalla completa
    for &side in &[-1.0_f32, 1.0_f32] {
        canvas.save();
        canvas.translate(cx, cy);
        canvas.scale(side, 1.0);
        canvas.rotate(0.42); // Ángulo diagonal

        let len = width * 0.48;

        // Pabellón de la campana acústica (flaring bell)
        let bell_x = len;
        let mut bell_pb = tiny_skia::PathBuilder::new();
        bell_pb.move_to(bell_x - 90.0, -18.0);
        bell_pb.cubic_to(bell_x - 35.0, -25.0, bell_x, -75.0, bell_x + 35.0, -95.0);
        bell_pb.line_to(bell_x + 35.0, 95.0);
        bell_pb.cubic_to(bell_x, 75.0, bell_x - 35.0, 25.0, bell_x - 90.0, 18.0);
        bell_pb.close();
        if let Some(path) = bell_pb.finish() {
            canvas.fill_path(&path, gold.with_alpha(0.85));
            canvas.stroke_path_fine(&path, amber, 2.5);
        }

        // Tubo cónico principal de latón de 650px
        let mut tube_pb = tiny_skia::PathBuilder::new();
        tube_pb.move_to(-len * 0.7, -7.0);
        tube_pb.line_to(bell_x - 90.0, -18.0);
        tube_pb.line_to(bell_x - 90.0, 18.0);
        tube_pb.line_to(-len * 0.7, 7.0);
        tube_pb.close();
        if let Some(path) = tube_pb.finish() {
            canvas.fill_path(&path, gold);
            canvas.stroke_path_fine(&path, amber, 2.2);
        }

        // Boquilla de plata tallada
        canvas.stroke_line_fine(-len * 0.7, -12.0, -len * 0.7, 12.0, Color::hex("#EFEFEF"), 3.5);

        // Bloque de pistones y válvulas en el centro del tubo
        for p in 0..3 {
            let px = -len * 0.2 + (p as f32) * 32.0;
            canvas.fill_rect(px - 9.0, -32.0, 18.0, 64.0, Color::hex("#F2E090"));
            canvas.stroke_rect(px - 9.0, -32.0, 18.0, 64.0, amber, 1.8);
            // Botón superior del pistón
            canvas.fill_circle(px, -36.0, 7.0, Color::hex("#FFF0AA"));
            canvas.stroke_circle(px, -36.0, 7.0, amber, 1.4);
        }

        // Estandarte carmesí colgante con bordado heráldico colonial
        let mut banner_pb = tiny_skia::PathBuilder::new();
        banner_pb.move_to(-len * 0.4, 15.0);
        banner_pb.line_to(-len * 0.05, 15.0);
        let wave_b = (tau * 8.0 * PI).sin() * 12.0;
        banner_pb.cubic_to(-len * 0.05 + 15.0, 140.0 + wave_b, -len * 0.2, 220.0 + wave_b, -len * 0.22, 260.0);
        banner_pb.cubic_to(-len * 0.35, 210.0 + wave_b, -len * 0.4 - 15.0, 140.0 + wave_b, -len * 0.4, 15.0);
        banner_pb.close();
        if let Some(b_path) = banner_pb.finish() {
            canvas.fill_path(&b_path, banner_crimson);
            canvas.stroke_path_fine(&b_path, gold, 2.0);
            // Flecos dorados
            for f in 0..12 {
                let fx = -len * 0.38 + (f as f32) * 14.0;
                canvas.stroke_line_fine(fx, 210.0 + wave_b, fx, 235.0 + wave_b, gold, 1.5);
            }
        }

        canvas.restore();
    }

    // Rayos X Acústicos: Ondas esféricas de presión armónica concéntricas en BlendMode::Screen
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);
    let wave_rings = 7;
    for w in 0..wave_rings {
        let phase = (tau * 2.5 + w as f32 / wave_rings as f32) % 1.0;
        let rad = 140.0 + phase * 680.0;
        let alpha = (1.0 - phase) * 0.75;
        canvas.stroke_circle(cx, cy, rad, wave_cyan.with_alpha(alpha), 2.2);
        canvas.stroke_circle(cx, cy, rad * 0.96, gold.with_alpha(alpha * 0.6), 1.2);
    }
    canvas.restore();

    // Callouts técnicos y cotas de frecuencia acústica
    draw_node_callout(canvas, cx + width * 0.35, cy - 140.0, "HERALDIC CLARION RESONANCE", "HARMONIC FORMANT • 440 Hz (A4)", Color::hex("#FFDF70"), true);
    draw_node_callout(canvas, cx - width * 0.35, cy - 140.0, "ACOUSTIC PRESSURE WAVE", "WAVELENGTH λ = 0.77 m", Color::hex("#00E5FF"), false);
}

/// 3. Bloque de Hielo en Rayos X: Red Cristalina Molecular Fractal Hexagonal
/// El asombro primordial de José Arcadio Buendía: "Es el diamante más grande del mundo".
pub fn draw_xray_hielo_cristalino(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let ice_cyan = Color::hex("#B0F5FF");
    let ice_deep = Color::hex("#12385A").with_alpha(0.85);
    let neon_blue = Color::hex("#00BFFF");
    let ruler_col = Color::hex("#80D0FF");
    let prism_spectrum = [
        Color::hex("#FF3366"),
        Color::hex("#FF9900"),
        Color::hex("#FFDD00"),
        Color::hex("#33FF99"),
        Color::hex("#00CCFF"),
        Color::hex("#6666FF"),
        Color::hex("#CC33FF"),
    ];

    canvas.save();
    canvas.translate(cx, cy);

    // Escalas de temperatura criogénica en los márgenes laterales
    for side in [-480.0, 480.0] {
        canvas.stroke_line_fine(side, -550.0, side, 550.0, ruler_col.with_alpha(0.6), 1.5);
        for tick in 0..=22 {
            let ty = -550.0 + (tick as f32) * 50.0;
            let tick_len = if tick % 2 == 0 { 16.0 } else { 8.0 };
            let dir = if side < 0.0 { 1.0 } else { -1.0 };
            canvas.stroke_line_fine(side, ty, side + dir * tick_len, ty, ruler_col.with_alpha(0.7), 1.2);
            if tick % 2 == 0 {
                let temp_val = 100 - (tick as i32) * 17;
                let val_str = format!("{}°C", temp_val);
                draw_vector_text(canvas, side + dir * 32.0, ty + 4.0, &val_str, 9.0, ruler_col.with_alpha(0.8), true);
            }
        }
    }

    let hw = 400.0 * scale;
    let hh = 330.0 * scale;

    // Cara frontal del cubo de hielo transparente con facetas cinceladas
    let mut ice_pb = tiny_skia::PathBuilder::new();
    ice_pb.move_to(-hw, -hh);
    ice_pb.line_to(hw - 40.0 * scale, -hh + 30.0 * scale);
    ice_pb.line_to(hw, hh);
    ice_pb.line_to(-hw + 30.0 * scale, hh - 20.0 * scale);
    ice_pb.close();
    if let Some(path) = ice_pb.finish() {
        canvas.fill_path(&path, ice_deep);
        canvas.stroke_path_fine(&path, ice_cyan, 3.2 * scale);
    }

    // Cara superior en perspectiva isométrica
    let mut top_pb = tiny_skia::PathBuilder::new();
    top_pb.move_to(-hw, -hh);
    top_pb.line_to(-hw + 70.0 * scale, -hh - 90.0 * scale);
    top_pb.line_to(hw + 25.0 * scale, -hh - 55.0 * scale);
    top_pb.line_to(hw - 40.0 * scale, -hh + 30.0 * scale);
    top_pb.close();
    if let Some(tpath) = top_pb.finish() {
        canvas.fill_path(&tpath, Color::hex("#285B82").with_alpha(0.70));
        canvas.stroke_path_fine(&tpath, ice_cyan, 2.5 * scale);
    }

    // Cara lateral derecha
    let mut side_pb = tiny_skia::PathBuilder::new();
    side_pb.move_to(hw - 40.0 * scale, -hh + 30.0 * scale);
    side_pb.line_to(hw + 25.0 * scale, -hh - 55.0 * scale);
    side_pb.line_to(hw + 65.0 * scale, hh - 50.0 * scale);
    side_pb.line_to(hw, hh);
    side_pb.close();
    if let Some(spath) = side_pb.finish() {
        canvas.fill_path(&spath, Color::hex("#0D243B").with_alpha(0.85));
        canvas.stroke_path_fine(&spath, ice_cyan, 2.4 * scale);
    }

    // Rayos X de la estructura cristalina molecular (Ice Ih: Red hexagonal de enlaces de hidrógeno)
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);

    let hex_rows = 6;
    let hex_cols = 7;
    let hex_r = 44.0 * scale;
    for r in 0..hex_rows {
        let gy = -hh * 0.70 + (r as f32) * hex_r * 1.55;
        for c in 0..hex_cols {
            let offset_x = if r % 2 == 1 { hex_r * 0.866 } else { 0.0 };
            let gx = -hw * 0.75 + (c as f32) * hex_r * 1.732 + offset_x;

            if gx < hw * 0.80 && gy < hh * 0.80 {
                let mut h_pb = tiny_skia::PathBuilder::new();
                for i in 0..6 {
                    let a = (i as f32 / 6.0) * PI * 2.0;
                    let px = gx + a.cos() * hex_r;
                    let py = gy + a.sin() * hex_r;
                    if i == 0 { h_pb.move_to(px, py); } else { h_pb.line_to(px, py); }
                }
                h_pb.close();
                if let Some(hp) = h_pb.finish() {
                    canvas.stroke_path_fine(&hp, neon_blue.with_alpha(0.65), 1.3 * scale);
                }

                // Átomos de oxígeno
                for i in 0..6 {
                    let a = (i as f32 / 6.0) * PI * 2.0;
                    let px = gx + a.cos() * hex_r;
                    let py = gy + a.sin() * hex_r;
                    canvas.fill_circle(px, py, 3.2 * scale, ice_cyan);
                }
            }
        }
    }

    // Haz de luz blanca incidente desde el extremo izquierdo de la pantalla
    canvas.stroke_line_fine(-500.0, -hh * 0.25, -hw * 0.85, -hh * 0.2, Color::WHITE.with_alpha(0.9), 3.5 * scale);

    // Refracción prismática espectral que cruza el bloque y sale por la derecha hasta el borde
    let ray_start_x = -hw * 0.85;
    let ray_start_y = -hh * 0.2;
    for (i, &col) in prism_spectrum.iter().enumerate() {
        let a = 0.16 + (i as f32) * 0.05 + (tau * 2.0 * PI).sin() * 0.02;
        let rx2 = 500.0;
        let ry2 = ray_start_y + (rx2 - ray_start_x) * a.tan();
        canvas.stroke_line_fine(ray_start_x, ray_start_y, rx2, ry2, col.with_alpha(0.85), 2.8 * scale);
    }

    // APARATO ALQUÍMICO INFERIOR (RETORTA, CONDENSADOR EN ESPIRAL Y MATRAZ)
    let chem_y = hh + 80.0 * scale;
    // Matraz receptor esférico con líquido alquímico
    canvas.stroke_circle(0.0, chem_y, 45.0 * scale, Color::hex("#80D0FF"), 2.0 * scale);
    canvas.fill_circle(0.0, chem_y + 12.0 * scale, 30.0 * scale, Color::hex("#FFD700").with_alpha(0.75));
    // Tubo en espiral de condensación
    for t in 0..25 {
        let a = (t as f32) * 0.6 + tau * 6.0 * PI;
        let py = chem_y - 80.0 * scale + (t as f32) * 3.0 * scale;
        let px = a.sin() * 15.0 * scale;
        canvas.fill_circle(px, py, 2.5 * scale, ice_cyan);
    }

    canvas.restore(); // Termina BlendMode::Screen

    // Lupa microscópica 50X enfocando la celda unitaria de agua sólida
    let loupe_x = hw * 0.85;
    let loupe_y = -hh * 0.75;
    draw_magnifying_loupe(canvas, loupe_x, loupe_y, 75.0 * scale, "ICE Ih LATTICE 50X", ice_cyan);
    canvas.fill_circle(loupe_x, loupe_y, 14.0 * scale, Color::hex("#FF2A55"));
    canvas.fill_circle(loupe_x - 22.0 * scale, loupe_y - 18.0 * scale, 8.0 * scale, Color::hex("#FFFFFF"));
    canvas.fill_circle(loupe_x + 22.0 * scale, loupe_y - 18.0 * scale, 8.0 * scale, Color::hex("#FFFFFF"));
    canvas.stroke_line_fine(loupe_x, loupe_y, loupe_x - 22.0 * scale, loupe_y - 18.0 * scale, ice_cyan, 2.2 * scale);
    canvas.stroke_line_fine(loupe_x, loupe_y, loupe_x + 22.0 * scale, loupe_y - 18.0 * scale, ice_cyan, 2.2 * scale);

    // Cotas de ingeniería y llamadas
    draw_dimension_bracket(canvas, -hw, hh + 20.0 * scale, hw, hh + 20.0 * scale, "MONOLITHIC BLOCK • 800 mm", ice_cyan, 20.0 * scale);
    draw_node_callout(canvas, 0.0, -hh - 65.0 * scale, "THERMAL EQUILIBRIUM", "T = 273.15 K (0.00 °C)", ice_cyan, true);

    canvas.restore();
}

/// 4. Plano Naval y Corte de Rayos X del Galeón Español Encallado en la Selva
/// Corte longitudinal a pantalla completa: quilla de roble, 3 cubiertas, cuadernas entrelazadas con orquídeas.
pub fn draw_xray_galeon_naval(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
    tau: f32,
) {
    let timber_dark = Color::hex("#2B1E16");
    let timber_mid = Color::hex("#5A3E2C");
    let timber_line = Color::hex("#C49A6C");
    let foliage_deep = Color::hex("#152C16");
    let orchid_magenta = Color::hex("#E83182");
    let gold_cross = Color::hex("#E5BE38");

    // Fondo selvático denso con grandes helechos que cubren la pantalla
    let tree_cols = 5;
    for c in 0..tree_cols {
        let tx = (c as f32 / (tree_cols - 1) as f32) * width;
        canvas.stroke_line_fine(tx, 0.0, tx, height, foliage_deep.with_alpha(0.35), 45.0);
    }

    canvas.save();
    canvas.translate(cx, cy + 80.0);

    let ship_len = width * 0.44;

    // 1. Quilla y casco del galeón colonial español
    let mut hull_pb = tiny_skia::PathBuilder::new();
    hull_pb.move_to(-ship_len, -60.0);
    hull_pb.cubic_to(-ship_len * 0.6, 120.0, ship_len * 0.5, 140.0, ship_len, 40.0);
    hull_pb.line_to(ship_len - 30.0, -110.0);
    hull_pb.line_to(-ship_len + 40.0, -90.0);
    hull_pb.close();
    if let Some(hpath) = hull_pb.finish() {
        canvas.fill_path(&hpath, timber_dark);
        canvas.stroke_path_fine(&hpath, timber_line, 2.5);
    }

    // 2. Corte de Rayos X naval: Las 3 cubiertas interiores (Bodega, Entrepuente, Castillo)
    let decks = [-70.0, -15.0, 45.0];
    for &dy in &decks {
        canvas.stroke_line_fine(-ship_len * 0.85, dy, ship_len * 0.82, dy, timber_line.with_alpha(0.9), 2.2);
    }

    // Cuadernas y costillares de roble naval (Ribs)
    let rib_count = 18;
    for r in 0..rib_count {
        let rx = -ship_len * 0.85 + (r as f32 / (rib_count - 1) as f32) * (ship_len * 1.65);
        let mut r_pb = tiny_skia::PathBuilder::new();
        r_pb.move_to(rx, -85.0);
        r_pb.cubic_to(rx + 15.0, 20.0, rx - 10.0, 80.0, rx - 20.0, 120.0);
        if let Some(rp) = r_pb.finish() {
            canvas.stroke_path_fine(&rp, timber_line.with_alpha(0.70), 1.8);
        }
    }

    // Cañones de bronce en el entrepuente
    for g in 0..5 {
        let gx = -ship_len * 0.55 + (g as f32) * 65.0;
        canvas.fill_rect(gx, -24.0, 32.0, 9.0, Color::hex("#C48820"));
        canvas.stroke_rect(gx, -24.0, 32.0, 9.0, Color::hex("#FFEAA0"), 1.2);
    }

    // 3. Los tres mástiles gigantescos que suben hacia la copa de los árboles (Trinquete, Mayor, Mesana)
    let masts = [
        (-ship_len * 0.45, -420.0, 14.0_f32),
        (0.0, -560.0, 18.0),
        (ship_len * 0.40, -480.0, 12.0),
    ];
    for &(mx, top_y, mw) in &masts {
        canvas.stroke_line_fine(mx, -80.0, mx, top_y, timber_mid, mw);
        canvas.stroke_line_fine(mx, -80.0, mx, top_y, timber_line, 2.0);

        // Vergas cruzadas horizontales
        let yard_y = top_y + 120.0;
        canvas.stroke_line_fine(mx - 130.0, yard_y, mx + 130.0, yard_y, timber_mid, 7.0);
        canvas.stroke_line_fine(mx - 130.0, yard_y, mx + 130.0, yard_y, timber_line, 1.4);

        // Cordaje de jarcias cubierto de enredaderas
        for s in -3..=3 {
            let sx = mx + (s as f32) * 35.0;
            canvas.stroke_line_fine(mx, top_y, sx, -80.0, timber_line.with_alpha(0.4), 1.0);
        }
    }

    // Lianas gigantes y orquídeas parásitas floreciendo sobre el maderamen
    for o in 0..14 {
        let t = o as f32 / 13.0;
        let ox = -ship_len * 0.7 + t * (ship_len * 1.5);
        let oy = -220.0 + (t * 5.0 * PI + tau * 2.0).sin() * 80.0;
        canvas.fill_circle(ox, oy, 7.0, orchid_magenta);
        canvas.stroke_circle(ox, oy, 7.0, Color::hex("#FFAAE0"), 1.2);
    }

    // Mascarón de proa en dorado: Un arcángel con espada
    canvas.fill_circle(ship_len - 15.0, 25.0, 14.0, gold_cross);
    canvas.stroke_circle(ship_len - 15.0, 25.0, 14.0, Color::hex("#FFFFFF"), 1.8);

    // Cotas de arquitectura naval
    draw_dimension_bracket(canvas, -ship_len, 165.0, ship_len, 165.0, "GALEON REAL • ESLORA 54 METROS", timber_line, 24.0);
    draw_node_callout(canvas, 0.0, -570.0, "PALO MAYOR CUBIERTO DE HELECHOS", "PINUS SYLVESTRIS • ALTURA 42 m", timber_line, true);
    draw_node_callout(canvas, -ship_len * 0.5, 55.0, "CUADERNA DE ROBLE COLONIAL", "QUILLA ENCALLADA A 12 KM DEL MAR", timber_line, false);

    canvas.restore();
}

/// 5. Mapa Cartográfico Histórico Colonial a Pantalla Completa:
/// "CARTA GEOGRÁFICA DE LA CIÉNAGA GRANDE, MACONDO Y LA RUTA DE LOS GALEONES"
pub fn draw_mapa_cartografico_macondo(
    canvas: &mut Canvas,
    width: f32,
    height: f32,
    _tau: f32,
) {
    let ink_dark = Color::hex("#2B1C10");
    let ink_sepia = Color::hex("#7A5538");
    let water_blue = Color::hex("#285568").with_alpha(0.40);
    let sea_foam = Color::hex("#A8D8E8");
    let compass_gold = Color::hex("#D4AF37");

    let margin = 50.0;

    // 1. Marco perimetral doble cartográfico graduado
    canvas.stroke_rect(margin, margin, width - margin * 2.0, height - margin * 2.0, ink_dark, 2.8);
    canvas.stroke_rect(margin + 12.0, margin + 12.0, width - (margin + 12.0) * 2.0, height - (margin + 12.0) * 2.0, ink_sepia, 1.2);

    // Marcas de latitud y longitud en los bordes
    let mut my = margin + 12.0;
    let mut deg_y = 10;
    while my <= height - margin - 12.0 {
        canvas.stroke_line_fine(margin, my, margin + 12.0, my, ink_dark, 1.5);
        canvas.stroke_line_fine(width - margin - 12.0, my, width - margin, my, ink_dark, 1.5);
        if deg_y % 2 == 0 {
            draw_vector_text(canvas, margin + 18.0, my - 4.0, &format!("10°{:02}' N", deg_y), 8.5, ink_sepia, false);
        }
        my += 45.0;
        deg_y += 1;
    }

    // 2. Líneas loxodrómicas de navegación náutica que cruzan el mapa
    let center_x = width * 0.5;
    let center_y = height * 0.48;
    for i in 0..16 {
        let a = (i as f32 / 16.0) * PI * 2.0;
        canvas.stroke_line_fine(center_x, center_y, center_x + a.cos() * 900.0, center_y + a.sin() * 900.0, ink_sepia.with_alpha(0.25), 0.9);
    }

    // 3. Línea costera de la Ciénaga Grande y el Mar Caribe
    let mut coast_pb = tiny_skia::PathBuilder::new();
    coast_pb.move_to(margin + 20.0, height * 0.22);
    coast_pb.cubic_to(width * 0.35, height * 0.28, width * 0.45, height * 0.42, width * 0.28, height * 0.65);
    coast_pb.cubic_to(width * 0.20, height * 0.78, width * 0.38, height * 0.88, width - margin - 20.0, height * 0.85);
    coast_pb.line_to(width - margin - 20.0, height - margin - 20.0);
    coast_pb.line_to(margin + 20.0, height - margin - 20.0);
    coast_pb.close();
    if let Some(cp) = coast_pb.finish() {
        canvas.fill_path(&cp, water_blue);
        canvas.stroke_path_fine(&cp, ink_dark, 2.2);
    }

    // Ondas de agua marina rayadas a mano
    for w in 0..18 {
        let wy = height * 0.55 + (w as f32) * 22.0;
        let wx = width * 0.45 + ((w as f32 * 0.8).sin()) * 60.0;
        canvas.stroke_line_fine(wx, wy, wx + 90.0, wy, sea_foam.with_alpha(0.65), 1.2);
    }

    // Cordillera de la Sierra Nevada con sombreado achurado
    for m in 0..8 {
        let mx = width * 0.55 + (m as f32) * 45.0;
        let my = height * 0.24 - ((m as f32 * 0.9).sin()) * 40.0;
        // Pico triangular
        let mut peak_pb = tiny_skia::PathBuilder::new();
        peak_pb.move_to(mx - 40.0, my + 60.0);
        peak_pb.line_to(mx, my);
        peak_pb.line_to(mx + 40.0, my + 60.0);
        peak_pb.close();
        if let Some(pp) = peak_pb.finish() {
            canvas.fill_path(&pp, Color::hex("#EFE8DA"));
            canvas.stroke_path_fine(&pp, ink_dark, 1.8);
            // Sombreado a la derecha
            canvas.stroke_line_fine(mx, my, mx + 20.0, my + 60.0, ink_sepia, 1.0);
        }
    }

    // Emplazamiento de Macondo marcado con una cruz y sol dorado
    let macondo_x = width * 0.52;
    let macondo_y = height * 0.44;
    canvas.fill_circle(macondo_x, macondo_y, 8.0, Color::hex("#D91438"));
    canvas.stroke_circle(macondo_x, macondo_y, 18.0, compass_gold, 2.0);
    draw_vector_text(canvas, macondo_x + 28.0, macondo_y - 6.0, "MACONDO (FUNDADO 1820)", 13.0, ink_dark, false);

    // Emplazamiento del Galeón Encallado
    let galleon_x = width * 0.68;
    let galleon_y = height * 0.62;
    canvas.fill_circle(galleon_x, galleon_y, 6.0, compass_gold);
    canvas.stroke_circle(galleon_x, galleon_y, 14.0, ink_dark, 1.4);
    draw_vector_text(canvas, galleon_x + 22.0, galleon_y - 5.0, "SITIO DEL GALEON ENCALLADO", 11.0, ink_sepia, false);

    // 4. Gran Rosa de los Vientos de 16 puntas en el tercio superior
    let compass_x = width * 0.28;
    let compass_y = height * 0.20;
    let cr = 75.0;
    canvas.stroke_circle(compass_x, compass_y, cr, ink_dark, 2.0);
    canvas.stroke_circle(compass_x, compass_y, cr * 0.55, ink_sepia, 1.0);
    for p in 0..8 {
        let a = (p as f32 / 8.0) * PI * 2.0;
        let is_major = p % 2 == 0;
        let len = if is_major { cr + 15.0 } else { cr * 0.85 };
        let col = if is_major { compass_gold } else { ink_dark };
        canvas.stroke_line_fine(compass_x, compass_y, compass_x + a.cos() * len, compass_y + a.sin() * len, col, if is_major { 2.2 } else { 1.2 });
    }
    draw_vector_text(canvas, compass_x, compass_y - cr - 25.0, "N", 16.0, ink_dark, true);

    // Monstruo marino mítico nadando en la ciénaga
    let sea_x = width * 0.72;
    let sea_y = height * 0.76;
    let mut sea_pb = tiny_skia::PathBuilder::new();
    sea_pb.move_to(sea_x - 60.0, sea_y);
    sea_pb.cubic_to(sea_x - 30.0, sea_y - 25.0, sea_x + 10.0, sea_y + 25.0, sea_x + 50.0, sea_y);
    if let Some(sp) = sea_pb.finish() {
        canvas.stroke_path_fine(&sp, ink_dark, 2.5);
    }

    // Cartela cartográfica ornamental barroca arriba
    draw_blueprint_header(canvas, width, "CARTA GEOGRAPHICA DEL TERRITORIO DE MACONDO", "EXPLICATIO LOCORUM • EXPEDITION DE GABRIEL");
}

/// 6. Placa Botánica Monumental del Castaño Ancestral a Pantalla Completa
/// Raíces masivas abajo, tronco centenario con cadenas reales, copa colosal arriba y remolinos de recuerdos.
pub fn draw_castano_monumental_plate(
    canvas: &mut Canvas,
    cx: f32,
    width: f32,
    height: f32,
    tau: f32,
    _seed: u32,
) {
    let bark_deep = Color::hex("#1C130B");
    let bark_mid = Color::hex("#3F2B1D");
    let bark_line = Color::hex("#8A684C");
    let canopy_dark = Color::hex("#1B3315");
    let canopy_light = Color::hex("#3F6B2B");
    let chain_iron = Color::hex("#7E8794");
    let flower_gold = Color::hex("#FFD700");

    let trunk_top_y = height * 0.28;
    let trunk_bottom_y = height * 0.82;
    let trunk_w = width * 0.30;

    // 1. Raíces nudosas colosales que se agarran al suelo en el tercio inferior
    for r in -4..=4 {
        let rx = cx + (r as f32) * 55.0;
        let mut root_pb = tiny_skia::PathBuilder::new();
        root_pb.move_to(rx, trunk_bottom_y - 60.0);
        root_pb.cubic_to(rx + (r as f32) * 45.0, trunk_bottom_y + 80.0, rx + (r as f32) * 90.0, height - 120.0, rx + (r as f32) * 110.0, height - 60.0);
        if let Some(rp) = root_pb.finish() {
            canvas.stroke_path_fine(&rp, bark_deep, 28.0);
            canvas.stroke_path_fine(&rp, bark_line, 2.0);
        }
    }

    // 2. Tronco masivo de roble/castaño ancestral que llena el centro
    let mut trunk_pb = tiny_skia::PathBuilder::new();
    trunk_pb.move_to(cx - trunk_w * 0.45, trunk_top_y);
    trunk_pb.cubic_to(cx - trunk_w * 0.60, height * 0.45, cx - trunk_w * 0.75, height * 0.70, cx - trunk_w * 0.90, trunk_bottom_y);
    trunk_pb.line_to(cx + trunk_w * 0.90, trunk_bottom_y);
    trunk_pb.cubic_to(cx + trunk_w * 0.75, height * 0.70, cx + trunk_w * 0.60, height * 0.45, cx + trunk_w * 0.45, trunk_top_y);
    trunk_pb.close();
    if let Some(tp) = trunk_pb.finish() {
        canvas.fill_path(&tp, bark_mid);
        canvas.stroke_path_fine(&tp, bark_deep, 3.5);

        // Estrías profundas de corteza centenaria
        for s in 0..16 {
            let sx = cx - trunk_w * 0.70 + (s as f32) * (trunk_w * 1.4 / 15.0);
            let mut s_pb = tiny_skia::PathBuilder::new();
            s_pb.move_to(sx, trunk_top_y);
            s_pb.cubic_to(sx + 15.0, height * 0.45, sx - 12.0, height * 0.65, sx + 20.0, trunk_bottom_y);
            if let Some(sp) = s_pb.finish() {
                canvas.stroke_path_fine(&sp, bark_line, 1.4);
            }
        }
    }

    // 3. Gran copa frondosa que cubre todo el tercio superior de 1080px
    let cluster_count = 11;
    for c in 0..cluster_count {
        let angle = (c as f32 / cluster_count as f32) * PI;
        let c_rad = width * 0.42;
        let clx = cx + angle.cos() * c_rad;
        let cly = trunk_top_y - 20.0 - angle.sin() * 120.0;
        canvas.fill_circle(clx, cly, 110.0, canopy_dark);
        canvas.fill_circle(clx, cly, 90.0, canopy_light);
        canvas.stroke_circle(clx, cly, 110.0, bark_deep, 2.2);
    }

    // 4. Don José Arcadio Buendía sentado al pie del árbol
    let arcadio_y = trunk_bottom_y - 40.0;
    // Túnica blanca sucia de ermitaño
    canvas.fill_circle(cx, arcadio_y - 50.0, 32.0, Color::hex("#E8E2D5"));
    canvas.stroke_circle(cx, arcadio_y - 50.0, 32.0, bark_deep, 2.0);
    // Cabeza y larga barba blanca bíblica
    canvas.fill_circle(cx, arcadio_y - 95.0, 22.0, Color::hex("#DBCFBA"));
    canvas.stroke_circle(cx, arcadio_y - 95.0, 22.0, bark_deep, 1.8);
    // Barba fluyente
    let mut beard_pb = tiny_skia::PathBuilder::new();
    beard_pb.move_to(cx - 15.0, arcadio_y - 85.0);
    beard_pb.cubic_to(cx - 20.0, arcadio_y - 30.0, cx + 20.0, arcadio_y - 30.0, cx + 15.0, arcadio_y - 85.0);
    beard_pb.close();
    if let Some(bp) = beard_pb.finish() {
        canvas.fill_path(&bp, Color::hex("#F2EFE8"));
        canvas.stroke_path_fine(&bp, Color::hex("#9A958A"), 1.2);
    }

    // 5. Cadenas de hierro forjado reales que lo atan al tronco
    for c in 0..8 {
        let cy_link = arcadio_y - 80.0 + (c as f32) * 16.0;
        canvas.stroke_line_fine(cx - trunk_w * 0.55, cy_link - 15.0, cx - 18.0, cy_link, chain_iron, 4.5);
        canvas.stroke_line_fine(cx + 18.0, cy_link, cx + trunk_w * 0.55, cy_link - 15.0, chain_iron, 4.5);
    }

    // 6. Remolino matemático de recuerdos: Espiral áurea de Arquímedes con ecuaciones
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);
    let spiral_pts = 60;
    let mut sp_pb = tiny_skia::PathBuilder::new();
    for p in 0..spiral_pts {
        let theta = tau * 4.0 * PI + (p as f32) * 0.22;
        let r = 70.0 + (p as f32) * 6.5;
        let px = cx + theta.cos() * r;
        let py = (height * 0.46) + theta.sin() * (r * 0.70);
        if p == 0 { sp_pb.move_to(px, py); } else { sp_pb.line_to(px, py); }
    }
    if let Some(spath) = sp_pb.finish() {
        canvas.stroke_path_fine(&spath, flower_gold.with_alpha(0.65), 2.2);
    }
    canvas.restore();

    // Lluvia de florecitas amarillas que caen lentamente
    for f in 0..16 {
        let ft = (tau * 1.8 + (f as f32) * 0.08) % 1.0;
        let fx = cx - width * 0.40 + (f as f32) * 55.0;
        let fy = trunk_top_y + ft * (height * 0.65);
        canvas.fill_circle(fx, fy, 4.5, flower_gold);
        canvas.stroke_circle(fx, fy, 4.5, Color::hex("#FFFFFF"), 1.0);
    }
}

/// 7. Ascensión Monumental de Remedios la Bella a Pantalla Completa
/// Sábanas gigantescas de bramante blanco de lado a lado, arcos celestes de 600px, rosas y violines de orquesta.
pub fn draw_remedios_ascension_monumental(
    canvas: &mut Canvas,
    cx: f32,
    ascend_y: f32,
    width: f32,
    tau: f32,
) {
    let sheet_white = Color::hex("#FCFAF2");
    let sheet_shade = Color::hex("#D9D2C3");
    let celestial_gold = Color::hex("#F5CA38");
    let rose_carmine = Color::hex("#D61546");

    // 1. Arcos astronómicos celestes concéntricos de 600px de diámetro con marcas de cuadrante
    let halo_r = 320.0;
    canvas.stroke_circle(cx, ascend_y - 60.0, halo_r, celestial_gold.with_alpha(0.60), 2.0);
    canvas.stroke_circle(cx, ascend_y - 60.0, halo_r * 0.85, celestial_gold.with_alpha(0.35), 1.2);
    for i in 0..32 {
        let a = (i as f32 / 32.0) * PI * 2.0 + tau * 0.3;
        let r1 = halo_r - 12.0;
        let r2 = halo_r + 12.0;
        canvas.stroke_line_fine(cx + a.cos() * r1, (ascend_y - 60.0) + a.sin() * r1, cx + a.cos() * r2, (ascend_y - 60.0) + a.sin() * r2, celestial_gold, 1.4);
    }

    // 2. Sábanas monumentales de bramante blanco ondeando de borde a borde (ancho completo de 1080px)
    for s in 0..4 {
        let sy_offset = (s as f32) * 55.0;
        let wave_phase = tau * 6.0 * PI + (s as f32) * 1.4;

        let mut sheet_pb = tiny_skia::PathBuilder::new();
        sheet_pb.move_to(40.0, ascend_y + sy_offset + 90.0);
        sheet_pb.cubic_to(
            width * 0.25, ascend_y + sy_offset + wave_phase.sin() * 70.0,
            width * 0.75, ascend_y + sy_offset - wave_phase.cos() * 70.0,
            width - 40.0, ascend_y + sy_offset + 90.0,
        );
        sheet_pb.cubic_to(
            width * 0.75, ascend_y + sy_offset + 140.0 - wave_phase.cos() * 60.0,
            width * 0.25, ascend_y + sy_offset + 140.0 + wave_phase.sin() * 60.0,
            40.0, ascend_y + sy_offset + 90.0,
        );
        sheet_pb.close();
        if let Some(sp) = sheet_pb.finish() {
            canvas.fill_path(&sp, if s % 2 == 0 { sheet_white } else { sheet_shade });
            canvas.stroke_path_fine(&sp, Color::hex("#9E9585"), 2.2);
        }
    }

    // 3. Figura celestial de Remedios la Bella en el centro elevándose
    // Cabello negro larguísimo hasta los tobillos
    let hair_wave = (tau * 10.0 * PI).sin() * 22.0;
    canvas.stroke_line_fine(cx, ascend_y - 80.0, cx - 45.0 + hair_wave, ascend_y + 110.0, Color::hex("#120E0A"), 22.0);
    canvas.stroke_line_fine(cx, ascend_y - 80.0, cx + 45.0 - hair_wave, ascend_y + 110.0, Color::hex("#120E0A"), 22.0);

    // Rostro esculpido en mármol y aureola
    canvas.fill_circle(cx, ascend_y - 95.0, 28.0, Color::hex("#FAF6ED"));
    canvas.stroke_circle(cx, ascend_y - 95.0, 28.0, Color::hex("#615645"), 2.0);
    canvas.fill_circle(cx, ascend_y - 95.0, 45.0, celestial_gold.with_alpha(0.35));

    // Brazos abiertos hacia el cielo infinito
    canvas.stroke_line_fine(cx - 15.0, ascend_y - 75.0, cx - 95.0, ascend_y - 140.0, Color::hex("#FAF6ED"), 12.0);
    canvas.stroke_line_fine(cx - 95.0, ascend_y - 140.0, cx - 120.0, ascend_y - 175.0, Color::hex("#FAF6ED"), 9.0);
    canvas.stroke_line_fine(cx + 15.0, ascend_y - 75.0, cx + 95.0, ascend_y - 140.0, Color::hex("#FAF6ED"), 12.0);
    canvas.stroke_line_fine(cx + 95.0, ascend_y - 140.0, cx + 120.0, ascend_y - 175.0, Color::hex("#FAF6ED"), 9.0);

    // 4. Lluvia de rosas carmesí y sangre mística cayendo de las sábanas
    for r in 0..20 {
        let rt = (tau * 2.5 + (r as f32) * 0.05) % 1.0;
        let rx = cx - width * 0.42 + (r as f32) * 44.0;
        let ry = ascend_y + 40.0 + rt * 550.0;
        // Pétalos de rosa
        canvas.fill_circle(rx, ry, 7.0, rose_carmine);
        canvas.stroke_circle(rx, ry, 7.0, Color::hex("#FFA0B8"), 1.2);
    }
}

/// 8. Mosaico Macroscópico a Pantalla Completa de Escamas de Mariposa Amarilla (50 µm)
/// Equivalente monumental de la Escena 11 de Monarch: cubre todo el formato 1080x1920 con cientos de tejas doradas.
pub fn draw_mosaico_escamas_mariposas(
    canvas: &mut Canvas,
    width: f32,
    height: f32,
    tau: f32,
    _seed: u32,
) {
    let base_dark = Color::hex("#5A3A0A");
    let scale_yellow = Color::hex("#FFD200");
    let scale_amber = Color::hex("#F5A010");
    let ridge_gold = Color::hex("#FFF5A0");

    canvas.clear(base_dark);

    let rows = 14;
    let cols = 10;
    let sh_w = width / (cols as f32 - 1.0);
    let sh_h = height / (rows as f32 - 1.0);

    for r in 0..rows {
        let gy = (r as f32) * sh_h;
        for c in 0..cols {
            let offset_x = if r % 2 == 1 { sh_w * 0.5 } else { 0.0 };
            let gx = (c as f32) * sh_w + offset_x - sh_w * 0.5;

            let shimmer = ((tau * 8.0 * PI + (r + c) as f32 * 0.8).sin() * 0.15).max(0.0);
            let col = if (r + c) % 3 == 0 { scale_amber } else { scale_yellow };

            // Escama festoneada individual (teja de quitina)
            let mut sc_pb = tiny_skia::PathBuilder::new();
            sc_pb.move_to(gx, gy - sh_h * 0.4);
            sc_pb.cubic_to(gx + sh_w * 0.5, gy - sh_h * 0.3, gx + sh_w * 0.5, gy + sh_h * 0.5, gx, gy + sh_h * 0.7);
            sc_pb.cubic_to(gx - sh_w * 0.5, gy + sh_h * 0.5, gx - sh_w * 0.5, gy - sh_h * 0.3, gx, gy - sh_h * 0.4);
            sc_pb.close();
            if let Some(scp) = sc_pb.finish() {
                canvas.fill_path(&scp, col);
                canvas.stroke_path_fine(&scp, base_dark, 1.8);

                // Estrías longitudinales micro-estructurales
                for s in -2..=2 {
                    let st_x = gx + (s as f32) * (sh_w * 0.15);
                    canvas.stroke_line_fine(st_x, gy - sh_h * 0.25, st_x, gy + sh_h * 0.55, ridge_gold.with_alpha(0.6 + shimmer), 1.0);
                }
            }
        }
    }
}

// =============================================================================
// NUEVAS ESTRUCTURAS MONUMENTALES A PANTALLA COMPLETA (FULL-BLEED)
// =============================================================================

/// 9. El Gran Bastidor Colonial y la Mortaja de Amaranta (Full-Bleed 1080x1920)
/// Telar monumental que ocupa de lado a lado con 40 hilos de urdimbre, encaje de bolillos,
/// guitarra barroca y el rollo perforado de pianola de Pietro Crespi.
pub fn draw_bastidor_mortaja_monumental(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
    _seed: u32,
) {
    let cedar_dark = Color::hex("#3A2312");
    let cedar_mid = Color::hex("#6B4423");
    let gold_thread = Color::hex("#FFD700");
    let linen_white = Color::hex("#FBF7EE");
    let linen_shade = Color::hex("#DDD2BC");
    let ink_line = Color::hex("#2A1C13");

    // Marco del bastidor de madera de cedro (de x=80 a x=w-80)
    let left_x = 80.0;
    let right_x = w - 80.0;
    let top_y = 240.0;
    let bot_y = h - 220.0;
    let beam_w = 42.0;

    // Postes verticales del bastidor
    canvas.fill_rect(left_x, top_y, beam_w, bot_y - top_y, cedar_mid);
    canvas.stroke_rect(left_x, top_y, beam_w, bot_y - top_y, cedar_dark, 3.0);

    canvas.fill_rect(right_x - beam_w, top_y, beam_w, bot_y - top_y, cedar_mid);
    canvas.stroke_rect(right_x - beam_w, top_y, beam_w, bot_y - top_y, cedar_dark, 3.0);

    // Travesaños horizontales
    canvas.fill_rect(left_x - 15.0, top_y, right_x - left_x + 30.0, beam_w, cedar_mid);
    canvas.stroke_rect(left_x - 15.0, top_y, right_x - left_x + 30.0, beam_w, cedar_dark, 3.0);

    canvas.fill_rect(left_x - 15.0, bot_y - beam_w, right_x - left_x + 30.0, beam_w, cedar_mid);
    canvas.stroke_rect(left_x - 15.0, bot_y - beam_w, right_x - left_x + 30.0, beam_w, cedar_dark, 3.0);

    // Clavijas de tensión de latón en los travesaños
    for k in 0..18 {
        let kx = left_x + 40.0 + (k as f32) * ((right_x - left_x - 80.0) / 17.0);
        canvas.fill_circle(kx, top_y + beam_w * 0.5, 6.0, gold_thread);
        canvas.fill_circle(kx, bot_y - beam_w * 0.5, 6.0, gold_thread);
    }

    // 40 Hilos verticales de urdimbre tensados de arriba abajo
    let warp_count = 38;
    for i in 0..warp_count {
        let wx = left_x + beam_w + 20.0 + (i as f32) * ((right_x - left_x - 2.0 * beam_w - 40.0) / (warp_count - 1) as f32);
        let col = if i % 4 == 0 { gold_thread.with_alpha(0.8) } else { linen_shade.with_alpha(0.6) };
        canvas.stroke_line_fine(wx, top_y + beam_w, wx, bot_y - beam_w, col, 1.2);
    }

    // El sudario de encaje tejido en el centro (anchura 620px, altura 740px)
    let shroud_w = 580.0;
    let shroud_h = 700.0;
    let shroud_x = cx - shroud_w * 0.5;
    let shroud_y = cy - shroud_h * 0.45;

    canvas.fill_rect(shroud_x, shroud_y, shroud_w, shroud_h, linen_white.with_alpha(0.92));
    canvas.stroke_rect(shroud_x, shroud_y, shroud_w, shroud_h, ink_line, 2.5);

    // Filigrana de encaje de bolillos (patrón de rombos y rosetas)
    let lace_rows = 10;
    let lace_cols = 8;
    let cell_w = shroud_w / lace_cols as f32;
    let cell_h = shroud_h / lace_rows as f32;

    for lr in 0..lace_rows {
        let ly = shroud_y + (lr as f32) * cell_h;
        for lc in 0..lace_cols {
            let lx = shroud_x + (lc as f32) * cell_w;
            let center_x = lx + cell_w * 0.5;
            let center_y = ly + cell_h * 0.5;

            // Rombo de encaje
            let mut r_pb = tiny_skia::PathBuilder::new();
            r_pb.move_to(center_x, ly);
            r_pb.line_to(lx + cell_w, center_y);
            r_pb.line_to(center_x, ly + cell_h);
            r_pb.line_to(lx, center_y);
            r_pb.close();
            if let Some(rp) = r_pb.finish() {
                canvas.stroke_path_fine(&rp, ink_line.with_alpha(0.4), 1.0);
            }

            // Flor central de 4 pétalos
            canvas.fill_circle(center_x, center_y, 4.0, gold_thread);
        }
    }

    // Naveta de tejer dorada que se mueve de un lado a otro
    let shuttle_x = cx + (tau * 12.0 * PI).sin() * (shroud_w * 0.42);
    let shuttle_y = shroud_y + shroud_h * 0.85;
    canvas.fill_circle(shuttle_x, shuttle_y, 14.0, gold_thread);
    canvas.stroke_circle(shuttle_x, shuttle_y, 14.0, ink_line, 2.0);
    // Hilo dorado que sale de la naveta
    canvas.stroke_line_fine(shuttle_x, shuttle_y, cx, shroud_y + shroud_h, gold_thread, 2.2);

    // Rollo perforado de pianola de Pietro Crespi en el costado derecho
    let roll_x = right_x - beam_w - 60.0;
    canvas.fill_rect(roll_x, top_y + 80.0, 50.0, 500.0, linen_white);
    canvas.stroke_rect(roll_x, top_y + 80.0, 50.0, 500.0, ink_line, 2.0);
    for p in 0..24 {
        let py = top_y + 95.0 + (p as f32) * 20.0;
        let px = roll_x + 12.0 + ((p * 7) % 26) as f32;
        canvas.fill_circle(px, py, 3.2, ink_line);
    }
    draw_vector_text(canvas, roll_x + 25.0, top_y + 600.0, "PIANOLA", 10.0, ink_line, true);

    // Silueta de guitarra española clásica al costado izquierdo
    let g_x = left_x + beam_w + 60.0;
    let g_y = cy;
    // Caja armónica
    canvas.stroke_circle(g_x, g_y + 70.0, 65.0, cedar_dark, 2.2);
    canvas.stroke_circle(g_x, g_y - 45.0, 50.0, cedar_dark, 2.2);
    canvas.fill_circle(g_x, g_y + 20.0, 18.0, ink_line); // Boca
    canvas.stroke_circle(g_x, g_y + 20.0, 24.0, gold_thread, 1.8); // Roseta
    canvas.stroke_line_fine(g_x, g_y - 45.0, g_x, top_y + 80.0, cedar_dark, 14.0); // Mástil
}

/// 10. El Árbol Genealógico Monumental de Úrsula Iguarán (Full-Bleed 1080x1920)
/// El roble centenario de las 7 estirpes de Buendía que abarca desde las raíces inferiores
/// hasta las ramas celestiales, con la venerable matriarca Úrsula y el reloj de arena.
pub fn draw_arbol_genealogico_ursula_monumental(
    canvas: &mut Canvas,
    cx: f32,
    w: f32,
    h: f32,
    tau: f32,
    _seed: u32,
) {
    let trunk_col = Color::hex("#4A2E1B");
    let trunk_line = Color::hex("#28160B");
    let gold_leaf = Color::hex("#FFD700");
    let scroll_bg = Color::hex("#FBF7EE");
    let scroll_line = Color::hex("#2A1C13");
    let green_foliage = Color::hex("#2E5339").with_alpha(0.35);

    // 1. Raíces colosales que se hunden en la base (de x=80 a x=w-80)
    let root_y = h - 220.0;
    for r in 0..9 {
        let rx_end = 80.0 + (r as f32) * ((w - 160.0) / 8.0);
        let mut r_pb = tiny_skia::PathBuilder::new();
        r_pb.move_to(cx - 70.0 + (r as f32 * 16.0), h * 0.62);
        r_pb.cubic_to(cx + (r as f32 - 4.0) * 45.0, h * 0.74, rx_end - 30.0, h * 0.84, rx_end, root_y);
        if let Some(rp) = r_pb.finish() {
            canvas.stroke_path_fine(&rp, trunk_line, 8.0);
            canvas.stroke_path_fine(&rp, trunk_col, 5.0);
        }
    }

    // 2. Tronco colosal ascendente
    let mut tr_pb = tiny_skia::PathBuilder::new();
    tr_pb.move_to(cx - 85.0, h * 0.64);
    tr_pb.cubic_to(cx - 65.0, h * 0.50, cx - 75.0, h * 0.40, cx - 50.0, h * 0.30);
    tr_pb.line_to(cx + 50.0, h * 0.30);
    tr_pb.cubic_to(cx + 75.0, h * 0.40, cx + 65.0, h * 0.50, cx + 85.0, h * 0.64);
    tr_pb.close();
    if let Some(tp) = tr_pb.finish() {
        canvas.fill_path(&tp, trunk_col);
        canvas.stroke_path_fine(&tp, trunk_line, 3.5);
    }

    // Rayado de aguafuerte en el tronco
    for t in 0..16 {
        let ty = h * 0.32 + (t as f32) * (h * 0.02);
        canvas.stroke_line_fine(cx - 55.0, ty, cx + 55.0, ty + 12.0, trunk_line.with_alpha(0.6), 1.5);
    }

    // 3. Ramas principales que se abren hacia todos los cuadrantes
    let branches = [
        (cx - 50.0, h * 0.32, 140.0, h * 0.22),
        (cx + 50.0, h * 0.32, w - 140.0, h * 0.22),
        (cx - 45.0, h * 0.36, 110.0, h * 0.36),
        (cx + 45.0, h * 0.36, w - 110.0, h * 0.36),
        (cx - 35.0, h * 0.42, 120.0, h * 0.48),
        (cx + 35.0, h * 0.42, w - 120.0, h * 0.48),
    ];
    for &(bx0, by0, bx1, by1) in &branches {
        let mut b_pb = tiny_skia::PathBuilder::new();
        b_pb.move_to(bx0, by0);
        b_pb.cubic_to((bx0 + bx1) * 0.5, by0 - 40.0, (bx0 + bx1) * 0.5, by1 + 40.0, bx1, by1);
        if let Some(bp) = b_pb.finish() {
            canvas.stroke_path_fine(&bp, green_foliage, 48.0);
            canvas.stroke_path_fine(&bp, trunk_line, 6.0);
            canvas.stroke_path_fine(&bp, trunk_col, 3.5);
        }
    }

    // 4. Cartelas genealógicas de las 7 Generaciones
    let generations = [
        ("I • FUNDADORES: JOSE ARCADIO BUENDIA & URSULA IGUARAN", cx, h * 0.21, 560.0),
        ("II • CORONEL AURELIANO • AMARANTA • JOSE ARCADIO", cx - 220.0, h * 0.31, 380.0),
        ("III • ARCADIO • AURELIANO JOSE • 17 AURELIANOS", cx + 220.0, h * 0.31, 380.0),
        ("IV • AURELIANO SEGUNDO • J.A. SEGUNDO • REMEDIOS", cx - 220.0, h * 0.43, 390.0),
        ("V • MEME • JOSE ARCADIO • AMARANTA URSULA", cx + 220.0, h * 0.43, 380.0),
        ("VI • AURELIANO BABILONIA (EL DESCIFRADOR)", cx - 180.0, h * 0.52, 360.0),
        ("VII • EL ULTIMO DE LA ESTIRPE CON COLA DE CERDO", cx + 180.0, h * 0.52, 380.0),
    ];

    for &(text, gx, gy, gw) in &generations {
        let gh = 28.0;
        canvas.fill_rect(gx - gw * 0.5, gy - gh * 0.5, gw, gh, scroll_bg);
        canvas.stroke_rect(gx - gw * 0.5, gy - gh * 0.5, gw, gh, scroll_line, 1.8);
        canvas.stroke_rect(gx - gw * 0.5 + 3.0, gy - gh * 0.5 + 3.0, gw - 6.0, gh - 6.0, gold_leaf.with_alpha(0.6), 1.0);
        draw_vector_text(canvas, gx, gy + 4.0, text, 10.0, scroll_line, true);
    }

    // 5. La venerable Úrsula Iguarán en su mecedora al pie del tronco
    let u_x = cx;
    let u_y = h * 0.65;
    let rock_rot = (tau * 4.0 * PI).sin() * 0.05;

    canvas.save();
    canvas.translate(u_x, u_y);
    canvas.rotate(rock_rot);

    // Patines de la mecedora
    let mut roc_pb = tiny_skia::PathBuilder::new();
    roc_pb.move_to(-95.0, 95.0);
    roc_pb.cubic_to(-40.0, 115.0, 40.0, 115.0, 95.0, 95.0);
    if let Some(rp) = roc_pb.finish() {
        canvas.stroke_path_fine(&rp, trunk_line, 5.0);
    }

    // Estructura de la mecedora
    canvas.stroke_line_fine(-55.0, 20.0, 55.0, 20.0, trunk_line, 4.0); // Asiento
    canvas.stroke_line_fine(-50.0, 20.0, -75.0, -85.0, trunk_line, 4.0); // Respaldo
    canvas.stroke_line_fine(-50.0, 20.0, -70.0, 105.0, trunk_line, 3.5); // Pata tras.
    canvas.stroke_line_fine(50.0, 20.0, 65.0, 105.0, trunk_line, 3.5); // Pata del.

    // Vestido largo negro de luto de Úrsula
    let mut dress_pb = tiny_skia::PathBuilder::new();
    dress_pb.move_to(-45.0, -60.0);
    dress_pb.line_to(-10.0, -60.0);
    dress_pb.line_to(45.0, 25.0);
    dress_pb.line_to(15.0, 75.0);
    dress_pb.line_to(-65.0, 75.0);
    dress_pb.line_to(-45.0, 20.0);
    dress_pb.close();
    if let Some(dp) = dress_pb.finish() {
        canvas.fill_path(&dp, Color::hex("#1C1512"));
        canvas.stroke_path_fine(&dp, trunk_line, 2.0);
    }

    // Rostro de Úrsula centenaria y pañolón
    canvas.fill_circle(-30.0, -75.0, 16.0, Color::hex("#DEBA9C"));
    canvas.stroke_circle(-30.0, -75.0, 16.0, trunk_line, 1.8);
    // Pañolón blanco
    canvas.stroke_line_fine(-45.0, -75.0, -30.0, -95.0, Color::hex("#FBF7EE"), 3.0);
    canvas.stroke_line_fine(-30.0, -95.0, -15.0, -75.0, Color::hex("#FBF7EE"), 3.0);

    // Gallitos de caramelo en su regazo
    canvas.fill_circle(10.0, 15.0, 9.0, Color::hex("#FF7640"));
    canvas.fill_circle(25.0, 15.0, 9.0, Color::hex("#FFD700"));

    canvas.restore();

    // 6. Gran reloj de arena centenario a la derecha de la mecedora
    draw_reloj_arena_ursula(canvas, cx + 160.0, h * 0.65, 1.45, tau);
}

/// 11. El Acordeón de Francisco el Hombre Monumental (Full-Bleed 1080x1920)
/// Acordeón Hohner Corona III de 860px de anchura, ondas acústicas armónicas a pantalla completa,
/// y los rieles del tren bananero atravesando la base con la locomotora.
pub fn draw_acordeon_monumental_plate(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
) {
    let navy_line = Color::hex("#80D0FF");
    let gold_line = Color::hex("#FFD700");
    let white_pearl = Color::hex("#FBF7EE");
    let dim_line = Color::hex("#C8C1EF");

    // Ondas acústicas esféricas en BlendMode::Screen que llenan todo el fondo
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);
    for ring in 0..10 {
        let phase = (tau * 2.5 + ring as f32 * 0.1) % 1.0;
        let rad = 180.0 + phase * 750.0;
        let alpha = (1.0 - phase) * 0.45;
        canvas.stroke_circle(cx, cy - 60.0, rad, navy_line.with_alpha(alpha), 2.0);
        if ring % 2 == 0 {
            canvas.stroke_circle(cx, cy - 60.0, rad * 0.98, gold_line.with_alpha(alpha * 0.8), 1.2);
        }
    }
    canvas.restore();

    // El Acordeón Hohner Corona III a escala colosal (ancho ~840px, alto ~520px)
    let pump = (tau * 4.0 * PI).sin() * 45.0;
    let acc_h = 420.0;

    // Caja de botones agudos (Treble cabinet) a la derecha
    let right_x = cx + 110.0 + pump;
    canvas.fill_rect(right_x, cy - 60.0 - acc_h * 0.5, 160.0, acc_h, Color::hex("#0D1D3A"));
    canvas.stroke_rect(right_x, cy - 60.0 - acc_h * 0.5, 160.0, acc_h, navy_line, 3.5);

    // Diapasón con 3 hileras de 31 botones de nácar
    for col in 0..3 {
        let bx = right_x + 35.0 + (col as f32) * 42.0;
        let num_buttons = if col == 1 { 11 } else { 10 };
        for btn in 0..num_buttons {
            let by = cy - 60.0 - acc_h * 0.42 + (btn as f32) * (acc_h * 0.84 / (num_buttons - 1) as f32);
            canvas.fill_circle(bx, by, 11.0, white_pearl);
            canvas.stroke_circle(bx, by, 11.0, Color::WHITE, 2.0);
            canvas.stroke_circle(bx, by, 14.0, gold_line.with_alpha(0.6), 1.0);
        }
    }

    // Caja de bajos a la izquierda (Bass cabinet)
    let left_x = cx - 110.0 - pump - 140.0;
    canvas.fill_rect(left_x, cy - 60.0 - acc_h * 0.5, 140.0, acc_h, Color::hex("#0D1D3A"));
    canvas.stroke_rect(left_x, cy - 60.0 - acc_h * 0.5, 140.0, acc_h, navy_line, 3.5);

    // 12 bajos mecánicos
    for col in 0..2 {
        let bx = left_x + 40.0 + (col as f32) * 50.0;
        for btn in 0..6 {
            let by = cy - 60.0 - acc_h * 0.35 + (btn as f32) * (acc_h * 0.70 / 5.0);
            canvas.fill_circle(bx, by, 13.0, white_pearl);
            canvas.stroke_circle(bx, by, 13.0, gold_line, 2.2);
        }
    }

    // Fuelle central (Bellows) de 14 pliegues que se expande
    let bellow_start = left_x + 140.0;
    let bellow_end = right_x;
    let b_total_w = bellow_end - bellow_start;
    let folds = 12;
    let fold_step = b_total_w / folds as f32;

    for f in 0..folds {
        let fx0 = bellow_start + (f as f32) * fold_step;
        let fx1 = fx0 + fold_step * 0.5;
        let fx2 = fx0 + fold_step;

        let mut fold_pb = tiny_skia::PathBuilder::new();
        fold_pb.move_to(fx0, cy - 60.0 - acc_h * 0.46);
        fold_pb.line_to(fx1, cy - 60.0 - acc_h * 0.52);
        fold_pb.line_to(fx2, cy - 60.0 - acc_h * 0.46);
        fold_pb.line_to(fx2, cy - 60.0 + acc_h * 0.46);
        fold_pb.line_to(fx1, cy - 60.0 + acc_h * 0.52);
        fold_pb.line_to(fx0, cy - 60.0 + acc_h * 0.46);
        fold_pb.close();

        if let Some(path) = fold_pb.finish() {
            let b_col = if f % 2 == 0 { Color::hex("#122A54") } else { Color::hex("#1E4282") };
            canvas.fill_path(&path, b_col);
            canvas.stroke_path_fine(&path, navy_line, 2.0);

            // Cantoneras metálicas de níquel
            canvas.fill_circle(fx1, cy - 60.0 - acc_h * 0.51, 4.0, gold_line);
            canvas.fill_circle(fx1, cy - 60.0 + acc_h * 0.51, 4.0, gold_line);
        }
    }

    // Rieles de ferrocarril de la Compañía Bananera que cruzan todo el ancho abajo
    let rail_y1 = h - 260.0;
    let rail_y2 = rail_y1 + 28.0;
    canvas.stroke_line_fine(0.0, rail_y1, w, rail_y1, navy_line, 4.0);
    canvas.stroke_line_fine(0.0, rail_y2, w, rail_y2, navy_line, 4.0);

    // Durmientes de madera
    let ties = 24;
    for t in 0..=ties {
        let tx = (t as f32 / ties as f32) * w;
        canvas.stroke_line_fine(tx, rail_y1 - 10.0, tx, rail_y2 + 10.0, dim_line, 2.5);
    }

    // Tren amarillo en miniatura cruzando sobre los rieles
    let train_x = (tau * 1.5 * w) % (w + 400.0) - 200.0;
    canvas.fill_rect(train_x, rail_y1 - 55.0, 180.0, 48.0, Color::hex("#FFD700"));
    canvas.stroke_rect(train_x, rail_y1 - 55.0, 180.0, 48.0, Color::hex("#2A1C13"), 2.2);
    // Chimenea de la locomotora
    canvas.fill_rect(train_x + 140.0, rail_y1 - 85.0, 22.0, 32.0, Color::hex("#2A1C13"));
}

/// 12. Mauricio Babilonia, la Ventana de Meme y el Remolino de Mariposas (Full-Bleed 1080x1920)
/// Fachada colonial de cal y canto, ventana enrejada con silueta romántica,
/// y un vórtice colosal de 45 mariposas amarillas que cruza toda la pantalla.
pub fn draw_mauricio_mariposas_monumental(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
    seed: u32,
) {
    let line_col = Color::hex("#80D0FF");
    let wall_col = Color::hex("#0A162B");

    // 1. Muro de sillería colonial que cubre de lado a lado
    canvas.fill_rect(60.0, 220.0, w - 120.0, h - 440.0, wall_col);
    canvas.stroke_rect(60.0, 220.0, w - 120.0, h - 440.0, line_col.with_alpha(0.4), 2.0);

    // Sillares de piedra grabados
    for row in 0..12 {
        let sy = 240.0 + (row as f32) * 110.0;
        canvas.stroke_line_fine(60.0, sy, w - 60.0, sy, line_col.with_alpha(0.18), 1.0);
    }

    // 2. Ventana alta colonial enrejada (Meme)
    let win_x = cx - 190.0;
    let win_y = cy - 180.0;
    let win_w = 260.0;
    let win_h = 360.0;

    // Vano con arco de medio punto
    let mut arch_pb = tiny_skia::PathBuilder::new();
    arch_pb.move_to(win_x - win_w * 0.5, win_y + win_h * 0.5);
    arch_pb.line_to(win_x - win_w * 0.5, win_y - win_h * 0.2);
    arch_pb.cubic_to(win_x - win_w * 0.5, win_y - win_h * 0.6, win_x + win_w * 0.5, win_y - win_h * 0.6, win_x + win_w * 0.5, win_y - win_h * 0.2);
    arch_pb.line_to(win_x + win_w * 0.5, win_y + win_h * 0.5);
    arch_pb.close();
    if let Some(ap) = arch_pb.finish() {
        canvas.fill_path(&ap, Color::hex("#040814"));
        canvas.stroke_path_fine(&ap, line_col, 3.5);
    }

    // Reja de hierro forjado
    for bar in 0..8 {
        let bx = win_x - win_w * 0.4 + (bar as f32) * (win_w * 0.8 / 7.0);
        canvas.stroke_line_fine(bx, win_y - win_h * 0.35, bx, win_y + win_h * 0.48, line_col.with_alpha(0.75), 2.5);
    }
    // Travesaños de la reja
    for ty in [-0.1, 0.2] {
        let bar_y = win_y + win_h * ty;
        canvas.stroke_line_fine(win_x - win_w * 0.45, bar_y, win_x + win_w * 0.45, bar_y, line_col.with_alpha(0.75), 2.5);
    }

    // Silueta de Meme en la ventana con larga cabellera negra
    canvas.fill_circle(win_x, win_y - 45.0, 22.0, line_col);
    let mut hair_pb = tiny_skia::PathBuilder::new();
    hair_pb.move_to(win_x - 24.0, win_y - 45.0);
    hair_pb.cubic_to(win_x - 38.0, win_y + 10.0, win_x - 30.0, win_y + 85.0, win_x - 18.0, win_y + 115.0);
    hair_pb.line_to(win_x + 12.0, win_y + 115.0);
    hair_pb.cubic_to(win_x + 22.0, win_y + 70.0, win_x + 28.0, win_y + 10.0, win_x + 22.0, win_y - 45.0);
    hair_pb.close();
    if let Some(hp) = hair_pb.finish() {
        canvas.fill_path(&hp, Color::hex("#03060E"));
    }

    // 3. Mauricio Babilonia en el patio abajo a la derecha
    let mx = cx + 220.0;
    let my = cy + 240.0;
    // Mesa de trabajo con tornillo de banco y herramientas
    canvas.fill_rect(mx - 100.0, my + 40.0, 200.0, 90.0, Color::hex("#0D1A33"));
    canvas.stroke_rect(mx - 100.0, my + 40.0, 200.0, 90.0, line_col, 2.5);
    // Llave inglesa y engranajes mecánicos
    canvas.stroke_line_fine(mx - 60.0, my + 30.0, mx - 20.0, my + 10.0, Color::hex("#FFD700"), 4.0);
    canvas.stroke_circle(mx + 40.0, my + 25.0, 18.0, line_col, 2.0);

    // Silueta de Mauricio con camisa de mecánico
    canvas.fill_circle(mx, my - 65.0, 22.0, line_col);
    canvas.stroke_circle(mx, my - 65.0, 22.0, Color::hex("#FFFFFF"), 1.5);
    // Torso y brazos
    canvas.stroke_line_fine(mx, my - 45.0, mx, my + 35.0, line_col, 18.0);
    canvas.stroke_line_fine(mx, my - 30.0, mx - 50.0, my + 10.0, line_col, 7.0);
    canvas.stroke_line_fine(mx, my - 30.0, mx + 50.0, my + 10.0, line_col, 7.0);

    // 4. Vórtice colosal de 45 mariposas amarillas que conecta a Mauricio con la ventana de Meme
    let butterfly_count = 42;
    for i in 0..butterfly_count {
        let prog = (tau * 1.8 + (i as f32 / butterfly_count as f32)) % 1.0;
        // Trayectoria espiral ascendente en forma de cinta de Moebius
        let ang = prog * 6.0 * PI + (i as f32 * 0.4);
        let spiral_r = 80.0 + prog * 380.0;
        let bx = cx + (prog - 0.5) * 220.0 + ang.cos() * spiral_r;
        let by = (my + 40.0) - prog * (h * 0.65) + ang.sin() * (spiral_r * 0.45);

        let flap = ((tau * 30.0 + i as f32 * 1.5) * PI).sin();
        let b_scale = 0.35 + (1.0 - (prog - 0.5).abs() * 1.5).max(0.15) * 0.55;
        draw_yellow_butterfly(canvas, bx, by, b_scale, flap, seed + i as u32);
    }
}

/// 13. El Gran Viento Bíblico y Ciclón Apocalíptico Monumental (Full-Bleed 1080x1920)
/// El huracán final que borra a Macondo con 48 hojas de pergaminos en sánscrito volando en 3D
/// a través de espirales cósmicas que abarcan toda la pantalla.
pub fn draw_viento_biblico_monumental(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    let wind_cyan = Color::hex("#00F5D4").with_alpha(0.40);
    let wind_gold = Color::hex("#FFD700").with_alpha(0.50);
    let parchment_col = Color::hex("#EFE3C9");
    let ink_col = Color::hex("#1E140C");

    // 3 Espirales huracanadas colosales que se extienden hasta el borde de la pantalla (radio 760px)
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);

    for sp in 0..4 {
        let offset = sp as f32 * (PI * 0.5);
        let mut pb = tiny_skia::PathBuilder::new();
        let steps = 60;
        for i in 0..steps {
            let theta = (i as f32) * 0.16 + tau * 8.0 * PI + offset;
            let r = 40.0 + (i as f32) * 12.0;
            let px = cx + theta.cos() * r;
            let py = cy + theta.sin() * (r * 1.15);
            if i == 0 { pb.move_to(px, py); } else { pb.line_to(px, py); }
        }
        if let Some(path) = pb.finish() {
            let col = if sp % 2 == 0 { wind_cyan } else { wind_gold };
            canvas.stroke_path_fine(&path, col, 4.0);
        }
    }
    canvas.restore();

    // 45 Hojas de pergaminos descifrados de Melquíades volando en torbellino
    let sheets = 42;
    for s in 0..sheets {
        let prog = (tau * 2.2 + (s as f32 / sheets as f32)) % 1.0;
        let theta = prog * 8.0 * PI + (s as f32 * 1.4);
        let r = 50.0 + prog * 680.0;
        let px = cx + theta.cos() * r;
        let py = cy + theta.sin() * (r * 1.1);
        let rot = theta + PI * 0.5 + (rng.next_f32() - 0.5) * 0.8;

        if px > 20.0 && px < w - 20.0 && py > 60.0 && py < h - 60.0 {
            canvas.save();
            canvas.translate(px, py);
            canvas.rotate(rot);

            let pw = 48.0;
            let ph = 34.0;
            canvas.fill_rect(-pw * 0.5, -ph * 0.5, pw, ph, parchment_col);
            canvas.stroke_rect(-pw * 0.5, -ph * 0.5, pw, ph, ink_col, 1.8);

            // Caracteres en sánscrito
            for line in 0..3 {
                let ly = -ph * 0.28 + (line as f32) * 10.0;
                canvas.stroke_line_fine(-pw * 0.38, ly, pw * 0.38, ly, ink_col.with_alpha(0.7), 1.2);
            }

            canvas.restore();
        }
    }
}

