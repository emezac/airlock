use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::schematic::draw_vector_text;
use std::f32::consts::PI;

// =============================================================================
// PALETA CROMÁTICA NOBLE Y REALISMO MÁGICO (COLOMBIA / MACONDO / ÓSCAR CHÁVEZ)
// =============================================================================

pub struct MacondoRealistaPalette {
    pub paper_cream: Color,
    pub paper_grain: Color,
    pub ink_sepia: Color,
    pub ink_dark: Color,
    pub gold_high: Color,
    pub gold_base: Color,
    pub gold_deep: Color,
    pub brass_bell: Color,
    pub ruby_eye: Color,
    pub wood_spruce: Color,
    pub wood_cedar: Color,
    pub wood_ebony: Color,
    pub silver_key: Color,
    pub leaf_emerald: Color,
    pub leaf_deep: Color,
    pub butterfly_yellow: Color,
    pub butterfly_orange: Color,
    pub linen_white: Color,
    pub iron_chain: Color,
    pub iron_shadow: Color,
}

impl Default for MacondoRealistaPalette {
    fn default() -> Self {
        Self {
            paper_cream: Color::hex("#FAF5EA"),
            paper_grain: Color::hex("#EFE4D0"),
            ink_sepia: Color::hex("#382315"),
            ink_dark: Color::hex("#1F130B"),
            gold_high: Color::hex("#FFFBE0"),
            gold_base: Color::hex("#F6C432"),
            gold_deep: Color::hex("#A8760C"),
            brass_bell: Color::hex("#DDA823"),
            ruby_eye: Color::hex("#D9183B"),
            wood_spruce: Color::hex("#E2B371"),
            wood_cedar: Color::hex("#994921"),
            wood_ebony: Color::hex("#1B1919"),
            silver_key: Color::hex("#E8ECF2"),
            leaf_emerald: Color::hex("#2C7E3A"),
            leaf_deep: Color::hex("#14431C"),
            butterfly_yellow: Color::hex("#FFDF00"),
            butterfly_orange: Color::hex("#FFA000"),
            linen_white: Color::hex("#FCFDFD"),
            iron_chain: Color::hex("#6B7280"),
            iron_shadow: Color::hex("#374151"),
        }
    }
}

// =============================================================================
// 1. PLACA ARTÍSTICA DE GRABADO Y AGUAFUERTE PARA 16:9 (1920x1080)
// =============================================================================

pub fn draw_fine_art_plate(
    canvas: &mut Canvas,
    w: u32,
    h: u32,
    title: &str,
    subtitle: &str,
    chapter: &str,
) {
    let pal = MacondoRealistaPalette::default();
    let wf = w as f32;
    let hf = h as f32;

    // Fondo cálido de papel verjurado artesanal
    canvas.clear(pal.paper_cream);

    // Textura de grano sutil de papel
    for i in 0..60 {
        let x = (i as f32 * 32.0) % wf;
        canvas.stroke_line(x, 0.0, x + 18.0, hf, pal.paper_grain.with_alpha(0.18), 0.7);
    }

    // Marco exterior de aguafuerte clásico con filetes concéntricos
    let margin = 32.0;
    canvas.stroke_rect(margin, margin, wf - margin * 2.0, hf - margin * 2.0, pal.ink_sepia.with_alpha(0.85), 3.0);
    canvas.stroke_rect(margin + 6.0, margin + 6.0, wf - (margin + 6.0) * 2.0, hf - (margin + 6.0) * 2.0, pal.gold_deep.with_alpha(0.60), 1.2);
    canvas.stroke_rect(margin + 12.0, margin + 12.0, wf - (margin + 12.0) * 2.0, hf - (margin + 12.0) * 2.0, pal.ink_sepia.with_alpha(0.35), 0.8);

    // Cartuchos ornamentales en las 4 esquinas
    for &(cx, cy, sx, sy) in &[
        (margin + 18.0, margin + 18.0, 1.0_f32, 1.0_f32),
        (wf - margin - 18.0, margin + 18.0, -1.0, 1.0),
        (margin + 18.0, hf - margin - 18.0, 1.0, -1.0),
        (wf - margin - 18.0, hf - margin - 18.0, -1.0, -1.0),
    ] {
        canvas.save();
        canvas.translate(cx, cy);
        canvas.scale(sx, sy);
        let mut c_pb = tiny_skia::PathBuilder::new();
        c_pb.move_to(0.0, 32.0);
        c_pb.cubic_to(0.0, 10.0, 10.0, 0.0, 32.0, 0.0);
        c_pb.cubic_to(20.0, 8.0, 8.0, 20.0, 0.0, 32.0);
        if let Some(path) = c_pb.finish() {
            canvas.fill_path(&path, pal.gold_base.with_alpha(0.6));
            canvas.stroke_path_fine(&path, pal.ink_sepia, 1.2);
        }
        canvas.fill_circle(14.0, 14.0, 3.5, pal.gold_deep);
        canvas.restore();
    }

    // Cabecera superior tipográfica
    if !chapter.is_empty() {
        draw_vector_text(canvas, wf * 0.5, margin + 28.0, chapter, 13.0, pal.gold_deep, true);
    }
    if !title.is_empty() {
        draw_vector_text(canvas, wf * 0.5, margin + 56.0, title, 28.0, pal.ink_dark, true);
    }
    if !subtitle.is_empty() {
        draw_vector_text(canvas, wf * 0.5, margin + 84.0, subtitle, 12.5, pal.ink_sepia.with_alpha(0.85), true);
    }
}

// =============================================================================
// 2. TROMPETA DE ORO REALISTA (B-FLAT FANFARE TRUMPET CON MECÁNICA EXACTA)
// =============================================================================

