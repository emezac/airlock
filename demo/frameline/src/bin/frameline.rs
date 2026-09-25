//! Frameline: storyboard and animatic from a YAML spec.
//!
//!   frameline check spec.yaml
//!   frameline board spec.yaml -o board.png [--tile-width 360]
//!   frameline animatic spec.yaml -o ad.mp4 [--width 720]

use anyhow::Result;
use clap::{Parser, Subcommand};
use taller_film::{export_mp4, render_board, Spec};

#[derive(Parser)]
#[command(name = "frameline", about = "Storyboards and animatics from a YAML spec")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate a spec and print a summary.
    Check { spec: String },
    /// Render the storyboard: the key frame of every scene on one image.
    Board {
        spec: String,
        #[arg(short, long, default_value = "board.png")]
        output: String,
        #[arg(long, default_value_t = 360)]
        tile_width: u32,
    },
    /// Render the animatic video (needs ffmpeg).
    Animatic {
        spec: String,
        #[arg(short, long, default_value = "animatic.mp4")]
        output: String,
        #[arg(long, default_value_t = 720)]
        width: u32,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Check { spec } => {
            let s = Spec::load(&spec)?;
            let seconds: f32 = s.scenes.iter().map(|x| x.duration).sum();
            println!("ok: '{}' · {} · {} · {} scenes · {:.1}s", s.title, s.format, s.style, s.scenes.len(), seconds);
        }
        Command::Board { spec, output, tile_width } => {
            let s = Spec::load(&spec)?;
            render_board(&s, tile_width).save(&output)?;
            println!("board: {output}");
        }
        Command::Animatic { spec, output, width } => {
            let s = Spec::load(&spec)?;
            let (w, h) = s.size(width);
            export_mp4(&s.timeline(), Some(&s.silent_audio()), None, w, h - h % 2, &output, 1)?;
        }
    }
    Ok(())
}
