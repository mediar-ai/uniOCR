use anyhow::Result;
use uni_ocr::{OcrEngine, OcrProvider};

#[tokio::main]
async fn main() -> Result<()> {
    // Use platform-specific provider
    #[cfg(target_os = "macos")]
    let engine = OcrEngine::new(OcrProvider::MacOS)?;

    #[cfg(target_os = "windows")]
    let engine = OcrEngine::new(OcrProvider::Windows)?;

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let engine = OcrEngine::new(OcrProvider::Tesseract)?;

    let (text, _, confidence) = engine.recognize_file("examples/sample.png").await?;
    println!("Text: {}", text);
    println!("Confidence: {:.2}%", confidence.unwrap_or(0.0));

    Ok(())
}
