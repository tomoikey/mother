mod cli;
mod drawer;
mod text_box;

use crate::cli::{Args, OutputFileExtension};
use crate::drawer::Drawer;
use anyhow::{anyhow, bail};
use clap::Parser;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let output_path = args.output_path();
    let frames = Drawer::new()?.draw(args.text())?;
    match args.output_file_extension()? {
        OutputFileExtension::Gif => {
            let mut encoder = GifEncoder::new(File::create(output_path)?);
            encoder.set_repeat(Repeat::Infinite)?;
            frames
                .into_iter()
                .map(|f| Frame::from_parts(f, 0, 0, Delay::from_numer_denom_ms(args.speed(), 1)))
                .for_each(|f| encoder.encode_frame(f).expect("Failed to encode frame"))
        }
        OutputFileExtension::Png => {
            let output_path = output_path.to_str().ok_or(anyhow!(""))?;
            frames.into_iter().enumerate().for_each(|(i, frame)| {
                frame
                    .save(Path::new(&format!("{output_path}-{i}.png")))
                    .expect("Failed to save frame")
            })
        }
        OutputFileExtension::Mp4 => {
            let mut child = Command::new("ffmpeg")
                .args([
                    "-y",
                    "-f",
                    "rawvideo",
                    "-pixel_format",
                    "rgba",
                    "-video_size",
                    &format!("{}x{}", drawer::WIDTH, drawer::HEIGHT),
                    "-framerate",
                    &args.speed().to_string(),
                    "-i",
                    "-",
                    "-c:v",
                    "libx264",
                    "-pix_fmt",
                    "yuv420p",
                    "-movflags",
                    "faststart",
                    output_path.to_str().ok_or(anyhow!("invalid path"))?,
                ])
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|e| anyhow!("failed to spawn ffmpeg: {}", e))?;

            let mut stdin = child.stdin.take().expect("failed to open stdin");
            frames.into_iter().for_each(|frame| {
                stdin
                    .write_all(&frame.into_raw())
                    .expect("failed to write frame")
            });
            drop(stdin);

            let status = child.wait()?;
            if !status.success() {
                bail!("ffmpeg exited with status {}", status);
            }
        }
    }
    Ok(())
}
