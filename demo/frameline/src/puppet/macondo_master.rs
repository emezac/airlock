use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::rng::Rng;
use crate::core::schematic::draw_vector_text;
use tiny_skia::BlendMode;
use std::f32::consts::PI;

// =============================================================================
// 1. ÁRBOL DE BANANO BOTÁNICO AUTÉNTICO (Musa acuminata - El Platanal de Macondo)
// =============================================================================

pub fn draw_master_banana_tree(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    wind_phase: f32,
) {
    let trunk_h = 480.0 * scale;
    let trunk_w = 54.0 * scale;

    let stem_base_col = Color::hex("#384816");
    let stem_mid_col = Color::hex("#546E22");
    let stem_sheath_col = Color::hex("#755E32");
    let leaf_green_deep = Color::hex("#14440C");
    let leaf_green_mid = Color::hex("#287018");
    let leaf_green_bright = Color::hex("#44992A");
    let midrib_col = Color::hex("#A6DF4E");
    let banana_green = Color::hex("#76A828");
    let banana_gold = Color::hex("#B8C734");
    let blossom_purple = Color::hex("#56142C");

    canvas.save();
    canvas.translate(cx, cy);

    // A. Pseudotallo cónico con vainas fibrosas
    let mut stem_pb = tiny_skia::PathBuilder::new();
    stem_pb.move_to(-trunk_w * 0.75, 0.0);
    stem_pb.cubic_to(-trunk_w * 0.65, -trunk_h * 0.40, -trunk_w * 0.50, -trunk_h * 0.75, -trunk_w * 0.38, -trunk_h);
    stem_pb.line_to(trunk_w * 0.38, -trunk_h);
    stem_pb.cubic_to(trunk_w * 0.50, -trunk_h * 0.75, trunk_w * 0.65, -trunk_h * 0.40, trunk_w * 0.75, 0.0);
    stem_pb.close();

    if let Some(path) = stem_pb.finish() {
        canvas.fill_path(&path, stem_mid_col);
        canvas.stroke_path_fine(&path, stem_base_col, 2.5 * scale);

        for i in 0..9 {
            let sy = -(i as f32) * (trunk_h / 8.5);
            let sw = trunk_w * (0.70 - (i as f32) * 0.04);
            canvas.stroke_line_fine(-sw, sy, sw, sy - 8.0 * scale, stem_sheath_col, 2.5 * scale);
        }
    }

    // B. Racimo de plátanos con pedúnculo curvo y bellota
    let bunch_x = -20.0 * scale;
    let bunch_y = -trunk_h + 35.0 * scale;
    let mut stalk_pb = tiny_skia::PathBuilder::new();
    stalk_pb.move_to(0.0, -trunk_h);
    stalk_pb.cubic_to(bunch_x - 35.0 * scale, bunch_y - 20.0 * scale, bunch_x - 55.0 * scale, bunch_y + 90.0 * scale, bunch_x - 42.0 * scale, bunch_y + 160.0 * scale);
    if let Some(sp) = stalk_pb.finish() {
        canvas.stroke_path_fine(&sp, stem_base_col, 12.0 * scale);
        canvas.stroke_path_fine(&sp, stem_mid_col, 7.0 * scale);
    }

    // 4 Manos de plátanos verdes
    for hand in 0..4 {
        let hy = bunch_y + 40.0 * scale + (hand as f32) * 26.0 * scale;
        let hx = bunch_x - 45.0 * scale + (hand as f32) * 4.0 * scale;
        for b in 0..5 {
            let bx = hx - 24.0 * scale + (b as f32) * 12.0 * scale;
            let mut banana_pb = tiny_skia::PathBuilder::new();
            banana_pb.move_to(bx, hy);
            banana_pb.cubic_to(bx + 14.0 * scale, hy + 22.0 * scale, bx + 18.0 * scale, hy + 34.0 * scale, bx + 5.0 * scale, hy + 46.0 * scale);
            if let Some(bp) = banana_pb.finish() {
                canvas.stroke_path_fine(&bp, banana_green, 9.0 * scale);
                canvas.stroke_path_fine(&bp, banana_gold, 5.0 * scale);
            }
        }
    }

    // Flor / Bellota de plátano púrpura
    let blossom_y = bunch_y + 160.0 * scale;
    let blossom_x = bunch_x - 42.0 * scale;
    let mut blossom_pb = tiny_skia::PathBuilder::new();
    blossom_pb.move_to(blossom_x, blossom_y);
    blossom_pb.cubic_to(blossom_x - 22.0 * scale, blossom_y + 18.0 * scale, blossom_x - 20.0 * scale, blossom_y + 55.0 * scale, blossom_x, blossom_y + 75.0 * scale);
    blossom_pb.cubic_to(blossom_x + 20.0 * scale, blossom_y + 55.0 * scale, blossom_x + 22.0 * scale, blossom_y + 18.0 * scale, blossom_x, blossom_y);
    blossom_pb.close();
    if let Some(blp) = blossom_pb.finish() {
        canvas.fill_path(&blp, blossom_purple);
        canvas.stroke_path_fine(&blp, Color::hex("#320A1A"), 2.2 * scale);
    }

    // C. 8 Hojas gigantescas lanceoladas arqueadas (hasta 560px)
    let leaf_configs = [
        (-1.0, 0.42, 540.0, 140.0, 0.25),
        ( 1.0, 0.38, 560.0, 145.0, -0.22),
        (-1.0, 0.72, 480.0, 130.0, 0.55),
        ( 1.0, 0.68, 500.0, 135.0, -0.48),
        (-1.0, 1.08, 410.0, 115.0, 0.85),
        ( 1.0, 1.02, 430.0, 120.0, -0.80),
        (-0.3, 0.20, 460.0, 125.0, 0.10),
        ( 0.3, 0.18, 470.0, 125.0, -0.10),
    ];

    for &(side, angle_base, leaf_len, leaf_w, curve_dir) in &leaf_configs {
        let wind_bend = (wind_phase + curve_dir * 2.0).sin() * 25.0 * scale;
        let attach_y = -trunk_h + 18.0 * scale;

        canvas.save();
        canvas.translate(0.0, attach_y);

        let rad = angle_base * side;
        canvas.rotate(rad);

        let l = leaf_len * scale;
        let w = leaf_w * scale;

        let mut leaf_pb = tiny_skia::PathBuilder::new();
        leaf_pb.move_to(0.0, 0.0);
        leaf_pb.cubic_to(l * 0.35, -w * 0.55, l * 0.75, -w * 0.65 + wind_bend, l, wind_bend * 1.5);
        leaf_pb.cubic_to(l * 0.75,  w * 0.55 + wind_bend, l * 0.35,  w * 0.45, 0.0, 0.0);
        leaf_pb.close();

        if let Some(lp) = leaf_pb.finish() {
            canvas.fill_path(&lp, leaf_green_mid);
            canvas.stroke_path_fine(&lp, leaf_green_deep, 2.2 * scale);

            let mut midrib_pb = tiny_skia::PathBuilder::new();
            midrib_pb.move_to(0.0, 0.0);
            midrib_pb.cubic_to(l * 0.4, 0.0, l * 0.75, wind_bend * 0.6, l, wind_bend * 1.5);
            if let Some(mp) = midrib_pb.finish() {
                canvas.stroke_path_fine(&mp, midrib_col, 5.0 * scale);
            }

            for v in 1..=9 {
                let vx = (v as f32 / 10.0) * l;
                let vy_mid = (vx / l) * wind_bend * 0.8;
                let slit_len = w * 0.44 * (1.0 - (vx / l - 0.5).abs() * 0.8);
                canvas.stroke_line_fine(vx, vy_mid, vx + 25.0 * scale, vy_mid - slit_len, leaf_green_bright, 1.6 * scale);
                canvas.stroke_line_fine(vx, vy_mid, vx + 25.0 * scale, vy_mid + slit_len, leaf_green_deep, 1.6 * scale);

                if v % 3 == 0 {
                    canvas.stroke_line_fine(vx + 18.0 * scale, vy_mid - slit_len * 0.4, vx + 25.0 * scale, vy_mid - slit_len, Color::hex("#0C2206"), 2.2 * scale);
                }
            }
        }

        canvas.restore();
    }

    canvas.restore();
}

// =============================================================================
// 2. TROMPETA HERÁLDICA MONUMENTAL (Pabellón de 200px con Blasón de Macondo)
// =============================================================================

pub fn draw_master_trumpet_heraldic(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
    is_playing: bool,
) {
    let gold_base = Color::hex("#F6C432");
    let gold_deep = Color::hex("#B8860B");
    let gold_high = Color::hex("#FFF3A8");
    let silver_key = Color::hex("#E8ECEF");
    let ink_dark = Color::hex("#1A1208");
    let banner_red = Color::hex("#8B1824");
    let banner_gold = Color::hex("#F5D061");

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    let l = 620.0 * scale;
    let tube_r = 8.0 * scale;
    let bell_lip_r = 95.0 * scale;

    // A. Estandarte heráldico colonial
    let b_x1 = -l * 0.25;
    let b_x2 = l * 0.18;
    let b_w = b_x2 - b_x1;
    let b_wave = (tau * 8.0 * PI).sin() * 15.0 * scale;

    let mut banner_pb = tiny_skia::PathBuilder::new();
    banner_pb.move_to(b_x1, 18.0 * scale);
    banner_pb.line_to(b_x2, 18.0 * scale);
    banner_pb.cubic_to(b_x2 + 12.0 * scale, 160.0 * scale + b_wave, b_x2 - 25.0 * scale, 250.0 * scale + b_wave, b_x1 + b_w * 0.5, 290.0 * scale + b_wave);
    banner_pb.cubic_to(b_x1 + 25.0 * scale, 250.0 * scale + b_wave, b_x1 - 12.0 * scale, 160.0 * scale + b_wave, b_x1, 18.0 * scale);
    banner_pb.close();

    if let Some(bp) = banner_pb.finish() {
        canvas.fill_path(&bp, banner_red);
        canvas.stroke_path_fine(&bp, banner_gold, 3.0 * scale);

        canvas.stroke_line_fine(b_x1 + 24.0 * scale, 0.0, b_x1 + 24.0 * scale, 18.0 * scale, banner_gold, 3.0 * scale);
        canvas.stroke_line_fine(b_x2 - 24.0 * scale, 0.0, b_x2 - 24.0 * scale, 18.0 * scale, banner_gold, 3.0 * scale);

        for f in 0..16 {
            let fx = b_x1 + 18.0 * scale + (f as f32) * (b_w - 36.0 * scale) / 15.0;
            canvas.stroke_line_fine(fx, 240.0 * scale + b_wave, fx, 275.0 * scale + b_wave, banner_gold, 2.2 * scale);
        }

        draw_vector_text(canvas, b_x1 + b_w * 0.5, 110.0 * scale + b_wave, "M A C O N D O", 20.0 * scale, banner_gold, true);
        draw_vector_text(canvas, b_x1 + b_w * 0.5, 145.0 * scale + b_wave, "TROMPETAS DE GABRIEL", 13.0 * scale, Color::hex("#FFF8D6"), true);
    }

    // B. Boquilla torneada de plata
    let mp_x = -l * 0.52;
    let mp_y = -18.0 * scale;
    canvas.fill_circle(mp_x - 16.0 * scale, mp_y, 13.0 * scale, silver_key);
    canvas.stroke_circle(mp_x - 16.0 * scale, mp_y, 13.0 * scale, ink_dark, 1.8 * scale);
    canvas.fill_circle(mp_x - 16.0 * scale, mp_y, 8.5 * scale, gold_high);

    // C. Leadpipe superior
    let lead_x1 = mp_x + 10.0 * scale;
    let lead_x2 = l * 0.20;
    canvas.fill_rect(lead_x1, mp_y - tube_r, lead_x2 - lead_x1, tube_r * 2.0, gold_base);
    canvas.stroke_line_fine(lead_x1, mp_y - tube_r, lead_x2, mp_y - tube_r, ink_dark, 1.8 * scale);
    canvas.stroke_line_fine(lead_x1, mp_y + tube_r, lead_x2, mp_y + tube_r, gold_deep, 1.8 * scale);
    canvas.stroke_line_fine(lead_x1, mp_y - 2.5 * scale, lead_x2, mp_y - 2.5 * scale, gold_high, 2.5 * scale);

    // D. Bomba de afinación principal curva 180°
    let slide_x = lead_x2;
    let lower_y = 18.0 * scale;
    let mut slide_pb = tiny_skia::PathBuilder::new();
    slide_pb.move_to(slide_x, mp_y - tube_r);
    slide_pb.cubic_to(slide_x + 65.0 * scale, mp_y - tube_r, slide_x + 65.0 * scale, lower_y + tube_r, slide_x, lower_y + tube_r);
    slide_pb.line_to(slide_x, lower_y - tube_r);
    slide_pb.cubic_to(slide_x + 45.0 * scale, lower_y - tube_r, slide_x + 45.0 * scale, mp_y + tube_r, slide_x, mp_y + tube_r);
    slide_pb.close();
    if let Some(sp) = slide_pb.finish() {
        canvas.fill_path(&sp, gold_base);
        canvas.stroke_path_fine(&sp, ink_dark, 1.8 * scale);
    }

    let ret_x = -l * 0.40;
    canvas.fill_rect(ret_x, lower_y - tube_r, slide_x - ret_x, tube_r * 2.0, gold_base);
    canvas.stroke_line_fine(ret_x, lower_y - tube_r, slide_x, lower_y - tube_r, ink_dark, 1.8 * scale);
    canvas.stroke_line_fine(ret_x, lower_y + tube_r, slide_x, lower_y + tube_r, gold_deep, 1.8 * scale);
    canvas.stroke_line_fine(ret_x, lower_y - 2.5 * scale, slide_x, lower_y - 2.5 * scale, gold_high, 2.5 * scale);

    // E. Bloque de 3 pistones mecánicos
    let valve_start_x = -l * 0.08;
    let valve_spacing = 36.0 * scale;
    let valve_h = 88.0 * scale;
    let valve_top_y = -42.0 * scale;
    let valve_w = 19.0 * scale;

    for v in 0..3 {
        let vx = valve_start_x + (v as f32) * valve_spacing;
        let press_offset = if is_playing {
            let cycle = (tau * 16.0 + (v as f32) * 1.5).sin();
            if cycle > 0.2 { 12.0 * scale } else { 0.0 }
        } else {
            0.0
        };

        canvas.fill_rect(vx - valve_w * 0.5, valve_top_y, valve_w, valve_h, gold_base);
        canvas.stroke_rect(vx - valve_w * 0.5, valve_top_y, valve_w, valve_h, ink_dark, 1.6 * scale);
        canvas.stroke_line_fine(vx - valve_w * 0.15, valve_top_y, vx - valve_w * 0.15, valve_top_y + valve_h, gold_high, 3.0 * scale);

        let stem_top = valve_top_y - 30.0 * scale + press_offset;
        canvas.fill_rect(vx - 3.0 * scale, stem_top, 6.0 * scale, 24.0 * scale, silver_key);
        canvas.stroke_rect(vx - 3.0 * scale, stem_top, 6.0 * scale, 24.0 * scale, ink_dark, 1.2 * scale);

        canvas.fill_circle(vx, stem_top, 9.5 * scale, gold_deep);
        canvas.stroke_circle(vx, stem_top, 9.5 * scale, ink_dark, 1.5 * scale);
        canvas.fill_circle(vx, stem_top, 7.0 * scale, Color::hex("#FAF6ED"));
    }

    // F. Campana hiperbólica monumental
    let bow_x = ret_x;
    let mut bow_pb = tiny_skia::PathBuilder::new();
    bow_pb.move_to(bow_x, lower_y - tube_r);
    bow_pb.cubic_to(bow_x - 52.0 * scale, lower_y - tube_r, bow_x - 52.0 * scale, -tube_r, bow_x, -tube_r);
    bow_pb.line_to(bow_x, tube_r);
    bow_pb.cubic_to(bow_x - 35.0 * scale, tube_r, bow_x - 35.0 * scale, lower_y + tube_r, bow_x, lower_y + tube_r);
    bow_pb.close();
    if let Some(bp) = bow_pb.finish() {
        canvas.fill_path(&bp, gold_base);
        canvas.stroke_path_fine(&bp, ink_dark, 1.8 * scale);
    }

    let flare_start_x = l * 0.06;
    let flare_end_x = l * 0.58;

    let mut bell_pb = tiny_skia::PathBuilder::new();
    bell_pb.move_to(bow_x, -tube_r);
    bell_pb.line_to(flare_start_x, -tube_r);
    bell_pb.cubic_to(flare_start_x + 110.0 * scale, -tube_r - 3.0 * scale, flare_end_x - 85.0 * scale, -bell_lip_r * 0.55, flare_end_x, -bell_lip_r);
    bell_pb.line_to(flare_end_x, bell_lip_r);
    bell_pb.cubic_to(flare_end_x - 85.0 * scale, bell_lip_r * 0.55, flare_start_x + 110.0 * scale, tube_r + 3.0 * scale, flare_start_x, tube_r);
    bell_pb.line_to(bow_x, tube_r);
    bell_pb.close();

    if let Some(bell_path) = bell_pb.finish() {
        canvas.fill_path(&bell_path, gold_base);
        canvas.stroke_path_fine(&bell_path, ink_dark, 2.5 * scale);

        let mut gl_pb = tiny_skia::PathBuilder::new();
        gl_pb.move_to(flare_start_x, -tube_r * 0.3);
        gl_pb.cubic_to(flare_start_x + 110.0 * scale, -tube_r * 0.8, flare_end_x - 70.0 * scale, -bell_lip_r * 0.40, flare_end_x, -bell_lip_r * 0.75);
        if let Some(gp) = gl_pb.finish() {
            canvas.stroke_path_fine(&gp, Color::WHITE.with_alpha(0.85), 3.5 * scale);
        }

        draw_vector_text(canvas, flare_end_x - 115.0 * scale, 0.0, "G.G.M. MACONDO", 12.0 * scale, gold_deep, true);
    }

    canvas.fill_rect(flare_end_x - 5.0 * scale, -bell_lip_r, 10.0 * scale, bell_lip_r * 2.0, gold_deep);
    canvas.stroke_rect(flare_end_x - 5.0 * scale, -bell_lip_r, 10.0 * scale, bell_lip_r * 2.0, ink_dark, 1.8 * scale);
    canvas.stroke_line_fine(flare_end_x, -bell_lip_r + 5.0 * scale, flare_end_x, bell_lip_r - 5.0 * scale, Color::WHITE, 2.5 * scale);

    // G. Ondas acústicas esféricas colosales en BlendMode::Screen
    if is_playing {
        canvas.save();
        canvas.set_blend_mode(BlendMode::Screen);
        let wave_cyan = Color::hex("#00F5D4");
        for w in 0..6 {
            let phase = (tau * 3.5 + (w as f32) * 0.18) % 1.0;
            let wr = 50.0 * scale + phase * 420.0 * scale;
            let alpha = (1.0 - phase) * 0.85;

            let mut wp = tiny_skia::PathBuilder::new();
            wp.move_to(flare_end_x + wr * 0.4, -wr * 0.80);
            wp.cubic_to(flare_end_x + wr * 0.98, -wr * 0.38, flare_end_x + wr * 0.98, wr * 0.38, flare_end_x + wr * 0.4, wr * 0.80);
            if let Some(wpath) = wp.finish() {
                canvas.stroke_path_fine(&wpath, wave_cyan.with_alpha(alpha), 3.8 * scale);
                canvas.stroke_path_fine(&wpath, gold_high.with_alpha(alpha * 0.75), 1.6 * scale);
            }
        }
        canvas.restore();
    }

    canvas.restore();
}

