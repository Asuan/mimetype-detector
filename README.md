# mimetype-detector

A comprehensive Rust library for detecting MIME types and extensions based on magic numbers, ported from the Go [mimetype](https://github.com/gabriel-vasile/mimetype) library with extensive enhancements.

## Features

- **Fast and precise** MIME type and file extension detection
- **100+ supported file formats** across all major categories
- **Hierarchical detection** structure for optimal performance
- **WHATWG MIME Sniffing Standard** compliance
- **Thread-safe** operations with lazy initialization
- **Zero dependencies** - pure Rust implementation
- **Low memory usage** - reads only file headers (up to 3KB)
- **Enhanced detection algorithms** with sophisticated format analysis

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mimetype-detector = "0.1.0"
```

### Basic Usage

```rust
use mimetype_detector::{detect, detect_file, detect_reader};
use std::fs::File;

// Detect from byte slice
let data = b"\x89PNG\r\n\x1a\n";
let mime_type = detect(data);
println!("MIME type: {}", mime_type);
println!("Extension: {}", mime_type.extension());

// Detect from file
let mime_type = detect_file("image.png")?;
println!("File type: {}", mime_type);

// Detect from reader
let file = File::open("document.pdf")?;
let mime_type = detect_reader(file)?;
println!("Document type: {}", mime_type);
```

### Advanced Usage

```rust
use mimetype_detector::{equals_any, match_mime, is_supported};

// Check if MIME type is one of several
let is_image = equals_any(
    "image/png",
    &["image/png", "image/jpeg", "image/gif"],
);

// Check if data matches specific MIME type
let png_data = b"\x89PNG\r\n\x1a\n";
let is_png = match_mime(png_data, "image/png");

// Check if MIME type is supported
let supported = is_supported("application/pdf");

// Check specific MIME type
if mime_type.is("image/png") {
    println!("This is a PNG image!");
}
```

## Supported Formats

The library supports **100+ file formats** across all major categories:

### **Images (25+ formats)**

- **Modern**: PNG, APNG, JPEG, JPEG XL (JXL), JPEG 2000 (JP2/JPX/JPM), JPEG XS (JXS), JPEG XR (JXR)
- **Traditional**: GIF, WebP, TIFF, BMP, ICO, ICNS
- **Professional**: PSD (Photoshop), XCF (GIMP), PAT/GBR (GIMP assets), HDR (Radiance), XPM
- **Next-gen**: BPG (Better Portable Graphics), HEIC/HEIF (Apple/ISO), AVIF support planned

### **Audio (15+ formats)**

- **Lossy**: MP3 (enhanced), AAC, AMR, QCP, MusePack
- **Lossless**: FLAC, APE (Monkey's Audio), WAV, AIFF, AU
- **Other**: MIDI (enhanced), OGG, VOC, M3U/M3U8 playlists, M4A

### **Video (10+ formats)**

- **Modern**: MP4 (enhanced), WebM (enhanced), MKV (enhanced), AVI (enhanced)
- **Streaming**: MPEG (enhanced), QuickTime, FLV, ASF/WMV
- **Specialized**: RMVB (RealMedia), M4V

### **Archives & Compression (15+ formats)**

- **Common**: ZIP, 7Z, TAR (enhanced), GZIP (enhanced), BZIP2, XZ
- **Advanced**: RAR, ZSTD (enhanced), LZIP, CPIO
- **Specialized**: XAR, DEB, WARC, CAB, InstallShield CAB, FITS

### **Documents & Office (10+ formats)**

- **Adobe**: PDF, PostScript
- **Microsoft**: DOC/DOCX, XLS/XLSX, PPT/PPTX (via OLE/ZIP detection)
- **Standards**: HTML (enhanced), XML (enhanced), RTF
- **eBooks**: EPUB, MOBI, LIT

### **Executables & Binary (8+ formats)**

- **Cross-platform**: ELF (Linux), PE/EXE (Windows), Mach-O (macOS)
- **Specialized**: Java CLASS, WebAssembly (WASM), Chrome CRX
- **Libraries**: AR archives, shared objects

### **Fonts (6+ formats)**

- **Web**: WOFF, WOFF2
- **Desktop**: TTF, OTF, TTC (collections)
- **Legacy**: EOT (Embedded OpenType)

### **Text & Markup (Enhanced detection)**

- **Encoding-aware**: UTF-8, UTF-16BE, UTF-16LE (with BOM detection)
- **Markup**: HTML (case-insensitive tags), XML (whitespace-tolerant)
- **Structured**: JSON, CSV, programming languages

### **Specialized Formats**

- **CAD**: DWG (15+ AutoCAD versions)
- **Medical**: DICOM (DCM)
- **Database**: SQLite, DBF
- **Graphics**: DJVU
- **Torrents**: BitTorrent files

## Performance & Architecture

### **Hierarchical Detection**

The library uses a sophisticated tree-based detection system that:

- **Minimizes checks** - Tests parent formats before children
- **Prioritizes common formats** - Frequent formats checked first  
- **Memory efficient** - Reads only 3KB file headers
- **Cache-friendly** - Lazy initialization with static lifetime data

### **Enhanced Algorithms**

- **Smart container analysis** - Proper MP4 ftyp box parsing, ZIP internal structure
- **Advanced text detection** - WHATWG-compliant binary vs text classification
- **Robust archive handling** - TAR checksum validation, proper EBML parsing for Matroska
- **Version-aware detection** - Supports multiple format versions (DWG, Office files)

## API

### Main Functions

- `detect(data: &[u8]) -> &'static MimeType` - Detect MIME type from byte slice
- `detect_file(path: &str) -> io::Result<&'static MimeType>` - Detect from file
- `detect_reader<R: Read>(reader: R) -> io::Result<&'static MimeType>` - Detect from reader

### Utility Functions

- `equals_any(mime_type: &str, types: &[&str]) -> bool` - Check if MIME type matches any in list
- `match_mime(data: &[u8], mime_type: &str) -> bool` - Check if data matches MIME type
- `match_reader<R: Read>(reader: R, mime_type: &str) -> io::Result<bool>` - Check reader against MIME type
- `match_file(path: &str, mime_type: &str) -> io::Result<bool>` - Check file against MIME type
- `is_supported(mime_type: &str) -> bool` - Check if MIME type is supported
- `is_supported_extension(extension: &str) -> bool` - Check if extension is supported
- `match_extension(data: &[u8], extension: &str) -> bool` - Check if data matches extension
- `match_file_extension(path: &str, extension: &str) -> io::Result<bool>` - Check file against extension

### MimeType Methods

- `mime() -> &'static str` - Get MIME type string
- `extension() -> &'static str` - Get file extension
- `is(expected: &str) -> bool` - Check if matches expected MIME type
- `parent() -> Option<&'static MimeType>` - Get parent MIME type in hierarchy

## Architecture

- **Private matchers**: Detection logic is encapsulated within the tree module
- **No unsafe code**: Thread-safe using RwLock and LazyLock
- **Zero dependencies**: Pure Rust with standard library only
- **Lazy initialization**: MIME types loaded on first use

## Advanced Features

### **WHATWG MIME Sniffing Compliance**

- Implements the official [WHATWG MIME Sniffing Standard](https://mimesniff.spec.whatwg.org/)
- Accurate binary vs text classification
- Support for UTF BOMs (Byte Order Marks)

## Why mimetype-detector?

### **Compared to other Rust libraries:**

- **More formats**: 100+ vs ~20-30 in alternatives
- **Better accuracy**: Sophisticated algorithms vs simple magic number matching
- **Standards compliance**: WHATWG MIME Sniffing vs ad-hoc detection
- **Zero dependencies**: Pure Rust

## Contributing

Contributions are welcome! Areas for improvement:

- Additional file format support
- Performance optimizations  
- Enhanced detection algorithms
- Better test coverage

## License

MIT License - see LICENSE file for details.

## Credits

This library is a comprehensive Rust port of the excellent Go [mimetype](https://github.com/gabriel-vasile/mimetype) library by Gabriel Vasile, with enhancements
