# uniocr 📸

[![MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)

universal ocr engine for rust that works everywhere. supports native ocr on macos, windows, tesseract, and cloud providers.

need a feature like NodeJS, HTTP example, etc.? open an issue or PR.

### features 🚀

- **native ocr**
  - macos: native vision kit api
  - windows: windows ocr engine
- **tesseract integration**
  - full support for tesseract with custom models
  - fast initialization and caching
- **cloud providers**
  - custom ocr provider
- **unified api**
  - single interface for all providers
  - easy provider switching
  - batch processing support
- **performance focused**
  - async/await support
  - parallel processing
  - memory efficient
  - unsafe code memory leaks battle tested

### quickstart 🏃

```toml
[dependencies]
uni-ocr = { git = "https://github.com/mediar-ai/uniocr.git" }
```

```rust
use uniocr::{OcrEngine, OcrProvider};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // auto-detect best available provider
    let engine = OcrEngine::new(OcrProvider::Auto)?;
    
    // perform ocr on an image
    let text = engine.recognize_file("path/to/image.png").await?;
    println!("extracted text: {}", text);
    
    Ok(())
}
```

### providers 🔌

```rust
// use native macos vision
let engine = OcrEngine::new(OcrProvider::MacOS)?;

// use windows ocr
let engine = OcrEngine::new(OcrProvider::Windows)?;

// use tesseract
let engine = OcrEngine::new(OcrProvider::Tesseract)?;

// use google cloud vision
// let engine = OcrEngine::new(OcrProvider::GoogleCloud {
//     credentials: ...,
// })?;
```

### advanced usage 🛠️

```rust
use uniocr::{OcrEngine, OcrProvider, OcrOptions};

// configure ocr options
let options = OcrOptions::default()
    .languages(vec!["eng", "fra"])
    .confidence_threshold(0.8)
    .timeout(std::time::Duration::from_secs(30));

let engine = OcrEngine::new(OcrProvider::Auto)?
    .with_options(options);

// batch processing
let images = vec!["img1.png", "img2.png", "img3.png"];
let results = engine.recognize_batch(images).await?;
```

### installation requirements 🔧

- **macos**: no additional setup (vision kit included)
- **windows**: windows 10+ with ocr capabilities
- **tesseract**: `tesseract-ocr` installed:
  ```bash
  # macos
  brew install tesseract
  
  # ubuntu
  apt-get install tesseract-ocr
  
  # windows
  winget install tesseract
  ```

### performance 📊

benchmark results on m4 macbook pro max (images/second):

| provider      | speed  | accuracy |
|--------------|--------|----------|
| macos vision | 3.2    | 90.0%    |
| windows ocr  | 1.2   | 95.2%    |
| tesseract    | tbd    | tbd    |
| google cloud | tbd   | tbd    |


### contributing 🤝

contributions welcome! 

### license 📜

this project is licensed under either of:

- apache license, version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- mit license ([LICENSE-MIT](LICENSE-MIT))

at your option.

### acknowledgments 🙏

- apple vision team
- microsoft windows ocr team
- tesseract ocr project
- cloud provider teams 

### examples 📚

the repository includes several example programs demonstrating different use cases:

#### run examples

```bash
# basic example
cargo run --example basic

# batch processing
cargo run --example batch_processing

# custom options
cargo run --example custom_options

# platform specific
cargo run --example platform_specific
```

check the [examples](examples/) directory for more detailed examples including:
- batch processing multiple images
- configuring custom options
- using platform-specific providers
- handling multilingual text 

