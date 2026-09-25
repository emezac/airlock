use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette, FinishKind};
use crate::core::rng::Rng;
use tiny_skia::{Path, PathBuilder};

/// Opciones para el sombreado de rayado (hatching)
#[derive(Clone, Copy, Debug)]
pub struct HatchOptions {
    pub angle: f32,
    pub gap: f32,
    pub len: f32,
    pub jitter: f32,
    pub color: Color,
    pub alpha: f32,
    pub width: f32,
    pub seed: u32,
}

impl Default for HatchOptions {
    fn default() -> Self {
        Self {
            angle: 0.9,
            gap: 7.0,
            len: 14.0,
            jitter: 6.0,
            color: Color::rgba(0.12, 0.09, 0.19, 0.35),
            alpha: 0.35,
            width: 1.2,
            seed: 1,
        }
    }
}

/// Opciones para la trama de puntos / semitono (dotScreen)
#[derive(Clone, Copy, Debug)]
pub struct DotScreenOptions {
    pub cell: f32,
    pub color: Color,
    pub density: f32,
    pub angle: f32,
    pub jitter: f32,
    pub seed: u32,
    pub alpha: f32,
}

impl Default for DotScreenOptions {
    fn default() -> Self {
        Self {
            cell: 7.0,
            color: Color::rgba(0.0, 0.47, 0.75, 0.9),
            density: 0.5,
            angle: 0.26,
            jitter: 0.35,
            seed: 1,
            alpha: 0.9,
        }
    }
}

/// Trazo tembloroso orgánico (wob): deforma los puntos para dar apariencia hecha a mano
pub fn wob_path(points: &[[f32; 2]], amp: f32, seed: u32, close: bool) -> Option<Path> {
    if points.is_empty() {
        return None;
    }
    let mut rng = Rng::new(seed);
    let mut pb = PathBuilder::new();

    for (i, p) in points.iter().enumerate() {
        let x = p[0] + (rng.next_f32() - 0.5) * amp;
        let y = p[1] + (rng.next_f32() - 0.5) * amp;
        if i == 0 {
            pb.move_to(x, y);
        } else {
            pb.line_to(x, y);
        }
    }
    if close {
        pb.close();
    }
    pb.finish()
}

/// Trazo estilo crayón / rizo de papel en tres pasadas temblorosas
pub fn draw_crayon(
    canvas: &mut Canvas,
    points: &[[f32; 2]],
    color: Color,
    width: f32,
    seed: u32,
    close: bool,
) {
    for k in 0..3 {
        let alpha = if k == 0 { 0.85 } else { 0.35 };
        let w = width * (if k == 0 { 1.0 } else { 0.7 });
        let amp = 2.5 + k as f32 * 1.5;
        if let Some(path) = wob_path(points, amp, seed + k * 7, close) {
            canvas.stroke_path(&path, color.with_alpha(color.a * alpha), w);
        }
    }
}

/// Sombreado de rayado fino (hatch) contenido dentro de un trazado
pub fn draw_hatch(
    canvas: &mut Canvas,
    path: &Path,
    bbox: [f32; 4], // [x, y, w, h]
    opts: HatchOptions,
) {
    let mut rng = Rng::new(opts.seed);
    let [bx, by, bw, bh] = bbox;
    let cx = bx + bw / 2.0;
    let cy = by + bh / 2.0;
    let r_radius = (bw * bw + bh * bh).sqrt() / 2.0;
    let ca = opts.angle.cos();
    let sa = opts.angle.sin();

    let mut pb = PathBuilder::new();
    let mut has_lines = false;

    let mut v = -r_radius;
    while v <= r_radius {
        let mut u = -r_radius;
        while u <= r_radius {
            let uu = u + (rng.next_f32() - 0.5) * opts.jitter * 2.0;
            let len = opts.len * (0.6 + rng.next_f32() * 0.8);
            let u0 = uu - len / 2.0;
            let u1 = uu + len / 2.0;

            let x1 = cx + ca * u0 - sa * v;
            let y1 = cy + sa * u0 + ca * v;
            let x2 = cx + ca * u1 - sa * v;
            let y2 = cy + sa * u1 + ca * v;

            pb.move_to(x1, y1);
            pb.line_to(x2, y2);
            has_lines = true;

            u += opts.len * 1.7;
        }
        v += opts.gap;
    }

    if has_lines {
        if let Some(hatch_path) = pb.finish() {
            canvas.save();
            canvas.clip_path(path);
            canvas.stroke_path(&hatch_path, opts.color.with_alpha(opts.alpha), opts.width);
            canvas.restore();
        }
    }
}

