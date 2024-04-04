use anyhow::Result;
use reqwest::header;
use reqwest::Client as ReqClient;
use std::fmt;

use crate::request::{
    ControlNetRequest, EditImageRequest, ImageToImageRequest, RepaintImageRequest,
    TextToImageRequest,
};
use crate::response::ToImageResponse;

// Constants
pub(crate) const BASE_URL: &str = "https://api.getimg.ai/v1";

/// GetImg API client structure.
#[derive(Clone)]
pub struct Client {
    /// Reqwest client instance.
    pub client: ReqClient,

    /// API key for authentication.
    pub api_key: String,

    /// Model to be used.
    pub model: String,

    /// API URL for GetImg.
    pub api_url: &'static str,
}

impl Client {
    /// Creates a new instance of the GetImg Client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string representing the API key for authentication.
    /// * `model` - A string representing the model to be used.
    ///
    /// # Returns
    ///
    /// A new instance of the GetImg Client.
    ///
    /// # Panics
    ///
    /// Panics if there is an issue parsing the GetImg API URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use getimg::client::Client;
    ///
    /// let client = Client::new("your_api_key", "your_model");
    /// ```
    pub fn new(api_key: &str, model: &str) -> Self {
        Self {
            client: ReqClient::new(),
            api_key: api_key.to_owned(),
            model: model.to_owned(),
            api_url: BASE_URL,
        }
    }

