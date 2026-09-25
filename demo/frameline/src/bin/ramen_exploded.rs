//! Script ejecutable independiente para renderizar el film "Ramen Exploded Diagram"
//! Inspirado en Arrow 2 / @99cccc:
//! https://x.com/99cccc/status/2101618191583895693
//!
//! Uso:
//!   cargo run --release --bin ramen_exploded -- [OPCIONES]
//!
//! Ejemplos:
//!   cargo run --release --bin ramen_exploded -- -o out/ramen_film.mp4
//!   cargo run --release --bin ramen_exploded -- --grid 24 -o out/ramen_contact.jpg

use clap::Parser;
use std::fs::create_dir_all;
use std::path::Path;
use taller_film::{create_ramen_film, export_mp4, generate_contact_sheet};

#[derive(Parser, Debug)]
#[command(name = "ramen_exploded")]
#[command(author = "Antigravity & User")]
#[command(version = "0.1.0")]
#[command(about = "Renderiza el diagrama técnico vectorial animado del tazón de ramen explosionado (Arrow 2)", long_about = None)]
struct Args {
    /// Ruta de salida para el video MP4 o imagen de contacto
    #[arg(short, long, default_value = "out/ramen_film.mp4")]
    output: String,

    /// Generar hoja de contacto (contact sheet) con N fotogramas (ej: 24) en lugar de MP4
    #[arg(short, long)]
    grid: Option<usize>,

    /// Ancho en píxeles del renderizado (por defecto 720, relación 4:5)
    #[arg(long, default_value_t = 720)]
    width: u32,

    /// Alto en píxeles del renderizado (por defecto 900, relación 4:5)
    #[arg(long, default_value_t = 900)]
    height: u32,

    /// Deshabilitar el audio
    #[arg(long, default_value_t = false)]
    no_audio: bool,

    /// Ruta a archivo de audio personalizado (por defecto auto-detecta assets/audio/ramen_arrow2.aac)
    #[arg(long)]
    audio_file: Option<String>,

    /// Factor de supersampling (SSAA) para nitidez extrema (1 = 1x, 2 = 2x SSAA Lanczos3)
    #[arg(long, default_value_t = 2)]
    ss: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("🍜 ========================================================");
    println!("   RAMEN EXPLODED DIAGRAM — Technical Vector Film");
    println!("   Inspirado en Arrow 2 / temma (@99cccc)");
    println!("============================================================");

    // Crear directorio de salida si no existe
    if let Some(parent) = Path::new(&args.output).parent() {
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent)?;
        }
    }

    // Cargar la película del ramen
    let (timeline, audio) = create_ramen_film();

    let width = args.width;
    let height = args.height;

    println!(
        "📐 Formato: {}x{} (4:5) | Duración: {:.2}s ({} fotogramas a {} fps)",
        width,
        height,
        timeline.total_duration_secs(),
        timeline.total_frames(),
        timeline.fps
    );

    // Si se solicitó hoja de contacto (--grid N)
    if let Some(tile_count) = args.grid {
        let contact_path = if args.output.ends_with(".mp4") {
            args.output.replace(".mp4", "-contact.jpg")
        } else {
            args.output.clone()
        };

        let cols = if tile_count == 16 || tile_count == 24 { 4 } else { 6 };
        println!(
            "🖼️ Generando hoja de contacto de {} fotogramas (rejilla {}x{}, SSAA {}x)...",
            tile_count,
            cols,
            (tile_count + cols - 1) / cols,
            args.ss
        );
        generate_contact_sheet(&timeline, width, height, tile_count, cols, &contact_path, args.ss)?;
        println!("✅ Hoja de contacto guardada en: {}", contact_path);
        return Ok(());
    }

    // Resolver archivo de audio
    let audio_path_candidate = "assets/audio/ramen_arrow2.aac";
    let audio_file_resolved = if args.no_audio {
        None
    } else if let Some(ref path) = args.audio_file {
        Some(path.as_str())
    } else if Path::new(audio_path_candidate).exists() {
        println!("🎵 Detectada pista de audio original: {}", audio_path_candidate);
        Some(audio_path_candidate)
    } else {
        println!("🎹 Usando síntesis de audio procedural zen integrada...");
        None
    };

    let audio_ref = if args.no_audio || audio_file_resolved.is_some() {
        None
    } else {
        Some(&audio)
    };

    println!("🎬 Renderizando video MP4 con SSAA {}x...", args.ss);
    export_mp4(&timeline, audio_ref, audio_file_resolved, width, height, &args.output, args.ss)?;

    // Generar también automáticamente la hoja de contacto para inspección visual rápida
    let contact_sheet_path = args.output.replace(".mp4", "-contact.jpg");
    println!("🖼️ Generando hoja de contacto complementaria...");
    generate_contact_sheet(&timeline, width, height, 24, 6, &contact_sheet_path, args.ss)?;

    println!("✨ ¡Renderizado completo exitosamente!");
    println!("   Video: {}", args.output);
    println!("   Contacto: {}", contact_sheet_path);

    Ok(())
}
