use clap::Parser;
use std::fs::create_dir_all;
use std::path::Path;
use taller_film::{
    create_demo_quality_film, create_fly_film, create_four_looks_film, create_macondo_film,
    create_monarch_film, create_okazz_film, create_ramen_film, create_tortoise_film,
    create_xray_film, export_mp4, generate_contact_sheet,
};

#[derive(Parser, Debug)]
#[command(name = "taller_film")]
#[command(author = "Antigravity & User")]
#[command(version = "0.1.0")]
#[command(about = "Motor de animación procedural y minivideos artesanales en Rust (hand-drawn/riso/ink)", long_about = None)]
struct Args {
    /// Película o estilo a renderizar: "ramen", "tortoise", "okazz", "demo", "macondo", "monarch", "fly", "xray", "four_looks"
    #[arg(short, long, default_value = "ramen")]
    film: String,

    /// Ruta de salida para el video MP4 o imagen de contacto
    #[arg(short, long, default_value = "out/tortoise_film.mp4")]
    output: String,

    /// Generar hoja de contacto (contact sheet) con N fotogramas (ej: 24) en lugar de MP4
    #[arg(short, long)]
    grid: Option<usize>,

    /// Relación de aspecto: "1:1" (cuadrado), "16:9" (horizontal), "9:16" (vertical/reels)
    #[arg(long, default_value = "default")]
    ar: String,

    /// Ancho en píxeles del renderizado
    #[arg(long, default_value_t = 1080)]
    width: u32,

    /// Deshabilitar la banda sonora procedural
    #[arg(long, default_value_t = false)]
    no_audio: bool,

    /// Ruta a archivo de audio externo (ej: MP3/WAV) para usar la pista musical real
    #[arg(long)]
    audio_file: Option<String>,

    /// Factor de supersampling (SSAA) para nitidez extrema en líneas finas (1 = 1x estándar, 2 = 2x SSAA con Lanczos3)
    #[arg(long, default_value_t = 2)]
    ss: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Crear directorio de salida si no existe
    if let Some(parent) = Path::new(&args.output).parent() {
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent)?;
        }
    }

    // Calcular dimensiones según el aspect ratio y la película
    let (width, height) = match args.ar.as_str() {
        "16:9" => (1920, 1080),
        "9:16" => (1080, 1920),
        "1:1" => (args.width, args.width),
        _ => {
            if args.film == "macondo" {
                (1920, 1080) // 16:9 Panorámico para YouTube por defecto en Macondo
            } else if args.film == "monarch" {
                (1080, 1920) // Vertical por defecto para la monarca (9:16 formato cine/reels)
            } else if args.film == "ramen" || args.film == "ramen_exploded" || args.film == "arrow2" {
                (720, 900) // Relación técnica 4:5 nativa para el diagrama de ramen (Arrow 2)
            } else {
                (args.width, args.width) // 1:1 por defecto para tortoise, okazz y los demás
            }
        }
    };

    // Cargar la película seleccionada
    let (timeline, audio) = match args.film.as_str() {
        "ramen" | "ramen_exploded" | "arrow2" | "fideos" => {
            println!("🍜 Cargando película: 'Ramen Exploded Diagram' (12.5s — Arrow 2 / @99cccc)...");
            create_ramen_film()
        }
        "tortoise" | "kevin_tortoise" | "flight" | "geese" | "tortuga" => {
            println!("🐢 Cargando película: 'Tortoise Flight' (64s — Kevin Ngo & Claude Opus 5)...");
            create_tortoise_film()
        }
        "okazz" | "rain" | "puddle" => {
            println!("🌧️ Cargando película: 'Okazz Rain' (16s — Ondas elípticas, corona de salpicaduras y gotas de agua @okazz_)...");
            create_okazz_film()
        }
        "demo" | "demo_quality" | "kevin" => {
            println!("✨ Cargando película: 'High-Quality Cinema Demo' (16s — Estándar Kevin Ngo / Claude Opus)...");
            create_demo_quality_film()
        }
        "macondo" => {
            println!("🎺 Cargando película: 'Macondo' (209s — Óscar Chávez & Realismo Mágico)...");
            create_macondo_film()
        }
        "monarch" => {
            println!("🦋 Cargando película: 'The Life of a Monarch Butterfly' (32s — estilo procedural-film)...");
            create_monarch_film()
        }
        "fly" => {
            println!("🪰 Cargando película: 'Fly Style' (The life of a fruit fly)...");
            create_fly_film()
        }
        "xray" => {
            println!("🔬 Cargando película: 'X-Ray Story' (18s — efectos visuales ricos)...");
            create_xray_film()
        }
        _ => {
            println!("⛵ Cargando película: 'Four Looks' (Paper boat: Riso, Screen, Pencil, Ink)...");
            create_four_looks_film()
        }
    };

    println!(
        "📐 Formato: {}x{} ({}) | Duración: {:.1}s ({} fotogramas a {} fps)",
        width,
        height,
        args.ar,
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
        println!("🖼️ Generando hoja de contacto de {} fotogramas (rejilla {}x{}, SSAA {}x)...", tile_count, cols, (tile_count + cols - 1) / cols, args.ss);
        generate_contact_sheet(&timeline, width, height, tile_count, cols, &contact_path, args.ss)?;
        return Ok(());
    }

    // Si se solicitó exportar a video MP4
    let audio_file_resolved = if args.no_audio {
        None
    } else if let Some(ref path) = args.audio_file {
        Some(path.as_str())
    } else if (args.film == "ramen" || args.film == "ramen_exploded" || args.film == "arrow2" || args.film == "fideos") && Path::new("assets/audio/ramen_arrow2.aac").exists() {
        Some("assets/audio/ramen_arrow2.aac")
    } else if (args.film == "tortoise" || args.film == "kevin_tortoise" || args.film == "flight" || args.film == "geese" || args.film == "tortuga") && Path::new("assets/audio/tortoise_kevin_ngo.aac").exists() {
        Some("assets/audio/tortoise_kevin_ngo.aac")
    } else if args.film == "macondo" && Path::new("assets/audio/macondo_oscar_chavez.mp3").exists() {
        Some("assets/audio/macondo_oscar_chavez.mp3")
    } else if args.film == "monarch" && Path::new("assets/audio/monarch_score.aac").exists() {
        Some("assets/audio/monarch_score.aac")
    } else {
        None
    };

    let audio_ref = if args.no_audio || audio_file_resolved.is_some() { None } else { Some(&audio) };
    export_mp4(&timeline, audio_ref, audio_file_resolved, width, height, &args.output, args.ss)?;

    // Generar también automáticamente la hoja de contacto para inspección visual rápida
    let contact_sheet_path = args.output.replace(".mp4", "-contact.jpg");
    println!("🖼️ Generando hoja de contacto complementaria...");
    generate_contact_sheet(&timeline, width, height, 24, 6, &contact_sheet_path, args.ss)?;

    Ok(())
}
