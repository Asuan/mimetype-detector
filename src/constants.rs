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

/// HTML document with UTF-16 encoding
pub const TEXT_HTML_UTF16: &str = "text/html; charset=utf-16";

/// XML document with UTF-8 encoding
pub const TEXT_XML: &str = "text/xml; charset=utf-8";

/// XML document with UTF-16 encoding
pub const TEXT_XML_UTF16: &str = "text/xml; charset=utf-16";

/// Alternative XML MIME type
pub const APPLICATION_XML: &str = "application/xml";

/// Alternative XML MIME type with UTF-16 encoding
pub const APPLICATION_XML_UTF16: &str = "application/xml; charset=utf-16";

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

/// Forms Data Format (PDF variant)
pub const APPLICATION_VND_FDF: &str = "application/vnd.fdf";

/// PostScript document
pub const APPLICATION_POSTSCRIPT: &str = "application/postscript";

/// Microsoft OLE storage (legacy Office documents)
pub const APPLICATION_X_OLE_STORAGE: &str = "application/x-ole-storage";

/// Advanced Authoring Format
pub const APPLICATION_X_AAF: &str = APPLICATION_OCTET_STREAM;

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

/// LZIP compression (alias)
pub const APPLICATION_X_LZIP: &str = "application/x-lzip";

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

/// FITS (alias)
pub const IMAGE_FITS: &str = "image/fits";

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

/// JPEG 2000 Multi-part (alias)
pub const VIDEO_JPM: &str = "video/jpm";

/// JPEG XS
pub const IMAGE_JXS: &str = "image/jxs";

/// JPEG XR
pub const IMAGE_JXR: &str = "image/jxr";

/// JPEG XR (alias)
pub const IMAGE_VND_MS_PHOTO: &str = "image/vnd.ms-photo";

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

/// Portable Bitmap (Netpbm)
pub const IMAGE_X_PORTABLE_BITMAP: &str = "image/x-portable-bitmap";

/// Portable Graymap (Netpbm)
pub const IMAGE_X_PORTABLE_GRAYMAP: &str = "image/x-portable-graymap";

/// Portable Pixmap (Netpbm)
pub const IMAGE_X_PORTABLE_PIXMAP: &str = "image/x-portable-pixmap";

/// Portable Arbitrary Map (Netpbm)
pub const IMAGE_X_PORTABLE_ARBITRARYMAP: &str = "image/x-portable-arbitrarymap";

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

/// AutoCAD Drawing Exchange Format (DXF)
pub const IMAGE_VND_DXF: &str = "image/vnd.dxf";

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

/// OGG Audio
pub const AUDIO_OGG: &str = "audio/ogg";

/// OGG Video
pub const VIDEO_OGG: &str = "video/ogg";

/// Monkey's Audio
pub const AUDIO_APE: &str = "audio/ape";

/// Musepack
pub const AUDIO_MUSEPACK: &str = "audio/musepack";

/// Sun/NeXT Audio
pub const AUDIO_BASIC: &str = "audio/basic";

/// Adaptive Multi-Rate
pub const AUDIO_AMR: &str = "audio/amr";

/// Adaptive Multi-Rate (alias)
pub const AUDIO_AMR_NB: &str = "audio/amr-nb";

/// Creative Voice File
pub const AUDIO_X_UNKNOWN: &str = "audio/x-unknown";

/// M3U playlist
pub const AUDIO_X_MPEGURL: &str = "audio/x-mpegurl";

/// M3U playlist (alias)
pub const AUDIO_MPEGURL: &str = "audio/mpegurl";

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
pub const APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE: &str =
    "application/vnd.microsoft.portable-executable";

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

/// PKCS#7 Signature
pub const APPLICATION_PKCS7_SIGNATURE: &str = "application/pkcs7-signature";

// ============================================================================
// SPECIALIZED FORMATS
// ============================================================================

/// DICOM medical imaging
pub const APPLICATION_DICOM: &str = "application/dicom";

