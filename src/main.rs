use anyhow::{Context, Result};
use clap::Parser;
use image::{codecs::jpeg::JpegEncoder, imageops, Rgba, RgbaImage};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::{info, warn};

#[derive(Clone, Copy)]
struct HexColor(Rgba<u8>);

impl FromStr for HexColor {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let n = u32::from_str_radix(s, 16).map_err(|e| e.to_string())?;
        Ok(HexColor(Rgba([(n >> 16) as u8, (n >> 8) as u8, n as u8, 255])))
    }
}

#[derive(Parser)]
#[command(about = "Add borders to images.")]
struct Args {
    /// Input image files to process
    files: Vec<String>,

    #[arg(
        long,
        default_value_t = 0.01,
        help = "Top border as a fraction of image height (portrait)"
    )]
    top: f64,

    #[arg(
        long,
        default_value_t = 0.02,
        help = "Bottom border as a fraction of image height (portrait)"
    )]
    bottom: f64,

    #[arg(
        long,
        default_value_t = 0.01,
        help = "Side border as a fraction of image width (landscape)"
    )]
    side: f64,

    #[arg(
        long,
        default_value_t = 0.45,
        help = "Fraction of vertical border placed on top (landscape); must sum to 1.0 with --bottom-wide"
    )]
    top_wide: f64,

    #[arg(
        long,
        default_value_t = 0.55,
        help = "Fraction of vertical border placed on bottom (landscape); must sum to 1.0 with --top-wide"
    )]
    bottom_wide: f64,

    #[arg(
        long,
        default_value_t = 0.8,
        help = "Target aspect ratio (width/height) for the output image"
    )]
    ratio: f64,

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

fn process(
    file: &str,
    output_dir: &Path,
    top: f64,
    bottom: f64,
    side: f64,
    top_wide: f64,
    bottom_wide: f64,
    ratio: f64,
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

    let result = if width >= height {
        let border = (width as f64 * side) as u32;
        let with_sides = add_border(&img, border, border, 0, 0, color);
        let new_width = with_sides.width();
        let new_height = (new_width as f64 / ratio) as u32;
        let total_border = new_height.saturating_sub(height);
        let border_top = (total_border as f64 * top_wide) as u32;
        let border_bottom = (total_border as f64 * bottom_wide) as u32;
        add_border(&with_sides, 0, 0, border_top, border_bottom, color)
    } else {
        let border_top = (height as f64 * top) as u32;
        let border_bottom = (height as f64 * bottom) as u32;
        let with_tb = add_border(&img, 0, 0, border_top, border_bottom, color);
        let new_height = with_tb.height();
        let new_width = (new_height as f64 * ratio) as u32;
        let total_border = new_width.saturating_sub(width);
        let side_border = total_border / 2;
        add_border(&with_tb, side_border, side_border, 0, 0, color)
    };

    let rgb = image::DynamicImage::ImageRgba8(result).into_rgb8();
    let ext = output_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => {
            let mut f = std::fs::File::create(&output_path)
                .with_context(|| format!("failed to create {}", output_path.display()))?;
            JpegEncoder::new_with_quality(&mut f, quality)
                .encode_image(&rgb)
                .with_context(|| format!("failed to save to {}", output_path.display()))?;
        }
        _ => {
            rgb.save(&output_path)
                .with_context(|| format!("failed to save to {}", output_path.display()))?;
        }
    }

    info!("Saved to {}", output_path.display());
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    assert!(
        (args.top_wide + args.bottom_wide - 1.0).abs() < 1e-9,
        "top_wide + bottom_wide must equal 1.0"
    );

    let output_dir = PathBuf::from(&args.output_dir);
    std::fs::create_dir_all(&output_dir).context("failed to create output directory")?;

    for file in args.files {
        if let Err(e) = process(
            &file,
            &output_dir,
            args.top,
            args.bottom,
            args.side,
            args.top_wide,
            args.bottom_wide,
            args.ratio,
            args.quality,
            args.color.0,
        ) {
            warn!("error processing {file}: {e:#}");
        }
    }

    Ok(())
}
