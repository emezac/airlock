use crate::core::canvas::Canvas;
use crate::core::color::Palette;

/// Rasgo que implementa cualquier escena de la película
pub trait Scene: Send + Sync {
    fn name(&self) -> &str;
    fn duration_frames(&self) -> usize;
    fn palette(&self) -> &Palette;
    /// `tau`: progreso normalizado de la escena [0.0, 1.0]
    /// `frame`: fotograma actual de la animación
    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize);
}

/// Estructura que orquesta la línea de tiempo completa de un film
pub struct FilmTimeline {
    pub scenes: Vec<Box<dyn Scene>>,
    pub fps: u32,
    pub on_twos: bool, // Por defecto true (12 fps animados duplicados a 24 fps)
    pub motion_blur: f32, // Factor de acumulación de desenfoque de movimiento (0.0 a 0.4)
}

impl FilmTimeline {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            fps: 24,
            on_twos: true,
            motion_blur: 0.0,
        }
    }

    pub fn add_scene<S: Scene + 'static>(&mut self, scene: S) {
        self.scenes.push(Box::new(scene));
    }

    /// Retorna el índice de la escena activa en un fotograma dado
    pub fn scene_index_at_frame(&self, global_frame: usize) -> usize {
        let mut accumulated = 0;
        for (idx, scene) in self.scenes.iter().enumerate() {
            let dur = scene.duration_frames();
            if global_frame < accumulated + dur {
                return idx;
            }
            accumulated += dur;
        }
        self.scenes.len().saturating_sub(1)
    }

    pub fn total_frames(&self) -> usize {
        self.scenes.iter().map(|s| s.duration_frames()).sum()
    }

    pub fn total_duration_secs(&self) -> f32 {
        self.total_frames() as f32 / self.fps as f32
    }

    /// Renderiza un fotograma global de la película
    pub fn render_frame(&self, canvas: &mut Canvas, global_frame: usize) {
        let frame_to_draw = if self.on_twos {
            (global_frame / 2) * 2
        } else {
            global_frame
        };

        let mut accumulated = 0;
        for scene in &self.scenes {
            let dur = scene.duration_frames();
            if frame_to_draw < accumulated + dur {
                let local_frame = frame_to_draw - accumulated;
                let tau = if dur > 1 {
                    local_frame as f32 / (dur - 1) as f32
                } else {
                    0.0
                };
                scene.render(canvas, tau.clamp(0.0, 1.0), frame_to_draw);
                return;
            }
            accumulated += dur;
        }

        // Si excede, renderizar el último estado de la última escena
        if let Some(last) = self.scenes.last() {
            last.render(canvas, 1.0, frame_to_draw);
        }
    }
}
