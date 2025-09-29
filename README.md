# mimetype-detector

A fast Rust library for detecting MIME types and file extensions from content, supporting 204 file formats.

## Features

- **204 supported formats** across all major categories
- **Fast hierarchical detection** with parent-child relationships
- **Thread-safe** with zero dependencies
- **Low memory usage** - reads only file headers (up to 3KB)
- **Type-safe constants** for all MIME types
- **WHATWG MIME Sniffing Standard** compliance

## Installation

```toml
[dependencies]
mimetype-detector = "0.2.3"
```

## Quick Start

```rust
use mimetype_detector::{detect, detect_file, constants::*};

// Detect from bytes
let data = b"\x89PNG\r\n\x1a\n";
let mime_type = detect(data);
println!("{} -> {}", mime_type.mime(), mime_type.extension());
// Output: image/png -> .png

if mime_type.is(IMAGE_PNG) {
    println!("PNG image detected!");
}

// Detect from file (accepts various path types)
let mime_type = detect_file("document.pdf")?;  // &str
// let mime_type = detect_file(path_buf)?;      // PathBuf
// let mime_type = detect_file(&path)?;         // &Path

```

## Advanced Usage

```rust
use mimetype_detector::{equals_any, match_mime, constants::*};

// Pattern matching
match mime_type.mime() {
    IMAGE_PNG => println!("PNG image"),
    APPLICATION_PDF => println!("PDF document"),
    _ => println!("Other: {}", mime_type.mime()),
}

// Bulk checking
let is_image = equals_any(mime_type.mime(), &[IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF]);

// Direct matching
let is_png = match_mime(data, IMAGE_PNG);
```

## Using the `is` Method

The `is` method provides flexible MIME type checking with both string and constant support:

```rust
use mimetype_detector::{detect, constants::*};

let data = b"\x89PNG\r\n\x1a\n";
let mime_type = detect(data);

// Using type-safe constants (recommended)
if mime_type.is(IMAGE_PNG) {
    println!("This is a PNG image");
}

// Using string literals (also works)
if mime_type.is("image/png") {
    println!("This is a PNG image");
}

// Check multiple formats
if mime_type.is(IMAGE_JPEG) {
    println!("JPEG image");
} else if mime_type.is(IMAGE_GIF) {
    println!("GIF image");
} else if mime_type.is(IMAGE_WEBP) {
    println!("WebP image");
}

// Practical examples for different categories
let file_data = std::fs::read("unknown_file")?;
let detected = detect(&file_data);

// Document checking
if detected.is(APPLICATION_PDF) {
    println!("PDF document - can open with PDF viewer");
} else if detected.is(APPLICATION_VND_OPENXML_WORDPROCESSINGML_DOCUMENT) {
    println!("Word document - .docx format");
} else if detected.is(TEXT_HTML) {
    println!("HTML document - can open in browser");
}

// Media file checking
if detected.is(AUDIO_MP3) {
    println!("MP3 audio file");
} else if detected.is(VIDEO_MP4) {
    println!("MP4 video file");
} else if detected.is(IMAGE_PNG) || detected.is(IMAGE_JPEG) {
    println!("Common image format");
}

// Archive checking
if detected.is(APPLICATION_ZIP) {
    println!("ZIP archive");
} else if detected.is(APPLICATION_X_7Z_COMPRESSED) {
    println!("7-Zip archive");
} else if detected.is(APPLICATION_X_TAR) {
    println!("TAR archive");
}

// Using with match for comprehensive handling
match detected.mime() {
    mime if detected.is(APPLICATION_PDF) => {
        println!("Processing PDF: {}", mime);
        // Handle PDF-specific logic
    },
    mime if detected.is(IMAGE_PNG) => {
        println!("Processing PNG: {}", mime);
        // Handle image processing
    },
    mime if detected.is(APPLICATION_JSON) => {
        println!("Processing JSON: {}", mime);
        // Handle JSON parsing
    },
    _ => {
        println!("Unknown or unsupported format: {}", detected.mime());
    }
}
```

## Supported Formats

### Major Categories

- **Images (31+)**: PNG, JPEG, GIF, WebP, AVIF, HEIC, SVG, TIFF, BMP, etc.
- **Audio (15)**: MP3, FLAC, WAV, AAC, OGG, MIDI, etc.
- **Video (14+)**: MP4, WebM, AVI, MKV, MOV, etc.
- **Archives (15+)**: ZIP, 7Z, TAR, RAR, GZIP, etc.
- **Documents (20+)**: PDF, DOCX, XLSX, PPTX, ODT, HTML, XML, etc.
- **Programming (33+)**: JavaScript, Python, PHP, JSON, CSV, etc.
- **Executables (15+)**: ELF, PE/EXE, Mach-O, WASM, etc.
- **Fonts (5)**: TTF, OTF, WOFF, WOFF2, etc.

### Smart Detection Features

- **Container formats**: ZIP files detected as DOCX/XLSX/EPUB/JAR/APK based on content
- **Office formats**: Both legacy (DOC/XLS/PPT) and modern (DOCX/XLSX/PPTX)
- **UTF encoding**: Automatic BOM detection for UTF-8/UTF-16
- **Binary analysis**: ELF subtypes (executables, libraries, core dumps)

## API Reference

### Core Functions

- `detect(data: &[u8]) -> &'static MimeType`
- `detect_file<P: AsRef<Path>>(path: P) -> io::Result<&'static MimeType>`
- `detect_reader<R: Read>(reader: R) -> io::Result<&'static MimeType>`

### Utilities

- `match_mime(data: &[u8], mime_type: &str) -> bool`
- `equals_any(mime_type: &str, types: &[&str]) -> bool`
- `is_supported(mime_type: &str) -> bool`

### MimeType Methods

- `mime() -> &'static str` - Get MIME type string
- `extension() -> &'static str` - Get file extension
- `is(expected: &str) -> bool` - Check if matches expected type

## Performance

- **Hierarchical detection** minimizes unnecessary checks
- **Lazy initialization** with static lifetime data
- **Memory efficient** - only reads file headers
- **Zero dependencies** - pure Rust implementation

## License

MIT License

## Credits

Rust port of the Go [mimetype](https://github.com/gabriel-vasile/mimetype) library with enhancements.
