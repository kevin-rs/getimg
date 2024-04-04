use serde::Serialize;

/// Struct representing the request body for edited image generation endpoint.
///
/// This struct contains fields necessary for generating an edited image, such as the model name,
/// prompts, image data, guidance parameters, and output format.
#[derive(Debug, Serialize)]
pub struct EditImageRequest {
    /// Model name.
    pub model: String,
    /// Text prompt guiding the image editing process.
    pub prompt: String,
    /// Optional text prompt that contradicts the guidance for editing.
    pub negative_prompt: Option<String>,
    /// Base64 encoded image data.
    pub image: String,
    /// Guidance scale for the image editing process.
    pub image_guidance: f64,
    /// Number of steps in the editing process.
    pub steps: usize,
    /// Guidance scale for the editing process.
    pub guidance: f64,
    /// Seed for deterministic generation.
    pub seed: usize,
    /// Scheduler used in the editing process.
    pub scheduler: String,
    /// Output format of the generated image.
    pub output_format: String,
}

/// Struct representing the request body for repainted image generation endpoint.
///
/// This struct contains fields necessary for generating a repainted image, such as the model name,
/// prompts, image and mask data, strength, guidance parameters, and output format.
#[derive(Debug, Serialize)]
pub struct RepaintImageRequest {
    /// Model name.
    pub model: String,
    /// Text prompt guiding the image repainting process.
    pub prompt: String,
    /// Optional text prompt that contradicts the guidance for repainting.
    pub negative_prompt: Option<String>,
    /// Base64 encoded image data.
    pub image: String,
    /// Base64 encoded mask image data indicating areas to be repainted.
    pub mask_image: String,
    /// Strength of the repainting effect.
    pub strength: Option<f64>,
    /// Width of the generated image.
    pub width: usize,
    /// Height of the generated image.
    pub height: usize,
    /// Number of steps in the repainting process.
    pub steps: usize,
    /// Guidance scale for the repainting process.
    pub guidance: f64,
    /// Seed for deterministic generation.
    pub seed: usize,
    /// Scheduler used in the repainting process.
    pub scheduler: String,
    /// Output format of the generated image.
    pub output_format: String,
}

/// Struct representing the request body for text-to-image generation endpoint.
///
/// This struct contains fields necessary for generating an image from text input, such as the prompts,
/// model name, image dimensions, generation steps, output format, and seed.
#[derive(Debug, Serialize)]
pub struct TextToImageRequest {
    /// Text prompt guiding the image generation process.
    pub prompt: String,
    /// Model name.
    pub model: String,
    /// Optional text prompt that contradicts the guidance for image generation.
    pub negative_prompt: Option<String>,
    /// Width of the generated image.
    pub width: usize,
    /// Height of the generated image.
    pub height: usize,
    /// Number of steps in the generation process.
    pub steps: usize,
    /// Output format of the generated image.
    pub output_format: String,
    /// Seed for deterministic generation.
    pub seed: Option<usize>,
}

/// Struct representing the request body for image-to-image generation endpoint.
///
/// This struct contains fields necessary for generating an image from another image, such as the prompts,
/// model name, image data, strength, generation steps, output format, and seed.
#[derive(Debug, Serialize)]
pub struct ImageToImageRequest {
    /// Model name.
    pub model: String,
    /// Text prompt guiding the image generation process.
    pub prompt: String,
    /// Optional text prompt that contradicts the guidance for image generation.
    pub negative_prompt: Option<String>,
    /// Base64 encoded image data.
    pub image: String,
    /// Strength of the conditioning effect.
    pub strength: Option<f64>,
    /// Number of steps in the generation process.
    pub steps: usize,
    /// Output format of the generated image.
    pub output_format: String,
    /// Seed for deterministic generation.
    pub seed: Option<usize>,
}

/// Struct representing the request body for ControlNet generation endpoint.
///
/// This struct contains fields necessary for generating images using ControlNet conditioning,
/// such as the ControlNet type, model name, prompts, image data, strength, image dimensions,
/// generation steps, guidance parameters, scheduler, output format, and seed.
#[derive(Debug, Serialize)]
pub struct ControlNetRequest {
    /// Type of ControlNet conditioning.
    pub controlnet: String,
    /// Model name.
    pub model: String,
    /// Text prompt guiding the image generation process.
    pub prompt: String,
    /// Optional text prompt that contradicts the guidance for image generation.
    pub negative_prompt: Option<String>,
    /// Base64 encoded image data.
    pub image: String,
    /// Strength of the conditioning effect.
    pub strength: f64,
    /// Width of the generated image.
    pub width: usize,
    /// Height of the generated image.
    pub height: usize,
    /// Number of steps in the generation process.
    pub steps: usize,
    /// Guidance scale for the generation process.
    pub guidance: f64,
    /// Seed for deterministic generation.
    pub seed: usize,
    /// Scheduler used in the generation process.
    pub scheduler: String,
    /// Output format of the generated image.
    pub output_format: String,
}
