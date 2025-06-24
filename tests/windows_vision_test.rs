#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use std::{path::PathBuf, time::Instant};
    use uni_ocr::{OcrEngine, OcrProvider};

    #[tokio::test]
    async fn test_ocr_windows() {
        // Use an absolute path that works in both local and CI environments
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("testing_OCR.png");
        println!("Path to testing_OCR.png: {:?}", path);
        let image = image::open(&path).expect("Failed to open image");

        let engine = OcrEngine::new(OcrProvider::Windows).expect("Failed to create OCR engine");

        let (text, _, confidence) = engine.recognize_image(&image).await.expect("OCR failed");

        assert!(confidence.is_some());
        assert!(!text.is_empty());
        // Add more specific assertions based on expected behavior
        println!("Recognized text: {}", text);
    }
}