    /// Generates an image based on a text prompt.
    ///
    /// # Arguments
    ///
    /// * `prompt` - A string representing the input text for content generation.
    /// * `width` - Width of the generated image.
    /// * `height` - Height of the generated image.
    /// * `steps` - The number of denoising steps.
    /// * `output_format` - File format of the output image.
    /// * `negative_prompt` - Text input that will not guide the image generation.
    /// * `seed` - Seed for making generation deterministic.
    ///
    /// # Returns
    ///
    /// A Result containing the generated content as a string or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use getimg::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let result = client.generate_image_from_text("Rusty crab on the beach", 512, 512, 4, "jpeg", None, Some(512)).await;
    ///     match result {
    ///         Ok(content) => println!("Generated Content: {:?}", content),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn generate_image_from_text(
        &mut self,
        prompt: &str,
        width: usize,
        height: usize,
        steps: usize,
        output_format: &str,
        negative_prompt: Option<&str>,
        seed: Option<usize>,
    ) -> Result<ToImageResponse> {
        let request_body = TextToImageRequest {
            prompt: prompt.to_string(),
            model: self.model.clone(),
            negative_prompt: negative_prompt.map(|s| s.to_string()),
            width,
            height,
            steps,
            output_format: output_format.to_string(),
            seed,
        };

        let response = self
            .client
            .post(format!("{}/latent-consistency/text-to-image", self.api_url))
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let result = response.json::<ToImageResponse>().await?;
        Ok(result)
    }

    /// Generates an image based on an image prompt.
    ///
    /// # Arguments
    ///
    /// * `prompt` - A string representing the input text for content generation.
    /// * `image_data` - A base64-encoded string representing the image data.
    /// * `negative_prompt` - Text input that will not guide the image generation.
    /// * `strength` - Indicates how much to transform the reference image.
    /// * `steps` - The number of denoising steps.
    /// * `seed` - Makes generation deterministic.
    /// * `output_format` - File format of the output image.
    ///
    /// # Returns
    ///
    /// A Result containing the generated content as a string or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use getimg::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let result = client.generate_image_from_image("a photo of an astronaut riding a crab on mars", "base64_encoded_image_data", 5, 512, "png",None, Some(0.5)).await;
    ///     match result {
    ///         Ok(content) => println!("Generated Content: {:?}", content),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn generate_image_from_image(
        &mut self,
        prompt: &str,
        image_data: &str,
        steps: usize,
        seed: usize,
        output_format: &str,
        negative_prompt: Option<&str>,
        strength: Option<f64>,
    ) -> Result<ToImageResponse> {
        let request_body = ImageToImageRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            negative_prompt: negative_prompt.map(|s| s.to_string()),
            image: image_data.to_string(),
            strength,
            steps,
            output_format: output_format.to_string(),
            seed: Some(seed),
        };

        let response = self
            .client
            .post(format!(
                "{}/latent-consistency/image-to-image",
                self.api_url
            ))
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let result = response.json::<ToImageResponse>().await?;
        Ok(result)
    }

    /// Generates an image using the ControlNet endpoint.
    ///
    /// # Arguments
    ///
    /// * `controlnet` - Type of ControlNet conditioning.
    /// * `prompt` - Text input required to guide the image generation.
    /// * `negative_prompt` - Text input that will not guide the image generation.
    /// * `image` - Base64 encoded image that will be used as the ControlNet input condition.
    /// * `strength` - Indicates the scale at which ControlNet conditioning is applied.
    /// * `width` - The width of the generated image in pixels.
    /// * `height` - The height of the generated image in pixels.
    /// * `steps` - The number of denoising steps.
    /// * `guidance` - Guidance scale as defined in Classifier-Free Diffusion Guidance.
    /// * `seed` - Makes generation deterministic.
    /// * `scheduler` - Scheduler used to denoise the encoded image latents.
    /// * `output_format` - File format of the output image.
    ///
    /// # Returns
    ///
    /// A Result containing the generated content as a string or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use getimg::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let result = client.generate_image_using_controlnet("softedge-1.1", "a photo of an astronaut riding a crab on mars", "Disfigured, cartoon, blurry", "base64_encoded_image_data", 1.0, 512, 512, 25, 7.5, 512, "euler", "png").await;
    ///     match result {
    ///         Ok(content) => println!("Generated Content: {:?}", content),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn generate_image_using_controlnet(
        &mut self,
        controlnet: &str,
        prompt: &str,
        negative_prompt: &str,
        image: &str,
        strength: f64,
        width: usize,
        height: usize,
        steps: usize,
        guidance: f64,
        seed: usize,
        scheduler: &str,
        output_format: &str,
    ) -> Result<ToImageResponse> {
        let request_body = ControlNetRequest {
            controlnet: controlnet.to_string(),
            model: "stable-diffusion-v1-5".to_string(),
            prompt: prompt.to_string(),
            negative_prompt: Some(negative_prompt.to_string()),
            image: image.to_string(),
            strength,
            width,
            height,
            steps,
            guidance,
            seed,
            scheduler: scheduler.to_string(),
            output_format: output_format.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/stable-diffusion/controlnet", self.api_url))
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let result = response.json::<ToImageResponse>().await?;
        Ok(result)
    }

    /// Generates a repainted image using the GetImg API.
    ///
    /// # Arguments
    ///
    /// * `prompt` - Text input that guides the image repainting process.
    /// * `negative_prompt` - Optional text input that contradicts the guidance for repainting.
    /// * `image_data` - Base64 encoded image data to be repainted.
    /// * `mask_image_data` - Base64 encoded mask image data indicating areas to be repainted.
    /// * `strength` - Strength of the repainting effect.
    /// * `width` - Width of the generated image.
    /// * `height` - Height of the generated image.
    /// * `steps` - Number of steps in the repainting process.
    /// * `guidance` - Guidance scale for the repainting process.
    /// * `seed` - Seed for deterministic generation.
    /// * `scheduler` - Scheduler used in the repainting process.
    /// * `output_format` - Output format of the generated image.
    ///
    /// # Returns
    ///
    /// A Result containing the repainted image response or an error if the request fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn generate_repainted_image(
        &mut self,
        prompt: &str,
        negative_prompt: Option<&str>,
        image_data: &str,
        mask_image_data: &str,
        strength: Option<f64>,
        width: usize,
        height: usize,
        steps: usize,
        guidance: f64,
        seed: usize,
        scheduler: &str,
        output_format: &str,
    ) -> Result<ToImageResponse> {
        let request_body = RepaintImageRequest {
            model: "stable-diffusion-v1-5-inpainting".to_string(),
            prompt: prompt.to_string(),
            negative_prompt: negative_prompt.map(|s| s.to_string()),
            image: image_data.to_string(),
            mask_image: mask_image_data.to_string(),
            strength,
            width,
            height,
            steps,
            guidance,
            seed,
            scheduler: scheduler.to_string(),
            output_format: output_format.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/stable-diffusion/inpaint", self.api_url))
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let result = response.json::<ToImageResponse>().await?;
        Ok(result)
    }

    /// Generates an edited image using the GetImg API.
    ///
    /// # Arguments
    ///
    /// * `prompt` - Text input guiding the image editing process.
    /// * `negative_prompt` - Optional text input that contradicts the guidance for editing.
    /// * `image_data` - Base64 encoded image data to be edited.
    /// * `image_guidance` - Guidance scale for the image editing process.
    /// * `steps` - Number of steps in the editing process.
    /// * `guidance` - Guidance scale for the editing process.
    /// * `seed` - Seed for deterministic generation.
    /// * `scheduler` - Scheduler used in the editing process.
    /// * `output_format` - Output format of the generated image.
    ///
    /// # Returns
    ///
    /// A Result containing the edited image response or an error if the request fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn generate_edited_image(
        &mut self,
        prompt: &str,
        negative_prompt: Option<&str>,
        image_data: &str,
        image_guidance: f64,
        steps: usize,
        guidance: f64,
        seed: usize,
        scheduler: &str,
        output_format: &str,
    ) -> Result<ToImageResponse> {
        let request_body = EditImageRequest {
            model: "instruct-pix2pix".to_string(),
            prompt: prompt.to_string(),
            negative_prompt: negative_prompt.map(|s| s.to_string()),
            image: image_data.to_string(),
            image_guidance,
            steps,
            guidance,
            seed,
            scheduler: scheduler.to_string(),
            output_format: output_format.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/stable-diffusion/instruct", self.api_url))
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let result = response.json::<ToImageResponse>().await?;
        Ok(result)
    }
}

/// Custom Debug trait implementation for Client struct.
///
/// This implementation hides the API key from being exposed in debug output.
impl fmt::Debug for Client {
    /// Formats the Client struct for debug output.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter used to write the output.
    ///
    /// # Returns
    ///
    /// A fmt::Result indicating success or failure of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("model", &self.model)
            .finish()
    }
}
