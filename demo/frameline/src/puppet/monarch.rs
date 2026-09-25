use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::primitives::ellipse_points;
use crate::core::rng::Rng;
use crate::core::schematic::{
    draw_blueprint_header, draw_dimension_bracket, draw_magnifying_loupe, draw_node_callout,
    draw_paper_plate, draw_vector_text,
};
use std::f32::consts::PI;

const DEG: f32 = PI / 180.0;

/// Paleta de colores biológicos precisa de la Mariposa Monarca
pub struct MonarchPalette {
    pub orange_rich: Color,
    pub orange_light: Color,
    pub velvet_black: Color,
    pub vein_black: Color,
    pub spot_white: Color,
    pub thorax_black: Color,
    pub jade_chrysalis: Color,
    pub gold_suture: Color,
    pub caterpillar_yellow: Color,
    pub leaf_green: Color,
    pub leaf_vein: Color,
}

impl Default for MonarchPalette {
    fn default() -> Self {
        Self {
            orange_rich: Color::hex("#df5e18"),
            orange_light: Color::hex("#f28638"),
            velvet_black: Color::hex("#161413"),
            vein_black: Color::hex("#110f0e"),
            spot_white: Color::hex("#fcfbf7"),
            thorax_black: Color::hex("#1a1716"),
            jade_chrysalis: Color::hex("#38a872"),
            gold_suture: Color::hex("#e5be38"),
            caterpillar_yellow: Color::hex("#fad02c"),
            leaf_green: Color::hex("#689f66"),
            leaf_vein: Color::hex("#4a7a48"),
        }
    }
}

/// Dibuja la mariposa monarca completa con soporte para perspectiva alar de aleteo 3D
/// `wing_flap_angle`: ángulo de aleteo en radianes (0 = alas planas abiertas, PI*0.4 = alas elevadas)
pub fn draw_monarch(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    wing_flap_angle: f32,
    blueprint_mode: bool,
    seed: u32,
) {
    let pal = MonarchPalette::default();
    let flap_cos = wing_flap_angle.cos().abs().max(0.12);
    let flap_lift_y = -wing_flap_angle.sin() * 25.0 * scale;

    canvas.save();
    canvas.translate(cx, cy);

    let wing_fill = if blueprint_mode {
        Color::hex("#102a4a").with_alpha(0.85)
    } else {
        pal.orange_rich
    };
    let border_color = if blueprint_mode {
        Color::hex("#40a0df")
    } else {
        pal.velvet_black
    };
    let vein_color = if blueprint_mode {
        Color::hex("#80d0ff").with_alpha(0.95)
    } else {
        pal.vein_black
    };
    let spot_color = if blueprint_mode {
        Color::hex("#ffffff")
    } else {
        pal.spot_white
    };

    // Dibujar ambas alas (simetría izquierda / derecha)
    for &side in &[-1.0_f32, 1.0_f32] {
        canvas.save();
        canvas.scale(side * flap_cos, 1.0);
        canvas.translate(0.0, flap_lift_y);

        // 1. ALA POSTERIOR (Hindwing)
        draw_hindwing(canvas, scale, wing_fill, border_color, vein_color, spot_color, blueprint_mode, seed);

        // 2. ALA ANTERIOR (Forewing)
        draw_forewing(canvas, scale, wing_fill, border_color, vein_color, spot_color, blueprint_mode, seed);

        canvas.restore();
    }

    // 3. CUERPO (Tórax, abdomen, cabeza, antenas, probóscide)
    draw_monarch_body(canvas, scale, border_color, spot_color, blueprint_mode, seed);

    canvas.restore();
}

fn draw_forewing(
    canvas: &mut Canvas,
    scale: f32,
    fill_col: Color,
    border_col: Color,
    vein_col: Color,
    spot_col: Color,
    blueprint_mode: bool,
    _seed: u32,
) {
    let base_x = 10.0 * scale;
    let base_y = -18.0 * scale;

    let apex_x = 240.0 * scale;
    let apex_y = -210.0 * scale;
    let tornus_x = 190.0 * scale;
    let tornus_y = -10.0 * scale;

    // 1. Contorno anatómico del ala anterior (G5 costa, apex, termen, dorsum)
    let mut wing_pb = tiny_skia::PathBuilder::new();
    wing_pb.move_to(base_x, base_y);
    // Costa suavemente arqueada hacia el ápice
    wing_pb.cubic_to(
        base_x + 60.0 * scale,
        base_y - 130.0 * scale,
        base_x + 150.0 * scale,
        apex_y - 12.0 * scale,
        apex_x,
        apex_y,
    );
    // Ápice redondeado y termen festoneado hacia el torno
    wing_pb.cubic_to(
        apex_x + 12.0 * scale,
        apex_y + 85.0 * scale,
        tornus_x + 32.0 * scale,
        tornus_y - 65.0 * scale,
        tornus_x,
        tornus_y,
    );
    // Dorsum hacia la base
    wing_pb.cubic_to(
        tornus_x - 65.0 * scale,
        tornus_y + 6.0 * scale,
        base_x + 45.0 * scale,
        base_y + 16.0 * scale,
        base_x,
        base_y,
    );
    wing_pb.close();

    if let Some(wing_path) = wing_pb.finish() {
        canvas.fill_path(&wing_path, fill_col);

        // Borde exterior negro marginal
        let border_w = if blueprint_mode { 2.0 * scale } else { 24.0 * scale };
        canvas.stroke_path(&wing_path, border_col, border_w);

        // Ápice negro sólido triangular característico
        if !blueprint_mode {
            let mut apex_pb = tiny_skia::PathBuilder::new();
            apex_pb.move_to(apex_x, apex_y);
            apex_pb.line_to(apex_x - 70.0 * scale, apex_y + 32.0 * scale);
            apex_pb.cubic_to(
                apex_x - 50.0 * scale,
                apex_y + 65.0 * scale,
                apex_x - 15.0 * scale,
                apex_y + 105.0 * scale,
                apex_x - 10.0 * scale,
                apex_y + 115.0 * scale,
            );
            apex_pb.close();
            if let Some(apex_path) = apex_pb.finish() {
                canvas.fill_path(&apex_path, border_col);
            }
        }

        // 2. Celda discal cerrada alargada
        let discal_tip_x = base_x + 115.0 * scale;
        let discal_tip_y = base_y - 88.0 * scale;
        let mut disc_pb = tiny_skia::PathBuilder::new();
        disc_pb.move_to(base_x, base_y);
        disc_pb.cubic_to(
            base_x + 35.0 * scale,
            base_y - 58.0 * scale,
            base_x + 75.0 * scale,
            base_y - 88.0 * scale,
            discal_tip_x,
            discal_tip_y,
        );
        disc_pb.cubic_to(
            discal_tip_x - 24.0 * scale,
            discal_tip_y + 40.0 * scale,
            base_x + 42.0 * scale,
            base_y + 12.0 * scale,
            base_x,
            base_y,
        );
        if let Some(disc_path) = disc_pb.finish() {
            canvas.stroke_path(&disc_path, vein_col, 3.6 * scale);
        }

        // 3. Sistema completo de 11 venas anatómicas (R1..R5, M1..M3, Cu1..Cu2, 2A)
        let subcostals = [
            (base_x + 65.0 * scale, base_y - 65.0 * scale, apex_x - 85.0 * scale, apex_y + 10.0 * scale),
            (base_x + 85.0 * scale, base_y - 78.0 * scale, apex_x - 50.0 * scale, apex_y + 20.0 * scale),
        ];
        for (x1, y1, x2, y2) in subcostals {
            canvas.stroke_line(x1, y1, x2, y2, vein_col, 2.8 * scale);
        }

        let main_radials = [
            (apex_x - 22.0 * scale, apex_y + 40.0 * scale),
            (apex_x - 10.0 * scale, apex_y + 80.0 * scale),
            (apex_x - 5.0 * scale, apex_y + 120.0 * scale),
            (tornus_x + 28.0 * scale, tornus_y - 75.0 * scale),
            (tornus_x + 16.0 * scale, tornus_y - 40.0 * scale),
            (tornus_x + 6.0 * scale, tornus_y - 12.0 * scale),
        ];

        for (vx, vy) in main_radials {
            let mut vein_pb = tiny_skia::PathBuilder::new();
            vein_pb.move_to(discal_tip_x, discal_tip_y);
            let mid_x = (discal_tip_x + vx) * 0.5 + 4.0 * scale;
            let mid_y = (discal_tip_y + vy) * 0.5 - 4.0 * scale;
            vein_pb.cubic_to(mid_x, mid_y, mid_x, mid_y, vx, vy);
            if let Some(vp) = vein_pb.finish() {
                canvas.stroke_path(&vp, vein_col, 2.6 * scale);
            }
        }

        // Vena anal 2A desde la base al margen interno
        canvas.stroke_line(base_x + 20.0 * scale, base_y + 10.0 * scale, tornus_x - 25.0 * scale, tornus_y + 2.0 * scale, vein_col, 3.0 * scale);

        // 4. Cadenas dobles de puntos blancos festoneados
        if !blueprint_mode {
            let num_spots = 18;
            for i in 0..num_spots {
                let t = i as f32 / (num_spots - 1) as f32;
                let sx = apex_x + (tornus_x - apex_x) * t + 10.0 * scale * (t * PI).sin();
                let sy = apex_y + (tornus_y - apex_y) * t;

                // Fila exterior (borde festoneado)
                canvas.fill_circle(sx, sy, 2.8 * scale, spot_col);
                // Fila interior pareada
                if i > 1 && i < num_spots - 1 {
                    canvas.fill_circle(sx - 10.0 * scale, sy - 3.0 * scale, 1.9 * scale, spot_col);
                }
            }

            // Manchas blancas alargadas en el parche apical negro (subapical spots)
            let apical_spots = [
                (apex_x - 42.0 * scale, apex_y + 40.0 * scale, 4.0 * scale, 2.5 * scale),
                (apex_x - 28.0 * scale, apex_y + 65.0 * scale, 3.5 * scale, 2.2 * scale),
                (apex_x - 55.0 * scale, apex_y + 25.0 * scale, 3.2 * scale, 2.0 * scale),
            ];
            for (ax, ay, rx, ry) in apical_spots {
                let pts = ellipse_points(ax, ay, rx, ry, 16);
                let mut pb = tiny_skia::PathBuilder::new();
                for (k, p) in pts.iter().enumerate() {
                    if k == 0 { pb.move_to(p[0], p[1]); } else { pb.line_to(p[0], p[1]); }
                }
                pb.close();
                if let Some(p) = pb.finish() {
                    canvas.fill_path(&p, spot_col);
                }
            }
        } else {
            // Marcador técnico en modo Blueprint
            canvas.stroke_circle(discal_tip_x, discal_tip_y, 7.0 * scale, Color::hex("#C8C1EF"), 1.2);
            canvas.stroke_line(discal_tip_x - 12.0 * scale, discal_tip_y, discal_tip_x + 12.0 * scale, discal_tip_y, Color::hex("#C8C1EF"), 0.8);
        }
    }
}

