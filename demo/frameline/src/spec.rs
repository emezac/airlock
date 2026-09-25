//! Storyboards described as data. A spec is YAML that a person or a model can
//! write; `Spec::parse` rejects anything outside the schema before a frame is
//! drawn, so a malformed model answer fails loudly instead of rendering noise.

use crate::audio::AudioTrack;
use crate::core::canvas::Canvas;
use crate::core::color::{Color, Palette};
use crate::core::easing::ease_in_out_cubic;
use crate::core::finishes::draw_paper;
use crate::core::schematic::{draw_vector_text, VECTOR_CHARSET};
use crate::puppet::{FruitFly, PaperBoat, PatchworkBalloon, TortoisePuppet};
use crate::timeline::{FilmTimeline, Scene};
use anyhow::{bail, Context, Result};
use serde::Deserialize;

pub const FPS: u32 = 24;
const MAX_SCENES: usize = 24;
const MAX_ELEMENTS: usize = 8;
const MAX_CAPTION_CHARS: usize = 80;
/// Puppets are drawn at their natural size on an 800 px canvas.
const REFERENCE_WIDTH: f32 = 800.0;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    pub title: String,
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_style")]
    pub style: String,
    pub scenes: Vec<SceneSpec>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneSpec {
    pub name: String,
    pub duration: f32,
    #[serde(default = "default_shot")]
    pub shot: String,
    #[serde(default)]
    pub camera: Option<CameraMove>,
    #[serde(default)]
    pub elements: Vec<Element>,
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default)]
    pub style: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraMove {
    pub from: CameraPoint,
    pub to: CameraPoint,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraPoint {
    pub x: f32,
    pub y: f32,
    #[serde(default = "one")]
    pub zoom: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Element {
    pub kind: String,
    pub x: f32,
    pub y: f32,
    #[serde(default = "one")]
    pub scale: f32,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default = "default_motion")]
    pub motion: String,
}

fn default_format() -> String { "1:1".into() }
fn default_style() -> String { "paper_ink".into() }
fn default_shot() -> String { "wide".into() }
fn default_motion() -> String { "still".into() }
fn one() -> f32 { 1.0 }

pub const FORMATS: &[&str] = &["1:1", "16:9", "9:16"];
pub const STYLES: &[&str] = &["paper_ink", "riso_pop", "screen_sea", "pencil_minimal", "blueprint_night"];
pub const SHOTS: &[&str] = &["wide", "medium", "close_up"];
pub const KINDS: &[&str] = &["boat", "fly", "balloon", "tortoise", "product", "text"];
pub const MOTIONS: &[&str] = &["still", "bob", "drift", "rise"];

impl Spec {
    pub fn parse(yaml: &str) -> Result<Self> {
        let spec: Spec = serde_yaml::from_str(yaml).context("spec is not valid YAML for the Frameline schema")?;
        spec.validate()?;
        Ok(spec)
    }

    pub fn load(path: &str) -> Result<Self> {
        Self::parse(&std::fs::read_to_string(path).with_context(|| format!("cannot read {path}"))?)
    }

