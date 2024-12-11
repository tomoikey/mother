mod text_box;

use ab_glyph::{FontRef, PxScale};
use image::{GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;

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

fn main() {
    let mut image_buffer = image::ImageBuffer::new(WIDTH, HEIGHT);
    let text_box = image::open("assets/dialog.png").expect("Error opening image");

    for (x, y, pixel) in text_box.pixels() {
        image_buffer.put_pixel(x, y, pixel);
    }

    let font = FontRef::try_from_slice(FONT_BYTES).expect("Error constructing Font");
    draw_text(
        &mut image_buffer,
        &font,
        "◆ガチャン！ツーツーツー".to_string(),
        40.0,
        60.0,
    );
    image_buffer.save("output.png").unwrap();
}
