use anyhow::anyhow;
use clap::Parser;
use std::path::Path;

#[derive(Parser)]
pub struct Args {
    /// Text to display
    #[clap(short, long)]
    text: String,
    /// Output file
    #[clap(short, long, default_value = "./output.gif")]
    output: String,
    /// Speed of the gif
    #[clap(short, long, default_value = "8")]
    speed: u32,
}

impl Args {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn output_path(&self) -> &Path {
        Path::new(self.output.as_str())
    }

    pub fn output_file_extension(&self) -> anyhow::Result<OutputFileExtension> {
        let extension = self
            .output_path()
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or(anyhow::anyhow!("Output file must have an extension"))?;

        match extension {
            "gif" => Ok(OutputFileExtension::Gif),
            "png" => Ok(OutputFileExtension::Png),
            "mp4" => Ok(OutputFileExtension::Mp4),
            _ => Err(anyhow!("Unknown extension: {}", extension)),
        }
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }
}

pub enum OutputFileExtension {
    Gif,
    Png,
    Mp4,
}