pub fn draw_realistic_trumpet(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
    is_playing: bool,
) {
    let pal = MacondoRealistaPalette::default();

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    let vibrato = if is_playing {
        (tau * 36.0 * PI).sin() * 1.5 * scale
    } else {
        0.0
    };
    canvas.translate(0.0, vibrato);

    let l = 540.0 * scale;

    // A. Ondas acústicas y notas brotando de la campana
    if is_playing {
        let bell_mouth_x = l * 0.58;
        let bell_mouth_y = 0.0;

        for w in 0..5 {
            let phase = ((tau * 4.0 + (w as f32) * 0.22) % 1.0);
            let arc_r = 50.0 * scale + phase * 220.0 * scale;
            let alpha = (1.0 - phase) * 0.75;
            let mut arc_pb = tiny_skia::PathBuilder::new();
            arc_pb.move_to(bell_mouth_x + arc_r * 0.5, bell_mouth_y - arc_r * 0.7);
            arc_pb.cubic_to(
                bell_mouth_x + arc_r * 0.95,
                bell_mouth_y - arc_r * 0.35,
                bell_mouth_x + arc_r * 0.95,
                bell_mouth_y + arc_r * 0.35,
                bell_mouth_x + arc_r * 0.5,
                bell_mouth_y + arc_r * 0.7,
            );
            if let Some(path) = arc_pb.finish() {
                canvas.stroke_path_fine(&path, pal.gold_base.with_alpha(alpha), 3.0 * scale);
                canvas.stroke_path_fine(&path, pal.gold_high.with_alpha(alpha * 0.8), 1.2 * scale);
            }
        }

        for n in 0..4 {
            let n_phase = ((tau * 3.5 + (n as f32) * 0.25) % 1.0);
            let nx = bell_mouth_x + 90.0 * scale + n_phase * 260.0 * scale;
            let ny = bell_mouth_y - 80.0 * scale + ((n as f32 * 1.8 + tau * 6.0).sin()) * 45.0 * scale;
            let n_alpha = (1.0 - n_phase).min(n_phase * 4.0).clamp(0.0, 1.0);
            draw_musical_note(canvas, nx, ny, 16.0 * scale, pal.gold_base.with_alpha(n_alpha));
        }
    }

    // B. Boquilla torneada (Mouthpiece)
    let mp_x = -l * 0.54;
    let mp_y = -18.0 * scale;

    canvas.fill_circle(mp_x - 14.0 * scale, mp_y, 11.0 * scale, pal.silver_key);
    canvas.stroke_circle(mp_x - 14.0 * scale, mp_y, 11.0 * scale, pal.ink_sepia, 1.5 * scale);
    canvas.fill_circle(mp_x - 14.0 * scale, mp_y, 8.0 * scale, pal.gold_high);

    let mut mp_pb = tiny_skia::PathBuilder::new();
    mp_pb.move_to(mp_x - 14.0 * scale, mp_y - 9.0 * scale);
    mp_pb.cubic_to(mp_x - 4.0 * scale, mp_y - 7.0 * scale, mp_x, mp_y - 4.0 * scale, mp_x + 22.0 * scale, mp_y - 4.0 * scale);
    mp_pb.line_to(mp_x + 22.0 * scale, mp_y + 4.0 * scale);
    mp_pb.cubic_to(mp_x, mp_y + 4.0 * scale, mp_x - 4.0 * scale, mp_y + 7.0 * scale, mp_x - 14.0 * scale, mp_y + 9.0 * scale);
    mp_pb.close();
    if let Some(path) = mp_pb.finish() {
        canvas.fill_path(&path, pal.silver_key);
        canvas.stroke_path_fine(&path, pal.ink_dark, 1.2 * scale);
    }
    canvas.stroke_line(mp_x - 8.0 * scale, mp_y - 2.0 * scale, mp_x + 18.0 * scale, mp_y - 2.0 * scale, Color::WHITE, 2.0 * scale);

    // C. Leadpipe (Tubo superior)
    let lead_x1 = mp_x + 22.0 * scale;
    let lead_x2 = l * 0.18;
    let lead_y = mp_y;
    let tube_r = 6.0 * scale;

    canvas.fill_rect(lead_x1, lead_y - 7.0 * scale, 12.0 * scale, 14.0 * scale, pal.gold_deep);
    canvas.stroke_rect(lead_x1, lead_y - 7.0 * scale, 12.0 * scale, 14.0 * scale, pal.ink_dark, 1.2 * scale);

    canvas.fill_rect(lead_x1 + 12.0 * scale, lead_y - tube_r, (lead_x2 - lead_x1 - 12.0 * scale), tube_r * 2.0, pal.gold_base);
    canvas.stroke_line(lead_x1 + 12.0 * scale, lead_y - tube_r, lead_x2, lead_y - tube_r, pal.ink_dark, 1.4 * scale);
    canvas.stroke_line(lead_x1 + 12.0 * scale, lead_y + tube_r, lead_x2, lead_y + tube_r, pal.gold_deep, 1.4 * scale);
    canvas.stroke_line(lead_x1 + 12.0 * scale, lead_y - 2.0 * scale, lead_x2, lead_y - 2.0 * scale, pal.gold_high, 2.2 * scale);
    canvas.stroke_line(lead_x1 + 12.0 * scale, lead_y - 2.5 * scale, lead_x2, lead_y - 2.5 * scale, Color::WHITE, 0.8 * scale);

    // D. Bomba de afinación con curva 180° y salivadera
    let slide_x = lead_x2;
    let lower_tube_y = 18.0 * scale;
    let mut slide_pb = tiny_skia::PathBuilder::new();
    slide_pb.move_to(slide_x, lead_y - tube_r);
    slide_pb.cubic_to(
        slide_x + 55.0 * scale,
        lead_y - tube_r,
        slide_x + 55.0 * scale,
        lower_tube_y + tube_r,
        slide_x,
        lower_tube_y + tube_r,
    );
    slide_pb.line_to(slide_x, lower_tube_y - tube_r);
    slide_pb.cubic_to(
        slide_x + 38.0 * scale,
        lower_tube_y - tube_r,
        slide_x + 38.0 * scale,
        lead_y + tube_r,
        slide_x,
        lead_y + tube_r,
    );
    slide_pb.close();
    if let Some(path) = slide_pb.finish() {
        canvas.fill_path(&path, pal.gold_base);
        canvas.stroke_path_fine(&path, pal.ink_dark, 1.5 * scale);
    }
    let wk_x = slide_x + 35.0 * scale;
    let wk_y = lower_tube_y + tube_r;
    canvas.fill_circle(wk_x, wk_y, 3.5 * scale, pal.silver_key);
    canvas.stroke_line(wk_x - 6.0 * scale, wk_y - 2.0 * scale, wk_x + 12.0 * scale, wk_y + 6.0 * scale, pal.silver_key, 2.0 * scale);
    canvas.fill_circle(wk_x + 12.0 * scale, wk_y + 6.0 * scale, 2.5 * scale, Color::hex("#A8422B"));

    let lower_ret_x = -l * 0.15;
    canvas.fill_rect(lower_ret_x, lower_tube_y - tube_r, (slide_x - lower_ret_x), tube_r * 2.0, pal.gold_base);
    canvas.stroke_line(lower_ret_x, lower_tube_y - tube_r, slide_x, lower_tube_y - tube_r, pal.ink_dark, 1.4 * scale);
    canvas.stroke_line(lower_ret_x, lower_tube_y + tube_r, slide_x, lower_tube_y + tube_r, pal.gold_deep, 1.4 * scale);
    canvas.stroke_line(lower_ret_x, lower_tube_y - 2.0 * scale, slide_x, lower_tube_y - 2.0 * scale, pal.gold_high, 2.0 * scale);

    for &bx in &[-l * 0.32, -l * 0.05, l * 0.12] {
        canvas.fill_rect(bx - 3.0 * scale, lead_y + tube_r, 6.0 * scale, (lower_tube_y - lead_y - tube_r * 2.0), pal.gold_deep);
        canvas.stroke_rect(bx - 3.0 * scale, lead_y + tube_r, 6.0 * scale, (lower_tube_y - lead_y - tube_r * 2.0), pal.ink_dark, 1.0 * scale);
    }

    // E. Bloque de 3 pistones verticales
    let valve_start_x = -l * 0.08;
    let valve_spacing = 30.0 * scale;
    let valve_h = 76.0 * scale;
    let valve_top_y = -38.0 * scale;
    let valve_w = 16.0 * scale;

    for v in 0..3 {
        let vx = valve_start_x + (v as f32) * valve_spacing;

        let press_offset = if is_playing {
            let cycle = (tau * 12.0 + (v as f32) * 1.3).sin();
            if cycle > 0.2 { 10.0 * scale } else { 0.0 }
        } else {
            0.0
        };

        canvas.fill_rect(vx - valve_w * 0.55, valve_top_y + valve_h, valve_w * 1.1, 10.0 * scale, pal.gold_deep);
        canvas.stroke_rect(vx - valve_w * 0.55, valve_top_y + valve_h, valve_w * 1.1, 10.0 * scale, pal.ink_dark, 1.2 * scale);

        canvas.fill_rect(vx - valve_w * 0.5, valve_top_y, valve_w, valve_h, pal.gold_base);
        canvas.stroke_line(vx - valve_w * 0.5, valve_top_y, vx - valve_w * 0.5, valve_top_y + valve_h, pal.ink_dark, 1.4 * scale);
        canvas.stroke_line(vx + valve_w * 0.5, valve_top_y, vx + valve_w * 0.5, valve_top_y + valve_h, pal.ink_dark, 1.4 * scale);
        canvas.stroke_line(vx - valve_w * 0.15, valve_top_y, vx - valve_w * 0.15, valve_top_y + valve_h, pal.gold_high, 2.5 * scale);

        canvas.fill_rect(vx - valve_w * 0.55, valve_top_y - 8.0 * scale, valve_w * 1.1, 8.0 * scale, pal.gold_deep);
        canvas.stroke_rect(vx - valve_w * 0.55, valve_top_y - 8.0 * scale, valve_w * 1.1, 8.0 * scale, pal.ink_dark, 1.2 * scale);

        let stem_top = valve_top_y - 28.0 * scale + press_offset;
        canvas.fill_rect(vx - 2.5 * scale, stem_top, 5.0 * scale, 20.0 * scale, pal.silver_key);
        canvas.stroke_rect(vx - 2.5 * scale, stem_top, 5.0 * scale, 20.0 * scale, pal.ink_dark, 1.0 * scale);
        canvas.fill_rect(vx - 6.0 * scale, valve_top_y - 11.0 * scale, 12.0 * scale, 3.0 * scale, pal.ink_dark);

        canvas.fill_circle(vx, stem_top, 8.0 * scale, pal.gold_deep);
        canvas.stroke_circle(vx, stem_top, 8.0 * scale, pal.ink_dark, 1.2 * scale);
        canvas.fill_circle(vx, stem_top, 6.0 * scale, pal.linen_white);
        canvas.fill_circle(vx - 2.0 * scale, stem_top - 2.0 * scale, 2.5 * scale, Color::WHITE);
    }

    // F. Bombas de los pistones
    let v2_x = valve_start_x + valve_spacing;
    canvas.fill_rect(v2_x - 6.0 * scale, -2.0 * scale, 12.0 * scale, 16.0 * scale, pal.gold_base);
    canvas.stroke_rect(v2_x - 6.0 * scale, -2.0 * scale, 12.0 * scale, 16.0 * scale, pal.ink_dark, 1.2 * scale);

    let v1_x = valve_start_x;
    let mut v1_pb = tiny_skia::PathBuilder::new();
    v1_pb.move_to(v1_x, -5.0 * scale);
    v1_pb.line_to(v1_x - 36.0 * scale, -5.0 * scale);
    v1_pb.cubic_to(v1_x - 48.0 * scale, -5.0 * scale, v1_x - 48.0 * scale, 12.0 * scale, v1_x - 36.0 * scale, 12.0 * scale);
    v1_pb.line_to(v1_x, 12.0 * scale);
    if let Some(path) = v1_pb.finish() {
        canvas.stroke_path_fine(&path, pal.gold_base, 8.0 * scale);
        canvas.stroke_path_fine(&path, pal.ink_dark, 1.2 * scale);
    }
    canvas.stroke_line(v1_x - 30.0 * scale, -8.0 * scale, v1_x - 30.0 * scale, -1.0 * scale, pal.silver_key, 2.5 * scale);

    let v3_x = valve_start_x + valve_spacing * 2.0;
    let mut v3_pb = tiny_skia::PathBuilder::new();
    v3_pb.move_to(v3_x, 4.0 * scale);
    v3_pb.line_to(v3_x + 55.0 * scale, 4.0 * scale);
    v3_pb.cubic_to(v3_x + 72.0 * scale, 4.0 * scale, v3_x + 72.0 * scale, 24.0 * scale, v3_x + 55.0 * scale, 24.0 * scale);
    v3_pb.line_to(v3_x, 24.0 * scale);
    if let Some(path) = v3_pb.finish() {
        canvas.stroke_path_fine(&path, pal.gold_base, 8.0 * scale);
        canvas.stroke_path_fine(&path, pal.ink_dark, 1.2 * scale);
    }
    canvas.stroke_circle(v3_x + 48.0 * scale, 28.0 * scale, 6.5 * scale, pal.silver_key, 2.0 * scale);

    // G. Campana acústica hiperbólica
    let bell_bow_x = -l * 0.42;
    let mut bow_pb = tiny_skia::PathBuilder::new();
    bow_pb.move_to(bell_bow_x, lower_tube_y - tube_r);
    bow_pb.cubic_to(
        bell_bow_x - 45.0 * scale,
        lower_tube_y - tube_r,
        bell_bow_x - 45.0 * scale,
        -tube_r,
        bell_bow_x,
        -tube_r,
    );
    bow_pb.line_to(bell_bow_x, tube_r);
    bow_pb.cubic_to(
        bell_bow_x - 30.0 * scale,
        tube_r,
        bell_bow_x - 30.0 * scale,
        lower_tube_y + tube_r,
        bell_bow_x,
        lower_tube_y + tube_r,
    );
    bow_pb.close();
    if let Some(path) = bow_pb.finish() {
        canvas.fill_path(&path, pal.gold_base);
        canvas.stroke_path_fine(&path, pal.ink_dark, 1.5 * scale);
    }

    let flare_start_x = l * 0.05;
    let flare_end_x = l * 0.56;
    let bell_lip_y = 65.0 * scale;

    let mut bell_pb = tiny_skia::PathBuilder::new();
    bell_pb.move_to(bell_bow_x, -tube_r);
    bell_pb.line_to(flare_start_x, -tube_r);
    bell_pb.cubic_to(
        flare_start_x + 90.0 * scale,
        -tube_r - 2.0 * scale,
        flare_end_x - 60.0 * scale,
        -bell_lip_y * 0.45,
        flare_end_x,
        -bell_lip_y,
    );
    bell_pb.line_to(flare_end_x, bell_lip_y);
    bell_pb.cubic_to(
        flare_end_x - 60.0 * scale,
        bell_lip_y * 0.45,
        flare_start_x + 90.0 * scale,
        tube_r + 2.0 * scale,
        flare_start_x,
        tube_r,
    );
    bell_pb.line_to(bell_bow_x, tube_r);
    bell_pb.close();

    if let Some(path) = bell_pb.finish() {
        canvas.fill_path(&path, pal.brass_bell);
        canvas.stroke_path_fine(&path, pal.ink_dark, 1.6 * scale);

        let mut shine_pb = tiny_skia::PathBuilder::new();
        shine_pb.move_to(flare_start_x, -tube_r * 0.3);
        shine_pb.cubic_to(
            flare_start_x + 90.0 * scale,
            -tube_r * 0.4,
            flare_end_x - 60.0 * scale,
            -bell_lip_y * 0.35,
            flare_end_x - 8.0 * scale,
            -bell_lip_y * 0.85,
        );
        if let Some(spath) = shine_pb.finish() {
            canvas.stroke_path_fine(&spath, pal.gold_high, 4.5 * scale);
            canvas.stroke_path_fine(&spath, Color::WHITE, 1.5 * scale);
        }
    }

    canvas.fill_circle(flare_end_x, 0.0, bell_lip_y * 0.95, pal.gold_deep);
    canvas.fill_circle(flare_end_x, 0.0, bell_lip_y * 0.70, pal.ink_sepia);
    canvas.fill_circle(flare_end_x, 0.0, bell_lip_y * 0.40, pal.ink_dark);

    let mut rim_pb = tiny_skia::PathBuilder::new();
    rim_pb.move_to(flare_end_x, -bell_lip_y);
    rim_pb.cubic_to(
        flare_end_x + 12.0 * scale,
        -bell_lip_y * 0.5,
        flare_end_x + 12.0 * scale,
        bell_lip_y * 0.5,
        flare_end_x,
        bell_lip_y,
    );
    rim_pb.cubic_to(
        flare_end_x - 6.0 * scale,
        bell_lip_y * 0.5,
        flare_end_x - 6.0 * scale,
        -bell_lip_y * 0.5,
        flare_end_x,
        -bell_lip_y,
    );
    rim_pb.close();
    if let Some(rpath) = rim_pb.finish() {
        canvas.fill_path(&rpath, pal.gold_high);
        canvas.stroke_path_fine(&rpath, pal.ink_dark, 1.4 * scale);
    }

    let eng_x = flare_end_x - 95.0 * scale;
    canvas.stroke_circle(eng_x, 0.0, 18.0 * scale, pal.gold_deep.with_alpha(0.6), 1.0 * scale);
    draw_vector_text(canvas, eng_x, 3.0 * scale, "MACONDO", 7.5 * scale, pal.ink_sepia.with_alpha(0.75), true);

    canvas.restore();
}

