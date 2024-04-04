use anyhow::Result;
use getimg::client::Client;
use getimg::utils::save_image;
use std::env;

#[tokio::test]
async fn test_client() -> Result<()> {
    // Example usage
    let api_key = env::var("GETIMG_API_KEY").unwrap_or_default().to_owned();
    let text_prompt = "a kanban-style task board with three columns: To Do, In Progress, and Done. Each column contains a list of tasks, with the To Do column having the most tasks and the Done column having the fewest. The tasks are color-coded, with the To Do tasks being red, the In Progress tasks being yellow, and the Done tasks being green. The board also has a header that includes the title \"To Do List\" and a search bar.";
    let negative_prompt = Some("Disfigured, cartoon, blurry");

    // Create a new instance of the Gemini Client
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
