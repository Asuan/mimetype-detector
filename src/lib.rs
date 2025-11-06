//! # mimetype-detector
//!
//! A comprehensive Rust library for detecting MIME types and file extensions based on magic numbers.
//! This library provides fast, accurate, and thread-safe MIME type detection for 450+ file formats
//! across all major categories.
//!
//! ## Features
//!
//! - **Fast and precise** MIME type detection using magic number analysis
//! - **450+ supported formats** including images, audio, video, documents, archives, and more
//! - **Thread-safe** operations with lazy initialization
//! - **Zero unsafe code** - built with RwLock and LazyLock for safety
//! - **Memory efficient** - reads only first 3KB of files
//! - **Zero dependencies** - pure Rust implementation
//!
//! ## Quick Start
//!
//! ```rust
//! use mimetype_detector::{detect, detect_file};
//!
//! // Detect from byte data
//! let data = b"\x89PNG\r\n\x1a\n";
//! let mime_type = detect(data);
//! println!("MIME type: {}", mime_type); // image/png
//! println!("Extension: {}", mime_type.extension()); // .png
//! println!("Kind: {}", mime_type.kind()); // IMAGE
//!
//! // Detect from file
//! # std::fs::write("test.png", b"\x89PNG\r\n\x1a\n").unwrap();
//! let mime_type = detect_file("test.png")?;
//! println!("File type: {}", mime_type);
//! # std::fs::remove_file("test.png").unwrap();
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## Architecture
//!
//! The library uses a hierarchical tree structure for MIME type detection:
//!
//! - **Tree-based detection**: Organizes formats by priority and relationships
//! - **Private matchers**: Detection algorithms are encapsulated and not exposed
//! - **Lazy initialization**: MIME type tree is built on first use
//! - **Static lifetime**: All MIME type data lives for the entire program duration
//!
//! ## Supported Formats
//!
//! - **Images**: PNG, JPEG, GIF, WebP, TIFF, BMP, SVG, DDS, PCX, KTX, ASTC, and 20+ others
//! - **Audio**: MP3, FLAC, WAV, OGG, AAC, WavPack, TTA, DSF, DFF, and 15+ others
//! - **Video**: MP4, WebM, AVI, MKV, MPEG, and 8+ others
//! - **Documents**: PDF, HTML, XML, Office formats, and more
//! - **Archives**: ZIP, TAR, 7Z, RAR, GZIP, LZ4, and 15+ others
//! - **Executables**: ELF, PE/EXE, Mach-O, Java CLASS, WASM
//! - **Fonts**: TTF, OTF, WOFF, WOFF2, EOT, TTC
//! - **3D/CAD**: Blender (.blend), PLY, GLB, GLTF, DWG, DXF
//! - **Network/Debug**: PCAP, PCAPNG for packet capture analysis
//! - **And many more**: See documentation for complete list
//!
//! ## Custom Matchers
//!
//! You can register custom detection functions for additional MIME types:
//!
//! ```rust
//! use mimetype_detector::{register_mime, match_mime};
//!
//! // Register a custom matcher
//! register_mime("application/x-custom", |data| {
//!     data.starts_with(b"CUSTOM")
//! });
//!
//! // Test the custom matcher
//! let data = b"CUSTOM file content";
//! assert!(match_mime(data, "application/x-custom"));
//! ```

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::{LazyLock, Once, RwLock};

pub mod mime_type;
pub use mime_type::MimeType;

pub mod kind;
pub use kind::MimeKind;

pub mod constants;
pub use constants::*;

#[macro_use]
mod macros;

mod tree;
use tree::ROOT;

static INIT: Once = Once::new();

fn ensure_init() {
    INIT.call_once(|| {
        tree::init_tree();
    });
}

const READ_LIMIT: usize = 3072;

/// Detects the MIME type of the given byte data.
///
/// This function examines the first 3072 bytes of the provided data
/// to determine its MIME type using magic number detection.
///
/// # Arguments
///
/// * `data` - A byte slice containing the data to analyze
///
/// # Returns
///
/// A reference to the detected MIME type
pub fn detect(data: &[u8]) -> &'static MimeType {
    ensure_init();
    let input = if data.len() > READ_LIMIT {
        &data[..READ_LIMIT]
    } else {
        data
    };
    ROOT.match_bytes(input)
}

