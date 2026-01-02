use crate::text_box::TextBox;
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 256;
const TEXT_COLOR_WHITE: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255]);
const TEXT_COLOR_BROWN: Rgba<u8> = Rgba([222u8, 163u8, 134u8, 255]);

const SCALE: f32 = 28.0;
const PX_SCALE: PxScale = PxScale { x: SCALE, y: SCALE };
const DIALOG_IMAGE_BYTES: &[u8] = include_bytes!("../assets/dialog.png");
const FONT_BYTES: &[u8] = include_bytes!("../assets/MOTHER PIXEL2.ttf");
const TEXT_LENGTH_LIMIT: usize = 25;
const TEXT_POSITIONS: [(f32, f32); 3] = [
    (40.0, 60.0),
    (40.0, HEIGHT as f32 / 2.0),
    (40.0, HEIGHT as f32 - 60.0),
];

pub struct Drawer {
    font: FontRef<'static>,
    original_text_box_image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Drawer {
    pub fn new() -> anyhow::Result<Self> {
        let text_box = image::load_from_memory(DIALOG_IMAGE_BYTES)
            .expect("Error loading embedded dialog image")
            .to_rgba8();

        let mut original_text_box_image_buffer = image::ImageBuffer::new(WIDTH, HEIGHT);
        text_box.enumerate_pixels().for_each(|(x, y, px)| {
            if x < WIDTH && y < HEIGHT {
                original_text_box_image_buffer.put_pixel(x, y, *px);
            }
        });
        Ok(Self {
            font: FontRef::try_from_slice(FONT_BYTES)?,
            original_text_box_image_buffer,
        })
    }

    pub fn draw(&self, text: &str) -> anyhow::Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>> {
        let mut text = TextBox::<TEXT_LENGTH_LIMIT>::new(text);
        let mut result = Vec::new();
        while let Some(lines) = text.next() {
            let mut image_buffer = self.original_text_box_image_buffer.clone();
            TEXT_POSITIONS
                .into_iter()
                .enumerate()
                .for_each(|(i, (x, y))| {
                    draw_text(&mut image_buffer, &self.font, lines[i].clone(), x, y)
                });
            result.push(image_buffer);
        }
        Ok(result)
    }
}

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