/// Mobipocket eBook
pub const APPLICATION_X_MOBIPOCKET_EBOOK: &str = "application/x-mobipocket-ebook";

/// Fasoo document protection
pub const APPLICATION_X_FASOO: &str = "application/x-fasoo";

/// PGP NetShare
pub const APPLICATION_X_PGP_NET_SHARE: &str = "application/x-pgp-net-share";

// ============================================================================
// MICROSOFT OFFICE & DOCUMENT FORMATS
// ============================================================================

/// Microsoft Word 2007+ Document
pub const APPLICATION_VND_OPENXML_WORDPROCESSINGML_DOCUMENT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";

/// Microsoft Excel 2007+ Spreadsheet
pub const APPLICATION_VND_OPENXML_SPREADSHEETML_SHEET: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

/// Microsoft PowerPoint 2007+ Presentation
pub const APPLICATION_VND_OPENXML_PRESENTATIONML_PRESENTATION: &str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";

/// Microsoft Visio Drawing 2007+
pub const APPLICATION_VND_MS_VISIO_DRAWING_MAIN_XML: &str =
    "application/vnd.ms-visio.drawing.main+xml";

/// Microsoft HTML Help
pub const APPLICATION_VND_MS_HTMLHELP: &str = "application/vnd.ms-htmlhelp";

/// Microsoft OneNote
pub const APPLICATION_ONENOTE: &str = "application/onenote";

/// EPUB Electronic Publication
pub const APPLICATION_EPUB_ZIP: &str = "application/epub+zip";

/// Java Archive
pub const APPLICATION_JAVA_ARCHIVE: &str = "application/java-archive";

/// Java Archive (aliases)
pub const APPLICATION_JAR: &str = "application/jar";
pub const APPLICATION_JAR_ARCHIVE: &str = "application/jar-archive";
pub const APPLICATION_X_JAVA_ARCHIVE: &str = "application/x-java-archive";

/// Android Package
pub const APPLICATION_VND_ANDROID_PACKAGE_ARCHIVE: &str = "application/vnd.android.package-archive";

/// Microsoft Excel legacy format
pub const APPLICATION_VND_MS_EXCEL: &str = "application/vnd.ms-excel";

/// Microsoft Word legacy format
pub const APPLICATION_MSWORD: &str = "application/msword";

/// WordPerfect Document
pub const APPLICATION_VND_WORDPERFECT: &str = "application/vnd.wordperfect";

/// Microsoft PowerPoint legacy format
pub const APPLICATION_VND_MS_POWERPOINT: &str = "application/vnd.ms-powerpoint";

/// Microsoft Publisher
pub const APPLICATION_VND_MS_PUBLISHER: &str = "application/vnd.ms-publisher";

/// Microsoft Outlook Message
pub const APPLICATION_VND_MS_OUTLOOK: &str = "application/vnd.ms-outlook";

/// Microsoft Installer
pub const APPLICATION_X_MS_INSTALLER: &str = "application/x-ms-installer";

/// Microsoft Reader eBook
pub const APPLICATION_X_MS_READER: &str = "application/x-ms-reader";

// ============================================================================
// OPEN DOCUMENT FORMATS
// ============================================================================

/// OpenDocument Text
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT: &str = "application/vnd.oasis.opendocument.text";

/// OpenDocument Spreadsheet
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET: &str =
    "application/vnd.oasis.opendocument.spreadsheet";

/// OpenDocument Presentation
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION: &str =
    "application/vnd.oasis.opendocument.presentation";

/// OpenDocument Graphics
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS: &str =
    "application/vnd.oasis.opendocument.graphics";

/// OpenDocument Formula
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA: &str =
    "application/vnd.oasis.opendocument.formula";

/// OpenDocument Chart
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_CHART: &str =
    "application/vnd.oasis.opendocument.chart";

/// OpenOffice Calc Spreadsheet
pub const APPLICATION_VND_SUN_XML_CALC: &str = "application/vnd.sun.xml.calc";

