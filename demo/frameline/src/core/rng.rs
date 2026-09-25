/// Generador pseudoaleatorio determinista LCG lineal idéntico al de `core.js`.
/// Garantiza que texturas, tramas y granos no parpadeen entre fotogramas.
#[derive(Clone, Copy, Debug)]
pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    /// Retorna un entero de 32 bits pseudoaleatorio
    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    /// Retorna un flotante determinista en el rango [0.0, 1.0)
    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / 4294967296.0
    }

    /// Retorna un flotante en el rango [min, max)
    pub fn next_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
    }

    /// Retorna un booleano con probabilidad `p`
    pub fn next_bool(&mut self, p: f32) -> bool {
        self.next_f32() < p
    }

    /// Selecciona un elemento aleatorio de una lista de slices
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let idx = (self.next_f32() * slice.len() as f32).floor() as usize;
            slice.get(idx.min(slice.len() - 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_determinism() {
        let mut r1 = Rng::new(42);
        let mut r2 = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(r1.next_f32(), r2.next_f32());
        }
    }
}