    fn validate(&self) -> Result<()> {
        one_of("format", &self.format, FORMATS)?;
        one_of("style", &self.style, STYLES)?;
        if self.title.trim().is_empty() {
            bail!("title is empty");
        }
        if self.scenes.is_empty() || self.scenes.len() > MAX_SCENES {
            bail!("a spec needs 1 to {MAX_SCENES} scenes, got {}", self.scenes.len());
        }
        for (i, scene) in self.scenes.iter().enumerate() {
            let at = format!("scene {} ({})", i + 1, scene.name);
            if !(0.5..=20.0).contains(&scene.duration) {
                bail!("{at}: duration must be between 0.5 and 20 seconds");
            }
            one_of(&format!("{at}: shot"), &scene.shot, SHOTS)?;
            if let Some(style) = &scene.style {
                one_of(&format!("{at}: style"), style, STYLES)?;
            }
            if let Some(caption) = &scene.caption {
                if caption.chars().count() > MAX_CAPTION_CHARS {
                    bail!("{at}: caption longer than {MAX_CAPTION_CHARS} characters");
                }
                drawable(&format!("{at}: caption"), caption)?;
            }
            if let Some(cam) = scene.camera {
                for p in [cam.from, cam.to] {
                    unit(&format!("{at}: camera x"), p.x)?;
                    unit(&format!("{at}: camera y"), p.y)?;
                    if !(0.5..=4.0).contains(&p.zoom) {
                        bail!("{at}: camera zoom must be between 0.5 and 4");
                    }
                }
            }
            if scene.elements.len() > MAX_ELEMENTS {
                bail!("{at}: at most {MAX_ELEMENTS} elements");
            }
            for e in &scene.elements {
                one_of(&format!("{at}: element kind"), &e.kind, KINDS)?;
                one_of(&format!("{at}: element motion"), &e.motion, MOTIONS)?;
                unit(&format!("{at}: element x"), e.x)?;
                unit(&format!("{at}: element y"), e.y)?;
                if !(0.1..=4.0).contains(&e.scale) {
                    bail!("{at}: element scale must be between 0.1 and 4");
                }
                if matches!(e.kind.as_str(), "product" | "text") && e.label.as_deref().unwrap_or("").is_empty() {
                    bail!("{at}: a {} element needs a label", e.kind);
                }
                if let Some(label) = &e.label {
                    drawable(&format!("{at}: element label"), label)?;
                }
            }
        }
        Ok(())
    }

    /// Output size in pixels for a given width.
    pub fn size(&self, width: u32) -> (u32, u32) {
        let (w, h) = match self.format.as_str() {
            "16:9" => (16, 9),
            "9:16" => (9, 16),
            _ => (1, 1),
        };
        let height = (width as f32 * h as f32 / w as f32).round() as u32;
        (width, height.max(1))
    }

    pub fn timeline(&self) -> FilmTimeline {
        let mut timeline = FilmTimeline::new();
        for scene in &self.scenes {
            timeline.add_scene(SpecScene::new(scene.clone(), &self.style));
        }
        timeline
    }

    /// Frame index at the middle of each scene: the frames a storyboard shows.
    pub fn key_frames(&self) -> Vec<usize> {
        let mut start = 0usize;
        self.scenes
            .iter()
            .map(|s| {
                let len = frames(s.duration);
                let mid = start + len / 2;
                start += len;
                mid
            })
            .collect()
    }

    pub fn silent_audio(&self) -> AudioTrack {
        AudioTrack::new(self.scenes.iter().map(|s| s.duration).sum(), 48000)
    }
}

fn frames(seconds: f32) -> usize {
    ((seconds * FPS as f32).round() as usize).max(1)
}

fn one_of(what: &str, value: &str, allowed: &[&str]) -> Result<()> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        bail!("{what}: '{value}' is not one of {}", allowed.join(", "))
    }
}

fn unit(what: &str, v: f32) -> Result<()> {
    if (0.0..=1.0).contains(&v) {
        Ok(())
    } else {
        bail!("{what} must be between 0 and 1, got {v}")
    }
}

pub fn palette(name: &str) -> Palette {
    match name {
        "riso_pop" => Palette::riso_pop(),
        "screen_sea" => Palette::screen_sea(),
        "pencil_minimal" => Palette::pencil_minimal(),
        "blueprint_night" => Palette::blueprint_night(),
        _ => Palette::paper_ink(),
    }
}

/// One scene of a spec, drawn by the existing taller_film primitives.
pub struct SpecScene {
    spec: SceneSpec,
    palette: Palette,
    frames: usize,
}

impl SpecScene {
    pub fn new(spec: SceneSpec, film_style: &str) -> Self {
        let palette = palette(spec.style.as_deref().unwrap_or(film_style));
        let frames = frames(spec.duration);
        Self { spec, palette, frames }
    }

    fn shot_zoom(&self) -> f32 {
        match self.spec.shot.as_str() {
            "medium" => 1.4,
            "close_up" => 2.0,
            _ => 1.0,
        }
    }

    fn ink(&self) -> Color {
        if self.palette.name.contains("blueprint") { self.palette.chalk } else { self.palette.ink }
    }
}

impl Scene for SpecScene {
    fn name(&self) -> &str { &self.spec.name }
    fn duration_frames(&self) -> usize { self.frames }
    fn palette(&self) -> &Palette { &self.palette }

