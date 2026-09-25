use crate::core::rng::Rng;
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum Waveform {
    Sine,
    Triangle,
    Square,
    /// Diente de sierra suave (más agradable que Square para leads)
    Saw,
    Sawtooth,
}

/// Frecuencia en escala pentatónica según emezac/tools:
/// `pentHz = (o, s, base = 220) => base * Math.pow(2, o + [0, 2, 4, 7, 9][s % 5] / 12)`
pub fn pent_hz(octave: f32, step: usize, base: f32) -> f32 {
    let scale_semitones = [0.0, 2.0, 4.0, 7.0, 9.0];
    let semitone = scale_semitones[step % 5];
    base * 2.0_f32.powf(octave + semitone / 12.0)
}

/// Búfer de audio estéreo a 48 kHz para síntesis procedural
pub struct AudioTrack {
    pub sample_rate: u32,
    pub left: Vec<f32>,
    pub right: Vec<f32>,
}

impl AudioTrack {
    pub fn new(duration_secs: f32, sample_rate: u32) -> Self {
        let len = (duration_secs * sample_rate as f32).ceil() as usize;
        Self {
            sample_rate,
            left: vec![0.0; len],
            right: vec![0.0; len],
        }
    }

    /// Agrega una nota musical con envolvente ADSR suave
    /// gain recomendado: 0.06–0.14 (antes era 0.12–0.25; reducido ~40%)
    pub fn add_note(
        &mut self,
        start_secs: f32,
        duration_secs: f32,
        freq: f32,
        wave: Waveform,
        gain: f32,
        pan: f32, // -1.0 izquierda, 0.0 centro, +1.0 derecha
    ) {
        let start_sample = (start_secs * self.sample_rate as f32) as usize;
        let total_samples = (duration_secs * self.sample_rate as f32) as usize;
        let end_sample = (start_sample + total_samples).min(self.left.len());

        let attack_samples = (0.025 * self.sample_rate as f32) as usize;
        let release_samples = (0.08 * self.sample_rate as f32) as usize;
        let pan_l = ((1.0 - pan) / 2.0).clamp(0.0, 1.0).sqrt();
        let pan_r = ((1.0 + pan) / 2.0).clamp(0.0, 1.0).sqrt();

        for i in start_sample..end_sample {
            let sample_idx = i - start_sample;
            let t = sample_idx as f32 / self.sample_rate as f32;
            let remaining = total_samples.saturating_sub(sample_idx);

            // Envolvente ADSR: ataque lineal → sustain exponencial → release
            let env = if sample_idx < attack_samples {
                sample_idx as f32 / attack_samples as f32
            } else if remaining < release_samples {
                remaining as f32 / release_samples as f32
            } else {
                let decay_t = (sample_idx - attack_samples) as f32
                    / (total_samples.saturating_sub(attack_samples + release_samples).max(1)) as f32;
                (-decay_t * 3.5).exp() * 0.85 + 0.15
            };

            // Generador de oscilador
            let phase = 2.0 * PI * freq * t;
            let raw = match wave {
                Waveform::Sine => phase.sin(),
                Waveform::Triangle => {
                    let p = (phase / (2.0 * PI)).fract();
                    if p < 0.5 { 4.0 * p - 1.0 } else { 3.0 - 4.0 * p }
                }
                Waveform::Square => {
                    // Square suavizado con coeficiente 0.6 para evitar aliasing duro
                    if phase.sin() >= 0.0 { 0.6 } else { -0.6 }
                }
                Waveform::Saw | Waveform::Sawtooth => {
                    // Diente de sierra: rampa 0→1 por ciclo, centrada
                    let p = (phase / (2.0 * PI)).fract();
                    2.0 * p - 1.0
                }
            };

            // Suavizado soft-clip para evitar picos duros
            let sample_val = soft_clip(raw * env * gain);

            self.left[i] += sample_val * pan_l;
            self.right[i] += sample_val * pan_r;
        }
    }

    /// Pad ambiental: drone suave de fondo con mezcla lenta entre dos frecuencias.
    /// gain recomendado: 0.04–0.08
    pub fn add_ambient_pad(
        &mut self,
        start_secs: f32,
        duration_secs: f32,
        freq_low: f32,
        freq_high: f32,
        gain: f32,
        seed: u32,
    ) {
        let start_sample = (start_secs * self.sample_rate as f32) as usize;
        let total_samples = (duration_secs * self.sample_rate as f32) as usize;
        let end_sample = (start_sample + total_samples).min(self.left.len());

        let fade_samples = (0.4 * self.sample_rate as f32) as usize;
        let mut rng = Rng::new(seed);
        // Offset de fase aleatorio para evitar fase perfecta entre canales
        let phase_offset = rng.next_f32() * PI * 2.0;

        for i in start_sample..end_sample {
            let sample_idx = i - start_sample;
            let t = sample_idx as f32 / self.sample_rate as f32;
            let remaining = total_samples.saturating_sub(sample_idx);

            // Fade in / fade out suave
            let fade = if sample_idx < fade_samples {
                sample_idx as f32 / fade_samples as f32
            } else if remaining < fade_samples {
                remaining as f32 / fade_samples as f32
            } else {
                1.0
            };

            // LFO muy lento para modulación de mezcla
            let mix_t = (t * 0.15).sin() * 0.5 + 0.5;
            let freq = freq_low + (freq_high - freq_low) * mix_t;

            let phase = 2.0 * PI * freq * t;
            let sin_l = (phase + phase_offset).sin();
            let sin_r = (phase - phase_offset * 0.5).sin();

            // Armónico suave añadido (-12dB relativo al fundamental)
            let harm = (phase * 2.0 + phase_offset).sin() * 0.25;

            self.left[i] += soft_clip((sin_l + harm) * fade * gain);
            self.right[i] += soft_clip((sin_r + harm) * fade * gain);
        }
    }

