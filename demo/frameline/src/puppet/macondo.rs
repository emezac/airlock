use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::rng::Rng;
use crate::core::schematic::{
    draw_blueprint_header, draw_dimension_bracket, draw_magnifying_loupe, draw_node_callout,
    draw_vector_text,
};
use tiny_skia::BlendMode;
use std::f32::consts::PI;

/// Dibuja una mariposa amarilla de Mauricio Babilonia
/// `wing_angle`: ángulo de aleteo en radianes (0.0 = alas abiertas planas, 0.8 = alas cerradas)
pub fn draw_yellow_butterfly(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    wing_angle: f32,
    _seed: u32,
) {
    let wing_cos = wing_angle.cos().abs().max(0.15);
    let lift_y = -wing_angle.sin() * 12.0 * scale;

    let yellow_bright = Color::hex("#ffd000");
    let yellow_gold = Color::hex("#f5a623");
    let amber_vein = Color::hex("#c47d10");
    let body_col = Color::hex("#2a1e08");

    canvas.save();
    canvas.translate(cx, cy);

    // Ambas alas con perspectiva simétrica
    for &side in &[-1.0_f32, 1.0_f32] {
        canvas.save();
        canvas.scale(side * wing_cos, 1.0);
        canvas.translate(0.0, lift_y);

        // Ala posterior
        let mut hw_pb = tiny_skia::PathBuilder::new();
        hw_pb.move_to(3.0 * scale, 5.0 * scale);
        hw_pb.cubic_to(35.0 * scale, 15.0 * scale, 65.0 * scale, 45.0 * scale, 45.0 * scale, 75.0 * scale);
        hw_pb.cubic_to(25.0 * scale, 95.0 * scale, 10.0 * scale, 65.0 * scale, 3.0 * scale, 25.0 * scale);
        hw_pb.close();
        if let Some(path) = hw_pb.finish() {
            canvas.fill_path(&path, yellow_gold);
            canvas.stroke_path(&path, amber_vein, 1.4 * scale);
        }

        // Ala anterior (más grande, elegante y translúcida)
        let mut fw_pb = tiny_skia::PathBuilder::new();
        fw_pb.move_to(4.0 * scale, -5.0 * scale);
        fw_pb.cubic_to(25.0 * scale, -45.0 * scale, 70.0 * scale, -80.0 * scale, 95.0 * scale, -70.0 * scale);
        fw_pb.cubic_to(105.0 * scale, -40.0 * scale, 85.0 * scale, -10.0 * scale, 60.0 * scale, 10.0 * scale);
        fw_pb.cubic_to(35.0 * scale, 20.0 * scale, 15.0 * scale, 10.0 * scale, 4.0 * scale, -5.0 * scale);
        fw_pb.close();
        if let Some(path) = fw_pb.finish() {
            canvas.fill_path(&path, yellow_bright);
            canvas.stroke_path(&path, amber_vein, 1.8 * scale);

            // Venación alar dorada
            canvas.stroke_line(4.0 * scale, -5.0 * scale, 55.0 * scale, -35.0 * scale, amber_vein, 1.2 * scale);
            canvas.stroke_line(55.0 * scale, -35.0 * scale, 90.0 * scale, -60.0 * scale, amber_vein, 1.0 * scale);
            canvas.stroke_line(55.0 * scale, -35.0 * scale, 78.0 * scale, -25.0 * scale, amber_vein, 1.0 * scale);
            canvas.stroke_line(55.0 * scale, -35.0 * scale, 58.0 * scale, 5.0 * scale, amber_vein, 1.0 * scale);

            // Borde soleado con destello
            canvas.stroke_circle(95.0 * scale, -70.0 * scale, 3.0 * scale, Color::hex("#ffffff").with_alpha(0.8), 1.0);
        }

        canvas.restore();
    }

    // Cuerpo pequeño y antenas
    canvas.fill_circle(0.0, -10.0 * scale, 4.0 * scale, body_col);
    canvas.stroke_line(0.0, -5.0 * scale, 0.0, 25.0 * scale, body_col, 3.2 * scale);

    // Antenas finas
    for &side in &[-1.0_f32, 1.0_f32] {
        let mut ant_pb = tiny_skia::PathBuilder::new();
        ant_pb.move_to(side * 2.0 * scale, -12.0 * scale);
        ant_pb.cubic_to(side * 8.0 * scale, -22.0 * scale, side * 14.0 * scale, -30.0 * scale, side * 18.0 * scale, -38.0 * scale);
        if let Some(path) = ant_pb.finish() {
            canvas.stroke_path(&path, body_col, 1.0 * scale);
            canvas.fill_circle(side * 18.0 * scale, -38.0 * scale, 1.5 * scale, yellow_gold);
        }
    }

    canvas.restore();
}

/// Dibuja el mítico pescadito de oro que fabricaba el Coronel Aureliano Buendía
pub fn draw_pescadito_oro(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    glow_pulse: f32,
) {
    let gold_base = Color::hex("#d4af37");
    let gold_bright = Color::hex("#ffdf6d");
    let gold_dark = Color::hex("#8c6f1a");
    let ruby_eye = Color::hex("#d91438");

    canvas.save();
    canvas.translate(cx, cy);

    // Halo dorado reflectante
    if glow_pulse > 0.0 {
        canvas.fill_circle(0.0, 0.0, 90.0 * scale * (1.0 + glow_pulse * 0.2), gold_bright.with_alpha(0.18 * glow_pulse));
    }

    // Cola bífida articulada
    let mut tail_pb = tiny_skia::PathBuilder::new();
    tail_pb.move_to(-40.0 * scale, 0.0);
    tail_pb.line_to(-75.0 * scale, -28.0 * scale);
    tail_pb.cubic_to(-65.0 * scale, -5.0 * scale, -65.0 * scale, 5.0 * scale, -75.0 * scale, 28.0 * scale);
    tail_pb.close();
    if let Some(path) = tail_pb.finish() {
        canvas.fill_path(&path, gold_base);
        canvas.stroke_path(&path, gold_dark, 1.8 * scale);
        // Rayas de la aleta
        canvas.stroke_line(-45.0 * scale, -2.0 * scale, -70.0 * scale, -20.0 * scale, gold_bright, 1.0 * scale);
        canvas.stroke_line(-45.0 * scale, 2.0 * scale, -70.0 * scale, 20.0 * scale, gold_bright, 1.0 * scale);
    }

    // Cuerpo fusiforme de pez articulado
    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(55.0 * scale, 0.0); // Boca
    body_pb.cubic_to(40.0 * scale, -26.0 * scale, -15.0 * scale, -28.0 * scale, -45.0 * scale, 0.0);
    body_pb.cubic_to(-15.0 * scale, 28.0 * scale, 40.0 * scale, 26.0 * scale, 55.0 * scale, 0.0);
    body_pb.close();
    if let Some(path) = body_pb.finish() {
        canvas.fill_path(&path, gold_base);
        canvas.stroke_path(&path, gold_dark, 2.2 * scale);

        // Escamas superpuestas cinceladas a mano
        let rows = 4;
        let cols = 5;
        for c in 0..cols {
            let sx = -32.0 * scale + (c as f32) * 15.0 * scale;
            for r in 0..rows {
                let sy = -14.0 * scale + (r as f32) * 9.0 * scale;
                canvas.stroke_circle(sx, sy, 5.5 * scale, gold_bright, 1.1 * scale);
            }
        }
    }

    // Aletas dorsal y ventral
    let mut fin_pb = tiny_skia::PathBuilder::new();
    fin_pb.move_to(-5.0 * scale, -22.0 * scale);
    fin_pb.cubic_to(10.0 * scale, -38.0 * scale, 25.0 * scale, -32.0 * scale, 20.0 * scale, -18.0 * scale);
    fin_pb.close();
    if let Some(path) = fin_pb.finish() {
        canvas.fill_path(&path, gold_bright);
        canvas.stroke_path(&path, gold_dark, 1.2 * scale);
    }

    // Ojo de rubí rojo vivo engarzado
    canvas.fill_circle(38.0 * scale, -6.0 * scale, 4.5 * scale, ruby_eye);
    canvas.fill_circle(36.5 * scale, -7.5 * scale, 1.5 * scale, Color::hex("#ffffff")); // Destello

    canvas.restore();
}

/// Dibuja a Don José Arcadio Buendía encadenado al castaño ancestral
pub fn draw_castano_encadenado(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
    seed: u32,
) {
    let bark_dark = Color::hex("#1F150D");
    let bark_mid = Color::hex("#422E1F");
    let leaf_dark = Color::hex("#233D1C");
    let leaf_mid = Color::hex("#3C642A");
    let leaf_light = Color::hex("#5A8C3E");
    let chain_col = Color::hex("#8B929E");
    let yellow_petal = Color::hex("#FFD700");

    let mut rng = Rng::new(seed);

    // 1. Gran copa frondosa del castaño ancestral (canopy)
    let crown_y = cy - 220.0 * scale;
    for c in 0..9 {
        let ang = (c as f32 / 9.0) * PI;
        let c_rad = 180.0 * scale + ((c as f32 * 1.5).sin()) * 40.0 * scale;
        let c_x = cx + ang.cos() * c_rad;
        let c_y = crown_y - ang.sin() * (c_rad * 0.7);
        canvas.fill_circle(c_x, c_y, 90.0 * scale, leaf_dark);
        canvas.fill_circle(c_x, c_y, 75.0 * scale, leaf_mid);
        canvas.stroke_circle(c_x, c_y, 90.0 * scale, bark_dark, 2.0 * scale);
    }
    // Núcleo central de la copa
    canvas.fill_circle(cx, crown_y - 40.0 * scale, 140.0 * scale, leaf_light);
    canvas.stroke_circle(cx, crown_y - 40.0 * scale, 140.0 * scale, leaf_dark, 3.0 * scale);

    // Flores amarillas que florecen en la copa del castaño
    for f in 0..18 {
        let fx = cx - 220.0 * scale + (f as f32 * 26.0 * scale);
        let fy = crown_y - 80.0 * scale + ((f as f32 * 2.1).sin() * 50.0 * scale);
        canvas.fill_circle(fx, fy, 8.0 * scale, yellow_petal);
        canvas.fill_circle(fx + 2.0 * scale, fy - 2.0 * scale, 4.0 * scale, Color::hex("#FFFFFF"));
    }

    // 2. Tronco monumental milenario del castaño
    let trunk_w = 150.0 * scale;
    let mut trunk_pb = tiny_skia::PathBuilder::new();
    trunk_pb.move_to(cx - trunk_w * 0.45, crown_y + 40.0 * scale);
    trunk_pb.line_to(cx - trunk_w * 0.55, cy + 220.0 * scale);
    trunk_pb.cubic_to(cx - trunk_w * 0.8, cy + 290.0 * scale, cx - trunk_w * 1.2, cy + 340.0 * scale, cx - trunk_w * 1.6, cy + 380.0 * scale);
    trunk_pb.line_to(cx + trunk_w * 1.6, cy + 380.0 * scale);
    trunk_pb.cubic_to(cx + trunk_w * 1.2, cy + 340.0 * scale, cx + trunk_w * 0.8, cy + 290.0 * scale, cx + trunk_w * 0.55, cy + 220.0 * scale);
    trunk_pb.line_to(cx + trunk_w * 0.45, crown_y + 40.0 * scale);
    trunk_pb.close();

    if let Some(path) = trunk_pb.finish() {
        canvas.fill_path(&path, bark_mid);
        canvas.stroke_path(&path, bark_dark, 4.0 * scale);
    }

    // Grietas y estrías rugosas en la corteza milenaria
    for i in 0..18 {
        let y_pos = crown_y + 60.0 * scale + (i as f32) * 28.0 * scale;
        let x_off = ((i as f32 * 1.6).sin()) * 35.0 * scale;
        canvas.stroke_line(cx + x_off - 25.0 * scale, y_pos, cx + x_off + 25.0 * scale, y_pos + 22.0 * scale, bark_dark, 3.5 * scale);
        canvas.stroke_line(cx + x_off - 15.0 * scale, y_pos + 6.0 * scale, cx + x_off + 15.0 * scale, y_pos + 18.0 * scale, Color::hex("#6B4D35"), 1.8 * scale);
    }

    // 3. Figura venerable de Don José Arcadio Buendía sentado al pie del árbol
    let patriarca_x = cx - 55.0 * scale;
    let patriarca_y = cy + 220.0 * scale;

    // Cabeza con cabello canoso
    canvas.fill_circle(patriarca_x, patriarca_y - 70.0 * scale, 24.0 * scale, Color::hex("#E2DCD2"));
    canvas.stroke_circle(patriarca_x, patriarca_y - 70.0 * scale, 24.0 * scale, Color::hex("#7A7265"), 2.0 * scale);

    // Rostro noble y cejas espesas
    canvas.fill_circle(patriarca_x + 6.0 * scale, patriarca_y - 72.0 * scale, 4.0 * scale, Color::hex("#38271A")); // Ojo meditabundo
    canvas.stroke_line(patriarca_x, patriarca_y - 78.0 * scale, patriarca_x + 14.0 * scale, patriarca_y - 78.0 * scale, Color::hex("#FFFFFF"), 2.5 * scale);

    // Barba blanca bíblica y torrencial
    let mut beard_pb = tiny_skia::PathBuilder::new();
    beard_pb.move_to(patriarca_x - 14.0 * scale, patriarca_y - 65.0 * scale);
    beard_pb.cubic_to(
        patriarca_x + 15.0 * scale,
        patriarca_y - 30.0 * scale,
        patriarca_x + 25.0 * scale,
        patriarca_y - 10.0 * scale,
        patriarca_x + 10.0 * scale,
        patriarca_y + 20.0 * scale,
    );
    beard_pb.cubic_to(
        patriarca_x - 10.0 * scale,
        patriarca_y + 10.0 * scale,
        patriarca_x - 25.0 * scale,
        patriarca_y - 30.0 * scale,
        patriarca_x - 14.0 * scale,
        patriarca_y - 65.0 * scale,
    );
    beard_pb.close();
    if let Some(path) = beard_pb.finish() {
        canvas.fill_path(&path, Color::hex("#FAF7F0"));
        canvas.stroke_path(&path, Color::hex("#C4BAA3"), 2.0 * scale);
    }

    // Gran manta de franela / ruana envolviendo el cuerpo
    let mut ruana_pb = tiny_skia::PathBuilder::new();
    ruana_pb.move_to(patriarca_x + 20.0 * scale, patriarca_y - 50.0 * scale);
    ruana_pb.cubic_to(
        patriarca_x - 65.0 * scale,
        patriarca_y - 20.0 * scale,
        patriarca_x - 85.0 * scale,
        patriarca_y + 50.0 * scale,
        patriarca_x - 20.0 * scale,
        patriarca_y + 85.0 * scale,
    );
    ruana_pb.line_to(patriarca_x + 55.0 * scale, patriarca_y + 85.0 * scale);
    ruana_pb.cubic_to(
        patriarca_x + 75.0 * scale,
        patriarca_y + 40.0 * scale,
        patriarca_x + 55.0 * scale,
        patriarca_y - 30.0 * scale,
        patriarca_x + 20.0 * scale,
        patriarca_y - 50.0 * scale,
    );
    ruana_pb.close();
    if let Some(path) = ruana_pb.finish() {
        canvas.fill_path(&path, Color::hex("#544335"));
        canvas.stroke_path(&path, Color::hex("#2B1E14"), 3.0 * scale);
    }

    // Eslabones gruesos y pesados de hierro forjado atándolo al tronco
    for i in 0..11 {
        let t = i as f32 / 10.0;
        let lx = (patriarca_x - 25.0 * scale) + t * (cx + 50.0 * scale - (patriarca_x - 25.0 * scale));
        let ly = patriarca_y + (t * PI).sin() * 18.0 * scale;
        canvas.stroke_circle(lx, ly, 9.0 * scale, chain_col, 3.5 * scale);
        canvas.stroke_circle(lx, ly, 4.0 * scale, Color::hex("#1F2429"), 2.0 * scale);
    }

    // 4. Lluvia y torbellino de recuerdos (flores amarillas que caen en Macondo)
    for i in 0..32 {
        let ang = (i as f32 * 0.45) + tau * PI * 4.0;
        let dist = 80.0 * scale + (i as f32) * 11.0 * scale;
        let px = cx + ang.cos() * dist;
        let py = cy - 40.0 * scale + ang.sin() * (dist * 0.55) + ((tau * 240.0 * scale + (i as f32 * 30.0)) % (480.0 * scale)) - 240.0 * scale;
        let rot = rng.next_f32() * PI;
        let psz = 5.0 * scale;

        canvas.save();
        canvas.translate(px, py);
        canvas.rotate(rot);
        canvas.fill_circle(0.0, 0.0, psz, yellow_petal);
        canvas.fill_circle(psz * 0.6, 0.0, psz * 0.7, Color::hex("#FFA500"));
        canvas.restore();
    }
}


