use clap::{builder::ValueParser, Parser};
use image::{ImageBuffer, ImageReader, Rgb};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    /// Path to the input image
    #[arg(short, long, value_name = "IMAGE_PATH", required = true)]
    image_path: PathBuf,

    /// Path to the output image
    #[arg(short, long, value_name = "OUTPUT_PATH", default_value = "output.png")]
    output_path: PathBuf,

    /// Disable resizing of the image
    #[arg(long)]
    no_resize: bool,

    /// To terminal display
    #[arg(short, long, conflicts_with_all = &["output_path", "no_resize"])]
    to_terminal: bool,

    /// Change the font size (default is 8)
    #[arg(long, value_name = "FONT_SIZE", default_value_t = 8.0, value_parser = ValueParser::new(parse_font_size))]
    font_size: f32,
}

const RATIO: [f32; 2] = [1.0, 0.5]; // [for file, for terminal]
const ASCII_CHARS: [char; 10] = [' ', '.', ':', 'c', 'o', 'P', 'O', '#', '%', '@'];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Image to ASCII converter");
    let args = Cli::parse();

    let image_path = args.image_path;
    println!("Reading image from path: {}", image_path.display());

    // Load the image
    let mut img = ImageReader::open(image_path).expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .into_rgb8();

    println!("--- Load completed ---");

    // Checking resize
    if !args.no_resize {
        println!("Resizing image...");
        img = resize_image(&img, args.to_terminal);
    } else {
        println!("Resizing is disabled, using original image size.");
    }

    // Checking output source
    if args.to_terminal {
        println!("--- Converting to ascii for terminal... ---");

        let ascii_output = to_ascii_terminal(&img);
        println!("{}", ascii_output);
    } else {
        // Load font
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("fonts/MozillaText-Regular.ttf"))
        .expect("Failed to load font");

        println!("--- Converting image to ASCII and saving to file... ---");
        println!("Using font size: {}", args.font_size);

        let output_image = to_ascii(&img, args.font_size, &font);
        output_image.save(&args.output_path).expect("Failed to save output image");
        println!("Output image saved to: {}", args.output_path.display());
    }

    Ok(())
}

fn resize_image(img: &image::RgbImage, to_terminal: bool) -> image::RgbImage{
    let (width, height) = img.dimensions();
    let mut new_width = if width > 1000 {width / 4} else {width / 2};
    let mut ratio = RATIO[0]; 

    if to_terminal {
        new_width = new_width.min(100);
        ratio = RATIO[1];
    }
    new_width = new_width.max(20);

    let new_height = (height as f32 * (new_width as f32 / width as f32 * ratio)) as u32;

    println!("Resizing image to {}x{} pixels.", new_width, new_height);

    image::imageops::resize(img, new_width, new_height, image::imageops::FilterType::Triangle)
}

// For outputing an image file
fn to_ascii(img: &image::RgbImage, font_size: f32, font: &ab_glyph::FontArc) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    let (width, height) = img.dimensions();
    let output_width = width * font_size as u32;
    let output_height = height * font_size as u32;
    let mut output_image = ImageBuffer::new(output_width, output_height);

    let scale = ab_glyph::PxScale::from(font_size);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = compute_brightness(pixel, ASCII_CHARS.len());
            let character = ASCII_CHARS[brightness.min(ASCII_CHARS.len() - 1)];
            let rgb = *pixel;

            imageproc::drawing::draw_text_mut(
                &mut output_image,
                rgb,
                (x * font_size as u32) as i32,
                (y * font_size as u32) as i32,
                scale,
                &font,
                &character.to_string(),
            );
        }
    }

    output_image
}

// For outputing to terminal
fn to_ascii_terminal(img: &image::RgbImage) -> String{
    let mut ascii_string = String::new();    
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = compute_brightness(pixel, ASCII_CHARS.len());
            let character = ASCII_CHARS[brightness.min(ASCII_CHARS.len() - 1)];
            ascii_string.push(character);
        }
        ascii_string.push('\n');
    }

    ascii_string
}

fn compute_brightness(pixel: &Rgb<u8>, len: usize) -> usize {
    let [r, g, b] = pixel.0;
    ((0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) * len as f32 / 256.0) as usize
}

fn parse_font_size(s: &str) -> Result<f32, String> {
    let size: f32 = s.parse().map_err(|_| "not a valid float".to_string())?;
    if size < 1.0 {
        return Err("font size must be at least 1.0".to_string());
    }
    Ok(size)
}