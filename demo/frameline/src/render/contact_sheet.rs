use crate::core::canvas::Canvas;
use crate::timeline::FilmTimeline;
use image::{Rgb, RgbImage};
use rayon::prelude::*;

/// Genera una hoja de contacto (contact sheet) con `tile_count` fotogramas distribuidos uniformemente
pub fn generate_contact_sheet(
    timeline: &FilmTimeline,
    width: u32,
    height: u32,
    tile_count: usize,
    columns: usize,
    output_path: &str,
    super_sample: u32,
) -> anyhow::Result<()> {
    let total_frames = timeline.total_frames().max(1);
    let rows = (tile_count + columns - 1) / columns;

    let thumb_w = width / 4;
    let thumb_h = height / 4;
    let gap = 12;

    let sheet_w = columns as u32 * thumb_w + (columns as u32 + 1) * gap;
    let sheet_h = rows as u32 * thumb_h + (rows as u32 + 1) * gap;

    let ss = super_sample.max(1);
    let render_w = width * ss;
    let render_h = height * ss;

    // Calcular los índices de fotogramas a muestrear (punto medio exacto de cada escena si tile_count == escenas)
    let frame_indices: Vec<usize> = if tile_count == timeline.scenes.len() && tile_count > 0 {
        let mut indices = Vec::with_capacity(tile_count);
        let mut accum = 0;
        for scene in &timeline.scenes {
            let d = scene.duration_frames();
            indices.push((accum + d / 2).min(total_frames - 1));
            accum += d;
        }
        indices
    } else {
        (0..tile_count)
            .map(|i| {
                if tile_count > 1 {
                    (i * (total_frames - 1)) / (tile_count - 1)
                } else {
                    0
                }
            })
            .collect()
    };

    // Renderizar fotogramas en paralelo con Rayon a resolución supersampleada
    let rendered_thumbs: Vec<(usize, Canvas)> = frame_indices
        .par_iter()
        .map(|&f| {
            let mut canvas = Canvas::new(render_w, render_h);
            timeline.render_frame(&mut canvas, f);
            (f, canvas)
        })
        .collect();

    // Crear la imagen compuesta de la hoja de contactos
    let mut sheet = RgbImage::from_pixel(sheet_w, sheet_h, Rgb([24, 26, 32]));

    for (idx, (_frame_num, canvas)) in rendered_thumbs.iter().enumerate() {
        let col = idx % columns;
        let row = idx / columns;
        let x0 = gap + col as u32 * (thumb_w + gap);
        let y0 = gap + row as u32 * (thumb_h + gap);

        // Reescalar fotograma al tamaño del thumbnail con Lanczos3 para preservar líneas finas de grabado
        let full_img = image::RgbaImage::from_raw(render_w, render_h, canvas.data().to_vec())
            .unwrap_or_else(|| image::RgbaImage::new(render_w, render_h));
        let thumb = image::imageops::resize(
            &full_img,
            thumb_w,
            thumb_h,
            image::imageops::FilterType::Lanczos3,
        );

        // Copiar sobre el lienzo de la hoja
        for ty in 0..thumb_h {
            for tx in 0..thumb_w {
                let p = thumb.get_pixel(tx, ty);
                sheet.put_pixel(x0 + tx, y0 + ty, Rgb([p[0], p[1], p[2]]));
            }
        }
    }

    sheet.save(output_path)?;
    println!("✅ Hoja de contacto generada: {} ({} fotogramas en {}x{}, SSAA {}x)", output_path, tile_count, columns, rows, ss);
    Ok(())
}