fn draw_musical_note(canvas: &mut Canvas, x: f32, y: f32, size: f32, color: Color) {
    canvas.save();
    canvas.translate(x, y);
    canvas.rotate(-0.35);
    canvas.fill_circle(0.0, 0.0, size * 0.5, color);
    canvas.restore();
    canvas.stroke_line(x + size * 0.45, y, x + size * 0.45, y - size * 1.6, color, size * 0.18);
    let mut c_pb = tiny_skia::PathBuilder::new();
    c_pb.move_to(x + size * 0.45, y - size * 1.6);
    c_pb.cubic_to(x + size * 0.9, y - size * 1.2, x + size * 0.9, y - size * 0.8, x + size * 0.45, y - size * 0.5);
    if let Some(path) = c_pb.finish() {
        canvas.stroke_path_fine(&path, color, size * 0.16);
    }
}

// =============================================================================
// 3. EL CUATRO LLANERO / COLOMBIANO REALISTA
// =============================================================================

pub fn draw_realistic_cuatro(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    let l = 480.0 * scale;
    let body_cx = -l * 0.12;
    let body_w = 210.0 * scale;
    let body_h = 320.0 * scale;

    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(body_cx, -body_h * 0.5);
    body_pb.cubic_to(
        body_cx + body_w * 0.50,
        -body_h * 0.50,
        body_cx + body_w * 0.50,
        -body_h * 0.16,
        body_cx + body_w * 0.35,
        -body_h * 0.08,
    );
    body_pb.cubic_to(
        body_cx + body_w * 0.20,
        0.0,
        body_cx + body_w * 0.20,
        body_h * 0.06,
        body_cx + body_w * 0.45,
        body_h * 0.16,
    );
    body_pb.cubic_to(
        body_cx + body_w * 0.58,
        body_h * 0.26,
        body_cx + body_w * 0.58,
        body_h * 0.50,
        body_cx,
        body_h * 0.50,
    );
    body_pb.cubic_to(
        body_cx - body_w * 0.58,
        body_h * 0.50,
        body_cx - body_w * 0.58,
        body_h * 0.26,
        body_cx - body_w * 0.45,
        body_h * 0.16,
    );
    body_pb.cubic_to(
        body_cx - body_w * 0.20,
        body_h * 0.06,
        body_cx - body_w * 0.20,
        0.0,
        body_cx - body_w * 0.35,
        -body_h * 0.08,
    );
    body_pb.cubic_to(
        body_cx - body_w * 0.50,
        -body_h * 0.16,
        body_cx - body_w * 0.50,
        -body_h * 0.50,
        body_cx,
        -body_h * 0.50,
    );
    body_pb.close();

    if let Some(path) = body_pb.finish() {
        canvas.fill_path(&path, pal.wood_spruce);
        canvas.stroke_path_fine(&path, pal.ink_dark, 2.5 * scale);
        canvas.stroke_path_fine(&path, pal.wood_cedar, 4.5 * scale);

        for g in -10..=10 {
            let gx = body_cx + (g as f32) * 9.0 * scale;
            canvas.stroke_line(gx, -body_h * 0.45, gx, body_h * 0.45, pal.wood_cedar.with_alpha(0.18), 0.8 * scale);
        }
    }

    let mut gp_pb = tiny_skia::PathBuilder::new();
    gp_pb.move_to(body_cx + 15.0 * scale, -body_h * 0.30);
    gp_pb.line_to(body_cx + body_w * 0.38, -body_h * 0.25);
    gp_pb.line_to(body_cx + body_w * 0.32, -body_h * 0.05);
    gp_pb.line_to(body_cx + 15.0 * scale, -body_h * 0.05);
    gp_pb.close();
    if let Some(gp_path) = gp_pb.finish() {
        canvas.fill_path(&gp_path, pal.wood_cedar);
        canvas.stroke_path_fine(&gp_path, pal.ink_dark, 1.0 * scale);
    }

    let hole_y = -body_h * 0.12;
    let hole_r = 38.0 * scale;

    canvas.fill_circle(body_cx, hole_y, hole_r * 1.55, pal.wood_cedar);
    canvas.stroke_circle(body_cx, hole_y, hole_r * 1.55, pal.ink_dark, 1.4 * scale);
    canvas.fill_circle(body_cx, hole_y, hole_r * 1.35, pal.gold_base);
    canvas.stroke_circle(body_cx, hole_y, hole_r * 1.35, pal.ink_sepia, 1.0 * scale);

    let petals = 24;
    for p in 0..petals {
        let a = (p as f32 / petals as f32) * PI * 2.0;
        let rx1 = body_cx + a.cos() * (hole_r * 1.15);
        let ry1 = hole_y + a.sin() * (hole_r * 1.15);
        let rx2 = body_cx + a.cos() * (hole_r * 1.32);
        let ry2 = hole_y + a.sin() * (hole_r * 1.32);
        canvas.stroke_line(rx1, ry1, rx2, ry2, pal.ink_dark, 1.5 * scale);
    }

    canvas.fill_circle(body_cx, hole_y, hole_r, pal.ink_dark);
    canvas.stroke_circle(body_cx, hole_y, hole_r, pal.ink_sepia, 2.0 * scale);

    let bridge_y = body_h * 0.24;
    let bridge_w = 90.0 * scale;
    let bridge_h = 24.0 * scale;

    canvas.fill_rect(body_cx - bridge_w * 0.5, bridge_y - bridge_h * 0.5, bridge_w, bridge_h, pal.wood_ebony);
    canvas.stroke_rect(body_cx - bridge_w * 0.5, bridge_y - bridge_h * 0.5, bridge_w, bridge_h, pal.ink_dark, 1.4 * scale);
    canvas.fill_rect(body_cx - bridge_w * 0.35, bridge_y - 2.0 * scale, bridge_w * 0.70, 4.0 * scale, pal.linen_white);

    let neck_w = 34.0 * scale;
    let neck_len = 220.0 * scale;
    let neck_top_y = -body_h * 0.5 - neck_len;

    canvas.fill_rect(body_cx - neck_w * 0.5, neck_top_y, neck_w, neck_len + 40.0 * scale, pal.wood_cedar);
    canvas.fill_rect(body_cx - neck_w * 0.45, neck_top_y, neck_w * 0.90, neck_len + 40.0 * scale, pal.wood_ebony);
    canvas.stroke_line(body_cx - neck_w * 0.45, neck_top_y, body_cx - neck_w * 0.45, -body_h * 0.2, pal.ink_dark, 1.5 * scale);
    canvas.stroke_line(body_cx + neck_w * 0.45, neck_top_y, body_cx + neck_w * 0.45, -body_h * 0.2, pal.ink_dark, 1.5 * scale);

    for tr in 0..14 {
        let t = 1.0 - (1.0 / (2.0_f32.powf((tr as f32 + 1.0) / 12.0)));
        let fy = neck_top_y + t * (neck_len + 35.0 * scale) * 1.35;
        if fy < -body_h * 0.18 {
            canvas.stroke_line(body_cx - neck_w * 0.42, fy, body_cx + neck_w * 0.42, fy, pal.silver_key, 1.8 * scale);
            if tr == 4 || tr == 6 || tr == 9 {
                canvas.fill_circle(body_cx, fy - 8.0 * scale, 2.5 * scale, pal.linen_white);
            }
        }
    }

    canvas.fill_rect(body_cx - neck_w * 0.48, neck_top_y - 4.0 * scale, neck_w * 0.96, 6.0 * scale, pal.linen_white);

    let head_h = 95.0 * scale;
    let head_y = neck_top_y - head_h;

    let mut head_pb = tiny_skia::PathBuilder::new();
    head_pb.move_to(body_cx - neck_w * 0.48, neck_top_y - 4.0 * scale);
    head_pb.line_to(body_cx - neck_w * 0.65, head_y + 25.0 * scale);
    head_pb.cubic_to(body_cx - neck_w * 0.4, head_y - 8.0 * scale, body_cx + neck_w * 0.4, head_y - 8.0 * scale, body_cx + neck_w * 0.65, head_y + 25.0 * scale);
    head_pb.line_to(body_cx + neck_w * 0.48, neck_top_y - 4.0 * scale);
    head_pb.close();
    if let Some(h_path) = head_pb.finish() {
        canvas.fill_path(&h_path, pal.wood_cedar);
        canvas.stroke_path_fine(&h_path, pal.ink_dark, 1.6 * scale);
    }

    let peg_y = [head_y + 28.0 * scale, head_y + 60.0 * scale];
    for &py in &peg_y {
        canvas.stroke_line(body_cx - neck_w * 0.6, py, body_cx - neck_w * 0.95, py, pal.silver_key, 3.0 * scale);
        canvas.fill_circle(body_cx - neck_w * 1.05, py, 6.0 * scale, pal.linen_white);
        canvas.stroke_circle(body_cx - neck_w * 1.05, py, 6.0 * scale, pal.ink_dark, 1.0 * scale);

        canvas.stroke_line(body_cx + neck_w * 0.6, py, body_cx + neck_w * 0.95, py, pal.silver_key, 3.0 * scale);
        canvas.fill_circle(body_cx + neck_w * 1.05, py, 6.0 * scale, pal.linen_white);
        canvas.stroke_circle(body_cx + neck_w * 1.05, py, 6.0 * scale, pal.ink_dark, 1.0 * scale);
    }

    for s in 0..4 {
        let offset = ((s as f32) - 1.5) * 7.5 * scale;
        let str_x = body_cx + offset;
        let vib = (tau * 40.0 * PI + s as f32 * 1.5).sin() * 1.5 * scale;

        canvas.stroke_line(str_x, neck_top_y, str_x + vib, bridge_y, pal.gold_high, 1.5 * scale);
        canvas.stroke_line(str_x, neck_top_y, str_x + vib, bridge_y, Color::WHITE.with_alpha(0.85), 0.7 * scale);
    }

    let strum_y = hole_y + (tau * 14.0 * PI).sin() * 18.0 * scale;
    let hand_x = body_cx + 42.0 * scale;

    canvas.fill_circle(hand_x + 35.0 * scale, strum_y + 15.0 * scale, 18.0 * scale, Color::hex("#E8B896"));
    canvas.stroke_circle(hand_x + 35.0 * scale, strum_y + 15.0 * scale, 18.0 * scale, pal.ink_sepia, 1.2 * scale);

    for d in 0..4 {
        let dy = strum_y - 12.0 * scale + (d as f32) * 8.0 * scale;
        canvas.stroke_line(hand_x + 30.0 * scale, dy + 5.0 * scale, hand_x - 5.0 * scale, dy, Color::hex("#DBA47E"), 4.5 * scale);
        canvas.stroke_line(hand_x + 30.0 * scale, dy + 5.0 * scale, hand_x - 5.0 * scale, dy, pal.ink_sepia, 1.0 * scale);
    }

    canvas.restore();
}

