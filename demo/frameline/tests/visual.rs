//! Visual regression: every example spec renders a small storyboard that must
//! match its golden PNG. Regenerate goldens on purpose with
//! `UPDATE_GOLDEN=1 cargo test --test visual`, and review the new images.

use std::path::{Path, PathBuf};
use taller_film::{changed_pixels, difference, render_board, Spec};

/// Boards are rendered small so the test is fast and the goldens stay light.
const TILE_WIDTH: u32 = 200;
/// A pixel counts as changed when a channel moves by more than this (0–255),
/// which absorbs anti-aliasing noise across CPUs.
const TOLERANCE: u8 = 24;
/// Changed pixels allowed before a board fails. Rendering is deterministic, so
/// the budget is tiny: a one-letter caption edit (! for .) changes ~10 pixels.
const MAX_CHANGED: u64 = 2;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn specs() -> Vec<PathBuf> {
    let mut specs: Vec<PathBuf> = std::fs::read_dir(root().join("examples/specs"))
        .expect("examples/specs exists")
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "yaml"))
        .collect();
    specs.sort();
    specs
}

fn golden_for(spec: &Path) -> PathBuf {
    let name = spec.file_stem().unwrap().to_string_lossy();
    root().join("tests/golden").join(format!("{name}.png"))
}

#[test]
fn boards_match_goldens() {
    let update = std::env::var("UPDATE_GOLDEN").is_ok_and(|v| v == "1");
    let mut failures = Vec::new();
    for path in specs() {
        let spec = Spec::load(path.to_str().unwrap()).expect("example spec is valid");
        let board = render_board(&spec, TILE_WIDTH);
        let golden = golden_for(&path);
        if update || !golden.exists() {
            board.save(&golden).expect("write golden");
            if !update {
                failures.push(format!("{}: no golden, wrote one; review and commit it", golden.display()));
            }
            continue;
        }
        let expected = image::open(&golden).expect("read golden").to_rgba8();
        let changed = changed_pixels(&board, &expected, TOLERANCE);
        if changed > MAX_CHANGED {
            let actual = golden.with_extension("actual.png");
            board.save(&actual).expect("write actual");
            failures.push(format!(
                "{}: {changed} pixels changed (mean difference {:.5}); see {}",
                path.display(),
                difference(&board, &expected),
                actual.display()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn rendering_is_deterministic() {
    let spec = Spec::load(specs()[0].to_str().unwrap()).unwrap();
    let a = render_board(&spec, 120);
    let b = render_board(&spec, 120);
    assert_eq!(difference(&a, &b), 0.0);
}

#[test]
fn difference_detects_a_changed_scene() {
    let path = root().join("examples/specs/tortoise_story.yaml");
    let yaml = std::fs::read_to_string(path).unwrap();
    let original = Spec::parse(&yaml).unwrap();
    let moved = Spec::parse(&yaml.replacen("x: 0.5", "x: 0.2", 1)).unwrap();
    let n = changed_pixels(&render_board(&original, TILE_WIDTH), &render_board(&moved, TILE_WIDTH), TOLERANCE);
    assert!(n > MAX_CHANGED, "moving an element changed only {n} pixels");
}

#[test]
fn difference_detects_a_caption_edit() {
    let path = root().join("examples/specs/lumen_ad.yaml");
    let yaml = std::fs::read_to_string(path).unwrap();
    let original = Spec::parse(&yaml).unwrap();
    let edited = Spec::parse(&yaml.replacen("Meet Lumen.", "Meet Lumen!", 1)).unwrap();
    let n = changed_pixels(&render_board(&original, TILE_WIDTH), &render_board(&edited, TILE_WIDTH), TOLERANCE);
    assert!(n > MAX_CHANGED, "a caption edit changed only {n} pixels");
}