fn draw_hindwing(
    canvas: &mut Canvas,
    scale: f32,
    fill_col: Color,
    border_col: Color,
    vein_col: Color,
    spot_col: Color,
    blueprint_mode: bool,
    _seed: u32,
) {
    let base_x = 8.0 * scale;
    let base_y = 6.0 * scale;

    let margin_x = 155.0 * scale;
    let margin_y = 75.0 * scale;
    let bottom_x = 85.0 * scale;
    let bottom_y = 165.0 * scale;

    let mut hw_pb = tiny_skia::PathBuilder::new();
    hw_pb.move_to(base_x, base_y);
    // Borde costal posterior
    hw_pb.cubic_to(
        base_x + 55.0 * scale,
        base_y + 6.0 * scale,
        margin_x - 12.0 * scale,
        margin_y - 28.0 * scale,
        margin_x,
        margin_y,
    );
    // Margen exterior festoneado
    hw_pb.cubic_to(
        margin_x + 18.0 * scale,
        margin_y + 50.0 * scale,
        bottom_x + 45.0 * scale,
        bottom_y + 8.0 * scale,
        bottom_x,
        bottom_y,
    );
    // Margen anal de retorno al abdomen
    hw_pb.cubic_to(
        bottom_x - 40.0 * scale,
        bottom_y - 18.0 * scale,
        base_x + 18.0 * scale,
        base_y + 65.0 * scale,
        base_x,
        base_y,
    );
    hw_pb.close();

    if let Some(hw_path) = hw_pb.finish() {
        canvas.fill_path(&hw_path, fill_col);
        let border_w = if blueprint_mode { 2.0 * scale } else { 18.0 * scale };
        canvas.stroke_path(&hw_path, border_col, border_w);

        // Celda discal posterior
        let hw_disc_x = base_x + 65.0 * scale;
        let hw_disc_y = base_y + 52.0 * scale;
        canvas.stroke_line(base_x, base_y, hw_disc_x, hw_disc_y, vein_col, 3.4 * scale);

        // Venas anatómicas radiantes (Sc, Rs, M1..M3, Cu1..Cu2, 1A)
        let hw_veins = [
            (margin_x - 8.0 * scale, margin_y + 8.0 * scale),
            (margin_x + 8.0 * scale, margin_y + 40.0 * scale),
            (bottom_x + 48.0 * scale, bottom_y - 15.0 * scale),
            (bottom_x + 22.0 * scale, bottom_y + 4.0 * scale),
            (bottom_x - 8.0 * scale, bottom_y - 4.0 * scale),
            (bottom_x - 26.0 * scale, bottom_y - 22.0 * scale),
        ];
        for (vx, vy) in hw_veins {
            canvas.stroke_line(hw_disc_x, hw_disc_y, vx, vy, vein_col, 2.4 * scale);
        }

        // Puntos blancos dobles en el margen posterior
        if !blueprint_mode {
            let num_hw_spots = 14;
            for i in 0..num_hw_spots {
                let t = i as f32 / (num_hw_spots - 1) as f32;
                let sx = margin_x + (bottom_x - margin_x) * t + 10.0 * scale * (t * PI).sin();
                let sy = margin_y + (bottom_y - margin_y) * t;
                canvas.fill_circle(sx, sy, 2.6 * scale, spot_col);
                canvas.fill_circle(sx - 8.0 * scale, sy - 3.0 * scale, 1.7 * scale, spot_col);
            }
        }
    }
}

fn draw_monarch_body(
    canvas: &mut Canvas,
    scale: f32,
    body_col: Color,
    spot_col: Color,
    blueprint_mode: bool,
    _seed: u32,
) {
    // Abdomen segmentado
    let abd_w = 14.0 * scale;
    let abd_h = 75.0 * scale;
    let abd_top_y = 5.0 * scale;
    let mut abd_pb = tiny_skia::PathBuilder::new();
    abd_pb.move_to(-abd_w * 0.5, abd_top_y);
    abd_pb.cubic_to(-abd_w * 0.7, abd_top_y + 30.0 * scale, -abd_w * 0.3, abd_top_y + abd_h, 0.0, abd_top_y + abd_h);
    abd_pb.cubic_to(abd_w * 0.3, abd_top_y + abd_h, abd_w * 0.7, abd_top_y + 30.0 * scale, abd_w * 0.5, abd_top_y);
    abd_pb.close();

    if let Some(abd_path) = abd_pb.finish() {
        canvas.fill_path(&abd_path, body_col);
        // Anillos transversales abdominales
        let segs = 7;
        for i in 1..segs {
            let sy = abd_top_y + (abd_h / segs as f32) * i as f32;
            let sw = abd_w * 0.5 * (1.0 - (i as f32 / segs as f32) * 0.5);
            let ring_col = if blueprint_mode { Color::hex("#3a8ad0") } else { Color::hex("#2a2624") };
            canvas.stroke_line(-sw, sy, sw, sy, ring_col, 1.2 * scale);
        }
    }

    // Tórax robusto
    let th_w = 18.0 * scale;
    let th_y = -22.0 * scale;
    canvas.fill_circle(0.0, th_y, th_w * 0.65, body_col);
    canvas.fill_circle(0.0, th_y + 14.0 * scale, th_w * 0.55, body_col);

    // Manchas blancas características del tórax de la monarca
    if !blueprint_mode {
        let spot_coords = [
            (-4.0 * scale, th_y - 4.0 * scale),
            (4.0 * scale, th_y - 4.0 * scale),
            (-5.0 * scale, th_y + 6.0 * scale),
            (5.0 * scale, th_y + 6.0 * scale),
            (0.0, th_y + 12.0 * scale),
        ];
        for (sx, sy) in spot_coords {
            canvas.fill_circle(sx, sy, 1.8 * scale, spot_col);
        }
    }

    // Cabeza con ojos compuestos
    let head_y = -35.0 * scale;
    canvas.fill_circle(0.0, head_y, 7.5 * scale, body_col);
    // Ojos laterales oscuros brillantes
    let eye_col = if blueprint_mode { Color::hex("#00f0ff") } else { Color::hex("#0d0c0c") };
    canvas.fill_circle(-6.5 * scale, head_y - 1.0 * scale, 4.0 * scale, eye_col);
    canvas.fill_circle(6.5 * scale, head_y - 1.0 * scale, 4.0 * scale, eye_col);

    // Antenas clavadas curvadas (con extremo bulboso)
    for &side in &[-1.0_f32, 1.0_f32] {
        let start_x = side * 3.5 * scale;
        let start_y = head_y - 5.0 * scale;
        let tip_x = side * 45.0 * scale;
        let tip_y = head_y - 65.0 * scale;

        let mut ant_pb = tiny_skia::PathBuilder::new();
        ant_pb.move_to(start_x, start_y);
        ant_pb.cubic_to(
            side * 15.0 * scale,
            head_y - 30.0 * scale,
            side * 28.0 * scale,
            head_y - 50.0 * scale,
            tip_x,
            tip_y,
        );
        if let Some(ant_path) = ant_pb.finish() {
            let ant_col = if blueprint_mode { Color::hex("#60c0ff") } else { body_col };
            canvas.stroke_path(&ant_path, ant_col, 1.6 * scale);
            // Maza / bulbo apical de la antena
            canvas.fill_circle(tip_x, tip_y, 3.2 * scale, ant_col);
        }
    }

    // Probóscide espiral enrollada (visible en vista lateral o ligeramente extendida)
    let prob_y = head_y + 6.0 * scale;
    let mut pr_pb = tiny_skia::PathBuilder::new();
    pr_pb.move_to(0.0, prob_y);
    for i in 0..12 {
        let t = i as f32 / 12.0;
        let a = t * PI * 2.5;
        let r = 5.0 * scale * (1.0 - t * 0.6);
        let px = a.cos() * r * 0.4;
        let py = prob_y + a.sin() * r;
        pr_pb.line_to(px, py);
    }
    if let Some(pr_path) = pr_pb.finish() {
        canvas.stroke_path(&pr_path, Color::hex("#2a2018"), 1.1 * scale);
    }
}

/// Dibuja la oruga monarca con sus anillos transversales característicos y tentáculos móviles
pub fn draw_caterpillar(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    crawl_tau: f32, // fase de avance peristáltico [0..1]
    _seed: u32,
) {
    let pal = MonarchPalette::default();
    let num_segments = 13;
    let seg_w = 20.0 * scale;
    let total_len = num_segments as f32 * seg_w;

    let start_x = cx - total_len * 0.5;

    // Onda peristáltica en Y
    let wave_amp = 12.0 * scale;
    let wave_freq = PI * 2.0;

    let mut body_centers = Vec::with_capacity(num_segments);
    for i in 0..num_segments {
        let t = i as f32 / (num_segments - 1) as f32;
        let seg_x = start_x + (i as f32) * seg_w;
        let seg_y = cy + (t * wave_freq - crawl_tau * PI * 2.0).sin() * wave_amp;
        body_centers.push((seg_x, seg_y));
    }

    // Dibujar cada segmento con bandas negro / blanco / amarillo
    for (i, &(sx, sy)) in body_centers.iter().enumerate() {
        let seg_r = 18.0 * scale * (0.85 + 0.15 * (1.0 - ((i as f32 - 6.0) / 6.0).powi(2)));

        // Fondo del segmento
        canvas.fill_circle(sx, sy, seg_r, pal.velvet_black);

        // Bandas concéntricas de color
        canvas.fill_circle(sx, sy, seg_r * 0.78, pal.caterpillar_yellow);
        canvas.fill_circle(sx, sy, seg_r * 0.52, pal.spot_white);
        canvas.fill_circle(sx, sy, seg_r * 0.28, pal.velvet_black);

        // Falsas patas abdominales (prolegs) en segmentos medios
        if i >= 4 && i <= 8 {
            canvas.fill_circle(sx, sy + seg_r * 0.85, 4.0 * scale, pal.velvet_black);
        }
    }

    // Cabeza en el extremo derecho (o izquierdo según orientación)
    let head_idx = num_segments - 1;
    let (hx, hy) = body_centers[head_idx];
    canvas.fill_circle(hx + 8.0 * scale, hy, 16.0 * scale, pal.velvet_black);
    // Rayas en la cápsula cefálica
    canvas.stroke_circle(hx + 8.0 * scale, hy, 11.0 * scale, pal.spot_white, 2.0 * scale);
    canvas.stroke_circle(hx + 8.0 * scale, hy, 6.0 * scale, pal.caterpillar_yellow, 2.0 * scale);

    // Filamentos anteriores largos en T2 (cerca de la cabeza)
    let t2_idx = num_segments - 3;
    let (t2x, t2y) = body_centers[t2_idx];
    let fil_sway = (crawl_tau * PI * 4.0).sin() * 15.0 * scale;
    let mut fil_front = tiny_skia::PathBuilder::new();
    fil_front.move_to(t2x, t2y - 12.0 * scale);
    fil_front.cubic_to(
        t2x + fil_sway * 0.4,
        t2y - 45.0 * scale,
        t2x + 10.0 * scale + fil_sway,
        t2y - 75.0 * scale,
        t2x + 20.0 * scale + fil_sway * 1.5,
        t2y - 95.0 * scale,
    );
    if let Some(path) = fil_front.finish() {
        canvas.stroke_path(&path, pal.velvet_black, 3.2 * scale);
    }

    // Filamentos posteriores más cortos en A8 (cerca de la cola)
    let a8_idx = 2;
    let (a8x, a8y) = body_centers[a8_idx];
    let fil_back_sway = -(crawl_tau * PI * 4.0).cos() * 10.0 * scale;
    let mut fil_back = tiny_skia::PathBuilder::new();
    fil_back.move_to(a8x, a8y - 10.0 * scale);
    fil_back.cubic_to(
        a8x + fil_back_sway * 0.3,
        a8y - 30.0 * scale,
        a8x - 10.0 * scale + fil_back_sway,
        a8y - 50.0 * scale,
        a8x - 15.0 * scale + fil_back_sway,
        a8y - 65.0 * scale,
    );
    if let Some(path) = fil_back.finish() {
        canvas.stroke_path(&path, pal.velvet_black, 2.6 * scale);
    }
}