/// Google Earth KMZ (Zipped KML)
pub const APPLICATION_VND_GOOGLE_EARTH_KMZ: &str = "application/vnd.google-earth.kmz";

/// OpenDocument Text Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.text-template";

/// OpenDocument Spreadsheet Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.spreadsheet-template";

/// OpenDocument Presentation Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.presentation-template";

/// OpenDocument Graphics Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.graphics-template";

// ============================================================================
// DATABASE FORMATS
// ============================================================================

/// Microsoft Access Database
pub const APPLICATION_X_MSACCESS: &str = "application/x-msaccess";

/// dBase Database File
pub const APPLICATION_X_DBF: &str = "application/x-dbf";

/// Lotus 1-2-3 Spreadsheet
pub const APPLICATION_VND_LOTUS_1_2_3: &str = "application/vnd.lotus-1-2-3";

/// MARC Library Records
pub const APPLICATION_MARC: &str = "application/marc";

/// SQLite database
pub const APPLICATION_VND_SQLITE3: &str = "application/vnd.sqlite3";

/// SQLite database (alias)
pub const APPLICATION_X_SQLITE3: &str = "application/x-sqlite3";

// ============================================================================
// PROGRAMMING & TEXT FORMATS
// ============================================================================

/// PHP Script
pub const TEXT_X_PHP: &str = "text/x-php";

/// JavaScript
pub const TEXT_JAVASCRIPT: &str = "text/javascript";
pub const APPLICATION_JAVASCRIPT: &str = "application/javascript";

/// Python Script
pub const TEXT_X_PYTHON: &str = "text/x-python";

/// Python Script (aliases)
pub const TEXT_X_SCRIPT_PYTHON: &str = "text/x-script.python";
pub const APPLICATION_X_PYTHON: &str = "application/x-python";

/// Perl Script
pub const TEXT_X_PERL: &str = "text/x-perl";

/// Ruby Script
pub const TEXT_X_RUBY: &str = "text/x-ruby";

/// Ruby Script (alias)
pub const APPLICATION_X_RUBY: &str = "application/x-ruby";

/// Lua Script
pub const TEXT_X_LUA: &str = "text/x-lua";

/// Shell Script
pub const TEXT_X_SHELLSCRIPT: &str = "text/x-shellscript";

/// Shell Script (aliases)
pub const TEXT_X_SH: &str = "text/x-sh";
pub const APPLICATION_X_SHELLSCRIPT: &str = "application/x-shellscript";
pub const APPLICATION_X_SH: &str = "application/x-sh";

/// Tcl Script
pub const TEXT_X_TCL: &str = "text/x-tcl";

/// Tcl Script (alias)
pub const APPLICATION_X_TCL: &str = "application/x-tcl";

/// JSON Data
pub const APPLICATION_JSON: &str = "application/json";

/// JSON Data with UTF-16 encoding
pub const APPLICATION_JSON_UTF16: &str = "application/json; charset=utf-16";

/// GeoJSON Geographic Data
pub const APPLICATION_GEO_JSON: &str = "application/geo+json";

/// Newline Delimited JSON
pub const APPLICATION_X_NDJSON: &str = "application/x-ndjson";

/// CSV Data
pub const TEXT_CSV: &str = "text/csv";

/// CSV Data with UTF-16 encoding
pub const TEXT_CSV_UTF16: &str = "text/csv; charset=utf-16";

/// Tab Separated Values
pub const TEXT_TAB_SEPARATED_VALUES: &str = "text/tab-separated-values";

/// Tab Separated Values with UTF-16 encoding
pub const TEXT_TAB_SEPARATED_VALUES_UTF16: &str = "text/tab-separated-values; charset=utf-16";

/// Rich Text Format
pub const TEXT_RTF: &str = "text/rtf";

/// Rich Text Format with UTF-16 encoding
pub const TEXT_RTF_UTF16: &str = "text/rtf; charset=utf-16";

/// Rich Text Format (alias)
pub const APPLICATION_RTF: &str = "application/rtf";

