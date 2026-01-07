use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageReader};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

/// Upload result
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResult {
    /// Storage key for the file
    pub key: String,
    /// Whether the file already existed (instant upload)
    pub exists: bool,
    /// Compressed size in bytes
    pub size: usize,
}

/// Image processing options
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessOptions {
    /// Maximum width or height (default: 2560)
    #[serde(default = "default_max_dimension")]
    pub max_dimension: u32,
    /// JPEG quality 1-100 (default: 88)
    #[serde(default = "default_quality")]
    pub quality: u8,
}

fn default_max_dimension() -> u32 {
    2560
}
fn default_quality() -> u8 {
    88
}

impl Default for ProcessOptions {
    fn default() -> Self {
        Self {
            max_dimension: default_max_dimension(),
            quality: default_quality(),
        }
    }
}

/// API response wrapper
#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    code: i32,
    #[allow(dead_code)]
    message: String,
    data: T,
}

/// Presign request to server
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PresignRequest {
    filename: String,
    content_type: String,
    hash: String,
}

/// Presign response from server
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PresignResponse {
    upload_url: Option<String>,
    key: String,
    exists: bool,
}

/// Upload image: compress, hash, presign, upload to OSS
#[tauri::command]
pub async fn upload_image(
    image_data: Vec<u8>,
    api_url: String,
    token: String,
    options: Option<ProcessOptions>,
) -> Result<UploadResult, String> {
    let opts = options.unwrap_or_default();

    // Process image (compress + hash)
    let (jpeg_data, hash) = process_image_internal(&image_data, &opts)?;
    let size = jpeg_data.len();

    // Call presign API
    let client = Client::new();
    let presign_url = format!("{}/api/upload/presign", api_url);

    let presign_resp = client
        .post(&presign_url)
        .bearer_auth(&token)
        .json(&PresignRequest {
            filename: "image.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            hash,
        })
        .send()
        .await
        .map_err(|e| format!("Failed to call presign API: {}", e))?;

    if !presign_resp.status().is_success() {
        return Err(format!(
            "Presign API failed with status: {}",
            presign_resp.status()
        ));
    }

    let api_resp: ApiResponse<PresignResponse> = presign_resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse presign response: {}", e))?;

    if api_resp.code != 0 {
        return Err(format!("Presign API error: {}", api_resp.message));
    }

    let presign = api_resp.data;

    // If file already exists, instant upload
    if presign.exists {
        return Ok(UploadResult {
            key: presign.key,
            exists: true,
            size,
        });
    }

    // Upload to OSS
    let upload_url = presign
        .upload_url
        .ok_or("Missing upload URL in presign response")?;

    let upload_resp = client
        .put(&upload_url)
        .header("Content-Type", "image/jpeg")
        .body(jpeg_data)
        .send()
        .await
        .map_err(|e| format!("Failed to upload to OSS: {}", e))?;

    if !upload_resp.status().is_success() {
        return Err(format!(
            "OSS upload failed with status: {}",
            upload_resp.status()
        ));
    }

    Ok(UploadResult {
        key: presign.key,
        exists: false,
        size,
    })
}

/// Internal function to process image, returns (jpeg_data, hash)
fn process_image_internal(
    image_data: &[u8],
    opts: &ProcessOptions,
) -> Result<(Vec<u8>, String), String> {
    // Decode image
    let reader = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()
        .map_err(|e| format!("Failed to read image: {}", e))?;

    let img = reader
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Resize if needed (preserve aspect ratio)
    let img = resize_if_needed(img, opts.max_dimension);

    // Encode to JPEG
    let mut jpeg_data = Vec::new();
    {
        let mut encoder = JpegEncoder::new_with_quality(&mut jpeg_data, opts.quality);
        encoder
            .encode_image(&img)
            .map_err(|e| format!("Failed to encode JPEG: {}", e))?;
    }

    // Calculate blake3 hash (hex string, 64 chars)
    let hash = blake3::hash(&jpeg_data).to_string();

    Ok((jpeg_data, hash))
}

fn resize_if_needed(img: DynamicImage, max_dimension: u32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let max_side = w.max(h);

    if max_side <= max_dimension {
        return img;
    }

    let scale = max_dimension as f64 / max_side as f64;
    let new_w = (w as f64 * scale).round() as u32;
    let new_h = (h as f64 * scale).round() as u32;

    img.resize(new_w, new_h, FilterType::Lanczos3)
}