/// Dibuja la crisálida de jade con corona de oro y opción de revelar las alas interiores
pub fn draw_chrysalis(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    reveal_wings: f32, // 0.0 = verde jade puro, 1.0 = cutícula transparente con alas oscuras
    blueprint_mode: bool,
    _seed: u32,
) {
    let pal = MonarchPalette::default();

    canvas.save();
    canvas.translate(cx, cy);

    // Rama o pedúnculo superior donde cuelga el cremáster
    canvas.stroke_line(-50.0 * scale, -125.0 * scale, 50.0 * scale, -125.0 * scale, Color::hex("#4a3828"), 6.0 * scale);
    // Almohadilla de seda blanca (silk button)
    canvas.fill_circle(0.0, -122.0 * scale, 7.0 * scale, Color::hex("#f0ede6"));
    // Cremáster negro brillante (tallo de anclaje)
    canvas.stroke_line(0.0, -122.0 * scale, 0.0, -95.0 * scale, Color::hex("#11100f"), 3.5 * scale);

    // Cuerpo en forma de pera de la crisálida
    let top_w = 42.0 * scale;
    let bot_w = 26.0 * scale;
    let len = 145.0 * scale;

    let mut ch_pb = tiny_skia::PathBuilder::new();
    ch_pb.move_to(0.0, -95.0 * scale);
    // Lado derecho
    ch_pb.cubic_to(
        top_w * 1.3,
        -70.0 * scale,
        top_w * 1.25,
        -10.0 * scale,
        bot_w * 1.1,
        len * 0.5,
    );
    ch_pb.cubic_to(
        bot_w * 0.9,
        len * 0.8,
        bot_w * 0.3,
        len,
        0.0,
        len,
    );
    // Lado izquierdo
    ch_pb.cubic_to(
        -bot_w * 0.3,
        len,
        -bot_w * 0.9,
        len * 0.8,
        -bot_w * 1.1,
        len * 0.5,
    );
    ch_pb.cubic_to(
        -top_w * 1.25,
        -10.0 * scale,
        -top_w * 1.3,
        -70.0 * scale,
        0.0,
        -95.0 * scale,
    );
    ch_pb.close();

    if let Some(ch_path) = ch_pb.finish() {
        if blueprint_mode {
            canvas.fill_path(&ch_path, Color::hex("#0a223f").with_alpha(0.85));
            canvas.stroke_path(&ch_path, Color::hex("#00f0ff"), 2.2 * scale);
        } else {
            // Relleno base verde jade
            let chrysalis_green = pal.jade_chrysalis;
            canvas.fill_path(&ch_path, chrysalis_green);

            // Si madura (reveal_wings > 0), mostrar el oscurecimiento y las alas naranjas plegadas dentro
            if reveal_wings > 0.05 {
                let dark_overlay = Color::hex("#14100c").with_alpha(reveal_wings * 0.78);
                canvas.fill_path(&ch_path, dark_overlay);

                // Patrón de venación alar visible tras la cutícula
                let orange_glow = pal.orange_rich.with_alpha(reveal_wings * 0.85);
                let mut wing_inner = tiny_skia::PathBuilder::new();
                wing_inner.move_to(0.0, -10.0 * scale);
                wing_inner.cubic_to(top_w * 0.8, 10.0 * scale, bot_w * 0.8, len * 0.7, 0.0, len * 0.9);
                wing_inner.cubic_to(-bot_w * 0.8, len * 0.7, -top_w * 0.8, 10.0 * scale, 0.0, -10.0 * scale);
                if let Some(wi_path) = wing_inner.finish() {
                    canvas.fill_path(&wi_path, orange_glow);
                    canvas.stroke_path(&wi_path, Color::hex("#0a0807"), 2.4 * scale);
                }
            }

            // Contorno suave de volumen
            canvas.stroke_path(&ch_path, Color::hex("#236c49"), 1.8 * scale);
        }

        // Corona / faja dorsal de puntos de oro (Golden suture ring)
        let ring_y = -38.0 * scale;
        let num_gold_dots = 12;
        for i in 0..num_gold_dots {
            let t = (i as f32 / (num_gold_dots - 1) as f32) * 2.0 - 1.0; // [-1..1]
            let gx = t * top_w * 0.82;
            let gy = ring_y + (1.0 - t * t) * 6.0 * scale;

            let dot_col = if blueprint_mode {
                Color::hex("#ffdd40")
            } else {
                pal.gold_suture
            };
            canvas.fill_circle(gx, gy, 2.5 * scale, dot_col);
            // Brillo especular
            if !blueprint_mode {
                canvas.fill_circle(gx - 0.7 * scale, gy - 0.7 * scale, 0.9 * scale, Color::hex("#ffffff"));
            }
        }
    }

    canvas.restore();
}

/// Dibuja un mosaico macroscópico de escamas individuales de quitina (Scale Mosaic)
pub fn draw_scale_mosaic(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    zoom_tau: f32, // [0..1] progresión de zoom
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    let pal = MonarchPalette::default();

    // Tamaño de escama según zoom
    let scale_sz = 14.0 + zoom_tau * 26.0;
    let cols = (w / (scale_sz * 0.75)) as usize + 2;
    let rows = (h / (scale_sz * 0.85)) as usize + 2;

    for r in 0..rows {
        for c in 0..cols {
            let offset_x = if r % 2 == 1 { scale_sz * 0.38 } else { 0.0 };
            let px = x + (c as f32) * (scale_sz * 0.75) + offset_x;
            let py = y + (r as f32) * (scale_sz * 0.85);

            // Patrón de zona: banda negra diagonal, zona naranja y manchas blancas
            let norm_x = (px - x) / w;
            let norm_y = (py - y) / h;
            let diag = norm_x * 0.7 + norm_y * 0.3;

            let base_col = if diag > 0.65 {
                // Zona blanca
                pal.spot_white
            } else if diag > 0.45 {
                // Banda negra marginal
                pal.velvet_black
            } else {
                // Área naranja principal con ligera variación de tono
                if rng.next_f32() > 0.35 {
                    pal.orange_rich
                } else {
                    pal.orange_light
                }
            };

            // Escama con forma de teja festoneada en la punta (scalloped shingle)
            let mut sc_pb = tiny_skia::PathBuilder::new();
            let sw = scale_sz * 0.7;
            let sh = scale_sz * 1.1;
            sc_pb.move_to(px, py);
            sc_pb.line_to(px + sw, py);
            sc_pb.line_to(px + sw * 0.95, py + sh * 0.75);
            // Dientes o muescas apicales de la escama
            sc_pb.line_to(px + sw * 0.75, py + sh);
            sc_pb.line_to(px + sw * 0.5, py + sh * 0.9);
            sc_pb.line_to(px + sw * 0.25, py + sh);
            sc_pb.line_to(px + sw * 0.05, py + sh * 0.75);
            sc_pb.close();

            if let Some(sc_path) = sc_pb.finish() {
                canvas.fill_path(&sc_path, base_col);
                canvas.stroke_path(&sc_path, Color::hex("#080605").with_alpha(0.35), 0.8);

                // Micro-estrías longitudinales de difracción de luz
                if zoom_tau > 0.3 {
                    let stria_count = 3;
                    for s in 1..=stria_count {
                        let st_x = px + sw * (s as f32 / (stria_count + 1) as f32);
                        canvas.stroke_line(st_x, py + 2.0, st_x, py + sh * 0.85, Color::hex("#ffffff").with_alpha(0.18), 0.5);
                    }
                }
            }
        }
    }
}

/// Dibuja la mariposa monarca en pose de reposo con alas cerradas en perfil (lateral view)
/// apoyada sobre la esfera floral de algodoncillo
pub fn draw_monarch_resting(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    seed: u32,
) {
    let mut _rng = Rng::new(seed);
    let s = scale;

    let ink = Color::hex("#2A1C13");
    let under_orange = Color::hex("#D9A45A");
    let deep_orange = Color::hex("#B55A1C");
    let white = Color::hex("#F6F0E2");

    canvas.save();
    canvas.translate(cx, cy);

    // Patas articuladas que se aferran a las flores
    let leg_col = Color::hex("#1a1512");
    // Pata 1
    canvas.stroke_line(-12.0 * s, 45.0 * s, -28.0 * s, 85.0 * s, leg_col, 2.8 * s);
    canvas.stroke_line(-28.0 * s, 85.0 * s, -38.0 * s, 115.0 * s, leg_col, 2.2 * s);
    // Pata 2
    canvas.stroke_line(8.0 * s, 50.0 * s, 15.0 * s, 92.0 * s, leg_col, 2.8 * s);
    canvas.stroke_line(15.0 * s, 92.0 * s, 26.0 * s, 118.0 * s, leg_col, 2.2 * s);
    // Pata 3
    canvas.stroke_line(22.0 * s, 45.0 * s, 45.0 * s, 88.0 * s, leg_col, 2.8 * s);
    canvas.stroke_line(45.0 * s, 88.0 * s, 58.0 * s, 112.0 * s, leg_col, 2.2 * s);

    // Ala anterior asomando detrás (apex)
    let mut fore_pb = tiny_skia::PathBuilder::new();
    fore_pb.move_to(-8.0 * s, 10.0 * s);
    fore_pb.cubic_to(-35.0 * s, -80.0 * s, -50.0 * s, -180.0 * s, -20.0 * s, -260.0 * s); // costa
    fore_pb.cubic_to(20.0 * s, -230.0 * s, 65.0 * s, -160.0 * s, 50.0 * s, -70.0 * s);   // margen
    fore_pb.close();
    if let Some(p) = fore_pb.finish() {
        canvas.fill_path(&p, deep_orange);
        // Zona apical negra
        let mut apex_pb = tiny_skia::PathBuilder::new();
        apex_pb.move_to(-20.0 * s, -260.0 * s);
        apex_pb.cubic_to(0.0, -250.0 * s, 25.0 * s, -220.0 * s, 35.0 * s, -180.0 * s);
        apex_pb.cubic_to(10.0 * s, -170.0 * s, -15.0 * s, -190.0 * s, -20.0 * s, -260.0 * s);
        apex_pb.close();
        if let Some(ap) = apex_pb.finish() {
            canvas.fill_path(&ap, ink);
            // Manchas blancas apicales
            canvas.fill_circle(-5.0 * s, -230.0 * s, 4.0 * s, white);
            canvas.fill_circle(12.0 * s, -205.0 * s, 5.0 * s, white);
        }
        canvas.stroke_path(&p, ink, 2.5 * s);
    }

    // Ala posterior cerrada en reposo (cara ventral/underside)
    let mut hind_pb = tiny_skia::PathBuilder::new();
    hind_pb.move_to(-5.0 * s, 20.0 * s);
    hind_pb.cubic_to(-25.0 * s, -30.0 * s, -35.0 * s, -110.0 * s, 0.0, -170.0 * s);
    hind_pb.cubic_to(55.0 * s, -160.0 * s, 115.0 * s, -100.0 * s, 125.0 * s, -20.0 * s);
    hind_pb.cubic_to(110.0 * s, 40.0 * s, 65.0 * s, 85.0 * s, 15.0 * s, 80.0 * s);
    hind_pb.close();
    if let Some(hp) = hind_pb.finish() {
        canvas.fill_path(&hp, under_orange);

        // Venación ventral característica
        let veins = [
            (-5.0, 20.0, 15.0, -90.0, 110.0, -20.0),
            (-5.0, 20.0, 35.0, -60.0, 85.0, 45.0),
            (-5.0, 20.0, 0.0, -70.0, 45.0, -165.0),
            (15.0, -90.0, 50.0, -125.0, 85.0, -100.0),
            (35.0, -60.0, 65.0, -10.0, 105.0, 15.0),
        ];
        for &(x0, y0, cx0, cy0, x1, y1) in &veins {
            let mut vp = tiny_skia::PathBuilder::new();
            vp.move_to(x0 * s, y0 * s);
            vp.quad_to(cx0 * s, cy0 * s, x1 * s, y1 * s);
            if let Some(v_path) = vp.finish() {
                canvas.stroke_path(&v_path, ink, 3.2 * s);
            }
        }

        // Borde marginal negro festoneado con doble hilera de puntos blancos
        canvas.stroke_path(&hp, ink, 14.0 * s);
        // Doble hilera de puntos blancos marginales
        let spot_pts = [
            (115.0, -30.0), (118.0, -10.0), (110.0, 15.0), (95.0, 40.0), (75.0, 60.0), (45.0, 75.0),
            (105.0, -30.0), (108.0, -10.0), (100.0, 12.0), (85.0, 35.0), (65.0, 52.0), (38.0, 68.0),
        ];
        for &(sx, sy) in &spot_pts {
            canvas.fill_circle(sx * s, sy * s, 2.4 * s, white);
        }
        canvas.stroke_path(&hp, ink, 2.5 * s);
    }

    // Tórax y abdomen
    let mut body_pb = tiny_skia::PathBuilder::new();
    body_pb.move_to(-12.0 * s, 10.0 * s);
    body_pb.cubic_to(-18.0 * s, 30.0 * s, -15.0 * s, 55.0 * s, -5.0 * s, 70.0 * s);
    body_pb.cubic_to(5.0 * s, 55.0 * s, 5.0 * s, 30.0 * s, 0.0, 10.0 * s);
    body_pb.close();
    if let Some(bp) = body_pb.finish() {
        canvas.fill_path(&bp, ink);
        // Puntos blancos en el tórax
        for y_off in [20.0, 35.0, 50.0] {
            canvas.fill_circle(-8.0 * s, y_off * s, 2.0 * s, white);
            canvas.fill_circle(-2.0 * s, (y_off + 4.0) * s, 1.8 * s, white);
        }
    }

    // Cabeza, ojo compuesto y antena curvada
    canvas.fill_circle(-16.0 * s, 5.0 * s, 9.0 * s, ink);
    canvas.fill_circle(-18.0 * s, 4.0 * s, 5.0 * s, Color::hex("#120d0a")); // ojo
    // Antena con maza apical
    let mut ant_pb = tiny_skia::PathBuilder::new();
    ant_pb.move_to(-20.0 * s, 0.0);
    ant_pb.cubic_to(-45.0 * s, -20.0 * s, -65.0 * s, -50.0 * s, -70.0 * s, -85.0 * s);
    if let Some(ap) = ant_pb.finish() {
        canvas.stroke_path(&ap, ink, 2.0 * s);
        canvas.fill_circle(-70.0 * s, -85.0 * s, 4.0 * s, ink); // maza
    }

    canvas.restore();
}

