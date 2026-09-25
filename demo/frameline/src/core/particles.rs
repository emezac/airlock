//! Sistema de partículas procedural con pool fijo, física integrada y render directo al Canvas.
//!
//! Diseñado para efectos de: polvo dorado, mariposas, flores cayendo, chispas,
//! lluvia de pétalos, esporas de Macondo, fuegos fatuos y humo.
//!
//! Uso básico:
//! ```ignore
//! let mut pool = ParticlePool::new();
//! // En cada fotograma:
//! pool.emit(EmitParams { cx: 540.0, cy: 960.0, count: 4, ..Default::default() });
//! pool.update(dt);
//! pool.draw(&mut canvas);
//! ```

use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::rng::Rng;
use tiny_skia::{BlendMode, PathBuilder};
use std::f32::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Constantes del pool
// ─────────────────────────────────────────────────────────────────────────────

/// Capacidad máxima de partículas simultáneas.
/// 512 cubre casi todos los casos de uso sin impactar el presupuesto de frame.
const MAX_PARTICLES: usize = 512;

// ─────────────────────────────────────────────────────────────────────────────
// Tipos de renderizado
// ─────────────────────────────────────────────────────────────────────────────

/// Forma visual de cada partícula
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParticleShape {
    /// Círculo simple
    Circle,
    /// Cuadrado / confeti
    Square,
    /// Estrella de 4 puntas
    Star,
    /// Línea corta orientada según velocidad
    Streak,
    /// Curva de mariposa (dos lóbulos)
    Butterfly,
    /// Pétalo de flor
    Petal,
    /// Triángulo
    Triangle,
}

// ─────────────────────────────────────────────────────────────────────────────
// Partícula individual
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub ax: f32,    // aceleración x (gravedad horizontal, viento)
    pub ay: f32,    // aceleración y (gravedad)
    pub age: f32,   // tiempo vivido [0, life]
    pub life: f32,  // tiempo de vida total
    pub size: f32,
    pub size_end: f32, // tamaño al morir (para shrink)
    pub angle: f32, // rotación actual (radianes)
    pub angular_vel: f32, // velocidad angular
    pub color_start: Color,
    pub color_end: Color,
    pub shape: ParticleShape,
    pub blend: BlendMode,
    pub active: bool,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            age: 0.0,
            life: 1.0,
            size: 4.0,
            size_end: 0.0,
            angle: 0.0,
            angular_vel: 0.0,
            color_start: Color::rgba(1.0, 1.0, 1.0, 1.0),
            color_end: Color::rgba(1.0, 1.0, 1.0, 0.0),
            shape: ParticleShape::Circle,
            blend: BlendMode::SourceOver,
            active: false,
        }
    }
}

impl Particle {
    /// Progreso normalizado de la vida: 0.0 = recién nacida, 1.0 = muerta
    #[inline]
    pub fn tau(&self) -> f32 {
        if self.life <= 0.0 { 1.0 } else { (self.age / self.life).clamp(0.0, 1.0) }
    }

    /// Color interpolado según progreso de vida
    pub fn current_color(&self) -> Color {
        let t = self.tau();
        // Curva de fade: rápido al final
        let fade = 1.0 - t * t;
        self.color_start.mix(self.color_end, t).with_alpha(
            (self.color_start.a + (self.color_end.a - self.color_start.a) * t) * fade
        )
    }

    /// Tamaño interpolado
    #[inline]
    pub fn current_size(&self) -> f32 {
        let t = self.tau();
        self.size + (self.size_end - self.size) * t
    }