/// Detects the MIME type by reading from a `Read` implementor.
///
/// Reads up to 3072 bytes from the reader and analyzes them
/// to determine the MIME type.
///
/// # Arguments
///
/// * `reader` - Any type implementing the `Read` trait
///
/// # Returns
///
/// A `Result` containing the detected MIME type or an I/O error
pub fn detect_reader<R: Read>(mut reader: R) -> io::Result<&'static MimeType> {
    let mut buffer: [u8; READ_LIMIT] = [0x0; READ_LIMIT];
    let n = reader.read(&mut buffer)?;
    Ok(detect(&buffer[..n]))
}

/// Detects the MIME type of a file at the given path.
///
/// Opens the file and reads its content to determine the MIME type.
///
/// # Arguments
///
/// * `path` - The file system path to the file to analyze (accepts &str, String, Path, PathBuf, etc.)
///
/// # Returns
///
/// A `Result` containing the detected MIME type or an I/O error
pub fn detect_file<P: AsRef<Path>>(path: P) -> io::Result<&'static MimeType> {
    let file = File::open(path)?;
    detect_reader(file)
}

/// Checks if a MIME type equals any of the provided types.
///
/// Normalizes all MIME types by removing parameters (everything after ';')
/// before comparison.
///
/// # Arguments
///
/// * `mime_type` - The MIME type to check
/// * `types` - A slice of MIME type strings to compare against
///
/// # Returns
///
/// `true` if the MIME type matches any in the list, `false` otherwise
pub fn equals_any(mime_type: &str, types: &[&str]) -> bool {
    let normalized = normalize_mime_type(mime_type);
    types.iter().any(|&t| normalized == normalize_mime_type(t))
}

fn normalize_mime_type(mime_type: &str) -> &str {
    mime_type.split(';').next().unwrap_or("").trim()
}

type MatcherVec = Vec<fn(&[u8]) -> bool>;

static MIME_REGISTRY: LazyLock<RwLock<HashMap<String, MatcherVec>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static EXT_REGISTRY: LazyLock<RwLock<HashMap<String, MatcherVec>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Registers a custom matcher function for a specific MIME type.
///
/// This allows extending the library with custom detection logic
/// for additional MIME types.
///
/// # Arguments
///
/// * `mime_type` - The MIME type string to register
/// * `matcher` - A function that takes byte data and returns true if it matches
pub fn register_mime(mime_type: &str, matcher: fn(&[u8]) -> bool) {
    MIME_REGISTRY
        .write()
        .expect("MIME registry lock poisoned")
        .entry(mime_type.to_string())
        .or_default()
        .push(matcher);
}

/// Registers a custom matcher function for a specific file extension.
///
/// This allows extending the library with custom detection logic
/// for additional file extensions.
///
/// # Arguments
///
/// * `extension` - The file extension to register
/// * `matcher` - A function that takes byte data and returns true if it matches
pub fn register_extension(extension: &str, matcher: fn(&[u8]) -> bool) {
    EXT_REGISTRY
        .write()
        .expect("Extension registry lock poisoned")
        .entry(extension.to_string())
        .or_default()
        .push(matcher);
}

/// Checks if a MIME type is supported by the library.
///
/// Returns true if the MIME type has registered matchers.
///
/// # Arguments
///
/// * `mime_type` - The MIME type to check for support
///
/// # Returns
///
/// `true` if the MIME type is supported, `false` otherwise
pub fn is_supported(mime_type: &str) -> bool {
    ensure_init();
    let normalized = normalize_mime_type(mime_type);
    MIME_REGISTRY
        .read()
        .expect("MIME registry lock poisoned")
        .contains_key(normalized)
}

