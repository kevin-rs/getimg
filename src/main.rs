use anyhow::Result;

/// The main entry point of `getimg`.
///
/// It parses command-line arguments using the `clap` crate, configures the client based on
/// the provided command-line options, and performs an operation using the specified subcommand.
#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "cli")]
    {
        use clap::Parser;
        use getimg::cli::{Cli, Command};
        use getimg::client::Client;
        use getimg::utils::{load_and_encode_image, save_image};
        use std::env;

        let args: Cli = Cli::parse();

        let api_key = if args.api_key.is_none() {
            env::var("GETIMG_API_KEY").unwrap_or_default().to_owned()
        } else {
            args.api_key.unwrap().to_owned()
        };

        let model = if args.model.is_none() {
            env::var("GETIMG_MODEL")
                .unwrap_or("lcm-realistic-vision-v5-1".to_string())
                .to_owned()
        } else {
            args.model.unwrap().to_owned()
        };

        let mut getimg_client = Client::new(&api_key, &model);

        match args.cmd {
            Command::Edit(cmd) => {
                println!("Generating edited image...");
                let result = getimg_client
                    .generate_edited_image(
                        &cmd.prompt,
                        Some(&cmd.negative_prompt),
                        &load_and_encode_image(&cmd.image)?,
                        cmd.image_guidance,
                        cmd.steps,
                        cmd.guidance,
                        cmd.seed.try_into().unwrap(),
                        &cmd.scheduler,
                        &cmd.output_format,
                    )
                    .await?;
                save_image(&result.image, "edited_image.png")?;
                println!("Edited image generated and stored successfully.");
            }
            Command::Repaint(cmd) => {
                println!("Repainting image...");
                let result = getimg_client
                    .generate_repainted_image(
                        &cmd.prompt,
                        Some(&cmd.negative_prompt),
                        &load_and_encode_image(&cmd.image)?,
                        &load_and_encode_image(&cmd.mask_image)?,
                        Some(cmd.strength),
                        cmd.width,
                        cmd.height,
                        cmd.steps,
                        cmd.guidance,
                        cmd.seed.try_into().unwrap(),
                        &cmd.scheduler,
                        &cmd.output_format,
                    )
                    .await?;
                save_image(&result.image, "edited_image.png")?;
                println!("Image repainted and stored successfully.");
            }
            Command::TextToImage(cmd) => {
                println!("Generating image from text...");
                let result = getimg_client
                    .generate_image_from_text(
                        &cmd.prompt,
                        cmd.width,
                        cmd.height,
                        cmd.steps,
                        &cmd.output_format,
                        Some(&cmd.negative_prompt),
                        Some(cmd.seed),
                    )
                    .await?;
                save_image(&result.image, "t2i.png")?;
                println!("Edited image generated and stored successfully.");
            }
            Command::ImageToImage(cmd) => {
                println!("Generating image from image...");
                let result = getimg_client
                    .generate_image_from_image(
                        &cmd.prompt,
                        &load_and_encode_image(&cmd.image)?,
                        cmd.steps,
                        cmd.seed,
                        &cmd.output_format,
                        Some(&cmd.negative_prompt),
                        Some(cmd.strength),
                    )
                    .await?;
                save_image(&result.image, "i2i.png")?;
                println!("Edited image generated and stored successfully.");
            }
            Command::ControlNet(cmd) => {
                println!("Generating image using ControlNet...");
                let result = getimg_client
                    .generate_image_using_controlnet(
                        &cmd.net,
                        &cmd.prompt,
                        &cmd.negative_prompt,
                        &load_and_encode_image(&cmd.image)?,
                        cmd.strength,
                        cmd.width,
                        cmd.height,
                        cmd.steps,
                        cmd.guidance,
                        cmd.seed,
                        &cmd.scheduler,
                        &cmd.output_format,
                    )
                    .await?;
                save_image(&result.image, "cnet.png")?;
                println!("Edited image generated and stored successfully.");
            }
        }
    }
    Ok(())
}
