//! Public constants for all supported MIME types.
//!
//! This module provides public string constants for all MIME types supported by the library.
//! These constants can be used for comparisons, pattern matching, and configuration.
//!
//! # Usage
//!
//! ```rust
//! use mimetype_detector::{detect, constants::*};
//!
//! let data = b"\x89PNG\r\n\x1a\n";
//! let mime_type = detect(data);
//! 
//! if mime_type.is(IMAGE_PNG) {
//!     println!("Found a PNG image!");
//! }
//! ```
//!
//! # Categories
//!
//! Constants are organized by format category:
//! - **Text formats**: HTML, XML, UTF variants
//! - **Document formats**: PDF, PostScript, Office documents
//! - **Archive formats**: ZIP, TAR, 7Z, RAR, and others
//! - **Image formats**: PNG, JPEG, GIF, WebP, and many more
//! - **Audio formats**: MP3, FLAC, WAV, OGG, and others
//! - **Video formats**: MP4, WebM, AVI, MKV, and others
//! - **Executable formats**: ELF, PE/EXE, Java CLASS, WASM
//! - **Font formats**: TTF, OTF, WOFF, WOFF2
//! - **Web & multimedia**: SWF, CRX
//! - **Specialized formats**: DICOM, SQLite, CAD, eBooks

// ============================================================================
// TEXT FORMATS
// ============================================================================

/// HTML document with UTF-8 encoding
pub const TEXT_HTML: &str = "text/html; charset=utf-8";

/// XML document with UTF-8 encoding
pub const TEXT_XML: &str = "text/xml; charset=utf-8";

/// Plain text with UTF-8 encoding and BOM
pub const TEXT_UTF8_BOM: &str = "text/plain; charset=utf-8";

/// Plain text with UTF-16 Big Endian encoding
pub const TEXT_UTF16_BE: &str = "text/plain; charset=utf-16be";

/// Plain text with UTF-16 Little Endian encoding
pub const TEXT_UTF16_LE: &str = "text/plain; charset=utf-16le";

/// Plain text with UTF-8 encoding
pub const TEXT_UTF8: &str = "text/plain; charset=utf-8";

/// Generic plain text
pub const TEXT_PLAIN: &str = "text/plain";

// ============================================================================
// DOCUMENT FORMATS
// ============================================================================

/// Adobe Portable Document Format
pub const APPLICATION_PDF: &str = "application/pdf";

/// Alternative PDF MIME type
pub const APPLICATION_X_PDF: &str = "application/x-pdf";

/// PostScript document
pub const APPLICATION_POSTSCRIPT: &str = "application/postscript";

/// Microsoft OLE storage (legacy Office documents)
pub const APPLICATION_X_OLE_STORAGE: &str = "application/x-ole-storage";

// ============================================================================
// ARCHIVE & COMPRESSION FORMATS
// ============================================================================

/// 7-Zip archive
pub const APPLICATION_X_7Z_COMPRESSED: &str = "application/x-7z-compressed";

/// ZIP archive
pub const APPLICATION_ZIP: &str = "application/zip";

/// Alternative ZIP MIME types
pub const APPLICATION_X_ZIP: &str = "application/x-zip";
pub const APPLICATION_X_ZIP_COMPRESSED: &str = "application/x-zip-compressed";

/// RAR archive
pub const APPLICATION_X_RAR_COMPRESSED: &str = "application/x-rar-compressed";
pub const APPLICATION_X_RAR: &str = "application/x-rar";

/// GZIP compression
pub const APPLICATION_GZIP: &str = "application/gzip";
pub const APPLICATION_X_GZIP: &str = "application/x-gzip";
pub const APPLICATION_X_GUNZIP: &str = "application/x-gunzip";
pub const APPLICATION_GZIPPED: &str = "application/gzipped";
pub const APPLICATION_GZIP_COMPRESSED: &str = "application/gzip-compressed";
pub const APPLICATION_X_GZIP_COMPRESSED: &str = "application/x-gzip-compressed";
pub const GZIP_DOCUMENT: &str = "gzip/document";

