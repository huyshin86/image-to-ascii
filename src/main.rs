use image::{ImageReader, ImageBuffer, Rgb};
use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Image to ASCII converter");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        return Err("No image path provided.".into());
    }

    let image_path = &args[1];
    println!("Reading image from path: {}", image_path);

    // Load the image
    let img = ImageReader::open(image_path)?
        .decode()?
        .to_luma8();

    println!("Load complete, converting image... ");
    
    let resized = resize_image(&img);
    let final_image = to_ascii(&resized);

    let output_path = "output.png";
    println!("Saving image to path: {}", output_path);
    final_image.save(output_path)?;

    Ok(())
}

fn resize_image(img: &image::GrayImage) -> image::GrayImage{
    let (width, height) = img.dimensions();
    let new_width = (if width > 1000 {width / 6} else {width / 2}).min(100) as u32;
    let new_height = (height as f32 * (new_width as f32 / width as f32 / 2.0)) as u32;

    image::imageops::resize(img, new_width, new_height, image::imageops::FilterType::Nearest)
}

fn to_ascii(img: &image::GrayImage) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    const ASCII_CHARS: [char; 10] = [' ', '.', ':', 'c', 'o', 'P', 'O', '#', '%', '@'];
    const FONT_SIZE: f32 = 6.0;
    let (width, height) = img.dimensions();
    let output_width = width * FONT_SIZE as u32;
    let output_height = height * FONT_SIZE as u32;
    let mut output_image = ImageBuffer::new(output_width, output_height);

    let font = ab_glyph::FontArc::try_from_slice(include_bytes!("../MozillaText-Regular.ttf"))
    .expect("Failed to load font");

    let scale = ab_glyph::PxScale::from(FONT_SIZE);

    for y in 0..height {
        for x in 0..width {
            let brightness = img.get_pixel(x, y)[0] as usize * ASCII_CHARS.len() / 256;
            let character = ASCII_CHARS[brightness];

            imageproc::drawing::draw_text_mut(
                &mut output_image,
                Rgb([255, 255, 255]),
                (x * FONT_SIZE as u32) as i32,
                (y * FONT_SIZE as u32) as i32,
                scale,
                &font,
                &character.to_string(),
            );
        }
    }

    output_image
}