// =============================================================================
// 4. EL OBOE DE MELQUÍADES REALISTA (CONSERVATOIRE KEY SYSTEM & CAÑA DOBLE)
// =============================================================================

pub fn draw_realistic_oboe(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    let l = 620.0 * scale;
    let top_y = -l * 0.48;
    let bottom_y = l * 0.48;

    let reed_top_y = top_y - 65.0 * scale;

    canvas.fill_rect(-4.5 * scale, top_y - 18.0 * scale, 9.0 * scale, 18.0 * scale, Color::hex("#D2AC70"));
    canvas.stroke_rect(-4.5 * scale, top_y - 18.0 * scale, 9.0 * scale, 18.0 * scale, pal.ink_dark, 1.0 * scale);

    canvas.fill_rect(-3.0 * scale, top_y - 32.0 * scale, 6.0 * scale, 14.0 * scale, pal.gold_base);

    canvas.fill_rect(-4.0 * scale, top_y - 48.0 * scale, 8.0 * scale, 16.0 * scale, Color::hex("#A81C26"));
    for th in 0..6 {
        let ty = top_y - 48.0 * scale + (th as f32) * 2.5 * scale;
        canvas.stroke_line(-4.0 * scale, ty, 4.0 * scale, ty, Color::hex("#FFD700"), 0.8 * scale);
    }

    let mut cane_pb = tiny_skia::PathBuilder::new();
    cane_pb.move_to(-3.5 * scale, top_y - 48.0 * scale);
    cane_pb.cubic_to(-4.5 * scale, reed_top_y + 6.0 * scale, -5.5 * scale, reed_top_y, 0.0, reed_top_y);
    cane_pb.cubic_to(5.5 * scale, reed_top_y, 4.5 * scale, reed_top_y + 6.0 * scale, 3.5 * scale, top_y - 48.0 * scale);
    cane_pb.close();
    if let Some(c_path) = cane_pb.finish() {
        canvas.fill_path(&c_path, Color::hex("#E8C988"));
        canvas.stroke_path_fine(&c_path, pal.ink_sepia, 1.0 * scale);
    }
    let reed_vib = (tau * 48.0 * PI).sin() * 1.5 * scale;
    canvas.stroke_line(-5.0 * scale, reed_top_y, 5.0 * scale, reed_top_y + reed_vib, pal.gold_high, 1.2 * scale);

    let bell_w = 42.0 * scale;
    let top_w = 14.0 * scale;

    let mut oboe_pb = tiny_skia::PathBuilder::new();
    oboe_pb.move_to(-top_w * 0.5, top_y);
    oboe_pb.line_to(-top_w * 0.75, bottom_y - 70.0 * scale);
    oboe_pb.cubic_to(
        -top_w * 0.85,
        bottom_y - 30.0 * scale,
        -bell_w * 0.5,
        bottom_y - 10.0 * scale,
        -bell_w * 0.5,
        bottom_y,
    );
    oboe_pb.line_to(bell_w * 0.5, bottom_y);
    oboe_pb.cubic_to(
        bell_w * 0.5,
        bottom_y - 10.0 * scale,
        top_w * 0.85,
        bottom_y - 30.0 * scale,
        top_w * 0.75,
        bottom_y - 70.0 * scale,
    );
    oboe_pb.line_to(top_w * 0.5, top_y);
    oboe_pb.close();

    if let Some(path) = oboe_pb.finish() {
        canvas.fill_path(&path, pal.wood_ebony);
        canvas.stroke_path_fine(&path, pal.ink_dark, 1.8 * scale);
        canvas.stroke_line(-2.0 * scale, top_y, -3.0 * scale, bottom_y - 70.0 * scale, Color::hex("#423B38"), 2.5 * scale);
    }

    for &ring_y in &[top_y, top_y + 190.0 * scale, bottom_y - 70.0 * scale, bottom_y] {
        let rw = if ring_y >= bottom_y - 75.0 * scale { bell_w * 0.9 } else { top_w * 1.3 };
        canvas.fill_rect(-rw * 0.5, ring_y - 3.0 * scale, rw, 6.0 * scale, pal.silver_key);
        canvas.stroke_rect(-rw * 0.5, ring_y - 3.0 * scale, rw, 6.0 * scale, pal.ink_dark, 1.0 * scale);
    }

    canvas.stroke_line(-top_w * 0.35, top_y + 20.0 * scale, -top_w * 0.50, bottom_y - 80.0 * scale, pal.silver_key, 2.0 * scale);
    canvas.stroke_line(top_w * 0.35, top_y + 30.0 * scale, top_w * 0.50, bottom_y - 80.0 * scale, pal.silver_key, 2.0 * scale);

    let key_y = [
        top_y + 50.0 * scale, top_y + 85.0 * scale, top_y + 120.0 * scale,
        top_y + 155.0 * scale, top_y + 230.0 * scale, top_y + 265.0 * scale,
        top_y + 300.0 * scale, top_y + 340.0 * scale, top_y + 380.0 * scale,
    ];

    for (k, &ky) in key_y.iter().enumerate() {
        let kx = if k % 2 == 0 { -4.0 * scale } else { 4.0 * scale };
        canvas.stroke_line(0.0, ky, kx, ky, pal.silver_key, 2.2 * scale);
        canvas.fill_circle(kx, ky, 6.5 * scale, pal.silver_key);
        canvas.stroke_circle(kx, ky, 6.5 * scale, pal.ink_dark, 1.2 * scale);
        canvas.fill_circle(kx - 1.5 * scale, ky - 1.5 * scale, 2.0 * scale, Color::WHITE);
    }

    let smoke_y = bottom_y + 25.0 * scale;
    for s in 0..4 {
        let sw = (tau * 10.0 + (s as f32) * 1.5).sin() * 16.0 * scale;
        canvas.stroke_circle(sw, smoke_y + (s as f32) * 22.0 * scale, 14.0 * scale + (s as f32) * 8.0 * scale, pal.gold_base.with_alpha(0.25), 1.2 * scale);
    }

    canvas.restore();
}

// =============================================================================
// 5. EL VIOLÍN DE REMEDIOS REALISTA (CON ARCO EN MOVIMIENTO Y F-HOLES)
// =============================================================================

