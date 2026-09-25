//! Película Procedural "Okazz Rain" (Puddle Ripples & Splashes)
//! Recrea fielmente el efecto generativo de @okazz_ (p5.js / Creative Coding):
//! https://x.com/okazz_/status/2100938132002914437
//!
//! Características:
//! 1. Plano isométrico en ángulo rasante (~25°) con elipses horizontales (3.2:1).
//! 2. Escalado de profundidad progresivo (los impactos superiores son distantes y pequeños; los inferiores, cercanos y grandes).
//! 3. Gotas en caída libre acelerada con trazo de cápsula vertical de color vivo.
//! 4. Ondas concéntricas (2-3 por impacto) con expansión no lineal (ease-out cubic) y desvanecimiento suave.
//! 5. Perlas y cuentas que orbitan o reposan en el perímetro exterior de las ondas.
//! 6. Corona de salpicaduras (Splash Crown): abanico de 8 a 14 rayos balísticos disparados hacia arriba.
//! 7. Gotas esféricas en las puntas de los rayos con gravedad balística.
//! 8. Paleta cromática emblemática de Okazz: fondo carbón profundo con cyan, magenta, amarillo sol, menta y blanco hielo.
//! 9. Banda sonora procedural con síntesis de gotas de agua pentatónicas estéreo sincronizadas y rumor de lluvia.

use crate::audio::{pent_hz, AudioTrack, Waveform};
use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette};
use crate::core::rng::Rng;
use crate::timeline::{FilmTimeline, Scene};
use std::f32::consts::PI;
use tiny_skia::{PathBuilder, Stroke};

// ─────────────────────────────────────────────────────────────────────────────
// Estructuras de Datos del Modelo Físico / Generativo
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SplashRay {
    pub angle: f32,       // Ángulo en radianes (apuntando hacia arriba, [-155°, -25°])
    pub max_len: f32,     // Longitud máxima alcanzada
    pub speed: f32,       // Velocidad relativa
    pub has_tip_dot: bool,// Si lleva una perla en la punta
    pub tip_dot_radius: f32,
    pub color_idx: usize, // 0: primary, 1: secondary, 2: accent
}

#[derive(Clone, Debug)]
pub struct RingBead {
    pub ring_index: usize, // A qué anillo concéntrico pertenece
    pub initial_angle: f32,// Ángulo inicial en la elipse
    pub angular_speed: f32,// Velocidad de rotación a lo largo del perímetro
    pub radius: f32,       // Radio del punto
    pub color_idx: usize,
}