/// SubRip Subtitles
pub const APPLICATION_X_SUBRIP: &str = "application/x-subrip";

/// SubRip Subtitles with UTF-16 encoding
pub const APPLICATION_X_SUBRIP_UTF16: &str = "application/x-subrip; charset=utf-16";

/// SubRip Subtitles (aliases)
pub const APPLICATION_X_SRT: &str = "application/x-srt";
pub const TEXT_X_SRT: &str = "text/x-srt";

/// WebVTT Subtitles
pub const TEXT_VTT: &str = "text/vtt";

/// WebVTT Subtitles with UTF-16 encoding
pub const TEXT_VTT_UTF16: &str = "text/vtt; charset=utf-16";

/// vCard Contact
pub const TEXT_VCARD: &str = "text/vcard";

/// vCard Contact with UTF-16 encoding
pub const TEXT_VCARD_UTF16: &str = "text/vcard; charset=utf-16";

/// iCalendar
pub const TEXT_CALENDAR: &str = "text/calendar";

/// iCalendar with UTF-16 encoding
pub const TEXT_CALENDAR_UTF16: &str = "text/calendar; charset=utf-16";

/// Scalable Vector Graphics
pub const IMAGE_SVG_XML: &str = "image/svg+xml";

/// Scalable Vector Graphics with UTF-16 encoding
pub const IMAGE_SVG_XML_UTF16: &str = "image/svg+xml; charset=utf-16";

// ============================================================================
// BASE MIME TYPES WITHOUT CHARSET (for UTF-16 aliases)
// ============================================================================

/// Base HTML MIME type without charset
pub const TEXT_HTML_BASE: &str = "text/html";

/// Base XML MIME type without charset
pub const TEXT_XML_BASE: &str = "text/xml";

/// Base JSON MIME type without charset
pub const APPLICATION_JSON_BASE: &str = "application/json";

/// Base CSV MIME type without charset
pub const TEXT_CSV_BASE: &str = "text/csv";

/// Base TSV MIME type without charset
pub const TEXT_TAB_SEPARATED_VALUES_BASE: &str = "text/tab-separated-values";

/// Base SRT MIME type without charset
pub const APPLICATION_X_SUBRIP_BASE: &str = "application/x-subrip";

/// Base VTT MIME type without charset
pub const TEXT_VTT_BASE: &str = "text/vtt";

/// Base vCard MIME type without charset
pub const TEXT_VCARD_BASE: &str = "text/vcard";

/// Base iCalendar MIME type without charset
pub const TEXT_CALENDAR_BASE: &str = "text/calendar";

/// Base RTF MIME type without charset
pub const TEXT_RTF_BASE: &str = "text/rtf";

/// Base SVG MIME type without charset
pub const IMAGE_SVG_XML_BASE: &str = "image/svg+xml";

// ============================================================================
// XML-BASED FORMATS
// ============================================================================

/// RSS Feed
pub const APPLICATION_RSS_XML: &str = "application/rss+xml";

/// RSS Feed (alias)
pub const TEXT_RSS: &str = "text/rss";

/// Atom Feed  
pub const APPLICATION_ATOM_XML: &str = "application/atom+xml";

/// X3D 3D Graphics
pub const MODEL_X3D_XML: &str = "model/x3d+xml";

/// Google Earth KML
pub const APPLICATION_VND_GOOGLE_EARTH_KML_XML: &str = "application/vnd.google-earth.kml+xml";

/// XLIFF Translation
pub const APPLICATION_X_XLIFF_XML: &str = "application/x-xliff+xml";

/// Collada 3D Graphics
pub const MODEL_VND_COLLADA_XML: &str = "model/vnd.collada+xml";

/// Geography Markup Language
pub const APPLICATION_GML_XML: &str = "application/gml+xml";

/// GPS Exchange Format
pub const APPLICATION_GPX_XML: &str = "application/gpx+xml";

/// Training Center XML
pub const APPLICATION_VND_GARMIN_TCX_XML: &str = "application/vnd.garmin.tcx+xml";