pub fn draw_realistic_violin(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    let body_h = 340.0 * scale;
    let body_w = 210.0 * scale;

    let mut v_pb = tiny_skia::PathBuilder::new();
    v_pb.move_to(0.0, -body_h * 0.5);
    v_pb.cubic_to(body_w * 0.46, -body_h * 0.50, body_w * 0.46, -body_h * 0.22, body_w * 0.38, -body_h * 0.16);
    v_pb.line_to(body_w * 0.42, -body_h * 0.14);
    v_pb.cubic_to(body_w * 0.22, -body_h * 0.05, body_w * 0.22, body_h * 0.05, body_w * 0.44, body_h * 0.14);
    v_pb.line_to(body_w * 0.39, body_h * 0.16);
    v_pb.cubic_to(body_w * 0.52, body_h * 0.24, body_w * 0.52, body_h * 0.50, 0.0, body_h * 0.50);
    v_pb.cubic_to(-body_w * 0.52, body_h * 0.50, -body_w * 0.52, body_h * 0.24, -body_w * 0.39, body_h * 0.16);
    v_pb.line_to(-body_w * 0.44, body_h * 0.14);
    v_pb.cubic_to(-body_w * 0.22, body_h * 0.05, -body_w * 0.22, -body_h * 0.05, -body_w * 0.42, -body_h * 0.14);
    v_pb.line_to(-body_w * 0.38, -body_h * 0.16);
    v_pb.cubic_to(-body_w * 0.46, -body_h * 0.22, -body_w * 0.46, -body_h * 0.50, 0.0, -body_h * 0.50);
    v_pb.close();

    if let Some(path) = v_pb.finish() {
        canvas.fill_path(&path, Color::hex("#D98F3A"));
        canvas.stroke_path_fine(&path, pal.ink_dark, 2.2 * scale);
        canvas.stroke_path_fine(&path, pal.wood_ebony, 4.0 * scale);
        canvas.stroke_path_fine(&path, pal.gold_base, 1.2 * scale);
    }

    for &side in &[-1.0_f32, 1.0_f32] {
        let fx = side * 42.0 * scale;
        let mut f_pb = tiny_skia::PathBuilder::new();
        f_pb.move_to(fx, -32.0 * scale);
        f_pb.cubic_to(fx - side * 15.0 * scale, -10.0 * scale, fx + side * 12.0 * scale, 15.0 * scale, fx, 36.0 * scale);
        if let Some(fpath) = f_pb.finish() {
            canvas.stroke_path_fine(&fpath, pal.ink_dark, 4.5 * scale);
            canvas.fill_circle(fx, -32.0 * scale, 4.0 * scale, pal.ink_dark);
            canvas.fill_circle(fx, 36.0 * scale, 4.5 * scale, pal.ink_dark);
        }
    }

    let bridge_y = 6.0 * scale;
    canvas.fill_rect(-24.0 * scale, bridge_y - 2.0 * scale, 48.0 * scale, 5.0 * scale, pal.linen_white);
    canvas.stroke_rect(-24.0 * scale, bridge_y - 2.0 * scale, 48.0 * scale, 5.0 * scale, pal.ink_dark, 1.0 * scale);

    let mut tp_pb = tiny_skia::PathBuilder::new();
    tp_pb.move_to(-14.0 * scale, bridge_y + 35.0 * scale);
    tp_pb.line_to(14.0 * scale, bridge_y + 35.0 * scale);
    tp_pb.line_to(7.0 * scale, body_h * 0.46);
    tp_pb.line_to(-7.0 * scale, body_h * 0.46);
    tp_pb.close();
    if let Some(tp_path) = tp_pb.finish() {
        canvas.fill_path(&tp_path, pal.wood_ebony);
        canvas.stroke_path_fine(&tp_path, pal.ink_dark, 1.2 * scale);
    }
    for m in 0..4 {
        let mx = -9.0 * scale + (m as f32) * 6.0 * scale;
        canvas.fill_circle(mx, bridge_y + 36.0 * scale, 1.8 * scale, pal.gold_base);
    }

    canvas.fill_circle(-body_w * 0.24, body_h * 0.38, 26.0 * scale, pal.wood_ebony);
    canvas.stroke_circle(-body_w * 0.24, body_h * 0.38, 26.0 * scale, pal.ink_dark, 1.4 * scale);

    let neck_top_y = -body_h * 0.5 - 180.0 * scale;
    canvas.fill_rect(-12.0 * scale, neck_top_y, 24.0 * scale, 220.0 * scale, pal.wood_ebony);
    canvas.stroke_rect(-12.0 * scale, neck_top_y, 24.0 * scale, 220.0 * scale, pal.ink_dark, 1.2 * scale);

    for (p, &py) in [neck_top_y - 25.0 * scale, neck_top_y - 45.0 * scale].iter().enumerate() {
        let side = if p == 0 { -1.0 } else { 1.0 };
        canvas.stroke_line(0.0, py, side * 28.0 * scale, py, pal.wood_ebony, 3.5 * scale);
        canvas.fill_circle(side * 28.0 * scale, py, 4.5 * scale, pal.wood_ebony);
    }

    let scroll_y = neck_top_y - 75.0 * scale;
    let mut sc_pb = tiny_skia::PathBuilder::new();
    sc_pb.move_to(0.0, neck_top_y - 50.0 * scale);
    sc_pb.cubic_to(16.0 * scale, neck_top_y - 50.0 * scale, 18.0 * scale, scroll_y, 0.0, scroll_y);
    sc_pb.cubic_to(-12.0 * scale, scroll_y, -12.0 * scale, scroll_y + 16.0 * scale, 0.0, scroll_y + 16.0 * scale);
    if let Some(sc_path) = sc_pb.finish() {
        canvas.stroke_path_fine(&sc_path, pal.wood_cedar, 4.0 * scale);
        canvas.stroke_path_fine(&sc_path, pal.ink_dark, 1.2 * scale);
    }

    for s in 0..4 {
        let sx = -7.5 * scale + (s as f32) * 5.0 * scale;
        canvas.stroke_line(sx, neck_top_y, sx, bridge_y + 35.0 * scale, pal.silver_key, 1.2 * scale);
    }

    let bow_shift = (tau * 16.0 * PI).sin() * 80.0 * scale;
    let bow_y = bridge_y - 22.0 * scale;

    canvas.stroke_line(-220.0 * scale + bow_shift, bow_y - 12.0 * scale, 220.0 * scale + bow_shift, bow_y + 12.0 * scale, Color::hex("#732E16"), 3.2 * scale);
    canvas.stroke_line(-215.0 * scale + bow_shift, bow_y - 7.0 * scale, 215.0 * scale + bow_shift, bow_y + 17.0 * scale, pal.linen_white, 2.0 * scale);
    canvas.fill_circle(-200.0 * scale + bow_shift, bow_y - 9.0 * scale, 4.5 * scale, pal.wood_ebony);

    canvas.restore();
}

// =============================================================================
// 6. EL ACORDEÓN HOHNER CORONA III REALISTA (FUELLE VALLENATO RESPIRANDO)
// =============================================================================

pub fn draw_realistic_accordion(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    canvas.save();
    canvas.translate(cx, cy);

    let breathe = (tau * 12.0 * PI).sin();
    let bellows_w = (140.0 + breathe * 45.0) * scale;
    let box_h = 240.0 * scale;
    let box_w = 95.0 * scale;

    let pleats = 12;
    let p_step = bellows_w / pleats as f32;
    let b_left = -bellows_w * 0.5;

    for p in 0..pleats {
        let px1 = b_left + (p as f32) * p_step;
        let px2 = px1 + p_step * 0.5;
        let px3 = px1 + p_step;

        let mut pleat_pb = tiny_skia::PathBuilder::new();
        pleat_pb.move_to(px1, -box_h * 0.42);
        pleat_pb.line_to(px2, -box_h * 0.48);
        pleat_pb.line_to(px3, -box_h * 0.42);
        pleat_pb.line_to(px3, box_h * 0.42);
        pleat_pb.line_to(px2, box_h * 0.48);
        pleat_pb.line_to(px1, box_h * 0.42);
        pleat_pb.close();

        if let Some(path) = pleat_pb.finish() {
            let pleat_col = if p % 2 == 0 { Color::hex("#C41E3A") } else { Color::hex("#8B0000") };
            canvas.fill_path(&path, pleat_col);
            canvas.stroke_path_fine(&path, pal.ink_dark, 1.2 * scale);
        }

        canvas.fill_circle(px2, -box_h * 0.48, 3.5 * scale, pal.silver_key);
        canvas.fill_circle(px2, box_h * 0.48, 3.5 * scale, pal.silver_key);
    }

    let right_box_x = bellows_w * 0.5;
    canvas.fill_rect(right_box_x, -box_h * 0.5, box_w, box_h, Color::hex("#B22222"));
    canvas.stroke_rect(right_box_x, -box_h * 0.5, box_w, box_h, pal.ink_dark, 2.0 * scale);

    canvas.fill_rect(right_box_x + 8.0 * scale, -box_h * 0.44, box_w * 0.45, box_h * 0.88, pal.silver_key);
    canvas.stroke_rect(right_box_x + 8.0 * scale, -box_h * 0.44, box_w * 0.45, box_h * 0.88, pal.ink_dark, 1.2 * scale);

    let btn_col_x = [right_box_x + box_w * 0.62, right_box_x + box_w * 0.74, right_box_x + box_w * 0.86];
    for (col, &bx) in btn_col_x.iter().enumerate() {
        let count = if col == 1 { 11 } else { 10 };
        for b in 0..count {
            let by = -box_h * 0.38 + (b as f32) * 17.5 * scale;
            canvas.fill_circle(bx, by, 5.0 * scale, pal.linen_white);
            canvas.stroke_circle(bx, by, 5.0 * scale, pal.ink_dark, 1.0 * scale);
        }
    }

    let left_box_x = -bellows_w * 0.5 - box_w * 0.8;
    canvas.fill_rect(left_box_x, -box_h * 0.5, box_w * 0.8, box_h, Color::hex("#B22222"));
    canvas.stroke_rect(left_box_x, -box_h * 0.5, box_w * 0.8, box_h, pal.ink_dark, 2.0 * scale);

    canvas.stroke_line(left_box_x + 8.0 * scale, -box_h * 0.35, left_box_x + 8.0 * scale, box_h * 0.35, Color::hex("#3E2723"), 14.0 * scale);

    for bc in 0..2 {
        for br in 0..6 {
            let bx = left_box_x + 24.0 * scale + (bc as f32) * 18.0 * scale;
            let by = -box_h * 0.28 + (br as f32) * 24.0 * scale;
            canvas.fill_circle(bx, by, 6.0 * scale, pal.silver_key);
            canvas.stroke_circle(bx, by, 6.0 * scale, pal.ink_dark, 1.2 * scale);
        }
    }

    draw_vector_text(canvas, right_box_x + box_w * 0.3, -box_h * 0.38, "CORONA III", 9.0 * scale, pal.gold_base, true);

    canvas.restore();
}

// =============================================================================
// 7. EL GALEÓN ESPAÑOL EN LA SELVA REALISTA (17TH CENTURY GALLEON)
// =============================================================================

