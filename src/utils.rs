use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::fs::File;
use std::io::Read;
use std::io::Write;

/// Saves an image to a file.
///
/// # Arguments
///
/// * `image_data` - A base64-encoded string representing the image data.
/// * `filename` - A string slice representing the name of the file to save the image to.
///
/// # Returns
///
/// A `Result` indicating success or failure of the operation. Returns `Ok(())` if the image is saved successfully,
/// or an `std::io::Error` if there is an issue with file creation or writing.
pub fn save_image(image_data: &str, filename: &str) -> Result<()> {
    let decoded_image_data = STANDARD.decode(image_data)?;
    let mut file = File::create(filename)?;
    file.write_all(&decoded_image_data)?;
    println!("Image saved as: {}", filename);
    Ok(())
}

/// Load the image from the given path and encode it as a base64 string.
///
/// # Arguments
///
/// * `image_path` - A string slice representing the path to the image file.
///
/// # Returns
///
/// A `Result` containing the base64-encoded string on success, or an `std::io::Error` if the file cannot be read.
pub fn load_and_encode_image(image_path: &str) -> Result<String, std::io::Error> {
    // Open the image file
    let mut file = File::open(image_path)?;

    // Read the contents of the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Encode the buffer as a base64 string
    let base64_string = STANDARD.encode(&buffer);

    Ok(base64_string)
}