/// Checks if the given data matches a specific MIME type.
///
/// Uses registered matchers to determine if the byte data
/// corresponds to the specified MIME type.
///
/// # Arguments
///
/// * `data` - The byte data to analyze
/// * `mime_type` - The MIME type to match against
///
/// # Returns
///
/// `true` if the data matches the MIME type, `false` otherwise
pub fn match_mime(data: &[u8], mime_type: &str) -> bool {
    ensure_init();
    let input = if data.len() > READ_LIMIT {
        &data[..READ_LIMIT]
    } else {
        data
    };

    let normalized = normalize_mime_type(mime_type);
    if let Some(matchers) = MIME_REGISTRY
        .read()
        .expect("MIME registry lock poisoned")
        .get(normalized)
    {
        return matchers.iter().any(|matcher| matcher(input));
    }
    false
}

/// Checks if data from a reader matches a specific MIME type.
///
/// Reads from the provided reader and checks if the data
/// matches the specified MIME type.
///
/// # Arguments
///
/// * `reader` - Any type implementing the `Read` trait
/// * `mime_type` - The MIME type to match against
///
/// # Returns
///
/// A `Result` containing `true` if the data matches, or an I/O error
pub fn match_reader<R: Read>(mut reader: R, mime_type: &str) -> io::Result<bool> {
    let mut buffer: [u8; READ_LIMIT] = [0x0; READ_LIMIT];
    let n = reader.read(&mut buffer)?;
    Ok(match_mime(&buffer[..n], mime_type))
}

/// Checks if a file matches a specific MIME type.
///
/// Opens the file at the given path and checks if its content
/// matches the specified MIME type.
///
/// # Arguments
///
/// * `path` - The file system path to the file (accepts &str, String, Path, PathBuf, etc.)
/// * `mime_type` - The MIME type to match against
///
/// # Returns
///
/// A `Result` containing `true` if the file matches, or an I/O error
pub fn match_file<P: AsRef<Path>>(path: P, mime_type: &str) -> io::Result<bool> {
    let file = File::open(path)?;
    match_reader(file, mime_type)
}

/// Checks if a file extension is supported by the library.
///
/// Returns true if the extension has registered matchers.
///
/// # Arguments
///
/// * `extension` - The file extension to check for support
///
/// # Returns
///
/// `true` if the extension is supported, `false` otherwise
pub fn is_supported_extension(extension: &str) -> bool {
    ensure_init();
    EXT_REGISTRY
        .read()
        .expect("Extension registry lock poisoned")
        .contains_key(extension)
}

/// Checks if the given data matches a specific file extension.
///
/// Uses registered matchers to determine if the byte data
/// corresponds to the specified file extension.
///
/// # Arguments
///
/// * `data` - The byte data to analyze
/// * `extension` - The file extension to match against
///
/// # Returns
///
/// `true` if the data matches the extension, `false` otherwise
pub fn match_extension(data: &[u8], extension: &str) -> bool {
    ensure_init();
    let input = if data.len() > READ_LIMIT {
        &data[..READ_LIMIT]
    } else {
        data
    };

    if let Some(matchers) = EXT_REGISTRY
        .read()
        .expect("Extension registry lock poisoned")
        .get(extension)
    {
        return matchers.iter().any(|matcher| matcher(input));
    }
    false
}

/// Checks if data from a reader matches a specific file extension.
///
/// Reads from the provided reader and checks if the data
/// matches the specified file extension.
///
/// # Arguments
///
/// * `reader` - Any type implementing the `Read` trait
/// * `extension` - The file extension to match against
///
/// # Returns
///
/// A `Result` containing `true` if the data matches, or an I/O error
pub fn match_reader_extension<R: Read>(mut reader: R, extension: &str) -> io::Result<bool> {
    let mut buffer: [u8; READ_LIMIT] = [0x0; READ_LIMIT];
    let n = reader.read(&mut buffer)?;
    Ok(match_extension(&buffer[..n], extension))
}

/// Checks if a file matches a specific file extension.
///
/// Opens the file at the given path and checks if its content
/// matches the specified file extension.
///
/// # Arguments
///
/// * `path` - The file system path to the file (accepts &str, String, Path, PathBuf, etc.)
/// * `extension` - The file extension to match against
///
/// # Returns
///
/// A `Result` containing `true` if the file matches, or an I/O error
pub fn match_file_extension<P: AsRef<Path>>(path: P, extension: &str) -> io::Result<bool> {
    let file = File::open(path)?;
    match_reader_extension(file, extension)
}
