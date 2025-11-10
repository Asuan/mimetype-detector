# mimetype-detector

Fast MIME type detection for 450+ file formats with zero dependencies.

## Features

- **450+ supported formats** - Comprehensive coverage including images, audio, video, documents, archives, CAD, 3D models, and more
- **Fast & lightweight** - Reads only file headers (≤3KB)
- **Thread-safe** - Zero dependencies, pure Rust
- **Smart detection** - Hierarchical format relationships (ZIP→DOCX/JAR/APK, OLE→Office/CAD)
- **Type-safe constants** - Compile-time MIME type validation
- **Professional formats** - Adobe Creative Suite, Microsoft Office, CAD (SolidWorks, Inventor, 3DS Max), and design tools (Sketch, Figma)

## Installation

```toml
[dependencies]
mimetype-detector = "0.2.8"
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

## Supported Formats (450+)

### Common Formats

- **Images**: PNG, JPEG, GIF, WebP, AVIF, HEIC, HEIF, SVG, TIFF, BMP, ICO, PSD
- **Audio**: MP3, FLAC, WAV, AAC, OGG, MIDI, M4A, WMA, OPUS
- **Video**: MP4, WebM, AVI, MKV, MOV, FLV, WMV, 3GP, M4V
- **Archives**: ZIP, 7Z, TAR, RAR, GZIP, BZIP2, XZ, ZSTD, LZ4
- **Documents**: PDF, DOCX, XLSX, PPTX, ODT, ODS, ODP, RTF, EPUB

### Professional & Creative Tools

- **Adobe**: Photoshop (PSD), Illustrator (AI), InDesign (INDD, IDML), Flash (SWF, FLA)
- **Microsoft Office**: Word (DOC, DOCX), Excel (XLS, XLSX), PowerPoint (PPT, PPTX), Visio (VSD, VSDX), Publisher, OneNote, Project
- **CAD/3D**: SolidWorks (SLDASM, SLDDRW, SLDPRT), Autodesk Inventor (IAM, IDW, IPT), 3DS Max (MAX), AutoCAD (DWG, DXF), Blender, FBX, STL, STEP, IGES
- **Design Tools**: Sketch, Figma, draw.io

### Legacy & Office Formats

- **OpenOffice/LibreOffice**: ODT, ODS, ODP, ODF, ODB, and templates
- **StarOffice**: StarWriter, StarCalc, StarImpress, StarDraw (SDA, SDC, SDD, SDW)
- **Sun XML**: SXW, SXC, SXI, SXM, and templates (STC, STD, STI, STW)
- **WordPerfect**: WPD, WPG, WPS
- **Lotus**: 1-2-3 spreadsheets (WK1, WK3, WK4)

### Development & System

- **Programming**: JavaScript, Python, PHP, Ruby, Perl, Lua, Shell, Batch, LaTeX
- **Data**: JSON, XML, CSV, TSV, YAML, TOML
- **Executables**: ELF, PE/EXE/DLL, Mach-O, WASM, Java Class/JAR, Android APK/AAB
- **Fonts**: TTF, OTF, WOFF, WOFF2, EOT

### Specialized Formats

- **3D Models**: GLTF/GLB, USD/USDZ, COLLADA, FBX, Draco, VOX, IQM
- **Camera RAW**: CR2, CR3, NEF, RAF, ORF, RW2, DNG, ARW
- **eBooks**: EPUB, MOBI, FictionBook (FB2, FBZ), LIT, LRF
- **Scientific**: HDF4/HDF5, FITS, Parquet, DICOM
- **XML-Based**: RSS, Atom, SVG, KML, GPX, MathML, MusicXML, TTML, SOAP

See [SUPPORTED_FORMATS.md](SUPPORTED_FORMATS.md) for the complete list with MIME types and detection details.

### Smart Detection

- **Container awareness**: ZIP→DOCX/JAR/APK/EPUB, OLE→Office/CAD files
- **UTF encoding**: BOM detection for UTF-8/UTF-16 variants
- **Binary analysis**: ELF subtypes (executable/library/core), PE variants (EXE/DLL/SYS)
- **Content inspection**: XML namespaces, OLE metadata, ZIP manifest files

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