// =============================================================================
// 3. CUATRO VENEZOLANO / COLOMBIANO REALISTA (La Tristeza de Aureliano)
// =============================================================================

pub fn draw_master_cuatro(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
) {
    let wood_cedar = Color::hex("#C47D3B");
    let wood_walnut = Color::hex("#4A2511");
    let soundboard_spruce = Color::hex("#E6C285");
    let purfling_ebony = Color::hex("#1F140A");
    let bone_white = Color::hex("#FFFDF5");
    let gold_fret = Color::hex("#DDA823");
    let pickguard_col = Color::hex("#38160B");

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    let body_w = 230.0 * scale;
    let body_h = 340.0 * scale;

    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(0.0, -body_h * 0.48);
    body_pb.cubic_to(-body_w * 0.42, -body_h * 0.48, -body_w * 0.46, -body_h * 0.22, -body_w * 0.32, -body_h * 0.08);
    body_pb.cubic_to(-body_w * 0.24, 0.0, -body_w * 0.28, body_h * 0.08, -body_w * 0.48, body_h * 0.26);
    body_pb.cubic_to(-body_w * 0.54, body_h * 0.45, -body_w * 0.25, body_h * 0.50, 0.0, body_h * 0.50);
    body_pb.cubic_to(body_w * 0.25, body_h * 0.50, body_w * 0.54, body_h * 0.45, body_w * 0.48, body_h * 0.26);
    body_pb.cubic_to(body_w * 0.28, body_h * 0.08, body_w * 0.24, 0.0, body_w * 0.32, -body_h * 0.08);
    body_pb.cubic_to(body_w * 0.46, -body_h * 0.22, body_w * 0.42, -body_h * 0.48, 0.0, -body_h * 0.48);
    body_pb.close();

    if let Some(bp) = body_pb.finish() {
        canvas.fill_path(&bp, soundboard_spruce);
        canvas.stroke_path_fine(&bp, purfling_ebony, 4.5 * scale);
        canvas.stroke_path_fine(&bp, wood_walnut, 2.2 * scale);

        for i in -10..=10 {
            let gx = (i as f32) * 11.0 * scale;
            canvas.stroke_line_fine(gx, -body_h * 0.44, gx, body_h * 0.46, Color::hex("#CFAF7A").with_alpha(0.35), 0.9 * scale);
        }
    }

    let soundhole_y = -body_h * 0.12;
    let soundhole_r = 42.0 * scale;
    canvas.stroke_circle(0.0, soundhole_y, soundhole_r + 16.0 * scale, wood_walnut, 3.5 * scale);
    canvas.stroke_circle(0.0, soundhole_y, soundhole_r + 10.0 * scale, wood_cedar, 4.5 * scale);
    canvas.stroke_circle(0.0, soundhole_y, soundhole_r + 4.5 * scale, purfling_ebony, 2.5 * scale);
    canvas.fill_circle(0.0, soundhole_y, soundhole_r, Color::hex("#120B06"));
    canvas.stroke_circle(0.0, soundhole_y, soundhole_r, purfling_ebony, 2.2 * scale);

    let mut pg_pb = tiny_skia::PathBuilder::new();
    pg_pb.move_to(-28.0 * scale, -body_h * 0.44);
    pg_pb.line_to(28.0 * scale, -body_h * 0.44);
    pg_pb.cubic_to(body_w * 0.38, -body_h * 0.35, body_w * 0.34, -body_h * 0.12, 18.0 * scale, soundhole_y);
    pg_pb.cubic_to(-soundhole_r * 0.6, soundhole_y - 18.0 * scale, -28.0 * scale, -body_h * 0.30, -28.0 * scale, -body_h * 0.44);
    pg_pb.close();
    if let Some(pg) = pg_pb.finish() {
        canvas.fill_path(&pg, pickguard_col);
        canvas.stroke_path_fine(&pg, wood_walnut, 1.8 * scale);
    }

    let bridge_y = body_h * 0.22;
    let bridge_w = 105.0 * scale;
    let bridge_h = 26.0 * scale;
    canvas.fill_rect(-bridge_w * 0.5, bridge_y - bridge_h * 0.5, bridge_w, bridge_h, wood_walnut);
    canvas.stroke_rect(-bridge_w * 0.5, bridge_y - bridge_h * 0.5, bridge_w, bridge_h, purfling_ebony, 1.8 * scale);
    canvas.fill_rect(-bridge_w * 0.35, bridge_y - 3.5 * scale, bridge_w * 0.70, 7.0 * scale, bone_white);

    let neck_w = 38.0 * scale;
    let neck_h = 260.0 * scale;
    let neck_top_y = -body_h * 0.48 - neck_h;

    canvas.fill_rect(-neck_w * 0.5, neck_top_y, neck_w, neck_h + body_h * 0.36, wood_walnut);
    canvas.stroke_rect(-neck_w * 0.5, neck_top_y, neck_w, neck_h + body_h * 0.36, purfling_ebony, 2.2 * scale);

    for f in 0..14 {
        let fy = neck_top_y + (f as f32) * (neck_h / 13.5);
        canvas.stroke_line_fine(-neck_w * 0.5, fy, neck_w * 0.5, fy, gold_fret, 1.8 * scale);
    }

    let head_w = 50.0 * scale;
    let head_h = 105.0 * scale;
    let head_top_y = neck_top_y - head_h;

    let mut head_pb = tiny_skia::PathBuilder::new();
    head_pb.move_to(-neck_w * 0.5, neck_top_y);
    head_pb.line_to(-head_w * 0.5, head_top_y + 22.0 * scale);
    head_pb.cubic_to(-head_w * 0.55, head_top_y, 0.0, head_top_y - 12.0 * scale, head_w * 0.55, head_top_y);
    head_pb.line_to(head_w * 0.5, head_top_y + 22.0 * scale);
    head_pb.line_to(neck_w * 0.5, neck_top_y);
    head_pb.close();
    if let Some(hp) = head_pb.finish() {
        canvas.fill_path(&hp, wood_cedar);
        canvas.stroke_path_fine(&hp, purfling_ebony, 2.2 * scale);
    }

    for p in 0..4 {
        let side = if p % 2 == 0 { -1.0_f32 } else { 1.0_f32 };
        let py = head_top_y + 28.0 * scale + (p / 2) as f32 * 42.0 * scale;
        let px = side * (head_w * 0.5 + 16.0 * scale);
        canvas.fill_circle(px, py, 8.5 * scale, bone_white);
        canvas.stroke_circle(px, py, 8.5 * scale, purfling_ebony, 1.4 * scale);
        canvas.stroke_line_fine(side * (head_w * 0.4), py, px, py, gold_fret, 3.0 * scale);
    }

    for s in 0..4 {
        let sx = -neck_w * 0.35 + (s as f32) * (neck_w * 0.70 / 3.0);
        let vib = (tau * 24.0 * PI + s as f32 * 1.8).sin() * 2.8 * scale;
        let mut string_pb = tiny_skia::PathBuilder::new();
        string_pb.move_to(sx, neck_top_y + 10.0 * scale);
        string_pb.cubic_to(sx + vib, soundhole_y, sx + vib * 0.5, bridge_y - 50.0 * scale, sx, bridge_y);
        if let Some(stp) = string_pb.finish() {
            canvas.stroke_path_fine(&stp, Color::WHITE.with_alpha(0.92), 1.8 * scale);
        }
    }

    canvas.restore();
}

// =============================================================================
// 4. VIOLÍN BARROCO ITALIANO REALISTA (La Belleza de Remedios)
// =============================================================================

pub fn draw_master_violin(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
) {
    let amber_varnish = Color::hex("#C85A17");
    let dark_maple = Color::hex("#5A240A");
    let spruce_face = Color::hex("#E8A856");
    let ebony_black = Color::hex("#120E0B");
    let bone_white = Color::hex("#FFFDF0");

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    let body_w = 210.0 * scale;
    let body_h = 340.0 * scale;

    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(0.0, -body_h * 0.50);
    body_pb.cubic_to(-body_w * 0.45, -body_h * 0.50, -body_w * 0.48, -body_h * 0.28, -body_w * 0.38, -body_h * 0.16);
    body_pb.line_to(-body_w * 0.42, -body_h * 0.14);
    body_pb.cubic_to(-body_w * 0.26, -body_h * 0.05, -body_w * 0.26, body_h * 0.05, -body_w * 0.42, body_h * 0.14);
    body_pb.line_to(-body_w * 0.38, body_h * 0.16);
    body_pb.cubic_to(-body_w * 0.50, body_h * 0.28, -body_w * 0.45, body_h * 0.50, 0.0, body_h * 0.50);
    body_pb.cubic_to(body_w * 0.45, body_h * 0.50, body_w * 0.50, body_h * 0.28, body_w * 0.38, body_h * 0.16);
    body_pb.line_to(body_w * 0.42, body_h * 0.14);
    body_pb.cubic_to(body_w * 0.26, body_h * 0.05, body_w * 0.26, -body_h * 0.05, body_w * 0.42, -body_h * 0.14);
    body_pb.line_to(body_w * 0.38, -body_h * 0.16);
    body_pb.cubic_to(body_w * 0.48, -body_h * 0.28, body_w * 0.45, -body_h * 0.50, 0.0, -body_h * 0.50);
    body_pb.close();

    if let Some(bp) = body_pb.finish() {
        canvas.fill_path(&bp, spruce_face);
        canvas.stroke_path_fine(&bp, dark_maple, 4.0 * scale);
        canvas.stroke_path_fine(&bp, ebony_black, 1.4 * scale);
    }

    for &side in &[-1.0_f32, 1.0_f32] {
        let fx = side * 48.0 * scale;
        let mut f_pb = tiny_skia::PathBuilder::new();
        f_pb.move_to(fx, -48.0 * scale);
        f_pb.cubic_to(fx + side * 16.0 * scale, -22.0 * scale, fx - side * 16.0 * scale, 22.0 * scale, fx + side * 6.0 * scale, 48.0 * scale);
        if let Some(fp) = f_pb.finish() {
            canvas.stroke_path_fine(&fp, ebony_black, 5.5 * scale);
            canvas.fill_circle(fx, -48.0 * scale, 5.0 * scale, ebony_black);
            canvas.fill_circle(fx + side * 6.0 * scale, 48.0 * scale, 5.5 * scale, ebony_black);
        }
    }

    canvas.fill_rect(-26.0 * scale, 14.0 * scale, 52.0 * scale, 9.0 * scale, Color::hex("#D2A66C"));
    canvas.stroke_rect(-26.0 * scale, 14.0 * scale, 52.0 * scale, 9.0 * scale, dark_maple, 1.6 * scale);

    let mut tail_pb = tiny_skia::PathBuilder::new();
    tail_pb.move_to(-18.0 * scale, 38.0 * scale);
    tail_pb.line_to(18.0 * scale, 38.0 * scale);
    tail_pb.line_to(9.0 * scale, body_h * 0.46);
    tail_pb.line_to(-9.0 * scale, body_h * 0.46);
    tail_pb.close();
    if let Some(tp) = tail_pb.finish() {
        canvas.fill_path(&tp, ebony_black);
    }
    canvas.fill_circle(-body_w * 0.22, body_h * 0.38, 30.0 * scale, ebony_black);

    let neck_w = 28.0 * scale;
    let neck_h = 240.0 * scale;
    let neck_top_y = -body_h * 0.50 - neck_h;
    canvas.fill_rect(-neck_w * 0.5, neck_top_y, neck_w, neck_h + 130.0 * scale, ebony_black);

    let scroll_y = neck_top_y - 75.0 * scale;
    canvas.fill_circle(0.0, scroll_y, 24.0 * scale, dark_maple);
    canvas.stroke_circle(0.0, scroll_y, 24.0 * scale, ebony_black, 2.2 * scale);
    canvas.stroke_circle(0.0, scroll_y, 15.0 * scale, amber_varnish, 1.6 * scale);

    for p in 0..4 {
        let side = if p % 2 == 0 { -1.0_f32 } else { 1.0_f32 };
        let py = neck_top_y - 22.0 * scale - (p as f32) * 13.0 * scale;
        canvas.fill_circle(side * 30.0 * scale, py, 5.5 * scale, ebony_black);
        canvas.stroke_line_fine(0.0, py, side * 30.0 * scale, py, ebony_black, 2.8 * scale);
    }

    for s in 0..4 {
        let sx = -neck_w * 0.35 + (s as f32) * (neck_w * 0.70 / 3.0);
        canvas.stroke_line_fine(sx, neck_top_y + 10.0 * scale, sx * 0.7, 38.0 * scale, Color::WHITE.with_alpha(0.85), 1.4 * scale);
    }

    let bow_y = 6.0 * scale;
    let bow_x_offset = (tau * 16.0 * PI).sin() * 75.0 * scale;
    canvas.stroke_line_fine(-190.0 * scale + bow_x_offset, bow_y, 190.0 * scale + bow_x_offset, bow_y, bone_white, 2.2 * scale);
    canvas.stroke_line_fine(-190.0 * scale + bow_x_offset, bow_y - 9.0 * scale, 190.0 * scale + bow_x_offset, bow_y - 9.0 * scale, dark_maple, 3.2 * scale);

    canvas.restore();
}

// =============================================================================
// 5. ACORDEÓN HOHNER DIATÓNICO MONUMENTAL (La Cumbia de Macondo)
// =============================================================================

pub fn draw_master_accordion(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let red_pearloid = Color::hex("#B81D24");
    let chrome_silver = Color::hex("#E6ECF0");
    let gold_trim = Color::hex("#DDA823");
    let bellows_cream = Color::hex("#FBF7EB");
    let bellows_shade = Color::hex("#383226");
    let ebony_black = Color::hex("#15120E");
    let pearl_white = Color::hex("#FFFDF7");

    canvas.save();
    canvas.translate(cx, cy);

    let box_h = 260.0 * scale;
    let box_w = 82.0 * scale;

    let bellows_spread = 150.0 * scale + (tau * 8.0 * PI).sin().abs() * 75.0 * scale;

    let folds = 20;
    for f in 0..folds {
        let t1 = f as f32 / folds as f32;
        let t2 = (f + 1) as f32 / folds as f32;
        let x1 = -bellows_spread * 0.5 + t1 * bellows_spread;
        let x2 = -bellows_spread * 0.5 + t2 * bellows_spread;
        let is_even = f % 2 == 0;

        let mut fold_pb = tiny_skia::PathBuilder::new();
        fold_pb.move_to(x1, -box_h * 0.44);
        fold_pb.line_to(x2, -box_h * 0.44);
        fold_pb.line_to(x2,  box_h * 0.44);
        fold_pb.line_to(x1,  box_h * 0.44);
        fold_pb.close();

        if let Some(fp) = fold_pb.finish() {
            canvas.fill_path(&fp, if is_even { bellows_cream } else { bellows_shade });
            canvas.stroke_path_fine(&fp, ebony_black, 1.4 * scale);

            canvas.fill_rect(x1 - 2.5 * scale, -box_h * 0.44 - 4.0 * scale, 6.0 * scale, 14.0 * scale, chrome_silver);
            canvas.fill_rect(x1 - 2.5 * scale,  box_h * 0.44 - 10.0 * scale, 6.0 * scale, 14.0 * scale, chrome_silver);
        }
    }

    let left_x = -bellows_spread * 0.5 - box_w;
    canvas.fill_rect(left_x, -box_h * 0.50, box_w, box_h, red_pearloid);
    canvas.stroke_rect(left_x, -box_h * 0.50, box_w, box_h, ebony_black, 2.8 * scale);
    canvas.stroke_rect(left_x + 5.0 * scale, -box_h * 0.50 + 5.0 * scale, box_w - 10.0 * scale, box_h - 10.0 * scale, gold_trim, 2.0 * scale);

    for r in 0..2 {
        for c in 0..4 {
            let bx = left_x + 24.0 * scale + (r as f32) * 26.0 * scale;
            let by = -box_h * 0.25 + (c as f32) * 38.0 * scale;
            canvas.fill_circle(bx, by, 8.0 * scale, pearl_white);
            canvas.stroke_circle(bx, by, 8.0 * scale, ebony_black, 1.4 * scale);
        }
    }

    let right_x = bellows_spread * 0.5;
    let right_w = 105.0 * scale;
    canvas.fill_rect(right_x, -box_h * 0.50, right_w, box_h, red_pearloid);
    canvas.stroke_rect(right_x, -box_h * 0.50, right_w, box_h, ebony_black, 2.8 * scale);
    canvas.stroke_rect(right_x + 5.0 * scale, -box_h * 0.50 + 5.0 * scale, right_w - 10.0 * scale, box_h - 10.0 * scale, gold_trim, 2.0 * scale);

    let grille_x = right_x + 9.0 * scale;
    let grille_w = 42.0 * scale;
    canvas.fill_rect(grille_x, -box_h * 0.40, grille_w, box_h * 0.80, chrome_silver);
    canvas.stroke_rect(grille_x, -box_h * 0.40, grille_w, box_h * 0.80, ebony_black, 1.6 * scale);
    for slot in 0..14 {
        let sy = -box_h * 0.35 + (slot as f32) * 13.0 * scale;
        canvas.stroke_line_fine(grille_x + 4.0 * scale, sy, grille_x + grille_w - 4.0 * scale, sy, ebony_black, 2.5 * scale);
    }

    let key_x = right_x + 62.0 * scale;
    for col in 0..3 {
        let kx = key_x + (col as f32) * 13.0 * scale;
        for row in 0..10 {
            let ky = -box_h * 0.38 + (row as f32) * 19.0 * scale + (col as f32) * 9.0 * scale;
            let pressed = (tau * 16.0 + (row + col * 3) as f32 * 0.8).sin() > 0.4;
            let press_dx = if pressed { -2.5 * scale } else { 0.0 };

            canvas.fill_circle(kx + press_dx, ky, 5.5 * scale, pearl_white);
            canvas.stroke_circle(kx + press_dx, ky, 5.5 * scale, ebony_black, 1.2 * scale);
        }
    }

    canvas.restore();
}

// =============================================================================
// 6. RELOJ DE ARENA COLOSAL Y MONUMENTAL (Úrsula y el Tiempo en Carrusel)
// =============================================================================