#[derive(Clone, Debug)]
pub struct RainDrop {
    pub gx: f32,          // Posición X en el lienzo (normalizada 0..1)
    pub gy: f32,          // Posición Y de impacto en el charco (normalizada 0..1)
    pub t_impact: f32,    // Tiempo de impacto en segundos
    pub fall_duration: f32, // Duración de la caída en segundos
    pub fall_height_ratio: f32, // Altura de caída
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub ring_max_radii: [f32; 3], // Radios máximos de las 3 ondas concéntricas
    pub ring_delays: [f32; 3],    // Retardo de inicio de cada onda
    pub ring_durations: [f32; 3], // Duración de cada onda
    pub rays: Vec<SplashRay>,
    pub beads: Vec<RingBead>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Generador Determinista de Gotas según la Fase de la Película
// ─────────────────────────────────────────────────────────────────────────────

fn generate_rain_sequence(seed: u32, total_duration: f32) -> Vec<RainDrop> {
    let mut rng = Rng::new(seed);
    let mut drops = Vec::new();

    // Paleta de colores vivos característica de Okazz
    let colors = [
        Color::hex("#00f0ff"), // Cyan neón
        Color::hex("#ff2a6d"), // Magenta vibrante
        Color::hex("#ffb703"), // Amarillo sol / ámbar cálido
        Color::hex("#05ffa1"), // Menta / verde agua eléctrico
        Color::hex("#ffffff"), // Blanco puro hielo
        Color::hex("#7000ff"), // Violeta eléctrico
        Color::hex("#ff7b00"), // Naranja fuego
        Color::hex("#4361ee"), // Azul real brillante
    ];

    // Cantidad total de gotas calculada según el flujo temporal
    // Lluvia constante con progresión de densidad
    let mut t = 0.08;
    while t < total_duration - 0.15 {
        // Tasa de generación variable (frecuencia entre gotas)
        let rate = if t < 2.5 {
            // Fase 1: Gotas iniciales elegantes (cada 0.10s - 0.16s)
            0.10 + rng.next_f32() * 0.06
        } else if t < 7.0 {
            // Fase 2: Lluvia continua rítmica (cada 0.06s - 0.12s)
            0.06 + rng.next_f32() * 0.06
        } else if t < 13.0 {
            // Fase 3: Chubasco neón pleno y envolvente (cada 0.04s - 0.08s)
            0.04 + rng.next_f32() * 0.04
        } else {
            // Fase 4: Despeje gradual con grandes ondas resonantes (cada 0.08s - 0.15s)
            0.08 + rng.next_f32() * 0.07
        };

        // Posición de impacto en el charco
        // Margen horizontal [0.05, 0.95]
        let gx = 0.06 + rng.next_f32() * 0.88;
        // Margen vertical con distribución en profundidad
        let gy_raw = rng.next_f32();
        let gy = 0.10 + gy_raw.powf(0.82) * 0.84; // Buena distribución con perspectiva

        // Selección armónica de colores (primario, secundario, acento)
        let col_idx1 = (rng.next_u32() as usize) % colors.len();
        let mut col_idx2 = (col_idx1 + 1 + (rng.next_u32() as usize % (colors.len() - 1))) % colors.len();
        if col_idx2 == col_idx1 {
            col_idx2 = (col_idx1 + 2) % colors.len();
        }
        let col_idx3 = if rng.next_f32() > 0.35 { 4 } else { (col_idx2 + 2) % colors.len() }; // Blanco frecuente como acento

        let primary_color = colors[col_idx1];
        let secondary_color = colors[col_idx2];
        let accent_color = colors[col_idx3];

        // Ondas concéntricas: radios más generosos y mayor duración (1.4s a 2.3s)
        let base_r = 100.0 + rng.next_f32() * 90.0;
        let ring_max_radii = [
            base_r * (0.45 + rng.next_f32() * 0.15),
            base_r * (0.80 + rng.next_f32() * 0.25),
            base_r * (1.15 + rng.next_f32() * 0.35),
        ];
        let ring_delays = [
            0.0,
            0.04 + rng.next_f32() * 0.06,
            0.10 + rng.next_f32() * 0.08,
        ];
        let ring_durations = [
            1.20 + rng.next_f32() * 0.40,
            1.50 + rng.next_f32() * 0.50,
            1.80 + rng.next_f32() * 0.50,
        ];

        // Generar rayos de la corona de salpicadura (8 a 14 rayos)
        let ray_count = 8 + (rng.next_u32() % 7) as usize;
        let mut rays = Vec::with_capacity(ray_count);
        for i in 0..ray_count {
            // Ángulo en abanico fanning upward: [-162°, -18°]
            let angle_deg = -162.0 + (i as f32 / (ray_count - 1) as f32) * 144.0 + (rng.next_f32() - 0.5) * 10.0;
            let angle = angle_deg.to_radians();
            let max_len = 45.0 + rng.next_f32() * 65.0;
            let speed = 0.90 + rng.next_f32() * 0.35;
            let has_tip_dot = rng.next_f32() > 0.30;
            let tip_dot_radius = 2.4 + rng.next_f32() * 2.4;
            let color_idx = (i + rng.next_u32() as usize) % 3;

            rays.push(SplashRay {
                angle,
                max_len,
                speed,
                has_tip_dot,
                tip_dot_radius,
                color_idx,
            });
        }

        // Generar perlas en las ondas (1 a 3 perlas en el borde exterior)
        let bead_count = if rng.next_f32() > 0.25 { 1 + (rng.next_u32() % 3) as usize } else { 0 };
        let mut beads = Vec::with_capacity(bead_count);
        for b in 0..bead_count {
            let ring_index = if b == 0 { 0 } else { 1 };
            let initial_angle = rng.next_f32() * PI * 2.0;
            let angular_speed = (rng.next_f32() - 0.5) * 1.8; // Giro suave a lo largo de la elipse
            let radius = 3.0 + rng.next_f32() * 2.5;
            let color_idx = if b % 2 == 0 { 2 } else { 0 };
            beads.push(RingBead {
                ring_index,
                initial_angle,
                angular_speed,
                radius,
                color_idx,
            });
        }

        let fall_duration = 0.28 + rng.next_f32() * 0.10;
        let fall_height_ratio = 0.20 + rng.next_f32() * 0.12;

        drops.push(RainDrop {
            gx,
            gy,
            t_impact: t,
            fall_duration,
            fall_height_ratio,
            primary_color,
            secondary_color,
            accent_color,
            ring_max_radii,
            ring_delays,
            ring_durations,
            rays,
            beads,
        });

        t += rate;
    }

    drops
}

// ─────────────────────────────────────────────────────────────────────────────
// Escena Principal: OkazzRainScene
// ─────────────────────────────────────────────────────────────────────────────

pub struct OkazzRainScene {
    pub duration_frames: usize,
    pub palette: Palette,
    pub drops: Vec<RainDrop>,
}

impl OkazzRainScene {
    pub fn new(duration_frames: usize, seed: u32) -> Self {
        let total_secs = duration_frames as f32 / 24.0;
        let drops = generate_rain_sequence(seed, total_secs);
        Self {
            duration_frames,
            palette: Palette::paper_ink(), // Base de paleta
            drops,
        }
    }
}

impl Scene for OkazzRainScene {
    fn name(&self) -> &str {
        "Okazz Rain: Generative Puddle"
    }