    /// Actualiza física en `dt` segundos
    pub fn update(&mut self, dt: f32) {
        if !self.active { return; }

        // Física de Euler semi-implícita
        self.vx += self.ax * dt;
        self.vy += self.ay * dt;

        // Amortiguación del aire (drag)
        let drag = 0.985;
        self.vx *= drag;
        self.vy *= drag;

        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.angle += self.angular_vel * dt;
        self.age += dt;

        if self.age >= self.life {
            self.active = false;
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pool de partículas
// ─────────────────────────────────────────────────────────────────────────────

pub struct ParticlePool {
    particles: Vec<Particle>,
    rng: Rng,
    /// Tiempo total acumulado (útil para emisión basada en tiempo)
    pub time: f32,
}

impl ParticlePool {
    pub fn new() -> Self {
        Self {
            particles: vec![Particle::default(); MAX_PARTICLES],
            rng: Rng::new(12345),
            time: 0.0,
        }
    }

    pub fn with_seed(seed: u32) -> Self {
        Self {
            particles: vec![Particle::default(); MAX_PARTICLES],
            rng: Rng::new(seed),
            time: 0.0,
        }
    }

    /// Número de partículas activas actualmente
    pub fn active_count(&self) -> usize {
        self.particles.iter().filter(|p| p.active).count()
    }

    /// Emite nuevas partículas según los parámetros dados
    pub fn emit(&mut self, params: &EmitParams) {
        let mut emitted = 0;
        for p in self.particles.iter_mut() {
            if emitted >= params.count { break; }
            if p.active { continue; }

            // Posición inicial con spread circular o rectangular
            let (off_x, off_y) = match params.spread {
                SpreadShape::Disc { radius } => {
                    let r = self.rng.next_f32().sqrt() * radius;
                    let a = self.rng.next_f32() * PI * 2.0;
                    (r * a.cos(), r * a.sin())
                }
                SpreadShape::Rect { w, h } => {
                    ((self.rng.next_f32() - 0.5) * w, (self.rng.next_f32() - 0.5) * h)
                }
                SpreadShape::Line { dx, dy } => {
                    let t = self.rng.next_f32();
                    (t * dx, t * dy)
                }
            };

            // Velocidad inicial en cono de ángulo ±spread_angle alrededor de la dirección base
            let base_angle = params.angle_deg.to_radians();
            let spread = params.speed_spread_deg.to_radians();
            let emit_angle = base_angle + (self.rng.next_f32() - 0.5) * spread;
            let speed = params.speed_min + self.rng.next_f32() * (params.speed_max - params.speed_min);

            let life = params.life_min + self.rng.next_f32() * (params.life_max - params.life_min);
            let size = params.size_min + self.rng.next_f32() * (params.size_max - params.size_min);

            // Color con jitter
            let color_start = jitter_color(params.color_start, params.color_jitter, &mut self.rng);
            let color_end = params.color_end;

            *p = Particle {
                x: params.cx + off_x,
                y: params.cy + off_y,
                vx: emit_angle.cos() * speed,
                vy: emit_angle.sin() * speed,
                ax: params.gravity_x,
                ay: params.gravity_y,
                age: 0.0,
                life,
                size,
                size_end: params.size_end,
                angle: self.rng.next_f32() * PI * 2.0,
                angular_vel: (self.rng.next_f32() - 0.5) * params.max_angular_vel,
                color_start,
                color_end,
                shape: params.shape,
                blend: params.blend,
                active: true,
            };
            emitted += 1;
        }
    }

    /// Avanza la física de todas las partículas activas
    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        for p in self.particles.iter_mut() {
            if p.active {
                p.update(dt);
            }
        }
    }

    /// Dibuja todas las partículas activas sobre el canvas
    pub fn draw(&self, canvas: &mut Canvas) {
        for p in self.particles.iter().filter(|p| p.active) {
            let color = p.current_color();
            if color.a < 0.005 { continue; }

            let size = p.current_size();
            if size < 0.3 { continue; }

            canvas.save();
            canvas.set_blend_mode(p.blend);
            canvas.translate(p.x, p.y);
            canvas.rotate(p.angle);

            draw_particle_shape(canvas, size, color, p.shape, p.vx, p.vy);

            canvas.restore();
        }
    }

    /// Elimina todas las partículas activas (reset de escena)
    pub fn clear(&mut self) {
        for p in self.particles.iter_mut() {
            p.active = false;
        }
        self.time = 0.0;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Parámetros de emisión
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub enum SpreadShape {
    Disc { radius: f32 },
    Rect { w: f32, h: f32 },
    Line { dx: f32, dy: f32 },
}

/// Parámetros completos para emitir un lote de partículas
#[derive(Clone, Copy, Debug)]
pub struct EmitParams {
    /// Centro de emisión X
    pub cx: f32,
    /// Centro de emisión Y
    pub cy: f32,
    /// Número de partículas a emitir en esta llamada
    pub count: usize,
    /// Forma del área de spawn
    pub spread: SpreadShape,
    /// Ángulo base de salida en grados (0=derecha, 270=arriba)
    pub angle_deg: f32,
    /// Abanico de variación del ángulo en grados (±)
    pub speed_spread_deg: f32,
    /// Velocidad mínima (px/s)
    pub speed_min: f32,
    /// Velocidad máxima (px/s)
    pub speed_max: f32,
    /// Tiempo de vida mínimo (segundos)
    pub life_min: f32,
    /// Tiempo de vida máximo (segundos)
    pub life_max: f32,
    /// Tamaño inicial mínimo (px)
    pub size_min: f32,
    /// Tamaño inicial máximo (px)
    pub size_max: f32,
    /// Tamaño al morir (permite shrink o grow)
    pub size_end: f32,
    /// Color inicial
    pub color_start: Color,
    /// Color al morir (interpolado)
    pub color_end: Color,
    /// Jitter de color (0.0 = exacto, 0.2 = ±20% por canal)
    pub color_jitter: f32,
    /// Gravedad horizontal (px/s²)
    pub gravity_x: f32,
    /// Gravedad vertical (px/s² — positivo = hacia abajo)
    pub gravity_y: f32,
    /// Velocidad angular máxima (rad/s)
    pub max_angular_vel: f32,
    /// Forma visual
    pub shape: ParticleShape,
    /// Modo de blend
    pub blend: BlendMode,
}

impl Default for EmitParams {
    fn default() -> Self {
        Self {
            cx: 0.0,
            cy: 0.0,
            count: 8,
            spread: SpreadShape::Disc { radius: 10.0 },
            angle_deg: 270.0, // hacia arriba
            speed_spread_deg: 90.0,
            speed_min: 80.0,
            speed_max: 200.0,
            life_min: 0.8,
            life_max: 2.0,
            size_min: 3.0,
            size_max: 8.0,
            size_end: 0.0,
            color_start: Color::rgba(1.0, 0.9, 0.3, 1.0),
            color_end: Color::rgba(1.0, 0.5, 0.1, 0.0),
            color_jitter: 0.15,
            gravity_x: 0.0,
            gravity_y: 60.0,
            max_angular_vel: 4.0,
            shape: ParticleShape::Circle,
            blend: BlendMode::SourceOver,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Presets de emisión comunes
// ─────────────────────────────────────────────────────────────────────────────

impl EmitParams {
    /// Esporas doradas de Macondo: pequeños puntos luminosos que suben
    pub fn macondo_spores(cx: f32, cy: f32) -> Self {
        Self {
            cx, cy,
            count: 3,
            spread: SpreadShape::Disc { radius: 20.0 },
            angle_deg: 280.0,
            speed_spread_deg: 50.0,
            speed_min: 30.0,
            speed_max: 90.0,
            life_min: 1.5,
            life_max: 3.5,
            size_min: 1.5,
            size_max: 4.0,
            size_end: 0.0,
            color_start: Color::hex("#f7e060"),
            color_end: Color::hex("#f0a010"),
            color_jitter: 0.2,
            gravity_x: 0.0,
            gravity_y: -20.0, // flotan hacia arriba
            max_angular_vel: 1.0,
            shape: ParticleShape::Circle,
            blend: BlendMode::Screen,
        }
    }

    /// Mariposas amarillas de Mauricio Babilonia
    pub fn yellow_butterflies(cx: f32, cy: f32) -> Self {
        Self {
            cx, cy,
            count: 2,
            spread: SpreadShape::Disc { radius: 60.0 },
            angle_deg: 260.0,
            speed_spread_deg: 140.0,
            speed_min: 50.0,
            speed_max: 160.0,
            life_min: 2.5,
            life_max: 5.0,
            size_min: 6.0,
            size_max: 14.0,
            size_end: 0.0,
            color_start: Color::hex("#f5d800"),
            color_end: Color::hex("#e8a000"),
            color_jitter: 0.15,
            gravity_x: 0.0,
            gravity_y: 0.0, // flotan libremente
            max_angular_vel: 2.5,
            shape: ParticleShape::Butterfly,
            blend: BlendMode::Screen,
        }
    }

    /// Pétalos de flores cayendo
    pub fn falling_petals(cx: f32, cy: f32) -> Self {
        Self {
            cx, cy,
            count: 2,
            spread: SpreadShape::Rect { w: 300.0, h: 10.0 },
            angle_deg: 80.0, // diagonal hacia abajo
            speed_spread_deg: 40.0,
            speed_min: 30.0,
            speed_max: 80.0,
            life_min: 2.0,
            life_max: 4.5,
            size_min: 4.0,
            size_max: 10.0,
            size_end: 2.0,
            color_start: Color::hex("#f9a8c0"),
            color_end: Color::hex("#f76090"),
            color_jitter: 0.2,
            gravity_x: 8.0,  // leve brisa lateral
            gravity_y: 40.0,
            max_angular_vel: 3.5,
            shape: ParticleShape::Petal,
            blend: BlendMode::SourceOver,
        }
    }

    /// Chispas de fuego / confeti explosivo
    pub fn sparks(cx: f32, cy: f32) -> Self {
        Self {
            cx, cy,
            count: 12,
            spread: SpreadShape::Disc { radius: 5.0 },
            angle_deg: 270.0,
            speed_spread_deg: 360.0,
            speed_min: 150.0,
            speed_max: 400.0,
            life_min: 0.4,
            life_max: 1.2,
            size_min: 2.0,
            size_max: 5.0,
            size_end: 0.0,
            color_start: Color::hex("#fff0a0"),
            color_end: Color::hex("#ff4000"),
            color_jitter: 0.1,
            gravity_x: 0.0,
            gravity_y: 200.0,
            max_angular_vel: 0.0,
            shape: ParticleShape::Streak,
            blend: BlendMode::Screen,
        }
    }

    /// Polvo / niebla etérea de fondo
    pub fn dust_motes(cx: f32, cy: f32) -> Self {
        Self {
            cx, cy,
            count: 1,
            spread: SpreadShape::Rect { w: 400.0, h: 400.0 },
            angle_deg: 270.0,
            speed_spread_deg: 360.0,
            speed_min: 5.0,
            speed_max: 25.0,
            life_min: 3.0,
            life_max: 6.0,
            size_min: 2.0,
            size_max: 8.0,
            size_end: 0.0,
            color_start: Color::rgba(1.0, 1.0, 0.8, 0.6),
            color_end: Color::rgba(1.0, 1.0, 0.8, 0.0),
            color_jitter: 0.1,
            gravity_x: 0.0,
            gravity_y: -10.0,
            max_angular_vel: 0.5,
            shape: ParticleShape::Circle,
            blend: BlendMode::Screen,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Renderizado de formas individuales
// ─────────────────────────────────────────────────────────────────────────────

fn draw_particle_shape(canvas: &mut Canvas, size: f32, color: Color, shape: ParticleShape, vx: f32, vy: f32) {
    match shape {
        ParticleShape::Circle => {
            canvas.fill_circle(0.0, 0.0, size, color);
        }

        ParticleShape::Square => {
            canvas.fill_rect(-size, -size, size * 2.0, size * 2.0, color);
        }

        ParticleShape::Triangle => {
            let mut pb = PathBuilder::new();
            pb.move_to(0.0, -size);
            pb.line_to(size * 0.866, size * 0.5);
            pb.line_to(-size * 0.866, size * 0.5);
            pb.close();
            if let Some(path) = pb.finish() {
                canvas.fill_path(&path, color);
            }
        }

        ParticleShape::Star => {
            let mut pb = PathBuilder::new();
            let outer = size;
            let inner = size * 0.4;
            for i in 0..8 {
                let a = i as f32 * PI / 4.0;
                let r = if i % 2 == 0 { outer } else { inner };
                let x = a.cos() * r;
                let y = a.sin() * r;
                if i == 0 { pb.move_to(x, y); } else { pb.line_to(x, y); }
            }
            pb.close();
            if let Some(path) = pb.finish() {
                canvas.fill_path(&path, color);
            }
        }

        ParticleShape::Streak => {
            // Línea orientada según velocidad
            let speed = (vx * vx + vy * vy).sqrt().max(0.001);
            let nx = vx / speed;
            let ny = vy / speed;
            let len = size * 2.5;
            canvas.stroke_line(-nx * len, -ny * len, nx * len * 0.3, ny * len * 0.3, color, size * 0.5);
        }

        ParticleShape::Butterfly => {
            // Dos elipses simétricas (alas)
            let wing_w = size * 1.4;
            let wing_h = size * 0.7;
            // Ala izquierda
            let steps = 12;
            let pts_l: Vec<[f32; 2]> = (0..=steps)
                .map(|i| {
                    let a = i as f32 / steps as f32 * PI * 2.0;
                    [-wing_w * 0.5 + wing_w * 0.5 * a.cos(), wing_h * a.sin()]
                })
                .collect();
            let mut pb = PathBuilder::new();
            for (i, p) in pts_l.iter().enumerate() {
                if i == 0 { pb.move_to(p[0], p[1]); } else { pb.line_to(p[0], p[1]); }
            }
            pb.close();
            if let Some(path) = pb.finish() {
                canvas.fill_path(&path, color);
            }
            // Ala derecha (espejo)
            let pts_r: Vec<[f32; 2]> = pts_l.iter().map(|p| [-p[0], p[1]]).collect();
            let mut pb2 = PathBuilder::new();
            for (i, p) in pts_r.iter().enumerate() {
                if i == 0 { pb2.move_to(p[0], p[1]); } else { pb2.line_to(p[0], p[1]); }
            }
            pb2.close();
            if let Some(path) = pb2.finish() {
                canvas.fill_path(&path, color.with_alpha(color.a * 0.85));
            }
        }

        ParticleShape::Petal => {
            // Elipse elongada con punta
            let steps = 10;
            let mut pb = PathBuilder::new();
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let a = t * PI;
                let bx = size * a.sin() * (1.0 - t * 0.3);
                let by = -size * 1.8 * t + size * 0.9;
                if i == 0 { pb.move_to(bx, by); } else { pb.line_to(bx, by); }
            }
            for i in (0..=steps).rev() {
                let t = i as f32 / steps as f32;
                let a = t * PI;
                let bx = -size * a.sin() * (1.0 - t * 0.3);
                let by = -size * 1.8 * t + size * 0.9;
                pb.line_to(bx, by);
            }
            pb.close();
            if let Some(path) = pb.finish() {
                canvas.fill_path(&path, color);
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Utilidad interna
// ─────────────────────────────────────────────────────────────────────────────

fn jitter_color(base: Color, jitter: f32, rng: &mut Rng) -> Color {
    if jitter < 0.001 { return base; }
    Color::rgba(
        (base.r + (rng.next_f32() - 0.5) * jitter).clamp(0.0, 1.0),
        (base.g + (rng.next_f32() - 0.5) * jitter).clamp(0.0, 1.0),
        (base.b + (rng.next_f32() - 0.5) * jitter).clamp(0.0, 1.0),
        base.a,
    )
}