pub fn draw_master_colossal_hourglass(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let wood_mahogany = Color::hex("#3B180A");
    let brass_polished = Color::hex("#E5B83B");
    let brass_shade = Color::hex("#8C6B14");
    let glass_highlight = Color::hex("#FFFFFF").with_alpha(0.85);
    let sand_gold = Color::hex("#FFD700");
    let sand_warm = Color::hex("#F2BE22");

    canvas.save();
    canvas.translate(cx, cy);

    let hg_h = 620.0 * scale;
    let hg_w = 310.0 * scale;

    for &side_y in &[-hg_h * 0.50, hg_h * 0.50 - 32.0 * scale] {
        canvas.fill_rect(-hg_w * 0.60, side_y, hg_w * 1.20, 32.0 * scale, wood_mahogany);
        canvas.stroke_rect(-hg_w * 0.60, side_y, hg_w * 1.20, 32.0 * scale, brass_polished, 3.5 * scale);
        for r in 0..11 {
            let rx = -hg_w * 0.52 + (r as f32) * (hg_w * 1.04 / 10.0);
            canvas.fill_circle(rx, side_y + 16.0 * scale, 4.0 * scale, brass_shade);
        }
    }

    for &col_x in &[-hg_w * 0.52, 0.0, hg_w * 0.52] {
        canvas.fill_rect(col_x - 9.0 * scale, -hg_h * 0.50 + 32.0 * scale, 18.0 * scale, hg_h - 64.0 * scale, brass_polished);
        canvas.stroke_rect(col_x - 9.0 * scale, -hg_h * 0.50 + 32.0 * scale, 18.0 * scale, hg_h - 64.0 * scale, brass_shade, 1.8 * scale);
        for &cy in &[-hg_h * 0.50 + 40.0 * scale, hg_h * 0.50 - 40.0 * scale] {
            canvas.fill_circle(col_x, cy, 14.0 * scale, brass_shade);
        }
    }

    let bulb_r = hg_w * 0.46;
    let bulb_upper_cy = -hg_h * 0.22;
    let bulb_lower_cy =  hg_h * 0.22;

    canvas.fill_circle(0.0, bulb_upper_cy, bulb_r, Color::hex("#E8F4F8").with_alpha(0.40));
    canvas.stroke_circle(0.0, bulb_upper_cy, bulb_r, Color::hex("#6B8A99"), 2.8 * scale);

    canvas.fill_circle(0.0, bulb_lower_cy, bulb_r, Color::hex("#E8F4F8").with_alpha(0.40));
    canvas.stroke_circle(0.0, bulb_lower_cy, bulb_r, Color::hex("#6B8A99"), 2.8 * scale);

    let mut neck_pb = tiny_skia::PathBuilder::new();
    neck_pb.move_to(-22.0 * scale, -28.0 * scale);
    neck_pb.cubic_to(-7.0 * scale, -6.0 * scale, -7.0 * scale, 6.0 * scale, -22.0 * scale, 28.0 * scale);
    neck_pb.line_to(22.0 * scale, 28.0 * scale);
    neck_pb.cubic_to(7.0 * scale, 6.0 * scale, 7.0 * scale, -6.0 * scale, 22.0 * scale, -28.0 * scale);
    neck_pb.close();
    if let Some(np) = neck_pb.finish() {
        canvas.fill_path(&np, Color::hex("#E8F4F8").with_alpha(0.35));
        canvas.stroke_path_fine(&np, Color::hex("#6B8A99"), 2.2 * scale);
    }

    let sand_cycle = (tau * 0.25) % 1.0;

    let upper_fill = (1.0 - sand_cycle) * bulb_r * 0.85;
    let mut upper_sand_pb = tiny_skia::PathBuilder::new();
    upper_sand_pb.move_to(-bulb_r * 0.85, bulb_upper_cy);
    upper_sand_pb.cubic_to(-bulb_r * 0.5, bulb_upper_cy + upper_fill, bulb_r * 0.5, bulb_upper_cy + upper_fill, bulb_r * 0.85, bulb_upper_cy);
    upper_sand_pb.cubic_to(bulb_r * 0.4, bulb_upper_cy - upper_fill * 0.4, -bulb_r * 0.4, bulb_upper_cy - upper_fill * 0.4, -bulb_r * 0.85, bulb_upper_cy);
    upper_sand_pb.close();
    if let Some(usp) = upper_sand_pb.finish() {
        canvas.fill_path(&usp, sand_gold);
    }

    for sy in 0..38 {
        let py = -10.0 * scale + (sy as f32) * (bulb_lower_cy / 22.0);
        let flicker = ((tau * 48.0 * PI + sy as f32 * 0.7).sin()) * 2.2 * scale;
        canvas.fill_circle(flicker, py, 4.0 * scale, sand_warm);
    }

    let lower_fill = sand_cycle * bulb_r * 0.90;
    let mut lower_sand_pb = tiny_skia::PathBuilder::new();
    lower_sand_pb.move_to(-bulb_r * 0.85, bulb_lower_cy + bulb_r * 0.85);
    lower_sand_pb.line_to(0.0, bulb_lower_cy + bulb_r * 0.85 - lower_fill);
    lower_sand_pb.line_to(bulb_r * 0.85, bulb_lower_cy + bulb_r * 0.85);
    lower_sand_pb.close();
    if let Some(lsp) = lower_sand_pb.finish() {
        canvas.fill_path(&lsp, sand_gold);
        canvas.stroke_path_fine(&lsp, sand_warm, 2.2 * scale);
    }

    let mut gl1_pb = tiny_skia::PathBuilder::new();
    gl1_pb.move_to(-bulb_r * 0.75, bulb_upper_cy - bulb_r * 0.45);
    gl1_pb.cubic_to(-bulb_r * 0.85, bulb_upper_cy, -bulb_r * 0.75, bulb_upper_cy + bulb_r * 0.45, -bulb_r * 0.45, bulb_upper_cy + bulb_r * 0.75);
    if let Some(gp1) = gl1_pb.finish() {
        canvas.stroke_path_fine(&gp1, glass_highlight, 4.5 * scale);
    }

    canvas.restore();
}

// =============================================================================
// 7. REMEDIOS LA BELLA CON SÁBANAS MONUMENTALES (Ascensión Etérea)
// =============================================================================

pub fn draw_master_remedios_ascension(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let sheet_pure = Color::hex("#FFFFFF");
    let sheet_mid = Color::hex("#F6F3EA");
    let sheet_shade = Color::hex("#DDD6C6");
    let sheet_deep = Color::hex("#C4BAA7");
    let gold_thread = Color::hex("#E0B434");
    let gold_bright = Color::hex("#FFD700");
    let skin_base = Color::hex("#FDF0E7");
    let skin_shade = Color::hex("#F4D8C8");
    let skin_blush = Color::hex("#E8A89A");
    let hair_jet = Color::hex("#120D09");
    let hair_highlight = Color::hex("#2C221C");
    let butterfly_yellow = Color::hex("#FFC800");

    canvas.save();
    canvas.translate(cx, cy);

    let asc_bob = (tau * 4.0 * PI).sin() * 18.0 * scale;
    canvas.translate(0.0, asc_bob);

    // ===== 1. AURA CELESTIAL Y MANDORLA DORADA =====
    let halo_cx = 0.0;
    let halo_cy = -120.0 * scale;
    let halo_r = 240.0 * scale;

    // Resplandor cálido
    canvas.fill_circle(halo_cx, halo_cy, halo_r, Color::hex("#FFF3B0").with_alpha(0.22));
    canvas.fill_circle(halo_cx, halo_cy, halo_r * 0.72, Color::hex("#FFE680").with_alpha(0.28));

    // Anillos concéntricos de geometría sagrada
    canvas.stroke_circle(halo_cx, halo_cy, halo_r, gold_thread.with_alpha(0.70), 2.5 * scale);
    canvas.stroke_circle(halo_cx, halo_cy, halo_r * 0.86, gold_thread.with_alpha(0.40), 1.4 * scale);
    canvas.stroke_circle(halo_cx, halo_cy, halo_r * 0.55, gold_bright.with_alpha(0.35), 1.2 * scale);

    // 36 Rayos de luz dorada alternando largos y cortos
    for r in 0..36 {
        let ra = (r as f32 / 36.0) * PI * 2.0 + tau * 0.35;
        let is_long = r % 2 == 0;
        let r1 = halo_r * 0.60;
        let r2 = if is_long { halo_r * 1.18 } else { halo_r * 0.95 };
        let th = if is_long { 2.2 * scale } else { 1.2 * scale };
        let col = if is_long { gold_bright } else { gold_thread.with_alpha(0.55) };
        canvas.stroke_line_fine(
            halo_cx + ra.cos() * r1,
            halo_cy + ra.sin() * r1,
            halo_cx + ra.cos() * r2,
            halo_cy + ra.sin() * r2,
            col,
            th,
        );
    }

    // ===== 2. SÁBANAS DE BRAMANTE EN SEGUNDO PLANO (Detrás de la figura) =====
    // Sábanas ondeantes que se elevan en espirales hacia arriba
    for layer in 0..3 {
        let l_t = layer as f32;
        let wave_phase = tau * 5.0 * PI + l_t * 1.8;
        let w_span = (280.0 + l_t * 70.0) * scale;
        let base_y = 60.0 * scale + l_t * 75.0 * scale;

        let mut bg_sheet = tiny_skia::PathBuilder::new();
        bg_sheet.move_to(-w_span * 0.85, base_y);
        bg_sheet.cubic_to(
            -w_span * 0.50, base_y - 140.0 * scale + (wave_phase).sin() * 45.0 * scale,
            -w_span * 0.15, base_y - 210.0 * scale + (wave_phase * 1.1).cos() * 40.0 * scale,
            -80.0 * scale, base_y - 320.0 * scale,
        );
        bg_sheet.cubic_to(
            w_span * 0.10, base_y - 260.0 * scale - (wave_phase).sin() * 35.0 * scale,
            w_span * 0.45, base_y - 120.0 * scale + (wave_phase).cos() * 50.0 * scale,
            w_span * 0.85, base_y - 20.0 * scale,
        );
        bg_sheet.cubic_to(
            w_span * 0.45, base_y + 90.0 * scale + (wave_phase).sin() * 40.0 * scale,
            -w_span * 0.40, base_y + 110.0 * scale - (wave_phase).cos() * 40.0 * scale,
            -w_span * 0.85, base_y,
        );
        bg_sheet.close();

        if let Some(sp) = bg_sheet.finish() {
            canvas.fill_path(&sp, if layer % 2 == 0 { sheet_mid } else { sheet_shade });
            canvas.stroke_path_fine(&sp, sheet_deep, 2.0 * scale);

            // Costuras y dobladillos bordados en hilo de oro
            for hem in 0..3 {
                let hy = base_y - 60.0 * scale + hem as f32 * 45.0 * scale;
                canvas.stroke_line_fine(
                    -w_span * 0.65,
                    hy + (wave_phase + hem as f32).sin() * 25.0 * scale,
                    w_span * 0.65,
                    hy - (wave_phase + hem as f32).cos() * 25.0 * scale,
                    gold_thread.with_alpha(0.55),
                    1.4 * scale,
                );
            }
        }
    }

    // ===== 3. CABELLERA OSCURA ONDULANTE EN EL VIENTO (Capa trasera) =====
    let hair_wave = (tau * 8.0 * PI).sin() * 22.0 * scale;
    let mut bg_hair = tiny_skia::PathBuilder::new();
    bg_hair.move_to(-25.0 * scale, -135.0 * scale);
    bg_hair.cubic_to(
        -85.0 * scale + hair_wave, -90.0 * scale,
        -125.0 * scale - hair_wave, 20.0 * scale,
        -95.0 * scale + hair_wave, 160.0 * scale,
    );
    bg_hair.cubic_to(
        -55.0 * scale, 130.0 * scale,
        -20.0 * scale, 30.0 * scale,
        -10.0 * scale, -80.0 * scale,
    );
    bg_hair.close();
    if let Some(hp) = bg_hair.finish() {
        canvas.fill_path(&hp, hair_jet);
        canvas.stroke_path_fine(&hp, hair_highlight, 1.8 * scale);
    }

    // Mechón derecho ondeante
    let mut bg_hair_r = tiny_skia::PathBuilder::new();
    bg_hair_r.move_to(20.0 * scale, -135.0 * scale);
    bg_hair_r.cubic_to(
        75.0 * scale - hair_wave * 0.8, -80.0 * scale,
        110.0 * scale + hair_wave, 40.0 * scale,
        70.0 * scale - hair_wave, 180.0 * scale,
    );
    bg_hair_r.cubic_to(
        40.0 * scale, 140.0 * scale,
        15.0 * scale, 20.0 * scale,
        5.0 * scale, -80.0 * scale,
    );
    bg_hair_r.close();
    if let Some(hp) = bg_hair_r.finish() {
        canvas.fill_path(&hp, hair_jet);
        canvas.stroke_path_fine(&hp, hair_highlight, 1.8 * scale);
    }

    // ===== 4. VESTIDO DE BRAMANTE Y CUERPO ESCULTÓRICO =====
    // Falda monumental con pliegues diagonales barrocos
    let mut dress_pb = tiny_skia::PathBuilder::new();
    dress_pb.move_to(-38.0 * scale, -45.0 * scale); // Cintura
    dress_pb.line_to(38.0 * scale, -45.0 * scale);
    dress_pb.cubic_to(
        75.0 * scale, 50.0 * scale,
        115.0 * scale + (tau * 6.0).sin() * 20.0 * scale, 160.0 * scale,
        70.0 * scale, 270.0 * scale,
    );
    dress_pb.cubic_to(
        15.0 * scale, 295.0 * scale,
        -40.0 * scale, 290.0 * scale,
        -95.0 * scale - (tau * 6.0).cos() * 20.0 * scale, 250.0 * scale,
    );
    dress_pb.cubic_to(
        -90.0 * scale, 140.0 * scale,
        -65.0 * scale, 40.0 * scale,
        -38.0 * scale, -45.0 * scale,
    );
    dress_pb.close();
    if let Some(dp) = dress_pb.finish() {
        canvas.fill_path(&dp, sheet_pure);
        canvas.stroke_path_fine(&dp, sheet_shade, 2.5 * scale);

        // Pliegues verticales profundos en la falda
        for f in 0..7 {
            let t = f as f32 / 6.0;
            let fx_top = -32.0 * scale + t * 64.0 * scale;
            let fx_bot = -80.0 * scale + t * 150.0 * scale;
            let wave = ((tau * 5.0 + t * 3.0).sin()) * 14.0 * scale;
            let mut fold_pb = tiny_skia::PathBuilder::new();
            fold_pb.move_to(fx_top, -45.0 * scale);
            fold_pb.cubic_to(fx_top + wave * 0.5, 60.0 * scale, fx_bot + wave, 160.0 * scale, fx_bot, 270.0 * scale);
            if let Some(fold) = fold_pb.finish() {
                canvas.stroke_path_fine(&fold, if f % 2 == 0 { sheet_shade } else { sheet_deep }, 2.0 * scale);
            }
        }

        // Cenefa de encaje dorado en el bajo del vestido
        let mut hem_pb = tiny_skia::PathBuilder::new();
        hem_pb.move_to(-95.0 * scale, 250.0 * scale);
        hem_pb.cubic_to(-40.0 * scale, 290.0 * scale, 15.0 * scale, 295.0 * scale, 70.0 * scale, 270.0 * scale);
        if let Some(hp) = hem_pb.finish() {
            canvas.stroke_path_fine(&hp, gold_bright, 3.5 * scale);
            canvas.stroke_path_fine(&hp, gold_thread, 1.5 * scale);
        }
    }

    // Torso / Corsé ceñido con encaje
    let mut corset_pb = tiny_skia::PathBuilder::new();
    corset_pb.move_to(-34.0 * scale, -45.0 * scale);
    corset_pb.line_to(-42.0 * scale, -95.0 * scale); // Pecho / hombros
    corset_pb.cubic_to(-20.0 * scale, -108.0 * scale, 20.0 * scale, -108.0 * scale, 42.0 * scale, -95.0 * scale);
    corset_pb.line_to(34.0 * scale, -45.0 * scale);
    corset_pb.close();
    if let Some(cp) = corset_pb.finish() {
        canvas.fill_path(&cp, sheet_pure);
        canvas.stroke_path_fine(&cp, gold_thread, 2.0 * scale);

        // Bordado central del corsé
        canvas.stroke_line_fine(0.0, -100.0 * scale, 0.0, -45.0 * scale, gold_bright, 2.2 * scale);
        for l in 0..5 {
            let ly = -90.0 * scale + (l as f32) * 10.0 * scale;
            canvas.stroke_line_fine(-14.0 * scale, ly, 14.0 * scale, ly, gold_thread, 1.4 * scale);
        }
    }

    // Cuello esbelto y escote
    let mut neck_pb = tiny_skia::PathBuilder::new();
    neck_pb.move_to(-16.0 * scale, -95.0 * scale);
    neck_pb.line_to(-13.0 * scale, -128.0 * scale);
    neck_pb.line_to(13.0 * scale, -128.0 * scale);
    neck_pb.line_to(16.0 * scale, -95.0 * scale);
    neck_pb.close();
    if let Some(np) = neck_pb.finish() {
        canvas.fill_path(&np, skin_base);
        canvas.stroke_path_fine(&np, skin_shade, 1.5 * scale);
    }

    // ===== 5. BRAZOS CELESTIALES ELEVADOS EN ÉXTASIS =====
    // Brazo izquierdo elevado hacia la izquierda superior
    let mut arm_l = tiny_skia::PathBuilder::new();
    arm_l.move_to(-40.0 * scale, -95.0 * scale); // Hombro
    arm_l.cubic_to(-65.0 * scale, -135.0 * scale, -85.0 * scale, -170.0 * scale, -105.0 * scale, -210.0 * scale); // Codo a muñeca
    arm_l.line_to(-94.0 * scale, -215.0 * scale);
    arm_l.cubic_to(-75.0 * scale, -175.0 * scale, -55.0 * scale, -140.0 * scale, -32.0 * scale, -100.0 * scale);
    arm_l.close();
    if let Some(ap) = arm_l.finish() {
        canvas.fill_path(&ap, skin_base);
        canvas.stroke_path_fine(&ap, skin_shade, 1.8 * scale);
    }
    // Mano izquierda abierta soltando la sábana
    canvas.fill_circle(-100.0 * scale, -216.0 * scale, 8.0 * scale, skin_base);
    for finger in 0..4 {
        let fa = -1.2 + (finger as f32) * 0.35;
        let fx2 = -100.0 * scale + fa.cos() * 16.0 * scale;
        let fy2 = -216.0 * scale + fa.sin() * 16.0 * scale;
        canvas.stroke_line_fine(-100.0 * scale, -216.0 * scale, fx2, fy2, skin_base, 3.2 * scale);
    }

    // Brazo derecho elevado hacia la derecha celestial
    let mut arm_r = tiny_skia::PathBuilder::new();
    arm_r.move_to(40.0 * scale, -95.0 * scale); // Hombro
    arm_r.cubic_to(65.0 * scale, -140.0 * scale, 90.0 * scale, -180.0 * scale, 115.0 * scale, -225.0 * scale);
    arm_r.line_to(125.0 * scale, -220.0 * scale);
    arm_r.cubic_to(100.0 * scale, -175.0 * scale, 75.0 * scale, -135.0 * scale, 48.0 * scale, -98.0 * scale);
    arm_r.close();
    if let Some(ap) = arm_r.finish() {
        canvas.fill_path(&ap, skin_base);
        canvas.stroke_path_fine(&ap, skin_shade, 1.8 * scale);
    }
    // Mano derecha en bendición / éxtasis
    canvas.fill_circle(120.0 * scale, -225.0 * scale, 8.0 * scale, skin_base);
    for finger in 0..4 {
        let fa = -1.8 + (finger as f32) * 0.35;
        let fx2 = 120.0 * scale + fa.cos() * 16.0 * scale;
        let fy2 = -225.0 * scale + fa.sin() * 16.0 * scale;
        canvas.stroke_line_fine(120.0 * scale, -225.0 * scale, fx2, fy2, skin_base, 3.2 * scale);
    }

    // Mangas de gasa transparente en los hombros
    for side in &[-1.0_f32, 1.0_f32] {
        let mut sleeve_pb = tiny_skia::PathBuilder::new();
        sleeve_pb.move_to(side * 36.0 * scale, -95.0 * scale);
        sleeve_pb.cubic_to(side * 55.0 * scale, -75.0 * scale, side * 62.0 * scale, -115.0 * scale, side * 44.0 * scale, -125.0 * scale);
        sleeve_pb.close();
        if let Some(sp) = sleeve_pb.finish() {
            canvas.fill_path(&sp, sheet_pure.with_alpha(0.70));
            canvas.stroke_path_fine(&sp, gold_thread, 1.2 * scale);
        }
    }

    // ===== 6. ROSTRO SERENO Y NOBLE (La Belleza de Remedios) =====
    let head_y = -142.0 * scale;
    // Óvalo craneal fino
    let mut head_pb = tiny_skia::PathBuilder::new();
    head_pb.move_to(-18.0 * scale, head_y - 12.0 * scale);
    head_pb.cubic_to(-20.0 * scale, head_y + 12.0 * scale, -14.0 * scale, head_y + 32.0 * scale, 0.0, head_y + 36.0 * scale); // Barbilla
    head_pb.cubic_to(14.0 * scale, head_y + 32.0 * scale, 20.0 * scale, head_y + 12.0 * scale, 18.0 * scale, head_y - 12.0 * scale);
    head_pb.cubic_to(16.0 * scale, head_y - 32.0 * scale, -16.0 * scale, head_y - 32.0 * scale, -18.0 * scale, head_y - 12.0 * scale);
    head_pb.close();
    if let Some(hp) = head_pb.finish() {
        canvas.fill_path(&hp, skin_base);
        canvas.stroke_path_fine(&hp, skin_shade, 1.5 * scale);
    }

    // Rubor suave en las mejillas
    canvas.fill_circle(-9.0 * scale, head_y + 10.0 * scale, 6.0 * scale, skin_blush.with_alpha(0.40));
    canvas.fill_circle(9.0 * scale, head_y + 10.0 * scale, 6.0 * scale, skin_blush.with_alpha(0.40));

    // Ojos cerrados en éxtasis sereno con pestañas curvadas
    for side in &[-1.0_f32, 1.0_f32] {
        let eye_x = side * 8.0 * scale;
        let eye_y = head_y + 2.0 * scale;
        let mut eyelid = tiny_skia::PathBuilder::new();
        eyelid.move_to(eye_x - 5.0 * scale, eye_y);
        eyelid.cubic_to(eye_x, eye_y + 3.5 * scale, eye_x + 3.0 * scale, eye_y + 3.5 * scale, eye_x + 5.0 * scale, eye_y);
        if let Some(ep) = eyelid.finish() {
            canvas.stroke_path_fine(&ep, hair_jet, 1.8 * scale);
        }
        // Pestañas
        canvas.stroke_line_fine(eye_x - 2.0 * scale, eye_y + 2.5 * scale, eye_x - 3.5 * scale, eye_y + 5.5 * scale, hair_jet, 1.0 * scale);
        canvas.stroke_line_fine(eye_x + 2.0 * scale, eye_y + 2.5 * scale, eye_x + 3.5 * scale, eye_y + 5.5 * scale, hair_jet, 1.0 * scale);
        // Ceja arqueada perfecta
        let mut brow = tiny_skia::PathBuilder::new();
        brow.move_to(eye_x - 6.0 * scale, eye_y - 7.0 * scale);
        brow.cubic_to(eye_x, eye_y - 11.0 * scale, eye_x + 4.0 * scale, eye_y - 10.0 * scale, eye_x + 7.0 * scale, eye_y - 6.0 * scale);
        if let Some(bp) = brow.finish() {
            canvas.stroke_path_fine(&bp, hair_jet, 1.4 * scale);
        }
    }

    // Nariz fina y aristocrática
    let mut nose = tiny_skia::PathBuilder::new();
    nose.move_to(0.0, head_y - 2.0 * scale);
    nose.line_to(1.5 * scale, head_y + 12.0 * scale);
    nose.line_to(-1.5 * scale, head_y + 14.0 * scale);
    if let Some(np) = nose.finish() {
        canvas.stroke_path_fine(&np, skin_shade, 1.2 * scale);
    }

    // Labios serenos color rosa
    let lip_y = head_y + 22.0 * scale;
    let mut lips = tiny_skia::PathBuilder::new();
    lips.move_to(-5.0 * scale, lip_y);
    lips.cubic_to(-2.0 * scale, lip_y - 2.0 * scale, 2.0 * scale, lip_y - 2.0 * scale, 5.0 * scale, lip_y);
    lips.cubic_to(2.0 * scale, lip_y + 2.5 * scale, -2.0 * scale, lip_y + 2.5 * scale, -5.0 * scale, lip_y);
    lips.close();
    if let Some(lp) = lips.finish() {
        canvas.fill_path(&lp, Color::hex("#D96B6B"));
    }

    // Corona de cabellos frontales con raya al medio
    let mut front_hair = tiny_skia::PathBuilder::new();
    front_hair.move_to(-18.0 * scale, head_y - 12.0 * scale);
    front_hair.cubic_to(-12.0 * scale, head_y - 26.0 * scale, 0.0, head_y - 18.0 * scale, 0.0, head_y - 14.0 * scale);
    front_hair.cubic_to(0.0, head_y - 18.0 * scale, 12.0 * scale, head_y - 26.0 * scale, 18.0 * scale, head_y - 12.0 * scale);
    front_hair.cubic_to(16.0 * scale, head_y - 34.0 * scale, -16.0 * scale, head_y - 34.0 * scale, -18.0 * scale, head_y - 12.0 * scale);
    front_hair.close();
    if let Some(fhp) = front_hair.finish() {
        canvas.fill_path(&fhp, hair_jet);
        canvas.stroke_path_fine(&fhp, hair_highlight, 1.2 * scale);
    }

    // ===== 7. SÁBANAS DE BRAMANTE EN PRIMER PLANO (Abrazando el cuerpo) =====
    // Cinta diagonal de sábana que cruza la cintura y vuela hacia arriba a la izquierda
    let drape_phase = tau * 6.0 * PI;
    let mut drape_pb = tiny_skia::PathBuilder::new();
    drape_pb.move_to(-140.0 * scale, -60.0 * scale + (drape_phase).sin() * 20.0 * scale);
    drape_pb.cubic_to(
        -80.0 * scale, -30.0 * scale,
        -40.0 * scale, -35.0 * scale,
        0.0, -38.0 * scale,
    );
    drape_pb.cubic_to(
        50.0 * scale, -40.0 * scale,
        90.0 * scale, -10.0 * scale,
        150.0 * scale, 20.0 * scale + (drape_phase).cos() * 25.0 * scale,
    );
    drape_pb.cubic_to(
        100.0 * scale, 55.0 * scale,
        40.0 * scale, 15.0 * scale,
        -20.0 * scale, 10.0 * scale,
    );
    drape_pb.cubic_to(
        -70.0 * scale, 5.0 * scale,
        -110.0 * scale, -10.0 * scale,
        -140.0 * scale, -60.0 * scale + (drape_phase).sin() * 20.0 * scale,
    );
    drape_pb.close();
    if let Some(dp) = drape_pb.finish() {
        canvas.fill_path(&dp, sheet_pure);
        canvas.stroke_path_fine(&dp, sheet_shade, 2.2 * scale);
        // Costura dorada
        canvas.stroke_path_fine(&dp, gold_thread.with_alpha(0.65), 1.2 * scale);
    }

    // ===== 8. NUBE DE MARIPOSAS AMARILLAS EN TORBELLINO ASCENDENTE =====
    for b in 0..11 {
        let b_t = b as f32;
        let b_angle = tau * 5.0 * PI + b_t * (PI * 2.0 / 11.0);
        let b_dist = (110.0 + (b_t * 19.0) % 95.0) * scale;
        let bx = halo_cx + b_angle.cos() * b_dist;
        let by = halo_cy + 80.0 * scale + b_angle.sin() * (b_dist * 0.85) - (b_t * 22.0 * scale);
        let b_flap = ((tau * 26.0 * PI + b_t * 2.1).sin()).abs();

        // Mariposa dorada con dos alas
        canvas.save();
        canvas.translate(bx, by);
        canvas.rotate(b_angle + PI * 0.5);

        let wing_w = (14.0 + (b % 3) as f32 * 4.0) * scale;
        let wing_h = (18.0 + (b % 3) as f32 * 5.0) * scale;

        // Ala izquierda
        let mut wl = tiny_skia::PathBuilder::new();
        wl.move_to(0.0, 0.0);
        wl.cubic_to(-wing_w * b_flap, -wing_h * 0.5, -wing_w * 1.2 * b_flap, wing_h * 0.4, 0.0, wing_h * 0.7);
        wl.close();
        if let Some(p) = wl.finish() {
            canvas.fill_path(&p, butterfly_yellow);
            canvas.stroke_path_fine(&p, Color::hex("#B8860B"), 1.0 * scale);
        }

        // Ala derecha
        let mut wr = tiny_skia::PathBuilder::new();
        wr.move_to(0.0, 0.0);
        wr.cubic_to(wing_w * b_flap, -wing_h * 0.5, wing_w * 1.2 * b_flap, wing_h * 0.4, 0.0, wing_h * 0.7);
        wr.close();
        if let Some(p) = wr.finish() {
            canvas.fill_path(&p, butterfly_yellow);
            canvas.stroke_path_fine(&p, Color::hex("#B8860B"), 1.0 * scale);
        }

        // Cuerpo de mariposa
        canvas.stroke_line_fine(0.0, -wing_h * 0.4, 0.0, wing_h * 0.6, hair_jet, 1.6 * scale);

        canvas.restore();
    }

    // Estrellas / centellas doradas flotantes
    for s in 0..8 {
        let sx = ((s as f32 * 57.3 + tau * 20.0).sin()) * 180.0 * scale;
        let sy = -260.0 * scale + (s as f32 * 50.0 * scale);
        let star_sz = (4.0 + (s % 3) as f32 * 2.5) * scale;
        canvas.fill_circle(sx, sy, star_sz, gold_bright.with_alpha(0.75));
        canvas.stroke_line_fine(sx - star_sz * 1.8, sy, sx + star_sz * 1.8, sy, Color::WHITE, 1.0 * scale);
        canvas.stroke_line_fine(sx, sy - star_sz * 1.8, sx, sy + star_sz * 1.8, Color::WHITE, 1.0 * scale);
    }

    canvas.restore();
}