/// Dibuja la inflorescencia esférica botánica de algodoncillo (Asclepias syriaca)
/// con corona de flores rosas/malva y hojas con nervadura lateral
pub fn draw_botanical_milkweed_ball(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    ball_radius: f32,
    stem_bottom: f32,
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    let stem_col = Color::hex("#7E9A63");
    let leaf_col = Color::hex("#9DB08A");
    let leaf_vein = Color::hex("#D5DEC4");
    let ink = Color::hex("#2A1C13");
    let flower_petal = Color::hex("#CF8FA0");
    let flower_hood = Color::hex("#A9607A");

    // Tallo principal botánico
    canvas.stroke_line(cx, cy + ball_radius * 0.7, cx, stem_bottom, stem_col, 16.0);
    canvas.stroke_line(cx - 7.0, cy + ball_radius * 0.7, cx - 7.0, stem_bottom, Color::hex("#6F8A5E"), 2.0);
    canvas.stroke_line(cx + 7.0, cy + ball_radius * 0.7, cx + 7.0, stem_bottom, ink.with_alpha(0.35), 1.4);

    // Hojas botánicas opuestas cuádruples (estándar G5 / leafGeom)
    // Par inferior: hojas anchas que van al borde inferior (ancho 270px)
    // Par superior: hojas medias (ancho 200px), hoja superior derecha con ápice doblado revelando envés pálido
    let leaves_def = [
        // (bx, by, tx, ty, width, is_upper_right_folded)
        (527.0_f32, 1842.0_f32, 70.0_f32, 1816.0_f32, 270.0_f32, false),
        (553.0_f32, 1842.0_f32, 1010.0_f32, 1826.0_f32, 270.0_f32, false),
        (527.0_f32, 1522.0_f32, 110.0_f32, 1370.0_f32, 200.0_f32, false),
        (553.0_f32, 1522.0_f32, 970.0_f32, 1360.0_f32, 200.0_f32, true),
    ];

    for &(bx, by, tx, ty, width, is_folded) in &leaves_def {
        let dx = tx - bx;
        let dy = ty - by;
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let ux = dx / len;
        let uy = dy / len;
        let mut nx = -uy;
        let mut ny = ux;
        if ny > 0.0 {
            nx = -nx;
            ny = -ny;
        }

        let pet = 38.0;
        let blade = len - pet;
        let hw = width * 0.5;

        // Puntos de silueta de la hoja
        let n_steps = 32;
        let mut upper_pts = Vec::with_capacity(n_steps + 1);
        let mut lower_pts = Vec::with_capacity(n_steps + 1);
        let mut mid_pts = Vec::with_capacity(n_steps + 1);

        for i in 0..=n_steps {
            let u = i as f32 / n_steps as f32;
            let shape = (PI * u.clamp(0.0, 1.0).powf(0.78)).sin().powf(0.72);
            let bend = 16.0 * 4.0 * u * (1.0 - u);
            let dist = pet + u * blade;

            let mx = bx + ux * dist + nx * bend;
            let my = by + uy * dist + ny * bend;
            mid_pts.push((mx, my));

            let cur_hw = hw * shape;
            upper_pts.push((mx + nx * cur_hw, my + ny * cur_hw));
            lower_pts.push((mx - nx * cur_hw * 0.96, my - ny * cur_hw * 0.96));
        }

        // Trazado y relleno de la lámina foliar
        let mut leaf_pb = tiny_skia::PathBuilder::new();
        leaf_pb.move_to(upper_pts[0].0, upper_pts[0].1);
        for pt in upper_pts.iter().skip(1) {
            leaf_pb.line_to(pt.0, pt.1);
        }
        for pt in lower_pts.iter().rev() {
            leaf_pb.line_to(pt.0, pt.1);
        }
        leaf_pb.close();

        if let Some(lp) = leaf_pb.finish() {
            canvas.fill_path(&lp, leaf_col);
            canvas.stroke_path(&lp, ink, 2.6);

            // Nervadura principal (midrib)
            let mut mp_pb = tiny_skia::PathBuilder::new();
            mp_pb.move_to(mid_pts[0].0, mid_pts[0].1);
            for pt in mid_pts.iter().skip(1) {
                mp_pb.line_to(pt.0, pt.1);
            }
            if let Some(mp) = mp_pb.finish() {
                canvas.stroke_path(&mp, leaf_vein, 4.5);
                canvas.stroke_path(&mp, ink.with_alpha(0.35), 1.2);
            }

            // 14 pares de nervaduras secundarias que parten casi perpendiculares y se arquean al ápice
            for k in 0..14 {
                let u0 = 0.08 + (k as f32) * 0.062;
                if u0 >= 0.94 {
                    break;
                }
                let idx = (u0 * n_steps as f32).round() as usize;
                let mid_p = mid_pts[idx.min(n_steps)];
                let up_p = upper_pts[idx.min(n_steps)];
                let lo_p = lower_pts[idx.min(n_steps)];

                canvas.stroke_line(mid_p.0, mid_p.1, mid_p.0 + (up_p.0 - mid_p.0) * 0.82, mid_p.1 + (up_p.1 - mid_p.1) * 0.82, leaf_vein, 2.0);
                canvas.stroke_line(mid_p.0, mid_p.1, mid_p.0 + (lo_p.0 - mid_p.0) * 0.82, mid_p.1 + (lo_p.1 - mid_p.1) * 0.82, leaf_vein, 2.0);
            }

            // Si es la hoja superior derecha con ápice plegado, dibujar el doblez mostrando el envés
            if is_folded {
                let fold_start = (n_steps as f32 * 0.65).round() as usize;
                let mut fold_pb = tiny_skia::PathBuilder::new();
                fold_pb.move_to(mid_pts[fold_start].0, mid_pts[fold_start].1);
                for i in fold_start..=n_steps {
                    fold_pb.line_to(upper_pts[i].0, upper_pts[i].1);
                }
                fold_pb.line_to(mid_pts[n_steps].0, mid_pts[n_steps].1);
                fold_pb.close();
                if let Some(fp) = fold_pb.finish() {
                    canvas.fill_path(&fp, Color::hex("#B9CF94")); // Envés pálido
                    canvas.stroke_path(&fp, ink, 2.0);
                }
            }
        }
    }

    // Receptáculo de pedicelos radiantes de la umbela
    let umbel_y = cy + ball_radius * 0.25;
    let num_florets = 42;
    for i in 0..num_florets {
        let a = (i as f32 / num_florets as f32) * PI * 2.0;
        let r = ball_radius * (0.45 + 0.55 * rng.next_f32());
        let fx = cx + a.cos() * r;
        let fy = cy + a.sin() * r * 0.85;

        // Pedicelo verde
        canvas.stroke_line(cx, umbel_y, fx, fy, Color::hex("#6F8A5E").with_alpha(0.75), 1.6);

        // Flor individual: 5 pétalos reflexos y 5 capuchones erectos (hoods)
        for p in 0..5 {
            let pa = (p as f32 / 5.0) * PI * 2.0 + (i as f32 * 0.4);
            let px = fx + pa.cos() * 11.0;
            let py = fy + pa.sin() * 11.0;
            // Pétalo
            canvas.fill_circle(px, py, 6.5, flower_petal);
            canvas.stroke_circle(px, py, 6.5, ink.with_alpha(0.4), 0.8);
        }
        // Centro y corona de capuchones
        canvas.fill_circle(fx, fy, 8.0, flower_hood);
        canvas.fill_circle(fx, fy, 4.0, Color::hex("#F0D9B5"));
        canvas.stroke_circle(fx, fy, 8.0, ink.with_alpha(0.6), 1.0);
    }
}

/// Dibuja el huevo acanalado biológico G1 (Chorion) con 18 costillas keeled y 34 peldaños
pub fn draw_ribbed_egg_chorion(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    is_blueprint: bool,
) {
    let egg_col = if is_blueprint { Color::hex("#0B1230") } else { Color::hex("#EFE4C6") };
    let line_col = if is_blueprint { Color::hex("#C8C1EF") } else { Color::hex("#2A1C13") };
    let rib_col = if is_blueprint { Color::hex("#C8C1EF").with_alpha(0.75) } else { Color::hex("#5B4331") };
    let rung_col = if is_blueprint { Color::hex("#8A735C").with_alpha(0.4) } else { Color::hex("#8A735C").with_alpha(0.55) };

    let top_y = cy - h * 0.5;
    let bot_y = cy + h * 0.5;

    // Sección de tejido foliar superior (en el envés del cual cuelga el huevo)
    if is_blueprint {
        // En blueprint: corte histológico de hoja de algodoncillo
        let leaf_y0 = top_y - 120.0;
        let leaf_y1 = top_y;
        canvas.stroke_line(cx - 380.0, leaf_y0, cx + 380.0, leaf_y0, line_col.with_alpha(0.6), 1.4);
        canvas.stroke_line(cx - 380.0, leaf_y1, cx + 380.0, leaf_y1, line_col.with_alpha(0.9), 2.2);
        // Células en empalizada (palisade mesophyll)
        let mut px = cx - 360.0;
        while px <= cx + 360.0 {
            canvas.stroke_rect(px, leaf_y0 + 8.0, 16.0, 48.0, rib_col, 0.8);
            px += 19.0;
        }
        // Tricomas
        for tx in [-240.0, -180.0, -120.0, 120.0, 180.0, 240.0] {
            canvas.stroke_line(cx + tx, leaf_y1, cx + tx + 14.0, leaf_y1 + 24.0, line_col.with_alpha(0.5), 1.2);
        }
    } else {
        // En modo papel: envés verde con nervadura
        let mut lf_pb = tiny_skia::PathBuilder::new();
        lf_pb.move_to(0.0, 0.0);
        lf_pb.line_to(canvas.width as f32, 0.0);
        lf_pb.line_to(canvas.width as f32, top_y);
        lf_pb.cubic_to(cx + 200.0, top_y + 10.0, cx - 200.0, top_y - 10.0, 0.0, top_y);
        lf_pb.close();
        if let Some(lp) = lf_pb.finish() {
            canvas.fill_path(&lp, Color::hex("#9DB08A"));
            canvas.stroke_line(0.0, top_y - 60.0, canvas.width as f32, top_y - 30.0, Color::hex("#D5DEC4"), 5.0);
        }
    }

    // Silueta abovedada del huevo con base plana adherida a la hoja
    let mut egg_pb = tiny_skia::PathBuilder::new();
    egg_pb.move_to(cx, top_y);
    egg_pb.cubic_to(cx + w * 0.9, top_y + h * 0.25, cx + w * 0.95, bot_y - h * 0.25, cx, bot_y);
    egg_pb.cubic_to(cx - w * 0.95, bot_y - h * 0.25, cx - w * 0.9, top_y + h * 0.25, cx, top_y);
    egg_pb.close();

    if let Some(egg_path) = egg_pb.finish() {
        canvas.fill_path(&egg_path, egg_col);

        // 18 costillas longitudinales curvadas de polo a polo
        let num_ribs = 18;
        for i in 1..num_ribs {
            let frac = (i as f32 / num_ribs as f32) * 2.0 - 1.0;
            let rx = cx + frac * w * 0.88;
            let mut r_pb = tiny_skia::PathBuilder::new();
            r_pb.move_to(cx, top_y);
            r_pb.cubic_to(rx * 1.05, top_y + h * 0.35, rx, bot_y - h * 0.35, cx, bot_y);
            if let Some(rp) = r_pb.finish() {
                canvas.stroke_path(&rp, rib_col, 1.2);
            }
        }

        // 34 peldaños horizontales transversales (ladder rungs)
        let num_rungs = 34;
        for s in 1..num_rungs {
            let sy = top_y + (h / (num_rungs as f32)) * (s as f32);
            let norm_h = (s as f32 / num_rungs as f32);
            let span = w * (norm_h * PI).sin().powf(0.65) * 0.9;
            canvas.stroke_line(cx - span, sy, cx + span, sy, rung_col, 0.9);
        }

        // Línea de contorno doble auténtica
        canvas.stroke_path(&egg_path, line_col, 3.2);
        if is_blueprint {
            // Contorno interior concéntrico a 5px
            canvas.stroke_path(&egg_path, line_col.with_alpha(0.4), 1.0);
        }
    }

    // Micrópilo (roseta apical)
    canvas.stroke_circle(cx, bot_y, 8.0, line_col, 1.5);
    canvas.fill_circle(cx, bot_y, 3.5, if is_blueprint { Color::hex("#FF3D98") } else { Color::hex("#2A1C13") });
}