/// Dibuja la ascensión celestial de Remedios la Bella entre sábanas de lino blanco
pub fn draw_remedios_ascension(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    wind_tau: f32,
) {
    let sheet_col = Color::hex("#FFFFFF");
    let sheet_cream = Color::hex("#F6EFE0");
    let sheet_shadow = Color::hex("#C4BAA3");
    let sheet_edge = Color::hex("#7A6D52");
    let skin_col = Color::hex("#F5D2B3");
    let skin_shadow = Color::hex("#D9A882");
    let hair_col = Color::hex("#120D08");
    let gold_light = Color::hex("#FFD700");

    canvas.save();
    canvas.translate(cx, cy);

    // 1. Rayos celestiales de gloria divina al fondo
    for r in 0..20 {
        let a = (r as f32 / 20.0) * 2.0 * PI + wind_tau * 0.4;
        let r1 = 80.0 * scale;
        let r2 = 380.0 * scale;
        canvas.stroke_line(a.cos() * r1, a.sin() * r1 - 80.0 * scale, a.cos() * r2, a.sin() * r2 - 80.0 * scale, gold_light.with_alpha(0.20), 2.5 * scale);
    }

    // 2. Grandes sábanas de bramante blanco ondeando en la corriente ascendente
    // Capa trasera de sábanas
    for &side in &[-1.0_f32, 1.0_f32] {
        let phase = wind_tau * 8.0 + side * 2.0;
        let w1 = phase.sin() * 45.0 * scale;
        let w2 = (phase + 1.2).cos() * 55.0 * scale;

        let mut b_pb = tiny_skia::PathBuilder::new();
        b_pb.move_to(side * 30.0 * scale, 120.0 * scale);
        b_pb.cubic_to(
            side * 140.0 * scale + w1,
            80.0 * scale,
            side * 280.0 * scale + w2,
            -60.0 * scale,
            side * 340.0 * scale,
            -220.0 * scale + w1,
        );
        b_pb.cubic_to(
            side * 320.0 * scale,
            -340.0 * scale,
            side * 180.0 * scale - w1,
            -260.0 * scale,
            side * 40.0 * scale,
            -80.0 * scale,
        );
        b_pb.close();
        if let Some(path) = b_pb.finish() {
            canvas.fill_path(&path, sheet_cream);
            canvas.stroke_path(&path, sheet_shadow, 3.0 * scale);
            canvas.stroke_path(&path, sheet_edge, 1.5 * scale);
        }
    }

    // 3. Figura entera y estilizada de Remedios la Bella
    let fig_y = (wind_tau * 4.0 * PI).sin() * 15.0 * scale;

    // Vestido largo de lino blanco con pliegues en caída vertical y arrastre
    let mut tunic_pb = tiny_skia::PathBuilder::new();
    tunic_pb.move_to(0.0, -90.0 * scale + fig_y);
    tunic_pb.cubic_to(-25.0 * scale, -50.0 * scale + fig_y, -45.0 * scale, 40.0 * scale + fig_y, -60.0 * scale, 180.0 * scale + fig_y);
    tunic_pb.cubic_to(0.0, 210.0 * scale + fig_y, 40.0 * scale, 195.0 * scale + fig_y, 60.0 * scale, 180.0 * scale + fig_y);
    tunic_pb.cubic_to(45.0 * scale, 40.0 * scale + fig_y, 25.0 * scale, -50.0 * scale + fig_y, 0.0, -90.0 * scale + fig_y);
    tunic_pb.close();
    if let Some(path) = tunic_pb.finish() {
        canvas.fill_path(&path, sheet_col);
        canvas.stroke_path(&path, sheet_shadow, 2.5 * scale);
        // Pliegues verticales sombreados en el vestido
        for p in &[-25.0_f32, -8.0, 10.0, 28.0] {
            canvas.stroke_line(p * 0.4 * scale, -40.0 * scale + fig_y, p * scale, 175.0 * scale + fig_y, sheet_shadow.with_alpha(0.65), 2.0 * scale);
        }
    }

    // Pies descalzos delicados asomando bajo el borde del lino
    canvas.fill_circle(-15.0 * scale, 190.0 * scale + fig_y, 7.0 * scale, skin_col);
    canvas.fill_circle(15.0 * scale, 190.0 * scale + fig_y, 7.0 * scale, skin_col);

    // Pecho y cuello esbelto
    let mut neck_pb = tiny_skia::PathBuilder::new();
    neck_pb.move_to(-12.0 * scale, -90.0 * scale + fig_y);
    neck_pb.line_to(-8.0 * scale, -125.0 * scale + fig_y);
    neck_pb.line_to(8.0 * scale, -125.0 * scale + fig_y);
    neck_pb.line_to(12.0 * scale, -90.0 * scale + fig_y);
    neck_pb.close();
    if let Some(path) = neck_pb.finish() {
        canvas.fill_path(&path, skin_col);
        canvas.stroke_path(&path, skin_shadow, 1.2 * scale);
    }

    // Cabeza erguida y serena mirando al firmamento
    canvas.fill_circle(0.0, -145.0 * scale + fig_y, 22.0 * scale, skin_col);
    canvas.stroke_circle(0.0, -145.0 * scale + fig_y, 22.0 * scale, skin_shadow, 1.5 * scale);

    // Rasgos delicados del rostro (perfil hacia el cielo)
    canvas.fill_circle(4.0 * scale, -148.0 * scale + fig_y, 3.5 * scale, Color::hex("#4a2c11")); // Ojo sereno
    canvas.stroke_line(12.0 * scale, -140.0 * scale + fig_y, 18.0 * scale, -138.0 * scale + fig_y, Color::hex("#B84252"), 2.0 * scale); // Labios

    // Brazos alzados al cielo diciendo adiós con gracia escultórica
    // Brazo izquierdo
    let mut la_pb = tiny_skia::PathBuilder::new();
    la_pb.move_to(-18.0 * scale, -80.0 * scale + fig_y);
    la_pb.cubic_to(-60.0 * scale, -120.0 * scale + fig_y, -90.0 * scale, -170.0 * scale + fig_y, -110.0 * scale, -230.0 * scale + fig_y);
    if let Some(path) = la_pb.finish() {
        canvas.stroke_path(&path, skin_col, 9.0 * scale);
        canvas.stroke_path(&path, skin_shadow, 2.0 * scale);
        // Mano izquierda abierta despidiéndose
        canvas.fill_circle(-110.0 * scale, -230.0 * scale + fig_y, 7.0 * scale, skin_col);
    }

    // Brazo derecho
    let mut ra_pb = tiny_skia::PathBuilder::new();
    ra_pb.move_to(18.0 * scale, -80.0 * scale + fig_y);
    ra_pb.cubic_to(60.0 * scale, -120.0 * scale + fig_y, 90.0 * scale, -170.0 * scale + fig_y, 110.0 * scale, -230.0 * scale + fig_y);
    if let Some(path) = ra_pb.finish() {
        canvas.stroke_path(&path, skin_col, 9.0 * scale);
        canvas.stroke_path(&path, skin_shadow, 2.0 * scale);
        // Mano derecha
        canvas.fill_circle(110.0 * scale, -230.0 * scale + fig_y, 7.0 * scale, skin_col);
    }

    // Cabellera azabache legendaria ondeando en rizos monumentales
    for s in 0..5 {
        let mut hair_pb = tiny_skia::PathBuilder::new();
        let h_offset = s as f32 * 8.0 * scale;
        let h_wave = (wind_tau * 12.0 + s as f32 * 1.5).sin() * 22.0 * scale;
        hair_pb.move_to(-5.0 * scale, -165.0 * scale + fig_y);
        hair_pb.cubic_to(
            -50.0 * scale + h_wave,
            -150.0 * scale + h_offset + fig_y,
            -90.0 * scale - h_wave,
            -80.0 * scale + h_offset + fig_y,
            -70.0 * scale + h_wave * 0.8,
            20.0 * scale + h_offset + fig_y,
        );
        hair_pb.cubic_to(
            -50.0 * scale,
            -30.0 * scale + fig_y,
            -25.0 * scale,
            -110.0 * scale + fig_y,
            -5.0 * scale,
            -165.0 * scale + fig_y,
        );
        hair_pb.close();
        if let Some(path) = hair_pb.finish() {
            canvas.fill_path(&path, hair_col);
        }
    }

    // 4. Capa frontal de sábanas envolventes (abrazando su cintura y elevándola)
    let s_wave = (wind_tau * 10.0).sin() * 30.0 * scale;
    let mut front_sheet_pb = tiny_skia::PathBuilder::new();
    front_sheet_pb.move_to(-120.0 * scale + s_wave, -40.0 * scale + fig_y);
    front_sheet_pb.cubic_to(
        -30.0 * scale,
        -10.0 * scale + fig_y,
        50.0 * scale,
        30.0 * scale + fig_y,
        150.0 * scale - s_wave,
        10.0 * scale + fig_y,
    );
    front_sheet_pb.cubic_to(
        90.0 * scale,
        -50.0 * scale + fig_y,
        -30.0 * scale,
        -70.0 * scale + fig_y,
        -120.0 * scale + s_wave,
        -40.0 * scale + fig_y,
    );
    front_sheet_pb.close();
    if let Some(path) = front_sheet_pb.finish() {
        canvas.fill_path(&path, sheet_col.with_alpha(0.92));
        canvas.stroke_path(&path, sheet_shadow, 2.5 * scale);
    }

    // 5. Enjambre de mariposas amarillas escoltando su ascensión
    for b in 0..8 {
        let b_tau = (wind_tau * 2.0 + b as f32 * 0.125) % 1.0;
        let b_ang = b_tau * 6.0 * PI + b as f32;
        let b_rad = 140.0 * scale + b_tau * 180.0 * scale;
        let bx = b_ang.cos() * b_rad;
        let by = -80.0 * scale - b_tau * 300.0 * scale + b_ang.sin() * (b_rad * 0.4);
        let b_flp = (wind_tau * 32.0 + b as f32 * 2.0).sin();
        draw_yellow_butterfly(canvas, bx, by, 0.45 * scale, b_flp, b as u32);
    }

    canvas.restore();
}

/// Dibuja la carpa alquímica y los inventos de Melquíades
pub fn draw_melquiades_alquimia(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let iron_col = Color::hex("#2a2826");
    let pot_col = Color::hex("#4a3e36");
    let parchment_bg = Color::hex("#d8c29d");
    let ink_col = Color::hex("#3b2615");

    canvas.save();
    canvas.translate(cx, cy);

    // Bloque de hielo transparente en el centro ("el gran diamante")
    let ice_w = 120.0 * scale;
    let ice_h = 90.0 * scale;
    let mut ice_pb = tiny_skia::PathBuilder::new();
    ice_pb.move_to(-ice_w * 0.5, 30.0 * scale);
    ice_pb.line_to(-ice_w * 0.35, -ice_h * 0.5);
    ice_pb.line_to(ice_w * 0.35, -ice_h * 0.5);
    ice_pb.line_to(ice_w * 0.5, 30.0 * scale);
    ice_pb.line_to(0.0, 55.0 * scale);
    ice_pb.close();
    if let Some(path) = ice_pb.finish() {
        // Relleno frío cristalino
        canvas.fill_path(&path, Color::hex("#d2f0f8").with_alpha(0.75));
        canvas.stroke_path(&path, Color::hex("#70c8e0"), 2.2 * scale);

        // Destellos de escarcha en el hielo
        canvas.stroke_line(-ice_w * 0.1, -15.0 * scale, ice_w * 0.2, 10.0 * scale, Color::hex("#ffffff"), 1.8 * scale);
        canvas.stroke_line(-ice_w * 0.25, 5.0 * scale, 0.0, 25.0 * scale, Color::hex("#ffffff"), 1.4 * scale);
    }

    // Dos imanes descomunales en herradura que arrastran calderos
    let magnet_rot = (tau * PI * 2.0).sin() * 0.2;
    for &side in &[-1.0_f32, 1.0_f32] {
        canvas.save();
        canvas.translate(side * 170.0 * scale, -40.0 * scale);
        canvas.rotate(side * magnet_rot);

        // Herradura magnética
        let mut mag_pb = tiny_skia::PathBuilder::new();
        mag_pb.move_to(-20.0 * scale, 40.0 * scale);
        mag_pb.line_to(-20.0 * scale, -20.0 * scale);
        mag_pb.cubic_to(-20.0 * scale, -50.0 * scale, 20.0 * scale, -50.0 * scale, 20.0 * scale, -20.0 * scale);
        mag_pb.line_to(20.0 * scale, 40.0 * scale);
        if let Some(path) = mag_pb.finish() {
            canvas.stroke_path(&path, iron_col, 16.0 * scale);
            // Polos norte/sur pintados de rojo y plata
            canvas.stroke_line(-20.0 * scale, 25.0 * scale, -20.0 * scale, 40.0 * scale, Color::hex("#d91438"), 16.0 * scale);
            canvas.stroke_line(20.0 * scale, 25.0 * scale, 20.0 * scale, 40.0 * scale, Color::hex("#e0e0e0"), 16.0 * scale);
        }

        // Líneas de fuerza magnética invisibles arrastrando ollas
        let pull_x = -side * 40.0 * scale;
        canvas.fill_circle(pull_x, 60.0 * scale, 18.0 * scale, pot_col);
        canvas.stroke_circle(pull_x, 60.0 * scale, 18.0 * scale, Color::hex("#1c1815"), 1.8 * scale);
        // Clavos y herrajes volando atraídos
        canvas.stroke_line(pull_x + 10.0 * scale, 30.0 * scale, pull_x + 18.0 * scale, 15.0 * scale, iron_col, 2.0 * scale);

        canvas.restore();
    }

    // Astrolabio y pergaminos proféticos en sánscrito
    let perg_w = 110.0 * scale;
    let perg_h = 75.0 * scale;
    let perg_y = 120.0 * scale;
    let mut pg_pb = tiny_skia::PathBuilder::new();
    pg_pb.move_to(-perg_w * 0.5, perg_y);
    pg_pb.line_to(perg_w * 0.5, perg_y);
    pg_pb.line_to(perg_w * 0.5, perg_y + perg_h);
    pg_pb.line_to(-perg_w * 0.5, perg_y + perg_h);
    pg_pb.close();
    if let Some(path) = pg_pb.finish() {
        canvas.fill_path(&path, parchment_bg);
        canvas.stroke_path(&path, Color::hex("#8a7050"), 1.4 * scale);

        // Trazos de escritura sánscrita
        for r in 1..6 {
            let ry = perg_y + (r as f32) * 12.0 * scale;
            canvas.stroke_line(-perg_w * 0.4, ry, perg_w * 0.4, ry, ink_col.with_alpha(0.7), 1.2 * scale);
        }
    }

    canvas.restore();
}

/// Dibuja el gran galeón español encallado en la selva virgen
pub fn draw_galeon_selva(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
) {
    let wood_hull = Color::hex("#38271a");
    let wood_deck = Color::hex("#543e2b");
    let rope_col = Color::hex("#8a765a");
    let sail_rag = Color::hex("#d4cebe");
    let jungle_green = Color::hex("#2d5c38");
    let orchid_purple = Color::hex("#b84da0");

    canvas.save();
    canvas.translate(cx, cy);

    // Casco curvo del galeón español del siglo XVI
    let mut hull_pb = tiny_skia::PathBuilder::new();
    hull_pb.move_to(-160.0 * scale, -20.0 * scale); // Popa alta
    hull_pb.cubic_to(-120.0 * scale, 60.0 * scale, 60.0 * scale, 75.0 * scale, 170.0 * scale, 10.0 * scale); // Quilla y proa
    hull_pb.cubic_to(140.0 * scale, -10.0 * scale, 110.0 * scale, -15.0 * scale, 80.0 * scale, -10.0 * scale);
    hull_pb.line_to(-130.0 * scale, -10.0 * scale);
    hull_pb.close();

    if let Some(path) = hull_pb.finish() {
        canvas.fill_path(&path, wood_hull);
        canvas.stroke_path(&path, Color::hex("#1c140d"), 2.6 * scale);

        // Tablones de la borda
        canvas.stroke_line(-150.0 * scale, 5.0 * scale, 140.0 * scale, 22.0 * scale, wood_deck, 2.0 * scale);
        canvas.stroke_line(-130.0 * scale, 28.0 * scale, 110.0 * scale, 40.0 * scale, wood_deck, 2.0 * scale);
    }

    // Castillo de popa elevado con ventanas de celosía
    let mut stern_pb = tiny_skia::PathBuilder::new();
    stern_pb.move_to(-160.0 * scale, -20.0 * scale);
    stern_pb.line_to(-160.0 * scale, -70.0 * scale);
    stern_pb.line_to(-105.0 * scale, -60.0 * scale);
    stern_pb.line_to(-105.0 * scale, -10.0 * scale);
    stern_pb.close();
    if let Some(path) = stern_pb.finish() {
        canvas.fill_path(&path, wood_hull);
        canvas.stroke_circle(-130.0 * scale, -42.0 * scale, 7.0 * scale, Color::hex("#ffd040"), 1.5 * scale);
    }

    // Tres mástiles con aparejos y jirones de velas devoradas por la selva
    let masts = [
        (-80.0 * scale, -160.0 * scale), // Trinquete
        (0.0, -210.0 * scale),           // Mayor
        (85.0 * scale, -145.0 * scale),  // Mesana
    ];

    for &(mx, my) in &masts {
        // Palo mayor
        canvas.stroke_line(mx, 0.0, mx, my, wood_hull, 4.8 * scale);
        // Vergas horizontales
        canvas.stroke_line(mx - 40.0 * scale, my + 45.0 * scale, mx + 40.0 * scale, my + 45.0 * scale, wood_deck, 2.4 * scale);

        // Jirones de velas antiguas
        let mut sail_pb = tiny_skia::PathBuilder::new();
        sail_pb.move_to(mx - 35.0 * scale, my + 47.0 * scale);
        sail_pb.cubic_to(mx - 20.0 * scale, my + 85.0 * scale, mx + 15.0 * scale, my + 95.0 * scale, mx + 35.0 * scale, my + 47.0 * scale);
        sail_pb.close();
        if let Some(path) = sail_pb.finish() {
            canvas.fill_path(&path, sail_rag.with_alpha(0.65));
        }

        // Jarcias / cabos
        canvas.stroke_line(mx, my, mx - 50.0 * scale, 0.0, rope_col, 1.0 * scale);
        canvas.stroke_line(mx, my, mx + 50.0 * scale, 0.0, rope_col, 1.0 * scale);
    }

    // Lianas, helechos y orquídeas que brotan del maderamen del casco
    for i in 0..14 {
        let lx = -130.0 * scale + (i as f32) * 20.0 * scale;
        let ly = 15.0 * scale + (i % 3) as f32 * 12.0 * scale;
        // Hojas de selva trepadora
        canvas.stroke_circle(lx, ly, 10.0 * scale, jungle_green, 2.5 * scale);
        if i % 3 == 0 {
            // Orquídeas silvestres moradas
            canvas.fill_circle(lx + 4.0 * scale, ly - 5.0 * scale, 5.0 * scale, orchid_purple);
            canvas.fill_circle(lx + 4.0 * scale, ly - 5.0 * scale, 2.0 * scale, Color::hex("#ffd040"));
        }
    }

    canvas.restore();
}