// =============================================================================
// 8. PESCADITO DE ORO ALQUÍMICO MONUMENTAL (Taller Orfebre de Aureliano Buendía)
// =============================================================================

pub fn draw_master_pescadito_oro_alquimia(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let cyan_glow = Color::hex("#00F5D4");
    let cyan_dim = Color::hex("#125B6C");
    let gold_line = Color::hex("#FFD700");
    let gold_bright = Color::hex("#FFF099");
    let gold_dark = Color::hex("#8B6E14");
    let ruby_red = Color::hex("#FF0055");
    let ruler_col = Color::hex("#80D0FF");

    canvas.save();
    canvas.translate(cx, cy);

    // 1. Círculos concéntricos de calibración astronómica / alquímica
    canvas.stroke_circle(0.0, 0.0, 220.0 * scale, cyan_dim.with_alpha(0.35), 1.0 * scale);
    canvas.stroke_circle(0.0, 0.0, 310.0 * scale, cyan_dim.with_alpha(0.20), 0.8 * scale);

    // Marcas de cuadrante (0°, 90°, 180°, 270°)
    for a in 0..4 {
        let ang = (a as f32) * PI * 0.5;
        let x1 = ang.cos() * 200.0 * scale;
        let y1 = ang.sin() * 200.0 * scale;
        let x2 = ang.cos() * 240.0 * scale;
        let y2 = ang.sin() * 240.0 * scale;
        canvas.stroke_line_fine(x1, y1, x2, y2, cyan_glow.with_alpha(0.70), 1.2 * scale);
    }

    // 2. Herramientas del orfebre en la base del taller (crisol, soplete, tenazas, yunque)
    let bench_y = 155.0 * scale;
    // Tabla del banco de trabajo
    canvas.fill_rect(-260.0 * scale, bench_y, 520.0 * scale, 12.0 * scale, Color::hex("#182A45"));
    canvas.stroke_rect(-260.0 * scale, bench_y, 520.0 * scale, 12.0 * scale, cyan_glow.with_alpha(0.80), 1.4 * scale);

    // Crisol de grafito con oro fundido
    let c_x = -170.0 * scale;
    let mut crisol_pb = tiny_skia::PathBuilder::new();
    crisol_pb.move_to(c_x - 22.0 * scale, bench_y - 36.0 * scale);
    crisol_pb.line_to(c_x + 22.0 * scale, bench_y - 36.0 * scale);
    crisol_pb.line_to(c_x + 14.0 * scale, bench_y);
    crisol_pb.line_to(c_x - 14.0 * scale, bench_y);
    crisol_pb.close();
    if let Some(cp) = crisol_pb.finish() {
        canvas.fill_path(&cp, Color::hex("#28161A"));
        canvas.stroke_path_fine(&cp, gold_line, 1.5 * scale);
        // Resplandor de oro líquido en el crisol
        canvas.fill_circle(c_x, bench_y - 36.0 * scale, 10.0 * scale, gold_bright);
    }

    // Mini yunque de acero templado
    let anvil_x = 170.0 * scale;
    canvas.fill_rect(anvil_x - 30.0 * scale, bench_y - 28.0 * scale, 60.0 * scale, 28.0 * scale, Color::hex("#203550"));
    canvas.stroke_rect(anvil_x - 30.0 * scale, bench_y - 28.0 * scale, 60.0 * scale, 28.0 * scale, cyan_glow, 1.4 * scale);

    // 3. El Pescadito de Oro Monumental en Rayos X
    let fish_len = 460.0 * scale;
    let fish_h = 135.0 * scale;

    // Cuerpo base exterior (filigrana dorada con escamas caladas)
    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(fish_len * 0.45, 0.0); // Hocico
    body_pb.cubic_to(fish_len * 0.28, -fish_h * 0.70, -fish_len * 0.22, -fish_h * 0.65, -fish_len * 0.38, 0.0);
    body_pb.cubic_to(-fish_len * 0.22, fish_h * 0.65, fish_len * 0.28, fish_h * 0.70, fish_len * 0.45, 0.0);
    body_pb.close();

    if let Some(bp) = body_pb.finish() {
        canvas.fill_path(&bp, Color::hex("#0D1A38").with_alpha(0.85));
        canvas.stroke_path_fine(&bp, gold_line, 2.8 * scale);

        // Escamas de orfebrería cincelada (3 filas de medias lunas)
        for col in 0..7 {
            let sx = -fish_len * 0.24 + (col as f32) * (fish_len * 0.08);
            for row in -1..=1 {
                let sy = (row as f32) * (fish_h * 0.25);
                canvas.stroke_circle(sx, sy, 16.0 * scale, gold_dark.with_alpha(0.65), 1.2 * scale);
            }
        }
    }

    // Cola bífida articulada con láminas de oro flexible
    let tail_root = -fish_len * 0.38;
    let tail_swing = (tau * 8.0 * PI).sin() * (16.0 * scale);
    let mut tail_pb = tiny_skia::PathBuilder::new();
    tail_pb.move_to(tail_root, 0.0);
    tail_pb.cubic_to(tail_root - 60.0 * scale, -fish_h * 0.85 + tail_swing, tail_root - 130.0 * scale, -fish_h * 0.75 + tail_swing, tail_root - 145.0 * scale, -fish_h * 0.45 + tail_swing);
    tail_pb.cubic_to(tail_root - 100.0 * scale, tail_swing * 0.5, tail_root - 100.0 * scale, tail_swing * 0.5, tail_root - 145.0 * scale, fish_h * 0.45 + tail_swing);
    tail_pb.cubic_to(tail_root - 130.0 * scale, fish_h * 0.75 + tail_swing, tail_root - 60.0 * scale, fish_h * 0.85 + tail_swing, tail_root, 0.0);
    tail_pb.close();
    if let Some(tp) = tail_pb.finish() {
        canvas.fill_path(&tp, Color::hex("#0A1630"));
        canvas.stroke_path_fine(&tp, gold_line, 2.2 * scale);

        // Rayas de articulación de la aleta caudal
        for r in 0..6 {
            let frac = (r as f32) / 5.0;
            let ty_end = (-fish_h * 0.40 + frac * (fish_h * 0.80)) + tail_swing;
            canvas.stroke_line_fine(tail_root, 0.0, tail_root - 135.0 * scale, ty_end, gold_bright.with_alpha(0.70), 1.2 * scale);
        }
    }

    // Aletas dorsal y ventral caladas
    let mut fin_pb = tiny_skia::PathBuilder::new();
    fin_pb.move_to(-20.0 * scale, -fish_h * 0.48);
    fin_pb.cubic_to(20.0 * scale, -fish_h * 0.95, 70.0 * scale, -fish_h * 0.85, 80.0 * scale, -fish_h * 0.45);
    fin_pb.close();
    if let Some(fp) = fin_pb.finish() {
        canvas.stroke_path_fine(&fp, gold_line, 1.8 * scale);
    }

    // 4. Esqueleto y Engranajes Internos de Rayos X (BlendMode::Screen)
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);

    // 24 vértebras articuladas numeradas
    let vertebra_count = 24;
    let spine_x1 = -fish_len * 0.32;
    let spine_x2 = fish_len * 0.30;
    for v in 0..vertebra_count {
        let t = v as f32 / (vertebra_count - 1) as f32;
        let vx = spine_x1 + t * (spine_x2 - spine_x1);
        let wave = (tau * 10.0 * PI + t * 4.0).sin() * (9.0 * scale * (1.0 - t));
        let vy = wave;

        // Cuerpo vertebral
        canvas.fill_circle(vx, vy, 4.2 * scale, cyan_glow);
        canvas.stroke_circle(vx, vy, 7.5 * scale, gold_line, 1.2 * scale);

        // Costillas espinosas articuladas
        if v % 2 == 0 && v > 1 && v < vertebra_count - 2 {
            let rib_h = (1.0 - (t - 0.45).abs() * 1.7).max(0.18) * (fish_h * 0.50);
            canvas.stroke_line_fine(vx, vy, vx - 8.0 * scale, vy - rib_h, cyan_glow.with_alpha(0.90), 1.5 * scale);
            canvas.stroke_line_fine(vx, vy, vx - 8.0 * scale, vy + rib_h, cyan_glow.with_alpha(0.90), 1.5 * scale);
        }
    }

    // Engranaje de reloj suizo en el abdomen (Aureliano incorporó mecanismos móviles)
    let gear_cx = -35.0 * scale;
    let gear_cy = 6.0 * scale;
    let gear_r = 32.0 * scale;
    canvas.stroke_circle(gear_cx, gear_cy, gear_r, gold_bright, 1.8 * scale);
    canvas.stroke_circle(gear_cx, gear_cy, gear_r * 0.45, cyan_glow, 1.2 * scale);
    canvas.fill_circle(gear_cx, gear_cy, 5.0 * scale, ruby_red);

    let teeth = 16;
    for t in 0..teeth {
        let a = (t as f32 / teeth as f32) * PI * 2.0 + tau * 4.0 * PI;
        let gx1 = gear_cx + a.cos() * (gear_r - 4.0 * scale);
        let gy1 = gear_cy + a.sin() * (gear_r - 4.0 * scale);
        let gx2 = gear_cx + a.cos() * (gear_r + 5.0 * scale);
        let gy2 = gear_cy + a.sin() * (gear_r + 5.0 * scale);
        canvas.stroke_line_fine(gx1, gy1, gx2, gy2, gold_line, 2.2 * scale);
    }

    // Ojo de rubí vivo engarzado en bisel de oro
    let eye_x = fish_len * 0.32;
    let eye_y = -12.0 * scale;
    canvas.fill_circle(eye_x, eye_y, 11.0 * scale, ruby_red);
    canvas.stroke_circle(eye_x, eye_y, 14.0 * scale, gold_bright, 2.0 * scale);
    canvas.fill_circle(eye_x - 3.0 * scale, eye_y - 3.0 * scale, 3.5 * scale, Color::WHITE);

    canvas.restore();

    // Rótulo técnico del pez
    draw_vector_text(canvas, 0.0, -fish_h * 0.85, "ORFEBRERIA DE AURELIANO • 24 ARTICULACIONES • RUBI NATURAL", 12.0 * scale, ruler_col.with_alpha(0.85), true);

    canvas.restore();
}