/// Dibuja el mapa histórico de la ruta migratoria de Norteamérica (Continental Flyways)
pub fn draw_north_america_migration_map(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    scale: f32,
    progress_tau: f32,
) {
    let paper = Color::hex("#EFE3C9");
    let ink = Color::hex("#2A1C13");
    let ink_faint = Color::hex("#8A735C").with_alpha(0.4);
    let teal = Color::hex("#3C8783");
    let arrow_col = Color::hex("#B55A1C");

    canvas.save();
    canvas.translate(cx, cy);

    // Fondo cartográfico pergamino a sangre completa
    canvas.fill_rect(-cx, -cy, canvas.width as f32, canvas.height as f32, paper);

    // Océano Atlántico: aguada turquesa suave al este de la costa
    let mut ocean_pb = tiny_skia::PathBuilder::new();
    ocean_pb.move_to(320.0 * scale, -cy);
    ocean_pb.cubic_to(280.0 * scale, -480.0 * scale, 210.0 * scale, -320.0 * scale, 180.0 * scale, -60.0 * scale);
    ocean_pb.cubic_to(185.0 * scale, 20.0 * scale, 195.0 * scale, 110.0 * scale, 170.0 * scale, 160.0 * scale);
    ocean_pb.cubic_to(100.0 * scale, -10.0 * scale, 30.0 * scale, 15.0 * scale, -40.0 * scale, 45.0 * scale);
    ocean_pb.cubic_to(-90.0 * scale, 70.0 * scale, -130.0 * scale, 120.0 * scale, -120.0 * scale, 200.0 * scale);
    ocean_pb.cubic_to(-110.0 * scale, 260.0 * scale, -70.0 * scale, 320.0 * scale, -30.0 * scale, 340.0 * scale);
    ocean_pb.line_to(canvas.width as f32 - cx, 340.0 * scale);
    ocean_pb.line_to(canvas.width as f32 - cx, -cy);
    ocean_pb.close();
    if let Some(op) = ocean_pb.finish() {
        canvas.fill_path(&op, Color::hex("#C9D3D2").with_alpha(0.65));
        // Líneas de hachura horizontal en el océano
        let mut oy = -cy + 20.0;
        while oy <= 340.0 * scale {
            canvas.stroke_line(150.0 * scale, oy, canvas.width as f32 - cx, oy, teal.with_alpha(0.25), 1.0);
            oy += 18.0;
        }
    }

    // Líneas de graticula náutica (meridianos y paralelos cada 10 grados)
    for gy in [-500.0, -300.0, -100.0, 100.0, 300.0, 500.0] {
        canvas.stroke_line(-cx, gy * scale, canvas.width as f32 - cx, gy * scale, ink_faint, 0.8);
    }
    for gx in [-350.0, -175.0, 0.0, 175.0, 350.0] {
        canvas.stroke_line(gx * scale, -cy, gx * scale, canvas.height as f32 - cy, ink_faint, 0.8);
    }

    // Líneas de costa del este de Norteamérica, Golfo de México y Península de Florida
    let mut coast_pb = tiny_skia::PathBuilder::new();
    // Nueva Escocia y costa atlántica
    coast_pb.move_to(320.0 * scale, -550.0 * scale);
    coast_pb.cubic_to(280.0 * scale, -480.0 * scale, 240.0 * scale, -420.0 * scale, 210.0 * scale, -320.0 * scale); // Boston / NY
    coast_pb.cubic_to(190.0 * scale, -240.0 * scale, 170.0 * scale, -150.0 * scale, 180.0 * scale, -60.0 * scale);  // Carolinas
    // Península de Florida
    coast_pb.cubic_to(185.0 * scale, 20.0 * scale, 195.0 * scale, 110.0 * scale, 170.0 * scale, 160.0 * scale);   // Miami
    coast_pb.cubic_to(150.0 * scale, 160.0 * scale, 140.0 * scale, 90.0 * scale, 135.0 * scale, 20.0 * scale);     // West Florida
    // Golfo de México hasta Texas
    coast_pb.cubic_to(100.0 * scale, -10.0 * scale, 30.0 * scale, 15.0 * scale, -40.0 * scale, 45.0 * scale);      // Delta Mississippi
    coast_pb.cubic_to(-90.0 * scale, 70.0 * scale, -130.0 * scale, 120.0 * scale, -120.0 * scale, 200.0 * scale); // Texas coast
    // México y Yucatán
    coast_pb.cubic_to(-110.0 * scale, 260.0 * scale, -70.0 * scale, 320.0 * scale, -30.0 * scale, 340.0 * scale);  // Veracruz
    coast_pb.cubic_to(20.0 * scale, 340.0 * scale, 60.0 * scale, 310.0 * scale, 90.0 * scale, 280.0 * scale);     // Yucatán
    if let Some(cp) = coast_pb.finish() {
        // Trazado de costa con hachurado de aguas
        canvas.stroke_path(&cp, ink, 3.2 * scale);
        canvas.stroke_path(&cp, teal.with_alpha(0.6), 8.0 * scale);
    }

    // Grandes Lagos (Superior, Michigan, Huron, Erie, Ontario) con rayado aguamarina
    let lakes = [
        (-40.0 * scale, -360.0 * scale, 55.0 * scale, 30.0 * scale), // Superior
        (-10.0 * scale, -290.0 * scale, 28.0 * scale, 55.0 * scale), // Michigan
        (35.0 * scale, -310.0 * scale, 38.0 * scale, 45.0 * scale),  // Huron
        (75.0 * scale, -265.0 * scale, 42.0 * scale, 20.0 * scale),  // Erie
        (120.0 * scale, -270.0 * scale, 35.0 * scale, 18.0 * scale), // Ontario
    ];
    for &(lx, ly, lw, _lh) in &lakes {
        canvas.fill_circle(lx, ly, lw * 0.5, teal.with_alpha(0.35));
        canvas.stroke_circle(lx, ly, lw * 0.5, teal, 1.6);
        // Rayado horizontal
        for row in [-8.0, 0.0, 8.0] {
            canvas.stroke_line(lx - lw * 0.35, ly + row * scale, lx + lw * 0.35, ly + row * scale, teal, 1.0);
        }
    }

    // Cadenas montañosas (Apalaches y Sierra Madre) con pequeños picos grabados
    let peaks = [
        (130.0, -200.0), (110.0, -140.0), (90.0, -80.0), (70.0, -20.0), // Apalaches
        (-150.0, 180.0), (-140.0, 240.0), (-130.0, 300.0), (-110.0, 360.0), // Sierra Madre Oriental
    ];
    for &(px, py) in &peaks {
        let hx = px * scale;
        let hy = py * scale;
        canvas.stroke_line(hx - 12.0 * scale, hy + 8.0 * scale, hx, hy - 14.0 * scale, ink, 1.4);
        canvas.stroke_line(hx, hy - 14.0 * scale, hx + 12.0 * scale, hy + 8.0 * scale, ink, 1.4);
    }

    // Rutas migratorias: flechas discontinuas que convergen desde Ontario y Maine hacia Michoacán
    let target_michoacan = (-90.0 * scale, 380.0 * scale);
    canvas.fill_circle(target_michoacan.0, target_michoacan.1, 7.0 * scale, Color::hex("#FF3D98"));
    canvas.stroke_circle(target_michoacan.0, target_michoacan.1, 16.0 * scale, Color::hex("#FF3D98"), 1.8);
    draw_vector_text(canvas, target_michoacan.0 + 24.0 * scale, target_michoacan.1 + 5.0 * scale, "MICHOACAN (2,800-3,200m)", 11.0, ink, false);

    let origins = [
        (80.0 * scale, -380.0 * scale),  // Canadá Este / Ontario
        (220.0 * scale, -350.0 * scale), // Nueva Inglaterra
        (-70.0 * scale, -280.0 * scale), // Medio Oeste
    ];

    let max_step = (progress_tau * 36.0) as usize;
    for &(ox, oy) in &origins {
        let steps = 30;
        for s in 0..steps.min(max_step) {
            let t0 = s as f32 / steps as f32;
            let t1 = (s + 1) as f32 / steps as f32;
            // Curva suave parabólica que canaliza a través de Texas
            let x0 = ox + (target_michoacan.0 - ox) * t0 + (t0 * PI).sin() * (-40.0 * scale);
            let y0 = oy + (target_michoacan.1 - oy) * t0;
            let x1 = ox + (target_michoacan.0 - ox) * t1 + (t1 * PI).sin() * (-40.0 * scale);
            let y1 = oy + (target_michoacan.1 - oy) * t1;

            if s % 2 == 0 {
                canvas.stroke_line(x0, y0, x1, y1, arrow_col, 2.5 * scale);
            }
        }
    }

    // Rosa de los vientos grabada en el Atlántico
    let rose_x = 320.0 * scale;
    let rose_y = 60.0 * scale;
    canvas.stroke_circle(rose_x, rose_y, 45.0 * scale, ink.with_alpha(0.6), 1.2);
    canvas.stroke_circle(rose_x, rose_y, 36.0 * scale, ink.with_alpha(0.4), 0.8);
    for i in 0..8 {
        let a = i as f32 * PI * 0.25;
        let r_out = if i % 2 == 0 { 42.0 } else { 28.0 } * scale;
        canvas.stroke_line(rose_x, rose_y, rose_x + a.cos() * r_out, rose_y + a.sin() * r_out, ink, if i % 2 == 0 { 2.0 } else { 1.0 });
    }
    draw_vector_text(canvas, rose_x, rose_y - 50.0 * scale, "N", 11.0, ink, true);

    canvas.restore();
}

/// Dibuja el santuario invernal de abetos sagrados Oyamel (Abies religiosa) con enjambres dormidos
pub fn draw_oyamel_winter_roost(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    let trunk_col = Color::hex("#4A3828");
    let cluster_gold = Color::hex("#C38F2E");
    let cluster_orange = Color::hex("#D9772B");
    let ink = Color::hex("#2A1C13");
    let mist_col = Color::hex("#CFC6E0").with_alpha(0.28);

    // 4 grandes troncos verticales de abeto Oyamel
    let trunks = [
        (cx - width * 0.32, 75.0_f32),
        (cx - width * 0.11, 85.0_f32),
        (cx + width * 0.12, 80.0_f32),
        (cx + width * 0.34, 70.0_f32),
    ];

    for &(tx, tw) in &trunks {
        // Tronco
        canvas.fill_rect(tx - tw * 0.5, 0.0, tw, height, trunk_col);
        canvas.stroke_line(tx - tw * 0.5, 0.0, tx - tw * 0.5, height, ink, 2.0);
        canvas.stroke_line(tx + tw * 0.5, 0.0, tx + tw * 0.5, height, ink, 2.0);

        // Fisuras de corteza vertical
        for fx in [-0.3, 0.0, 0.28] {
            canvas.stroke_line(tx + tw * fx, 0.0, tx + tw * fx, height, Color::hex("#2E2218"), 1.5);
        }

        // Ramas con agujas de abeto
        for by in [cy - 300.0, cy - 100.0, cy + 120.0, cy + 320.0] {
            let side = if rng.next_f32() > 0.5 { 1.0 } else { -1.0 };
            canvas.stroke_line(tx, by, tx + side * 90.0, by + 40.0, Color::hex("#2E2218"), 4.0);
            for n in 1..=8 {
                let nx = tx + side * (n as f32 * 10.0);
                let ny = by + (n as f32 * 4.5);
                canvas.stroke_line(nx, ny, nx + side * 15.0, ny + 15.0, Color::hex("#3E5C54"), 1.8);
            }
        }

        // Racimos densos de mariposas dormidas colgando del tronco
        for ry in [cy - 220.0, cy, cy + 240.0] {
            let cl_w = tw * 1.5;
            let cl_h = 130.0;
            let mut cl_pb = tiny_skia::PathBuilder::new();
            cl_pb.move_to(tx - cl_w * 0.5, ry);
            cl_pb.cubic_to(tx - cl_w * 0.65, ry + cl_h * 0.4, tx - cl_w * 0.2, ry + cl_h, tx, ry + cl_h);
            cl_pb.cubic_to(tx + cl_w * 0.2, ry + cl_h, tx + cl_w * 0.65, ry + cl_h * 0.4, tx + cl_w * 0.5, ry);
            cl_pb.close();

            if let Some(cp) = cl_pb.finish() {
                canvas.fill_path(&cp, cluster_gold);
                canvas.stroke_path(&cp, ink, 2.4);

                // Cientos de pequeñas escamas/alas apiñadas dentro del racimo
                for _ in 0..45 {
                    let rx = tx + (rng.next_f32() - 0.5) * cl_w * 0.8;
                    let ry_pos = ry + rng.next_f32() * cl_h * 0.9;
                    canvas.fill_circle(rx, ry_pos, 5.0, cluster_orange);
                    canvas.stroke_circle(rx, ry_pos, 5.0, ink.with_alpha(0.6), 0.8);
                }
            }
        }
    }

    // Capas de niebla y neblina fría de montaña a ras de suelo
    for my in [height - 350.0, height - 220.0, height - 90.0] {
        canvas.fill_rect(0.0, my, width, 110.0, mist_col);
    }
}

