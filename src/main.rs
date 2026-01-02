mod cli;
mod drawer;
mod text_box;

use crate::cli::{Args, OutputFileExtension};
use crate::drawer::Drawer;
use ab_glyph::PxScale;
use anyhow::anyhow;
use clap::Parser;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, Rgba};
use std::fs::File;
use std::path::Path;

const TEXT_COLOR_WHITE: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255]);
const TEXT_COLOR_BROWN: Rgba<u8> = Rgba([222u8, 163u8, 134u8, 255]);

const SCALE: f32 = 28.0;
const PX_SCALE: PxScale = PxScale { x: SCALE, y: SCALE };

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (text, output_path, output_file_extension, speed) = (
        args.text(),
        args.output_path(),
        args.output_file_extension(),
        args.speed(),
    );
    let output_file_extension = output_file_extension?;
    let frames = Drawer::new()?.generate_frames(text)?;
    match output_file_extension {
        OutputFileExtension::Gif => {
            let mut encoder = GifEncoder::new(File::create(output_path)?);
            encoder.set_repeat(Repeat::Infinite)?;
            frames
                .into_iter()
                .map(|frame| Frame::from_parts(frame, 0, 0, Delay::from_numer_denom_ms(speed, 1)))
                .for_each(|frame| {
                    encoder.encode_frame(frame).expect("Failed to encode frame");
                })
        }
        OutputFileExtension::Png => {
            let output_path = output_path.to_str().ok_or(anyhow!(""))?;
            frames.into_iter().enumerate().for_each(|(i, frame)| {
                frame
                    .save(Path::new(&format!("{output_path}-{i}.png")))
                    .expect("Failed to save frame");
            })
        }
    }

    Ok(())
}
