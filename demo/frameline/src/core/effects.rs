//! Efectos visuales post-render: X-Ray, Chromatic Aberration, Scanlines, VHS Glitch,
//! Bloom Glow, Film Burn, Pixel Dissolve y Vignette.
//!
//! Todos los efectos operan sobre un `Canvas` ya renderizado, aplicando transformaciones
//! de composición o manipulación directa de bytes RGBA.
//!
//! **Convención tiny-skia**: los bytes RGBA están en orden R-G-B-A con alpha premultiplicado.
//! Las funciones que manipulan bytes deben tener en cuenta esto.

use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::rng::Rng;
use tiny_skia::{BlendMode, PathBuilder};
use std::f32::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// 1. VIGNETTE — marco oscuro en bordes
// ─────────────────────────────────────────────────────────────────────────────

/// Aplica un viñeteo elíptico oscuro en los bordes del canvas.
///
/// * `strength` – opacidad máxima del borde (0.0–0.85 recomendado)
/// * `softness` – qué tan gradual es la caída (0.5 suave, 2.0 duro)
pub fn vignette(canvas: &mut Canvas, strength: f32, softness: f32) {
    let w = canvas.width as f32;
    let h = canvas.height as f32;
    let steps = 16usize;

    canvas.save();
    canvas.set_blend_mode(BlendMode::SourceOver);

    for i in 0..steps {
        // t = 0 → borde exterior, t = 1 → borde interior (sin oscurecer el centro)
        let t = i as f32 / steps as f32;
        // Solo oscurecer la franja exterior: el borde oscuro comienza en t=0, se desvanece a t=1
        let inset_factor = t;
        // rx/ry decrece desde las esquinas hacia el centro
        let rx = w * 0.5 * (1.0 - inset_factor * 0.92);
        let ry = h * 0.5 * (1.0 - inset_factor * 0.92);

        // Cada anillo aporta una pequeña cantidad de oscuridad; la suma crea el gradiente
        let alpha_per_ring = (strength / steps as f32) * (1.0 - t.powf(softness.max(0.2)));
        let alpha = alpha_per_ring.clamp(0.0, 0.12); // cap por anillo para evitar acumulación extrema

        if alpha < 0.003 { continue; }

        let pts: Vec<_> = (0..=48)
            .map(|k| {
                let theta = k as f32 / 48.0 * PI * 2.0;
                (w * 0.5 + rx * theta.cos(), h * 0.5 + ry * theta.sin())
            })
            .collect();
        let mut pb = PathBuilder::new();
        for (k, &(px, py)) in pts.iter().enumerate() {
            if k == 0 { pb.move_to(px, py); } else { pb.line_to(px, py); }
        }
        pb.close();
        // Dibujar el área FUERA de la elipse: usamos un path invertido
        // Rectángulo completo del canvas minus la elipse interna
        let mut pb2 = PathBuilder::new();
        pb2.move_to(0.0, 0.0);
        pb2.line_to(w, 0.0);
        pb2.line_to(w, h);
        pb2.line_to(0.0, h);
        pb2.close();
        // Elipse interior como "hueco" — dibujamos en sentido inverso
        for (k, &(px, py)) in pts.iter().enumerate().rev() {
            if k == pts.len() - 1 { pb2.move_to(px, py); } else { pb2.line_to(px, py); }
        }
        pb2.close();
        if let Some(path) = pb2.finish() {
            canvas.fill_path(&path, Color::rgba(0.0, 0.0, 0.0, alpha));
        }
    }

    canvas.restore();
}


// ─────────────────────────────────────────────────────────────────────────────
// 2. SCANLINES — líneas horizontales de monitor CRT
// ─────────────────────────────────────────────────────────────────────────────