    /// Ráfaga de ruido blanco filtrada (percusión, chasquido o crujido de papel).
    /// gain recomendado: 0.06–0.12 (antes era 0.28–0.35; reducido ~65%)
    pub fn add_noise_burst(
        &mut self,
        start_secs: f32,
        duration_secs: f32,
        gain: f32,
        seed: u32,
    ) {
        let start_sample = (start_secs * self.sample_rate as f32) as usize;
        let total_samples = (duration_secs * self.sample_rate as f32) as usize;
        let end_sample = (start_sample + total_samples).min(self.left.len());

        let mut rng = Rng::new(seed);
        // Filtro IIR de paso bajo muy simple (promedio de ventana 2) para suavizar ruido
        let mut prev_l = 0.0f32;
        let mut prev_r = 0.0f32;

        for i in start_sample..end_sample {
            let sample_idx = i - start_sample;
            let progress = sample_idx as f32 / total_samples as f32;
            // Envolvente más corta y suave: exponencial rápida
            let env = (-progress * 8.0).exp() * gain;

            let raw_l = rng.next_f32() * 2.0 - 1.0;
            let raw_r = rng.next_f32() * 2.0 - 1.0;

            // Filtro paso-bajo de primer orden
            let filtered_l = raw_l * 0.4 + prev_l * 0.6;
            let filtered_r = raw_r * 0.4 + prev_r * 0.6;
            prev_l = filtered_l;
            prev_r = filtered_r;

            self.left[i] += soft_clip(filtered_l * env);
            self.right[i] += soft_clip(filtered_r * env);
        }
    }

    /// Gota de agua procedural: tono puro con sweep de frecuencia rápido y decaimiento percusivo.
    /// Crea el característico "bloop/plink" orgánico de gotas sobre un charco.
    pub fn add_water_drop(
        &mut self,
        start_secs: f32,
        duration_secs: f32,
        base_freq: f32,
        pitch_sweep: f32, // Factor de barrido (ej: +0.4 sube tono, -0.2 baja tono)
        gain: f32,
        pan: f32,
    ) {
        let start_sample = (start_secs * self.sample_rate as f32) as usize;
        let total_samples = (duration_secs * self.sample_rate as f32) as usize;
        let end_sample = (start_sample + total_samples).min(self.left.len());

        let attack_samples = (0.003 * self.sample_rate as f32) as usize;
        let pan_l = ((1.0 - pan) / 2.0).clamp(0.0, 1.0).sqrt();
        let pan_r = ((1.0 + pan) / 2.0).clamp(0.0, 1.0).sqrt();

        let mut phase = 0.0f32;

        for i in start_sample..end_sample {
            let sample_idx = i - start_sample;
            let progress = sample_idx as f32 / total_samples as f32;

            // Envolvente de volumen: ataque casi instantáneo + decaimiento exponencial suave
            let env = if sample_idx < attack_samples {
                sample_idx as f32 / attack_samples as f32
            } else {
                (-progress * 18.0).exp()
            };

            // Frecuencia dinámica con sweep de tono
            let current_freq = base_freq * (1.0 + pitch_sweep * (-progress * 6.0).exp());
            phase += 2.0 * PI * current_freq / self.sample_rate as f32;

            // Tono sinusoidal con un toque sutil de segundo armónico (-14dB)
            let tone = phase.sin() + (phase * 2.0).sin() * 0.18;
            let sample_val = soft_clip(tone * env * gain);

            self.left[i] += sample_val * pan_l;
            self.right[i] += sample_val * pan_r;
        }
    }

    /// Normaliza el pico absoluto de la pista a `target_peak` (0.0–1.0).
    /// Equivalente a limitar en –3 dBFS cuando target_peak = 0.707.
    pub fn normalize_peak(&mut self, target_peak: f32) {
        let max_l = self.left.iter().copied().fold(0.0f32, |a, x| a.max(x.abs()));
        let max_r = self.right.iter().copied().fold(0.0f32, |a, x| a.max(x.abs()));
        let peak = max_l.max(max_r);

        if peak > 1e-6 {
            let ratio = target_peak / peak;
            for s in &mut self.left  { *s *= ratio; }
            for s in &mut self.right { *s *= ratio; }
        }
    }

    /// Guarda la pista en formato WAV PCM 16-bit estéreo mediante `hound`.
    /// Aplica normalización a –3 dBFS antes de escribir.
    pub fn save_wav(&self, path: &str) -> anyhow::Result<()> {
        // Clonar para normalizar sin modificar el original
        let peak_target = 0.707_f32; // –3 dBFS
        let max_l = self.left.iter().copied().fold(0.0f32, |a, x| a.max(x.abs()));
        let max_r = self.right.iter().copied().fold(0.0f32, |a, x| a.max(x.abs()));
        let peak = max_l.max(max_r);
        let ratio = if peak > 1e-6 { peak_target / peak } else { 1.0 };

        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(path, spec)?;
        for i in 0..self.left.len() {
            // Soft-limiter final antes de conversión a entero
            let l = soft_clip(self.left[i] * ratio);
            let r = soft_clip(self.right[i] * ratio);
            writer.write_sample((l.clamp(-1.0, 1.0) * 32767.0).round() as i16)?;
            writer.write_sample((r.clamp(-1.0, 1.0) * 32767.0).round() as i16)?;
        }
        writer.finalize()?;
        Ok(())
    }
}

/// Compresor/limiter suave tipo tanh: evita clipping duro manteniendo carácter tonal
#[inline(always)]
fn soft_clip(x: f32) -> f32 {
    // tanh aproximado de 3er orden: muy preciso para |x| < 2.0
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}
