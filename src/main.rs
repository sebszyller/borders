use anyhow::{Context, Result};
use clap::Parser;
use image::{codecs::jpeg::JpegEncoder, imageops, Rgba, RgbaImage};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::{error, info};

#[derive(Clone, Copy)]
struct HexColor(Rgba<u8>);

impl FromStr for HexColor {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let n = u32::from_str_radix(s, 16).map_err(|e| e.to_string())?;
        Ok(HexColor(Rgba([
            (n >> 16) as u8,
            (n >> 8) as u8,
            n as u8,
            255,
        ])))
    }
}

#[derive(Parser)]
#[command(about = "Add borders to images.")]
struct Args {
    files: Vec<String>,

    #[arg(
        long,
        default_value_t = 0.02,
        help = "Left/right borders as a fraction of image height"
    )]
    default_horizontal: f64,

    #[arg(
        long,
        default_value_t = 0.03,
        help = "Top/bottom borders as a fraction of image width"
    )]
    default_vertical: f64,

    #[arg(
        long,
        default_value_t = 0.45,
        help = "Fraction of vertical border placed on top; must sum to 1.0 with --fraction_top"
    )]
    fraction_top: f64,

    #[arg(
        long,
        default_value_t = 0.55,
        help = "Fraction of vertical border placed on bottom; must sum to 1.0 with --fraction_bottom"
    )]
    fraction_bottom: f64,

    #[arg(
        long,
        default_value_t = 0.8,
        help = "Target aspect ratio (width/height) for the output image"
    )]
    target_ratio: f64,

    #[arg(
        long,
        default_value = "with_borders",
        help = "Directory to write output images into"
    )]
    output_dir: String,

    #[arg(
        long,
        default_value_t = 100,
        help = "JPEG output quality (1–100); ignored for non-JPEG formats"
    )]
    quality: u8,

    #[arg(
        long,
        default_value = "ffffff",
        help = "Border colour as a hex string (e.g. ffffff)"
    )]
    color: HexColor,
}

fn add_border(
    img: &RgbaImage,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
    color: Rgba<u8>,
) -> RgbaImage {
    let new_width = img.width() + left + right;
    let new_height = img.height() + top + bottom;
    let mut out = RgbaImage::from_pixel(new_width, new_height, color);
    imageops::overlay(&mut out, img, left as i64, top as i64);
    out
}

fn save_image(result: RgbaImage, output_path: &Path, quality: u8) -> Result<()> {
    let rgb = image::DynamicImage::ImageRgba8(result).into_rgb8();
    let ext = output_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => {
            let mut f = std::fs::File::create(output_path)
                .with_context(|| format!("failed to create {}", output_path.display()))?;
            JpegEncoder::new_with_quality(&mut f, quality)
                .encode_image(&rgb)
                .with_context(|| format!("failed to save to {}", output_path.display()))?;
        }
        all_else => {
            panic!("Not sure how to save files with extension {all_else}");
        }
    }
    Ok(())
}

fn process(
    file: &str,
    output_dir: &Path,
    default_horizontal: f64,
    default_vertical: f64,
    fraction_top: f64,
    fraction_bottom: f64,
    target_ratio: f64,
    quality: u8,
    color: Rgba<u8>,
) -> Result<()> {
    let input_path = Path::new(file);
    let output_path = output_dir.join(input_path.file_name().context("no filename")?);

    info!("Processing {file}");

    let img = image::open(input_path)
        .with_context(|| format!("failed to open {file}"))?
        .into_rgba8();

    let width = img.width();
    let height = img.height();

    let border = (width as f64 * (default_horizontal / 2.0)) as u32;
    let with_sides = add_border(&img, border, border, 0, 0, color);
    let current_ratio = with_sides.width() as f64 / height as f64;

    const JPEG_MAX_DIM: u32 = 65535; // 2**16

    let result = if current_ratio >= target_ratio {
        let new_width = with_sides.width();
        let new_height = (new_width as f64 / target_ratio) as u32;
        if new_height > JPEG_MAX_DIM {
            anyhow::bail!("output height {new_height} exceeds JPEG maximum of {JPEG_MAX_DIM}");
        }
        let total_border = new_height.saturating_sub(height);
        let border_top = (total_border as f64 * fraction_top) as u32;
        let border_bottom = (total_border as f64 * fraction_bottom) as u32;
        add_border(&with_sides, 0, 0, border_top, border_bottom, color)
    } else {
        let border_top = (height as f64 * default_vertical * fraction_top) as u32;
        let border_bottom = (height as f64 * default_vertical * fraction_bottom) as u32;
        let with_tb = add_border(&img, 0, 0, border_top, border_bottom, color);
        let new_height = with_tb.height();
        let new_width = (new_height as f64 * target_ratio) as u32;
        if new_width > JPEG_MAX_DIM {
            anyhow::bail!("output width {new_width} exceeds JPEG maximum of {JPEG_MAX_DIM}");
        }
        let total_border = new_width.saturating_sub(width);
        let side_border = total_border / 2;
        add_border(&with_tb, side_border, side_border, 0, 0, color)
    };

    save_image(result, &output_path, quality)?;
    info!("Saved to {}", output_path.display());
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    assert!(
        (args.fraction_bottom + args.fraction_top - 1.0).abs() < 1e-9,
        "fraction_bottom + fraction_top must equal 1.0"
    );

    let output_dir = PathBuf::from(&args.output_dir);
    std::fs::create_dir_all(&output_dir).context("failed to create output directory")?;

    for file in args.files {
        if let Err(e) = process(
            &file,
            &output_dir,
            args.default_horizontal,
            args.default_vertical,
            args.fraction_top,
            args.fraction_bottom,
            args.target_ratio,
            args.quality,
            args.color.0,
        ) {
            error!("error processing {file}: {e:#}");
        }
    }

    Ok(())
}