/// Dibuja el tren amarillo de la compañía bananera entrando a Macondo
pub fn draw_tren_amarillo(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    canvas.save();
    canvas.translate(cx, cy);

    let yellow_train = Color::hex("#ffd000");
    let black_iron = Color::hex("#222222");
    let dark_metal = Color::hex("#3a3a3a");
    let steam_white = Color::hex("#fafaf9").with_alpha(0.65);

    // Caldera / cuerpo de la locomotora
    canvas.fill_rect(-140.0 * scale, -45.0 * scale, 180.0 * scale, 55.0 * scale, black_iron);
    // Cabina del maquinista (amarillo bananero)
    canvas.fill_rect(40.0 * scale, -85.0 * scale, 90.0 * scale, 95.0 * scale, yellow_train);
    canvas.stroke_rect(40.0 * scale, -85.0 * scale, 90.0 * scale, 95.0 * scale, black_iron, 3.0 * scale);
    // Ventana de la cabina
    canvas.fill_rect(60.0 * scale, -70.0 * scale, 45.0 * scale, 35.0 * scale, Color::hex("#93c5fd"));

    // Chimenea frontal
    canvas.fill_rect(-115.0 * scale, -95.0 * scale, 30.0 * scale, 50.0 * scale, black_iron);
    // Faro frontal dorado
    canvas.fill_circle(-140.0 * scale, -15.0 * scale, 16.0 * scale, Color::hex("#ffeaa7"));
    canvas.fill_circle(-140.0 * scale, -15.0 * scale, 35.0 * scale, Color::hex("#ffeaa7").with_alpha(0.35));

    // Nubes de vapor que salen de la chimenea
    for p in 0..5 {
        let p_tau = (tau * 4.0 + p as f32 * 0.2) % 1.0;
        let px = -100.0 * scale + p_tau * 120.0 * scale;
        let py = -105.0 * scale - p_tau * 60.0 * scale;
        let p_rad = (15.0 + p_tau * 25.0) * scale;
        canvas.fill_circle(px, py, p_rad, steam_white);
    }

    // Ruedas de hierro
    for i in 0..4 {
        let wx = -105.0 * scale + (i as f32) * 65.0 * scale;
        let wy = 15.0 * scale;
        canvas.fill_circle(wx, wy, 24.0 * scale, dark_metal);
        canvas.stroke_circle(wx, wy, 24.0 * scale, black_iron, 4.0 * scale);
        // Radios de la rueda
        let w_rot = tau * 16.0 * PI;
        canvas.stroke_line(wx - 20.0 * scale * w_rot.cos(), wy - 20.0 * scale * w_rot.sin(),
                           wx + 20.0 * scale * w_rot.cos(), wy + 20.0 * scale * w_rot.sin(), black_iron, 2.5 * scale);
    }

    // Biela de tracción horizontal
    let biela_y = 15.0 * scale + (tau * 16.0 * PI).sin() * 8.0 * scale;
    canvas.stroke_line(-110.0 * scale, biela_y, 90.0 * scale, biela_y, Color::hex("#e17055"), 5.0 * scale);

    canvas.restore();
}

/// Dibuja la lluvia de flores amarillas que cayó sobre Macondo
pub fn draw_lluvia_flores(
    canvas: &mut Canvas,
    w: f32,
    h: f32,
    tau: f32,
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    let flower_gold = Color::hex("#ffd700");
    let flower_amber = Color::hex("#f39c12");

    let count = 48;
    for i in 0..count {
        let rx = rng.next_f32() * w;
        let speed = 0.8 + rng.next_f32() * 1.5;
        let phase = rng.next_f32();
        let t = (tau * speed + phase) % 1.0;
        let fy = t * (h + 60.0) - 30.0;
        let fx = rx + (t * 6.0 * PI + (i as f32)).sin() * 35.0;

        let col = if i % 2 == 0 { flower_gold } else { flower_amber };
        // Florecita de 4 pétalos
        canvas.fill_circle(fx - 4.0, fy, 4.0, col);
        canvas.fill_circle(fx + 4.0, fy, 4.0, col);
        canvas.fill_circle(fx, fy - 4.0, 4.0, col);
        canvas.fill_circle(fx, fy + 4.0, 4.0, col);
        canvas.fill_circle(fx, fy, 2.5, Color::hex("#e67e22"));
    }
}

/// Dibuja la mariposa amarilla con la envergadura y precisión G5 (estilo Kevin Ngo)
pub fn draw_g5_yellow_butterfly(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    wing_flap_angle: f32,
    blueprint_mode: bool,
    _seed: u32,
) {
    let flap_cos = wing_flap_angle.cos().abs().max(0.12);
    let flap_lift_y = -wing_flap_angle.sin() * 25.0 * scale;

    let wing_fill = if blueprint_mode {
        Color::hex("#102a4a").with_alpha(0.85)
    } else {
        Color::hex("#FFD200") // Amarillo vivo radiante
    };
    let border_color = if blueprint_mode {
        Color::hex("#40a0df")
    } else {
        Color::hex("#1C150A") // Negro terciopelo
    };
    let vein_color = if blueprint_mode {
        Color::hex("#80d0ff").with_alpha(0.95)
    } else {
        Color::hex("#A66D03") // Ámbar dorado
    };
    let spot_color = if blueprint_mode {
        Color::hex("#ffffff")
    } else {
        Color::hex("#FFFFFF")
    };

    canvas.save();
    canvas.translate(cx, cy);

    for &side in &[-1.0_f32, 1.0_f32] {
        canvas.save();
        canvas.scale(side * flap_cos, 1.0);
        canvas.translate(0.0, flap_lift_y);

        // 1. ALA POSTERIOR (Hindwing)
        let mut hw_pb = tiny_skia::PathBuilder::new();
        hw_pb.move_to(8.0 * scale, 12.0 * scale);
        hw_pb.cubic_to(55.0 * scale, 16.0 * scale, 115.0 * scale, 48.0 * scale, 140.0 * scale, 95.0 * scale);
        hw_pb.cubic_to(155.0 * scale, 140.0 * scale, 120.0 * scale, 195.0 * scale, 65.0 * scale, 210.0 * scale);
        hw_pb.cubic_to(20.0 * scale, 215.0 * scale, 8.0 * scale, 180.0 * scale, 5.0 * scale, 120.0 * scale);
        hw_pb.close();

        if let Some(hw_path) = hw_pb.finish() {
            canvas.fill_path(&hw_path, wing_fill);
            let border_w = if blueprint_mode { 1.8 * scale } else { 16.0 * scale };
            canvas.stroke_path(&hw_path, border_color, border_w);

            // Venas del ala posterior
            let hind_veins = [
                (8.0 * scale, 35.0 * scale, 120.0 * scale, 70.0 * scale),
                (8.0 * scale, 45.0 * scale, 135.0 * scale, 115.0 * scale),
                (8.0 * scale, 55.0 * scale, 110.0 * scale, 165.0 * scale),
                (8.0 * scale, 65.0 * scale, 60.0 * scale, 195.0 * scale),
            ];
            for (vx1, vy1, vx2, vy2) in hind_veins {
                canvas.stroke_line(vx1, vy1, vx2, vy2, vein_color, 2.4 * scale);
            }

            // Puntos blancos marginales
            if !blueprint_mode {
                let spots = [(135.0 * scale, 95.0 * scale), (142.0 * scale, 130.0 * scale), (120.0 * scale, 170.0 * scale), (80.0 * scale, 195.0 * scale)];
                for (sx, sy) in spots {
                    canvas.fill_circle(sx, sy, 3.2 * scale, spot_color);
                }
            }
        }

        // 2. ALA ANTERIOR (Forewing G5)
        let apex_x = 240.0 * scale;
        let apex_y = -210.0 * scale;
        let tornus_x = 190.0 * scale;
        let tornus_y = -10.0 * scale;

        let mut fw_pb = tiny_skia::PathBuilder::new();
        fw_pb.move_to(10.0 * scale, -18.0 * scale);
        fw_pb.cubic_to(60.0 * scale, -130.0 * scale, 150.0 * scale, apex_y - 12.0 * scale, apex_x, apex_y);
        fw_pb.cubic_to(apex_x + 12.0 * scale, apex_y + 85.0 * scale, tornus_x + 32.0 * scale, tornus_y - 65.0 * scale, tornus_x, tornus_y);
        fw_pb.cubic_to(tornus_x - 65.0 * scale, tornus_y + 6.0 * scale, 45.0 * scale, -2.0 * scale, 10.0 * scale, -18.0 * scale);
        fw_pb.close();

        if let Some(fw_path) = fw_pb.finish() {
            canvas.fill_path(&fw_path, wing_fill);
            let border_w = if blueprint_mode { 2.0 * scale } else { 22.0 * scale };
            canvas.stroke_path(&fw_path, border_color, border_w);

            // Ápice negro triangular característico
            if !blueprint_mode {
                let mut apex_pb = tiny_skia::PathBuilder::new();
                apex_pb.move_to(apex_x, apex_y);
                apex_pb.line_to(apex_x - 65.0 * scale, apex_y + 30.0 * scale);
                apex_pb.cubic_to(apex_x - 45.0 * scale, apex_y + 60.0 * scale, apex_x - 12.0 * scale, apex_y + 100.0 * scale, apex_x - 8.0 * scale, apex_y + 110.0 * scale);
                apex_pb.close();
                if let Some(apex_path) = apex_pb.finish() {
                    canvas.fill_path(&apex_path, border_color);
                }
            }

            // Celda discal cerrada
            let mut disc_pb = tiny_skia::PathBuilder::new();
            disc_pb.move_to(10.0 * scale, -18.0 * scale);
            disc_pb.cubic_to(45.0 * scale, -65.0 * scale, 85.0 * scale, -95.0 * scale, 125.0 * scale, -95.0 * scale);
            disc_pb.cubic_to(105.0 * scale, -55.0 * scale, 55.0 * scale, -8.0 * scale, 10.0 * scale, -18.0 * scale);
            if let Some(disc_path) = disc_pb.finish() {
                canvas.stroke_path(&disc_path, vein_color, 3.4 * scale);
            }

            // Venas radiales
            let veins = [
                (125.0 * scale, -95.0 * scale, apex_x - 25.0 * scale, apex_y + 40.0 * scale),
                (125.0 * scale, -95.0 * scale, apex_x - 12.0 * scale, apex_y + 80.0 * scale),
                (125.0 * scale, -95.0 * scale, apex_x - 8.0 * scale, apex_y + 120.0 * scale),
                (105.0 * scale, -55.0 * scale, tornus_x + 25.0 * scale, tornus_y - 75.0 * scale),
                (55.0 * scale, -8.0 * scale, tornus_x + 10.0 * scale, tornus_y - 35.0 * scale),
            ];
            for (vx1, vy1, vx2, vy2) in veins {
                canvas.stroke_line(vx1, vy1, vx2, vy2, vein_color, 2.6 * scale);
            }

            // Puntos blancos marginales del ápice
            if !blueprint_mode {
                let fw_spots = [
                    (apex_x - 25.0 * scale, apex_y + 20.0 * scale),
                    (apex_x - 12.0 * scale, apex_y + 45.0 * scale),
                    (apex_x - 5.0 * scale, apex_y + 80.0 * scale),
                    (apex_x - 2.0 * scale, apex_y + 115.0 * scale),
                    (tornus_x + 22.0 * scale, tornus_y - 65.0 * scale),
                ];
                for (sx, sy) in fw_spots {
                    canvas.fill_circle(sx, sy, 3.5 * scale, spot_color);
                }
            }
        }

        canvas.restore();
    }

    // 3. CUERPO (Tórax aterciopelado y antenas)
    let body_col = if blueprint_mode { Color::hex("#40a0df") } else { Color::hex("#1A1408") };
    canvas.fill_circle(0.0, -10.0 * scale, 7.0 * scale, body_col);
    canvas.stroke_line(0.0, -8.0 * scale, 0.0, 48.0 * scale, body_col, 8.0 * scale);

    for &side in &[-1.0_f32, 1.0_f32] {
        let mut ant_pb = tiny_skia::PathBuilder::new();
        ant_pb.move_to(side * 3.0 * scale, -15.0 * scale);
        ant_pb.cubic_to(side * 12.0 * scale, -40.0 * scale, side * 30.0 * scale, -60.0 * scale, side * 45.0 * scale, -75.0 * scale);
        if let Some(path) = ant_pb.finish() {
            canvas.stroke_path(&path, body_col, 2.0 * scale);
            canvas.fill_circle(side * 45.0 * scale, -75.0 * scale, 3.5 * scale, body_col);
        }
    }

    canvas.restore();
}

/// Dibuja las trompetas barrocas de Gabriel anunciando a Macondo
pub fn draw_trompetas_heraldicas(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
    blueprint_mode: bool,
) {
    let gold_shine = Color::hex("#FFFDE6");
    let gold_bright = Color::hex("#FFD700");
    let gold_mid = Color::hex("#D4AF37");
    let gold_dark = Color::hex("#7A5D12");
    let banner_red = if blueprint_mode { Color::hex("#8B1E28") } else { Color::hex("#A81C26") };
    let wave_col = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#FFD700") };

    canvas.save();
    canvas.translate(cx, cy);

    // 1. Rayos solares de fanfarria
    for r in 0..16 {
        let ang = (r as f32 / 16.0) * 2.0 * PI + tau * 0.3;
        canvas.stroke_line(0.0, 0.0, ang.cos() * 420.0 * scale, ang.sin() * 420.0 * scale, gold_bright.with_alpha(0.18), 2.5 * scale);
    }

    // 2. Dos trompetas heráldicas cruzadas en V apuntando al cielo (izquierda y derecha)
    for &side in &[-1.0_f32, 1.0_f32] {
        canvas.save();
        // Espejo simétrico para que una apunte a la izquierda-arriba y otra a la derecha-arriba
        canvas.scale(side, 1.0);
        canvas.rotate(-0.55); // ~32 grados de inclinación hacia el cielo

        let horn_pulse = 1.0 + (tau * 16.0 * PI).sin().abs() * 0.04;
        canvas.scale(horn_pulse, horn_pulse);

        // A. Boquilla cónica de latón torneado
        let mut mp_pb = tiny_skia::PathBuilder::new();
        mp_pb.move_to(-210.0 * scale, -6.0 * scale);
        mp_pb.line_to(-190.0 * scale, -3.0 * scale);
        mp_pb.line_to(-190.0 * scale, 3.0 * scale);
        mp_pb.line_to(-210.0 * scale, 6.0 * scale);
        mp_pb.close();
        if let Some(path) = mp_pb.finish() {
            canvas.fill_path(&path, gold_mid);
            canvas.stroke_path(&path, gold_dark, 1.5 * scale);
        }
        canvas.fill_circle(-210.0 * scale, 0.0, 6.0 * scale, gold_bright);

        // B. Tubo principal cilíndrico (leadpipe)
        canvas.stroke_line(-190.0 * scale, 0.0, 160.0 * scale, 0.0, gold_dark, 14.0 * scale);
        canvas.stroke_line(-190.0 * scale, -1.0 * scale, 160.0 * scale, -1.0 * scale, gold_bright, 10.0 * scale);
        canvas.stroke_line(-190.0 * scale, -2.5 * scale, 160.0 * scale, -2.5 * scale, gold_shine, 3.0 * scale);

        // C. Curva de afinación inferior (tuning slide loop)
        let mut loop_pb = tiny_skia::PathBuilder::new();
        loop_pb.move_to(-80.0 * scale, 0.0);
        loop_pb.cubic_to(-80.0 * scale, 45.0 * scale, 40.0 * scale, 45.0 * scale, 40.0 * scale, 0.0);
        if let Some(path) = loop_pb.finish() {
            canvas.stroke_path(&path, gold_dark, 8.0 * scale);
            canvas.stroke_path(&path, gold_bright, 5.0 * scale);
        }

        // D. Bloque de 3 pistones / válvulas verticales
        for p in 0..3 {
            let px = -35.0 * scale + (p as f32) * 28.0 * scale;
            // Vástago y botón de nácar superior
            canvas.stroke_line(px, 0.0, px, -36.0 * scale, gold_dark, 5.0 * scale);
            canvas.stroke_line(px, 0.0, px, -36.0 * scale, gold_bright, 3.0 * scale);
            canvas.fill_circle(px, -38.0 * scale, 7.0 * scale, Color::hex("#FFF8F0"));
            canvas.stroke_circle(px, -38.0 * scale, 7.0 * scale, gold_dark, 1.5 * scale);

            // Carcasa cilíndrica del pistón
            canvas.stroke_line(px, -15.0 * scale, px, 20.0 * scale, gold_dark, 11.0 * scale);
            canvas.stroke_line(px, -15.0 * scale, px, 20.0 * scale, gold_bright, 8.0 * scale);
        }

        // E. Campana ancha y acampanada con curvatura acústica hiperbólica auténtica
        let mut bell_pb = tiny_skia::PathBuilder::new();
        bell_pb.move_to(120.0 * scale, -6.5 * scale);
        bell_pb.cubic_to(180.0 * scale, -12.0 * scale, 235.0 * scale, -38.0 * scale, 290.0 * scale, -84.0 * scale);
        bell_pb.line_to(290.0 * scale, 84.0 * scale);
        bell_pb.cubic_to(235.0 * scale, 38.0 * scale, 180.0 * scale, 12.0 * scale, 120.0 * scale, 6.5 * scale);
        bell_pb.close();

        if let Some(path) = bell_pb.finish() {
            canvas.fill_path(&path, gold_mid);
            canvas.stroke_path_fine(&path, gold_dark, 2.4 * scale);

            // Reflejos longitudinales bruñidos sobre el latón pulido
            let mut shine_pb = tiny_skia::PathBuilder::new();
            shine_pb.move_to(130.0 * scale, -2.5 * scale);
            shine_pb.cubic_to(190.0 * scale, -7.0 * scale, 240.0 * scale, -22.0 * scale, 286.0 * scale, -52.0 * scale);
            if let Some(spath) = shine_pb.finish() {
                canvas.stroke_path_fine(&spath, gold_shine, 3.2 * scale);
            }

            // Anillos ornamentales barrocos grabados en la campana (filigrana fina)
            for ring in &[160.0_f32, 195.0, 235.0, 265.0] {
                let rx = ring * scale;
                let r_flare = 8.0 + (rx / scale - 120.0) * 0.45;
                let mut ring_pb = tiny_skia::PathBuilder::new();
                ring_pb.move_to(rx, -r_flare * scale);
                ring_pb.line_to(rx, r_flare * scale);
                if let Some(rpath) = ring_pb.finish() {
                    canvas.stroke_path_fine(&rpath, gold_dark, 1.8 * scale);
                    canvas.stroke_path_fine(&rpath, gold_shine, 0.9 * scale);
                }
            }
        }

        // Boca de la campana en perspectiva lateral auténtica: elipse vertical esbelta con labio reforzado de latón
        let bx = 290.0 * scale;
        let rx = 18.0 * scale;
        let ry = 84.0 * scale;

        // Labio exterior biselado
        let mut rim_outer = tiny_skia::PathBuilder::new();
        rim_outer.move_to(bx, -ry);
        rim_outer.cubic_to(bx + rx * 1.33, -ry, bx + rx * 1.33, ry, bx, ry);
        rim_outer.cubic_to(bx - rx * 1.33, ry, bx - rx * 1.33, -ry, bx, -ry);
        rim_outer.close();
        if let Some(path) = rim_outer.finish() {
            canvas.fill_path(&path, gold_mid);
            canvas.stroke_path_fine(&path, gold_shine, 3.0 * scale);
            canvas.stroke_path_fine(&path, gold_dark, 1.2 * scale);
        }

        // Garganta interior oscura de la campana (profundidad acústica)
        let irx = 12.0 * scale;
        let iry = 74.0 * scale;
        let mut rim_inner = tiny_skia::PathBuilder::new();
        rim_inner.move_to(bx - 3.0 * scale, -iry);
        rim_inner.cubic_to(bx - 3.0 * scale + irx * 1.33, -iry, bx - 3.0 * scale + irx * 1.33, iry, bx - 3.0 * scale, iry);
        rim_inner.cubic_to(bx - 3.0 * scale - irx * 1.33, iry, bx - 3.0 * scale - irx * 1.33, -iry, bx - 3.0 * scale, -iry);
        rim_inner.close();
        if let Some(path) = rim_inner.finish() {
            canvas.fill_path(&path, Color::hex("#241706"));
            canvas.stroke_path_fine(&path, gold_dark, 1.5 * scale);
        }

        // F. Estandarte heráldico colgante (con flecos dorados y corte cola de golondrina)
        let flag_wave = (tau * 10.0 * PI).sin() * 6.0 * scale;
        let mut flag_pb = tiny_skia::PathBuilder::new();
        flag_pb.move_to(-70.0 * scale, 8.0 * scale);
        flag_pb.line_to(70.0 * scale, 8.0 * scale);
        flag_pb.line_to(65.0 * scale + flag_wave, 130.0 * scale);
        flag_pb.line_to(0.0, 105.0 * scale);
        flag_pb.line_to(-65.0 * scale + flag_wave, 130.0 * scale);
        flag_pb.close();

        if let Some(path) = flag_pb.finish() {
            canvas.fill_path(&path, banner_red);
            canvas.stroke_path(&path, gold_bright, 3.0 * scale);
            // Flecos de oro
            for i in 0..10 {
                let fx = -60.0 * scale + (i as f32) * 12.0 * scale;
                let fy = 115.0 * scale;
                canvas.stroke_line(fx, fy, fx, fy + 12.0 * scale, gold_bright, 1.8 * scale);
            }
            // Emblema de la mariposa dorada en el estandarte
            canvas.fill_circle(0.0, 55.0 * scale, 14.0 * scale, gold_bright);
        }

        // Cordones y borlas de oro que sujetan el estandarte
        canvas.stroke_line(-70.0 * scale, 0.0, -70.0 * scale, 8.0 * scale, gold_bright, 3.0 * scale);
        canvas.stroke_line(70.0 * scale, 0.0, 70.0 * scale, 8.0 * scale, gold_bright, 3.0 * scale);

        // G. Ondas sonoras emanando de la campana
        for w in 1..=4 {
            let w_prog = (tau * 4.0 + (w as f32) * 0.25) % 1.0;
            let wx = 280.0 * scale + w_prog * 180.0 * scale;
            let wr = 60.0 * scale + w_prog * 100.0 * scale;
            let alpha = (1.0 - w_prog) * 0.85;

            let mut w_pb = tiny_skia::PathBuilder::new();
            w_pb.move_to(wx, -wr);
            w_pb.cubic_to(wx + 30.0 * scale, -wr * 0.5, wx + 30.0 * scale, wr * 0.5, wx, wr);
            if let Some(wpath) = w_pb.finish() {
                canvas.stroke_path(&wpath, wave_col.with_alpha(alpha), 3.0 * scale);
            }
        }

        // H. Notas musicales doradas flotando desde la campana al aire
        for n in 0..3 {
            let n_prog = (tau * 2.5 + n as f32 * 0.33) % 1.0;
            let nx = 310.0 * scale + n_prog * 160.0 * scale;
            let ny = -40.0 * scale - n_prog * 80.0 * scale + (n as f32 * 25.0 * scale);
            canvas.fill_circle(nx, ny, 7.0 * scale, gold_bright);
            canvas.stroke_line(nx + 6.0 * scale, ny, nx + 6.0 * scale, ny - 22.0 * scale, gold_bright, 2.5 * scale);
            canvas.stroke_line(nx + 6.0 * scale, ny - 22.0 * scale, nx + 16.0 * scale, ny - 16.0 * scale, gold_bright, 2.5 * scale);
        }

        canvas.restore();
    }

    canvas.restore();
}