/// Additive Manufacturing Format
pub const APPLICATION_X_AMF: &str = "application/x-amf";

/// 3D Manufacturing Format
pub const APPLICATION_VND_MS_PACKAGE_3DMANUFACTURING_3DMODEL_XML: &str =
    "application/vnd.ms-package.3dmanufacturing-3dmodel+xml";

/// Adobe XFDF
pub const APPLICATION_VND_ADOBE_XFDF: &str = "application/vnd.adobe.xfdf";

/// OWL Ontology
pub const APPLICATION_OWL_XML: &str = "application/owl+xml";

/// XHTML
pub const APPLICATION_XHTML_XML: &str = "application/xhtml+xml";

/// HTTP Archive Format
pub const APPLICATION_JSON_HAR: &str = "application/json";

// ============================================================================
// 3D & GEOSPATIAL FORMATS
// ============================================================================

/// ESRI Shapefile
pub const APPLICATION_VND_SHP: &str = "application/vnd.shp";

/// ESRI Shapefile Index
pub const APPLICATION_VND_SHX: &str = "application/vnd.shx";

/// glTF Binary
pub const MODEL_GLTF_BINARY: &str = "model/gltf-binary";

/// glTF JSON
pub const MODEL_GLTF_JSON: &str = "model/gltf+json";

// ============================================================================
// NINTENDO & GAMING FORMATS
// ============================================================================

/// Nintendo Entertainment System ROM
pub const APPLICATION_VND_NINTENDO_SNES_ROM: &str = "application/vnd.nintendo.snes.rom";

// ============================================================================
// ADDITIONAL VIDEO FORMATS
// ============================================================================

/// 3GPP Multimedia
pub const VIDEO_3GPP: &str = "video/3gpp";

/// 3GPP Multimedia (aliases)
pub const VIDEO_3GP: &str = "video/3gp";
pub const AUDIO_3GPP: &str = "audio/3gpp";

/// 3GPP2 Multimedia
pub const VIDEO_3GPP2: &str = "video/3gpp2";

/// 3GPP2 Multimedia (aliases)
pub const VIDEO_3G2: &str = "video/3g2";
pub const AUDIO_3GPP2: &str = "audio/3gpp2";

/// Motion JPEG 2000
pub const VIDEO_MJ2: &str = "video/mj2";

/// Digital Video Broadcasting
pub const VIDEO_VND_DVB_FILE: &str = "video/vnd.dvb.file";

/// AVIF Image Sequence
pub const IMAGE_AVIF: &str = "image/avif";

// ============================================================================
// MISCELLANEOUS FORMATS
// ============================================================================

/// Hierarchical Data Format
pub const APPLICATION_X_HDF: &str = "application/x-hdf";

/// CBOR Binary Data
pub const APPLICATION_CBOR: &str = "application/cbor";

/// Apache Parquet
pub const APPLICATION_VND_APACHE_PARQUET: &str = "application/vnd.apache.parquet";

/// Apache Parquet (alias)
pub const APPLICATION_X_PARQUET: &str = "application/x-parquet";

/// Unix Link File
pub const APPLICATION_X_MS_SHORTCUT: &str = "application/x-ms-shortcut";

/// Mach-O Binary
pub const APPLICATION_X_MACH_BINARY: &str = "application/x-mach-binary";

/// Time Zone Information Format
pub const APPLICATION_TZIF: &str = "application/tzif";

// ============================================================================
// ELF EXECUTABLE TYPES
// ============================================================================

/// ELF Object File
pub const APPLICATION_X_OBJECT: &str = "application/x-object";

/// ELF Executable
pub const APPLICATION_X_EXECUTABLE: &str = "application/x-executable";

/// ELF Shared Library  
pub const APPLICATION_X_SHAREDLIB: &str = "application/x-sharedlib";

/// ELF Core Dump
pub const APPLICATION_X_COREDUMP: &str = "application/x-coredump";

/// Binary fallback
pub const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";
