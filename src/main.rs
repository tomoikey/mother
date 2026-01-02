mod text_box;
mod cli;

use crate::text_box::TextBox;
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use anyhow::{anyhow, bail};
use clap::Parser;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, GenericImageView, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use std::fs::File;
use std::path::Path;
use crate::cli::Args;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 256;
const FONT_BYTES: &[u8] = include_bytes!("../assets/MOTHER PIXEL2.ttf");
const DIALOG_IMAGE_BYTES: &[u8] = include_bytes!("../assets/dialog.png");
const TEXT_COLOR_WHITE: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255]);
const TEXT_COLOR_BROWN: Rgba<u8> = Rgba([222u8, 163u8, 134u8, 255]);

const SCALE: f32 = 28.0;
const PX_SCALE: PxScale = PxScale { x: SCALE, y: SCALE };

const TEXT_LENGTH_LIMIT: usize = 25;

fn draw_text(
    image_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &FontRef,
    text: String,
    x: f32,
    y: f32,
) {
    if text.starts_with("◆") {
        draw_text_mut(
            image_buffer,
            TEXT_COLOR_BROWN,
            x as i32,
            y as i32,
            PX_SCALE,
            font,
            "◆",
        );
        let text = text.chars().skip(1).collect::<String>();
        let font = font.as_scaled(PX_SCALE);
        draw_text_mut(
            image_buffer,
            TEXT_COLOR_WHITE,
            (x + font.h_advance(font.glyph_id('◆'))) as i32,
            y as i32,
            PX_SCALE,
            font.font(),
            &text,
        );
    } else {
        draw_text_mut(
            image_buffer,
            TEXT_COLOR_WHITE,
            x as i32,
            y as i32,
            PX_SCALE,
            font,
            &text,
        );
    }
}

fn init_image() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let text_box = image::load_from_memory(DIALOG_IMAGE_BYTES)
        .expect("Error loading embedded dialog image")
        .to_rgba8();

    let mut image_buffer = image::ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in text_box.enumerate_pixels() {
        if x < WIDTH && y < HEIGHT {
            image_buffer.put_pixel(x, y, *pixel);
        }
    }
    image_buffer
}

fn main() -> anyhow::Result<()> {
    let Args {
        text,
        output,
        speed,
        ..
    } = Args::parse();

    let image_buffer = init_image();

    let font = FontRef::try_from_slice(FONT_BYTES)?;
    let text = TextBox::<TEXT_LENGTH_LIMIT>::new(text);

    let path = Path::new(output.as_str());
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or(anyhow::anyhow!("Output file must have an extension"))?;

    match extension {
        "gif" => {
            generate_gif(
                GifEncoder::new(File::create(path)?),
                text,
                &font,
                image_buffer,
                speed,
            )?;
        }
        "png" => {
            generate_png(text, &font, image_buffer, path)?;
        }
        _ => bail!("Unsupported file extension: {}", extension),
    }

    Ok(())
}

fn generate_png(
    mut text: TextBox<TEXT_LENGTH_LIMIT>,
    font: &FontRef,
    base_image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    path: &Path,
) -> anyhow::Result<()> {
    let mut image_buffer = base_image_buffer.clone();
    let mut count = 0;
    while let Some(lines) = text.next() {
        draw_text(&mut image_buffer, font, lines[0].clone(), 40.0, 60.0);
        draw_text(
            &mut image_buffer,
            font,
            lines[1].clone(),
            40.0,
            HEIGHT as f32 / 2.0,
        );
        draw_text(
            &mut image_buffer,
            font,
            lines[2].clone(),
            40.0,
            HEIGHT as f32 - 60.0,
        );
        image_buffer.save(Path::new(&format!(
            "{}-{}.png",
            path.to_str().ok_or(anyhow!(""))?,
            count
        )))?;
        count += 1;
        image_buffer = base_image_buffer.clone();
    }
    Ok(())
}

fn generate_gif(
    mut encoder: GifEncoder<File>,
    mut text: TextBox<TEXT_LENGTH_LIMIT>,
    font: &FontRef,
    base_image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    speed: u32,
) -> anyhow::Result<()> {
    encoder.set_repeat(Repeat::Infinite)?;
    while let Some(lines) = text.next() {
        let mut image_buffer = base_image_buffer.clone();
        draw_text(&mut image_buffer, font, lines[0].clone(), 40.0, 60.0);
        draw_text(
            &mut image_buffer,
            font,
            lines[1].clone(),
            40.0,
            HEIGHT as f32 / 2.0,
        );
        draw_text(
            &mut image_buffer,
            font,
            lines[2].clone(),
            40.0,
            HEIGHT as f32 - 60.0,
        );

        let frame = Frame::from_parts(image_buffer, 0, 0, Delay::from_numer_denom_ms(speed, 1));
        encoder.encode_frame(frame)?;
    }
    Ok(())
}