/// Dibuja el taller de platería del Coronel Aureliano Buendía y el pescadito de oro bajo lupa
pub fn draw_aureliano_taller_plateria(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let workbench_col = Color::hex("#182B55");
    let gold_col = Color::hex("#FFD700");
    let ruby_eye = Color::hex("#FF1A4B");

    canvas.save();
    canvas.translate(cx, cy);

    // Banco de trabajo esquemático del platero
    canvas.stroke_line(-280.0 * scale, 140.0 * scale, 280.0 * scale, 140.0 * scale, workbench_col, 3.0 * scale);
    // Crisol de fundición a la izquierda
    canvas.stroke_circle(-180.0 * scale, 80.0 * scale, 35.0 * scale, Color::hex("#C8C1EF"), 2.0 * scale);
    canvas.fill_circle(-180.0 * scale, 80.0 * scale, 18.0 * scale, Color::hex("#FF7640")); // Fuego del crisol

    // Pesa de balanza de orfebre
    canvas.stroke_line(180.0 * scale, 40.0 * scale, 180.0 * scale, 140.0 * scale, Color::hex("#C8C1EF"), 2.0 * scale);
    canvas.stroke_line(140.0 * scale, 40.0 * scale, 220.0 * scale, 40.0 * scale, Color::hex("#C8C1EF"), 2.0 * scale);

    // Lupa de relojero central que amplía el pez de oro a 2.5X
    let loupe_r = 150.0 * scale;
    canvas.fill_circle(0.0, 0.0, loupe_r, Color::hex("#0D1D45"));
    canvas.stroke_circle(0.0, 0.0, loupe_r, Color::hex("#C8A47A"), 6.0 * scale);
    canvas.stroke_circle(0.0, 0.0, loupe_r + 6.0 * scale, Color::hex("#3A4A86"), 1.5 * scale);

    // Mango metálico de la lupa
    canvas.stroke_line(loupe_r * 0.707, loupe_r * 0.707, loupe_r * 1.4, loupe_r * 1.4, Color::hex("#C8A47A"), 12.0 * scale);

    // El pescadito de oro articulado dentro de la lente
    let fish_scale = scale * 1.35;
    let wiggle = (tau * 6.0 * PI).sin() * 8.0 * fish_scale;

    // Cuerpo con escamas articuladas
    for s in (0..7).rev() {
        let sx = -80.0 * fish_scale + (s as f32) * 24.0 * fish_scale;
        let sy = if s > 3 { (s as f32 * 0.4 + tau * 4.0).sin() * wiggle } else { 0.0 };
        let sr = (32.0 - (s as f32 - 3.0).abs() * 5.0) * fish_scale;
        canvas.fill_circle(sx, sy, sr, gold_col);
        canvas.stroke_circle(sx, sy, sr, Color::hex("#B8860B"), 2.2 * fish_scale);
    }

    // Ojo de rubí reluciente
    canvas.fill_circle(-65.0 * fish_scale, -8.0 * fish_scale, 6.0 * fish_scale, ruby_eye);
    canvas.fill_circle(-67.0 * fish_scale, -10.0 * fish_scale, 2.0 * fish_scale, Color::hex("#FFFFFF"));

    // Cola bífida móvil
    let mut tail_pb = tiny_skia::PathBuilder::new();
    tail_pb.move_to(80.0 * fish_scale, wiggle);
    tail_pb.line_to(120.0 * fish_scale, wiggle - 30.0 * fish_scale);
    tail_pb.line_to(105.0 * fish_scale, wiggle);
    tail_pb.line_to(120.0 * fish_scale, wiggle + 30.0 * fish_scale);
    tail_pb.close();
    if let Some(path) = tail_pb.finish() {
        canvas.fill_path(&path, gold_col);
        canvas.stroke_path(&path, Color::hex("#B8860B"), 2.0 * fish_scale);
    }

    // Calibrador de cota técnica: "<--- 42.0 mm --->"
    canvas.stroke_line(-120.0 * scale, -110.0 * scale, 120.0 * scale, -110.0 * scale, Color::hex("#C8C1EF"), 1.2);
    canvas.stroke_line(-120.0 * scale, -118.0 * scale, -120.0 * scale, -102.0 * scale, Color::hex("#C8C1EF"), 1.2);
    canvas.stroke_line(120.0 * scale, -118.0 * scale, 120.0 * scale, -102.0 * scale, Color::hex("#C8C1EF"), 1.2);

    canvas.restore();
}

/// Dibuja el laboratorio alquímico de Melquíades (imanes gigantes y bloque de hielo)
pub fn draw_melquiades_alquimia_blueprint(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let navy_line = Color::hex("#80D0FF");
    let mag_field = Color::hex("#FF7640");
    let ice_col = Color::hex("#E8F8FF");

    canvas.save();
    canvas.translate(cx, cy);

    // Dos grandes imanes herradura a los costados con líneas de flujo magnético
    for &side in &[-1.0_f32, 1.0_f32] {
        let mx = side * 220.0 * scale;
        // Cuerpo del imán herradura
        let mut magnet_pb = tiny_skia::PathBuilder::new();
        magnet_pb.move_to(mx - side * 40.0 * scale, -80.0 * scale);
        magnet_pb.cubic_to(mx - side * 110.0 * scale, -50.0 * scale, mx - side * 110.0 * scale, 50.0 * scale, mx - side * 40.0 * scale, 80.0 * scale);
        magnet_pb.line_to(mx - side * 15.0 * scale, 80.0 * scale);
        magnet_pb.cubic_to(mx - side * 75.0 * scale, 40.0 * scale, mx - side * 75.0 * scale, -40.0 * scale, mx - side * 15.0 * scale, -80.0 * scale);
        magnet_pb.close();
        if let Some(path) = magnet_pb.finish() {
            canvas.fill_path(&path, Color::hex("#182B55"));
            canvas.stroke_path(&path, navy_line, 2.5 * scale);
        }

        // Polos rojo y azul
        canvas.fill_rect(mx - side * 40.0 * scale, -80.0 * scale, side * 25.0 * scale, 25.0 * scale, Color::hex("#FF3D98"));
        canvas.fill_rect(mx - side * 40.0 * scale, 55.0 * scale, side * 25.0 * scale, 25.0 * scale, Color::hex("#40A0DF"));

        // Líneas de campo magnético que arrastran clavos
        for l in 0..4 {
            let l_r = 90.0 * scale + (l as f32) * 35.0 * scale;
            let a_phase = tau * 4.0 + (l as f32);
            canvas.stroke_circle(mx, 0.0, l_r, mag_field.with_alpha(0.35), 1.2);
            // Clavos metálicos atraídos volando
            let nx = mx + (a_phase * PI * 0.5).cos() * l_r;
            let ny = (a_phase * PI * 0.5).sin() * l_r * 0.5;
            canvas.stroke_line(nx - 8.0, ny - 4.0, nx + 8.0, ny + 4.0, Color::hex("#FFFFFF"), 2.0);
        }
    }

    // En el centro: El bloque de hielo gigantesco ("El diamante más grande del mundo")
    let ice_w = 170.0 * scale;
    let ice_h = 130.0 * scale;
    canvas.fill_rect(-ice_w * 0.5, -ice_h * 0.5, ice_w, ice_h, ice_col.with_alpha(0.85));
    canvas.stroke_rect(-ice_w * 0.5, -ice_h * 0.5, ice_w, ice_h, Color::hex("#FFFFFF"), 2.8 * scale);

    // Facetas y fracturas internas del hielo cristalino
    canvas.stroke_line(-ice_w * 0.5, -ice_h * 0.5, ice_w * 0.2, ice_h * 0.1, Color::hex("#A8D8F0"), 2.0 * scale);
    canvas.stroke_line(ice_w * 0.2, ice_h * 0.1, ice_w * 0.5, -ice_h * 0.3, Color::hex("#A8D8F0"), 1.8 * scale);
    canvas.stroke_line(ice_w * 0.2, ice_h * 0.1, -ice_w * 0.1, ice_h * 0.5, Color::hex("#A8D8F0"), 1.8 * scale);

    // Vapores de frío glacial que ascienden
    for i in 0..8 {
        let vx = -ice_w * 0.4 + (i as f32) * (ice_w * 0.11);
        let vy = -ice_h * 0.5 - ((tau * 60.0 + i as f32 * 15.0) % 70.0) * scale;
        canvas.stroke_line(vx, vy, vx + (i as f32 * 1.5).sin() * 8.0, vy - 15.0 * scale, Color::hex("#FFFFFF").with_alpha(0.65), 2.0 * scale);
    }

    // Figura augusta de Melquíades el gitano alquimista sobre el hielo
    let melq_y = -ice_h * 0.5 - 75.0 * scale;

    // Capa de terciopelo morado oscuro con ribete de oro
    let mut cloak_pb = tiny_skia::PathBuilder::new();
    cloak_pb.move_to(-40.0 * scale, melq_y - 25.0 * scale);
    cloak_pb.line_to(-80.0 * scale, melq_y + 65.0 * scale);
    cloak_pb.line_to(80.0 * scale, melq_y + 65.0 * scale);
    cloak_pb.line_to(40.0 * scale, melq_y - 25.0 * scale);
    cloak_pb.close();
    if let Some(path) = cloak_pb.finish() {
        canvas.fill_path(&path, Color::hex("#2A1838"));
        canvas.stroke_path(&path, Color::hex("#FFD700"), 2.2 * scale);
    }

    // Rostro y cuello
    canvas.fill_circle(0.0, melq_y - 35.0 * scale, 22.0 * scale, Color::hex("#E8BA92"));
    canvas.stroke_circle(0.0, melq_y - 35.0 * scale, 22.0 * scale, Color::hex("#80D0FF"), 2.0 * scale);

    // Ojos penetrantes de profeta
    canvas.fill_circle(-7.0 * scale, melq_y - 38.0 * scale, 3.0 * scale, Color::hex("#FFFFFF"));
    canvas.fill_circle(7.0 * scale, melq_y - 38.0 * scale, 3.0 * scale, Color::hex("#FFFFFF"));
    canvas.fill_circle(-7.0 * scale, melq_y - 38.0 * scale, 1.5 * scale, Color::hex("#101010"));
    canvas.fill_circle(7.0 * scale, melq_y - 38.0 * scale, 1.5 * scale, Color::hex("#101010"));

    // Argolla dorada de gitano en la oreja
    canvas.stroke_circle(24.0 * scale, melq_y - 32.0 * scale, 6.0 * scale, Color::hex("#FFD700"), 2.0 * scale);

    // Barba negra profética azabache
    let mut m_beard = tiny_skia::PathBuilder::new();
    m_beard.move_to(-16.0 * scale, melq_y - 30.0 * scale);
    m_beard.cubic_to(-20.0 * scale, melq_y - 5.0 * scale, -10.0 * scale, melq_y + 35.0 * scale, 0.0, melq_y + 55.0 * scale);
    m_beard.cubic_to(10.0 * scale, melq_y + 35.0 * scale, 20.0 * scale, melq_y - 5.0 * scale, 16.0 * scale, melq_y - 30.0 * scale);
    m_beard.close();
    if let Some(path) = m_beard.finish() {
        canvas.fill_path(&path, Color::hex("#0D0B08"));
        canvas.stroke_path(&path, Color::hex("#201A12"), 1.5 * scale);
    }

    // Sombrero negro de ala ancha de gitano trotador del mundo
    canvas.fill_rect(-65.0 * scale, melq_y - 54.0 * scale, 130.0 * scale, 8.0 * scale, Color::hex("#0D0B08"));
    canvas.stroke_rect(-65.0 * scale, melq_y - 54.0 * scale, 130.0 * scale, 8.0 * scale, Color::hex("#FFD700"), 1.8 * scale);
    // Copa alta del sombrero
    canvas.fill_rect(-28.0 * scale, melq_y - 92.0 * scale, 56.0 * scale, 38.0 * scale, Color::hex("#0D0B08"));
    canvas.stroke_rect(-28.0 * scale, melq_y - 92.0 * scale, 56.0 * scale, 38.0 * scale, Color::hex("#80D0FF"), 1.8 * scale);
    // Cinta dorada del sombrero
    canvas.fill_rect(-28.0 * scale, melq_y - 62.0 * scale, 56.0 * scale, 8.0 * scale, Color::hex("#FFD700"));

    // Oboe de madera negra de ébano con llaves de plata en sus manos
    let oboe_len = 110.0 * scale;
    canvas.stroke_line(-oboe_len * 0.45, melq_y + 35.0 * scale, oboe_len * 0.45, melq_y + 50.0 * scale, Color::hex("#101010"), 6.0 * scale);
    canvas.stroke_line(-oboe_len * 0.45, melq_y + 35.0 * scale, oboe_len * 0.45, melq_y + 50.0 * scale, Color::hex("#80D0FF"), 1.4 * scale);
    // Campana del oboe
    canvas.fill_circle(oboe_len * 0.46, melq_y + 51.0 * scale, 8.0 * scale, Color::hex("#FFD700"));
    // Llaves de plata
    for k in 0..6 {
        let kx = -oboe_len * 0.3 + (k as f32) * 12.0 * scale;
        let ky = melq_y + 38.0 * scale + (k as f32) * 2.0 * scale;
        canvas.fill_circle(kx, ky - 6.0 * scale, 2.5 * scale, Color::hex("#FFFFFF"));
        canvas.stroke_line(kx, ky, kx, ky - 6.0 * scale, Color::hex("#FFFFFF"), 1.2 * scale);
    }

    // Pergaminos antiguos de Melquíades abiertos al frente con sánscrito
    let parch_y = ice_h * 0.5 + 30.0 * scale;
    let pw = 260.0 * scale;
    let ph = 80.0 * scale;
    canvas.fill_rect(-pw * 0.5, parch_y, pw, ph, Color::hex("#F2E5C9"));
    canvas.stroke_rect(-pw * 0.5, parch_y, pw, ph, Color::hex("#FFD700"), 2.5 * scale);
    // Manchas de carcoma y sellos de lacre
    canvas.fill_circle(-pw * 0.35, parch_y + ph * 0.5, 12.0 * scale, Color::hex("#A8201A"));
    canvas.stroke_circle(-pw * 0.35, parch_y + ph * 0.5, 12.0 * scale, Color::hex("#FFD700"), 1.5 * scale);
    // Texto cifrado en sánscrito
    for l in 0..4 {
        let ly = parch_y + 16.0 * scale + (l as f32) * 15.0 * scale;
        canvas.stroke_line(-pw * 0.15, ly, pw * 0.42, ly, Color::hex("#281810"), 2.0 * scale);
    }

    canvas.restore();
}

