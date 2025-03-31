use anyhow::Result;
use uni_ocr::{OcrEngine, OcrProvider};

#[tokio::main]
async fn main() -> Result<()> {
    // Auto-detect best available provider
    let engine = OcrEngine::new(OcrProvider::Auto)?;

    // Perform OCR on a single image
    let (text, json, confidence) = engine.recognize_file("examples/sample.png").await?;
    println!("Text: {}", text);
    println!("Confidence: {:.2}%", confidence.unwrap_or(0.0));
    println!("JSON details: {}", json);

    Ok(())
}
