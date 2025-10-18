use anyhow::Result;
pub use custom_ocr::Credentials;
use image::DynamicImage;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum OcrProvider {
    Auto,
    MacOS,
    Windows,
    Tesseract,
    Custom { credentials: Credentials },
}

#[derive(Debug, Clone)]
pub struct OcrOptions {
    languages: Vec<Language>,
    confidence_threshold: f32,
    timeout: Duration,
}

impl Default for OcrOptions {
    fn default() -> Self {
        Self {
            languages: vec![Language::English],
            confidence_threshold: 0.0,
            timeout: Duration::from_secs(30),
        }
    }
}

impl OcrOptions {
    pub fn languages(mut self, langs: Vec<Language>) -> Self {
        self.languages = langs;
        self
    }

    pub fn confidence_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

pub struct OcrEngine {
    provider: OcrProvider,
    options: OcrOptions,
}

impl OcrEngine {
    pub fn new(provider: OcrProvider) -> Result<Self> {
        Ok(Self {
            provider,
            options: OcrOptions::default(),
        })
    }

    pub fn with_options(mut self, options: OcrOptions) -> Self {
        self.options = options;
        self
    }

    pub async fn recognize_image(
        &self,
        image: &DynamicImage,
    ) -> Result<(String, String, Option<f64>)> {
        match &self.provider {
            OcrProvider::MacOS => {
                #[cfg(target_os = "macos")]
                {
                    Ok(perform_ocr_apple(image, &self.options.languages))
                }
                #[cfg(not(target_os = "macos"))]
                {
                    Err(anyhow::anyhow!(
                        "macOS OCR is not available on this platform"
                    ))
                }
            }
            OcrProvider::Windows => {
                #[cfg(target_os = "windows")]
                {
                    perform_ocr_windows(image).await
                }
                #[cfg(not(target_os = "windows"))]
                {
                    Err(anyhow::anyhow!(
                        "Windows OCR is not available on this platform"
                    ))
                }
            }
            OcrProvider::Tesseract => {
                Ok(perform_ocr_tesseract(image, self.options.languages.clone()))
            }
            OcrProvider::Custom { credentials } => {
                perform_ocr_custom(image, self.options.languages.clone(), &credentials).await
            }
            OcrProvider::Auto => {
                #[cfg(target_os = "macos")]
                {
                    Ok(perform_ocr_apple(image, &self.options.languages))
                }
                #[cfg(target_os = "windows")]
                {
                    perform_ocr_windows(image).await
                }
                #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                {
                    Ok(perform_ocr_tesseract(image, self.options.languages.clone()))
                }
            }
        }
    }

    pub async fn recognize_file(&self, path: &str) -> Result<(String, String, Option<f64>)> {
        let img = image::open(path)?;
        self.recognize_image(&img).await
    }

    pub async fn recognize_batch(
        &self,
        paths: Vec<&str>,
    ) -> Result<Vec<(String, String, Option<f64>)>> {
        let mut results = Vec::with_capacity(paths.len());
        for path in paths {
            results.push(self.recognize_file(path).await?);
        }
        Ok(results)
    }
}

#[cfg(target_os = "macos")]
pub mod apple;
pub mod custom_ocr;
pub mod language;
#[cfg(target_os = "windows")]
pub mod microsoft;
pub mod tesseract;

#[cfg(target_os = "macos")]
pub use apple::perform_ocr_apple;
pub use custom_ocr::perform_ocr_custom;
pub use language::*;
#[cfg(target_os = "windows")]
pub use microsoft::perform_ocr_windows;
pub use tesseract::perform_ocr_tesseract;
