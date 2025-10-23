# mimetype-detector

Fast MIME type detection for 206+ file formats with zero dependencies.

## Features

- **206 supported formats** - Images, audio, video, documents, archives, and more
- **Fast & lightweight** - Reads only file headers (≤3KB)
- **Thread-safe** - Zero dependencies, pure Rust
- **Smart detection** - Hierarchical format relationships (ZIP→DOCX/JAR/APK)
- **Type-safe constants** - Compile-time MIME type validation

## Installation

```toml
[dependencies]
mimetype-detector = "0.2.5"
```

## Usage

```rust
use mimetype_detector::{detect, detect_file, constants::*};

// From bytes
let data = b"\x89PNG\r\n\x1a\n";
let mime = detect(data);
assert_eq!(mime.mime(), IMAGE_PNG);
assert_eq!(mime.extension(), ".png");

// From file
let mime = detect_file("document.pdf")?;
if mime.is(APPLICATION_PDF) {
    println!("PDF detected!");
}

// Pattern matching
match mime.mime() {
    IMAGE_PNG | IMAGE_JPEG => println!("Image"),
    APPLICATION_PDF => println!("PDF"),
    _ => println!("Other: {}", mime.mime()),
}

// Navigate type hierarchy
let docx = detect_file("document.docx")?;
println!("Type: {}", docx.mime());  // application/vnd.openxmlformats-officedocument.wordprocessingml.document
if let Some(parent) = docx.parent() {
    println!("Parent: {}", parent.mime());  // application/zip
}

// Check type categories using MimeKind
let png_data = b"\x89PNG\r\n\x1a\n";
let mime = detect(png_data);
if mime.kind().is_image() {
    println!("It's an image!");
}

// Multiple kinds display with pipe separator
let jar = detect(b"PK\x03\x04...META-INF/MANIFEST.MF");
println!("Kind: {}", jar.kind()); // Output: "ARCHIVE | APPLICATION"
```

## Supported Formats (206+)

- **Images**: PNG, JPEG, GIF, WebP, AVIF, HEIC, SVG, TIFF, BMP
- **Audio**: MP3, FLAC, WAV, AAC, OGG, MIDI
- **Video**: MP4, WebM, AVI, MKV, MOV
- **Archives**: ZIP, 7Z, TAR, RAR, GZIP
- **Documents**: PDF, DOCX, XLSX, PPTX, ODT, HTML, XML
- **Programming**: JavaScript, Python, PHP, JSON, CSV
- **Executables**: ELF, PE/EXE, Mach-O, WASM
- **Fonts**: TTF, OTF, WOFF, WOFF2

### Smart Detection

- Container awareness (ZIP→DOCX/JAR/APK)
- UTF encoding with BOM detection
- Binary analysis (ELF subtypes)

## API

```rust
// Core detection
detect(data: &[u8]) -> &'static MimeType
detect_file<P: AsRef<Path>>(path: P) -> io::Result<&'static MimeType>
detect_reader<R: Read>(reader: R) -> io::Result<&'static MimeType>

// MimeType methods
mime() -> &'static str                      // Get MIME type
extension() -> &'static str                 // Get extension
is(expected: &str) -> bool                  // Check type
parent() -> Option<&'static MimeType>       // Get parent type
kind() -> MimeKind                          // Get type category bitmask

// MimeKind methods (call on mime.kind())
is_image/video/audio/archive/document/...() // Category checks
contains(kind: MimeKind) -> bool            // Check if contains kind

// Utilities
match_mime(data: &[u8], mime: &str) -> bool
equals_any(mime: &str, types: &[&str]) -> bool
is_supported(mime: &str) -> bool
```

## License

MIT

## Credits

Based on Go [mimetype](https://github.com/gabriel-vasile/mimetype) library.