/// Dibuja líneas horizontales semitransparentes simulando un monitor CRT.
///
/// * `gap` – píxeles entre líneas (4 = muy denso, 8 = cómodo, 16 = retro)
/// * `alpha` – opacidad de cada línea (0.08–0.25 recomendado)
pub fn scanlines(canvas: &mut Canvas, gap: u32, alpha: f32) {
    let w = canvas.width as f32;
    let h = canvas.height as f32;
    let gap = gap.max(2) as f32;
    let color = Color::rgba(0.0, 0.0, 0.0, alpha.clamp(0.0, 0.5));

    let mut y = 0.0f32;
    while y < h {
        canvas.stroke_line(0.0, y, w, y, color, 1.0);
        y += gap;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. FILM BURN — quemado cinematográfico desde los bordes
// ─────────────────────────────────────────────────────────────────────────────

/// Simula un quemado de película: gradiente cálido desde las esquinas.
///
/// * `tau` – progreso normalizado (0.0 = sin efecto, 1.0 = quemado total)
/// * `warm` – color del quemado (naranja/ámbar sugerido)
pub fn film_burn(canvas: &mut Canvas, tau: f32, warm: Color) {
    if tau < 0.01 { return; }
    let w = canvas.width as f32;
    let h = canvas.height as f32;
    let steps = 20usize;

    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);

    for i in 0..steps {
        let t = i as f32 / steps as f32;
        // Elipse que crece desde las esquinas hacia adentro
        let rx = w * (0.15 + t * 0.45) * tau;
        let ry = h * (0.15 + t * 0.45) * tau;
        let alpha = ((1.0 - t) * tau * 0.35).clamp(0.0, 1.0);

        // 4 esquinas
        for &(cx, cy) in &[(0.0f32, 0.0f32), (w, 0.0), (0.0, h), (w, h)] {
            let pts: Vec<_> = (0..=32)
                .map(|k| {
                    let theta = k as f32 / 32.0 * PI * 2.0;
                    (cx + rx * theta.cos(), cy + ry * theta.sin())
                })
                .collect();
            let mut pb = PathBuilder::new();
            for (k, &(px, py)) in pts.iter().enumerate() {
                if k == 0 { pb.move_to(px, py); } else { pb.line_to(px, py); }
            }
            pb.close();
            if let Some(path) = pb.finish() {
                canvas.fill_path(&path, warm.with_alpha(alpha));
            }
        }
    }

    canvas.restore();
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. BLOOM GLOW — destello luminoso sobre toda la imagen
// ─────────────────────────────────────────────────────────────────────────────

/// Efecto de bloom: superpone el canvas sobreexpuesto con blend Screen para
/// simular dispersión de luz en áreas brillantes.
///
/// * `intensity` – cuánto bloom (0.1 sutil, 0.6 dramático)
/// * `glow_color` – tinte del destello (blanco puro o cálido)
pub fn bloom_glow(canvas: &mut Canvas, intensity: f32, glow_color: Color) {
    if intensity < 0.01 { return; }
    let w = canvas.width as f32;
    let h = canvas.height as f32;

    // Dibujamos múltiples capas de elipses difuminadas con Screen blend
    // La capa exterior es muy transparente, la interior más opaca → halo
    let layers = 8usize;
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);

    for i in 0..layers {
        let t = i as f32 / layers as f32;
        let rx = w * (0.3 + t * 0.25);
        let ry = h * (0.3 + t * 0.25);
        let alpha = ((1.0 - t) * intensity * 0.18).clamp(0.0, 1.0);

        let pts: Vec<_> = (0..=48)
            .map(|k| {
                let theta = k as f32 / 48.0 * PI * 2.0;
                (w * 0.5 + rx * theta.cos(), h * 0.5 + ry * theta.sin())
            })
            .collect();
        let mut pb = PathBuilder::new();
        for (k, &(px, py)) in pts.iter().enumerate() {
            if k == 0 { pb.move_to(px, py); } else { pb.line_to(px, py); }
        }
        pb.close();
        if let Some(path) = pb.finish() {
            canvas.fill_path(&path, glow_color.with_alpha(alpha));
        }
    }

    canvas.restore();
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. CHROMATIC ABERRATION — desplazamiento RGB por canal
// ─────────────────────────────────────────────────────────────────────────────

/// Desplaza el canal rojo +offset_px y el canal azul –offset_px horizontalmente.
/// Opera sobre los bytes crudos del Pixmap (RGBA premultiplicado tiny-skia).
///
/// * `offset_px` – píxeles de desplazamiento (2–8 para sutil, 12–20 para extremo)
pub fn chromatic_aberration(canvas: &mut Canvas, offset_px: i32) {
    if offset_px == 0 { return; }
    let w = canvas.width as usize;
    let h = canvas.height as usize;

    // Copia de los bytes originales
    let src: Vec<u8> = canvas.pixmap.data().to_vec();
    let dst = canvas.pixmap.data_mut();

    for y in 0..h {
        for x in 0..w {
            let dst_idx = (y * w + x) * 4;

            // Canal Rojo: pixel desplazado hacia la derecha
            let r_x = (x as i32 - offset_px).clamp(0, w as i32 - 1) as usize;
            let r_idx = (y * w + r_x) * 4;
            dst[dst_idx] = src[r_idx]; // R

            // Canal Verde: sin desplazamiento (ya está en dst de la copia si no cambiamos)
            // lo dejamos tal cual del src
            dst[dst_idx + 1] = src[dst_idx + 1]; // G

            // Canal Azul: pixel desplazado hacia la izquierda
            let b_x = (x as i32 + offset_px).clamp(0, w as i32 - 1) as usize;
            let b_idx = (y * w + b_x) * 4;
            dst[dst_idx + 2] = src[b_idx + 2]; // B

            // Alpha: promedio entre los tres orígenes para consistencia
            dst[dst_idx + 3] = src[dst_idx + 3];
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. VHS GLITCH — bloques de píxeles desplazados horizontalmente
// ─────────────────────────────────────────────────────────────────────────────

/// Simula el artefacto de tracking de VHS: bloques horizontales de la imagen
/// se desplazan aleatoriamente, creando efecto de "glitch".
///
/// * `intensity` – amplitud máxima de desplazamiento en píxeles (4–30)
/// * `block_height` – alto de cada bloque afectado (2–12)
/// * `seed` – semilla determinista para reproducibilidad
pub fn vhs_glitch(canvas: &mut Canvas, intensity: i32, block_height: u32, seed: u32) {
    if intensity == 0 { return; }
    let w = canvas.width as usize;
    let h = canvas.height as usize;
    let bh = (block_height as usize).max(2);
    let mut rng = Rng::new(seed);

    let src: Vec<u8> = canvas.pixmap.data().to_vec();
    let dst = canvas.pixmap.data_mut();

    let mut y = 0usize;
    while y < h {
        let block_end = (y + bh).min(h);
        // Solo aplicar glitch a ~20% de los bloques
        let apply = rng.next_f32() < 0.20;
        let shift = if apply {
            let raw = (rng.next_f32() * 2.0 - 1.0) * intensity as f32;
            raw as i32
        } else {
            0
        };

        if shift != 0 {
            for row in y..block_end {
                for x in 0..w {
                    let dst_idx = (row * w + x) * 4;
                    let src_x = (x as i32 + shift).clamp(0, w as i32 - 1) as usize;
                    let src_idx = (row * w + src_x) * 4;
                    dst[dst_idx..dst_idx + 4].copy_from_slice(&src[src_idx..src_idx + 4]);
                }
            }
        }

        y += bh;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. PIXEL DISSOLVE — aparición / desaparición por dithering
// ─────────────────────────────────────────────────────────────────────────────

/// Enmascara píxeles según un patrón dithering para simular aparición o desaparición.
///
/// * `tau` – 0.0 = invisible (todo enmascarado), 1.0 = totalmente visible
/// * `background` – color de fondo que "llena" los píxeles enmascarados
/// * `seed` – semilla determinista
pub fn pixel_dissolve(canvas: &mut Canvas, tau: f32, background: Color, seed: u32) {
    let tau = tau.clamp(0.0, 1.0);
    if tau >= 1.0 { return; } // Completamente visible: nada que hacer

    let w = canvas.width as usize;
    let h = canvas.height as usize;
    let mut rng = Rng::new(seed);

    // Patrón Bayer 4×4 para dithering ordenado (normalizado 0–1)
    let bayer4: [f32; 16] = [
         0.0/16.0,  8.0/16.0,  2.0/16.0, 10.0/16.0,
        12.0/16.0,  4.0/16.0, 14.0/16.0,  6.0/16.0,
         3.0/16.0, 11.0/16.0,  1.0/16.0,  9.0/16.0,
        15.0/16.0,  7.0/16.0, 13.0/16.0,  5.0/16.0,
    ];

    let bg = background.to_u8_array();
    let dst = canvas.pixmap.data_mut();

    for y in 0..h {
        for x in 0..w {
            let bayer_val = bayer4[(y % 4) * 4 + (x % 4)];
            // Añadimos un pequeño jitter de noise para romper el patrón mecánico
            let noise = rng.next_f32() * 0.08;
            let threshold = bayer_val * 0.7 + noise;

            if tau < threshold {
                // Reemplazar con color de fondo
                let idx = (y * w + x) * 4;
                dst[idx]     = bg[0];
                dst[idx + 1] = bg[1];
                dst[idx + 2] = bg[2];
                dst[idx + 3] = bg[3];
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. X-RAY OVERLAY — esqueleto y estructura interna
// ─────────────────────────────────────────────────────────────────────────────

/// Parámetros para el efecto X-Ray que se dibuja sobre un puppet.
pub struct XRayParams {
    /// Posición X del centro del puppet
    pub cx: f32,
    /// Posición Y del centro del puppet
    pub cy: f32,
    /// Escala del puppet (para ajustar el esqueleto)
    pub scale: f32,
    /// Progreso de la revelación: 0.0 invisible → 1.0 completamente visible
    pub reveal: f32,
    /// Color del esqueleto (por defecto cian luminoso)
    pub bone_color: Color,
    /// Semilla para jitter orgánico
    pub seed: u32,
}

impl Default for XRayParams {
    fn default() -> Self {
        Self {
            cx: 0.0,
            cy: 0.0,
            scale: 1.0,
            reveal: 1.0,
            bone_color: Color::hex("#7fe7ff"),
            seed: 1,
        }
    }
}

/// Dibuja una capa de rayos X sobre el puppet: esqueleto axial, costillas,
/// nervios alares y puntos de articulación.
///
/// Se usa con `BlendMode::Screen` para que brille sobre la imagen de base.
pub fn draw_xray_fly(canvas: &mut Canvas, params: &XRayParams) {
    let alpha = params.reveal.clamp(0.0, 1.0);
    if alpha < 0.01 { return; }

    let s = params.scale;
    let cx = params.cx;
    let cy = params.cy;
    let col = params.bone_color.with_alpha(alpha * 0.90);
    let col_dim = params.bone_color.with_alpha(alpha * 0.45);
    let col_faint = params.bone_color.with_alpha(alpha * 0.25);

    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);

    // ── Columna vertebral (espina dorsal axial) ──────────────────────────
    let spine_pts: &[[f32; 2]] = &[
        [cx, cy - 52.0 * s], // cráneo
        [cx, cy - 30.0 * s], // cuello
        [cx, cy - 10.0 * s], // tórax superior
        [cx, cy + 10.0 * s], // tórax inferior
        [cx, cy + 35.0 * s], // abdomen superior
        [cx, cy + 75.0 * s], // abdomen inferior
    ];
    draw_xray_spine(canvas, spine_pts, col, 1.6 * s, params.seed);

    // ── Costillas del tórax ──────────────────────────────────────────────
    let rib_pairs: &[(f32, f32, f32)] = &[
        // (y_offset, x_reach, arc_height)
        (-18.0, 26.0, 8.0),
        (-10.0, 28.0, 6.0),
        ( -2.0, 26.0, 5.0),
        (  6.0, 22.0, 4.0),
    ];
    for &(dy, rx, arc_h) in rib_pairs {
        let y0 = cy + dy * s;
        // Costilla izquierda
        draw_xray_arc(canvas, cx, y0, -rx * s, arc_h * s, col_dim, 1.2 * s);
        // Costilla derecha (espejo)
        draw_xray_arc(canvas, cx, y0,  rx * s, arc_h * s, col_dim, 1.2 * s);
    }

    // ── Nervios alares (venación de las alas) ───────────────────────────
    // Ala izquierda: nervios desde la raíz hacia los bordes
    draw_xray_wing_nerves(canvas, cx - 14.0 * s, cy - 12.0 * s, -1.0, s, col_faint, params.seed + 5);
    // Ala derecha
    draw_xray_wing_nerves(canvas, cx + 14.0 * s, cy - 12.0 * s,  1.0, s, col_faint, params.seed + 6);

    // ── Articulaciones: puntos de unión en blanco frío ──────────────────
    let joints: &[(f32, f32, f32)] = &[
        (cx, cy - 30.0 * s, 3.5 * s), // cuello
        (cx, cy - 10.0 * s, 4.0 * s), // tórax
        (cx, cy + 10.0 * s, 3.5 * s), // abdomen
        (cx - 14.0 * s, cy - 12.0 * s, 3.0 * s), // ala L
        (cx + 14.0 * s, cy - 12.0 * s, 3.0 * s), // ala R
    ];
    for &(jx, jy, jr) in joints {
        canvas.fill_circle(jx, jy, jr, col);
        canvas.stroke_circle(jx, jy, jr * 1.6, col_faint, 0.8 * s);
    }

    // ── Ojo interno: retícula hexagonal luminosa ─────────────────────────
    for &(ex, ey) in &[
        (cx - 13.0 * s, cy - 40.0 * s),
        (cx + 13.0 * s, cy - 40.0 * s),
    ] {
        canvas.stroke_circle(ex, ey, 9.0 * s, col, 1.0 * s);
        // Cruz de retícula
        canvas.stroke_line(ex - 9.0*s, ey, ex + 9.0*s, ey, col_dim, 0.7 * s);
        canvas.stroke_line(ex, ey - 9.0*s, ex, ey + 9.0*s, col_dim, 0.7 * s);
        canvas.fill_circle(ex, ey, 2.0 * s, col);
    }

    canvas.restore();
}

/// Dibuja la espina dorsal como segmentos con pequeñas marcas transversales (vértebras)
fn draw_xray_spine(canvas: &mut Canvas, pts: &[[f32; 2]], color: Color, width: f32, seed: u32) {
    use crate::core::finishes::wob_path;
    if let Some(path) = wob_path(pts, 1.2, seed, false) {
        canvas.stroke_path(&path, color, width);
    }
    // Discos intervertebrales: pequeños rectángulos horizontales entre cada par
    for pair in pts.windows(2) {
        let mx = (pair[0][0] + pair[1][0]) * 0.5;
        let my = (pair[0][1] + pair[1][1]) * 0.5;
        let half_w = width * 3.0;
        canvas.stroke_line(mx - half_w, my, mx + half_w, my, color.with_alpha(color.a * 0.6), width * 0.7);
    }
}

/// Dibuja un arco curvado (costilla) desde el centro hacia el lado indicado
fn draw_xray_arc(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    reach_x: f32,
    arc_h: f32,
    color: Color,
    width: f32,
) {
    let steps = 8usize;
    let pts: Vec<[f32; 2]> = (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            let x = cx + reach_x * t;
            // Arco parabólico: sube y luego baja
            let arc = arc_h * (1.0 - (2.0 * t - 1.0).powi(2));
            let y = cy - arc;
            [x, y]
        })
        .collect();

    use crate::core::finishes::wob_path;
    if let Some(path) = wob_path(&pts, 1.0, 42, false) {
        canvas.stroke_path(&path, color, width);
    }
}

/// Dibuja nervios alares: líneas que irradian desde la base del ala
fn draw_xray_wing_nerves(
    canvas: &mut Canvas,
    root_x: f32,
    root_y: f32,
    side: f32, // -1.0 izq, +1.0 der
    scale: f32,
    color: Color,
    seed: u32,
) {
    let mut rng = Rng::new(seed);
    // 6 nervios principales
    for k in 0..6usize {
        let angle_base = if side < 0.0 {
            PI + 0.1 + k as f32 * 0.15
        } else {
            -0.1 - k as f32 * 0.15
        };
        let length = (35.0 + rng.next_f32() * 20.0) * scale;
        let end_x = root_x + angle_base.cos() * length;
        let end_y = root_y + angle_base.sin() * length;
        canvas.stroke_line(root_x, root_y, end_x, end_y, color, 0.8 * scale);

        // Nervio secundario desde el punto medio
        if k < 4 {
            let mid_x = (root_x + end_x) * 0.5;
            let mid_y = (root_y + end_y) * 0.5;
            let sub_len = length * 0.4;
            let sub_angle = angle_base + side * 0.5;
            canvas.stroke_line(
                mid_x, mid_y,
                mid_x + sub_angle.cos() * sub_len,
                mid_y + sub_angle.sin() * sub_len,
                color.with_alpha(color.a * 0.6),
                0.6 * scale,
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Utilidad: smoothstep
// ─────────────────────────────────────────────────────────────────────────────

pub use crate::core::easing::smoothstep;

// ─────────────────────────────────────────────────────────────────────────────
// 10. CALIDAD CINEMATOGRÁFICA: Iluminación, Niebla y Motion Blur
// ─────────────────────────────────────────────────────────────────────────────

/// Acumula el frame previo sobre el actual para simular desenfoque de movimiento (Motion Blur).
///
/// * `blend_factor` – fracción retenida del fotograma anterior (típicamente 0.15 a 0.35).
pub fn motion_blur_accumulate(canvas: &mut Canvas, prev_canvas: &Canvas, blend_factor: f32) {
    if blend_factor <= 0.001 {
        return;
    }
    canvas.composite_over(prev_canvas, blend_factor);
}

/// Añade una fuente de luz puntual dinámica con atenuación cuadrática y halo luminoso.
///
/// Usa `BlendMode::Screen` para iluminar las capas existentes sin saturar a blanco plano.
///
/// * `lx, ly`     – posición de la fuente de luz
/// * `radius`     – radio máximo de alcance de la luz en px
/// * `color`      – color de la luz (ej. amarillo cálido o azul etéreo)
/// * `intensity`  – multiplicador de potencia (0.0 a 1.0+)
pub fn point_light(
    canvas: &mut Canvas,
    lx: f32,
    ly: f32,
    radius: f32,
    color: Color,
    intensity: f32,
) {
    if intensity <= 0.001 || radius <= 1.0 {
        return;
    }
    canvas.save();
    canvas.set_blend_mode(BlendMode::Screen);

    let base_alpha = (color.a * intensity.clamp(0.0, 1.5)).clamp(0.0, 1.0);
    let stops = [
        (0.0, color.with_alpha(base_alpha)),
        (0.18, color.with_alpha(base_alpha * 0.75)),
        (0.45, color.with_alpha(base_alpha * 0.35)),
        (0.75, color.with_alpha(base_alpha * 0.10)),
        (1.0, color.with_alpha(0.0)),
    ];

    canvas.fill_radial_gradient(lx, ly, radius, &stops);
    canvas.restore();
}

/// Añade una capa de niebla ambiental / calina volumétrica con gradiente de profundidad.
///
/// Permite situar siluetas y personajes en distintos planos de profundidad visual.
///
/// * `horizon_y` – altura en Y del horizonte o foco de densidad máxima de niebla
/// * `fog_color` – color atmosférico (ej. sepia, azul crepuscular, ocre dorado)
/// * `density`   – opacidad máxima de la niebla en el horizonte (0.0 a 1.0)
pub fn ambient_fog(
    canvas: &mut Canvas,
    horizon_y: f32,
    fog_color: Color,
    density: f32,
) {
    if density <= 0.001 {
        return;
    }
    let _w = canvas.width as f32;
    let h = canvas.height as f32;

    canvas.save();
    canvas.set_blend_mode(BlendMode::SourceOver);

    // Gradiente vertical con mayor densidad cerca del horizonte
    let d = density.clamp(0.0, 0.95);
    let y_top = (horizon_y - h * 0.35).max(0.0);
    let y_bot = (horizon_y + h * 0.45).min(h);

    let stops = [
        (0.0, fog_color.with_alpha(d * 0.2)),
        (0.45, fog_color.with_alpha(d * 0.85)),
        (0.65, fog_color.with_alpha(d * 0.6)),
        (1.0, fog_color.with_alpha(d * 0.05)),
    ];

    canvas.fill_linear_gradient(0.0, y_top, 0.0, y_bot, &stops);
    canvas.restore();
}