// =============================================================================
// 9. BASTIDOR DE MORTAJA Y TELAR EN RAYOS X (Las Pasiones de Amaranta)
// =============================================================================

pub fn draw_master_bastidor_mortaja_alquimia(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
    frame: u32,
) {
    let cyan_glow = Color::hex("#00F5D4");
    let cyan_dim = Color::hex("#1A485C");
    let gold_thread = Color::hex("#FFD700");
    let wood_beam = Color::hex("#122238");
    let wood_border = Color::hex("#00D2B8");
    let lace_line = Color::hex("#D2F4FF");

    canvas.save();
    canvas.translate(cx, cy);

    let half_w = w * 0.5;
    let half_h = h * 0.5;
    let beam_th = 24.0;

    // 1. Marco del telar de cedro en corte de rayos X
    // Postes verticales izquierdo y derecho
    canvas.fill_rect(-half_w, -half_h, beam_th, h, wood_beam);
    canvas.stroke_rect(-half_w, -half_h, beam_th, h, wood_border, 2.0);

    canvas.fill_rect(half_w - beam_th, -half_h, beam_th, h, wood_beam);
    canvas.stroke_rect(half_w - beam_th, -half_h, beam_th, h, wood_border, 2.0);

    // Travesaños horizontales superior e inferior
    canvas.fill_rect(-half_w - 12.0, -half_h, w + 24.0, beam_th, wood_beam);
    canvas.stroke_rect(-half_w - 12.0, -half_h, w + 24.0, beam_th, wood_border, 2.0);

    canvas.fill_rect(-half_w - 12.0, half_h - beam_th, w + 24.0, beam_th, wood_beam);
    canvas.stroke_rect(-half_w - 12.0, half_h - beam_th, w + 24.0, beam_th, wood_border, 2.0);

    // Clavijas de tensión de bronce en los travesaños
    let peg_count = 14;
    for p in 0..peg_count {
        let px = -half_w + 35.0 + (p as f32) * ((w - 70.0) / (peg_count - 1) as f32);
        canvas.fill_circle(px, -half_h + beam_th * 0.5, 5.0, gold_thread);
        canvas.fill_circle(px, half_h - beam_th * 0.5, 5.0, gold_thread);
    }

    // 2. 36 Hilos de urdimbre verticales tensados
    let warp_count = 36;
    let inner_w = w - beam_th * 2.0 - 20.0;
    for i in 0..warp_count {
        let wx = -inner_w * 0.5 + (i as f32) * (inner_w / (warp_count - 1) as f32);
        let col = if i % 4 == 0 { gold_thread.with_alpha(0.75) } else { cyan_dim.with_alpha(0.55) };
        canvas.stroke_line_fine(wx, -half_h + beam_th, wx, half_h - beam_th, col, 1.0);
    }

    // 3. El sudario de encaje tejido en filigrana de rayos X (vibrante y luminoso)
    let shroud_w = inner_w * 0.88;
    let shroud_h = h * 0.72;
    let shroud_x = -shroud_w * 0.5;
    let shroud_y = -shroud_h * 0.45;

    // Fondo translúcido azul zafiro técnico
    canvas.fill_rect(shroud_x, shroud_y, shroud_w, shroud_h, Color::hex("#081D33").with_alpha(0.88));
    canvas.stroke_rect(shroud_x, shroud_y, shroud_w, shroud_h, cyan_glow, 2.2);

    // Patrón geométrico de encaje de bolillos: celosía diagonal, rosetas y festones
    let lace_cols = 6;
    let lace_rows = 8;
    let cw = shroud_w / lace_cols as f32;
    let ch = shroud_h / lace_rows as f32;

    for r in 0..lace_rows {
        let ly = shroud_y + (r as f32) * ch;
        for c in 0..lace_cols {
            let lx = shroud_x + (c as f32) * cw;
            let mx = lx + cw * 0.5;
            let my = ly + ch * 0.5;

            // Rombo de encaje luminoso
            let mut rombo = tiny_skia::PathBuilder::new();
            rombo.move_to(mx, ly);
            rombo.line_to(lx + cw, my);
            rombo.line_to(mx, ly + ch);
            rombo.line_to(lx, my);
            rombo.close();
            if let Some(rp) = rombo.finish() {
                canvas.stroke_path_fine(&rp, Color::hex("#5AE5FF").with_alpha(0.70), 1.4);
            }

            // Roseta central de encaje con 4 pétalos
            canvas.fill_circle(mx, my, 4.0, gold_thread);
            canvas.stroke_circle(mx, my, 7.5, cyan_glow.with_alpha(0.80), 1.0);
        }
    }

    // Festón de encaje calado (scalloped hem) en el borde inferior del sudario
    for c in 0..lace_cols {
        let lx = shroud_x + (c as f32) * cw;
        canvas.stroke_circle(lx + cw * 0.5, shroud_y + shroud_h, cw * 0.45, gold_thread, 1.8);
    }

    // 4. Naveta de tejer dorada que se desplaza de izquierda a derecha continuamente
    let shuttle_x = (tau * 14.0 * PI).sin() * (shroud_w * 0.40);
    let shuttle_y = shroud_y + shroud_h * 0.82;
    canvas.fill_circle(shuttle_x, shuttle_y, 13.0, gold_thread);
    canvas.stroke_circle(shuttle_x, shuttle_y, 13.0, Color::WHITE, 2.0);
    // Hilo dorado continuo saliendo de la naveta
    canvas.stroke_line_fine(shuttle_x, shuttle_y, 0.0, shroud_y + shroud_h, gold_thread, 2.4);

    // 5. Tira perforada de la pianola de Pietro Crespi al costado derecho
    let strip_x = half_w - beam_th - 35.0;
    canvas.fill_rect(strip_x, -half_h + 40.0, 28.0, h - 80.0, Color::hex("#0D2335"));
    canvas.stroke_rect(strip_x, -half_h + 40.0, 28.0, h - 80.0, cyan_glow.with_alpha(0.85), 1.5);
    for p in 0..18 {
        let py = -half_h + 55.0 + (p as f32) * ((h - 110.0) / 17.0);
        let shift = ((p * 7 + (frame / 2) as usize) % 3) as f32 * 7.0;
        canvas.fill_circle(strip_x + 7.0 + shift, py, 3.0, gold_thread);
    }

    draw_vector_text(canvas, 0.0, half_h - 6.0, "TELAR DEL SUDARIO • AMARANTA BUENDIA • ENCAJE Y PIANOLA", 11.0, wood_border.with_alpha(0.85), true);

    canvas.restore();
}

// =============================================================================
// 10. GALEÓN ESPAÑOL EN LA MANIGUA EN RAYOS X (Arqueología Naval de Melquíades)
// =============================================================================

pub fn draw_master_galeon_naval_blueprint(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
) {
    let cyan_glow = Color::hex("#00F5D4");
    let cyan_bright = Color::hex("#80FFFF");
    let cyan_dim = Color::hex("#184E6C");
    let gold_line = Color::hex("#FFD700");
    let gold_bright = Color::hex("#FFF3B0");
    let jungle_cyan = Color::hex("#127565");

    canvas.save();
    canvas.translate(cx, cy + 30.0);

    let ship_len = w * 0.44;

    // 1. Enredaderas tropicales fosforescentes y lianas que envuelven el buque
    for v in 0..9 {
        let vx = -ship_len * 0.90 + (v as f32) * (ship_len * 0.22);
        let mut vine_pb = tiny_skia::PathBuilder::new();
        vine_pb.move_to(vx, -320.0);
        vine_pb.cubic_to(vx + 45.0, -160.0, vx - 50.0, 30.0, vx + 25.0, 180.0);
        if let Some(vp) = vine_pb.finish() {
            canvas.stroke_path_fine(&vp, jungle_cyan.with_alpha(0.60), 3.5);
            canvas.stroke_path_fine(&vp, cyan_glow.with_alpha(0.50), 1.4);
        }
    }

    // 2. Casco del Galeón: Quilla, Roda y Espejo de Popa en corte esquemático
    let mut hull_pb = tiny_skia::PathBuilder::new();
    hull_pb.move_to(-ship_len, -45.0); // Espejo de popa
    hull_pb.cubic_to(-ship_len * 0.65, 120.0, ship_len * 0.45, 140.0, ship_len, 30.0); // Proa
    hull_pb.line_to(ship_len - 25.0, -85.0); // Castillo de proa
    hull_pb.line_to(-ship_len + 35.0, -80.0); // Castillo de popa
    hull_pb.close();

    if let Some(hp) = hull_pb.finish() {
        canvas.fill_path(&hp, Color::hex("#0A1C36").with_alpha(0.92));
        canvas.stroke_path_fine(&hp, cyan_bright, 3.5);
        canvas.stroke_path_fine(&hp, gold_line, 1.5);
    }

    // Mascarón de proa: León heráldico en oro tallado
    let lion_x = ship_len + 15.0;
    let lion_y = 20.0;
    canvas.fill_circle(lion_x, lion_y, 14.0, gold_line);
    canvas.stroke_circle(lion_x, lion_y, 14.0, Color::WHITE, 2.0);

    // Farol de popa con cristal esmeralda
    canvas.fill_rect(-ship_len - 15.0, -95.0, 18.0, 26.0, gold_line);
    canvas.fill_circle(-ship_len - 6.0, -82.0, 6.0, cyan_glow);

    // 3. Las 3 cubiertas interiores de combate en corte transversal
    let decks = [-60.0, -15.0, 42.0];
    for &dy in &decks {
        canvas.stroke_line_fine(-ship_len * 0.88, dy, ship_len * 0.85, dy, gold_line.with_alpha(0.90), 2.5);
    }

    // Cuadernas y costillares del casco (20 arcos náuticos)
    let rib_count = 20;
    for r in 0..rib_count {
        let rx = -ship_len * 0.86 + (r as f32 / (rib_count - 1) as f32) * (ship_len * 1.70);
        let mut rib_pb = tiny_skia::PathBuilder::new();
        rib_pb.move_to(rx, -70.0);
        rib_pb.cubic_to(rx + 18.0, 15.0, rx - 12.0, 70.0, rx - 18.0, 115.0);
        if let Some(rp) = rib_pb.finish() {
            canvas.stroke_path_fine(&rp, cyan_dim.with_alpha(0.85), 2.0);
        }
    }

    // Cañones de bronce en el entrepuente
    for g in 0..6 {
        let gx = -ship_len * 0.60 + (g as f32) * 85.0;
        canvas.fill_rect(gx, -24.0, 42.0, 12.0, gold_line);
        canvas.stroke_rect(gx, -24.0, 42.0, 12.0, gold_bright, 1.5);
    }

    // 4. Tres palos colosales (Mesana, Mayor, Trinquete), vergas y velamen hinchado
    let masts = [
        (-ship_len * 0.45, -380.0, 16.0_f32),
        (0.0, -500.0, 20.0),
        (ship_len * 0.42, -420.0, 15.0),
    ];

    for &(mx, top_y, mw) in &masts {
        // Palo vertical
        canvas.stroke_line_fine(mx, -65.0, mx, top_y, Color::hex("#0D2D4D"), mw);
        canvas.stroke_line_fine(mx, -65.0, mx, top_y, cyan_bright, 2.5);

        // Verga inferior (mayor) y vela mayor
        let lower_yard_y = top_y + 180.0;
        let yard_span = 145.0;
        canvas.stroke_line_fine(mx - yard_span, lower_yard_y, mx + yard_span, lower_yard_y, gold_line, 4.5);

        let mut lower_sail = tiny_skia::PathBuilder::new();
        lower_sail.move_to(mx - yard_span + 10.0, lower_yard_y);
        lower_sail.cubic_to(mx - yard_span * 0.5, lower_yard_y + 110.0, mx + yard_span * 0.5, lower_yard_y + 110.0, mx + yard_span - 10.0, lower_yard_y);
        lower_sail.line_to(mx + yard_span * 0.85, lower_yard_y + 135.0);
        lower_sail.cubic_to(mx + yard_span * 0.4, lower_yard_y + 160.0, mx - yard_span * 0.4, lower_yard_y + 160.0, mx - yard_span * 0.85, lower_yard_y + 135.0);
        lower_sail.close();
        if let Some(sp) = lower_sail.finish() {
            canvas.fill_path(&sp, Color::hex("#123D64").with_alpha(0.80));
            canvas.stroke_path_fine(&sp, cyan_bright, 2.0);
            // Costuras verticales de la vela
            for s in -2..=2 {
                let sx = mx + (s as f32) * 28.0;
                canvas.stroke_line_fine(sx, lower_yard_y + 15.0, sx, lower_yard_y + 130.0, cyan_dim.with_alpha(0.80), 1.2);
            }
        }

        // Verga superior (gavia) y vela de gavia
        let upper_yard_y = top_y + 60.0;
        let upper_span = 110.0;
        canvas.stroke_line_fine(mx - upper_span, upper_yard_y, mx + upper_span, upper_yard_y, gold_line, 3.5);

        let mut upper_sail = tiny_skia::PathBuilder::new();
        upper_sail.move_to(mx - upper_span + 8.0, upper_yard_y);
        upper_sail.cubic_to(mx - upper_span * 0.5, upper_yard_y + 75.0, mx + upper_span * 0.5, upper_yard_y + 75.0, mx + upper_span - 8.0, upper_yard_y);
        upper_sail.line_to(mx + upper_span * 0.85, upper_yard_y + 95.0);
        upper_sail.cubic_to(mx + upper_span * 0.4, upper_yard_y + 115.0, mx - upper_span * 0.4, upper_yard_y + 115.0, mx - upper_span * 0.85, upper_yard_y + 95.0);
        upper_sail.close();
        if let Some(usp) = upper_sail.finish() {
            canvas.fill_path(&usp, Color::hex("#154570").with_alpha(0.75));
            canvas.stroke_path_fine(&usp, cyan_glow, 1.6);
        }
    }

    // Jarcias, obenques y flechastes
    for mx in [-ship_len * 0.45, 0.0, ship_len * 0.42] {
        for o in 0..6 {
            let ox = mx - 80.0 + (o as f32) * 32.0;
            canvas.stroke_line_fine(mx, -320.0, ox, -65.0, cyan_bright.with_alpha(0.40), 1.0);
        }
    }

    // Rótulo técnico naval
    draw_vector_text(canvas, 0.0, 150.0, "NAO VICTORIA • GALEON DEL SIGLO XVI • ENCALLADO A 12 KM DEL MAR", 12.0, gold_bright.with_alpha(0.90), true);

    canvas.restore();
}

// =============================================================================
// 11. EL CASTAÑO ANCESTRAL Y DON JOSÉ ARCADIO (Grabado Maestro)
// =============================================================================

