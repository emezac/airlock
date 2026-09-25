pub mod audio;
pub mod board;
pub mod spec;
pub mod core;
pub mod films;
pub mod puppet;
pub mod render;
pub mod timeline;

pub use audio::{pent_hz, AudioTrack, Waveform};
pub use core::{
    draw_construction, draw_crayon, draw_dot_screen, draw_grain, draw_hatch, draw_hex_lattice,
    draw_paper, draw_scribble, draw_surface, ellipse_points, round_rect_points, wob_path, Canvas,
    Color, DotScreenOptions, FinishKind, HatchOptions, Palette, Rng,
};
pub use films::{
    create_demo_quality_film, create_fly_film, create_four_looks_film, create_macondo_film,
    create_monarch_film, create_okazz_film, create_ramen_film, create_tortoise_film,
    create_xray_film,
};
pub use puppet::{
    AutumnTree, ClaudeSpark, CutoutPhoto, FruitFly, GooseFlock, GoosePuppet, PaperBoat,
    PatchworkBalloon, TortoisePuppet,
};
pub use render::{export_mp4, generate_contact_sheet};
pub use timeline::{Camera, FilmTimeline, Scene};

pub use board::{changed_pixels, difference, render_board};
pub use spec::Spec;
