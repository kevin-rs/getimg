# 📸 GetImg

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/kevin-rs/getimg/tree/main.svg?style=svg&circle-token=CCIPRJ_AkZQSYSmTNzZbyqWGE5RMG_b3c947d6792f048c97a359440bc48da827c92ce4)](https://dl.circleci.com/status-badge/redirect/gh/kevin-rs/getimg/tree/main)
[![Crates.io](https://img.shields.io/crates/v/getimg.svg)](https://crates.io/crates/getimg)
[![docs](https://docs.rs/getimg/badge.svg)](https://docs.rs/getimg/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> 📸 GetImg: A CLI, SDK for interacting with the GetImg API, enabling image generation and manipulation through various endpoints.

## 📖 Table of Contents

- [Installation](#-installation)
- [Features](#-features)
- [Usage](#-usage)
- [Options](#-options)
- [Subcommands](#-subcommands)
- [Examples](#-examples)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install the `getimg` CLI, use the following Cargo command:

```sh
cargo install --locked getimg --all-features
```

## ✨ Features

- Interact with the GetImg API from the command line interface.
- Generate and manipulate images using various endpoints.
- Perform text-to-image and image-to-image generation.
- Repaint images or generate edited images based on prompts.
- Utilize ControlNet conditioning for image generation.

## Usage

Before using the `getimg` CLI, make sure to set the following environment variable:

```sh
export GETIMG_API_KEY=<your_getimg_api_key>
```

Generate an API key from the [GetImg Dashboard](https://dashboard.getimg.ai/api-keys).

## ⌨ Usage as CLI

### Generate an edited image:

```sh
getimg edit -p "A man riding a horse on Mars." -i image.jpg -s 25 -g 7.5 -e 25 -y 1.5 -o png -n "Disfigured, cartoon, blurry" -c ddim
```

### Repaint an image:

```sh
getimg paint -p "An image of a cityscape with neon lights." -i image.png -m edited_image.png -w 512 -a 512 -e 50 -s 5 -g 10.0 -o jpeg -c euler -f 1 -n "Disfigured, cartoon, blurry"
```

### Generate an image from text:

```sh
getimg t2i -p "A colorful sunset over the ocean." -w 512 -a 512 -s 5 -e 42 -o png -n "Disfigured, cartoon, blurry"
```

### Generate an image from another image:

```sh
getimg i2i -p "Add a forest in the background." -i generated_image.png -s 6 -e 512 -o jpeg -f 0.5 -n "Disfigured, cartoon, blurry"
```

### Generate images using ControlNet conditioning:

```sh
getimg cnet -p "A painting of a landscape." -i generated_image.png -f 1.0 -w 512 -a 512 -s 25 -g 7.5 -e 512 -c lms -o png -r canny-1.1 -n "Disfigured, cartoon, blurry"
```

## 🎨 Options

| Option                   | Description                                              |
|--------------------------|----------------------------------------------------------|
| `--api-key`              | Specify the API key for accessing the GetImg API.        |
| `--model`                | Specify the model to use for image generation.           |


## 🛠 Subcommands

| Subcommand              | Description                                              |
|-------------------------|----------------------------------------------------------|
| `edit`                  | Generate an edited image.                                |
| `paint`               | Repaint an image based on prompts.                       |
| `t2i`         | Generate an image from text.                             |
| `i2i`        | Generate an image from another image.                    |
| `cnet`            | Generate images using ControlNet conditioning.           |

## ✨ Usage as Crate

1. Add the `getimg` crate:

    ```toml
    [dependencies]
    getimg = "0.0.1"
    ```

1. Use the provided structs and methods to interact with the GetImg API in your Rust project.

    ```rust
    use anyhow::Result;
    use getimg::client::Client;
    use getimg::utils::save_image;
    use std::env;

    #[tokio::main]
    async fn main() -> Result<()> {
        // Example usage
        let api_key = env::var("GETIMG_API_KEY").unwrap_or_default().to_owned();
        let text_prompt = "a kanban-style task board with three columns: To Do, In Progress, and Done. Each column contains a list of tasks, with the To Do column having the most tasks and the Done column having the fewest. The tasks are color-coded, with the To Do tasks being red, the In Progress tasks being yellow, and the Done tasks being green. The board also has a header that includes the title \"To Do List\" and a search bar.";
        let negative_prompt = Some("Disfigured, cartoon, blurry");

        // Create a new instance of the GetIMG Client
        let mut client = Client::new(&api_key, "lcm-realistic-vision-v5-1");
        println!("{:?}", client);

        // Generate image from text prompt
        let text_response = client
            .generate_image_from_text(
                text_prompt,
                1024,
                1024,
                4,
                "jpeg",
                negative_prompt,
                Some(512),
            )
            .await?;

        // Save text response image to file
        save_image(&text_response.image, "text_response.jpg")?;

        // Generate image from image prompt
        let image_response = client
            .generate_image_from_image(
                "a photo of an astronaut riding a horse on mars",
                &text_response.image,
                4,
                512,
                "jpeg",
                negative_prompt,
                Some(0.5),
            )
            .await?;

        // Save image response image to file
        save_image(&image_response.image, "image_response.png")?;

        let controlnet_response = client
            .generate_image_using_controlnet(
                "softedge-1.1",
                "a photo of an astronaut riding a horse on mars",
                "Disfigured, cartoon, blurry",
                &image_response.image,
                1.0,
                512,
                512,
                25,
                7.5,
                512,
                "euler",
                "png",
            )
            .await?;

        save_image(&controlnet_response.image, "controlnet_response.png")?;

        // Generate repainted image
        let repainted_image_response = client
            .generate_repainted_image(
                text_prompt,
                negative_prompt,
                &text_response.image,
                &image_response.image,
                Some(1.0),
                512,
                512,
                25,
                7.5,
                512,
                "euler",
                "png",
            )
            .await?;

        // Save repainted image to file
        save_image(&repainted_image_response.image, "repainted_image.png")?;

        // Generate edited image
        let edited_image_response = client
            .generate_edited_image(
                text_prompt,
                negative_prompt,
                &repainted_image_response.image,
                1.5,
                25,
                7.5,
                54,
                "ddim",
                "png",
            )
            .await?;

        // Save edited image to file
        save_image(&edited_image_response.image, "edited_image.png")?;

        Ok(())
    }
    ```

## 📄 License

This project is licensed under the [MIT License](LICENSE).