/// Dibuja el gran reloj de arena centenario y el árbol genealógico de Úrsula Iguarán
pub fn draw_reloj_arena_ursula(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let glass_col = Color::hex("#4a3728");
    let sand_col = Color::hex("#f1c40f");
    let wood_col = Color::hex("#2d2017");

    canvas.save();
    canvas.translate(cx, cy);

    // Bulbos de vidrio soplado del reloj de arena
    let bulb_w = 90.0 * scale;
    let bulb_h = 110.0 * scale;

    let mut glass_pb = tiny_skia::PathBuilder::new();
    glass_pb.move_to(-bulb_w, -bulb_h);
    glass_pb.cubic_to(-bulb_w * 0.9, -15.0 * scale, -10.0 * scale, -8.0 * scale, -5.0 * scale, 0.0);
    glass_pb.cubic_to(-10.0 * scale, 8.0 * scale, -bulb_w * 0.9, 15.0 * scale, -bulb_w, bulb_h);
    glass_pb.line_to(bulb_w, bulb_h);
    glass_pb.cubic_to(bulb_w * 0.9, 15.0 * scale, 10.0 * scale, 8.0 * scale, 5.0 * scale, 0.0);
    glass_pb.cubic_to(10.0 * scale, -8.0 * scale, bulb_w * 0.9, -15.0 * scale, bulb_w, -bulb_h);
    glass_pb.close();

    if let Some(path) = glass_pb.finish() {
        canvas.fill_path(&path, Color::hex("#F2E7CF").with_alpha(0.6));
        canvas.stroke_path(&path, glass_col, 3.2 * scale);
    }

    // Tapas de madera superior e inferior
    canvas.fill_rect(-bulb_w * 1.15, -bulb_h - 14.0 * scale, bulb_w * 2.3, 14.0 * scale, wood_col);
    canvas.fill_rect(-bulb_w * 1.15, bulb_h, bulb_w * 2.3, 14.0 * scale, wood_col);

    // Arena dorada cayendo
    let top_sand_lvl = -bulb_h + tau * bulb_h * 0.7;
    canvas.fill_rect(-bulb_w * 0.65, top_sand_lvl, bulb_w * 1.3, -top_sand_lvl - 10.0 * scale, sand_col);

    // Hilo fino de arena
    canvas.stroke_line(0.0, 0.0, 0.0, bulb_h - 15.0 * scale, sand_col, 2.5 * scale);

    // Montículo de arena dorada abajo
    let bot_sand_h = tau * bulb_h * 0.7;
    let mut sand_pb = tiny_skia::PathBuilder::new();
    sand_pb.move_to(-bulb_w * 0.75, bulb_h);
    sand_pb.cubic_to(-bulb_w * 0.3, bulb_h - bot_sand_h - 8.0 * scale, bulb_w * 0.3, bulb_h - bot_sand_h - 8.0 * scale, bulb_w * 0.75, bulb_h);
    sand_pb.close();
    if let Some(path) = sand_pb.finish() {
        canvas.fill_path(&path, sand_col);
    }

    // Ramas del árbol genealógico que abrazan el reloj
    for &side in &[-1.0_f32, 1.0_f32] {
        let mut b_pb = tiny_skia::PathBuilder::new();
        b_pb.move_to(side * bulb_w * 1.1, bulb_h);
        b_pb.cubic_to(side * (bulb_w * 1.7), 0.0, side * (bulb_w * 1.5), -bulb_h * 0.5, side * bulb_w * 1.1, -bulb_h);
        if let Some(path) = b_pb.finish() {
            canvas.stroke_path(&path, Color::hex("#7a5230"), 4.0 * scale);
        }
        // Pequeños animalitos de caramelo que cuelgan
        canvas.fill_circle(side * (bulb_w * 1.5), 0.0, 9.0 * scale, Color::hex("#e67e22"));
        canvas.stroke_circle(side * (bulb_w * 1.5), 0.0, 9.0 * scale, Color::hex("#d35400"), 1.5 * scale);
    }

    canvas.restore();
}

/// Dibuja el gran viento bíblico que borró a Macondo de la faz de la tierra
pub fn draw_viento_biblico_apocalipsis(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    let wind_col = Color::hex("#F2E7CF").with_alpha(0.45);
    let parchment_col = Color::hex("#EFE3C9");
    let ink_col = Color::hex("#38271a");

    canvas.save();
    canvas.translate(cx, cy);

    // Gran vórtice espiral de viento bíblico (3 espirales logarítmicas concéntricas)
    for sp in 0..3 {
        let offset = sp as f32 * (2.0 * PI / 3.0);
        let mut pb = tiny_skia::PathBuilder::new();
        for i in 0..45 {
            let theta = (i as f32) * 0.18 + tau * 8.0 * PI + offset;
            let r = 25.0 * scale + (i as f32) * 11.0 * scale;
            let px = theta.cos() * r;
            let py = theta.sin() * (r * 0.55);
            if i == 0 { pb.move_to(px, py); } else { pb.line_to(px, py); }
        }
        if let Some(path) = pb.finish() {
            canvas.stroke_path(&path, wind_col, 3.5 * scale);
        }
    }

    // Pergaminos de Melquíades que se deshojan y vuelan en remolinos
    let sheet_count = 18;
    for s in 0..sheet_count {
        let prog = (tau * 2.0 + (s as f32 / sheet_count as f32)) % 1.0;
        let theta = prog * 6.0 * PI + (s as f32 * 1.2);
        let r = 60.0 * scale + prog * 420.0 * scale;
        let px = theta.cos() * r;
        let py = theta.sin() * (r * 0.65);
        let rot = theta + PI * 0.5 + (rng.next_f32() - 0.5) * 0.6;

        canvas.save();
        canvas.translate(px, py);
        canvas.rotate(rot);

        // Hoja de pergamino
        let pw = 36.0 * scale;
        let ph = 26.0 * scale;
        canvas.fill_rect(-pw * 0.5, -ph * 0.5, pw, ph, parchment_col);
        canvas.stroke_rect(-pw * 0.5, -ph * 0.5, pw, ph, ink_col, 1.4 * scale);

        // Líneas de texto cifrado en sánscrito
        for line in 0..3 {
            let ly = -ph * 0.3 + (line as f32) * (ph * 0.3);
            canvas.stroke_line(-pw * 0.38, ly, pw * 0.38, ly, ink_col.with_alpha(0.6), 1.0 * scale);
        }

        canvas.restore();
    }

    canvas.restore();
}

/// Dibuja el astrolabio de latón y sextante de José Arcadio Buendía para medir el meridiano de Macondo
pub fn draw_astrolabio_jose_arcadio(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let brass_dark = Color::hex("#8c6f1a");
    let brass_mid = Color::hex("#d4af37");
    let brass_bright = Color::hex("#ffd700");
    let ink_col = Color::hex("#2a1c13");

    canvas.save();
    canvas.translate(cx, cy);

    let r_outer = 220.0 * scale;
    let r_inner = 190.0 * scale;

    // Trono superior de suspensión con anilla
    canvas.stroke_circle(0.0, -r_outer - 35.0 * scale, 25.0 * scale, brass_mid, 5.0 * scale);
    let mut throne_pb = tiny_skia::PathBuilder::new();
    throne_pb.move_to(-45.0 * scale, -r_outer + 5.0 * scale);
    throne_pb.cubic_to(-25.0 * scale, -r_outer - 25.0 * scale, 25.0 * scale, -r_outer - 25.0 * scale, 45.0 * scale, -r_outer + 5.0 * scale);
    throne_pb.close();
    if let Some(path) = throne_pb.finish() {
        canvas.fill_path(&path, brass_dark);
        canvas.stroke_path(&path, brass_bright, 2.0 * scale);
    }

    // Mater (disco exterior de latón macizo)
    canvas.fill_circle(0.0, 0.0, r_outer, Color::hex("#121b2d"));
    canvas.stroke_circle(0.0, 0.0, r_outer, brass_mid, 8.0 * scale);
    canvas.stroke_circle(0.0, 0.0, r_inner, brass_bright, 2.0 * scale);

    // Graduaciones circulares de 360 grados y cuadrantes
    for i in 0..72 {
        let ang = (i as f32) * (2.0 * PI / 72.0);
        let len = if i % 6 == 0 { 18.0 * scale } else { 8.0 * scale };
        let col = if i % 6 == 0 { brass_bright } else { brass_dark };
        let r1 = r_outer - 4.0 * scale;
        let r2 = r1 - len;
        canvas.stroke_line(ang.cos() * r1, ang.sin() * r1, ang.cos() * r2, ang.sin() * r2, col, if i % 6 == 0 { 2.0 * scale } else { 1.0 * scale });
    }

    // Rete gótica calada con punteros estelares en espiral
    let rete_rot = tau * 0.4 * PI;
    canvas.save();
    canvas.rotate(rete_rot);

    // Círculo eclíptico descentrado
    let ecliptic_r = 95.0 * scale;
    canvas.stroke_circle(0.0, -35.0 * scale, ecliptic_r, brass_bright.with_alpha(0.85), 2.2 * scale);
    canvas.stroke_circle(0.0, 0.0, 130.0 * scale, brass_mid.with_alpha(0.65), 1.6 * scale);

    // Puntas de estrellas míticas (Antares, Vega, Aldebarán, Sirio)
    for &ang in &[0.3_f32, 1.2, 2.1, 3.4, 4.2, 5.5] {
        let px = ang.cos() * 145.0 * scale;
        let py = ang.sin() * 145.0 * scale;
        canvas.fill_circle(px, py, 4.5 * scale, brass_bright);
        canvas.stroke_line(0.0, 0.0, px, py, brass_dark.with_alpha(0.5), 1.0 * scale);
    }
    canvas.restore();

    // Alidada giratoria de latón (regla con pínulas de mira astronómica)
    let alidade_ang = -0.65 + (tau * 2.0 * PI).sin() * 0.35;
    canvas.save();
    canvas.rotate(alidade_ang);
    canvas.stroke_line(-r_outer + 8.0 * scale, 0.0, r_outer - 8.0 * scale, 0.0, brass_bright, 6.0 * scale);
    canvas.stroke_line(-r_outer + 8.0 * scale, 0.0, r_outer - 8.0 * scale, 0.0, brass_dark, 2.0 * scale);

    // Pínulas de visada
    for &side in &[-1.0_f32, 1.0_f32] {
        let px = side * (r_outer * 0.65);
        canvas.fill_rect(px - 4.0 * scale, -14.0 * scale, 8.0 * scale, 28.0 * scale, brass_bright);
        canvas.stroke_rect(px - 4.0 * scale, -14.0 * scale, 8.0 * scale, 28.0 * scale, ink_col, 1.2 * scale);
        // Agujero de mira
        canvas.fill_circle(px, 0.0, 2.2 * scale, Color::hex("#ffffff"));
    }

    // Tornillo central del caballo (perno chaveta)
    canvas.fill_circle(0.0, 0.0, 16.0 * scale, brass_mid);
    canvas.stroke_circle(0.0, 0.0, 16.0 * scale, brass_bright, 2.5 * scale);
    canvas.fill_circle(0.0, 0.0, 5.0 * scale, ink_col);
    canvas.restore();

    canvas.restore();
}

/// Dibuja el bastidor de bordar de Amaranta y el sudario interminable que teje de día y desteje de noche
pub fn draw_amaranta_sudario(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let wood_col = Color::hex("#5c3a21");
    let linen_bg = Color::hex("#F9F6F0");
    let thread_gold = Color::hex("#d4af37");
    let thread_black = Color::hex("#1a1512");
    let lace_col = Color::hex("#2a231d");

    canvas.save();
    canvas.translate(cx, cy);

    let hoop_rx = 190.0 * scale;
    let hoop_ry = 230.0 * scale;

    // Bastidor ovalado de madera pulida
    let mut hoop_pb = tiny_skia::PathBuilder::new();
    hoop_pb.move_to(-hoop_rx, 0.0);
    hoop_pb.cubic_to(-hoop_rx, -hoop_ry, hoop_rx, -hoop_ry, hoop_rx, 0.0);
    hoop_pb.cubic_to(hoop_rx, hoop_ry, -hoop_rx, hoop_ry, -hoop_rx, 0.0);
    hoop_pb.close();

    if let Some(path) = hoop_pb.finish() {
        canvas.fill_path(&path, linen_bg);
        canvas.stroke_path(&path, wood_col, 14.0 * scale);
        canvas.stroke_path(&path, Color::hex("#8a5a36"), 4.0 * scale);
    }

    // Tornillo metálico de tensión en la parte superior del bastidor
    canvas.fill_rect(-14.0 * scale, -hoop_ry - 22.0 * scale, 28.0 * scale, 14.0 * scale, Color::hex("#c0a060"));
    canvas.stroke_rect(-14.0 * scale, -hoop_ry - 22.0 * scale, 28.0 * scale, 14.0 * scale, wood_col, 2.0 * scale);
    canvas.stroke_line(-22.0 * scale, -hoop_ry - 15.0 * scale, 22.0 * scale, -hoop_ry - 15.0 * scale, Color::hex("#ffd700"), 3.0 * scale);

    // Trama y urdimbre del lino fino (rejilla de punto de cruz)
    let grid_step = 18.0 * scale;
    let mut gx = -hoop_rx * 0.75;
    while gx <= hoop_rx * 0.75 {
        canvas.stroke_line(gx, -hoop_ry * 0.65, gx, hoop_ry * 0.65, Color::hex("#E2DAC8").with_alpha(0.6), 0.8 * scale);
        gx += grid_step;
    }
    let mut gy = -hoop_ry * 0.65;
    while gy <= hoop_ry * 0.65 {
        canvas.stroke_line(-hoop_rx * 0.75, gy, hoop_rx * 0.75, gy, Color::hex("#E2DAC8").with_alpha(0.6), 0.8 * scale);
        gy += grid_step;
    }

    // El laberinto funerario tejido en hilo dorado y negro
    let rows = 9;
    let cols = 11;
    let weave_prog = ((tau * 2.0) % 1.0) * (rows * cols) as f32;
    for r in 0..rows {
        for c in 0..cols {
            let idx = (r * cols + c) as f32;
            if idx <= weave_prog {
                let px = -hoop_rx * 0.55 + (c as f32) * (grid_step * 1.1);
                let py = -hoop_ry * 0.45 + (r as f32) * (grid_step * 1.1);
                let col = if (r + c) % 2 == 0 { thread_gold } else { thread_black };
                // Puntada en X
                let s = 6.0 * scale;
                canvas.stroke_line(px - s, py - s, px + s, py + s, col, 1.6 * scale);
                canvas.stroke_line(px - s, py + s, px + s, py - s, col, 1.6 * scale);
            }
        }
    }

    // Aguja fina de plata con hilo enhebrado
    let needle_x = 40.0 * scale + (tau * 4.0 * PI).sin() * 20.0 * scale;
    let needle_y = 30.0 * scale + (tau * 4.0 * PI).cos() * 15.0 * scale;
    canvas.stroke_line(needle_x - 35.0 * scale, needle_y - 25.0 * scale, needle_x + 35.0 * scale, needle_y + 25.0 * scale, Color::hex("#e0e8f0"), 2.2 * scale);
    canvas.fill_circle(needle_x + 35.0 * scale, needle_y + 25.0 * scale, 3.0 * scale, Color::hex("#ffffff"));

    // Hilo dorado ondulante que sale de la aguja
    let mut thread_pb = tiny_skia::PathBuilder::new();
    thread_pb.move_to(needle_x + 35.0 * scale, needle_y + 25.0 * scale);
    thread_pb.cubic_to(needle_x + 85.0 * scale, needle_y + 60.0 * scale, needle_x + 110.0 * scale, needle_y - 20.0 * scale, needle_x + 140.0 * scale, needle_y + 40.0 * scale);
    if let Some(path) = thread_pb.finish() {
        canvas.stroke_path(&path, thread_gold, 1.8 * scale);
    }

    // La venda negra de gasa en la mano quemada de Amaranta (símbolo de su luto perenne)
    let mut bandage_pb = tiny_skia::PathBuilder::new();
    bandage_pb.move_to(-hoop_rx * 0.9, hoop_ry * 0.4);
    bandage_pb.cubic_to(-hoop_rx * 1.3, hoop_ry * 0.7, -hoop_rx * 0.8, hoop_ry * 1.0, -hoop_rx * 0.5, hoop_ry * 0.9);
    bandage_pb.line_to(-hoop_rx * 0.4, hoop_ry * 0.75);
    bandage_pb.cubic_to(-hoop_rx * 0.7, hoop_ry * 0.85, -hoop_rx * 1.0, hoop_ry * 0.6, -hoop_rx * 0.8, hoop_ry * 0.35);
    bandage_pb.close();
    if let Some(path) = bandage_pb.finish() {
        canvas.fill_path(&path, lace_col);
        canvas.stroke_path(&path, Color::hex("#000000"), 2.0 * scale);
    }

    canvas.restore();
}