pub fn draw_realistic_galleon(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    _tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    canvas.save();
    canvas.translate(cx, cy);

    let l = 780.0 * scale;

    for f in 0..14 {
        let fx = -l * 0.5 + (f as f32) * (l / 13.0);
        let fy = 60.0 * scale + ((f as f32 * 1.5).sin()) * 40.0 * scale;
        draw_jungle_leaf(canvas, fx, fy, 80.0 * scale, pal.leaf_deep);
    }

    let bow_x = l * 0.48;
    let stern_x = -l * 0.44;
    let keel_y = 120.0 * scale;
    let deck_y = -30.0 * scale;

    let mut hull_pb = tiny_skia::PathBuilder::new();
    hull_pb.move_to(bow_x, -70.0 * scale);
    hull_pb.cubic_to(bow_x + 35.0 * scale, -60.0 * scale, bow_x + 20.0 * scale, keel_y * 0.4, bow_x - 40.0 * scale, keel_y);
    hull_pb.line_to(stern_x + 60.0 * scale, keel_y);
    hull_pb.cubic_to(stern_x - 10.0 * scale, keel_y * 0.8, stern_x - 30.0 * scale, keel_y * 0.2, stern_x, deck_y - 95.0 * scale);
    hull_pb.line_to(stern_x + 90.0 * scale, deck_y - 95.0 * scale);
    hull_pb.line_to(stern_x + 90.0 * scale, deck_y);
    hull_pb.line_to(bow_x - 100.0 * scale, deck_y);
    hull_pb.line_to(bow_x - 100.0 * scale, -70.0 * scale);
    hull_pb.close();

    if let Some(path) = hull_pb.finish() {
        canvas.fill_path(&path, Color::hex("#3D2817"));
        canvas.stroke_path_fine(&path, pal.ink_dark, 2.5 * scale);

        for p in 0..10 {
            let py = -70.0 * scale + (p as f32) * 19.0 * scale;
            canvas.stroke_line(stern_x + 10.0 * scale, py, bow_x - 10.0 * scale, py, pal.ink_dark.with_alpha(0.65), 1.2 * scale);
        }
    }

    let stern_gallery_x = stern_x + 25.0 * scale;
    let stern_gallery_y = deck_y - 75.0 * scale;
    for w in 0..5 {
        let wx = stern_gallery_x + (w as f32) * 14.0 * scale;
        canvas.fill_rect(wx, stern_gallery_y, 10.0 * scale, 18.0 * scale, pal.gold_base.with_alpha(0.85));
        canvas.stroke_rect(wx, stern_gallery_y, 10.0 * scale, 18.0 * scale, pal.ink_dark, 1.2 * scale);
        canvas.stroke_line(wx, stern_gallery_y + 9.0 * scale, wx + 10.0 * scale, stern_gallery_y + 9.0 * scale, pal.ink_dark, 0.8 * scale);
    }

    canvas.fill_circle(bow_x + 22.0 * scale, -60.0 * scale, 12.0 * scale, pal.gold_deep);
    canvas.stroke_circle(bow_x + 22.0 * scale, -60.0 * scale, 12.0 * scale, pal.ink_dark, 1.4 * scale);

    for gp in 0..6 {
        let gx = stern_x + 140.0 * scale + (gp as f32) * 65.0 * scale;
        let gy = deck_y + 35.0 * scale;
        canvas.fill_rect(gx - 8.0 * scale, gy - 8.0 * scale, 16.0 * scale, 16.0 * scale, pal.ink_dark);
        canvas.stroke_rect(gx - 8.0 * scale, gy - 8.0 * scale, 16.0 * scale, 16.0 * scale, pal.gold_deep, 1.2 * scale);
        canvas.fill_circle(gx, gy, 4.5 * scale, pal.brass_bell);
        canvas.fill_circle(gx, gy, 2.5 * scale, pal.ink_dark);
    }

    let mast_x = [stern_x + 110.0 * scale, -l * 0.05, bow_x - 140.0 * scale];
    let mast_h = [280.0 * scale, 380.0 * scale, 310.0 * scale];

    for (m, &mx) in mast_x.iter().enumerate() {
        let mh = mast_h[m];
        canvas.stroke_line(mx, deck_y + 10.0 * scale, mx, deck_y - mh, pal.wood_cedar, 7.0 * scale);
        canvas.stroke_line(mx, deck_y + 10.0 * scale, mx, deck_y - mh, pal.ink_dark, 1.2 * scale);

        canvas.fill_rect(mx - 18.0 * scale, deck_y - mh * 0.65, 36.0 * scale, 10.0 * scale, pal.wood_ebony);
        canvas.stroke_rect(mx - 18.0 * scale, deck_y - mh * 0.65, 36.0 * scale, 10.0 * scale, pal.ink_dark, 1.0 * scale);

        let yard_w = mh * 0.48;
        canvas.stroke_line(mx - yard_w, deck_y - mh * 0.78, mx + yard_w, deck_y - mh * 0.78, pal.wood_cedar, 4.5 * scale);

        let mut sail_pb = tiny_skia::PathBuilder::new();
        sail_pb.move_to(mx - yard_w * 0.85, deck_y - mh * 0.78);
        sail_pb.cubic_to(mx - yard_w * 0.4, deck_y - mh * 0.45, mx + yard_w * 0.4, deck_y - mh * 0.45, mx + yard_w * 0.85, deck_y - mh * 0.78);
        if let Some(sail_path) = sail_pb.finish() {
            canvas.fill_path(&sail_path, pal.paper_grain.with_alpha(0.85));
            canvas.stroke_path_fine(&sail_path, pal.ink_sepia, 1.5 * scale);
        }

        canvas.stroke_line(mx, deck_y - mh * 0.65, mx - 45.0 * scale, deck_y, pal.ink_sepia.with_alpha(0.6), 1.2 * scale);
        canvas.stroke_line(mx, deck_y - mh * 0.65, mx + 45.0 * scale, deck_y, pal.ink_sepia.with_alpha(0.6), 1.2 * scale);
    }

    canvas.stroke_line(bow_x - 30.0 * scale, -60.0 * scale, bow_x + 130.0 * scale, -130.0 * scale, pal.wood_cedar, 6.0 * scale);

    for v in 0..8 {
        let vx = stern_x + 60.0 * scale + (v as f32) * 75.0 * scale;
        let mut vine_pb = tiny_skia::PathBuilder::new();
        vine_pb.move_to(vx, deck_y - 200.0 * scale);
        vine_pb.cubic_to(vx + 25.0 * scale, deck_y - 80.0 * scale, vx - 20.0 * scale, deck_y + 40.0 * scale, vx + 10.0 * scale, keel_y + 20.0 * scale);
        if let Some(vpath) = vine_pb.finish() {
            canvas.stroke_path_fine(&vpath, pal.leaf_emerald, 3.2 * scale);
        }
        canvas.fill_circle(vx + 12.0 * scale, deck_y + 10.0 * scale, 7.5 * scale, Color::hex("#9B51E0"));
        canvas.fill_circle(vx + 12.0 * scale, deck_y + 10.0 * scale, 3.0 * scale, pal.gold_base);
    }

    canvas.restore();
}

fn draw_jungle_leaf(canvas: &mut Canvas, x: f32, y: f32, size: f32, color: Color) {
    let mut l_pb = tiny_skia::PathBuilder::new();
    l_pb.move_to(x, y);
    l_pb.cubic_to(x - size * 0.5, y - size * 0.6, x - size * 0.3, y - size, x, y - size * 1.3);
    l_pb.cubic_to(x + size * 0.3, y - size, x + size * 0.5, y - size * 0.6, x, y);
    l_pb.close();
    if let Some(path) = l_pb.finish() {
        canvas.fill_path(&path, color);
        canvas.stroke_line(x, y, x, y - size * 1.25, Color::hex("#1F130B"), 1.2);
    }
}

// =============================================================================
// 8. EL CASTAÑO ANCESTRAL Y DON JOSÉ ARCADIO ENCADENADO REALISTA
// =============================================================================

pub fn draw_realistic_chestnut_tree(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    _w: f32,
    h: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    let canopy_y = cy - 240.0;
    for c in 0..16 {
        let ang = (c as f32 / 16.0) * PI;
        let c_rad = 360.0 + ((c as f32 * 1.8).sin()) * 80.0;
        let c_x = cx + ang.cos() * c_rad;
        let c_y = canopy_y - ang.sin() * 200.0;
        canvas.fill_circle(c_x, c_y, 140.0, pal.leaf_deep);
        canvas.fill_circle(c_x, c_y, 115.0, pal.leaf_emerald);
        canvas.stroke_circle(c_x, c_y, 140.0, pal.ink_dark, 2.0);
    }

    for f in 0..24 {
        let fx = cx - 440.0 + (f as f32) * 38.0;
        let fy = canopy_y - 80.0 + ((f as f32 * 2.2).sin()) * 70.0;
        canvas.fill_circle(fx, fy, 9.0, pal.gold_base);
        canvas.fill_circle(fx + 2.0, fy - 2.0, 4.0, Color::WHITE);
    }

    let trunk_w = 360.0;
    let ground_y = h - 160.0;

    let mut trunk_pb = tiny_skia::PathBuilder::new();
    trunk_pb.move_to(cx - trunk_w * 0.45, canopy_y + 80.0);
    trunk_pb.cubic_to(cx - trunk_w * 0.50, cy - 40.0, cx - trunk_w * 0.65, cy + 120.0, cx - trunk_w * 0.95, ground_y);
    trunk_pb.line_to(cx + trunk_w * 0.95, ground_y);
    trunk_pb.cubic_to(cx + trunk_w * 0.65, cy + 120.0, cx + trunk_w * 0.50, cy - 40.0, cx + trunk_w * 0.45, canopy_y + 80.0);
    trunk_pb.close();

    if let Some(path) = trunk_pb.finish() {
        canvas.fill_path(&path, Color::hex("#4A3320"));
        canvas.stroke_path_fine(&path, pal.ink_dark, 4.0);

        for i in 0..22 {
            let y_pos = canopy_y + 100.0 + (i as f32) * 20.0;
            let x_off = ((i as f32 * 1.5).sin()) * 55.0;
            canvas.stroke_line(cx + x_off - 45.0, y_pos, cx + x_off + 45.0, y_pos + 18.0, pal.ink_dark, 3.8);
            canvas.stroke_line(cx + x_off - 35.0, y_pos + 4.0, cx + x_off + 35.0, y_pos + 14.0, Color::hex("#705036"), 2.0);
        }
    }

    for r in 0..5 {
        let rx = cx - trunk_w * 0.8 + (r as f32) * (trunk_w * 0.4);
        let mut r_pb = tiny_skia::PathBuilder::new();
        r_pb.move_to(rx, ground_y - 40.0);
        r_pb.cubic_to(rx - 60.0, ground_y + 20.0, rx - 30.0, ground_y + 80.0, rx - 90.0, ground_y + 120.0);
        if let Some(rpath) = r_pb.finish() {
            canvas.stroke_path_fine(&rpath, pal.ink_dark, 16.0);
            canvas.stroke_path_fine(&rpath, Color::hex("#5A3E26"), 10.0);
        }
    }

    let chain_y = cy + 50.0;
    let link_count = 18;
    for l in 0..link_count {
        let lx = cx - trunk_w * 0.58 + (l as f32) * (trunk_w * 1.16 / link_count as f32);
        let sag = ((l as f32 / link_count as f32) * PI).sin() * 26.0;
        let ly = chain_y + sag;

        canvas.stroke_circle(lx, ly, 15.0, pal.iron_shadow, 5.0);
        canvas.stroke_circle(lx, ly, 15.0, pal.iron_chain, 2.5);
        canvas.fill_circle(lx - 3.0, ly - 3.0, 3.0, Color::WHITE.with_alpha(0.6));
    }

    let lock_x = cx + 70.0;
    let lock_y = chain_y + 26.0;
    canvas.stroke_circle(lock_x, lock_y - 12.0, 16.0, pal.iron_shadow, 6.0);
    canvas.fill_rect(lock_x - 18.0, lock_y - 6.0, 36.0, 42.0, pal.iron_shadow);
    canvas.stroke_rect(lock_x - 18.0, lock_y - 6.0, 36.0, 42.0, pal.ink_dark, 2.5);
    canvas.fill_circle(lock_x, lock_y + 10.0, 4.5, pal.ink_dark);
    canvas.fill_rect(lock_x - 2.0, lock_y + 10.0, 4.0, 10.0, pal.ink_dark);

    let ja_x = cx - 110.0;
    let ja_y = ground_y - 60.0;

    canvas.stroke_line(ja_x - 40.0, ja_y - 120.0, ja_x - 40.0, ja_y + 60.0, pal.wood_cedar, 12.0);
    canvas.stroke_line(ja_x - 40.0, ja_y - 120.0, ja_x - 40.0, ja_y + 60.0, pal.ink_dark, 2.0);

    let mut robe_pb = tiny_skia::PathBuilder::new();
    robe_pb.move_to(ja_x - 25.0, ja_y - 60.0);
    robe_pb.line_to(ja_x + 50.0, ja_y - 45.0);
    robe_pb.line_to(ja_x + 65.0, ja_y + 55.0);
    robe_pb.line_to(ja_x - 30.0, ja_y + 55.0);
    robe_pb.close();
    if let Some(rpath) = robe_pb.finish() {
        canvas.fill_path(&rpath, pal.linen_white);
        canvas.stroke_path_fine(&rpath, pal.ink_sepia, 2.0);
    }

    canvas.fill_circle(ja_x, ja_y - 95.0, 26.0, pal.linen_white);
    canvas.stroke_circle(ja_x, ja_y - 95.0, 26.0, pal.ink_sepia, 2.0);

    canvas.fill_circle(ja_x + 8.0, ja_y - 96.0, 4.0, pal.ink_dark);
    canvas.stroke_line(ja_x, ja_y - 104.0, ja_x + 16.0, ja_y - 102.0, pal.linen_white, 3.5);

    let mut beard_pb = tiny_skia::PathBuilder::new();
    beard_pb.move_to(ja_x - 12.0, ja_y - 88.0);
    beard_pb.cubic_to(ja_x + 25.0, ja_y - 50.0, ja_x + 35.0, ja_y - 10.0, ja_x + 10.0, ja_y + 25.0);
    beard_pb.cubic_to(ja_x - 15.0, ja_y - 10.0, ja_x - 18.0, ja_y - 50.0, ja_x - 12.0, ja_y - 88.0);
    beard_pb.close();
    if let Some(bpath) = beard_pb.finish() {
        canvas.fill_path(&bpath, pal.linen_white);
        canvas.stroke_path_fine(&bpath, Color::hex("#B8B2A7"), 2.0);
        for m in 0..4 {
            let my = ja_y - 70.0 + (m as f32) * 20.0;
            canvas.stroke_line(ja_x - 5.0, my, ja_x + 15.0, my + 14.0, Color::hex("#8C8478"), 1.2);
        }
    }

    for l in 0..10 {
        let lx = cx - 350.0 + (l as f32) * 70.0 + (tau * 8.0 + l as f32).sin() * 25.0;
        let ly = canopy_y + 60.0 + ((tau * 2.0 + l as f32 * 0.3) % 1.0) * (h - canopy_y);
        canvas.fill_circle(lx, ly, 6.0, pal.gold_base);
    }
}

