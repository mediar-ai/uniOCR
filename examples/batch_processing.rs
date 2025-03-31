use anyhow::Result;
use uni_ocr::{OcrEngine, OcrProvider};

#[tokio::main]
async fn main() -> Result<()> {
    let engine = OcrEngine::new(OcrProvider::Auto)?;

    let images = vec![
        "examples/sample1.png",
        "examples/sample2.png",
        "examples/sample3.png",
    ];

    let results = engine.recognize_batch(images).await?;

    for (i, (text, _, confidence)) in results.iter().enumerate() {
        println!("Image {}: ", i + 1);
        println!("Text: {}", text);
        println!("Confidence: {:.2}%", confidence.unwrap_or(0.0));
        println!("---");
    }

    Ok(())
}
