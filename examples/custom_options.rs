use anyhow::Result;
use std::time::Duration;
use uni_ocr::{Language, OcrEngine, OcrOptions, OcrProvider};

#[tokio::main]
async fn main() -> Result<()> {
    // Configure custom options
    let options = OcrOptions::default()
        .languages(vec![Language::English, Language::French])
        .confidence_threshold(0.8)
        .timeout(Duration::from_secs(30));

    let engine = OcrEngine::new(OcrProvider::Auto)?.with_options(options);
    
    let (text, _, confidence) = engine.recognize_file("examples/multilingual.png").await?;
    println!("Multilingual text: {}", text);
    println!("Confidence: {:.2}%", confidence.unwrap_or(0.0));

    Ok(())
} 