    fn duration_frames(&self) -> usize {
        self.duration_frames
    }

    fn palette(&self) -> &Palette {
        &self.palette
    }

    fn render(&self, canvas: &mut Canvas, tau: f32, _frame: usize) {
        let w = canvas.width as f32;
        let h = canvas.height as f32;
        let current_time = tau * (self.duration_frames as f32 / 24.0);

        // 1. Fondo carbón oscuro profundo (#111216) uniforme, exactamente como en la obra original
        canvas.clear(Color::hex("#111216"));

        // 2. Filtrar gotas activas (en caída o en fase de onda/salpicadura)
        // Para respetar la perspectiva isométrica, las ordenamos por `gy` (de arriba a abajo)
        let mut active_indices: Vec<usize> = self
            .drops
            .iter()
            .enumerate()
            .filter(|(_, drop)| {
                let t_start = drop.t_impact - drop.fall_duration;
                let t_end = drop.t_impact + 2.5; // Vida máxima de las ondas
                current_time >= t_start && current_time <= t_end
            })
            .map(|(idx, _)| idx)
            .collect();

        // Ordenamiento por profundidad Y
        active_indices.sort_by(|&a, &b| {
            self.drops[a]
                .gy
                .partial_cmp(&self.drops[b].gy)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 3. Renderizar cada gota activa
        for &idx in &active_indices {
            let drop = &self.drops[idx];
            let x = drop.gx * w;
            let y = drop.gy * h;

            // Factor de perspectiva por profundidad:
            // Escala geométrica para mantener nitidez y presencia en toda la pantalla
            let depth = 0.45 + 0.55 * drop.gy;
            let size_scale = 0.65 + 0.45 * drop.gy;
            let ellipse_ratio = 0.29; // ry / rx = 0.29

            let dt = current_time - drop.t_impact;

            // A. GOTA EN CAÍDA (dt < 0.0)
            if dt < 0.0 {
                let fall_t = (current_time - (drop.t_impact - drop.fall_duration)) / drop.fall_duration;
                if fall_t >= 0.0 && fall_t <= 1.0 {
                    // Caída acelerada por gravedad
                    let fall_h = drop.fall_height_ratio * h * depth;
                    let cur_y = y - fall_h * (1.0 - fall_t * fall_t);

                    // Trazo vertical alargado (cápsula / aguja de lluvia gruesa y bien definida)
                    let drop_len = (18.0 + 32.0 * fall_t) * size_scale;
                    let drop_width = (4.5 * size_scale).max(2.8);
                    let drop_color = drop.primary_color;

                    // Dibujar cápsula vertical con terminales redondos
                    canvas.stroke_line(x, cur_y - drop_len, x, cur_y, drop_color, drop_width);

                    // Cabeza de la gota (punto redondeado de impacto)
                    canvas.fill_circle(x, cur_y, drop_width * 0.75, drop.accent_color);
                }
            } else {
                // B. ONDAS CONCÉNTRICAS EXPANSIVAS (dt >= 0.0)
                for ring_i in 0..3 {
                    let ring_dt = dt - drop.ring_delays[ring_i];
                    let ring_dur = drop.ring_durations[ring_i];

                    if ring_dt >= 0.0 && ring_dt <= ring_dur {
                        let prog = (ring_dt / ring_dur).clamp(0.0, 1.0);

                        // Easing cúbico hacia afuera (rápido inicio, suave desaceleración)
                        let eased = 1.0 - (1.0 - prog).powf(3.0);
                        let rx = drop.ring_max_radii[ring_i] * depth * eased;
                        let ry = rx * ellipse_ratio;

                        // Curva de opacidad de Okazz: completamente sólida (1.0) la mayor parte del tiempo,
                        // desvaneciéndose limpiamente solo en el último tercio
                        let alpha = if prog < 0.60 {
                            1.0
                        } else {
                            ((1.0 - prog) / 0.40).powf(1.3)
                        };

                        let ring_color = match ring_i {
                            0 => drop.primary_color,
                            1 => drop.secondary_color,
                            _ => drop.accent_color,
                        }
                        .with_alpha(alpha);

                        // Grosor del trazo audaz y visible (3.0px a 5.0px)
                        let stroke_w = match ring_i {
                            0 => (4.5 - 1.2 * prog) * size_scale,
                            1 => (3.6 - 1.0 * prog) * size_scale,
                            _ => (2.8 - 0.8 * prog) * size_scale,
                        }
                        .max(2.0);

                        // Trazar elipse concéntrica
                        draw_ellipse_stroke(canvas, x, y, rx, ry, ring_color, stroke_w);

                        // Perlas en el perímetro exterior
                        for bead in &drop.beads {
                            if bead.ring_index == ring_i {
                                let bead_ang = bead.initial_angle + bead.angular_speed * ring_dt;
                                let bx = x + rx * bead_ang.cos();
                                let by = y + ry * bead_ang.sin();
                                let b_color = match bead.color_idx {
                                    0 => drop.primary_color,
                                    1 => drop.secondary_color,
                                    _ => drop.accent_color,
                                }
                                .with_alpha(alpha);
                                let b_rad = (bead.radius * size_scale).max(2.5);
                                canvas.fill_circle(bx, by, b_rad, b_color);
                            }
                        }
                    }
                }

                // C. CORONA DE SALPICADURAS (SPLASH CROWN RAYS & BEADS)
                let splash_dur = 0.55; // Duración de la salpicadura
                if dt <= splash_dur {
                    let splash_t = dt / splash_dur;

                    for ray in &drop.rays {
                        // Progreso de punta y cola en rayos rectos fanning (efecto aguja voladora)
                        let tip_prog = (splash_t * ray.speed / 0.55).min(1.0);
                        let tail_prog = ((splash_t * ray.speed - 0.16) / 0.84).max(0.0).min(1.0);

                        let eased_tip = 1.0 - (1.0 - tip_prog).powf(2.0);
                        let eased_tail = 1.0 - (1.0 - tail_prog).powf(2.0);

                        let cur_len_tip = ray.max_len * size_scale * eased_tip;
                        let cur_len_tail = ray.max_len * size_scale * eased_tail;

                        // Rayos rectos vectoriales característicos de Okazz
                        let head_x = x + ray.angle.cos() * cur_len_tip;
                        let head_y = y + ray.angle.sin() * cur_len_tip;

                        let tail_x = x + ray.angle.cos() * cur_len_tail;
                        let tail_y = y + ray.angle.sin() * cur_len_tail;

                        // Opacidad del rayo: nítido y presente
                        let ray_alpha = if splash_t < 0.65 {
                            1.0
                        } else {
                            ((1.0 - splash_t) / 0.35).powf(1.2)
                        };

                        let ray_color = match ray.color_idx {
                            0 => drop.primary_color,
                            1 => drop.secondary_color,
                            _ => drop.accent_color,
                        }
                        .with_alpha(ray_alpha);

                        let ray_width = ((3.4 - 1.0 * splash_t) * size_scale).max(2.0);

                        // Dibujar segmento del rayo volante
                        if (head_x - tail_x).hypot(head_y - tail_y) > 1.0 {
                            canvas.stroke_line(tail_x, tail_y, head_x, head_y, ray_color, ray_width);
                        }

                        // Perla en la punta del rayo (droplet dot)
                        if ray.has_tip_dot && splash_t < 0.88 {
                            let dot_alpha = if splash_t < 0.60 {
                                1.0
                            } else {
                                ((1.0 - splash_t) / 0.40).max(0.0)
                            };
                            let dot_color = drop.accent_color.with_alpha(dot_alpha);
                            let dot_rad = (ray.tip_dot_radius * size_scale).max(2.2);
                            canvas.fill_circle(head_x, head_y, dot_rad, dot_color);
                        }
                    }

                    // Destello inicial en el epicentro de impacto
                    if dt < 0.08 {
                        let flash_prog = dt / 0.08;
                        let flash_r = (16.0 * (1.0 - flash_prog) * size_scale).max(3.0);
                        let flash_col = drop.accent_color.with_alpha(1.0 - flash_prog);
                        canvas.fill_circle(x, y, flash_r, flash_col);
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Utilidad de Trazado Preciso de Elipses con tiny-skia
// ─────────────────────────────────────────────────────────────────────────────

fn draw_ellipse_stroke(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    color: Color,
    width: f32,
) {
    if rx < 1.0 || ry < 0.3 {
        return;
    }
    let mut pb = PathBuilder::new();
    let steps = 48; // 48 segmentos para elipse ultra suave y nítida
    for i in 0..=steps {
        let theta = (i as f32 / steps as f32) * PI * 2.0;
        let px = cx + rx * theta.cos();
        let py = cy + ry * theta.sin();
        if i == 0 {
            pb.move_to(px, py);
        } else {
            pb.line_to(px, py);
        }
    }
    pb.close();

    if let Some(path) = pb.finish() {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(color.to_tiny_skia());
        paint.anti_alias = true;

        let stroke = Stroke {
            width,
            miter_limit: 4.0,
            line_cap: tiny_skia::LineCap::Round,
            line_join: tiny_skia::LineJoin::Round,
            dash: None,
        };

        canvas.pixmap.stroke_path(&path, &paint, &stroke, tiny_skia::Transform::identity(), None);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Constructor de la Película "Okazz Rain"
// ─────────────────────────────────────────────────────────────────────────────

/// Construye la película completa "Okazz Rain" (16 segundos a 24 fps = 384 fotogramas)
/// con síntesis de audio de gotas pentatónicas ("bloop" / chimes) y lluvia ambiental.
pub fn create_okazz_film() -> (FilmTimeline, AudioTrack) {
    let mut timeline = FilmTimeline::new();
    timeline.fps = 24;
    timeline.on_twos = false; // 24 fps continuos para máxima fluidez en ondas y partículas
    timeline.motion_blur = 0.0; // 0.0 para nitidez de vector puro (sin ghosting en gotas verticales)

    let total_frames = 24 * 16; // 16 segundos
    let seed = 2026_0918;

    let scene = OkazzRainScene::new(total_frames, seed);
    let drops_for_audio = scene.drops.clone();
    timeline.add_scene(scene);

    // ─────────────────────────────────────────────────────────────────────────
    // Diseño Sonoro Procedural: Gotas de Agua Pentatónicas & Lluvia Cálida
    // ─────────────────────────────────────────────────────────────────────────
    let total_secs = 16.0;
    let mut audio = AudioTrack::new(total_secs, 44100);

    // 1. Textura de lluvia constante y suave (filtrado cálido)
    // Se divide en fragmentos continuos con ganancia suave
    let mut noise_t = 0.0;
    let mut noise_seed = 420;
    while noise_t < total_secs {
        audio.add_noise_burst(noise_t, 0.45, 0.035, noise_seed);
        noise_t += 0.35;
        noise_seed = noise_seed.wrapping_add(17);
    }

    // 2. Drone sub-bass cálido y envolvente (acorde tonal en Re menor / pentatónica)
    audio.add_ambient_pad(0.0, total_secs, 73.42, 110.0, 0.06, 1234);
    audio.add_ambient_pad(0.0, total_secs, 146.83, 220.0, 0.035, 5678);

    // 3. Síntesis de "Water Drops" en cada impacto
    // Frecuencias pentatónicas armoniosas con paneo espacial según `gx`
    for (i, drop) in drops_for_audio.iter().enumerate() {
        if drop.t_impact >= total_secs - 0.1 {
            continue;
        }

        // Frecuencia asignada en escala pentatónica armoniosa
        // Gotas con gy pequeño (fondo) tienen tono más alto/delicado; gy grande (primer plano), más profundo
        let octave = 1.0 + (1.0 - drop.gy) * 1.5; // De octava 1 a 2.5
        let step = i % 5;
        let base_freq = pent_hz(octave, step, 261.63); // Base C4/D4

        // Paneo espacial estéreo: mapear gx [0..1] a [-0.85, +0.85]
        let pan = (drop.gx * 2.0 - 1.0) * 0.85;

        // Ganancia escalada por profundidad
        let gain = 0.04 + 0.07 * drop.gy;

        // Sweep de tono hacia arriba (+0.35) para el clásico sonido "bloop" de gota de agua
        audio.add_water_drop(
            drop.t_impact,
            0.14,
            base_freq,
            0.38,
            gain,
            pan,
        );

        // Cada 3 gotas, agregar un toque de campanita/chime tenue en octava superior
        if i % 3 == 0 {
            let chime_freq = base_freq * 2.0;
            audio.add_note(
                drop.t_impact + 0.02,
                0.28,
                chime_freq,
                Waveform::Sine,
                gain * 0.4,
                pan * 0.8,
            );
        }
    }

    // Normalizar la pista de audio a –3 dBFS para un balance impecable
    audio.normalize_peak(0.707);

    (timeline, audio)
}