// =============================================================================
// COMPONENTES MAESTROS DE ALTA FIDELIDAD (Estándar Kevin Ngo / procedural-film)
// =============================================================================

/// Dibuja la oruga de primer estadio eclosionando del huevo con mordiscos en el corion
pub fn draw_egg_hatch_caterpillar(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    tau: f32,
    seed: u32,
) {
    let ink = Color::hex("#2A1C13");
    let larva_col = Color::hex("#CBD3BC"); // Verde oliva translúcido
    let head_col = Color::hex("#17110D");  // Cabeza negra brillante
    let suture_col = Color::hex("#E6BE3A");

    // Cuerpo de la larva curvándose alrededor del cascarón
    let num_pts = 24;
    let mut body_pts = Vec::with_capacity(num_pts);
    let head_target_x = cx + 180.0;
    let head_target_y = cy + 120.0;

    for i in 0..num_pts {
        let u = i as f32 / (num_pts - 1) as f32;
        // Trayectoria en arco C envolviendo el cascarón
        let a = PI * 0.4 + u * PI * 0.85;
        let r = 160.0 + (1.0 - u) * 40.0;
        let px = cx + a.cos() * r;
        let py = cy + a.sin() * r * 0.9;
        body_pts.push((px, py));
    }

    // Trazo del cuerpo con segmentos sombreados
    for i in 0..num_pts - 1 {
        let (x0, y0) = body_pts[i];
        let (x1, y1) = body_pts[i + 1];
        let u = i as f32 / (num_pts - 1) as f32;
        let seg_r = 16.0 + u * 12.0;

        canvas.stroke_line(x0, y0, x1, y1, larva_col, seg_r * 2.0);
        canvas.stroke_line(x0, y0, x1, y1, ink.with_alpha(0.6), seg_r * 2.0 + 3.0);
        canvas.stroke_line(x0, y0, x1, y1, larva_col, seg_r * 2.0);

        // Anillos segmentarios oscuros
        if i % 2 == 0 {
            let dx = x1 - x0;
            let dy = y1 - y0;
            let len = (dx * dx + dy * dy).sqrt().max(1.0);
            let nx = -dy / len;
            let ny = dx / len;
            canvas.stroke_line(
                x0 - nx * seg_r,
                y0 - ny * seg_r,
                x0 + nx * seg_r,
                y0 + ny * seg_r,
                ink.with_alpha(0.75),
                2.0,
            );
        }
    }

    // Cabeza negra brillante masticando el borde
    let (hx, hy) = *body_pts.last().unwrap();
    canvas.fill_circle(hx, hy, 28.0, head_col);
    canvas.stroke_circle(hx, hy, 28.0, ink, 2.8);

    // Sutura en Y invertida de la cápsula cefálica
    canvas.stroke_line(hx, hy - 20.0, hx, hy, suture_col, 2.2);
    canvas.stroke_line(hx, hy, hx - 12.0, hy + 16.0, suture_col, 2.2);
    canvas.stroke_line(hx, hy, hx + 12.0, hy + 16.0, suture_col, 2.2);

    // Mandíbulas masticando el corion
    let jaw_cycle = (tau * PI * 16.0).sin().abs();
    canvas.stroke_line(hx - 14.0, hy + 18.0, hx - 4.0 - jaw_cycle * 8.0, hy + 30.0, Color::hex("#5B4331"), 3.2);
    canvas.stroke_line(hx + 14.0, hy + 18.0, hx + 4.0 + jaw_cycle * 8.0, hy + 30.0, Color::hex("#5B4331"), 3.2);

    // Muescas festoneadas de mordiscos en el huevo
    for b in 0..4 {
        let bx = cx + 80.0 + (b as f32) * 22.0;
        let by = cy + 90.0 + (b as f32) * 16.0;
        canvas.fill_circle(bx, by, 14.0, Color::hex("#EFE3C9"));
        canvas.stroke_circle(bx, by, 14.0, ink.with_alpha(0.6), 1.2);
    }
}

/// Dibuja el envés de hoja botánica gigante (Asclepias syriaca) ocupando el tercio superior
pub fn draw_giant_leaf_underside(
    canvas: &mut Canvas,
    w: f32,
    leaf_bottom_y: f32,
    seed: u32,
) {
    let leaf_col = Color::hex("#9DB08A");
    let vein_col = Color::hex("#D5DEC4");
    let ink = Color::hex("#2A1C13");
    let trichome = Color::hex("#FBF6EA");

    // Envés de la hoja cubriendo desde y=0 hasta leaf_bottom_y
    let mut lf_pb = tiny_skia::PathBuilder::new();
    lf_pb.move_to(-40.0, -20.0);
    lf_pb.line_to(w + 40.0, -20.0);
    lf_pb.line_to(w + 40.0, leaf_bottom_y - 40.0);
    // Margen inferior arqueado con curvatura botánica
    lf_pb.cubic_to(
        w * 0.7,
        leaf_bottom_y + 20.0,
        w * 0.3,
        leaf_bottom_y - 20.0,
        -40.0,
        leaf_bottom_y - 10.0,
    );
    lf_pb.close();

    if let Some(lp) = lf_pb.finish() {
        canvas.fill_path(&lp, leaf_col);
        canvas.stroke_path(&lp, ink, 3.2);

        // Nervadura principal curva cruzando la hoja
        let mid_y0 = leaf_bottom_y - 240.0;
        let mid_y1 = leaf_bottom_y - 180.0;
        let mut mid_pb = tiny_skia::PathBuilder::new();
        mid_pb.move_to(-40.0, mid_y0);
        mid_pb.cubic_to(w * 0.4, mid_y0 + 30.0, w * 0.7, mid_y1 - 20.0, w + 40.0, mid_y1);
        if let Some(mp) = mid_pb.finish() {
            canvas.stroke_path(&mp, vein_col, 9.0);
            canvas.stroke_path(&mp, ink.with_alpha(0.35), 2.0);
        }

        // 6 pares de venas secundarias pinnadas que se arquean hacia el margen
        for i in 0..7 {
            let vx = -20.0 + (i as f32) * (w / 6.0);
            let vy = mid_y0 + (i as f32) * 10.0;

            let mut v_pb = tiny_skia::PathBuilder::new();
            v_pb.move_to(vx, vy);
            v_pb.cubic_to(
                vx + 60.0,
                vy + 80.0,
                vx + 120.0,
                leaf_bottom_y - 10.0,
                vx + 160.0,
                leaf_bottom_y,
            );
            if let Some(vp) = v_pb.finish() {
                canvas.stroke_path(&vp, vein_col, 3.6);
                canvas.stroke_path(&vp, ink.with_alpha(0.3), 1.0);
            }
        }

        // Flecos de tricomas blancos (pelillos de la hoja) a lo largo del margen inferior
        let mut tx = -20.0;
        while tx <= w + 20.0 {
            let ty = leaf_bottom_y - 10.0 + (tx * 0.01).sin() * 8.0;
            canvas.stroke_line(tx, ty, tx + 6.0, ty + 18.0, trichome.with_alpha(0.65), 1.4);
            tx += 14.0;
        }

        // Gotas de látex blanco segregadas en puntos de corte
        for &(lx, ly) in &[(w * 0.35, leaf_bottom_y - 2.0), (w * 0.65, leaf_bottom_y + 4.0)] {
            canvas.fill_circle(lx, ly, 8.0, Color::hex("#FFFFFF"));
            canvas.stroke_circle(lx, ly, 8.0, ink.with_alpha(0.5), 1.2);
            canvas.fill_circle(lx + 2.0, ly + 6.0, 4.0, Color::hex("#FFFFFF"));
        }
    }
}

/// Dibuja el tallo botánico vertical con 3 pares de hojas opuestas, mordiscos y orugas escalando
pub fn draw_instar_stem_botanical_plate(
    canvas: &mut Canvas,
    cx: f32,
    w: f32,
    h: f32,
    tau: f32,
    seed: u32,
) {
    let stem_col = Color::hex("#7E9A63");
    let leaf_col = Color::hex("#9DB08A");
    let leaf_deep = Color::hex("#6F8A5E");
    let leaf_vein = Color::hex("#D5DEC4");
    let ink = Color::hex("#2A1C13");

    // Tallo principal vertical
    canvas.stroke_line(cx, 0.0, cx, h, stem_col, 32.0);
    canvas.stroke_line(cx - 14.0, 0.0, cx - 14.0, h, leaf_deep, 3.0);
    canvas.stroke_line(cx + 14.0, 0.0, cx + 14.0, h, ink.with_alpha(0.35), 2.0);

    // 3 pares de hojas opuestas en nodos verticales (1650, 1050, 450)
    let pair_ys = [1650.0_f32, 1050.0_f32, 450.0_f32];
    for (idx, &py) in pair_ys.iter().enumerate() {
        for &side in &[-1.0_f32, 1.0_f32] {
            let len = 420.0;
            let tip_x = cx + side * len;
            let tip_y = py - 70.0;

            let mut l_pb = tiny_skia::PathBuilder::new();
            l_pb.move_to(cx, py);
            l_pb.cubic_to(
                cx + side * len * 0.35,
                py - 110.0,
                cx + side * len * 0.75,
                py - 100.0,
                tip_x,
                tip_y,
            );
            l_pb.cubic_to(
                cx + side * len * 0.75,
                py + 80.0,
                cx + side * len * 0.35,
                py + 60.0,
                cx,
                py + 25.0,
            );
            l_pb.close();

            if let Some(lp) = l_pb.finish() {
                canvas.fill_path(&lp, leaf_col);
                canvas.stroke_path(&lp, ink, 3.0);

                // Nervadura central
                canvas.stroke_line(cx, py + 12.0, tip_x, tip_y, leaf_vein, 4.2);
                canvas.stroke_line(cx, py + 12.0, tip_x, tip_y, ink.with_alpha(0.3), 1.2);

                // Nervaduras secundarias
                for v in 1..=8 {
                    let f = v as f32 / 9.0;
                    let mx = cx + side * len * f;
                    let my = (py + 12.0) + (tip_y - (py + 12.0)) * f;
                    let vlen = 45.0 * (1.0 - (f - 0.5).abs());
                    canvas.stroke_line(mx, my, mx + side * vlen * 0.6, my - vlen * 0.8, leaf_vein, 2.0);
                    canvas.stroke_line(mx, my, mx + side * vlen * 0.6, my + vlen * 0.8, leaf_vein, 2.0);
                }

                // Daños de alimentación de oruga (mordiscos circulares y zanjas de látex)
                if idx == 0 {
                    // Hoja inferior: mordiscos festoneados en el margen
                    let bx = cx + side * 280.0;
                    let by = py - 40.0;
                    canvas.fill_circle(bx, by, 38.0, Color::hex("#EFE3C9"));
                    canvas.stroke_circle(bx, by, 38.0, ink, 2.2);
                } else if idx == 1 && side > 0.0 {
                    // Hoja media derecha: zanja de corte de látex con gotas
                    let hx = cx + 220.0;
                    let hy = py - 10.0;
                    canvas.fill_circle(hx, hy, 45.0, Color::hex("#EFE3C9"));
                    canvas.stroke_circle(hx, hy, 45.0, ink, 2.4);
                    // Gotas de látex
                    canvas.fill_circle(hx - 25.0, hy, 6.0, Color::hex("#FFFFFF"));
                    canvas.fill_circle(hx + 28.0, hy + 8.0, 5.0, Color::hex("#FFFFFF"));
                }
            }
        }
    }

    // Oruga de estadio avanzado escalando el tallo
    let cat_y = 1750.0 - tau * 1400.0;
    canvas.save();
    canvas.translate(cx, cat_y);
    canvas.rotate(-PI * 0.5);
    draw_caterpillar(canvas, 0.0, 0.0, 1.25, tau * 4.0, seed);
    canvas.restore();

    // Lupa circular 2.5x enfocando una oruga alimentándose en la hoja media
    let loupe_x = 285.0;
    let loupe_y = 1270.0;
    let loupe_r = 110.0;
    canvas.fill_circle(loupe_x, loupe_y, loupe_r, Color::hex("#F2E7CF"));
    // Dibujo ampliado dentro de la lupa
    canvas.save();
    canvas.translate(loupe_x, loupe_y);
    draw_caterpillar(canvas, 0.0, 0.0, 0.9, tau * 3.0, seed + 1);
    canvas.restore();
    // Aro de bronce y retícula
    canvas.stroke_circle(loupe_x, loupe_y, loupe_r, Color::hex("#C8A47A"), 6.0);
    canvas.stroke_circle(loupe_x, loupe_y, loupe_r, ink, 2.0);
    canvas.stroke_line(loupe_x - loupe_r, loupe_y, loupe_x + loupe_r, loupe_y, Color::hex("#C8A47A").with_alpha(0.6), 1.2);
    canvas.stroke_line(loupe_x, loupe_y - loupe_r, loupe_x, loupe_y + loupe_r, Color::hex("#C8A47A").with_alpha(0.6), 1.2);
    draw_vector_text(canvas, loupe_x, loupe_y + loupe_r + 24.0, "MAGNIFIED 2.5X • LATEX TRENCHING", 10.5, ink, true);

    // Líderes y regla milimétrica vertical a la derecha
    let ruler_x = 920.0;
    canvas.stroke_line(ruler_x, 220.0, ruler_x, 1720.0, Color::hex("#3B8EE0"), 2.2);
    let mut ry = 220.0;
    let mut mm = 50;
    while ry <= 1720.0 {
        let is_maj = mm % 10 == 0;
        let tlen = if is_maj { 22.0 } else { 10.0 };
        canvas.stroke_line(ruler_x - tlen, ry, ruler_x, ry, Color::hex("#3B8EE0"), if is_maj { 1.8 } else { 1.0 });
        if is_maj {
            let lbl = format!("{}mm", mm);
            draw_vector_text(canvas, ruler_x + 8.0, ry - 6.0, &lbl, 10.0, Color::hex("#3B8EE0"), false);
        }
        ry += 30.0;
        mm -= 1;
    }

    // Corchetes amarillos de estadio
    draw_dimension_bracket(canvas, ruler_x - 30.0, 1600.0, ruler_x - 30.0, 1400.0, "INSTAR 1-2", Color::hex("#EAB530"), 18.0);
    draw_dimension_bracket(canvas, ruler_x - 30.0, 1100.0, ruler_x - 30.0, 700.0, "INSTAR 3-4", Color::hex("#EAB530"), 18.0);
    draw_dimension_bracket(canvas, ruler_x - 30.0, 500.0, ruler_x - 30.0, 240.0, "INSTAR 5", Color::hex("#EAB530"), 18.0);
}

