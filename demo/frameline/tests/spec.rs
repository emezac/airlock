//! The spec is the contract agents write against: anything outside the
//! vocabulary must fail loudly, with the scene it came from.

use taller_film::Spec;

const MINIMAL: &str = r#"
title: Test
format: "16:9"
style: paper_ink
scenes:
  - name: one
    duration: 2
    shot: wide
    elements:
      - { kind: boat, x: 0.5, y: 0.5 }
"#;

fn err(yaml: &str) -> String {
    format!("{:#}", Spec::parse(yaml).expect_err("spec should be rejected"))
}

#[test]
fn minimal_spec_is_valid() {
    let spec = Spec::parse(MINIMAL).unwrap();
    assert_eq!(spec.size(320), (320, 180));
    assert_eq!(spec.key_frames().len(), 1);
}

#[test]
fn examples_are_valid() {
    for name in ["lumen_ad", "tortoise_story"] {
        let path = format!("{}/examples/specs/{name}.yaml", env!("CARGO_MANIFEST_DIR"));
        Spec::load(&path).unwrap_or_else(|e| panic!("{name}: {e:#}"));
    }
}

#[test]
fn unknown_fields_are_rejected() {
    assert!(err(&MINIMAL.replace("shot: wide", "shot: wide\n    music: loud")).contains("music"));
}

#[test]
fn closed_vocabularies() {
    assert!(err(&MINIMAL.replace("paper_ink", "watercolor")).contains("style"));
    assert!(err(&MINIMAL.replace("kind: boat", "kind: dragon")).contains("kind"));
    assert!(err(&MINIMAL.replace("shot: wide", "shot: dutch")).contains("shot"));
    assert!(err(&MINIMAL.replace("\"16:9\"", "\"4:3\"")).contains("format"));
}

#[test]
fn ranges_are_enforced() {
    assert!(err(&MINIMAL.replace("duration: 2", "duration: 60")).contains("duration"));
    assert!(err(&MINIMAL.replace("x: 0.5", "x: 1.5")).contains("scene 1 (one)"));
}

#[test]
fn text_needs_a_drawable_label() {
    let text = MINIMAL.replace("kind: boat", "kind: text");
    assert!(err(&text).contains("label"));
    let emoji = MINIMAL.replace("shot: wide", "shot: wide\n    caption: \"hi 🙂\"");
    assert!(err(&emoji).contains("cannot be drawn"));
}