/// Dibuja el anteproyecto (blueprint) técnico del acordeón diatónico de Francisco el Hombre
pub fn draw_acordeon_blueprint(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
) {
    let navy_line = Color::hex("#80D0FF");
    let gold_line = Color::hex("#FFD700");
    let white_line = Color::hex("#FFFFFF");
    let dim_line = Color::hex("#C8C1EF");

    canvas.save();
    canvas.translate(cx, cy);

    let acc_w = 420.0 * scale;
    let acc_h = 260.0 * scale;
    let bellow_pump = (tau * 4.0 * PI).sin() * 25.0 * scale;

    // Caja de botones agudos (Treble cabinet) a la derecha
    let right_x = (acc_w * 0.25) + bellow_pump;
    canvas.fill_rect(right_x, -acc_h * 0.5, 95.0 * scale, acc_h, Color::hex("#102045"));
    canvas.stroke_rect(right_x, -acc_h * 0.5, 95.0 * scale, acc_h, navy_line, 3.0 * scale);

    // Diapasón con 3 hileras de botones de nácar (31 botones diatónicos)
    for col in 0..3 {
        let bx = right_x + 20.0 * scale + (col as f32) * 25.0 * scale;
        let num_buttons = if col == 1 { 11 } else { 10 };
        for btn in 0..num_buttons {
            let by = -acc_h * 0.42 + (btn as f32) * (acc_h * 0.82 / (num_buttons - 1) as f32);
            canvas.fill_circle(bx, by, 7.0 * scale, Color::hex("#F5E6C8"));
            canvas.stroke_circle(bx, by, 7.0 * scale, white_line, 1.4 * scale);
        }
    }

    // Caja de bajos a la izquierda (Bass cabinet)
    let left_x = (-acc_w * 0.25) - bellow_pump - 80.0 * scale;
    canvas.fill_rect(left_x, -acc_h * 0.5, 80.0 * scale, acc_h, Color::hex("#102045"));
    canvas.stroke_rect(left_x, -acc_h * 0.5, 80.0 * scale, acc_h, navy_line, 3.0 * scale);

    // 12 bajos mecánicos
    for col in 0..2 {
        let bx = left_x + 25.0 * scale + (col as f32) * 28.0 * scale;
        for btn in 0..6 {
            let by = -acc_h * 0.35 + (btn as f32) * (acc_h * 0.7 / 5.0);
            canvas.fill_circle(bx, by, 8.0 * scale, Color::hex("#F5E6C8"));
            canvas.stroke_circle(bx, by, 8.0 * scale, gold_line, 1.6 * scale);
        }
    }

    // Fuelle central (Bellows) de 12 pliegues con cantoneras metálicas
    let bellow_start = left_x + 80.0 * scale;
    let bellow_end = right_x;
    let bellow_total_w = bellow_end - bellow_start;
    let folds = 10;
    let fold_step = bellow_total_w / folds as f32;

    for f in 0..folds {
        let fx0 = bellow_start + (f as f32) * fold_step;
        let fx1 = fx0 + fold_step * 0.5;
        let fx2 = fx0 + fold_step;

        let mut fold_pb = tiny_skia::PathBuilder::new();
        fold_pb.move_to(fx0, -acc_h * 0.46);
        fold_pb.line_to(fx1, -acc_h * 0.52);
        fold_pb.line_to(fx2, -acc_h * 0.46);
        fold_pb.line_to(fx2, acc_h * 0.46);
        fold_pb.line_to(fx1, acc_h * 0.52);
        fold_pb.line_to(fx0, acc_h * 0.46);
        fold_pb.close();

        if let Some(path) = fold_pb.finish() {
            let fold_col = if f % 2 == 0 { Color::hex("#182B55") } else { Color::hex("#0D1D45") };
            canvas.fill_path(&path, fold_col);
            canvas.stroke_path(&path, dim_line, 1.5 * scale);
            // Cantoneras cromadas
            canvas.fill_circle(fx1, -acc_h * 0.52, 4.0 * scale, gold_line);
            canvas.fill_circle(fx1, acc_h * 0.52, 4.0 * scale, gold_line);
        }
    }

    // Ondas acústicas sinusoidales de vallenato que emanan de la parrilla
    for i in 0..5 {
        let r = 80.0 * scale + (i as f32) * 45.0 * scale + ((tau * 120.0 * scale) % (45.0 * scale));
        let alpha = (1.0 - (i as f32 / 5.0)) * 0.7;
        canvas.stroke_circle(right_x + 95.0 * scale, 0.0, r, gold_line.with_alpha(alpha), 1.8 * scale);
    }

    // Cotas técnicas de plano de taller
    canvas.stroke_line(left_x, acc_h * 0.65, right_x + 95.0 * scale, acc_h * 0.65, dim_line, 1.2 * scale);
    canvas.stroke_line(left_x, acc_h * 0.60, left_x, acc_h * 0.70, dim_line, 1.2 * scale);
    canvas.stroke_line(right_x + 95.0 * scale, acc_h * 0.60, right_x + 95.0 * scale, acc_h * 0.70, dim_line, 1.2 * scale);

    canvas.restore();
}

/// Dibuja el galeón español encallado en la selva con estilo maestro de grabado decimonónico
pub fn draw_galeon_selva_master(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    let wood_dark = Color::hex("#2a1b12");
    let wood_mid = Color::hex("#523624");
    let foliage_col = Color::hex("#2d5a27");
    let orchid_col = Color::hex("#ffd700");
    let rope_col = Color::hex("#8a7258");

    canvas.save();
    canvas.translate(cx, cy);

    let hull_w = 460.0 * scale;
    let hull_h = 130.0 * scale;

    // Casco del galeón con cuadernas y tablazón de roble
    let mut hull_pb = tiny_skia::PathBuilder::new();
    hull_pb.move_to(-hull_w * 0.52, -hull_h * 0.4); // Proa alta con espolón
    hull_pb.cubic_to(-hull_w * 0.35, hull_h * 0.6, hull_w * 0.25, hull_h * 0.65, hull_w * 0.48, hull_h * 0.1); // Quilla curvada
    hull_pb.line_to(hull_w * 0.50, -hull_h * 0.7); // Popa con castillo alto
    hull_pb.cubic_to(hull_w * 0.2, -hull_h * 0.35, -hull_w * 0.2, -hull_h * 0.3, -hull_w * 0.52, -hull_h * 0.4);
    hull_pb.close();

    if let Some(path) = hull_pb.finish() {
        canvas.fill_path(&path, wood_mid);
        canvas.stroke_path(&path, wood_dark, 4.0 * scale);
    }

    // Tablazones horizontales del casco
    for i in 0..7 {
        let frac = (i as f32) / 6.0;
        let ly = -hull_h * 0.2 + frac * (hull_h * 0.6);
        canvas.stroke_line(-hull_w * 0.42 + frac * 50.0 * scale, ly, hull_w * 0.42, ly, wood_dark, 1.8 * scale);
    }

    // Troneras de cañones con ventanillas carcomidas por la selva
    for c in 0..5 {
        let cx_pos = -hull_w * 0.25 + (c as f32) * (hull_w * 0.14);
        let cy_pos = -hull_h * 0.05;
        canvas.fill_rect(cx_pos - 8.0 * scale, cy_pos - 8.0 * scale, 16.0 * scale, 16.0 * scale, wood_dark);
        canvas.stroke_rect(cx_pos - 8.0 * scale, cy_pos - 8.0 * scale, 16.0 * scale, 16.0 * scale, Color::hex("#7a5230"), 1.6 * scale);
    }

    // Tres mástiles colosales (Trinquete, Palo Mayor, Mesana)
    let masts = [
        (-hull_w * 0.24, -340.0 * scale, 10.0 * scale), // Trinquete
        (hull_w * 0.04, -430.0 * scale, 13.0 * scale),   // Mayor
        (hull_w * 0.32, -290.0 * scale, 8.0 * scale),   // Mesana
    ];

    for &(mx, my, mw) in &masts {
        canvas.stroke_line(mx, -hull_h * 0.3, mx, my, wood_dark, mw);
        // Vergas cruzadas
        for v in 0..3 {
            let vy = my + (v as f32) * 85.0 * scale;
            let vw = (140.0 - (v as f32) * 30.0) * scale;
            canvas.stroke_line(mx - vw * 0.5, vy, mx + vw * 0.5, vy, wood_mid, 4.0 * scale);
            // Jarcias y cabos caídos
            canvas.stroke_line(mx - vw * 0.5, vy, mx, vy + 40.0 * scale, rope_col.with_alpha(0.6), 1.2 * scale);
            canvas.stroke_line(mx + vw * 0.5, vy, mx, vy + 40.0 * scale, rope_col.with_alpha(0.6), 1.2 * scale);
        }
    }

    // Orquídeas amarillas y lianas que cuelgan de las vergas hacia la tierra
    for i in 0..24 {
        let lx = -hull_w * 0.35 + rng.next_f32() * (hull_w * 0.7);
        let ly_start = -300.0 * scale + rng.next_f32() * 200.0 * scale;
        let l_len = 120.0 * scale + rng.next_f32() * 160.0 * scale;

        let mut vine_pb = tiny_skia::PathBuilder::new();
        vine_pb.move_to(lx, ly_start);
        vine_pb.cubic_to(lx + 20.0 * scale, ly_start + l_len * 0.4, lx - 20.0 * scale, ly_start + l_len * 0.7, lx + 5.0 * scale, ly_start + l_len);
        if let Some(path) = vine_pb.finish() {
            canvas.stroke_path(&path, foliage_col, 2.5 * scale);
        }

        // Orquídeas amarillas floreciendo en la madera podrida
        let sway = (tau * 3.0 + i as f32).sin() * 5.0 * scale;
        canvas.fill_circle(lx + sway, ly_start + l_len, 7.0 * scale, orchid_col);
        canvas.stroke_circle(lx + sway, ly_start + l_len, 7.0 * scale, Color::hex("#d35400"), 1.2 * scale);
    }

    // Helechos gigantes prehistóricos al pie del casco
    for f in 0..12 {
        let fx = -hull_w * 0.5 + (f as f32) * (hull_w * 0.09);
        let fy = hull_h * 0.5;
        let f_len = 90.0 * scale + rng.next_f32() * 50.0 * scale;
        let f_ang = -PI * 0.5 + (rng.next_f32() - 0.5) * 0.8;

        canvas.stroke_line(fx, fy, fx + f_ang.cos() * f_len, fy + f_ang.sin() * f_len, foliage_col, 3.5 * scale);
    }

    canvas.restore();
}

/// Dibuja la escena completa de Amaranta tejiendo su mortaja con la guitarra española y la pianola de Pietro Crespi
pub fn draw_amaranta_figura_y_guitarras(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
    blueprint_mode: bool,
) {
    let line_col = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#2A1C13") };
    let gold_col = Color::hex("#FFD700");
    let dress_col = if blueprint_mode { Color::hex("#102045") } else { Color::hex("#1C1410") };
    let skin_col = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#F4D2B5") };
    let guitar_top = if blueprint_mode { Color::hex("#162B55") } else { Color::hex("#E8C07A") };
    let guitar_side = if blueprint_mode { Color::hex("#0D1A38") } else { Color::hex("#5A2B14") };
    let paper_col = if blueprint_mode { Color::hex("#142A58") } else { Color::hex("#F6EED8") };

    canvas.save();
    canvas.translate(cx, cy);

    // 1. Rollo de papel perforado de la pianola italiana de Pietro Crespi
    let mut roll_pb = tiny_skia::PathBuilder::new();
    roll_pb.move_to(-280.0 * scale, -220.0 * scale);
    roll_pb.cubic_to(-120.0 * scale, -260.0 * scale, 80.0 * scale, -180.0 * scale, 260.0 * scale, -230.0 * scale);
    roll_pb.line_to(260.0 * scale, -170.0 * scale);
    roll_pb.cubic_to(80.0 * scale, -120.0 * scale, -120.0 * scale, -200.0 * scale, -280.0 * scale, -160.0 * scale);
    roll_pb.close();
    if let Some(path) = roll_pb.finish() {
        canvas.fill_path(&path, paper_col);
        canvas.stroke_path(&path, line_col, 2.0 * scale);
    }

    // Perforaciones mecánicas rectangulares del rollo de pianola
    for p in 0..16 {
        let t = p as f32 / 15.0;
        let px = -250.0 * scale + t * 480.0 * scale;
        let py = -210.0 * scale + (t * PI * 2.0).sin() * 25.0 * scale;
        let perf_h = (8.0 + (p % 3) as f32 * 6.0) * scale;
        canvas.fill_rect(px, py - perf_h * 0.5, 6.0 * scale, perf_h, if blueprint_mode { gold_col } else { line_col });
    }

    // Notas musicales flotantes
    let notes = ["♪", "♫", "♩", "♬", "♪"];
    for (i, &note) in notes.iter().enumerate() {
        let nx = -200.0 * scale + (i as f32) * 95.0 * scale + (tau * 40.0).sin() * 10.0 * scale;
        let ny = -250.0 * scale - (i as f32 * 12.0) * scale + (tau * 4.0 + i as f32).sin() * 8.0 * scale;
        crate::core::schematic::draw_vector_text(canvas, cx + nx, cy + ny, note, 22.0 * scale, gold_col, true);
    }

    // 2. Guitarra clásica española de concierto (a la izquierda)
    let gx = -150.0 * scale;
    let gy = 60.0 * scale;
    canvas.save();
    canvas.translate(gx, gy);
    canvas.rotate(-0.28); // Leve inclinación elegante

    // Mástil y diapasón
    canvas.fill_rect(-14.0 * scale, -260.0 * scale, 28.0 * scale, 180.0 * scale, if blueprint_mode { Color::hex("#0F1E3D") } else { Color::hex("#381C08") });
    canvas.stroke_rect(-14.0 * scale, -260.0 * scale, 28.0 * scale, 180.0 * scale, line_col, 2.0 * scale);

    // Trastes metálicos dorados
    for fret in 0..9 {
        let fy = -250.0 * scale + (fret as f32) * 18.0 * scale;
        canvas.stroke_line(-14.0 * scale, fy, 14.0 * scale, fy, gold_col, 1.4 * scale);
    }

    // Pala con clavijero ranurado
    let mut head_pb = tiny_skia::PathBuilder::new();
    head_pb.move_to(-16.0 * scale, -260.0 * scale);
    head_pb.line_to(-18.0 * scale, -320.0 * scale);
    head_pb.cubic_to(-10.0 * scale, -332.0 * scale, 10.0 * scale, -332.0 * scale, 18.0 * scale, -320.0 * scale);
    head_pb.line_to(16.0 * scale, -260.0 * scale);
    head_pb.close();
    if let Some(path) = head_pb.finish() {
        canvas.fill_path(&path, if blueprint_mode { Color::hex("#0F1E3D") } else { Color::hex("#4A250B") });
        canvas.stroke_path(&path, line_col, 2.0 * scale);
    }
    // 6 clavijas de hueso
    for s in 0..3 {
        let py = -275.0 * scale - (s as f32) * 16.0 * scale;
        canvas.stroke_line(-26.0 * scale, py, -16.0 * scale, py, gold_col, 3.0 * scale);
        canvas.stroke_line(16.0 * scale, py, 26.0 * scale, py, gold_col, 3.0 * scale);
    }

    // Caja de resonancia (curvas en 8 de la guitarra clásica)
    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(0.0, -80.0 * scale);
    // Bóveda superior
    body_pb.cubic_to(75.0 * scale, -80.0 * scale, 85.0 * scale, -25.0 * scale, 55.0 * scale, 15.0 * scale);
    // Cintura estrecha
    body_pb.cubic_to(42.0 * scale, 35.0 * scale, 48.0 * scale, 45.0 * scale, 65.0 * scale, 65.0 * scale);
    // Bóveda inferior más amplia
    body_pb.cubic_to(110.0 * scale, 110.0 * scale, 95.0 * scale, 190.0 * scale, 0.0, 190.0 * scale);
    body_pb.cubic_to(-95.0 * scale, 190.0 * scale, -110.0 * scale, 110.0 * scale, -65.0 * scale, 65.0 * scale);
    body_pb.cubic_to(-48.0 * scale, 45.0 * scale, -42.0 * scale, 35.0 * scale, -55.0 * scale, 15.0 * scale);
    body_pb.cubic_to(-85.0 * scale, -25.0 * scale, -75.0 * scale, -80.0 * scale, 0.0, -80.0 * scale);
    body_pb.close();

    if let Some(path) = body_pb.finish() {
        canvas.fill_path(&path, guitar_top);
        canvas.stroke_path(&path, guitar_side, 8.0 * scale);
        canvas.stroke_path(&path, line_col, 2.5 * scale);
    }

    // Boca circular con roseta ornamentada
    canvas.fill_circle(0.0, -10.0 * scale, 28.0 * scale, if blueprint_mode { Color::hex("#080F1E") } else { Color::hex("#1A0D05") });
    canvas.stroke_circle(0.0, -10.0 * scale, 32.0 * scale, gold_col, 2.0 * scale);
    canvas.stroke_circle(0.0, -10.0 * scale, 36.0 * scale, line_col, 1.2 * scale);

    // Puente de palisandro
    canvas.fill_rect(-42.0 * scale, 115.0 * scale, 84.0 * scale, 18.0 * scale, if blueprint_mode { Color::hex("#0D1A38") } else { Color::hex("#381C08") });
    canvas.stroke_rect(-42.0 * scale, 115.0 * scale, 84.0 * scale, 18.0 * scale, line_col, 1.5 * scale);
    canvas.fill_rect(-32.0 * scale, 121.0 * scale, 64.0 * scale, 4.0 * scale, Color::hex("#FFFFFF"));

    // 6 Cuerdas doradas tensas
    for c in 0..6 {
        let cx_pos = -9.0 * scale + (c as f32) * 3.6 * scale;
        canvas.stroke_line(cx_pos * 0.7, -260.0 * scale, cx_pos, 120.0 * scale, gold_col.with_alpha(0.85), 1.0 * scale);
    }

    canvas.restore();

    // 3. Bastidor de bordado de la mortaja (al centro-derecha)
    let hoop_x = 100.0 * scale;
    let hoop_y = 15.0 * scale;
    draw_amaranta_sudario(canvas, hoop_x, hoop_y, 0.95 * scale, tau);

    // 4. Silueta noble y enlutada de Amaranta
    let ax = 200.0 * scale;
    let ay = 25.0 * scale;
    canvas.save();
    canvas.translate(ax, ay);

    // Vestido victoriano de luto riguroso (corsé y amplia falda)
    let mut dress_pb = tiny_skia::PathBuilder::new();
    dress_pb.move_to(-35.0 * scale, -50.0 * scale);
    dress_pb.cubic_to(-45.0 * scale, 20.0 * scale, -95.0 * scale, 110.0 * scale, -110.0 * scale, 190.0 * scale);
    dress_pb.line_to(60.0 * scale, 190.0 * scale);
    dress_pb.cubic_to(45.0 * scale, 110.0 * scale, 5.0 * scale, 20.0 * scale, 0.0, -50.0 * scale);
    dress_pb.close();
    if let Some(path) = dress_pb.finish() {
        canvas.fill_path(&path, dress_col);
        canvas.stroke_path(&path, line_col, 2.5 * scale);
    }

    // Pechera de encaje y cuello alto
    let mut lace_pb = tiny_skia::PathBuilder::new();
    lace_pb.move_to(-25.0 * scale, -50.0 * scale);
    lace_pb.line_to(0.0, -15.0 * scale);
    lace_pb.line_to(-5.0 * scale, -50.0 * scale);
    lace_pb.close();
    if let Some(path) = lace_pb.finish() {
        canvas.stroke_path(&path, gold_col, 1.8 * scale);
    }

    // Cabeza y peinado con peineta alta
    canvas.fill_circle(-18.0 * scale, -85.0 * scale, 18.0 * scale, skin_col);
    canvas.stroke_circle(-18.0 * scale, -85.0 * scale, 18.0 * scale, line_col, 2.0 * scale);

    // Moño y peineta de carey
    let mut comb_pb = tiny_skia::PathBuilder::new();
    comb_pb.move_to(-28.0 * scale, -98.0 * scale);
    comb_pb.line_to(-38.0 * scale, -135.0 * scale);
    comb_pb.cubic_to(-20.0 * scale, -145.0 * scale, -5.0 * scale, -145.0 * scale, 5.0 * scale, -130.0 * scale);
    comb_pb.line_to(-10.0 * scale, -98.0 * scale);
    comb_pb.close();
    if let Some(path) = comb_pb.finish() {
        canvas.fill_path(&path, if blueprint_mode { Color::hex("#0D1A38") } else { Color::hex("#3E1C0A") });
        canvas.stroke_path(&path, gold_col, 2.0 * scale);
    }

    // Mantilla negra de encaje que cae por la espalda
    let mut mant_pb = tiny_skia::PathBuilder::new();
    mant_pb.move_to(-25.0 * scale, -130.0 * scale);
    mant_pb.cubic_to(-75.0 * scale, -70.0 * scale, -80.0 * scale, 20.0 * scale, -60.0 * scale, 90.0 * scale);
    mant_pb.line_to(-35.0 * scale, 85.0 * scale);
    mant_pb.cubic_to(-50.0 * scale, 20.0 * scale, -45.0 * scale, -60.0 * scale, -12.0 * scale, -95.0 * scale);
    mant_pb.close();
    if let Some(path) = mant_pb.finish() {
        canvas.fill_path(&path, if blueprint_mode { Color::hex("#080F1E") } else { Color::hex("#120B07") });
        canvas.stroke_path(&path, line_col, 1.5 * scale);
    }

    // Brazo izquierdo extendido hacia el bastidor con la VENDA NEGRA DE GASA
    let arm_end_x = -110.0 * scale;
    let arm_end_y = 10.0 * scale;
    canvas.stroke_line(-25.0 * scale, -30.0 * scale, -70.0 * scale, -5.0 * scale, skin_col, 12.0 * scale);
    canvas.stroke_line(-70.0 * scale, -5.0 * scale, arm_end_x, arm_end_y, skin_col, 10.0 * scale);

    // Venda de gasa negra enrollada en la mano quemada
    let mut bandage_pb = tiny_skia::PathBuilder::new();
    bandage_pb.move_to(arm_end_x - 15.0 * scale, arm_end_y - 12.0 * scale);
    bandage_pb.line_to(arm_end_x + 18.0 * scale, arm_end_y - 5.0 * scale);
    bandage_pb.line_to(arm_end_x + 12.0 * scale, arm_end_y + 14.0 * scale);
    bandage_pb.line_to(arm_end_x - 18.0 * scale, arm_end_y + 8.0 * scale);
    bandage_pb.close();
    if let Some(path) = bandage_pb.finish() {
        canvas.fill_path(&path, Color::hex("#0A0604"));
        canvas.stroke_path(&path, if blueprint_mode { gold_col } else { Color::hex("#FFFFFF") }, 1.5 * scale);
    }
    // Lazos sueltos de la venda negra ondeando
    canvas.stroke_line(arm_end_x, arm_end_y + 8.0 * scale, arm_end_x - 18.0 * scale, arm_end_y + 35.0 * scale, Color::hex("#0A0604"), 3.0 * scale);
    canvas.stroke_line(arm_end_x + 6.0 * scale, arm_end_y + 6.0 * scale, arm_end_x + 14.0 * scale, arm_end_y + 30.0 * scale, Color::hex("#0A0604"), 2.2 * scale);

    canvas.restore();

    canvas.restore();
}