/// Dibuja la rama de madera rugosa horizontal superior para la suspensión en J y crisálida
pub fn draw_gnarled_branch_twig(
    canvas: &mut Canvas,
    w: f32,
    underside_y: f32,
    top_y: f32,
) {
    let bark_col = Color::hex("#5A4332");
    let bark_deep = Color::hex("#2A1C13");
    let ink = Color::hex("#2A1C13");

    let mut br_pb = tiny_skia::PathBuilder::new();
    br_pb.move_to(-20.0, top_y);
    br_pb.cubic_to(w * 0.3, top_y - 12.0, w * 0.7, top_y + 15.0, w + 20.0, top_y + 6.0);
    br_pb.line_to(w + 20.0, underside_y + 8.0);
    br_pb.cubic_to(w * 0.7, underside_y - 8.0, w * 0.3, underside_y + 14.0, -20.0, underside_y);
    br_pb.close();

    if let Some(bp) = br_pb.finish() {
        canvas.fill_path(&bp, bark_col);
        canvas.stroke_path(&bp, ink, 3.5);

        // Fisuras horizontales y nudos de la corteza
        for fy in [top_y + 15.0, top_y + 35.0, top_y + 50.0] {
            canvas.stroke_line(-10.0, fy, w + 10.0, fy + 4.0, bark_deep, 1.8);
        }
        // Nudos de corteza
        for &kx in &[w * 0.22, w * 0.78] {
            canvas.stroke_circle(kx, (top_y + underside_y) * 0.5, 14.0, bark_deep, 2.0);
            canvas.stroke_circle(kx, (top_y + underside_y) * 0.5, 6.0, ink, 1.5);
        }

        // Sombra arrojada proyectada debajo de la rama
        let shadow_y = underside_y + 12.0;
        canvas.stroke_line(-10.0, shadow_y, w + 10.0, shadow_y, ink.with_alpha(0.25), 6.0);
    }
}

/// Dibuja el péndulo y transportador de gravedad con plomada de latón
pub fn draw_pendulum_protractor_system(
    canvas: &mut Canvas,
    cx: f32,
    branch_y: f32,
    bob_y: f32,
    tau: f32,
) {
    let yellow = Color::hex("#EAB530");
    let ink = Color::hex("#2A1C13");

    // Hilo de plomada vertical fino
    canvas.stroke_line(cx, branch_y + 10.0, cx, bob_y, Color::hex("#5B4331").with_alpha(0.75), 1.4);

    // Pesa de plomada cónica de latón grabado
    let mut bob_pb = tiny_skia::PathBuilder::new();
    bob_pb.move_to(cx, bob_y + 24.0); // punta inferior
    bob_pb.line_to(cx - 10.0, bob_y - 10.0);
    bob_pb.line_to(cx + 10.0, bob_y - 10.0);
    bob_pb.close();
    if let Some(bp) = bob_pb.finish() {
        canvas.fill_path(&bp, Color::hex("#C8A47A"));
        canvas.stroke_path(&bp, ink, 1.8);
    }
    canvas.fill_circle(cx, bob_y - 10.0, 10.0, Color::hex("#D6A93C"));
    canvas.stroke_circle(cx, bob_y - 10.0, 10.0, ink, 1.5);

    // Arco graduado del transportador de péndulo en la parte inferior
    let arc_r = 1180.0;
    let arc_cy = branch_y;
    canvas.stroke_circle(cx, arc_cy, arc_r, yellow.with_alpha(0.65), 1.8);
    canvas.stroke_circle(cx, arc_cy, arc_r - 36.0, yellow.with_alpha(0.4), 1.2);

    // Marcas de grados (-30° a +30°)
    for deg in -20_i32..=20_i32 {
        let a = PI * 0.5 + (deg as f32 * DEG);
        let r0 = if deg % 5 == 0 { arc_r - 36.0 } else { arc_r - 18.0 };
        let r1 = arc_r;
        canvas.stroke_line(
            cx + a.cos() * r0,
            arc_cy + a.sin() * r0,
            cx + a.cos() * r1,
            arc_cy + a.sin() * r1,
            yellow.with_alpha(0.75),
            if deg % 5 == 0 { 1.8 } else { 1.0 },
        );
        if deg % 10 == 0 {
            let lbl = format!("{}°", deg.abs());
            draw_vector_text(canvas, cx + a.cos() * (r0 - 18.0), arc_cy + a.sin() * (r0 - 18.0), &lbl, 9.0, yellow, true);
        }
    }
}

/// Dibuja la ventana arqueada y reloj solar celestial de los 12 días de crisálida
pub fn draw_celestial_window_chrysalis_days(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
) {
    let wood = Color::hex("#5A4332");
    let ink = Color::hex("#2A1C13");
    let sun_col = Color::hex("#F1BF4A");
    let rose_dusk = Color::hex("#E3B1A1");
    let arc_y = cy - 80.0;

    // 1. Marco arqueado superior de ventana
    let win_r = 540.0;
    canvas.stroke_circle(cx, arc_y, win_r, wood, 14.0);
    canvas.stroke_circle(cx, arc_y, win_r, ink, 2.0);

    // Fondo celestial de degradado plano en arco (amanecer, cenit, ocaso)
    let roman_days = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII"];
    for (d, &rom) in roman_days.iter().enumerate() {
        let a0 = PI + (d as f32 / 12.0) * PI;
        let a1 = PI + ((d + 1) as f32 / 12.0) * PI;
        let bg_col = if d % 2 == 0 { Color::hex("#CF8FA0").with_alpha(0.35) } else { rose_dusk.with_alpha(0.35) };

        let mut s_pb = tiny_skia::PathBuilder::new();
        s_pb.move_to(cx, arc_y);
        s_pb.line_to(cx + a0.cos() * win_r, arc_y + a0.sin() * win_r);
        s_pb.line_to(cx + a1.cos() * win_r, arc_y + a1.sin() * win_r);
        s_pb.close();
        if let Some(sp) = s_pb.finish() {
            canvas.fill_path(&sp, bg_col);
            canvas.stroke_path(&sp, ink.with_alpha(0.4), 1.2);
        }

        // Número romano del día
        let mid_a = (a0 + a1) * 0.5;
        let rx = cx + mid_a.cos() * (win_r - 45.0);
        let ry = arc_y + mid_a.sin() * (win_r - 45.0);
        draw_vector_text(canvas, rx, ry, rom, 12.0, ink, true);
    }

    // Disco solar en su trayectoria
    let sun_a = PI + tau * PI;
    let sx = cx + sun_a.cos() * (win_r - 110.0);
    let sy = arc_y + sun_a.sin() * (win_r - 110.0);
    canvas.fill_circle(sx, sy, 22.0, sun_col);
    canvas.stroke_circle(sx, sy, 32.0, sun_col.with_alpha(0.5), 2.2);

    // Luna creciente opuesta
    let moon_a = sun_a + PI;
    let mx = cx + moon_a.cos() * (win_r - 110.0);
    let my = arc_y + moon_a.sin() * (win_r - 110.0);
    canvas.fill_circle(mx, my, 18.0, Color::hex("#FBF6EA"));
    canvas.fill_circle(mx + 6.0, my - 4.0, 15.0, Color::hex("#4E3F6E")); // mordisco lunar

    // Alféizar de ventana horizontal de madera oscura pasando tras la crisálida
    let sill_y0 = 574.0;
    let sill_y1 = 594.0;
    canvas.fill_rect(cx - win_r, sill_y0, win_r * 2.0, sill_y1 - sill_y0, wood);
    canvas.stroke_line(cx - win_r, sill_y0, cx + win_r, sill_y0, ink, 2.5);
    canvas.stroke_line(cx - win_r, sill_y1, cx + win_r, sill_y1, ink, 2.5);

    // Plano del suelo: gran hoja de algodoncillo en perspectiva con sombra de reloj solar
    let ground_y0 = 1030.0;
    let mut g_pb = tiny_skia::PathBuilder::new();
    g_pb.move_to(-40.0, h + 20.0);
    g_pb.line_to(w + 40.0, h + 20.0);
    g_pb.line_to(w + 40.0, ground_y0 + 200.0);
    g_pb.cubic_to(cx + 250.0, ground_y0, cx - 150.0, ground_y0 + 50.0, -40.0, ground_y0 + 140.0);
    g_pb.close();
    if let Some(gp) = g_pb.finish() {
        canvas.fill_path(&gp, Color::hex("#9DB08A"));
        canvas.stroke_path(&gp, ink, 3.2);

        // Nervadura de la hoja en perspectiva
        canvas.stroke_line(cx - 300.0, ground_y0 + 100.0, cx + 400.0, h - 50.0, Color::hex("#D5DEC4"), 6.0);

        // Sombra elíptica arrojada por la crisálida proyectada como aguja de reloj de sol
        let shadow_angle = (tau - 0.5) * 1.6;
        let sh_len = 220.0;
        let sh_tip_x = cx + shadow_angle.sin() * sh_len;
        let sh_tip_y = 1350.0 + shadow_angle.cos().abs() * 40.0;
        canvas.stroke_line(cx, 1350.0, sh_tip_x, sh_tip_y, ink.with_alpha(0.45), 18.0);
        canvas.stroke_circle(cx, 1350.0, 9.0, ink.with_alpha(0.7), 2.0);
    }
}