    fn render(&self, canvas: &mut Canvas, tau: f32, frame: usize) {
        let seed = frame as u32 + 1;
        let (w, h) = (canvas.width as f32, canvas.height as f32);
        draw_paper(canvas, &self.palette, seed);

        // Camera: ease between two framings, on top of the shot's base zoom.
        let t = ease_in_out_cubic(tau);
        let (cx, cy, zoom) = match self.spec.camera {
            Some(c) => (
                c.from.x + (c.to.x - c.from.x) * t,
                c.from.y + (c.to.y - c.from.y) * t,
                c.from.zoom + (c.to.zoom - c.from.zoom) * t,
            ),
            None => (0.5, 0.5, 1.0),
        };
        let zoom = zoom * self.shot_zoom();
        canvas.save();
        canvas.translate(w / 2.0, h / 2.0);
        canvas.scale(zoom, zoom);
        canvas.translate(-cx * w, -cy * h);

        let unit = w.min(h) / REFERENCE_WIDTH;
        for (i, e) in self.spec.elements.iter().enumerate() {
            let phase = tau * std::f32::consts::TAU + i as f32;
            let (dx, dy) = match e.motion.as_str() {
                "bob" => (0.0, phase.sin() * 8.0 * unit),
                "drift" => ((tau - 0.5) * 0.12 * w, 0.0),
                "rise" => (0.0, -(tau * 0.15 * h)),
                _ => (0.0, 0.0),
            };
            let (x, y, s) = (e.x * w + dx, e.y * h + dy, e.scale * unit);
            draw_element(canvas, e, x, y, s, &self.palette, self.ink(), seed + i as u32 * 7, phase);
        }
        canvas.restore();

        // Caption band in screen space, readable at any zoom.
        if let Some(caption) = &self.spec.caption {
            // The vector font advances 0.84 × size per character; shrink long
            // captions so they always fit inside the frame.
            let chars = caption.chars().count().max(1) as f32;
            let fit = w * 0.92 / (chars * 0.84);
            let size = (h * 0.045).min(w * 0.05).min(fit).max(6.0);
            canvas.fill_rect(0.0, h * 0.86, w, h * 0.14, self.palette.paper.with_alpha(0.85));
            draw_vector_text(canvas, w / 2.0, h * 0.93 - size / 2.0, caption, size, self.ink(), true);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_element(canvas: &mut Canvas, e: &Element, x: f32, y: f32, s: f32, palette: &Palette, ink: Color, seed: u32, phase: f32) {
    match e.kind.as_str() {
        "boat" => {
            let mut boat = PaperBoat::new(x, y, s * 1.6);
            boat.bob_angle = phase.sin() * 0.06;
            boat.draw(canvas, palette, seed);
        }
        "fly" => {
            let mut fly = FruitFly::new(x, y, s * 1.2);
            fly.wing_flap = phase.sin();
            fly.draw(canvas, palette, seed, false);
        }
        "balloon" => PatchworkBalloon::new(x, y, s * 0.8).draw(canvas),
        "tortoise" => {
            let mut t = TortoisePuppet::default();
            t.cx = x;
            t.cy = y;
            t.scale = s * 0.8;
            t.draw(canvas);
        }
        "product" => {
            let (bw, bh) = (220.0 * s, 140.0 * s);
            let fill = palette.accents.first().copied().unwrap_or(palette.blush);
            canvas.fill_rect(x - bw / 2.0, y - bh / 2.0, bw, bh, fill.with_alpha(0.9));
            canvas.stroke_rect(x - bw / 2.0, y - bh / 2.0, bw, bh, ink, 2.0 * s.max(0.5));
            draw_vector_text(canvas, x, y + 10.0 * s, e.label.as_deref().unwrap_or(""), 30.0 * s, ink, true);
        }
        "text" => draw_vector_text(canvas, x, y, e.label.as_deref().unwrap_or(""), 36.0 * s, ink, true),
        _ => {}
    }
}

/// Reject text the vector font would silently drop.
fn drawable(what: &str, text: &str) -> Result<()> {
    if let Some(c) = text.chars().find(|c| !VECTOR_CHARSET.contains(c.to_ascii_uppercase())) {
        bail!("{what}: character {c:?} cannot be drawn (allowed: letters, digits and {:?})", " .:-/°(),'!?");
    }
    Ok(())
}