pub fn draw_master_castano_ancestral(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    _tau: f32,
) {
    let bark_deep = Color::hex("#28160A");
    let bark_mid = Color::hex("#52341A");
    let bark_light = Color::hex("#7A512C");
    let bark_groove = Color::hex("#3E2414");
    let leaf_dark = Color::hex("#0E2A0B");
    let leaf_mid = Color::hex("#1D4A14");
    let leaf_bright = Color::hex("#36781F");
    let leaf_sun = Color::hex("#5BA83C");
    let leaf_vein = Color::hex("#82C44A");
    let chain_iron = Color::hex("#5A656E");
    let chain_highlight = Color::hex("#B8C4CC");
    let rust_col = Color::hex("#7A4020");

    let ground_y = h - 60.0;
    // El tronco parte desde abajo y el árbol ocupa casi toda la altura
    let trunk_top_y = h * 0.30; // El tronco sube hasta el 30% de la altura
    let trunk_base_w = w * 0.22; // Tronco monumental 22% del ancho

    // ===== SOMBRA DEL ÁRBOL EN EL SUELO =====
    let mut shadow_pb = tiny_skia::PathBuilder::new();
    shadow_pb.move_to(cx - w * 0.55, ground_y + 5.0);
    shadow_pb.cubic_to(cx - w * 0.30, ground_y - 12.0, cx + w * 0.30, ground_y - 12.0, cx + w * 0.55, ground_y + 5.0);
    shadow_pb.close();
    if let Some(sp) = shadow_pb.finish() {
        canvas.fill_path(&sp, Color::hex("#1A0D05").with_alpha(0.35));
    }

    // ===== RAÍCES COLOSALES RETORCIDAS =====
    let root_configs = [
        (cx - trunk_base_w * 1.20, -28.0_f32, -105.0_f32, 28.0_f32),
        (cx - trunk_base_w * 0.85, -18.0, -80.0, 22.0),
        (cx - trunk_base_w * 0.45, -12.0, -55.0, 32.0),
        (cx + trunk_base_w * 0.45, -12.0, -55.0, 32.0),
        (cx + trunk_base_w * 0.85, -18.0, -80.0, 22.0),
        (cx + trunk_base_w * 1.20, -28.0, -105.0, 28.0),
    ];
    for &(rx, dx, dy, th) in &root_configs {
        let mut r_pb = tiny_skia::PathBuilder::new();
        r_pb.move_to(rx, ground_y - 15.0);
        r_pb.cubic_to(rx + dx * 0.4, ground_y + 20.0, rx + dx * 0.7 + dy * 0.2, ground_y + 55.0, rx + dx, ground_y + 90.0);
        if let Some(rp) = r_pb.finish() {
            canvas.stroke_path_fine(&rp, bark_deep, th);
            canvas.stroke_path_fine(&rp, bark_mid, th * 0.5);
            canvas.stroke_path_fine(&rp, bark_light, th * 0.20);
        }
    }

    // ===== TRONCO PIRAMIDAL MONUMENTAL =====
    let mut trunk_pb = tiny_skia::PathBuilder::new();
    trunk_pb.move_to(cx - trunk_base_w * 0.5, trunk_top_y);
    trunk_pb.cubic_to(
        cx - trunk_base_w * 0.55, h * 0.55,
        cx - trunk_base_w * 0.80, ground_y - 20.0,
        cx - trunk_base_w, ground_y,
    );
    trunk_pb.line_to(cx + trunk_base_w, ground_y);
    trunk_pb.cubic_to(
        cx + trunk_base_w * 0.80, ground_y - 20.0,
        cx + trunk_base_w * 0.55, h * 0.55,
        cx + trunk_base_w * 0.5, trunk_top_y,
    );
    trunk_pb.close();
    if let Some(tp) = trunk_pb.finish() {
        canvas.fill_path(&tp, bark_mid);
        canvas.stroke_path_fine(&tp, bark_deep, 4.0);
        // Estrías de corteza centenaria
        for s in 0..28 {
            let t = s as f32 / 27.0;
            let sx_top = cx - trunk_base_w * 0.44 + t * trunk_base_w * 0.88;
            let sx_bot = cx - trunk_base_w * 0.88 + t * trunk_base_w * 1.76;
            canvas.stroke_line_fine(sx_top, trunk_top_y + 20.0, sx_bot, ground_y - 10.0, bark_groove, 2.2);
            canvas.stroke_line_fine(sx_top + 2.5, trunk_top_y + 35.0, sx_bot + 2.5, ground_y - 18.0, bark_light, 0.9);
        }
        // Nudos y cicatrices de la corteza
        for n in 0..6 {
            let nx = cx + (if n % 2 == 0 { -1.0 } else { 1.0 }) * trunk_base_w * (0.12 + (n as f32) * 0.06);
            let ny = trunk_top_y + 80.0 + (n as f32) * 85.0;
            canvas.stroke_circle(nx, ny, 18.0 + (n % 3) as f32 * 8.0, bark_deep, 3.5);
            canvas.stroke_circle(nx, ny, 12.0 + (n % 3) as f32 * 5.0, bark_groove, 1.8);
        }
    }

    // ===== RAMAS PRINCIPALES NUDOSAS =====
    let bough_configs: [(f32, f32, f32, f32, f32); 8] = [
        // (base_x_offset, base_y, end_x_offset_from_cx, end_y, thickness)
        (-trunk_base_w * 0.4, trunk_top_y + 20.0, -w * 0.42, h * 0.20, 38.0),
        (-trunk_base_w * 0.3, trunk_top_y + 10.0, -w * 0.26, h * 0.16, 32.0),
        (-trunk_base_w * 0.15, trunk_top_y,        -w * 0.12, h * 0.15, 28.0),
        (trunk_base_w * 0.15, trunk_top_y,          w * 0.12, h * 0.15, 28.0),
        (trunk_base_w * 0.3, trunk_top_y + 10.0,   w * 0.26, h * 0.16, 32.0),
        (trunk_base_w * 0.4, trunk_top_y + 20.0,   w * 0.42, h * 0.20, 38.0),
        (-trunk_base_w * 0.1, trunk_top_y + 5.0,  -w * 0.35, h * 0.24, 42.0),
        (trunk_base_w * 0.1, trunk_top_y + 5.0,    w * 0.35, h * 0.24, 42.0),
    ];
    for &(bx_off, by, ex, ey, th) in &bough_configs {
        let bx = cx + bx_off;
        let mid_x = bx + (ex - bx) * 0.5 + (ex - cx) * 0.15;
        let mid_y = (by + ey) * 0.5 - 45.0;
        let mut b_pb = tiny_skia::PathBuilder::new();
        b_pb.move_to(bx, by);
        b_pb.cubic_to(mid_x, mid_y, ex + (ex - cx) * 0.08, ey + 25.0, ex, ey);
        if let Some(bp) = b_pb.finish() {
            canvas.stroke_path_fine(&bp, bark_mid, th);
            canvas.stroke_path_fine(&bp, bark_deep, 3.5);
            canvas.stroke_path_fine(&bp, bark_light, th * 0.12);
        }
        // Sub-ramas secundarias
        let sub_x1 = ex + (ex - cx) * 0.10;
        let sub_y1 = ey - 15.0;
        let mut sb_pb = tiny_skia::PathBuilder::new();
        sb_pb.move_to(ex, ey);
        sb_pb.cubic_to(sub_x1 - 15.0, sub_y1 - 15.0, sub_x1 - 35.0, sub_y1 - 30.0, sub_x1 - 45.0, sub_y1 - 45.0);
        if let Some(sbp) = sb_pb.finish() {
            canvas.stroke_path_fine(&sbp, bark_mid, th * 0.45);
            canvas.stroke_path_fine(&sbp, bark_deep, 2.0);
        }
    }

    // ===== COPA DE HOJAS INDIVIDUALES: SILUETAS BOTANICAS EN CAPAS =====
    // Cada hoja es una forma lanceolada con nervadura central
    // Se dibujan desde las más profundas a las iluminadas por el sol caribeño
    let leaf_deep_shadow = Color::hex("#153612");
    let leaf_dark = Color::hex("#23541A");
    let leaf_mid = Color::hex("#367822");
    let leaf_bright = Color::hex("#54A32D");
    let leaf_sun = Color::hex("#7CB342");
    let leaf_gold = Color::hex("#AED581");
    let leaf_vein = Color::hex("#DCEDC8");
    let bur_col = Color::hex("#558B2F");
    let spike_col = Color::hex("#C0CA33");

    let leaf_clusters: &[(f32, f32, f32, usize)] = &[
        // (cx_cluster, cy_cluster, radio_cluster, n_hojas)
        // Cúpula superior (enmarcando con holgura bajo la cartela colonial y = 135+)
        (cx,             h * 0.18, 250.0, 32),
        (cx - w * 0.22,  h * 0.17, 240.0, 28),
        (cx + w * 0.22,  h * 0.17, 240.0, 28),
        // Extremos superiores
        (cx - w * 0.40,  h * 0.21, 215.0, 26),
        (cx + w * 0.40,  h * 0.21, 215.0, 26),
        // Nivel medio - sobre las ramas principales
        (cx - w * 0.15,  h * 0.25, 235.0, 30),
        (cx + w * 0.15,  h * 0.25, 235.0, 30),
        (cx - w * 0.32,  h * 0.29, 220.0, 28),
        (cx + w * 0.32,  h * 0.29, 220.0, 28),
        // Nivel inferior - follaje tupido enmarcando el tronco
        (cx - w * 0.44,  h * 0.35, 195.0, 24),
        (cx + w * 0.44,  h * 0.35, 195.0, 24),
        (cx - w * 0.24,  h * 0.38, 205.0, 26),
        (cx + w * 0.24,  h * 0.38, 205.0, 26),
        (cx,             h * 0.30, 215.0, 28),
    ];

    let leaf_cols = [leaf_dark, leaf_mid, leaf_mid, leaf_bright, leaf_bright, leaf_sun, leaf_gold];
    let mut leaf_seed: u32 = 0x7A3B1C;

    for &(lcx, lcy, lr, n_leaves) in leaf_clusters {
        // Capa de masa de sombra verde profunda
        let mut mass_pb = tiny_skia::PathBuilder::new();
        mass_pb.move_to(lcx - lr * 0.90, lcy);
        mass_pb.cubic_to(lcx - lr * 0.85, lcy - lr * 0.75, lcx + lr * 0.85, lcy - lr * 0.75, lcx + lr * 0.90, lcy);
        mass_pb.cubic_to(lcx + lr * 0.80, lcy + lr * 0.50, lcx - lr * 0.80, lcy + lr * 0.50, lcx - lr * 0.90, lcy);
        mass_pb.close();
        if let Some(mp) = mass_pb.finish() {
            canvas.fill_path(&mp, leaf_deep_shadow.with_alpha(0.85));
        }

        // Hojas individuales lanceoladas (forma de lanza, realista de castaño)
        for i in 0..n_leaves {
            // Generador LCG para posición pseudoaleatoria reproducible
            leaf_seed = leaf_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let t = (leaf_seed >> 8) as f32 / (u32::MAX as f32);
            leaf_seed = leaf_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let u_r = (leaf_seed >> 8) as f32 / (u32::MAX as f32);

            let angle = t * std::f32::consts::PI * 2.0;
            let r = (0.15 + u_r * 0.85) * lr;
            let lx = lcx + angle.cos() * r;
            let ly = lcy + angle.sin() * r * 0.72;

            leaf_seed = leaf_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let orient = (leaf_seed >> 8) as f32 / (u32::MAX as f32) * std::f32::consts::PI * 2.0;
            leaf_seed = leaf_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let sz = 34.0 + ((leaf_seed >> 8) as f32 / (u32::MAX as f32)) * 60.0;

            let col_idx = (i * 7 / n_leaves).min(6);
            let leaf_col = leaf_cols[col_idx];

            // Silueta de hoja lanceolada de castaño
            let mut leaf_pb = tiny_skia::PathBuilder::new();
            leaf_pb.move_to(lx + orient.cos() * sz * 0.5, ly + orient.sin() * sz * 0.5); // Ápice
            leaf_pb.cubic_to(
                lx + (-orient.sin()) * sz * 0.22 + orient.cos() * sz * 0.1,
                ly + orient.cos() * sz * 0.22 + orient.sin() * sz * 0.1,
                lx + (-orient.sin()) * sz * 0.18 - orient.cos() * sz * 0.35,
                ly + orient.cos() * sz * 0.18 - orient.sin() * sz * 0.35,
                lx - orient.cos() * sz * 0.5, ly - orient.sin() * sz * 0.5, // Base
            );
            leaf_pb.cubic_to(
                lx + orient.sin() * sz * 0.18 - orient.cos() * sz * 0.35,
                ly - orient.cos() * sz * 0.18 - orient.sin() * sz * 0.35,
                lx + orient.sin() * sz * 0.22 + orient.cos() * sz * 0.1,
                ly - orient.cos() * sz * 0.22 + orient.sin() * sz * 0.1,
                lx + orient.cos() * sz * 0.5, ly + orient.sin() * sz * 0.5,
            );
            leaf_pb.close();
            if let Some(lp) = leaf_pb.finish() {
                canvas.fill_path(&lp, leaf_col);
                canvas.stroke_path_fine(&lp, leaf_dark, 0.9);
                // Nervadura central
                canvas.stroke_line_fine(
                    lx - orient.cos() * sz * 0.45, ly - orient.sin() * sz * 0.45,
                    lx + orient.cos() * sz * 0.45, ly + orient.sin() * sz * 0.45,
                    leaf_vein.with_alpha(0.80), 1.3,
                );
            }
        }
    }

    // ===== ERIZOS DE CASTAÑA (Castañas silvestres con espinas doradas) =====
    let bur_locations = [
        (cx - w * 0.28, h * 0.27),
        (cx - w * 0.14, h * 0.31),
        (cx + w * 0.16, h * 0.29),
        (cx + w * 0.30, h * 0.26),
        (cx - w * 0.38, h * 0.35),
        (cx + w * 0.36, h * 0.33),
    ];
    for &(bx, by) in &bur_locations {
        for bur_i in 0..3 {
            let offset_x = (bur_i as f32 - 1.0) * 14.0;
            let offset_y = ((bur_i as f32 * 1.5).sin()) * 8.0;
            let b_cx = bx + offset_x;
            let b_cy = by + offset_y;
            // Erizos esféricos
            canvas.fill_circle(b_cx, b_cy, 12.0, bur_col);
            canvas.stroke_circle(b_cx, b_cy, 12.0, leaf_dark, 1.2);
            // Espinas radiantes
            for sp in 0..12 {
                let spa = (sp as f32 / 12.0) * PI * 2.0;
                let sx1 = b_cx + spa.cos() * 10.0;
                let sy1 = b_cy + spa.sin() * 10.0;
                let sx2 = b_cx + spa.cos() * 18.0;
                let sy2 = b_cy + spa.sin() * 18.0;
                canvas.stroke_line_fine(sx1, sy1, sx2, sy2, spike_col, 1.5);
            }
        }
    }

    // Musgo español y lianas colgando de ramas
    for m in 0..24 {
        let mx = cx - w * 0.46 + (m as f32) * (w * 0.92 / 23.0);
        let my = h * 0.21 + ((m as f32 * 1.7).sin()) * 30.0;
        let m_len = 95.0 + ((m as f32 * 2.3).cos()) * 55.0;
        let mut moss_pb = tiny_skia::PathBuilder::new();
        moss_pb.move_to(mx, my);
        moss_pb.cubic_to(mx + 15.0, my + m_len * 0.35, mx - 12.0, my + m_len * 0.7, mx + 6.0, my + m_len);
        if let Some(mp) = moss_pb.finish() {
            canvas.stroke_path_fine(&mp, leaf_mid.with_alpha(0.70), 3.8);
            canvas.stroke_path_fine(&mp, leaf_bright.with_alpha(0.45), 1.8);
            canvas.stroke_path_fine(&mp, leaf_gold.with_alpha(0.35), 0.9);
        }
    }

    // ===== CADENAS DE HIERRO FORJADO QUE RODEAN EL TRONCO =====
    let chain_levels = [h * 0.52, h * 0.57, h * 0.63];
    for (idx, &cl_y) in chain_levels.iter().enumerate() {
        let cw_chain = trunk_base_w * (1.0 + idx as f32 * 0.18);
        let link_count = 22;
        for l in 0..link_count {
            let lx = cx - cw_chain + (l as f32) * (cw_chain * 2.0 / (link_count - 1) as f32);
            let ly = cl_y + ((l as f32 * 0.85).sin()) * 6.0;
            // Eslabón oval
            canvas.fill_circle(lx, ly, 9.5, chain_iron);
            canvas.stroke_circle(lx, ly, 9.5, bark_deep, 2.8);
            canvas.stroke_circle(lx, ly, 7.5, chain_highlight, 1.2);
            // Óxido en la cadena
            if l % 4 == 0 {
                canvas.fill_circle(lx, ly, 5.0, rust_col.with_alpha(0.60));
            }
        }
    }
    // Candado arcaico de hierro forjado
    canvas.fill_rect(cx - 22.0, h * 0.57 - 22.0, 44.0, 52.0, chain_iron);
    canvas.stroke_rect(cx - 22.0, h * 0.57 - 22.0, 44.0, 52.0, bark_deep, 3.0);
    canvas.stroke_circle(cx, h * 0.57 - 28.0, 18.0, chain_iron, 5.5);
    canvas.stroke_circle(cx, h * 0.57 - 28.0, 14.0, chain_highlight, 1.8);
    // Ojo del candado
    canvas.fill_circle(cx, h * 0.57 + 2.0, 5.0, bark_deep);

    // ===== DON JOSÉ ARCADIO BUENDÍA ENCADENADO =====
    let ja_x = cx + trunk_base_w * 0.65;
    let ja_y = ground_y - 10.0;
    // Sombra en el suelo
    canvas.fill_circle(ja_x + 15.0, ja_y + 8.0, 40.0, Color::hex("#1A0D05").with_alpha(0.25));
    // Piernas dobladas sentado
    let mut leg_pb = tiny_skia::PathBuilder::new();
    leg_pb.move_to(ja_x - 28.0, ja_y);
    leg_pb.cubic_to(ja_x - 10.0, ja_y - 18.0, ja_x + 38.0, ja_y - 10.0, ja_x + 55.0, ja_y + 5.0);
    if let Some(lp) = leg_pb.finish() {
        canvas.stroke_path_fine(&lp, Color::hex("#B8AA92"), 22.0);
        canvas.stroke_path_fine(&lp, Color::hex("#8A7D62"), 3.0);
    }
    // Camisón blanco de lino desgastado
    let mut ja_pb = tiny_skia::PathBuilder::new();
    ja_pb.move_to(ja_x - 32.0, ja_y - 8.0);
    ja_pb.line_to(ja_x - 28.0, ja_y - 110.0);
    ja_pb.cubic_to(ja_x - 10.0, ja_y - 118.0, ja_x + 18.0, ja_y - 116.0, ja_x + 32.0, ja_y - 108.0);
    ja_pb.line_to(ja_x + 40.0, ja_y - 8.0);
    ja_pb.close();
    if let Some(jp) = ja_pb.finish() {
        canvas.fill_path(&jp, Color::hex("#EBE5D8"));
        canvas.stroke_path_fine(&jp, Color::hex("#8C7D62"), 2.5);
        // Arrugas de la tela
        for wr in 0..5 {
            let wry = ja_y - 20.0 - (wr as f32) * 18.0;
            canvas.stroke_line_fine(ja_x - 24.0, wry, ja_x + 32.0, wry + 4.0, Color::hex("#C2B89E"), 1.2);
        }
    }
    // Cabeza inclinada hacia abajo (mirada perdida)
    canvas.fill_circle(ja_x, ja_y - 128.0, 28.0, Color::hex("#D4A882"));
    canvas.stroke_circle(ja_x, ja_y - 128.0, 28.0, Color::hex("#6B4025"), 2.2);
    // Barba blanca larga y espesa
    let mut beard_pb = tiny_skia::PathBuilder::new();
    beard_pb.move_to(ja_x - 18.0, ja_y - 116.0);
    beard_pb.cubic_to(ja_x - 25.0, ja_y - 85.0, ja_x - 20.0, ja_y - 60.0, ja_x - 8.0, ja_y - 50.0);
    beard_pb.cubic_to(ja_x + 8.0, ja_y - 50.0, ja_x + 22.0, ja_y - 65.0, ja_x + 20.0, ja_y - 116.0);
    beard_pb.close();
    if let Some(brd) = beard_pb.finish() {
        canvas.fill_path(&brd, Color::hex("#F0ECDF"));
        canvas.stroke_path_fine(&brd, Color::hex("#B8B0A0"), 1.5);
    }
    // Ojos entreabiertos, mirada al vacío
    canvas.fill_circle(ja_x - 10.0, ja_y - 133.0, 5.0, Color::hex("#2C1A0A"));
    canvas.fill_circle(ja_x + 10.0, ja_y - 133.0, 5.0, Color::hex("#2C1A0A"));
    // Partículas de palabras en latín flotando
    for p in 0..8 {
        let px = ja_x - 60.0 + (p as f32) * 18.0;
        let py = ja_y - 145.0 - (p as f32 * 2.1).sin() * 20.0;
        canvas.fill_circle(px, py, 2.0, Color::hex("#C8A84A").with_alpha(0.55));
    }
}

// =============================================================================
// 12. TREN BANANERO HISTÓRICO MONUMENTAL (Baldwin 1910 y Vagones Amarillos)
// =============================================================================