/// TAR archive
pub const APPLICATION_X_TAR: &str = "application/x-tar";

/// BZIP2 compression
pub const APPLICATION_X_BZIP2: &str = "application/x-bzip2";

/// XZ compression
pub const APPLICATION_X_XZ: &str = "application/x-xz";

/// Zstandard compression
pub const APPLICATION_ZSTD: &str = "application/zstd";

/// LZIP compression
pub const APPLICATION_LZIP: &str = "application/lzip";

/// Microsoft Cabinet archive
pub const APPLICATION_VND_MS_CAB_COMPRESSED: &str = "application/vnd.ms-cab-compressed";

/// InstallShield Cabinet
pub const APPLICATION_X_INSTALLSHIELD: &str = "application/x-installshield";

/// CPIO archive
pub const APPLICATION_X_CPIO: &str = "application/x-cpio";

/// Unix AR archive
pub const APPLICATION_X_ARCHIVE: &str = "application/x-archive";
pub const APPLICATION_X_UNIX_ARCHIVE: &str = "application/x-unix-archive";

/// Red Hat Package Manager
pub const APPLICATION_X_RPM: &str = "application/x-rpm";

/// BitTorrent metadata
pub const APPLICATION_X_BITTORRENT: &str = "application/x-bittorrent";

/// FITS (Flexible Image Transport System)
pub const APPLICATION_FITS: &str = "application/fits";

/// XAR archive
pub const APPLICATION_X_XAR: &str = "application/x-xar";

/// Debian package
pub const APPLICATION_VND_DEBIAN_BINARY_PACKAGE: &str = "application/vnd.debian.binary-package";

/// Web ARChive format
pub const APPLICATION_WARC: &str = "application/warc";

// ============================================================================
// IMAGE FORMATS
// ============================================================================

/// Portable Network Graphics
pub const IMAGE_PNG: &str = "image/png";

/// Animated PNG
pub const IMAGE_VND_MOZILLA_APNG: &str = "image/vnd.mozilla.apng";

/// JPEG image
pub const IMAGE_JPEG: &str = "image/jpeg";

/// JPEG 2000
pub const IMAGE_JP2: &str = "image/jp2";

/// JPEG 2000 Extended
pub const IMAGE_JPX: &str = "image/jpx";

/// JPEG 2000 Multi-part
pub const IMAGE_JPM: &str = "image/jpm";

/// JPEG XS
pub const IMAGE_JXS: &str = "image/jxs";

/// JPEG XR
pub const IMAGE_JXR: &str = "image/jxr";

/// JPEG XL
pub const IMAGE_JXL: &str = "image/jxl";

/// Graphics Interchange Format
pub const IMAGE_GIF: &str = "image/gif";

/// WebP image format
pub const IMAGE_WEBP: &str = "image/webp";

/// Tagged Image File Format
pub const IMAGE_TIFF: &str = "image/tiff";

/// Windows Bitmap
pub const IMAGE_BMP: &str = "image/bmp";
pub const IMAGE_X_BMP: &str = "image/x-bmp";
pub const IMAGE_X_MS_BMP: &str = "image/x-ms-bmp";

/// Windows Icon
pub const IMAGE_X_ICON: &str = "image/x-icon";

/// Apple Icon Image
pub const IMAGE_X_ICNS: &str = "image/x-icns";

/// Adobe Photoshop Document
pub const IMAGE_VND_ADOBE_PHOTOSHOP: &str = "image/vnd.adobe.photoshop";
pub const IMAGE_X_PSD: &str = "image/x-psd";
pub const APPLICATION_PHOTOSHOP: &str = "application/photoshop";

/// High Efficiency Image Container
pub const IMAGE_HEIC: &str = "image/heic";

/// HEIC sequence
pub const IMAGE_HEIC_SEQUENCE: &str = "image/heic-sequence";