/// Dibuja la venerable matriarca centenaria Úrsula Iguarán en su mecedora con su bandeja de gallitos de caramelo y el gran reloj de arena
pub fn draw_ursula_matriarca_escena(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
    blueprint_mode: bool,
) {
    let line_col = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#2A1C13") };
    let gold_col = Color::hex("#FFD700");
    let amber_col = Color::hex("#FF9800");
    let shawl_col = if blueprint_mode { Color::hex("#122247") } else { Color::hex("#30241C") };
    let skin_col = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#F2D5BD") };
    let chair_col = if blueprint_mode { Color::hex("#0C1733") } else { Color::hex("#543118") };

    canvas.save();
    canvas.translate(cx, cy);

    // 1. Reloj de arena monumental de los Cien Años (a la derecha)
    let hourglass_x = 140.0 * scale;
    let hourglass_y = -30.0 * scale;
    draw_reloj_arena_ursula(canvas, hourglass_x, hourglass_y, 1.15 * scale, tau);

    // 2. Mecedora colonial de madera torneada con suave vaivén
    let rock_tilt = (tau * 4.0 * PI).sin() * 0.038;
    let chair_x = -70.0 * scale;
    let chair_y = 60.0 * scale;

    canvas.save();
    canvas.translate(chair_x, chair_y);
    canvas.rotate(rock_tilt);

    // Patines curvos de la mecedora
    let mut runner_pb = tiny_skia::PathBuilder::new();
    runner_pb.move_to(-125.0 * scale, 120.0 * scale);
    runner_pb.cubic_to(-40.0 * scale, 138.0 * scale, 60.0 * scale, 138.0 * scale, 135.0 * scale, 115.0 * scale);
    if let Some(path) = runner_pb.finish() {
        canvas.stroke_path(&path, chair_col, 8.0 * scale);
        canvas.stroke_path(&path, line_col, 2.0 * scale);
    }

    // Patas torneadas
    canvas.stroke_line(-70.0 * scale, 40.0 * scale, -85.0 * scale, 128.0 * scale, chair_col, 6.0 * scale);
    canvas.stroke_line(50.0 * scale, 40.0 * scale, 70.0 * scale, 128.0 * scale, chair_col, 6.0 * scale);

    // Asiento
    canvas.stroke_line(-80.0 * scale, 40.0 * scale, 60.0 * scale, 40.0 * scale, chair_col, 10.0 * scale);
    canvas.stroke_line(-80.0 * scale, 40.0 * scale, 60.0 * scale, 40.0 * scale, line_col, 2.0 * scale);

    // Respaldo alto de balaustres torneados
    canvas.stroke_line(-75.0 * scale, 40.0 * scale, -95.0 * scale, -130.0 * scale, chair_col, 8.0 * scale);
    canvas.stroke_line(-95.0 * scale, -130.0 * scale, -20.0 * scale, -145.0 * scale, chair_col, 8.0 * scale);
    for b in 0..5 {
        let bx = -70.0 * scale + (b as f32) * 12.0 * scale;
        canvas.stroke_line(bx, 35.0 * scale, bx - 14.0 * scale, -125.0 * scale, chair_col, 3.5 * scale);
    }

    // Reposabrazos curvo
    let mut arm_pb = tiny_skia::PathBuilder::new();
    arm_pb.move_to(-80.0 * scale, -30.0 * scale);
    arm_pb.cubic_to(-40.0 * scale, -40.0 * scale, 10.0 * scale, -35.0 * scale, 40.0 * scale, -10.0 * scale);
    if let Some(path) = arm_pb.finish() {
        canvas.stroke_path(&path, chair_col, 6.0 * scale);
    }

    // 3. Figura venerable de Úrsula Iguarán (115 años)
    // Cuerpo encorvado cubierto por el rebozo
    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(-60.0 * scale, -65.0 * scale);
    body_pb.cubic_to(-85.0 * scale, -20.0 * scale, -80.0 * scale, 30.0 * scale, -70.0 * scale, 45.0 * scale);
    body_pb.line_to(35.0 * scale, 45.0 * scale);
    body_pb.cubic_to(40.0 * scale, 20.0 * scale, 25.0 * scale, -30.0 * scale, -10.0 * scale, -55.0 * scale);
    body_pb.close();
    if let Some(path) = body_pb.finish() {
        canvas.fill_path(&path, shawl_col);
        canvas.stroke_path(&path, line_col, 2.5 * scale);
    }

    // Flecos del rebozo cayendo
    for f in 0..12 {
        let fx = -65.0 * scale + (f as f32) * 8.0 * scale;
        canvas.stroke_line(fx, 45.0 * scale, fx + 2.0 * scale, 65.0 * scale, if blueprint_mode { gold_col } else { Color::hex("#201610") }, 1.5 * scale);
    }

    // Delantal blanco de faena
    let mut apron_pb = tiny_skia::PathBuilder::new();
    apron_pb.move_to(-40.0 * scale, -10.0 * scale);
    apron_pb.line_to(20.0 * scale, -10.0 * scale);
    apron_pb.line_to(15.0 * scale, 42.0 * scale);
    apron_pb.line_to(-35.0 * scale, 42.0 * scale);
    apron_pb.close();
    if let Some(path) = apron_pb.finish() {
        canvas.fill_path(&path, if blueprint_mode { Color::hex("#0B152B") } else { Color::hex("#EFE8DA") });
        canvas.stroke_path(&path, line_col, 1.6 * scale);
    }

    // Cabeza venerable de perfil
    let head_x = -35.0 * scale;
    let head_y = -95.0 * scale;

    // Cabello blanco recogido en moño
    canvas.fill_circle(head_x - 14.0 * scale, head_y - 2.0 * scale, 15.0 * scale, Color::hex("#E6ECEF"));
    canvas.stroke_circle(head_x - 14.0 * scale, head_y - 2.0 * scale, 15.0 * scale, line_col, 1.8 * scale);

    // Rostro sereno
    canvas.fill_circle(head_x, head_y, 16.0 * scale, skin_col);
    canvas.stroke_circle(head_x, head_y, 16.0 * scale, line_col, 2.0 * scale);

    // Ojos ciegos perlados que ven el futuro y el pasado
    canvas.fill_circle(head_x + 6.0 * scale, head_y - 2.0 * scale, 3.2 * scale, Color::hex("#FFFFFF"));
    canvas.stroke_circle(head_x + 6.0 * scale, head_y - 2.0 * scale, 3.2 * scale, gold_col, 1.4 * scale);

    // 4. Bandeja de hojalata / plata con los GALLITOS DE CARAMELO
    let tray_x = 0.0;
    let tray_y = 5.0 * scale;

    let mut tray_pb = tiny_skia::PathBuilder::new();
    tray_pb.move_to(tray_x - 45.0 * scale, tray_y);
    tray_pb.cubic_to(tray_x - 20.0 * scale, tray_y + 16.0 * scale, tray_x + 35.0 * scale, tray_y + 16.0 * scale, tray_x + 60.0 * scale, tray_y);
    tray_pb.line_to(tray_x + 55.0 * scale, tray_y - 6.0 * scale);
    tray_pb.cubic_to(tray_x + 30.0 * scale, tray_y + 8.0 * scale, tray_x - 15.0 * scale, tray_y + 8.0 * scale, tray_x - 40.0 * scale, tray_y - 6.0 * scale);
    tray_pb.close();
    if let Some(path) = tray_pb.finish() {
        canvas.fill_path(&path, if blueprint_mode { Color::hex("#1E335E") } else { Color::hex("#C5D0D8") });
        canvas.stroke_path(&path, line_col, 2.0 * scale);
    }

    // Gallitos de caramelo en palitos (dulces de Úrsula que salvaron a la familia)
    for g in 0..5 {
        let gx = tray_x - 25.0 * scale + (g as f32) * 18.0 * scale;
        let gy = tray_y - 12.0 * scale - (g as f32 % 2.0) * 10.0 * scale;

        // Palito de madera
        canvas.stroke_line(gx, gy + 15.0 * scale, gx, gy - 12.0 * scale, if blueprint_mode { gold_col } else { Color::hex("#A07248") }, 2.0 * scale);

        // Silueta del gallito de azúcar ámbar
        let mut cock_pb = tiny_skia::PathBuilder::new();
        cock_pb.move_to(gx, gy);
        cock_pb.cubic_to(gx + 10.0 * scale, gy - 6.0 * scale, gx + 14.0 * scale, gy - 18.0 * scale, gx + 8.0 * scale, gy - 26.0 * scale);
        // Cresta roja
        cock_pb.line_to(gx + 4.0 * scale, gy - 32.0 * scale);
        cock_pb.line_to(gx, gy - 28.0 * scale);
        // Pecho y cola plumosa
        cock_pb.cubic_to(gx - 12.0 * scale, gy - 26.0 * scale, gx - 16.0 * scale, gy - 14.0 * scale, gx - 18.0 * scale, gy - 24.0 * scale);
        cock_pb.cubic_to(gx - 16.0 * scale, gy - 4.0 * scale, gx - 8.0 * scale, gy + 2.0 * scale, gx, gy);
        cock_pb.close();

        if let Some(path) = cock_pb.finish() {
            canvas.fill_path(&path, amber_col);
            canvas.stroke_path(&path, gold_col, 1.5 * scale);
        }
    }

    canvas.restore();

    canvas.restore();
}