// =============================================================================
// 9. REMEDIOS LA BELLA Y LA ASCENSIÓN CON SÁBANAS REALISTAS
// =============================================================================

pub fn draw_realistic_remedios_ascension(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    for row in 0..20 {
        let ry = (row as f32) * (h / 20.0);
        let t = row as f32 / 20.0;
        let col = Color::hex("#E88A3C").with_alpha((1.0 - t * 0.7) * 0.45);
        canvas.fill_rect(0.0, ry, w, h / 20.0, col);
    }

    for r in 0..12 {
        let rx = (r as f32) * (w / 11.0);
        canvas.stroke_line(rx, h, rx + 120.0, 0.0, pal.gold_high.with_alpha(0.22), 8.0);
    }

    let asc_y = cy - 20.0 + (tau * 4.0 * PI).sin() * 18.0;

    let sheet_w = 480.0;
    let sheet_h = 320.0;
    let wave1 = (tau * 10.0 * PI).sin() * 28.0;
    let wave2 = (tau * 10.0 * PI + 1.2).cos() * 24.0;

    for s in 0..2 {
        let offset_x = if s == 0 { -sheet_w * 0.4 } else { sheet_w * 0.1 };
        let offset_y = if s == 0 { -120.0 } else { -70.0 };

        let mut s_pb = tiny_skia::PathBuilder::new();
        s_pb.move_to(cx + offset_x, asc_y + offset_y);
        s_pb.cubic_to(
            cx + offset_x + sheet_w * 0.35,
            asc_y + offset_y - 120.0 + wave1,
            cx + offset_x + sheet_w * 0.65,
            asc_y + offset_y - 60.0 + wave2,
            cx + offset_x + sheet_w,
            asc_y + offset_y + 40.0,
        );
        s_pb.cubic_to(
            cx + offset_x + sheet_w * 0.75,
            asc_y + offset_y + sheet_h + wave1 * 0.5,
            cx + offset_x + sheet_w * 0.25,
            asc_y + offset_y + sheet_h * 0.8 + wave2 * 0.5,
            cx + offset_x,
            asc_y + offset_y + sheet_h * 0.5,
        );
        s_pb.close();

        if let Some(spath) = s_pb.finish() {
            canvas.fill_path(&spath, pal.linen_white);
            canvas.stroke_path_fine(&spath, pal.gold_base.with_alpha(0.65), 2.5);

            for pl in 0..5 {
                let py = asc_y + offset_y + (pl as f32) * 55.0;
                canvas.stroke_line(cx + offset_x + 40.0, py, cx + offset_x + sheet_w - 60.0, py + wave1 * 0.6, Color::hex("#E2DDD2"), 2.2);
            }
        }
    }

    let r_x = cx;
    let r_y = asc_y + 40.0;

    let mut dress_pb = tiny_skia::PathBuilder::new();
    dress_pb.move_to(r_x - 18.0, r_y - 40.0);
    dress_pb.line_to(r_x + 18.0, r_y - 40.0);
    dress_pb.cubic_to(r_x + 55.0, r_y + 80.0, r_x + 75.0, r_y + 140.0 + wave1, r_x + 20.0, r_y + 150.0);
    dress_pb.cubic_to(r_x - 45.0, r_y + 140.0 + wave2, r_x - 65.0, r_y + 80.0, r_x - 18.0, r_y - 40.0);
    dress_pb.close();
    if let Some(dpath) = dress_pb.finish() {
        canvas.fill_path(&dpath, pal.linen_white);
        canvas.stroke_path_fine(&dpath, pal.gold_base, 1.8);
    }

    canvas.stroke_line(r_x - 16.0, r_y - 30.0, r_x - 85.0, r_y - 85.0, Color::hex("#E8B896"), 7.0);
    canvas.stroke_line(r_x + 16.0, r_y - 30.0, r_x + 85.0, r_y - 85.0, Color::hex("#E8B896"), 7.0);

    canvas.fill_circle(r_x, r_y - 65.0, 18.0, Color::hex("#F2CBB0"));
    canvas.stroke_circle(r_x, r_y - 65.0, 18.0, pal.ink_sepia, 1.2);

    let mut hair_pb = tiny_skia::PathBuilder::new();
    hair_pb.move_to(r_x - 15.0, r_y - 75.0);
    hair_pb.cubic_to(r_x - 65.0, r_y - 110.0 + wave1, r_x - 110.0, r_y - 40.0, r_x - 85.0, r_y + 10.0);
    if let Some(hpath) = hair_pb.finish() {
        canvas.stroke_path_fine(&hpath, Color::hex("#21140E"), 9.0);
    }

    for p in 0..16 {
        let px = cx - 400.0 + (p as f32) * 55.0 + ((tau * 6.0 + p as f32).sin()) * 40.0;
        let py = (r_y - 200.0 + (p as f32) * 45.0 + (tau * 300.0)) % h;
        canvas.fill_circle(px, py, 7.5, Color::hex("#D9183B"));
        canvas.fill_circle(px + 2.0, py - 2.0, 3.5, Color::hex("#FF5E7E"));
    }
}

// =============================================================================
// 10. MAURICIO BABILONIA Y LA NUBE DE MARIPOSAS AMARILLAS REALISTA
// =============================================================================

pub fn draw_realistic_mauricio_butterflies(
    canvas: &mut Canvas,
    cx: f32,
    _cy: f32,
    w: f32,
    h: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    for b in 0..8 {
        let side = if b % 2 == 0 { -1.0 } else { 1.0 };
        let bx = if side < 0.0 { 120.0 } else { w - 120.0 };
        let by = 180.0 + (b as f32) * 110.0;
        draw_banana_leaf(canvas, bx, by, 320.0, side * 0.4, pal.leaf_emerald);
    }

    let mb_x = cx;
    let mb_y = h - 220.0;

    canvas.fill_rect(mb_x - 30.0, mb_y - 120.0, 60.0, 140.0, Color::hex("#2C3E50"));
    canvas.stroke_rect(mb_x - 30.0, mb_y - 120.0, 60.0, 140.0, pal.ink_dark, 2.0);
    canvas.fill_circle(mb_x, mb_y - 160.0, 24.0, Color::hex("#C48D66"));
    canvas.stroke_circle(mb_x, mb_y - 160.0, 24.0, pal.ink_dark, 1.8);
    canvas.fill_circle(mb_x, mb_y - 176.0, 26.0, Color::hex("#121110"));

    for i in 0..32 {
        let t_off = (i as f32) * 0.19;
        let ang = (tau * 4.0 * PI + t_off * 3.0);
        let dist = 110.0 + ((i as f32 * 1.7).sin()) * 260.0;
        let bf_x = cx + ang.cos() * dist;
        let bf_y = mb_y - 120.0 + ang.sin() * (dist * 0.55) + ((i as f32 * 2.3).cos()) * 40.0;
        let flap = ((tau * 36.0 * PI + (i as f32) * 1.5).sin()).abs();
        let scale = 0.55 + ((i % 5) as f32) * 0.18;

        draw_realistic_yellow_butterfly(canvas, bf_x, bf_y, scale, flap);
    }
}

fn draw_banana_leaf(canvas: &mut Canvas, x: f32, y: f32, length: f32, angle: f32, color: Color) {
    canvas.save();
    canvas.translate(x, y);
    canvas.rotate(angle);

    let mut l_pb = tiny_skia::PathBuilder::new();
    l_pb.move_to(0.0, 0.0);
    l_pb.cubic_to(length * 0.35, -70.0, length * 0.75, -50.0, length, 0.0);
    l_pb.cubic_to(length * 0.75, 50.0, length * 0.35, 70.0, 0.0, 0.0);
    l_pb.close();
    if let Some(path) = l_pb.finish() {
        canvas.fill_path(&path, color);
        canvas.stroke_line(0.0, 0.0, length, 0.0, Color::hex("#0D2811"), 4.0);
        for s in 1..10 {
            let sx = (s as f32 / 10.0) * length;
            canvas.stroke_line(sx, 0.0, sx + 25.0, -45.0, Color::hex("#0D2811").with_alpha(0.4), 1.5);
            canvas.stroke_line(sx, 0.0, sx + 25.0, 45.0, Color::hex("#0D2811").with_alpha(0.4), 1.5);
        }
    }

    canvas.restore();
}