/// High Efficiency Image Format
pub const IMAGE_HEIF: &str = "image/heif";

/// HEIF sequence
pub const IMAGE_HEIF_SEQUENCE: &str = "image/heif-sequence";

/// Better Portable Graphics
pub const IMAGE_BPG: &str = "image/bpg";

/// GIMP native format
pub const IMAGE_X_XCF: &str = "image/x-xcf";

/// GIMP pattern
pub const IMAGE_X_GIMP_PAT: &str = "image/x-gimp-pat";

/// GIMP brush
pub const IMAGE_X_GIMP_GBR: &str = "image/x-gimp-gbr";

/// Radiance HDR
pub const IMAGE_VND_RADIANCE: &str = "image/vnd.radiance";

/// X11 Pixmap
pub const IMAGE_X_XPIXMAP: &str = "image/x-xpixmap";

/// AutoCAD Drawing
pub const IMAGE_VND_DWG: &str = "image/vnd.dwg";
pub const IMAGE_X_DWG: &str = "image/x-dwg";
pub const APPLICATION_ACAD: &str = "application/acad";
pub const APPLICATION_X_ACAD: &str = "application/x-acad";
pub const APPLICATION_AUTOCAD_DWG: &str = "application/autocad_dwg";
pub const APPLICATION_DWG: &str = "application/dwg";
pub const APPLICATION_X_DWG: &str = "application/x-dwg";
pub const APPLICATION_X_AUTOCAD: &str = "application/x-autocad";
pub const DRAWING_DWG: &str = "drawing/dwg";

/// DjVu document format
pub const IMAGE_VND_DJVU: &str = "image/vnd.djvu";

// ============================================================================
// AUDIO FORMATS
// ============================================================================

/// MPEG Audio Layer 3
pub const AUDIO_MPEG: &str = "audio/mpeg";
pub const AUDIO_X_MPEG: &str = "audio/x-mpeg";
pub const AUDIO_MP3: &str = "audio/mp3";

/// Free Lossless Audio Codec
pub const AUDIO_FLAC: &str = "audio/flac";

/// Waveform Audio File Format
pub const AUDIO_WAV: &str = "audio/wav";
pub const AUDIO_X_WAV: &str = "audio/x-wav";
pub const AUDIO_VND_WAVE: &str = "audio/vnd.wave";
pub const AUDIO_WAVE: &str = "audio/wave";

/// Audio Interchange File Format
pub const AUDIO_AIFF: &str = "audio/aiff";

/// Musical Instrument Digital Interface
pub const AUDIO_MIDI: &str = "audio/midi";
pub const AUDIO_MID: &str = "audio/mid";

/// Ogg container format
pub const APPLICATION_OGG: &str = "application/ogg";

/// Monkey's Audio
pub const AUDIO_APE: &str = "audio/ape";

/// Musepack
pub const AUDIO_MUSEPACK: &str = "audio/musepack";

/// Sun/NeXT Audio
pub const AUDIO_BASIC: &str = "audio/basic";

/// Adaptive Multi-Rate
pub const AUDIO_AMR: &str = "audio/amr";

/// Creative Voice File
pub const AUDIO_X_UNKNOWN: &str = "audio/x-unknown";

/// M3U playlist
pub const AUDIO_X_MPEGURL: &str = "audio/x-mpegurl";

/// Advanced Audio Coding
pub const AUDIO_AAC: &str = "audio/aac";

/// Qualcomm PureVoice
pub const AUDIO_QCELP: &str = "audio/qcelp";

/// MPEG-4 Audio
pub const AUDIO_MP4: &str = "audio/mp4";
pub const AUDIO_X_M4A: &str = "audio/x-m4a";
pub const AUDIO_X_MP4A: &str = "audio/x-mp4a";

// ============================================================================
// VIDEO FORMATS
// ============================================================================

/// MPEG-4 Part 14
pub const VIDEO_MP4: &str = "video/mp4";