/// Dibuja a Mauricio Babilonia, el aprendiz de mecánica, con su llave inglesa y la nube espiral de mariposas amarillas frente a la ventana de Meme
pub fn draw_mauricio_babilonia_escena(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    tau: f32,
    blueprint_mode: bool,
    seed: u32,
) {
    let line_col = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#2A1C13") };
    let gold_col = Color::hex("#FFD700");
    let skin_col = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#E5BA98") };
    let shirt_col = if blueprint_mode { Color::hex("#122247") } else { Color::hex("#F4EDE0") };
    let pants_col = if blueprint_mode { Color::hex("#0D1730") } else { Color::hex("#2B3A4A") };
    let wall_col = if blueprint_mode { Color::hex("#091326") } else { Color::hex("#E8DFCF") };
    let shutter_col = if blueprint_mode { Color::hex("#102B50") } else { Color::hex("#2A6055") };

    canvas.save();
    canvas.translate(cx, cy);

    // 1. Fachada de la casa colonial con la ventana alta de los baños (donde se asoma Meme)
    let win_x = -170.0 * scale;
    let win_y = -140.0 * scale;
    let win_w = 110.0 * scale;
    let win_h = 160.0 * scale;

    // Muro de cal y canto
    canvas.fill_rect(win_x - 60.0 * scale, win_y - 80.0 * scale, win_w + 120.0 * scale, win_h + 120.0 * scale, wall_col);
    canvas.stroke_rect(win_x - 60.0 * scale, win_y - 80.0 * scale, win_w + 120.0 * scale, win_h + 120.0 * scale, line_col.with_alpha(0.35), 1.5 * scale);

    // Marco del vano con arco de medio punto superior
    let mut arch_pb = tiny_skia::PathBuilder::new();
    arch_pb.move_to(win_x - win_w * 0.5, win_y + win_h * 0.5);
    arch_pb.line_to(win_x - win_w * 0.5, win_y - win_h * 0.2);
    arch_pb.cubic_to(win_x - win_w * 0.5, win_y - win_h * 0.6, win_x + win_w * 0.5, win_y - win_h * 0.6, win_x + win_w * 0.5, win_y - win_h * 0.2);
    arch_pb.line_to(win_x + win_w * 0.5, win_y + win_h * 0.5);
    arch_pb.close();

    if let Some(path) = arch_pb.finish() {
        canvas.fill_path(&path, if blueprint_mode { Color::hex("#050A14") } else { Color::hex("#1E1610") });
        canvas.stroke_path(&path, line_col, 3.0 * scale);
    }

    // Postigos de madera abiertos hacia afuera
    canvas.fill_rect(win_x - win_w * 0.5 - 38.0 * scale, win_y - win_h * 0.25, 38.0 * scale, win_h * 0.75, shutter_col);
    canvas.stroke_rect(win_x - win_w * 0.5 - 38.0 * scale, win_y - win_h * 0.25, 38.0 * scale, win_h * 0.75, line_col, 2.0 * scale);

    canvas.fill_rect(win_x + win_w * 0.5, win_y - win_h * 0.25, 38.0 * scale, win_h * 0.75, shutter_col);
    canvas.stroke_rect(win_x + win_w * 0.5, win_y - win_h * 0.25, 38.0 * scale, win_h * 0.75, line_col, 2.0 * scale);

    // Silueta de Meme en la ventana, con cabello largo oscuro cayendo
    let meme_x = win_x;
    let meme_y = win_y - win_h * 0.1;
    canvas.fill_circle(meme_x, meme_y - 18.0 * scale, 12.0 * scale, skin_col);
    canvas.stroke_circle(meme_x, meme_y - 18.0 * scale, 12.0 * scale, line_col, 1.5 * scale);

    // Cabello largo suelto ondeando
    let mut hair_pb = tiny_skia::PathBuilder::new();
    hair_pb.move_to(meme_x - 14.0 * scale, meme_y - 20.0 * scale);
    hair_pb.cubic_to(meme_x - 22.0 * scale, meme_y + 10.0 * scale, meme_x - 18.0 * scale, meme_y + 45.0 * scale, meme_x - 12.0 * scale, meme_y + 60.0 * scale);
    hair_pb.line_to(meme_x + 8.0 * scale, meme_y + 60.0 * scale);
    hair_pb.cubic_to(meme_x + 14.0 * scale, meme_y + 35.0 * scale, meme_x + 16.0 * scale, meme_y, meme_x + 12.0 * scale, meme_y - 20.0 * scale);
    hair_pb.close();
    if let Some(path) = hair_pb.finish() {
        canvas.fill_path(&path, if blueprint_mode { Color::hex("#050A14") } else { Color::hex("#1C1008") });
    }

    // 2. Figura heroica de Mauricio Babilonia (aprendiz de mecánica)
    let mx = 110.0 * scale;
    let my = 70.0 * scale;
    canvas.save();
    canvas.translate(mx, my);

    // Pantalones de trabajo de dril oscuro
    canvas.fill_rect(-38.0 * scale, 40.0 * scale, 34.0 * scale, 140.0 * scale, pants_col);
    canvas.stroke_rect(-38.0 * scale, 40.0 * scale, 34.0 * scale, 140.0 * scale, line_col, 2.0 * scale);

    canvas.fill_rect(2.0 * scale, 40.0 * scale, 34.0 * scale, 140.0 * scale, pants_col);
    canvas.stroke_rect(2.0 * scale, 40.0 * scale, 34.0 * scale, 140.0 * scale, line_col, 2.0 * scale);

    // Botas de cuero de mecánico
    canvas.fill_rect(-42.0 * scale, 170.0 * scale, 42.0 * scale, 22.0 * scale, if blueprint_mode { Color::hex("#080E1C") } else { Color::hex("#18100A") });
    canvas.fill_rect(0.0, 170.0 * scale, 42.0 * scale, 22.0 * scale, if blueprint_mode { Color::hex("#080E1C") } else { Color::hex("#18100A") });

    // Torso con camisa de lino y mangas arremangadas
    let mut shirt_pb = tiny_skia::PathBuilder::new();
    shirt_pb.move_to(-44.0 * scale, -45.0 * scale);
    shirt_pb.line_to(40.0 * scale, -45.0 * scale);
    shirt_pb.line_to(36.0 * scale, 40.0 * scale);
    shirt_pb.line_to(-40.0 * scale, 40.0 * scale);
    shirt_pb.close();
    if let Some(path) = shirt_pb.finish() {
        canvas.fill_path(&path, shirt_col);
        canvas.stroke_path(&path, line_col, 2.2 * scale);
    }

    // Tirantes de cuero y cinturón portaherramientas
    canvas.stroke_line(-25.0 * scale, -45.0 * scale, -25.0 * scale, 40.0 * scale, if blueprint_mode { gold_col } else { Color::hex("#5A351C") }, 3.5 * scale);
    canvas.stroke_line(22.0 * scale, -45.0 * scale, 22.0 * scale, 40.0 * scale, if blueprint_mode { gold_col } else { Color::hex("#5A351C") }, 3.5 * scale);
    canvas.fill_rect(-42.0 * scale, 36.0 * scale, 82.0 * scale, 10.0 * scale, if blueprint_mode { gold_col } else { Color::hex("#3E200C") });

    // Brazos musculosos arremangados
    // Brazo izquierdo en jarra
    canvas.stroke_line(-40.0 * scale, -35.0 * scale, -65.0 * scale, 5.0 * scale, skin_col, 13.0 * scale);
    canvas.stroke_line(-65.0 * scale, 5.0 * scale, -38.0 * scale, 35.0 * scale, skin_col, 11.0 * scale);

    // Brazo derecho levantado empuñando la LLAVE INGLESA
    canvas.stroke_line(38.0 * scale, -35.0 * scale, 75.0 * scale, -15.0 * scale, skin_col, 13.0 * scale);
    canvas.stroke_line(75.0 * scale, -15.0 * scale, 85.0 * scale, -65.0 * scale, skin_col, 11.0 * scale);

    // Llave inglesa / llave stilson de mecánico de ferrocarril
    let wrench_x = 88.0 * scale;
    let wrench_y = -75.0 * scale;
    canvas.stroke_line(wrench_x - 10.0 * scale, wrench_y + 40.0 * scale, wrench_x + 15.0 * scale, wrench_y - 30.0 * scale, if blueprint_mode { gold_col } else { Color::hex("#68798A") }, 7.0 * scale);
    // Mordaza de la llave
    canvas.stroke_circle(wrench_x + 15.0 * scale, wrench_y - 30.0 * scale, 12.0 * scale, if blueprint_mode { gold_col } else { Color::hex("#435260") }, 4.0 * scale);

    // Cabeza, perfil mirando hacia la ventana de Meme
    let head_x = -2.0 * scale;
    let head_y = -72.0 * scale;
    canvas.fill_circle(head_x, head_y, 18.0 * scale, skin_col);
    canvas.stroke_circle(head_x, head_y, 18.0 * scale, line_col, 2.0 * scale);

    // Gorra de visera de mecánico (estilo newsboy)
    let mut cap_pb = tiny_skia::PathBuilder::new();
    cap_pb.move_to(head_x - 22.0 * scale, head_y - 5.0 * scale);
    cap_pb.cubic_to(head_x - 25.0 * scale, head_y - 25.0 * scale, head_x + 10.0 * scale, head_y - 30.0 * scale, head_x + 20.0 * scale, head_y - 12.0 * scale);
    // Visera inclinada hacia Meme
    cap_pb.line_to(head_x - 30.0 * scale, head_y - 8.0 * scale);
    cap_pb.close();
    if let Some(path) = cap_pb.finish() {
        canvas.fill_path(&path, if blueprint_mode { Color::hex("#0D1A38") } else { Color::hex("#2D3845") });
        canvas.stroke_path(&path, line_col, 1.8 * scale);
    }

    canvas.restore();

    // 3. Gran nube en espiral ascendente de MARIPOSAS AMARILLAS
    // Las mariposas vuelan en un vórtice desde Mauricio hacia la ventana de Meme
    let mut rng = Rng::new(seed);
    let butterfly_count = 22;
    for b in 0..butterfly_count {
        let t = (tau * 1.6 + b as f32 / butterfly_count as f32) % 1.0;
        // Trayectoria espiral entre Mauricio y Meme
        let start_x = mx + 20.0 * scale;
        let start_y = my + 30.0 * scale;
        let end_x = win_x;
        let end_y = win_y;

        // Curva Bezier cúbica con agitación
        let cx1 = start_x - 80.0 * scale;
        let cy1 = start_y - 140.0 * scale;
        let cx2 = end_x + 120.0 * scale;
        let cy2 = end_y + 80.0 * scale;

        let u = 1.0 - t;
        let bx = u*u*u * start_x + 3.0*u*u*t * cx1 + 3.0*u*t*t * cx2 + t*t*t * end_x;
        let by = u*u*u * start_y + 3.0*u*u*t * cy1 + 3.0*u*t*t * cy2 + t*t*t * end_y;

        let flutter = ((tau * 26.0 + b as f32 * 2.3) * PI).sin();
        let b_scale = (0.45 + (1.0 - t) * 0.45) * scale;
        let wobble_x = (tau * 12.0 + b as f32).sin() * 15.0 * scale;
        let wobble_y = (tau * 10.0 + b as f32 * 1.5).cos() * 15.0 * scale;

        draw_yellow_butterfly(canvas, bx + wobble_x, by + wobble_y, b_scale, flutter, seed + b as u32);

        // Destello dorado de polvo alar
        if rng.next_f32() > 0.4 {
            canvas.fill_circle(bx + wobble_x - 6.0 * scale, by + wobble_y + 6.0 * scale, 2.0 * scale, gold_col);
        }
    }

    canvas.restore();
}

/// Dibuja un violín barroco cremonese auténtico con cuerpo flameado, orificios en f, cordal y 4 cuerdas finas
pub fn draw_baroque_violin(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    angle: f32,
    tau: f32,
    blueprint_mode: bool,
) {
    let wood_top = if blueprint_mode { Color::hex("#1E3860") } else { Color::hex("#D9822B") };
    let wood_shade = if blueprint_mode { Color::hex("#0D1A38") } else { Color::hex("#8B4810") };
    let wood_edge = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#5A2A08") };
    let ebony = if blueprint_mode { Color::hex("#080D1A") } else { Color::hex("#1E1712") };
    let gold_string = if blueprint_mode { Color::hex("#EEF0FF") } else { Color::hex("#FFF0A0") };
    let gold_glow = if blueprint_mode { Color::hex("#80D0FF") } else { Color::hex("#FFD700") };

    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(angle);

    // Halo acústico resonante
    let pulse = (tau * 12.0 * PI).sin() * 0.08;
    canvas.stroke_circle(0.0, 30.0 * scale, 120.0 * scale * (1.0 + pulse), gold_glow.with_alpha(0.18), 1.2 * scale);

    // 1. Cuerpo del violín (caja de resonancia con lóbulos superior, C-bouts y lóbulo inferior)
    let mut body_pb = tiny_skia::PathBuilder::new();
    // Cuello unión
    body_pb.move_to(0.0, -110.0 * scale);
    // Lóbulo superior derecho
    body_pb.cubic_to(45.0 * scale, -110.0 * scale, 75.0 * scale, -80.0 * scale, 72.0 * scale, -40.0 * scale);
    // C-bout (cintura) derecha
    body_pb.cubic_to(70.0 * scale, -20.0 * scale, 48.0 * scale, -10.0 * scale, 48.0 * scale, 15.0 * scale);
    body_pb.cubic_to(48.0 * scale, 35.0 * scale, 72.0 * scale, 45.0 * scale, 78.0 * scale, 70.0 * scale);
    // Lóbulo inferior derecho
    body_pb.cubic_to(85.0 * scale, 105.0 * scale, 55.0 * scale, 145.0 * scale, 0.0, 148.0 * scale);
    // Lóbulo inferior izquierdo
    body_pb.cubic_to(-55.0 * scale, 145.0 * scale, -85.0 * scale, 105.0 * scale, -78.0 * scale, 70.0 * scale);
    // C-bout izquierdo
    body_pb.cubic_to(-72.0 * scale, 45.0 * scale, -48.0 * scale, 35.0 * scale, -48.0 * scale, 15.0 * scale);
    body_pb.cubic_to(-48.0 * scale, -10.0 * scale, -70.0 * scale, -20.0 * scale, -72.0 * scale, -40.0 * scale);
    // Lóbulo superior izquierdo
    body_pb.cubic_to(-75.0 * scale, -80.0 * scale, -45.0 * scale, -110.0 * scale, 0.0, -110.0 * scale);
    body_pb.close();

    if let Some(path) = body_pb.finish() {
        canvas.fill_path(&path, wood_top);
        canvas.stroke_path_fine(&path, wood_shade, 3.2 * scale);
        canvas.stroke_path_fine(&path, wood_edge, 1.4 * scale);

        // Fileteado perimetral doble (purfling veneciano de arce y ébano)
        canvas.stroke_path_fine(&path, gold_glow.with_alpha(0.45), 0.8 * scale);
    }

    // 2. Orificios de resonancia en 'f' tallados en la tapa armónica
    for &side in &[-1.0_f32, 1.0_f32] {
        let fx = side * 28.0 * scale;
        let mut f_pb = tiny_skia::PathBuilder::new();
        f_pb.move_to(fx - side * 2.0 * scale, -25.0 * scale);
        f_pb.cubic_to(fx + side * 10.0 * scale, -15.0 * scale, fx - side * 8.0 * scale, 20.0 * scale, fx + side * 4.0 * scale, 35.0 * scale);
        if let Some(path) = f_pb.finish() {
            canvas.stroke_path_fine(&path, ebony, 3.2 * scale);
            // Ojos circulares superior e inferior de la 'f'
            canvas.fill_circle(fx - side * 2.0 * scale, -25.0 * scale, 3.2 * scale, ebony);
            canvas.fill_circle(fx + side * 4.0 * scale, 35.0 * scale, 4.0 * scale, ebony);
        }
    }

    // 3. Cordal negro de ébano y puente de arce
    let mut tail_pb = tiny_skia::PathBuilder::new();
    tail_pb.move_to(-12.0 * scale, 142.0 * scale);
    tail_pb.line_to(12.0 * scale, 142.0 * scale);
    tail_pb.line_to(8.0 * scale, 85.0 * scale);
    tail_pb.line_to(-8.0 * scale, 85.0 * scale);
    tail_pb.close();
    if let Some(path) = tail_pb.finish() {
        canvas.fill_path(&path, ebony);
        canvas.stroke_path_fine(&path, wood_shade, 1.2 * scale);
    }
    // Puente arqueado
    canvas.stroke_line_fine(-16.0 * scale, 45.0 * scale, 16.0 * scale, 45.0 * scale, Color::hex("#FAF0D5"), 3.0 * scale);

    // 4. Mástil, diapasón largo de ébano y voluta (clavijero en caracol)
    // Diapasón sobre el cuerpo
    let mut fingerboard_pb = tiny_skia::PathBuilder::new();
    fingerboard_pb.move_to(-9.0 * scale, 50.0 * scale);
    fingerboard_pb.line_to(9.0 * scale, 50.0 * scale);
    fingerboard_pb.line_to(7.0 * scale, -180.0 * scale);
    fingerboard_pb.line_to(-7.0 * scale, -180.0 * scale);
    fingerboard_pb.close();
    if let Some(path) = fingerboard_pb.finish() {
        canvas.fill_path(&path, ebony);
        canvas.stroke_path_fine(&path, wood_edge, 1.2 * scale);
    }

    // Clavijero (pegbox)
    canvas.stroke_line_fine(-6.0 * scale, -180.0 * scale, -6.0 * scale, -230.0 * scale, wood_shade, 3.0 * scale);
    canvas.stroke_line_fine(6.0 * scale, -180.0 * scale, 6.0 * scale, -230.0 * scale, wood_shade, 3.0 * scale);
    // 4 clavijas laterales torneadas con botones
    for p in 0..4 {
        let py = -190.0 * scale - (p as f32) * 11.0 * scale;
        let side = if p % 2 == 0 { -1.0_f32 } else { 1.0_f32 };
        canvas.stroke_line_fine(0.0, py, side * 18.0 * scale, py, ebony, 2.5 * scale);
        canvas.fill_circle(side * 19.0 * scale, py, 3.5 * scale, ebony);
    }

    // Voluta barroca superior (caracol tallado)
    let mut scroll_pb = tiny_skia::PathBuilder::new();
    scroll_pb.move_to(-4.0 * scale, -230.0 * scale);
    scroll_pb.cubic_to(-12.0 * scale, -245.0 * scale, 12.0 * scale, -255.0 * scale, 14.0 * scale, -240.0 * scale);
    scroll_pb.cubic_to(14.0 * scale, -230.0 * scale, 0.0, -225.0 * scale, 0.0, -235.0 * scale);
    if let Some(path) = scroll_pb.finish() {
        canvas.stroke_path_fine(&path, wood_top, 4.0 * scale);
        canvas.stroke_path_fine(&path, wood_edge, 1.4 * scale);
    }

    // 5. Las cuatro cuerdas de tripa / plata (G, D, A, E) vibrando sutilmente
    let vib = (tau * 36.0 * PI).sin() * 0.8 * scale;
    for s in 0..4 {
        let x_nut = -4.5 * scale + (s as f32) * 3.0 * scale;
        let x_tail = -6.0 * scale + (s as f32) * 4.0 * scale;
        let x_bridge = -9.0 * scale + (s as f32) * 6.0 * scale + vib;
        let sw = 1.4 * scale - (s as f32) * 0.25 * scale;

        let mut string_pb = tiny_skia::PathBuilder::new();
        string_pb.move_to(x_nut, -180.0 * scale);
        string_pb.line_to(x_bridge, 45.0 * scale);
        string_pb.line_to(x_tail, 120.0 * scale);
        if let Some(path) = string_pb.finish() {
            canvas.stroke_path_fine(&path, gold_string, sw);
        }
    }

    // 6. El arco cruzado en diagonal con cerdas blancas
    let bow_rot = -0.38 + (tau * 8.0 * PI).sin() * 0.06;
    canvas.save();
    canvas.rotate(bow_rot);
    let bow_y = 42.0 * scale;
    // Vara de palo brasil
    canvas.stroke_line_fine(-170.0 * scale, bow_y, 170.0 * scale, bow_y, Color::hex("#7B3615"), 3.2 * scale);
    // Cerdas de crin blanca paralelas
    canvas.stroke_line_fine(-165.0 * scale, bow_y + 4.0 * scale, 165.0 * scale, bow_y + 4.0 * scale, Color::hex("#FFFBE8"), 1.6 * scale);
    // Nuez de ébano y talón de plata
    canvas.fill_circle(-160.0 * scale, bow_y + 2.0 * scale, 4.0 * scale, ebony);
    canvas.stroke_circle(-160.0 * scale, bow_y + 2.0 * scale, 4.0 * scale, Color::hex("#E8E8E8"), 1.2 * scale);
    canvas.restore();

    canvas.restore();
}


