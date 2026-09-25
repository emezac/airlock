//! Curvas de animación (easing) para movimientos orgánicos y temporización expresiva.
//!
//! Todos los valores `tau` están normalizados en [0.0, 1.0].
//! Las funciones retornan valores en [0.0, 1.0] excepto las elásticas que pueden
//! superar esos límites brevemente (overshoot intencional).

use std::f32::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// 1. Curvas clásicas de Bézier
// ─────────────────────────────────────────────────────────────────────────────

/// Interpolación lineal sin curva (referencia base)
#[inline]
pub fn linear(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

/// Ease-in cuadrático: arranca lento, termina rápido
#[inline]
pub fn ease_in_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t
}

/// Ease-out cuadrático: arranca rápido, frena suave
#[inline]
pub fn ease_out_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * (2.0 - t)
}

/// Ease-in-out cúbico: la curva de animación de referencia
/// Úsala para movimientos de cámara, posición de personajes, fade de opacidad
#[inline]
pub fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let f = 2.0 * t - 2.0;
        0.5 * f * f * f + 1.0
    }
}

/// Ease-in-out quíntico: arranque y frenado más dramáticos (cámara épica)
#[inline]
pub fn ease_in_out_quint(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        let f = 2.0 * t - 2.0;
        0.5 * f * f * f * f * f + 1.0
    }
}

/// Ease-out cúbico: movimiento natural de objetos que se desaceleran
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let f = t - 1.0;
    f * f * f + 1.0
}

/// Ease-in cúbico: arranque progresivo, como empujar algo pesado
#[inline]
pub fn ease_in_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. Curvas con overshoot y rebote
// ─────────────────────────────────────────────────────────────────────────────

/// Back ease-out: el objeto sobrepasa el destino y regresa (overshooting)
/// Excelente para mariposas que aterrizan, personajes que se paran abruptamente
#[inline]
pub fn ease_out_back(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let c1 = 1.70158_f32;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

/// Ease-in-out con overshoot: subida con impulso, caída con rebote
#[inline]
pub fn ease_in_out_back(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let c1 = 1.70158_f32;
    let c2 = c1 * 1.525;
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
    }
}

/// Bounce ease-out: como una pelota rebotando (gravedad simulada)
pub fn ease_out_bounce(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;
    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. Curvas elásticas y de resorte
// ─────────────────────────────────────────────────────────────────────────────

/// Ease-out elástico: oscilación suave al llegar (como un resorte)
/// Perfecto para antenas de insectos, alas que se posan
pub fn ease_out_elastic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 || t == 1.0 {
        return t;
    }
    let c4 = (2.0 * PI) / 3.0;
    2.0f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
}

/// Ease-in-out elástico: tensión y liberación dramáticas
pub fn ease_in_out_elastic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 || t == 1.0 {
        return t;
    }
    let c5 = (2.0 * PI) / 4.5;
    if t < 0.5 {
        -(2.0f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
    } else {
        (2.0f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
    }
}

/// Simulación de resorte amortiguado con masa y rigidez.
/// Úsala para movimientos de cámara o personajes que necesitan sensación física real.
///
/// * `t`          – tiempo normalizado [0, 1]
/// * `mass`       – masa (0.5 = liviano, 2.0 = pesado)
/// * `stiffness`  – rigidez del resorte (80.0 = suave, 200.0 = rígido)
/// * `damping`    – amortiguación (10.0 = oscilante, 26.0 = crítico)
pub fn spring(t: f32, mass: f32, stiffness: f32, damping: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t <= 0.0 { return 0.0; }
    if t >= 1.0 { return 1.0; }

    let omega0 = (stiffness / mass).sqrt();
    let zeta = damping / (2.0 * (mass * stiffness).sqrt());

    if zeta < 1.0 {
        // Sub-crítico: oscilación con decaimiento exponencial
        let omega_d = omega0 * (1.0 - zeta * zeta).sqrt();
        let env = (-zeta * omega0 * t).exp();
        1.0 - env * (
            (omega_d * t).cos()
            + (zeta * omega0 / omega_d) * (omega_d * t).sin()
        )
    } else {
        // Sobre-crítico: sin oscilación
        let r1 = -omega0 * (zeta - (zeta * zeta - 1.0).sqrt());
        let r2 = -omega0 * (zeta + (zeta * zeta - 1.0).sqrt());
        let c2 = r1 / (r1 - r2);
        let c1 = 1.0 - c2;
        1.0 - (c1 * (r1 * t).exp() + c2 * (r2 * t).exp())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. Curvas escalonadas y pulsantes
// ─────────────────────────────────────────────────────────────────────────────

/// Animación escalonada: discretiza `t` en N escalones iguales.
/// Úsala para parpadeos de texto, frames de animación "on-twos" intencionales
#[inline]
pub fn stepped(t: f32, steps: u32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let steps = steps.max(1) as f32;
    (t * steps).floor() / steps
}

/// Pulso sinusoidal normalizado: oscila entre 0 y 1 a frecuencia `freq` por unidad de t
#[inline]
pub fn pulse(t: f32, freq: f32) -> f32 {
    (t * freq * PI * 2.0).sin() * 0.5 + 0.5
}

/// Pulso de latido: pico rápido, caída suave — imita un corazón o ala batiendo
#[inline]
pub fn heartbeat(t: f32, bpm: f32) -> f32 {
    let phase = (t * bpm / 60.0).fract();
    // Doble pico (sístole + diástole) en una sola frecuencia
    let a = (-((phase * 8.0 - 1.0).powi(2))).exp();
    let b = (-((phase * 8.0 - 2.5).powi(2) * 0.5)).exp() * 0.7;
    (a + b).clamp(0.0, 1.0)
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. Remapeo de rangos (utilidades)
// ─────────────────────────────────────────────────────────────────────────────

/// Remapea `t` de [in_min, in_max] a [out_min, out_max] con clamp.
/// Útil para extraer sub-animaciones de una ventana temporal.
#[inline]
pub fn remap(t: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    if (in_max - in_min).abs() < f32::EPSILON { return out_min; }
    let normalized = ((t - in_min) / (in_max - in_min)).clamp(0.0, 1.0);
    out_min + normalized * (out_max - out_min)
}

/// Extrae una ventana temporal de `tau` global: retorna [0,1] dentro de [start, end]
/// Ideal para secuenciar sub-animaciones dentro de una escena.
///
/// ```ignore
/// // Animar el primer 40% de la escena
/// let local_t = window(tau, 0.0, 0.4);
/// // Animar el 50%-100% de la escena
/// let local_t2 = window(tau, 0.5, 1.0);
/// ```
#[inline]
pub fn window(tau: f32, start: f32, end: f32) -> f32 {
    remap(tau, start, end, 0.0, 1.0)
}

/// Interpola suavemente entre `a` y `b` usando `t` con curva ease-in-out cúbica
#[inline]
pub fn lerp_smooth(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * ease_in_out_cubic(t)
}

/// Interpolación lineal básica
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Smoothstep de GLSL: clásico para transiciones de shader
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smootherstep (Ken Perlin): transición aún más suave que smoothstep
#[inline]
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
