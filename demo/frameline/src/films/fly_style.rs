use crate::audio::{pent_hz, AudioTrack, Waveform};
use crate::core::canvas::Canvas;
use crate::core::color::Palette;
use crate::core::finishes::draw_paper;
use crate::puppet::FruitFly;
use crate::timeline::{FilmTimeline, Scene};
use std::f32::consts::PI;

/// Escena 1: Observación de laboratorio con líneas técnicas
pub struct FlyLabScene {
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for FlyLabScene {
    fn name(&self) -> &str {
        "1. Laboratorio e Inspección Técnica"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = (frame as u32) + 1;
        draw_paper(canvas, &self.palette, seed);

        let w = canvas.width as f32;
        let h = canvas.height as f32;

        let fly = FruitFly {
            x: w * 0.5,
            y: h * 0.5,
            scale: 1.5 + (tau * PI).sin() * 0.1,
            wing_flap: tau * 2.0,
            tilt: (tau * PI * 2.0).sin() * 0.05,
        };

        fly.draw(canvas, &self.palette, seed, true);
    }
}

/// Escena 2: Vuelo dinámico a través de la pantalla
pub struct FlyFlightScene {
    pub palette: Palette,
    pub duration: usize,
}

impl Scene for FlyFlightScene {
    fn name(&self) -> &str {
        "2. Vuelo Cinético y Batimiento de Alas"
    }

    fn duration_frames(&self) -> usize {
        self.duration
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = (frame as u32) + 1;
        draw_paper(canvas, &self.palette, seed);

        let w = canvas.width as f32;
        let h = canvas.height as f32;

        // Trayectoria curva en forma de S
        let x = w * (0.1 + tau * 0.8);
        let y = h * 0.5 + (tau * PI * 4.0).sin() * 120.0;
        let tilt = (tau * PI * 4.0).cos() * 0.4;

        let fly = FruitFly {
            x,
            y,
            scale: 1.8,
            wing_flap: tau * 24.0, // Batimiento rápido
            tilt,
        };

        fly.draw(canvas, &self.palette, seed, false);
    }
}

/// Construye la película "Fly Style" (Inspirada en 'The life of a fruit fly') de 6 segundos
pub fn create_fly_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();

    let fps = 24;
    let scene_frames = fps * 3; // 3 segundos por escena

    timeline.add_scene(FlyLabScene {
        palette: Palette::paper_ink(),
        duration: scene_frames,
    });

    timeline.add_scene(FlyFlightScene {
        palette: Palette::blueprint_night(),
        duration: scene_frames,
    });

    let total_secs = timeline.total_duration_secs();
    let mut audio = AudioTrack::new(total_secs, 48000);

    // Audio temático: zumbido sintetizado en micro-tonos y cortes limpios
    for s in 0..2 {
        let cut_t = s as f32 * 3.0;
        audio.add_noise_burst(cut_t, 0.15, 0.10, s as u32 + 10);
        audio.add_note(cut_t, 2.5, pent_hz(1.0, s * 3, 275.0), Waveform::Sine, 0.10, 0.0);
    }

    (timeline, audio)
}