/// WebM video format
pub const VIDEO_WEBM: &str = "video/webm";
pub const AUDIO_WEBM: &str = "audio/webm";

/// Matroska video
pub const VIDEO_X_MATROSKA: &str = "video/x-matroska";

/// Audio Video Interleave
pub const VIDEO_X_MSVIDEO: &str = "video/x-msvideo";
pub const VIDEO_AVI: &str = "video/avi";
pub const VIDEO_MSVIDEO: &str = "video/msvideo";

/// MPEG video
pub const VIDEO_MPEG: &str = "video/mpeg";

/// QuickTime movie
pub const VIDEO_QUICKTIME: &str = "video/quicktime";

/// Flash Video
pub const VIDEO_X_FLV: &str = "video/x-flv";

/// Advanced Systems Format
pub const VIDEO_X_MS_ASF: &str = "video/x-ms-asf";
pub const VIDEO_ASF: &str = "video/asf";
pub const VIDEO_X_MS_WMV: &str = "video/x-ms-wmv";

/// iTunes Video
pub const VIDEO_X_M4V: &str = "video/x-m4v";

/// RealMedia Variable Bitrate
pub const APPLICATION_VND_RN_REALMEDIA_VBR: &str = "application/vnd.rn-realmedia-vbr";

// ============================================================================
// EXECUTABLE & BINARY FORMATS
// ============================================================================

/// Windows Portable Executable
pub const APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE: &str = "application/vnd.microsoft.portable-executable";

/// Executable and Linkable Format
pub const APPLICATION_X_ELF: &str = "application/x-elf";

/// Java Class file
pub const APPLICATION_X_JAVA_APPLET: &str = "application/x-java-applet";
pub const APPLICATION_X_JAVA_APPLET_BINARY: &str = "application/x-java-applet; charset=binary";

/// WebAssembly binary
pub const APPLICATION_WASM: &str = "application/wasm";

// ============================================================================
// FONT FORMATS
// ============================================================================

/// TrueType Font
pub const FONT_TTF: &str = "font/ttf";
pub const FONT_SFNT: &str = "font/sfnt";
pub const APPLICATION_X_FONT_TTF: &str = "application/x-font-ttf";
pub const APPLICATION_FONT_SFNT: &str = "application/font-sfnt";

/// Web Open Font Format
pub const FONT_WOFF: &str = "font/woff";

/// Web Open Font Format 2
pub const FONT_WOFF2: &str = "font/woff2";

/// OpenType Font
pub const FONT_OTF: &str = "font/otf";

/// Embedded OpenType
pub const APPLICATION_VND_MS_FONTOBJECT: &str = "application/vnd.ms-fontobject";

/// TrueType Collection
pub const FONT_COLLECTION: &str = "font/collection";

// ============================================================================
// WEB & MULTIMEDIA FORMATS
// ============================================================================

/// Adobe Flash
pub const APPLICATION_X_SHOCKWAVE_FLASH: &str = "application/x-shockwave-flash";

/// Chrome Extension
pub const APPLICATION_X_CHROME_EXTENSION: &str = "application/x-chrome-extension";

// ============================================================================
// SPECIALIZED FORMATS
// ============================================================================

/// DICOM medical imaging
pub const APPLICATION_DICOM: &str = "application/dicom";

/// Mobipocket eBook
pub const APPLICATION_X_MOBIPOCKET_EBOOK: &str = "application/x-mobipocket-ebook";

/// Microsoft Reader eBook
pub const APPLICATION_X_MS_READER: &str = "application/x-ms-reader";

/// SQLite database
pub const APPLICATION_X_SQLITE3: &str = "application/x-sqlite3";

/// Fasoo document protection
pub const APPLICATION_X_FASOO: &str = "application/x-fasoo";

/// PGP NetShare
pub const APPLICATION_X_PGP_NET_SHARE: &str = "application/x-pgp-net-share";

/// Binary fallback
pub const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";