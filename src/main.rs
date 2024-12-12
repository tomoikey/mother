mod text_box;

use crate::text_box::TextBox;
use ab_glyph::{FontRef, PxScale};
use clap::{Parser, ValueEnum};
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use std::fs::File;
use std::path::Path;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 256;
const FONT_BYTES: &[u8] = include_bytes!("../assets/MOTHER PIXEL2.ttf");
const TEXT_COLOR: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255]);

const SCALE: f32 = 28.0;
const PX_SCALE: PxScale = PxScale { x: SCALE, y: SCALE };

fn draw_text(
    image_buffer: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &FontRef,
    text: String,
    x: f32,
    y: f32,
) {
    draw_text_mut(
        image_buffer,
        TEXT_COLOR,
        x as i32,
        y as i32,
        PX_SCALE,
        font,
        &text,
    );
}

fn init_image() -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image_buffer = image::ImageBuffer::new(WIDTH, HEIGHT);
    let text_box = image::open("assets/dialog.png").expect("Error opening image");
    for (x, y, pixel) in text_box.pixels() {
        image_buffer.put_pixel(x, y, pixel);
    }
    image_buffer
}

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    text: String,
    #[clap(short, long)]
    output: String,
    #[clap(short, long, default_value = "8")]
    speed: u32,
    #[clap(short, long, default_value = "plain")]
    theme: Theme,
}

#[derive(Clone, Copy, ValueEnum)]
enum Theme {
    Plain,
    Banana,
}

fn main() -> anyhow::Result<()> {
    let Args {
        text,
        output,
        speed,
        theme,
    } = Args::parse();

    let image_buffer = init_image();

    let font = FontRef::try_from_slice(FONT_BYTES)?;
    let mut text = TextBox::<20>::new(text);

    let mut encoder = GifEncoder::new(File::create(Path::new(output.as_str()))?);

    encoder.set_repeat(Repeat::Infinite)?;

    while let Some(lines) = text.next() {
        let mut image_buffer = image_buffer.clone();
        draw_text(&mut image_buffer, &font, lines[0].clone(), 40.0, 60.0);
        draw_text(
            &mut image_buffer,
            &font,
            lines[1].clone(),
            40.0,
            HEIGHT as f32 / 2.0,
        );
        draw_text(
            &mut image_buffer,
            &font,
            lines[2].clone(),
            40.0,
            HEIGHT as f32 - 60.0,
        );

        let frame = Frame::from_parts(image_buffer, 0, 0, Delay::from_numer_denom_ms(speed, 100));
        encoder.encode_frame(frame)?;
    }

    Ok(())
}