/// Trama de semitonos (dotScreen) risográfica o serigráfica contenida en un trazado
pub fn draw_dot_screen(
    canvas: &mut Canvas,
    path: &Path,
    bbox: [f32; 4],
    opts: DotScreenOptions,
) {
    let mut rng = Rng::new(opts.seed);
    let [bx, by, bw, bh] = bbox;
    let cx = bx + bw / 2.0;
    let cy = by + bh / 2.0;
    let r_radius = (bw * bw + bh * bh).sqrt() / 2.0;
    let ca = opts.angle.cos();
    let sa = opts.angle.sin();

    canvas.save();
    canvas.clip_path(path);

    let cell = opts.cell.max(3.0);
    let max_radius = (cell / 2.0) * opts.density.clamp(0.0, 1.0).sqrt() * 0.95;

    let mut v = -r_radius;
    while v <= r_radius {
        let mut u = -r_radius;
        while u <= r_radius {
            let jx = (rng.next_f32() - 0.5) * opts.jitter * cell;
            let jy = (rng.next_f32() - 0.5) * opts.jitter * cell;
            let px = cx + ca * u - sa * v + jx;
            let py = cy + sa * u + ca * v + jy;

            if max_radius > 0.4 {
                canvas.fill_circle(px, py, max_radius, opts.color.with_alpha(opts.alpha));
            }

            u += cell;
        }
        v += cell;
    }

    canvas.restore();
}

/// Micrograno disperso para dar sensación de pulpa de papel o grafito
pub fn draw_grain(
    canvas: &mut Canvas,
    path: &Path,
    bbox: [f32; 4],
    count: usize,
    color: Color,
    alpha: f32,
    seed: u32,
    size: f32,
) {
    let mut rng = Rng::new(seed);
    let [bx, by, bw, bh] = bbox;

    canvas.save();
    canvas.clip_path(path);

    let col = color.with_alpha(alpha);
    for _ in 0..count {
        let px = bx + rng.next_f32() * bw;
        let py = by + rng.next_f32() * bh;
        let s = size * (0.6 + rng.next_f32() * 0.8);
        canvas.stroke_line(px, py, px + s, py + s * 0.5, col, 0.8);
    }

    canvas.restore();
}

/// Acabado de superficie inteligente según la paleta activa
pub fn draw_surface(
    canvas: &mut Canvas,
    path: &Path,
    bbox: [f32; 4],
    palette: &Palette,
    seed: u32,
) {
    match palette.finish {
        FinishKind::Ink => {
            let opts = HatchOptions {
                angle: 1.2,
                gap: 4.5,
                len: 9.0,
                jitter: 3.0,
                color: palette.shade,
                alpha: 0.35,
                width: 1.0,
                seed,
            };
            draw_hatch(canvas, path, bbox, opts);
            draw_grain(canvas, path, bbox, 120, palette.shade, 0.3, seed + 1, 1.4);
        }
        FinishKind::Riso => {
            let opts = DotScreenOptions {
                cell: 7.0,
                color: palette.shade,
                density: 0.55,
                angle: 0.26,
                jitter: 0.35,
                seed,
                alpha: 0.88,
            };
            draw_dot_screen(canvas, path, bbox, opts);
        }
        FinishKind::Screen => {
            let opts = DotScreenOptions {
                cell: 6.0,
                color: palette.shade,
                density: 0.5,
                angle: 0.0,
                jitter: 0.05,
                seed,
                alpha: 0.9,
            };
            draw_dot_screen(canvas, path, bbox, opts);
        }
        FinishKind::Pencil => {
            let opts = HatchOptions {
                angle: 1.1,
                gap: 9.0,
                len: 30.0,
                jitter: 4.0,
                color: palette.shade,
                alpha: 0.22,
                width: 0.7,
                seed,
            };
            draw_hatch(canvas, path, bbox, opts);
            draw_grain(canvas, path, bbox, 40, palette.shade, 0.25, seed + 1, 1.2);
        }
        FinishKind::Doodle | FinishKind::Flat => {
            // En estilo doodle el relleno suele ser plano o con ligero grano
            draw_grain(canvas, path, bbox, 50, palette.shade, 0.2, seed, 1.0);
        }
    }
}

/// Dibuja el fondo de papel artesanal con grano determinista y bandas de luz
pub fn draw_paper(canvas: &mut Canvas, palette: &Palette, seed: u32) {
    canvas.clear(palette.paper);

    let w = canvas.width as f32;
    let h = canvas.height as f32;

    // Bandas de luz diagonales tenues si la paleta las especifica
    if let Some(band_color) = palette.paper_band {
        let mut rng = Rng::new(seed);
        let band_count = 5;
        for i in 0..band_count {
            let offset = (i as f32 - 1.0) * (w / 3.0) + (rng.next_f32() - 0.5) * 40.0;
            let mut pb = PathBuilder::new();
            pb.move_to(offset, 0.0);
            pb.line_to(offset + w * 0.3, 0.0);
            pb.line_to(offset - w * 0.2, h);
            pb.line_to(offset - w * 0.5, h);
            pb.close();
            if let Some(path) = pb.finish() {
                canvas.fill_path(&path, band_color.with_alpha(0.08));
            }
        }
    }

    // Grano disperso sobre todo el fondo de papel
    let mut rng = Rng::new(seed + 100);
    let grain_count = ((w * h) / 1400.0) as usize; // ~600 granos a 1080p
    let grain_color = palette.ink.with_alpha(0.035);

    for _ in 0..grain_count {
        let gx = rng.next_f32() * w;
        let gy = rng.next_f32() * h;
        let gl = 1.0 + rng.next_f32() * 2.0;
        canvas.stroke_line(gx, gy, gx + gl, gy + gl * 0.3, grain_color, 0.7);
    }
}