pub fn draw_master_locomotora_bananera(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    tau: f32,
) {
    let boiler_black = Color::hex("#181B22");
    let iron_trim = Color::hex("#3A3E4A");
    let brass_gold = Color::hex("#E0B838");
    let rail_steel = Color::hex("#757C8A");
    let yellow_car = Color::hex("#EAA820");
    let banana_green = Color::hex("#508A1C");

    let track_y = cy + 45.0;

    // 1. Vías del ferrocarril con traviesas de guayacán
    let sleeper_count = 32;
    for s in 0..sleeper_count {
        let sx = (s as f32 / (sleeper_count - 1) as f32) * w;
        canvas.fill_rect(sx - 12.0, track_y - 8.0, 24.0, 32.0, Color::hex("#351F0E"));
        canvas.stroke_rect(sx - 12.0, track_y - 8.0, 24.0, 32.0, Color::hex("#1C1006"), 1.8);
    }
    // Rieles de acero
    canvas.stroke_line_fine(0.0, track_y, w, track_y, rail_steel, 5.5);
    canvas.stroke_line_fine(0.0, track_y + 16.0, w, track_y + 16.0, rail_steel, 5.5);
    canvas.stroke_line_fine(0.0, track_y, w, track_y, Color::WHITE.with_alpha(0.85), 1.5);

    // Posición del tren desplazándose
    let train_x = cx + ((tau * 3.5) % 1.0) * 140.0 - 70.0;

    canvas.save();
    canvas.translate(train_x, cy);
    canvas.scale(1.30, 1.30); // Escala monumental 1.30x

    // 2. Vagones de carga amarillos con plátanos verdes
    for car in 0..2 {
        let car_x = -240.0 - (car as f32) * 220.0;
        // Chasis del vagón
        canvas.fill_rect(car_x, -50.0, 200.0, 70.0, yellow_car);
        canvas.stroke_rect(car_x, -50.0, 200.0, 70.0, Color::hex("#442E06"), 2.8);

        // Estructura de tablas de madera
        for t in 0..9 {
            let tx = car_x + (t as f32) * 22.0;
            canvas.stroke_line_fine(tx, -50.0, tx, 20.0, Color::hex("#684A0E"), 1.8);
        }

        // Racimos de banano que sobresalen del vagón
        for b in 0..7 {
            let bx = car_x + 16.0 + (b as f32) * 26.0;
            canvas.fill_circle(bx, -60.0, 15.0, banana_green);
            canvas.stroke_circle(bx, -60.0, 15.0, Color::hex("#26460A"), 1.6);
        }

        // Ruedas del vagón
        for w_pos in [car_x + 42.0, car_x + 158.0] {
            canvas.fill_circle(w_pos, 28.0, 15.0, iron_trim);
            canvas.stroke_circle(w_pos, 28.0, 15.0, Color::hex("#111317"), 2.2);
        }
    }

    // 3. Locomotora a vapor Baldwin (Caldera, Cabina, Chimenea, Fanal)
    let loco_x = -15.0;

    // Cabina de maquinista
    canvas.fill_rect(loco_x - 105.0, -105.0, 95.0, 125.0, boiler_black);
    canvas.stroke_rect(loco_x - 105.0, -105.0, 95.0, 125.0, brass_gold, 2.8);
    // Ventana de cabina
    canvas.fill_rect(loco_x - 90.0, -88.0, 42.0, 35.0, Color::hex("#F6F2E8"));
    canvas.stroke_rect(loco_x - 90.0, -88.0, 42.0, 35.0, iron_trim, 2.2);

    // Caldera cilíndrica horizontal
    canvas.fill_rect(loco_x - 10.0, -82.0, 190.0, 92.0, boiler_black);
    canvas.stroke_rect(loco_x - 10.0, -82.0, 190.0, 92.0, iron_trim, 2.8);

    // Bandas de latón reluciente de la caldera
    for b in 0..4 {
        let bx = loco_x + 18.0 + (b as f32) * 48.0;
        canvas.fill_rect(bx, -82.0, 7.0, 92.0, brass_gold);
    }

    // Chimenea cónica echando humo
    let stack_x = loco_x + 145.0;
    canvas.fill_rect(stack_x - 14.0, -145.0, 28.0, 65.0, boiler_black);
    canvas.stroke_rect(stack_x - 14.0, -145.0, 28.0, 65.0, iron_trim, 2.2);
    // Remate acampanado de latón
    canvas.fill_rect(stack_x - 20.0, -155.0, 40.0, 14.0, brass_gold);

    // Volutas de humo en grabado
    for sm in 0..5 {
        let sm_x = stack_x - (sm as f32) * 60.0 - (tau * 140.0) % 70.0;
        let sm_y = -175.0 - (sm as f32) * 32.0;
        let sm_r = 18.0 + (sm as f32) * 10.0;
        canvas.fill_circle(sm_x, sm_y, sm_r, Color::hex("#EDE7DC").with_alpha(0.75));
        canvas.stroke_circle(sm_x, sm_y, sm_r, Color::hex("#9C9282"), 1.8);
    }

    // Quitapiedras (Cowcatcher) triangular en el frente
    let cow_x = loco_x + 180.0;
    let mut cow_pb = tiny_skia::PathBuilder::new();
    cow_pb.move_to(cow_x, -12.0);
    cow_pb.line_to(cow_x + 55.0, 28.0);
    cow_pb.line_to(cow_x, 28.0);
    cow_pb.close();
    if let Some(cp) = cow_pb.finish() {
        canvas.fill_path(&cp, iron_trim);
        canvas.stroke_path_fine(&cp, brass_gold, 2.2);
    }

    // Fanal delantero proyectando un cono de luz dorada
    let light_x = cow_x + 12.0;
    let light_y = -48.0;
    canvas.fill_circle(light_x, light_y, 16.0, brass_gold);
    canvas.stroke_circle(light_x, light_y, 16.0, Color::WHITE, 2.2);

    // Haz de luz en BlendMode::Screen
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);
    let mut beam_pb = tiny_skia::PathBuilder::new();
    beam_pb.move_to(light_x + 16.0, light_y);
    beam_pb.line_to(light_x + 450.0, light_y - 100.0);
    beam_pb.line_to(light_x + 450.0, light_y + 120.0);
    beam_pb.close();
    if let Some(bp) = beam_pb.finish() {
        canvas.fill_path(&bp, Color::hex("#FFF099").with_alpha(0.30));
    }
    canvas.restore();

    // Ruedas motrices gigantes con biela de acero
    let wheels = [loco_x + 28.0, loco_x + 92.0, loco_x + 156.0];
    let wheel_ang = tau * 14.0 * PI;
    for &wx in &wheels {
        canvas.fill_circle(wx, 24.0, 24.0, iron_trim);
        canvas.stroke_circle(wx, 24.0, 24.0, brass_gold, 3.2);
        // Radios de la rueda
        for rd in 0..6 {
            let ra = wheel_ang + (rd as f32) * PI / 3.0;
            canvas.stroke_line_fine(wx, 24.0, wx + ra.cos() * 22.0, 24.0 + ra.sin() * 22.0, Color::WHITE.with_alpha(0.75), 1.6);
        }
    }

    // Biela horizontal de transmisión
    let crank_y = 24.0 + wheel_ang.sin() * 12.0;
    canvas.stroke_line_fine(wheels[0], crank_y, wheels[2], crank_y, brass_gold, 5.5);

    canvas.restore();
}


// =============================================================================
// 13. OBOE PROFESIONAL MONUMENTAL (El Embrujo de Melquíades)
//     Diseño masivo con cuerpo ancho, pabellón acampanado y llaves enormes
// =============================================================================

pub fn draw_master_oboe_monumental(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
) {
    let wood_dark = Color::hex("#120A06");
    let wood_mid = Color::hex("#28160E");
    let wood_sheen = Color::hex("#4E2E1D");
    let rim_cyan = Color::hex("#00E5C8"); // Contorno luminoso blueprint para contraste total
    let silver_key = Color::hex("#D2E0E8");
    let silver_bright = Color::hex("#FFFFFF");
    let silver_shadow = Color::hex("#587082");
    let gold_staple = Color::hex("#E5B834");
    let silk_red = Color::hex("#E52525");
    let reed_cane = Color::hex("#F6E2A4");
    let reed_tip = Color::hex("#FFEAA7");
    let pad_amber = Color::hex("#FFB300");

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    // Dimensiones monumentales y equilibradas para legibilidad máxima
    let body_len = 820.0 * scale;
    let body_top_w = 48.0 * scale;  // Mucho más ancho y visible
    let body_bot_w = 112.0 * scale; // Pabellón acampanado expansivo
    let top_y = -body_len * 0.48;
    let bot_y = body_len * 0.52;

    // ===== 1. RESPLANDOR Y AURA ESQUEMÁTICA / VIBRACIÓN ACÚSTICA =====
    canvas.save();
    canvas.set_blend_mode(tiny_skia::BlendMode::Screen);
    for w_idx in 0..5 {
        let phase = ((tau * 3.5 + w_idx as f32 * 0.22) % 1.0).abs();
        let wr = 50.0 * scale + phase * 320.0 * scale;
        let alpha = (1.0 - phase) * 0.60;
        canvas.stroke_circle(0.0, bot_y + 40.0 * scale, wr, rim_cyan.with_alpha(alpha), 2.2 * scale);
    }
    canvas.restore();

    // ===== 2. CAÑA DOBLE REALISTA (Oboe Double Reed) =====
    let reed_len = 70.0 * scale;
    let reed_top_y = top_y - reed_len;

    // Tudel de inserción metálico en corcho
    canvas.fill_rect(-7.0 * scale, top_y - 26.0 * scale, 14.0 * scale, 26.0 * scale, gold_staple);
    canvas.stroke_rect(-7.0 * scale, top_y - 26.0 * scale, 14.0 * scale, 26.0 * scale, wood_dark, 2.0 * scale);
    canvas.stroke_line_fine(-5.0 * scale, top_y - 13.0 * scale, 5.0 * scale, top_y - 13.0 * scale, silver_bright, 1.2 * scale);

    // Hilo de seda rojo escarlata enrollado con precisión
    for th in 0..12 {
        let ty = top_y - 52.0 * scale + (th as f32) * 2.2 * scale;
        canvas.stroke_line_fine(-9.5 * scale, ty, 9.5 * scale, ty, silk_red, 2.2 * scale);
    }
    canvas.stroke_line_fine(0.0, top_y - 53.0 * scale, 0.0, top_y - 26.0 * scale, Color::hex("#FF6B6B"), 1.0 * scale);

    // Caña de bambú doble (dos paletas finas con vibración)
    let reed_vib = (tau * 36.0 * PI).sin() * 1.5 * scale;
    let mut reed_l = tiny_skia::PathBuilder::new();
    reed_l.move_to(-3.5 * scale, top_y - 52.0 * scale);
    reed_l.cubic_to(-11.0 * scale, top_y - 70.0 * scale, -12.0 * scale + reed_vib, reed_top_y + 10.0 * scale, 0.0, reed_top_y);
    reed_l.line_to(-4.0 * scale, top_y - 52.0 * scale);
    reed_l.close();
    if let Some(rp) = reed_l.finish() {
        canvas.fill_path(&rp, reed_cane);
        canvas.stroke_path_fine(&rp, Color::hex("#A67C1E"), 1.2 * scale);
        // Punta translúcida
        canvas.fill_circle(0.0, reed_top_y + 4.0 * scale, 4.0 * scale, reed_tip);
    }

    let mut reed_r = tiny_skia::PathBuilder::new();
    reed_r.move_to(3.5 * scale, top_y - 52.0 * scale);
    reed_r.cubic_to(11.0 * scale, top_y - 70.0 * scale, 12.0 * scale - reed_vib, reed_top_y + 10.0 * scale, 0.0, reed_top_y);
    reed_r.line_to(4.0 * scale, top_y - 52.0 * scale);
    reed_r.close();
    if let Some(rp) = reed_r.finish() {
        canvas.fill_path(&rp, reed_cane);
        canvas.stroke_path_fine(&rp, Color::hex("#A67C1E"), 1.2 * scale);
    }

    // ===== 3. CUERPO CÓNICO DE GRANADILLA CON CONTORNO ILUMINADO =====
    // Perfil cónico auténtico del oboe
    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(-body_top_w * 0.5, top_y);
    body_pb.cubic_to(
        -body_top_w * 0.55, top_y + body_len * 0.35,
        -body_bot_w * 0.42, bot_y - body_len * 0.20,
        -body_bot_w * 0.5, bot_y,
    );
    body_pb.line_to(body_bot_w * 0.5, bot_y);
    body_pb.cubic_to(
        body_bot_w * 0.42, bot_y - body_len * 0.20,
        body_top_w * 0.55, top_y + body_len * 0.35,
        body_top_w * 0.5, top_y,
    );
    body_pb.close();

    if let Some(bp) = body_pb.finish() {
        // Resplandor exterior blueprint para recorte nítido
        canvas.stroke_path_fine(&bp, rim_cyan.with_alpha(0.70), 5.5 * scale);
        // Cuerpo de madera noble
        canvas.fill_path(&bp, wood_mid);
        canvas.stroke_path_fine(&bp, wood_dark, 3.5 * scale);

        // Brillo longitudinal en la madera
        for s in 0..16 {
            let t = s as f32 / 15.0;
            let sx_top = -body_top_w * 0.42 + t * body_top_w * 0.84;
            let sx_bot = -body_bot_w * 0.42 + t * body_bot_w * 0.84;
            let col = if s == 4 || s == 5 { wood_sheen } else { wood_dark };
            canvas.stroke_line_fine(sx_top, top_y + 6.0 * scale, sx_bot, bot_y - 20.0 * scale, col, 1.4 * scale);
        }
        // Reflejo central blanco translúcido
        let r_top = -body_top_w * 0.15;
        let r_bot = -body_bot_w * 0.15;
        canvas.stroke_line_fine(r_top, top_y + 10.0 * scale, r_bot, bot_y - 30.0 * scale, silver_bright.with_alpha(0.18), 3.0 * scale);
    }

    // ===== 4. ANILLOS DE UNIÓN ENTRE CUERPOS (Plata niquelada y espigas) =====
    let joint1_y = top_y + body_len * 0.36;  // Unión upper/lower joint
    let joint2_y = top_y + body_len * 0.72;  // Unión lower joint/bell
    let w_at_joint1 = body_top_w + (body_bot_w - body_top_w) * 0.36;
    let w_at_joint2 = body_top_w + (body_bot_w - body_top_w) * 0.72;

    for &(jy, jw) in &[(joint1_y, w_at_joint1), (joint2_y, w_at_joint2)] {
        // Anillo de plata brillante
        canvas.fill_rect(-jw * 0.62, jy - 7.0 * scale, jw * 1.24, 15.0 * scale, silver_key);
        canvas.stroke_rect(-jw * 0.62, jy - 7.0 * scale, jw * 1.24, 15.0 * scale, wood_dark, 2.0 * scale);
        canvas.stroke_line_fine(-jw * 0.58, jy - 2.0 * scale, jw * 0.58, jy - 2.0 * scale, silver_bright, 2.0 * scale);
        // Borde cyan blueprint
        canvas.stroke_line_fine(-jw * 0.60, jy + 6.0 * scale, jw * 0.60, jy + 6.0 * scale, rim_cyan.with_alpha(0.60), 1.0 * scale);
    }

    // ===== 5. PABELLÓN ACAMPANADO (BELL) Y ANILLO ENGRAVADO =====
    let mut bell_pb = tiny_skia::PathBuilder::new();
    bell_pb.move_to(-body_bot_w * 0.50, bot_y - 10.0 * scale);
    bell_pb.cubic_to(
        -body_bot_w * 0.65, bot_y + 16.0 * scale,
        -body_bot_w * 0.90, bot_y + 45.0 * scale,
        -body_bot_w * 1.10, bot_y + 70.0 * scale,
    );
    bell_pb.line_to(body_bot_w * 1.10, bot_y + 70.0 * scale);
    bell_pb.cubic_to(
        body_bot_w * 0.90, bot_y + 45.0 * scale,
        body_bot_w * 0.65, bot_y + 16.0 * scale,
        body_bot_w * 0.50, bot_y - 10.0 * scale,
    );
    bell_pb.close();
    if let Some(bellp) = bell_pb.finish() {
        canvas.stroke_path_fine(&bellp, rim_cyan.with_alpha(0.70), 5.0 * scale);
        canvas.fill_path(&bellp, wood_mid);
        canvas.stroke_path_fine(&bellp, wood_dark, 3.5 * scale);
    }

    // Anillo monumental del pabellón en marfil y plata
    let ring_w = body_bot_w * 2.24;
    canvas.fill_rect(-ring_w * 0.5, bot_y + 64.0 * scale, ring_w, 16.0 * scale, silver_key);
    canvas.stroke_rect(-ring_w * 0.5, bot_y + 64.0 * scale, ring_w, 16.0 * scale, wood_dark, 2.5 * scale);
    canvas.stroke_line_fine(-ring_w * 0.48, bot_y + 70.0 * scale, ring_w * 0.48, bot_y + 70.0 * scale, silver_bright, 2.5 * scale);
    // Interior hueco oscuro del pabellón
    canvas.fill_circle(0.0, bot_y + 75.0 * scale, body_bot_w * 0.85, Color::hex("#080503"));
    canvas.stroke_circle(0.0, bot_y + 75.0 * scale, body_bot_w * 0.85, rim_cyan.with_alpha(0.50), 1.5 * scale);

    // ===== 6. VARILLAS LONGITUDINALES Y POSTES DE PIVOTE (Mecanismo de agujas) =====
    let w_at = |t: f32| -> f32 { body_top_w + (body_bot_w - body_top_w) * t };

    // Dos barras espinales de acero cromado a ambos lados
    for &side in &[-1.0_f32, 1.0_f32] {
        let rod_x_top = side * (body_top_w * 0.38 + 6.0 * scale);
        let rod_x_bot = side * (body_bot_w * 0.38 + 10.0 * scale);
        canvas.stroke_line_fine(rod_x_top, top_y + 20.0 * scale, rod_x_bot, bot_y + 10.0 * scale, silver_shadow, 4.0 * scale);
        canvas.stroke_line_fine(rod_x_top, top_y + 20.0 * scale, rod_x_bot, bot_y + 10.0 * scale, silver_bright, 1.8 * scale);

        // Postes de anclaje (posts)
        for p in 0..7 {
            let pt = 0.08 + (p as f32) * 0.12;
            let py = top_y + body_len * pt;
            let px = side * (w_at(pt) * 0.38 + 8.0 * scale);
            canvas.fill_circle(px, py, 4.5 * scale, silver_bright);
            canvas.stroke_circle(px, py, 4.5 * scale, wood_dark, 1.2 * scale);
        }
    }

    // ===== 7. PLATOS CIRCULARES DEL SISTEMA DE LLAVES FRANCÉS (PLATEAU KEYS) =====
    // 12 llaves principales de gran tamaño con zapatillas de corcho y orificios resonantes
    let key_configs: &[(f32, f32, f32)] = &[
        // (t_pos, side, radius)
        (0.07, -1.0, 16.0),
        (0.12,  1.0, 15.0),
        (0.17, -1.0, 17.0),
        (0.23,  1.0, 16.0),
        (0.29, -1.0, 18.0),
        (0.35,  1.0, 17.0),
        (0.42, -1.0, 18.0),
        (0.48,  1.0, 18.0),
        (0.55, -1.0, 19.0),
        (0.62,  1.0, 19.0),
        (0.69, -1.0, 18.0),
        (0.76,  1.0, 20.0),
    ];

    for &(t, side, r_base) in key_configs {
        let ky = top_y + body_len * t;
        let bw = w_at(t);
        let kx = side * (bw * 0.48 + 16.0 * scale);
        let kr = r_base * scale;

        // Brazo de conexión de la llave (key arm)
        canvas.stroke_line_fine(side * bw * 0.25, ky, kx, ky, silver_shadow, 5.0 * scale);
        canvas.stroke_line_fine(side * bw * 0.25, ky, kx, ky, silver_bright, 2.5 * scale);

        // Orificio tonal interior (chimenea acústica brillante)
        let vib_phase = ((tau * 24.0 + t * 20.0).sin()).abs();
        canvas.fill_circle(side * bw * 0.25, ky, kr * 0.65, Color::hex("#0A0604"));
        canvas.stroke_circle(side * bw * 0.25, ky, kr * 0.65, rim_cyan.with_alpha(0.60), 1.2 * scale);
        if vib_phase > 0.4 {
            canvas.fill_circle(side * bw * 0.25, ky, kr * 0.45, pad_amber.with_alpha(0.85));
        }

        // Plato circular exterior (cup / pad)
        canvas.fill_circle(kx, ky, kr, silver_shadow);
        canvas.fill_circle(kx, ky, kr * 0.86, silver_key);
        canvas.stroke_circle(kx, ky, kr, wood_dark, 2.0 * scale);
        canvas.stroke_circle(kx, ky, kr * 0.86, silver_bright, 1.5 * scale);

        // Cúpula central abovedada
        canvas.fill_circle(kx, ky, kr * 0.42, silver_bright);
        // Brillo especular
        canvas.fill_circle(kx - kr * 0.25, ky - kr * 0.25, kr * 0.24, Color::WHITE);
    }

    // ===== 8. LLAVES DE TRINOL Y OCTAVA (Palancas superiores y laterales) =====
    // Llaves de octava cerca del tudel
    canvas.stroke_line_fine(-12.0 * scale, top_y + 15.0 * scale, -24.0 * scale, top_y + 2.0 * scale, silver_bright, 3.2 * scale);
    canvas.fill_circle(-25.0 * scale, top_y + 2.0 * scale, 6.5 * scale, silver_key);
    canvas.stroke_circle(-25.0 * scale, top_y + 2.0 * scale, 6.5 * scale, wood_dark, 1.2 * scale);

    canvas.stroke_line_fine(12.0 * scale, top_y + 25.0 * scale, 26.0 * scale, top_y + 12.0 * scale, silver_bright, 3.2 * scale);
    canvas.fill_circle(27.0 * scale, top_y + 12.0 * scale, 6.5 * scale, silver_key);
    canvas.stroke_circle(27.0 * scale, top_y + 12.0 * scale, 6.5 * scale, wood_dark, 1.2 * scale);

    // Apoyapulgar (Thumb rest) en el tercio inferior
    let tr_y = top_y + body_len * 0.60;
    canvas.fill_rect(0.0, tr_y - 6.0 * scale, 16.0 * scale, 12.0 * scale, silver_key);
    canvas.stroke_rect(0.0, tr_y - 6.0 * scale, 16.0 * scale, 12.0 * scale, wood_dark, 1.5 * scale);

    canvas.restore();
}

