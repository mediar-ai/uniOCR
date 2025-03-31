use anyhow::Result;
use image::DynamicImage;
use uni_ocr::{OcrEngine, OcrProvider};
use xcap::Monitor;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize OCR engine
    let engine = OcrEngine::new(OcrProvider::Auto)?;

    // Get all monitors
    let monitors = Monitor::all()?;

    for (i, monitor) in monitors.iter().enumerate() {
        println!("capturing monitor {}", i);

        // Capture screen
        let image = monitor.capture_image()?;

        // Convert xcap image to DynamicImage
        let dynamic_image = DynamicImage::ImageRgba8(image);

        // Perform OCR
        let (text, _, confidence) = engine.recognize_image(&dynamic_image).await?;

        println!("monitor {}: ", i);
        println!("text: {}", text);
        println!("confidence: {:.2}%", confidence.unwrap_or(0.0));
        println!("---");
    }

    Ok(())
}