pub fn draw_realistic_yellow_butterfly(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    flap: f32,
) {
    let pal = MacondoRealistaPalette::default();
    let flap_w = (1.0 - flap * 0.75).max(0.18);

    canvas.save();
    canvas.translate(cx, cy);

    canvas.fill_circle(0.0, -12.0 * scale, 3.5 * scale, pal.ink_dark);
    canvas.fill_rect(-2.5 * scale, -8.0 * scale, 5.0 * scale, 18.0 * scale, pal.ink_dark);
    canvas.stroke_line(0.0, -12.0 * scale, -8.0 * scale, -24.0 * scale, pal.ink_dark, 1.0 * scale);
    canvas.stroke_line(0.0, -12.0 * scale, 8.0 * scale, -24.0 * scale, pal.ink_dark, 1.0 * scale);

    for &side in &[-1.0_f32, 1.0_f32] {
        canvas.save();
        canvas.scale(side * flap_w, 1.0);

        let mut fw_pb = tiny_skia::PathBuilder::new();
        fw_pb.move_to(3.0 * scale, -6.0 * scale);
        fw_pb.cubic_to(20.0 * scale, -35.0 * scale, 55.0 * scale, -38.0 * scale, 75.0 * scale, -20.0 * scale);
        fw_pb.cubic_to(65.0 * scale, 5.0 * scale, 35.0 * scale, 8.0 * scale, 3.0 * scale, 2.0 * scale);
        fw_pb.close();
        if let Some(fw_path) = fw_pb.finish() {
            canvas.fill_path(&fw_path, pal.butterfly_yellow);
            canvas.stroke_path_fine(&fw_path, pal.butterfly_orange, 1.5 * scale);
            canvas.stroke_path_fine(&fw_path, pal.ink_dark, 1.0 * scale);
        }

        let mut hw_pb = tiny_skia::PathBuilder::new();
        hw_pb.move_to(3.0 * scale, 2.0 * scale);
        hw_pb.cubic_to(35.0 * scale, 8.0 * scale, 50.0 * scale, 25.0 * scale, 30.0 * scale, 45.0 * scale);
        hw_pb.cubic_to(10.0 * scale, 40.0 * scale, 5.0 * scale, 20.0 * scale, 3.0 * scale, 2.0 * scale);
        hw_pb.close();
        if let Some(hw_path) = hw_pb.finish() {
            canvas.fill_path(&hw_path, pal.butterfly_yellow);
            canvas.stroke_path_fine(&hw_path, pal.ink_dark, 1.0 * scale);
        }

        canvas.restore();
    }

    canvas.restore();
}

// =============================================================================
// 11. EL TALLER DEL CORONEL AURELIANO Y EL PECECITO DE ORO REALISTA
// =============================================================================

pub fn draw_realistic_aureliano_workshop(
    canvas: &mut Canvas,
    cx: f32,
    _cy: f32,
    w: f32,
    h: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    let table_y = h * 0.62;
    canvas.fill_rect(0.0, table_y, w, h - table_y, Color::hex("#382315"));
    canvas.stroke_line(0.0, table_y, w, table_y, pal.ink_dark, 4.0);

    let lamp_x = cx - 380.0;
    let lamp_y = table_y - 80.0;
    canvas.fill_rect(lamp_x - 22.0, lamp_y, 44.0, 80.0, pal.brass_bell);
    canvas.stroke_rect(lamp_x - 22.0, lamp_y, 44.0, 80.0, pal.ink_dark, 2.0);

    let flame_w = (tau * 18.0 * PI).sin() * 4.0;
    canvas.fill_circle(lamp_x + flame_w, lamp_y - 20.0, 16.0, pal.gold_base);
    canvas.fill_circle(lamp_x + flame_w, lamp_y - 20.0, 8.0, Color::WHITE);

    canvas.fill_circle(cx + 380.0, table_y - 20.0, 35.0, pal.iron_shadow);
    canvas.stroke_circle(cx + 380.0, table_y - 20.0, 35.0, pal.ink_dark, 2.0);
    canvas.fill_circle(cx + 380.0, table_y - 20.0, 24.0, pal.gold_base);

    draw_monumental_golden_fish(canvas, cx, table_y - 70.0, 2.4, tau);
}

fn draw_monumental_golden_fish(canvas: &mut Canvas, cx: f32, cy: f32, scale: f32, tau: f32) {
    let pal = MacondoRealistaPalette::default();

    canvas.save();
    canvas.translate(cx, cy);

    let fish_len = 160.0 * scale;

    let mut head_pb = tiny_skia::PathBuilder::new();
    head_pb.move_to(-fish_len * 0.35, -28.0 * scale);
    head_pb.cubic_to(-fish_len * 0.55, -22.0 * scale, -fish_len * 0.60, 15.0 * scale, -fish_len * 0.35, 28.0 * scale);
    head_pb.close();
    if let Some(hpath) = head_pb.finish() {
        canvas.fill_path(&hpath, pal.gold_base);
        canvas.stroke_path_fine(&hpath, pal.gold_deep, 2.0 * scale);
    }

    canvas.fill_circle(-fish_len * 0.44, -5.0 * scale, 6.0 * scale, pal.ruby_eye);
    canvas.stroke_circle(-fish_len * 0.44, -5.0 * scale, 6.0 * scale, pal.gold_deep, 1.5 * scale);
    canvas.fill_circle(-fish_len * 0.46, -7.0 * scale, 2.0 * scale, Color::WHITE);

    let rows = 6;
    let cols = 4;
    for r in 0..rows {
        for c in 0..cols {
            let sx = -fish_len * 0.30 + (r as f32) * (fish_len * 0.12);
            let sy = -20.0 * scale + (c as f32) * (14.0 * scale);
            let wave = ((tau * 12.0 * PI + (r as f32) * 0.8).sin()) * 3.0 * scale;

            canvas.fill_circle(sx, sy + wave, 9.5 * scale, pal.gold_base);
            canvas.stroke_circle(sx, sy + wave, 9.5 * scale, pal.gold_deep, 1.4 * scale);
            canvas.fill_circle(sx - 2.5 * scale, sy + wave - 2.5 * scale, 3.0 * scale, pal.gold_high);
        }
    }

    let tail_x = fish_len * 0.45;
    let tail_wave = (tau * 16.0 * PI).sin() * 8.0 * scale;
    let mut tail_pb = tiny_skia::PathBuilder::new();
    tail_pb.move_to(tail_x, 0.0);
    tail_pb.cubic_to(tail_x + 35.0 * scale, -45.0 * scale + tail_wave, tail_x + 65.0 * scale, -35.0 * scale + tail_wave, tail_x + 55.0 * scale, 0.0);
    tail_pb.cubic_to(tail_x + 65.0 * scale, 35.0 * scale + tail_wave, tail_x + 35.0 * scale, 45.0 * scale + tail_wave, tail_x, 0.0);
    tail_pb.close();
    if let Some(tpath) = tail_pb.finish() {
        canvas.fill_path(&tpath, pal.gold_base);
        canvas.stroke_path_fine(&tpath, pal.gold_deep, 2.0 * scale);
    }

    canvas.restore();
}

// =============================================================================
// 12. EL TREN AMARILLO DE LA BANANERA REALISTA
// =============================================================================

pub fn draw_realistic_train_plantation(
    canvas: &mut Canvas,
    _cx: f32,
    _cy: f32,
    w: f32,
    h: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    let bridge_y = h * 0.68;
    for b in 0..12 {
        let bx = (b as f32) * (w / 11.0);
        canvas.stroke_line(bx, bridge_y, bx + 15.0, h, Color::hex("#422A1D"), 10.0);
        canvas.stroke_line(bx, bridge_y, bx + 15.0, h, pal.ink_dark, 2.0);
    }
    canvas.stroke_line(0.0, bridge_y - 6.0, w, bridge_y - 6.0, pal.iron_shadow, 8.0);

    let train_x = ((tau * w * 1.2) % (w + 600.0)) - 300.0;
    let train_y = bridge_y - 12.0;

    canvas.fill_rect(train_x - 180.0, train_y - 95.0, 180.0, 95.0, pal.butterfly_yellow);
    canvas.stroke_rect(train_x - 180.0, train_y - 95.0, 180.0, 95.0, pal.ink_dark, 2.5);

    canvas.fill_rect(train_x, train_y - 75.0, 110.0, 75.0, pal.ink_dark);
    canvas.stroke_rect(train_x, train_y - 75.0, 110.0, 75.0, pal.gold_deep, 2.0);

    canvas.fill_rect(train_x + 75.0, train_y - 125.0, 22.0, 50.0, pal.ink_dark);
    for v in 0..6 {
        let vx = train_x + 60.0 - (v as f32) * 55.0;
        let vy = train_y - 150.0 - (v as f32) * 20.0;
        let vr = 22.0 + (v as f32) * 12.0;
        canvas.fill_circle(vx, vy, vr, Color::WHITE.with_alpha(0.65));
    }

    for r in 0..4 {
        let rx = train_x - 140.0 + (r as f32) * 75.0;
        canvas.fill_circle(rx, train_y, 24.0, pal.iron_shadow);
        canvas.stroke_circle(rx, train_y, 24.0, pal.ink_dark, 2.5);
    }
}

// =============================================================================
// 13. EL VIENTO BÍBLICO Y LOS PERGAMINOS EN SÁNSCRITO REALISTA
// =============================================================================

pub fn draw_realistic_biblical_whirlwind(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    _w: f32,
    _h: f32,
    tau: f32,
) {
    let pal = MacondoRealistaPalette::default();

    for sp in 0..18 {
        let t = sp as f32 / 18.0;
        let r_spiral = 60.0 + t * 580.0;
        let ang = tau * 8.0 * PI + t * 14.0;
        let sx = cx + ang.cos() * r_spiral;
        let sy = cy + ang.sin() * (r_spiral * 0.55);

        canvas.save();
        canvas.translate(sx, sy);
        canvas.rotate(ang + 0.5);

        canvas.fill_rect(-25.0, -18.0, 50.0, 36.0, pal.paper_cream);
        canvas.stroke_rect(-25.0, -18.0, 50.0, 36.0, pal.gold_base, 1.5);
        canvas.stroke_line(-18.0, -6.0, 18.0, -6.0, pal.ink_sepia, 1.2);
        canvas.stroke_line(-18.0, 6.0, 18.0, 6.0, pal.ink_sepia, 1.2);

        canvas.restore();
    }
}