// =============================================================================
// 14. ASTROLABIO ALQUÍMICO & ESFERA ARMILAR MONUMENTAL (Melquíades)
//     Obra maestra de navegación astronómica, círculos graduados, zodiaco y alidada
// =============================================================================

pub fn draw_master_astrolabio_armilar_alquimia(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let brass_gold = Color::hex("#E5B834");
    let brass_light = Color::hex("#FFE082");
    let brass_dark = Color::hex("#8C6B16");
    let cyan_line = Color::hex("#80D0FF");
    let cyan_bright = Color::hex("#00E5C8");
    let alchemical_red = Color::hex("#FF5252");
    let parchment_col = Color::hex("#F6F0DC");

    canvas.save();
    canvas.translate(cx, cy);

    let r_outer = 250.0 * scale;

    // ===== 1. SUSPENSIÓN SUPERIOR DEL ASTROLABIO (Trono y Anilla de Latón) =====
    let throne_y = -r_outer - 35.0 * scale;
    // Anilla de suspensión
    canvas.stroke_circle(0.0, throne_y - 30.0 * scale, 34.0 * scale, brass_gold, 5.0 * scale);
    canvas.stroke_circle(0.0, throne_y - 30.0 * scale, 34.0 * scale, brass_light, 2.0 * scale);
    canvas.stroke_circle(0.0, throne_y - 30.0 * scale, 34.0 * scale, brass_dark, 1.2 * scale);

    // Trono calado barroco de unión al limbo
    let mut throne_pb = tiny_skia::PathBuilder::new();
    throne_pb.move_to(-48.0 * scale, -r_outer + 4.0 * scale);
    throne_pb.cubic_to(-42.0 * scale, throne_y - 5.0 * scale, -25.0 * scale, throne_y - 20.0 * scale, 0.0, throne_y - 24.0 * scale);
    throne_pb.cubic_to(25.0 * scale, throne_y - 20.0 * scale, 42.0 * scale, throne_y - 5.0 * scale, 48.0 * scale, -r_outer + 4.0 * scale);
    throne_pb.close();
    if let Some(tp) = throne_pb.finish() {
        canvas.fill_path(&tp, Color::hex("#121E3E"));
        canvas.stroke_path_fine(&tp, brass_gold, 3.5 * scale);
        canvas.stroke_path_fine(&tp, brass_light, 1.5 * scale);
    }
    // Calados en el trono
    canvas.stroke_circle(-20.0 * scale, throne_y, 8.0 * scale, cyan_bright, 1.5 * scale);
    canvas.stroke_circle(20.0 * scale, throne_y, 8.0 * scale, cyan_bright, 1.5 * scale);
    canvas.stroke_circle(0.0, throne_y - 8.0 * scale, 9.0 * scale, brass_light, 1.8 * scale);

    // ===== 2. LIMBO Y MATER GRADUADA EXTERIOR (360° con división fina) =====
    // Fondo oscuro con retícula celestial
    canvas.fill_circle(0.0, 0.0, r_outer, Color::hex("#0D1B3E"));
    canvas.stroke_circle(0.0, 0.0, r_outer, brass_gold, 6.0 * scale);
    canvas.stroke_circle(0.0, 0.0, r_outer, brass_light, 2.0 * scale);
    canvas.stroke_circle(0.0, 0.0, r_outer - 18.0 * scale, brass_dark, 2.5 * scale);
    canvas.stroke_circle(0.0, 0.0, r_outer - 28.0 * scale, cyan_line.with_alpha(0.60), 1.5 * scale);

    // 360 marcas de graduación en el limbo
    for d in 0..72 {
        let da = (d as f32 / 72.0) * PI * 2.0;
        let is_major = d % 6 == 0;
        let is_mid = d % 2 == 0;
        let r1 = if is_major { r_outer - 26.0 * scale } else if is_mid { r_outer - 18.0 * scale } else { r_outer - 12.0 * scale };
        let th = if is_major { 2.5 * scale } else { 1.2 * scale };
        let col = if is_major { brass_light } else { brass_gold.with_alpha(0.70) };
        canvas.stroke_line_fine(da.cos() * r1, da.sin() * r1, da.cos() * (r_outer - 2.0 * scale), da.sin() * (r_outer - 2.0 * scale), col, th);
    }

    // Cuadrantes y números romanos
    let roman = ["XII", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI"];
    for (i, &num) in roman.iter().enumerate() {
        let na = (i as f32 / 12.0) * PI * 2.0 - PI * 0.5;
        let nr = r_outer - 42.0 * scale;
        draw_vector_text(canvas, na.cos() * nr, na.sin() * nr + 4.0 * scale, num, 11.0 * scale, brass_light, true);
    }

    // ===== 3. ESFERA ARMILAR: ANILLOS COLÚRICOS, ECUADOR Y TRÓPICOS =====
    let arm_r = r_outer - 55.0 * scale;

    // Resplandor celestial interior
    canvas.fill_circle(0.0, 0.0, arm_r, Color::hex("#081432"));
    canvas.stroke_circle(0.0, 0.0, arm_r, cyan_line, 2.2 * scale);

    // Coluro solsticial (eje vertical) y equinoccial (eje horizontal)
    canvas.stroke_line_fine(0.0, -arm_r, 0.0, arm_r, cyan_line.with_alpha(0.55), 1.4 * scale);
    canvas.stroke_line_fine(-arm_r, 0.0, arm_r, 0.0, cyan_line.with_alpha(0.55), 1.4 * scale);

    // Ecuador celeste (elipse horizontal)
    let eq_h = arm_r * 0.38;
    let mut eq_pb = tiny_skia::PathBuilder::new();
    eq_pb.move_to(-arm_r, 0.0);
    eq_pb.cubic_to(-arm_r, -eq_h * 1.33, arm_r, -eq_h * 1.33, arm_r, 0.0);
    eq_pb.cubic_to(arm_r, eq_h * 1.33, -arm_r, eq_h * 1.33, -arm_r, 0.0);
    eq_pb.close();
    if let Some(ep) = eq_pb.finish() {
        canvas.stroke_path_fine(&ep, cyan_bright, 2.0 * scale);
    }

    // Trópico de Cáncer y Trópico de Capricornio
    for &sign in &[-1.0_f32, 1.0_f32] {
        let ty = sign * arm_r * 0.42;
        let tw = (arm_r * arm_r - ty * ty).max(0.0).sqrt();
        let th = eq_h * 0.65;
        let mut tr_pb = tiny_skia::PathBuilder::new();
        tr_pb.move_to(-tw, ty);
        tr_pb.cubic_to(-tw, ty - th * 1.25, tw, ty - th * 1.25, tw, ty);
        tr_pb.cubic_to(tw, ty + th * 1.25, -tw, ty + th * 1.25, -tw, ty);
        tr_pb.close();
        if let Some(tp) = tr_pb.finish() {
            canvas.stroke_path_fine(&tp, cyan_line.with_alpha(0.40), 1.2 * scale);
        }
    }

    // ===== 4. BANDA ZODIACAL INCLINADA DE LA ECLÍPTICA (23.5° DE OBLICUIDAD) =====
    canvas.save();
    let ecliptic_rot = -0.41 + tau * 0.15; // Rotación celeste lenta
    canvas.rotate(ecliptic_rot);

    let ecl_w = arm_r * 0.95;
    let ecl_h = arm_r * 0.46;
    let mut ecl_pb = tiny_skia::PathBuilder::new();
    ecl_pb.move_to(-ecl_w, 0.0);
    ecl_pb.cubic_to(-ecl_w, -ecl_h * 1.33, ecl_w, -ecl_h * 1.33, ecl_w, 0.0);
    ecl_pb.cubic_to(ecl_w, ecl_h * 1.33, -ecl_w, ecl_h * 1.33, -ecl_w, 0.0);
    ecl_pb.close();
    if let Some(ec) = ecl_pb.finish() {
        canvas.stroke_path_fine(&ec, brass_gold, 4.5 * scale);
        canvas.stroke_path_fine(&ec, brass_light, 1.8 * scale);
    }

    // 12 Constelaciones zodiacales marcadas en la elipse de la eclíptica
    let zodiac_signs = ["ARIES", "TAURUS", "GEMINI", "CANCER", "LEO", "VIRGO", "LIBRA", "SCORPIO", "SAGITTARIUS", "CAPRICORN", "AQUARIUS", "PISCES"];
    for (zi, &z_name) in zodiac_signs.iter().enumerate() {
        let za = (zi as f32 / 12.0) * PI * 2.0;
        let zx = za.cos() * ecl_w;
        let zy = za.sin() * ecl_h;

        // Marcador estelar dorado
        canvas.fill_circle(zx, zy, 5.5 * scale, brass_light);
        canvas.stroke_circle(zx, zy, 5.5 * scale, alchemical_red, 1.2 * scale);
        // Destello de estrella
        canvas.stroke_line_fine(zx - 8.0 * scale, zy, zx + 8.0 * scale, zy, Color::WHITE, 1.2 * scale);
        canvas.stroke_line_fine(zx, zy - 8.0 * scale, zx, zy + 8.0 * scale, Color::WHITE, 1.2 * scale);

        // Abreviatura de 3 letras
        let abbrev = &z_name[..3];
        draw_vector_text(canvas, zx * 1.14, zy * 1.14 + 3.0 * scale, abbrev, 8.0 * scale, brass_light, true);
    }

    canvas.restore();

    // ===== 5. LA ARAÑA DEL ASTROLABIO (RETE CALADO CON ARABESCOS Y ESTRELLAS) =====
    // Punteros de estrellas de primera magnitud (Sirio, Vega, Aldebarán, Arturo, etc.)
    let stars: &[(&str, f32, f32)] = &[
        ("SIRIUS", 0.45, 0.72),
        ("VEGA", 1.85, 0.65),
        ("ALDEBARAN", 2.95, 0.55),
        ("ARCTURUS", 4.10, 0.68),
        ("RIGEL", 5.25, 0.78),
    ];

    for &(s_name, s_ang, s_rad_f) in stars {
        let sa = s_ang + tau * 0.25;
        let sr = arm_r * s_rad_f;
        let sx = sa.cos() * sr;
        let sy = sa.sin() * sr;

        // Puntero en forma de llama o colmillo de latón desde el centro
        let mut pointer_pb = tiny_skia::PathBuilder::new();
        pointer_pb.move_to(sx * 0.3, sy * 0.3);
        pointer_pb.cubic_to(sx * 0.6 + 10.0 * scale, sy * 0.6 - 10.0 * scale, sx - 6.0 * scale, sy - 6.0 * scale, sx, sy);
        pointer_pb.cubic_to(sx + 4.0 * scale, sy + 4.0 * scale, sx * 0.6 - 10.0 * scale, sy * 0.6 + 10.0 * scale, sx * 0.3, sy * 0.3);
        pointer_pb.close();
        if let Some(pp) = pointer_pb.finish() {
            canvas.fill_path(&pp, brass_gold.with_alpha(0.75));
            canvas.stroke_path_fine(&pp, brass_light, 1.2 * scale);
        }

        // Estrella brillante
        canvas.fill_circle(sx, sy, 4.5 * scale, Color::WHITE);
        canvas.stroke_circle(sx, sy, 7.5 * scale, cyan_bright, 1.2 * scale);
        draw_vector_text(canvas, sx, sy - 10.0 * scale, s_name, 7.5 * scale, cyan_line, true);
    }

    // ===== 6. TIERRA EN EL CENTRO ("La tierra es redonda como una naranja") =====
    let earth_r = 38.0 * scale;
    // Esfera terrestre en azul océano y latón
    canvas.fill_circle(0.0, 0.0, earth_r, Color::hex("#1D6E90"));
    canvas.stroke_circle(0.0, 0.0, earth_r, brass_gold, 3.0 * scale);
    canvas.stroke_circle(0.0, 0.0, earth_r, brass_light, 1.2 * scale);

    // Meridianos y paralelos terrestres
    for m in 0..4 {
        let ma = (m as f32 / 4.0) * PI;
        let mut m_pb = tiny_skia::PathBuilder::new();
        m_pb.move_to(0.0, -earth_r);
        m_pb.cubic_to(ma.cos() * earth_r * 1.1, 0.0, ma.cos() * earth_r * 1.1, 0.0, 0.0, earth_r);
        if let Some(mp) = m_pb.finish() {
            canvas.stroke_path_fine(&mp, cyan_line.with_alpha(0.65), 1.2 * scale);
        }
    }
    canvas.stroke_line_fine(-earth_r, 0.0, earth_r, 0.0, cyan_line.with_alpha(0.70), 1.2 * scale);

    // ===== 7. LA ALIDADA ROTANTE (Regla de mira de navegación con pínulas) =====
    canvas.save();
    let alidade_angle = tau * 2.0 * PI * 0.4 + 0.65;
    canvas.rotate(alidade_angle);

    let alid_len = r_outer - 12.0 * scale;
    let alid_w = 14.0 * scale;

    // Regla de latón con extremos en punta de flecha / trébol
    let mut alid_pb = tiny_skia::PathBuilder::new();
    alid_pb.move_to(-alid_len, 0.0);
    alid_pb.line_to(-alid_len + 25.0 * scale, -alid_w * 0.5);
    alid_pb.line_to(alid_len - 25.0 * scale, -alid_w * 0.5);
    alid_pb.line_to(alid_len, 0.0);
    alid_pb.line_to(alid_len - 25.0 * scale, alid_w * 0.5);
    alid_pb.line_to(-alid_len + 25.0 * scale, alid_w * 0.5);
    alid_pb.close();
    if let Some(ap) = alid_pb.finish() {
        canvas.fill_path(&ap, brass_gold);
        canvas.stroke_path_fine(&ap, brass_dark, 2.5 * scale);
        canvas.stroke_line_fine(-alid_len + 15.0 * scale, 0.0, alid_len - 15.0 * scale, 0.0, brass_light, 1.8 * scale);
    }

    // Pínulas con mirillas en los dos extremos
    for &side in &[-1.0_f32, 1.0_f32] {
        let px = side * (alid_len - 45.0 * scale);
        canvas.fill_rect(px - 6.0 * scale, -16.0 * scale, 12.0 * scale, 32.0 * scale, brass_dark);
        canvas.stroke_rect(px - 6.0 * scale, -16.0 * scale, 12.0 * scale, 32.0 * scale, brass_light, 1.5 * scale);
        // Orificio de mira
        canvas.fill_circle(px, 0.0, 3.2 * scale, Color::WHITE);
        // Rayo de luz celeste atravesando la pínula
        canvas.stroke_line_fine(px - 14.0 * scale, 0.0, px + 14.0 * scale, 0.0, cyan_bright, 1.8 * scale);
    }

    // Perno central / Tuerca en forma de cabeza de caballo (el "caballo" del astrolabio)
    canvas.fill_circle(0.0, 0.0, 16.0 * scale, brass_light);
    canvas.stroke_circle(0.0, 0.0, 16.0 * scale, brass_dark, 2.5 * scale);
    canvas.fill_circle(0.0, 0.0, 7.0 * scale, alchemical_red);

    canvas.restore();

    // ===== 8. CARTELA CARTOGRÁFICA Y SÍMBOLOS ALQUÍMICOS =====
    // Glifos alquímicos en los 4 costados
    let alch_glyphs = ["☉ SOL", "☽ LUNA", "☿ MERCURIUS", "♃ IUPITER"];
    let glyph_coords = [
        (-r_outer + 35.0 * scale, -r_outer + 50.0 * scale),
        (r_outer - 85.0 * scale, -r_outer + 50.0 * scale),
        (-r_outer + 35.0 * scale, r_outer - 40.0 * scale),
        (r_outer - 85.0 * scale, r_outer - 40.0 * scale),
    ];
    for (i, &(gx, gy)) in glyph_coords.iter().enumerate() {
        draw_vector_text(canvas, gx, gy, alch_glyphs[i], 10.0 * scale, cyan_bright, false);
    }

    // Inscripción de autenticidad de Melquíades en el fondo inferior
    draw_vector_text(canvas, 0.0, r_outer - 80.0 * scale, "SPHAERA ARMILLARIS • MELQUIADES", 11.5 * scale, brass_light, true);
    draw_vector_text(canvas, 0.0, r_outer - 64.0 * scale, "POLUS ARCTICUS • COELUM MACONDO", 9.0 * scale, cyan_line, true);

    canvas.restore();
}
