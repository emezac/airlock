use crate::audio::{pent_hz, AudioTrack, Waveform};
use crate::core::canvas::Canvas;
use crate::core::color::Palette;
use crate::core::finishes::{draw_paper, wob_path};
use crate::puppet::PaperBoat;
use crate::timeline::{FilmTimeline, Scene};
use std::f32::consts::PI;

/// Escena del barco con una paleta y acabado específicos
pub struct BoatScene {
    pub title: String,
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for BoatScene {
    fn name(&self) -> &str {
        &self.title
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = (frame as u32) + 1;

        // 1. Dibujar fondo de papel con grano determinista
        draw_paper(canvas, &self.palette, seed);

        // 2. Ondas de horizonte o fondo según el estilo
        let w = canvas.width as f32;
        let h = canvas.height as f32;
        let cy = h * 0.55;

        // Línea de horizonte flotante
        let horizon_pts = [
            [0.0, cy],
            [w * 0.3, cy + 10.0 * (tau * PI * 2.0).sin()],
            [w * 0.7, cy - 8.0 * (tau * PI * 2.0 + 1.0).cos()],
            [w, cy],
        ];
        if let Some(path) = wob_path(&horizon_pts, 2.0, seed + 10, false) {
            canvas.stroke_path(&path, self.palette.ink.with_alpha(0.4), 1.2);
        }

        // 3. Posición y movimiento del barco (vaivén sinusoidal)
        let bob_angle = (tau * PI * 4.0).sin() * 0.08;
        let boat_y = cy - 20.0 + (tau * PI * 4.0).cos() * 8.0;
        let boat_x = w * 0.5 + (tau - 0.5) * 60.0;

        let boat = PaperBoat {
            x: boat_x,
            y: boat_y,
            scale: 1.6,
            bob_angle,
        };

        boat.draw(canvas, &self.palette, seed + 20);

        // 4. Punto semilla (seed dot) característico si es estilo Riso
        if self.palette.name.contains("riso") {
            let seed_col = self.palette.accents.first().copied().unwrap_or(self.palette.blush);
            canvas.fill_circle(w * 0.12, h * 0.15, 6.0, seed_col);
        }
    }
}

/// Construye la película "Four Looks" (Barco en Riso, Screen, Pencil e Ink) de 12 segundos
pub fn create_four_looks_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();

    let fps = 24;
    let scene_frames = fps * 3; // 3 segundos por look (72 fotogramas) = 12 segundos total

    // Look 1: Riso Pop
    timeline.add_scene(BoatScene {
        title: "1. Risografía Fluorescente (risoPop)".into(),
        palette: Palette::riso_pop(),
        duration: scene_frames,
    });

    // Look 2: Flat Screen Print
    timeline.add_scene(BoatScene {
        title: "2. Serigrafía Plana (screenSea)".into(),
        palette: Palette::screen_sea(),
        duration: scene_frames,
    });

    // Look 3: Graphite Minimalism
    timeline.add_scene(BoatScene {
        title: "3. Minimalismo de Grafito (pencilMinimal)".into(),
        palette: Palette::pencil_minimal(),
        duration: scene_frames,
    });

    // Look 4: Ink on Warm Paper
    timeline.add_scene(BoatScene {
        title: "4. Tinta sobre Papel Cálido (paperInk)".into(),
        palette: Palette::paper_ink(),
        duration: scene_frames,
    });

    // Síntesis de banda sonora procedural sincronizada con los cortes
    let total_secs = timeline.total_duration_secs();
    let mut audio = AudioTrack::new(total_secs, 48000);

    // Cada 3 segundos ocurre un corte de escena: golpe de campana pentatónica y chasquido de papel
    for i in 0..4 {
        let cut_time = i as f32 * 3.0;

        // Golpe de percusión sutil en el corte
        audio.add_noise_burst(cut_time, 0.18, 0.10, (i as u32) + 1);

        // Acorde o arpegio pentatónico
        let f1 = pent_hz(1.0, i * 2, 220.0);
        let f2 = pent_hz(1.0, i * 2 + 2, 220.0);
        let f3 = pent_hz(2.0, i * 2 + 4, 220.0);

        audio.add_note(cut_time, 1.8, f1, Waveform::Triangle, 0.10, -0.3);
        audio.add_note(cut_time + 0.15, 1.6, f2, Waveform::Triangle, 0.08, 0.3);
        audio.add_note(cut_time + 0.30, 2.2, f3, Waveform::Sine, 0.07, 0.0);

        // Pulsos rítmicos intermedios
        audio.add_note(cut_time + 1.0, 0.5, pent_hz(0.0, i + 1, 220.0), Waveform::Sine, 0.06, -0.2);
        audio.add_note(cut_time + 2.0, 0.5, pent_hz(0.0, i + 3, 220.0), Waveform::Sine, 0.06, 0.2);
    }

    (timeline, audio)
}