/// Dibuja el mosaico masivo macro de miles de escamas festoneadas en bandas diagonales
pub fn draw_massive_scale_mosaic(
    canvas: &mut Canvas,
    w: f32,
    h: f32,
    tau: f32,
    seed: u32,
) {
    let orange = Color::hex("#D9772B");
    let orange_deep = Color::hex("#B55A1C");
    let black = Color::hex("#221A15");
    let white = Color::hex("#F6F0E2");
    let ink = Color::hex("#2A1C13");

    let tile_w = 26.0_f32;
    let tile_h = 19.0_f32;
    let nx = (w / tile_w) as usize + 2;
    let ny = (h / tile_h) as usize + 3;

    // Filas solapadas tipo tejas o escamas de pez
    for y_idx in 0..ny {
        let y = (y_idx as f32) * tile_h - 10.0;
        let x_shift = if y_idx % 2 == 0 { 0.0 } else { tile_w * 0.5 };

        for x_idx in 0..nx {
            let x = (x_idx as f32) * tile_w + x_shift - 10.0;

            // Franjas diagonales de color: naranja, negro y blanco
            let diag = (x * 0.6 + y * 0.4) / 280.0;
            let col = if diag % 3.0 < 1.4 {
                orange
            } else if diag % 3.0 < 2.3 {
                black
            } else {
                white
            };

            // Escama festoneada redondeada
            let mut sc_pb = tiny_skia::PathBuilder::new();
            sc_pb.move_to(x, y);
            sc_pb.cubic_to(x + tile_w * 0.2, y + tile_h * 1.1, x + tile_w * 0.8, y + tile_h * 1.1, x + tile_w, y);
            sc_pb.close();
            if let Some(sp) = sc_pb.finish() {
                canvas.fill_path(&sp, col);
                canvas.stroke_path(&sp, ink.with_alpha(0.55), 1.0);

                // Microranuras longitudinales estructurales de la escama
                if col == orange {
                    canvas.stroke_line(x + tile_w * 0.35, y + 2.0, x + tile_w * 0.35, y + tile_h * 0.7, orange_deep, 0.8);
                    canvas.stroke_line(x + tile_w * 0.65, y + 2.0, x + tile_w * 0.65, y + tile_h * 0.7, orange_deep, 0.8);
                }
            }
        }
    }

    // Retícula de microscopía circular al centro (500X)
    let loupe_x = w * 0.5;
    let loupe_y = h * 0.5;
    let loupe_r = 190.0;

    canvas.fill_circle(loupe_x, loupe_y, loupe_r, Color::hex("#FBF6EA"));
    // Detalle de una sola escama ampliada 500X dentro de la retícula
    let s_sc = 1.8;
    canvas.save();
    canvas.translate(loupe_x, loupe_y);
    let mut big_sc = tiny_skia::PathBuilder::new();
    big_sc.move_to(-60.0 * s_sc, -70.0 * s_sc);
    big_sc.cubic_to(-70.0 * s_sc, 70.0 * s_sc, 70.0 * s_sc, 70.0 * s_sc, 60.0 * s_sc, -70.0 * s_sc);
    big_sc.close();
    if let Some(bp) = big_sc.finish() {
        canvas.fill_path(&bp, orange);
        canvas.stroke_path(&bp, ink, 3.5);
        // Nanoridgelines (peines paralelos)
        for r in -4..=4 {
            let rx = (r as f32) * 12.0 * s_sc;
            canvas.stroke_line(rx, -60.0 * s_sc, rx, 50.0 * s_sc, orange_deep, 2.2);
        }
    }
    canvas.restore();

    canvas.stroke_circle(loupe_x, loupe_y, loupe_r, Color::hex("#C8A47A"), 8.0);
    canvas.stroke_circle(loupe_x, loupe_y, loupe_r, ink, 2.4);
    canvas.stroke_line(loupe_x - loupe_r, loupe_y, loupe_x + loupe_r, loupe_y, Color::hex("#C8A47A").with_alpha(0.6), 1.4);
    canvas.stroke_line(loupe_x, loupe_y - loupe_r, loupe_x, loupe_y + loupe_r, Color::hex("#C8A47A").with_alpha(0.6), 1.4);
}

/// Dibuja el paisaje panorámico de migración con colinas, valles, sol y río de mariposas
pub fn draw_migration_panoramic_landscape(
    canvas: &mut Canvas,
    w: f32,
    h: f32,
    tau: f32,
    seed: u32,
) {
    let ink = Color::hex("#2A1C13");

    // Cielo crema cálido
    draw_paper_plate(canvas, w as u32, h as u32, seed);

    // Sol con rayos de compás grabado arriba a la izquierda
    let sun_x = 180.0;
    let sun_y = 260.0;
    canvas.fill_circle(sun_x, sun_y, 42.0, Color::hex("#F1BF4A"));
    canvas.stroke_circle(sun_x, sun_y, 42.0, ink, 2.0);
    for i in 0..16 {
        let a = (i as f32 / 16.0) * PI * 2.0;
        let r0 = 50.0;
        let r1 = if i % 2 == 0 { 85.0 } else { 68.0 };
        canvas.stroke_line(sun_x + a.cos() * r0, sun_y + a.sin() * r0, sun_x + a.cos() * r1, sun_y + a.sin() * r1, Color::hex("#F1BF4A"), 1.8);
    }

    // Estratos de colinas onduladas (horizonte y valles)
    let hill_data = [
        (h * 0.62, Color::hex("#CDB77E"), 80.0_f32),
        (h * 0.74, Color::hex("#9DB08A"), 60.0_f32),
        (h * 0.85, Color::hex("#6F8A5E"), 45.0_f32),
        (h * 0.94, Color::hex("#3E5C54"), 30.0_f32),
    ];

    for &(hy, col, amp) in &hill_data {
        let mut h_pb = tiny_skia::PathBuilder::new();
        h_pb.move_to(0.0, h + 20.0);
        h_pb.line_to(0.0, hy);
        h_pb.cubic_to(w * 0.35, hy - amp, w * 0.65, hy + amp, w, hy - amp * 0.5);
        h_pb.line_to(w, h + 20.0);
        h_pb.close();
        if let Some(hp) = h_pb.finish() {
            canvas.fill_path(&hp, col);
            canvas.stroke_path(&hp, ink, 2.4);

            // Rayado de bosque de pinos en la capa más baja
            if col == Color::hex("#3E5C54") {
                let mut tx = 10.0;
                while tx <= w - 10.0 {
                    canvas.stroke_line(tx, hy - 15.0, tx, hy + 25.0, ink, 1.4);
                    tx += 16.0;
                }
            }
        }
    }

    // Carretera / río sinuoso blanco en el valle
    let mut road_pb = tiny_skia::PathBuilder::new();
    road_pb.move_to(w * 0.2, h);
    road_pb.cubic_to(w * 0.45, h * 0.82, w * 0.3, h * 0.72, w * 0.85, h * 0.64);
    if let Some(rp) = road_pb.finish() {
        canvas.stroke_path(&rp, Color::hex("#FBF6EA"), 14.0);
        canvas.stroke_path(&rp, ink, 2.0);
    }

    // Río migratorio de cientos de mariposas en una corriente térmica sinuosa
    let mut rng = Rng::new(seed + 888);
    let swarm_n = 70;
    for i in 0..swarm_n {
        let u = (i as f32 / swarm_n as f32 + tau * 0.8) % 1.0;
        let stream_x = w * (0.85 - u * 0.7) + (rng.next_f32() - 0.5) * 160.0;
        let stream_y = h * (0.25 + u * 0.68) + (rng.next_f32() - 0.5) * 120.0;
        let flap = (tau * PI * 18.0 + (i as f32)).sin() * 0.5;
        let sc = 0.16 + rng.next_f32() * 0.18;

        draw_monarch(canvas, stream_x, stream_y, sc, flap, false, seed + i as u32);
    }
}

/// Dibuja la malla alámbrica 3D completa del huevo G1 en plano azul (18 meridianos + 34 paralelos)
pub fn draw_wireframe_egg_3d_blueprint(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    tau: f32,
) {
    let navy = Color::hex("#0B1230");
    let grid_col = Color::hex("#3A4A86");
    let lav = Color::hex("#C8C1EF");
    let white = Color::hex("#EEF0FF");
    let spark_glow = Color::hex("#FFF3DC");
    let magenta = Color::hex("#FF3D98");

    // Fondo azul plano profundo con rejilla ortogonal
    canvas.clear(navy);
    let grid_size = 60.0_f32;
    let mut gx = 0.0;
    while gx <= canvas.width as f32 {
        canvas.stroke_line(gx, 0.0, gx, canvas.height as f32, grid_col.with_alpha(0.55), 1.0);
        gx += grid_size;
    }
    let mut gy = 0.0;
    while gy <= canvas.height as f32 {
        canvas.stroke_line(0.0, gy, canvas.width as f32, gy, grid_col.with_alpha(0.55), 1.0);
        gy += grid_size;
    }

    // Cabecera arquitectónica superior
    draw_blueprint_header(canvas, canvas.width as f32, "STAGE 01 : CHORION BLASTODERM", "CALIBRATION SCALE 1:100 • RES 0.01mm");

    // Malla alámbrica 3D del huevo (meridianos elípticos y paralelos)
    let top_y = cy - h * 0.5;
    let bot_y = cy + h * 0.5;

    // 18 costillas meridionales de polo a polo
    let num_meridians = 18;
    for m in 0..num_meridians {
        let a = (m as f32 / num_meridians as f32) * PI;
        let rx = a.cos() * w * 0.92;

        let mut m_pb = tiny_skia::PathBuilder::new();
        m_pb.move_to(cx, top_y);
        m_pb.cubic_to(cx + rx, top_y + h * 0.32, cx + rx, bot_y - h * 0.32, cx, bot_y);
        if let Some(mp) = m_pb.finish() {
            let is_edge = m == 0 || m == num_meridians - 1;
            canvas.stroke_path(&mp, if is_edge { white } else { lav.with_alpha(0.65) }, if is_edge { 2.4 } else { 1.1 });
        }
    }

    // 34 paralelos de latitud transversales
    let num_latitudes = 34;
    for l in 1..num_latitudes {
        let frac = l as f32 / num_latitudes as f32;
        let ly = top_y + h * frac;
        let span = w * (frac * PI).sin().powf(0.68) * 0.92;

        // Paralelo en elipse proyectada ligeramente inclinada hacia arriba
        let mut p_pb = tiny_skia::PathBuilder::new();
        p_pb.move_to(cx - span, ly);
        p_pb.cubic_to(cx - span * 0.5, ly - 12.0, cx + span * 0.5, ly - 12.0, cx + span, ly);
        p_pb.cubic_to(cx + span * 0.5, ly + 12.0, cx - span * 0.5, ly + 12.0, cx - span, ly);
        p_pb.close();
        if let Some(pp) = p_pb.finish() {
            canvas.stroke_path(&pp, lav.with_alpha(0.45), 0.9);
        }
    }

    // Chispa nuclear brillante al centro (Star-burst de 8 puntas y halo)
    let spark_x = cx;
    let spark_y = cy;
    canvas.fill_circle(spark_x, spark_y, 14.0, spark_glow);
    canvas.stroke_circle(spark_x, spark_y, 35.0, white.with_alpha(0.6), 2.0);
    for i in 0..8 {
        let a = (i as f32 / 8.0) * PI * 2.0;
        let r_out = if i % 2 == 0 { 65.0 } else { 40.0 };
        canvas.stroke_line(spark_x, spark_y, spark_x + a.cos() * r_out, spark_y + a.sin() * r_out, white, if i % 2 == 0 { 2.4 } else { 1.2 });
    }

    // Retículas y cotas técnicas
    draw_dimension_bracket(canvas, cx + w + 30.0, top_y, cx + w + 30.0, bot_y, "1.2 mm", lav, 30.0);
    draw_node_callout(canvas, cx, bot_y, "MICROPYLE", "FERTILIZATION ROSETTE", magenta, true);

    // Retículas circulares en las esquinas inferiores (división celular y corte de corion)
    let ret_l = 220.0;
    let ret_r = canvas.width as f32 - 220.0;
    let ret_y = canvas.height as f32 - 260.0;
    draw_magnifying_loupe(canvas, ret_l, ret_y, 75.0, "SYNCYTIAL BLASTODERM", lav);
    // 6 núcleos en división
    for n in 0..6 {
        let na = (n as f32 / 6.0) * PI * 2.0;
        canvas.fill_circle(ret_l + na.cos() * 32.0, ret_y + na.sin() * 32.0, 7.0, magenta);
    }
    draw_magnifying_loupe(canvas, ret_r, ret_y, 75.0, "CHORION CROSS-SECTION", lav);
    for r in 0..12 {
        let ry_pos = ret_y - 45.0 + (r as f32) * 8.0;
        canvas.stroke_line(ret_r - 40.0, ry_pos, ret_r + 40.0, ry_pos, lav.with_alpha(0.7), 1.2);
    }
}

