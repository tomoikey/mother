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

pub const WIDTH: u32 = 960;
pub const HEIGHT: u32 = 256;

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
            const BEEP_WAV: &[u8] = include_bytes!("../assets/text_blip.wav");
            let temp_se_path = std::env::temp_dir().join("mother_beep.wav");
            std::fs::write(&temp_se_path, BEEP_WAV)?;
            let fps = args.speed() as f32;
            let frame_count = frames.len();
            let ms_per_frame = 1000.0 / fps;
            let mut split_labels = Vec::new();
            for i in 0..frame_count {
                split_labels.push(format!("[s{i}]"));
            }
            let split_cmd = format!(
                "[1:a]volume=10.0,asplit={frame_count}{}",
                split_labels.join("")
            );
            let mut adelays = Vec::new();
            let mut amix_inputs = Vec::new();
            for i in 0..frame_count {
                let delay_ms = (i as f32 * ms_per_frame) as u32;
                let input_label = format!("[s{}]", i);
                let output_label_name = format!("a{}", i);
                adelays.push(format!(
                    "{}adelay={}|{}[{}]",
                    input_label, delay_ms, delay_ms, output_label_name
                ));
                amix_inputs.push(format!("[{}]", output_label_name));
            }

            let filter_complex = format!(
                "{}; {}; {}amix=inputs={}:duration=longest[outa]",
                split_cmd,
                adelays.join(";"),
                amix_inputs.join(""),
                frame_count
            );

            let mut child = Command::new("ffmpeg")
                .args([
                    "-y",
                    "-f",
                    "rawvideo",
                    "-pixel_format",
                    "rgba",
                    "-video_size",
                    &format!("{WIDTH}x{HEIGHT}"),
                    "-framerate",
                    &fps.to_string(),
                    "-i",
                    "-",
                    "-i",
                    temp_se_path.to_str().ok_or(anyhow!("invalid temp path"))?,
                    "-filter_complex",
                    &filter_complex,
                    "-map",
                    "0:v:0",
                    "-map",
                    "[outa]",
                    "-c:v",
                    "libx264",
                    "-pix_fmt",
                    "yuv420p",
                    "-movflags",
                    "faststart",
                    output_path.to_str().ok_or(anyhow!("invalid path"))?,
                ])
                .stdin(Stdio::piped())
                .spawn()?;

            let mut stdin = child.stdin.take().expect("failed to open stdin");
            frames.into_iter().for_each(|frame| {
                stdin
                    .write_all(&frame.into_raw())
                    .expect("failed to write frame")
            });
            drop(stdin);

            let status = child.wait()?;
            std::fs::remove_file(&temp_se_path).ok();
            if !status.success() {
                bail!("ffmpeg exited with status {status}");
            }
        }
    }

    if args.open_immediately() {
        open::that(output_path).map_err(|e| anyhow!("failed to open immediately: {e}"))?;
    }

    Ok(())
}
