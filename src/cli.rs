//! This module contains the CLI functionalities for interacting with the GetImg API.

#[cfg(feature = "cli")]
use clap::builder::styling::{AnsiColor, Effects, Styles};
#[cfg(feature = "cli")]
use clap::{Args, Parser, Subcommand};

#[cfg(feature = "cli")]
fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[cfg(feature = "cli")]
#[derive(Parser, Debug, Clone)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    name = "getimg",
    propagate_version = true,
    styles = styles(),
    help_template = r#"{before-help}{name} {version}
{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    about=r#"
📸 GetImg
=========

A command-line tool for interacting with the GetImg AI API.

FUNCTIONALITIES:
  - Generate Edited Image: Generate an edited image using the GetImg API.
  - Repaint Image: Repaint an image using the GetImg API.
  - Generate Image from Text: Generate an image from text using the GetImg API.
  - Generate Image from Another Image: Generate an image from another image using the GetImg API.
  - Generate Images using ControlNet Conditioning: Generate images using ControlNet conditioning with the GetImg API.

USAGE:
  getimg [OPTIONS] <COMMAND>

EXAMPLES:
  Generate an edited image:
    getimg edit -p "A man riding a horse on Mars." -i image.jpg -s 25 -g 7.5 -e 25 -y 1.5 -o png -n "Disfigured, cartoon, blurry" -c ddim

  Repaint an image:
    getimg paint -p "An image of a cityscape with neon lights." -i image.png -m edited_image.png -w 512 -a 512 -e 50 -s 5 -g 10.0 -o jpeg -c euler -f 1 -n "Disfigured, cartoon, blurry"

  Generate an image from text:
    getimg t2i -p "A colorful sunset over the ocean." -w 512 -a 512 -s 5 -e 42 -o png -n "Disfigured, cartoon, blurry"

  Generate an image from another image:
    getimg i2i -p "Add a forest in the background." -i generated_image.png -s 6 -e 512 -o jpeg -f 0.5 -n "Disfigured, cartoon, blurry"

  Generate images using ControlNet conditioning:
    getimg cnet -p "A painting of a landscape." -i generated_image.png -f 1.0 -w 512 -a 512 -s 25 -g 7.5 -e 512 -c lms -o png -r canny-1.1 -n "Disfigured, cartoon, blurry"

For more information, visit: github.com/kevin-rs/getimg
"#
)]
pub struct Cli {
    /// API key for authentication.
    #[clap(short, long)]
    pub api_key: Option<String>,
    /// Model to be used.
    #[clap(short, long)]
    pub model: Option<String>,
    #[clap(subcommand)]
    pub cmd: Command,
}

#[cfg(feature = "cli")]
#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Edit(Edit),
    #[clap(name = "paint")]
    Repaint(Repaint),
    #[clap(name = "t2i")]
    TextToImage(TextToImage),
    #[clap(name = "i2i")]
    ImageToImage(ImageToImage),
    #[clap(name = "cnet")]
    ControlNet(ControlNet),
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct Edit {
    /// Text prompt for generating the edited image.
    #[clap(short, long)]
    pub prompt: String,
    /// Text input that will not guide the image generation.
    #[clap(short, long)]
    pub negative_prompt: String,
    /// Path to the input image file.
    #[clap(short, long)]
    pub image: String,
    /// Image guidance parameter.
    #[clap(short, long)]
    pub guidance: f64,
    /// Number of steps for image generation.
    #[clap(short, long)]
    pub steps: usize,
    /// Seed parameter.
    #[clap(short = 'e', long = "eed")]
    pub seed: usize,
    /// Scheduler parameter.
    #[clap(short = 'c', long = "cheduler")]
    pub scheduler: String,
    /// Output format for the image.
    #[clap(short, long)]
    pub output_format: String,
    /// Higher image guidance produces images that are closely linked to the source image.
    #[clap(short = 'y', long = "yuidance")]
    pub image_guidance: f64,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct Repaint {
    /// Text prompt for repainting the image.
    #[clap(short, long)]
    pub prompt: String,
    /// Text input that will not guide the image generation.
    #[clap(short, long)]
    pub negative_prompt: String,
    /// Path to the input image file.
    #[clap(short, long)]
    pub image: String,
    /// Path to the mask image file.
    #[clap(short, long)]
    pub mask_image: String,
    /// Width of the image.
    #[clap(short, long)]
    pub width: usize,
    /// Height of the image.
    #[clap(short = 'a', long = "hauteur")]
    pub height: usize,
    /// Number of steps for image generation.
    #[clap(short, long)]
    pub steps: usize,
    /// Scheduler parameter.
    #[clap(short = 'c', long = "cheduler")]
    pub scheduler: String,
    /// Seed parameter.
    #[clap(short = 'e', long = "eed")]
    pub seed: usize,
    /// Strength parameter for image generation.
    #[clap(short = 'f', long = "force")]
    pub strength: f64,
    /// Guidance parameter.
    #[clap(short, long)]
    pub guidance: f64,
    /// Output format for the image.
    #[clap(short, long)]
    pub output_format: String,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct TextToImage {
    /// Text prompt for generating the image.
    #[clap(short, long)]
    pub prompt: String,
    /// Text input that will not guide the image generation.
    #[clap(short, long)]
    pub negative_prompt: String,
    /// Width of the image.
    #[clap(short, long)]
    pub width: usize,
    /// Height of the image.
    #[clap(short = 'a', long = "hauteur")]
    pub height: usize,
    /// Number of steps for image generation.
    #[clap(short, long)]
    pub steps: usize,
    /// Seed parameter.
    #[clap(short = 'e', long = "eed")]
    pub seed: usize,
    /// Output format for the image.
    #[clap(short, long)]
    pub output_format: String,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct ImageToImage {
    /// Text prompt for generating the image.
    #[clap(short, long)]
    pub prompt: String,
    /// Text input that will not guide the image generation.
    #[clap(short, long)]
    pub negative_prompt: String,
    /// Path to the input image file.
    #[clap(short, long)]
    pub image: String,
    /// Strength parameter for image generation.
    #[clap(short = 'f', long = "force")]
    pub strength: f64,
    /// Number of steps for image generation.
    #[clap(short, long)]
    pub steps: usize,
    /// Output format for the image.
    #[clap(short, long)]
    pub output_format: String,
    /// Seed parameter.
    #[clap(short = 'e', long = "eed")]
    pub seed: usize,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct ControlNet {
    /// ControlNet conditioning type.
    #[clap(short = 'r', long)]
    pub net: String,
    /// Text prompt for generating the image.
    #[clap(short, long)]
    pub prompt: String,
    /// Text input that will not guide the image generation.
    #[clap(short, long)]
    pub negative_prompt: String,
    /// Path to the input image file.
    #[clap(short, long)]
    pub image: String,
    /// Strength parameter for image generation.
    #[clap(short = 'f', long = "force")]
    pub strength: f64,
    /// Width of the image.
    #[clap(short, long)]
    pub width: usize,
    /// Height of the image.
    #[clap(short = 'a', long = "hauteur")]
    pub height: usize,
    /// Number of steps for image generation.
    #[clap(short, long)]
    pub steps: usize,
    /// Guidance parameter.
    #[clap(short, long)]
    pub guidance: f64,
    /// Output format for the image.
    #[clap(short, long)]
    pub output_format: String,
    /// Seed parameter.
    #[clap(short = 'e', long = "eed")]
    pub seed: usize,
    /// Scheduler parameter.
    #[clap(short = 'c', long = "cheduler")]
    pub scheduler: String,
}
