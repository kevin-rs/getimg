use serde::Deserialize;

/// Struct representing the response body for text-to-image and image-to-image generation endpoint.
///
/// This struct contains fields representing the generated image, seed used for generation (if applicable),
/// and the cost of generation (if applicable).
#[derive(Debug, Deserialize)]
pub struct ToImageResponse {
    /// The generated image data.
    pub image: String,
    /// The seed used for generation, if applicable.
    pub seed: Option<usize>,
    /// The cost of generation, if applicable.
    pub cost: Option<f64>,
}
