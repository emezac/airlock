//! A storyboard: the key frame of every scene, laid out on a grid with its
//! number, so a reviewer (human or model) can read the whole film at once.

use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::schematic::draw_vector_text;
use crate::spec::Spec;
use image::{imageops, RgbaImage};

pub fn render_board(spec: &Spec, tile_width: u32) -> RgbaImage {
    let (tw, th) = spec.size(tile_width);
    let n = spec.scenes.len() as u32;
    let columns = n.min(if spec.format == "9:16" { 4 } else { 3 }).max(1);
    let rows = n.div_ceil(columns);
    let gap = (tile_width / 24).max(6);
    let label_h = (tile_width / 10).max(14);
    let (bw, bh) = (columns * tw + (columns + 1) * gap, rows * (th + label_h) + (rows + 1) * gap);
    let mut board = RgbaImage::from_pixel(bw, bh, image::Rgba([245, 243, 238, 255]));

    let timeline = spec.timeline();
    for (i, frame) in spec.key_frames().into_iter().enumerate() {
        let mut canvas = Canvas::new(tw, th);
        timeline.render_frame(&mut canvas, frame);
        let tile = RgbaImage::from_raw(tw, th, canvas.data().to_vec()).expect("canvas size matches");
        let (col, row) = (i as u32 % columns, i as u32 / columns);
        let x = gap + col * (tw + gap);
        let y = gap + row * (th + label_h + gap);
        imageops::overlay(&mut board, &tile, x as i64, y as i64);

        let mut label = Canvas::new(tw, label_h);
        label.clear(Color::hex("#f5f3ee"));
        let text = format!("{}  {}", i + 1, spec.scenes[i].name);
        let size = label_h as f32 * 0.5;
        draw_vector_text(&mut label, 4.0, (label_h as f32 - size) / 2.0, &text, size, Color::hex("#2b2b2b"), false);
        let strip = RgbaImage::from_raw(tw, label_h, label.data().to_vec()).expect("label size matches");
        imageops::overlay(&mut board, &strip, x as i64, (y + th) as i64);
    }
    board
}

/// Mean absolute difference per channel, from 0 (identical) to 1.
/// Images of different sizes are maximally different.
pub fn difference(a: &RgbaImage, b: &RgbaImage) -> f64 {
    if a.dimensions() != b.dimensions() {
        return 1.0;
    }
    let total: u64 = a.as_raw().iter().zip(b.as_raw()).map(|(x, y)| x.abs_diff(*y) as u64).sum();
    total as f64 / (a.as_raw().len() as f64 * 255.0)
}

/// Pixels where any channel differs by more than `tolerance` (0–255).
/// Images of different sizes count every pixel as changed.
pub fn changed_pixels(a: &RgbaImage, b: &RgbaImage, tolerance: u8) -> u64 {
    if a.dimensions() != b.dimensions() {
        let (w, h) = a.dimensions().max(b.dimensions());
        return w as u64 * h as u64;
    }
    a.pixels()
        .zip(b.pixels())
        .filter(|(p, q)| p.0.iter().zip(q.0.iter()).any(|(x, y)| x.abs_diff(*y) > tolerance))
        .count() as u64
}
