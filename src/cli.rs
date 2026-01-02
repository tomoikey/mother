use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Text to display
    #[clap(short, long)]
    pub text: String,
    /// Output file
    #[clap(short, long, default_value = "./output.gif")]
    pub output: String,
    /// Speed of the gif
    #[clap(short, long, default_value = "8")]
    pub speed: u32,
}