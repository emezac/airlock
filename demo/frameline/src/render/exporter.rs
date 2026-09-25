use crate::audio::AudioTrack;
use crate::core::canvas::Canvas;
use crate::timeline::FilmTimeline;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

/// Exporta la película completa a un archivo MP4 con audio AAC multiplexado mediante FFmpeg
pub fn export_mp4(
    timeline: &FilmTimeline,
    audio: Option<&AudioTrack>,
    external_audio_file: Option<&str>,
    width: u32,
    height: u32,
    output_mp4: &str,
    super_sample: u32,
) -> anyhow::Result<()> {
    let total_frames = timeline.total_frames();
    if total_frames == 0 {
        anyhow::bail!("La película no tiene fotogramas para renderizar");
    }

    let start_time = Instant::now();
    let fps = timeline.fps;
    let ss = super_sample.max(1);
    let render_w = width * ss;
    let render_h = height * ss;

    println!(
        "🎬 Iniciando renderizado de {} fotogramas a {}x{} @ {} fps (SSAA {}x: render a {}x{})...",
        total_frames, width, height, fps, ss, render_w, render_h
    );

    // 1. Determinar fuente de audio: archivo externo preferente, o synth procedural
    let (audio_input_path, is_temp_wav) = if let Some(ext_path) = external_audio_file {
        println!("🎵 Usando pista de audio externa: {}", ext_path);
        (Some(ext_path.to_string()), false)
    } else if let Some(track) = audio {
        let wav_path = format!("{}.temp.wav", output_mp4);
        track.save_wav(&wav_path)?;
        (Some(wav_path), true)
    } else {
        (None, false)
    };

    // 2. Configurar el subproceso de FFmpeg
    let mut ffmpeg_cmd = Command::new("ffmpeg");
    ffmpeg_cmd
        .arg("-y") // Sobrescribir
        .arg("-f").arg("rawvideo")
        .arg("-vcodec").arg("rawvideo")
        .arg("-s").arg(format!("{}x{}", width, height))
        .arg("-pix_fmt").arg("rgba")
        .arg("-r").arg(fps.to_string())
        .arg("-i").arg("-"); // Entrada por stdin

    if let Some(ref audio_path) = audio_input_path {
        ffmpeg_cmd.arg("-i").arg(audio_path);
    }

    ffmpeg_cmd
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-profile:v").arg("high")
        .arg("-level").arg("4.1")
        .arg("-colorspace").arg("bt709")
        .arg("-color_primaries").arg("bt709")
        .arg("-color_trc").arg("bt709")
        .arg("-crf").arg("15")
        .arg("-preset").arg(if ss > 1 { "medium" } else { "fast" });

    if audio_input_path.is_some() {
        ffmpeg_cmd
            .arg("-c:a").arg("aac")
            .arg("-b:a").arg("256k")
            .arg("-shortest");
    }

    ffmpeg_cmd
        .arg(output_mp4)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit());

    let mut child = ffmpeg_cmd.spawn()
        .map_err(|e| anyhow::anyhow!("No se pudo ejecutar ffmpeg. Asegúrate de tener ffmpeg instalado en el sistema: {}", e))?;

    let mut stdin = child.stdin.take().expect("No se pudo capturar stdin de ffmpeg");

    // 3. Renderizar y enviar fotogramas secuencialmente (con soporte para motion blur por doble buffer)
    let mut prev_canvas: Option<Canvas> = None;
    let mut prev_scene_idx: Option<usize> = None;

    for frame_idx in 0..total_frames {
        let mut canvas = Canvas::new(render_w, render_h);
        timeline.render_frame(&mut canvas, frame_idx);

        let current_scene_idx = timeline.scene_index_at_frame(frame_idx);

        if timeline.motion_blur > 0.001 {
            if let (Some(prev), Some(prev_idx)) = (&prev_canvas, prev_scene_idx) {
                if prev_idx == current_scene_idx {
                    canvas.composite_over(prev, timeline.motion_blur);
                }
            }
            let mut clone = Canvas::new(render_w, render_h);
            clone.pixmap.data_mut().copy_from_slice(canvas.data());
            prev_canvas = Some(clone);
            prev_scene_idx = Some(current_scene_idx);
        }

        if ss > 1 {
            // Downscale sub-pixel con Lanczos3 para grabado de máxima precisión
            let full_img = image::RgbaImage::from_raw(render_w, render_h, canvas.data().to_vec())
                .unwrap_or_else(|| image::RgbaImage::new(render_w, render_h));
            let downscaled = image::imageops::resize(
                &full_img,
                width,
                height,
                image::imageops::FilterType::Lanczos3,
            );
            stdin.write_all(downscaled.as_raw())?;
        } else {
            stdin.write_all(canvas.data())?;
        }

        if (frame_idx + 1) % (fps as usize) == 0 || frame_idx == total_frames - 1 {
            let elapsed = start_time.elapsed().as_secs_f32();
            let current_fps = (frame_idx + 1) as f32 / elapsed.max(0.001);
            print!("\r⏳ Progreso: [{}/{}] fotogramas ({:.1} fps)", frame_idx + 1, total_frames, current_fps);
            std::io::stdout().flush().ok();
        }
    }
    println!();

    drop(stdin); // Cerrar stdin para que ffmpeg finalice
    let status = child.wait()?;

    if !status.success() {
        anyhow::bail!("FFmpeg finalizó con código de error: {:?}", status.code());
    }

    // Limpiar archivo WAV temporal si se generó procedurally
    if is_temp_wav {
        if let Some(ref wav_file) = audio_input_path {
            let _ = std::fs::remove_file(wav_file);
        }
    }

    let elapsed = start_time.elapsed();
    println!("✨ ¡Video exportado con éxito en {:.2}s!: {}", elapsed.as_secs_f32(), output_mp4);

    Ok(())
}
