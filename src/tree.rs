//! MIME Type Detection Tree
//!
//! This module implements the hierarchical MIME type detection system using a tree structure
//! for optimal performance and accuracy. The detection tree is organized by priority, with
//! more specific formats checked before generic ones.
//!
//! # Architecture
//!
//! The tree uses a parent-child relationship where:
//! - ROOT (application/octet-stream) is the fallback for all binary data
//! - Text formats have highest priority for quick elimination of text files
//! - Binary formats are organized by frequency and specificity
//! - Each MIME type contains its own detection function and metadata
//!
//! # Detection Process
//!
//! 1. Initialize the tree on first use (lazy initialization)
//! 2. Start from ROOT and traverse children
//! 3. Test each MIME type's matcher function against input data
//! 4. Return the first successful match (most specific)
//! 5. Fall back to parent if no children match
//!
//! # Thread Safety
//!
//! All operations are thread-safe through the use of static data with 'static lifetime.
//! The initialization is protected by std::sync::Once to ensure single execution.

use crate::constants::*;
use crate::mime_type::MimeType;
use crate::MimeKind;

/// Root MIME type that serves as the fallback for all unrecognized binary data.
///
/// This is the entry point for the detection tree. It contains references to all
/// top-level MIME type categories organized by detection priority:
///
/// 1. Text formats (HTML, XML, UTF variants) - highest priority
/// 2. Documents (PDF, PostScript, OLE)  
/// 3. Archives and compression formats
/// 4. Images (organized by popularity)
/// 5. Audio formats
/// 6. Video formats
/// 7. Executables and binary formats
/// 8. Fonts
/// 9. Web and multimedia formats
/// 10. Specialized formats
/// 11. Generic text (UTF-8) - lowest priority fallback
pub static ROOT: MimeType = MimeType::new(
    APPLICATION_OCTET_STREAM,
    "",
    |_| true,
    &[
        // Go ordering: xpm, sevenZ, zip, pdf, fdf, ole, ps, psd, p7s, ogg, png, jpg, jxl, jp2, jpx,
        // jpm, jxs, gif, webp, exe, elf, ar, tar, xar, bz2, fits, tiff, bmp, lotus, ico,
        // mp3, flac, midi, ape, musePack, amr, wav, aiff, au, mpeg, quickTime, mp4, webM,
        // avi, flv, mkv, asf, aac, voc, m3u, rmvb, gzip, class, swf, crx, ttf, woff,
        // woff2, otf, ttc, eot, wasm, shx, dbf, dcm, rar, djvu, mobi, lit, bpg, cbor,
        // sqlite3, dwg, nes, lnk, macho, qcp, icns, hdr, mrc, mdb, accdb, zstd, cab,
        // rpm, xz, lzip, torrent, cpio, tzif, xcf, pat, gbr, glb, cabIS, jxr, parquet, text
        &XPM,
        &SEVEN_Z,
        &ZIP,
        &PDF,
        &FDF, // PDF variant
        &OLE,
        // Text encoding formats need to be in root for early detection
        &UTF8_BOM,
        &UTF16_BE,
        &UTF16_LE,
        &PS,
        &PSD,
        &PBM,
        &PGM,
        &PPM,
        &PAM,
        &P7S, // PKCS7 signature
        &OGG,
        &PNG,
        &JPG,
        &JXL,
        &JP2,
        &JPX,
        &JPM,
        &JXS,
        &GIF,
        &WEBP,
        &EXE,
        &ELF,
        &AR,
        &TAR,
        &XAR,
        &BZ2,
        &FITS,
        &TIFF,
        &BMP,
        &LOTUS123,
        &ICO,
        &MP3,
        &FLAC,
        &MIDI,
        &APE,
        &MUSEPACK,
        &AMR,
        &WAV,
        &AIFF,
        &AU,
        &MPEG,
        &QUICKTIME,
        &MQV,
        &MP4_PRECISE,
        &WEBM,
        &AVI,
        &FLV,
        &MKV,
        &ASF,
        &AAC,
        &VOC,
        &M3U,
        &RMVB,
        &GZIP,
        &CLASS,
        &SWF,
        &CRX,
        &TTF,
        &WOFF,
        &WOFF2,
        &OTF,
        &TTC,
        &EOT,
        &WASM,
        &SHX,
        &DBF,
        &DCM,
        &RAR,
        &DJVU,
        &MOBI,
        &LIT,
        &BPG,
        &CBOR_FORMAT,
        &SQLITE3,
        &DWG,
        &DXF,
        &WPD,
        &NES,
        &LNK,
        &MACHO,
        &QCP,
        &ICNS,
        &HDR,
        &HDF,
        &MRC,
        &MDB,
        &ACCDB,
        &ZSTD,
        &CAB,
        &CHM,
        &RPM,
        &XZ,
        &LZIP,
        &TORRENT,
        &CPIO,
        &TZIF,
        &XCF,
        &PAT,
        &GBR,
        &GLB,
        &INSTALL_SHIELD_CAB,
        &JXR,
        &PARQUET,
        // Keep text last because it is the slowest check (matches Go comment)
        &UTF8,
    ],
);

// ============================================================================
// TEXT FORMATS
// ============================================================================
//
// Text formats have the highest priority in the detection tree because:
// 1. They're common and can be quickly identified
// 2. They help eliminate binary format checks for text files
// 3. Encoding-specific variants (UTF-8, UTF-16) are checked first
// 4. Generic UTF-8 is the lowest priority fallback for all text

/// HTML format with enhanced case-insensitive tag detection.
///
/// Detects HTML files by looking for common HTML tags while handling:
/// - Case insensitive matching (handles both <html> and <HTML>)
/// - Proper tag termination validation
/// - DOCTYPE declarations and comments
/// - Whitespace tolerance at the beginning of files
static HTML: MimeType = MimeType::new(TEXT_HTML, ".html", html, &[])
    .with_extension_aliases(&[".htm"])
    .with_parent(&UTF8);

static XML: MimeType = MimeType::new(
    TEXT_XML,
    ".xml",
    xml,
    &[
        &RSS, &ATOM, &X3D, &KML, &XLIFF, &COLLADA, &GML, &GPX, &TCX, &AMF, &THREEMF, &XFDF, &OWL2,
        &XHTML,
    ],
)
.with_aliases(&[APPLICATION_XML])
.with_parent(&UTF8);

static UTF8_BOM: MimeType =
    MimeType::new(TEXT_UTF8_BOM, ".txt", utf8_bom, &[]).with_kind(MimeKind::TEXT);

static UTF16_BE: MimeType = MimeType::new(
    TEXT_UTF16_BE,
    ".txt",
    utf16_be,
    &[
        &HTML_UTF16_BE,
        &XML_UTF16_BE,
        &SVG_UTF16_BE,
        &JSON_UTF16_BE,
        &CSV_UTF16_BE,
        &TSV_UTF16_BE,
        &SRT_UTF16_BE,
        &VTT_UTF16_BE,
        &VCARD_UTF16_BE,
        &ICALENDAR_UTF16_BE,
        &RTF_UTF16_BE,
    ],
)
.with_kind(MimeKind::TEXT);

static UTF16_LE: MimeType = MimeType::new(
    TEXT_UTF16_LE,
    ".txt",
    utf16_le,
    &[
        &HTML_UTF16_LE,
        &XML_UTF16_LE,
        &SVG_UTF16_LE,
        &JSON_UTF16_LE,
        &CSV_UTF16_LE,
        &TSV_UTF16_LE,
        &SRT_UTF16_LE,
        &VTT_UTF16_LE,
        &VCARD_UTF16_LE,
        &ICALENDAR_UTF16_LE,
        &RTF_UTF16_LE,
    ],
)
.with_kind(MimeKind::TEXT);

static UTF8: MimeType = MimeType::new(
    TEXT_UTF8,
    ".txt",
    utf8,
    &[
        &HTML,
        &XML,
        &RTF, // RTF must come before JSON (both start with {, RTF has more specific pattern)
        &PHP,
        &JAVASCRIPT,
        &PYTHON,
        &PERL,
        &RUBY,
        &LUA,
        &SHELL,
        &TCL,
        &JSON,
        &CSV_FORMAT,
        &TSV,
        &SRT,
        &VTT,
        &VCARD,
        &ICALENDAR,
        &SVG,
        &WARC,
    ],
)
.with_aliases(&[TEXT_PLAIN])
.with_extension_aliases(&[
    "",
    ".pub",
    ".html",
    ".htm",
    ".shtml",
    ".svg",
    ".xml",
    ".rss",
    ".atom",
    ".x3d",
    ".kml",
    ".xlf",
    ".dae",
    ".gml",
    ".gpx",
    ".tcx",
    ".amf",
    "3mf",
    ".php",
    ".js",
    ".lua",
    ".pl",
    ".py",
    ".json",
    ".geojson",
    ".ndjson",
    ".rtf",
    ".tcl",
    ".csv",
    ".tsv",
    ".vcf",
    ".vcard",
    ".ics",
    ".ical",
    ".icalendar",
    ".warc",
])
.with_kind(MimeKind::TEXT);

// ============================================================================
// DOCUMENT FORMATS
// ============================================================================

static PDF: MimeType = MimeType::new(APPLICATION_PDF, ".pdf", pdf, &[])
    .with_aliases(&[APPLICATION_X_PDF])
    .with_kind(MimeKind::DOCUMENT);

static FDF: MimeType =
    MimeType::new(APPLICATION_VND_FDF, ".fdf", fdf, &[]).with_kind(MimeKind::DOCUMENT);

static PS: MimeType =
    MimeType::new(APPLICATION_POSTSCRIPT, ".ps", ps, &[]).with_kind(MimeKind::DOCUMENT);

static OLE: MimeType = MimeType::new(
    APPLICATION_X_OLE_STORAGE,
    "",
    ole,
    &[
        &MSI,
        &AAF,
        &MSG,
        &XLS,
        &PUB,
        &PPT,
        &DOC,
        &ONENOTE,
        &FASOO,
        &PGP_NET_SHARE,
    ],
)
.with_extension_aliases(&[".xls", ".pub", ".ppt", ".doc", ".chm", ".one"])
.with_kind(MimeKind::DOCUMENT);

static AAF: MimeType = MimeType::new(APPLICATION_X_AAF, ".aaf", aaf, &[]).with_parent(&OLE);

// ============================================================================
// ARCHIVE & COMPRESSION FORMATS
// ============================================================================
//
// Archive formats are prioritized by popularity and detection reliability:
// 1. Common formats like ZIP and 7Z come first
// 2. Enhanced variants provide more sophisticated detection
// 3. TAR uses checksum validation for reliability
// 4. Compression formats are organized by algorithm type

/// 7-Zip archive format with distinctive signature.
///
/// 7Z files start with a unique 6-byte signature that makes detection reliable.
/// This format supports multiple compression algorithms and strong encryption.
static SEVEN_Z: MimeType = MimeType::new(APPLICATION_X_7Z_COMPRESSED, ".7z", seven_z, &[])
    .with_kind(MimeKind::ARCHIVE)
    .with_parent(&ROOT);

static ZIP: MimeType = MimeType::new(
    APPLICATION_ZIP,
    ".zip",
    zip,
    &[
        &DOCX, &XLSX, &PPTX, &VSDX, &EPUB, &JAR, &APK, &ODT, &ODS, &ODP, &ODG, &ODF, &ODC, &SXC,
        &KMZ,
    ],
)
.with_aliases(&[APPLICATION_X_ZIP, APPLICATION_X_ZIP_COMPRESSED])
.with_extension_aliases(&[
    ".xlsx", ".docx", ".pptx", ".vsdx", ".epub", ".jar", ".odt", ".ods", ".odp", ".odg", ".odf",
    ".sxc", ".kmz",
])
.with_kind(MimeKind::ARCHIVE);

static RAR: MimeType = MimeType::new(APPLICATION_X_RAR_COMPRESSED, ".rar", rar, &[])
    .with_aliases(&[APPLICATION_X_RAR])
    .with_kind(MimeKind::ARCHIVE);

static GZIP: MimeType = MimeType::new(APPLICATION_GZIP, ".gz", gzip, &[])
    .with_aliases(&[
        APPLICATION_X_GZIP,
        APPLICATION_X_GUNZIP,
        APPLICATION_GZIPPED,
        APPLICATION_GZIP_COMPRESSED,
        APPLICATION_X_GZIP_COMPRESSED,
        GZIP_DOCUMENT,
    ])
    .with_extension_aliases(&[".tgz", ".taz"])
    .with_kind(MimeKind::ARCHIVE);

static TAR: MimeType =
    MimeType::new(APPLICATION_X_TAR, ".tar", tar, &[]).with_kind(MimeKind::ARCHIVE);

static BZ2: MimeType =
    MimeType::new(APPLICATION_X_BZIP2, ".bz2", bz2, &[]).with_kind(MimeKind::ARCHIVE);

static XZ: MimeType = MimeType::new(APPLICATION_X_XZ, ".xz", xz, &[]).with_kind(MimeKind::ARCHIVE);

static ZSTD: MimeType =
    MimeType::new(APPLICATION_ZSTD, ".zst", zstd, &[]).with_kind(MimeKind::ARCHIVE);

static LZIP: MimeType = MimeType::new(APPLICATION_LZIP, ".lz", lzip, &[])
    .with_aliases(&[APPLICATION_X_LZIP])
    .with_kind(MimeKind::ARCHIVE);

static CAB: MimeType =
    MimeType::new(APPLICATION_VND_MS_CAB_COMPRESSED, ".cab", cab, &[]).with_kind(MimeKind::ARCHIVE);

static INSTALL_SHIELD_CAB: MimeType =
    MimeType::new(APPLICATION_X_INSTALLSHIELD, ".cab", install_shield_cab, &[])
        .with_kind(MimeKind::ARCHIVE);

static CPIO: MimeType =
    MimeType::new(APPLICATION_X_CPIO, ".cpio", cpio, &[]).with_kind(MimeKind::ARCHIVE);

static AR: MimeType = MimeType::new(APPLICATION_X_ARCHIVE, ".a", ar, &[&DEB])
    .with_aliases(&[APPLICATION_X_UNIX_ARCHIVE])
    .with_extension_aliases(&[".deb"])
    .with_kind(MimeKind::ARCHIVE);

static RPM: MimeType =
    MimeType::new(APPLICATION_X_RPM, ".rpm", rpm, &[]).with_kind(MimeKind::ARCHIVE);

static TORRENT: MimeType = MimeType::new(APPLICATION_X_BITTORRENT, ".torrent", torrent, &[]);

static FITS: MimeType = MimeType::new(APPLICATION_FITS, ".fits", fits, &[])
    .with_aliases(&[IMAGE_FITS])
    .with_kind(MimeKind::IMAGE);

static XAR: MimeType =
    MimeType::new(APPLICATION_X_XAR, ".xar", xar, &[]).with_kind(MimeKind::ARCHIVE);

static DEB: MimeType = MimeType::new(APPLICATION_VND_DEBIAN_BINARY_PACKAGE, ".deb", deb, &[])
    .with_kind(MimeKind::ARCHIVE);

static WARC: MimeType = MimeType::new(APPLICATION_WARC, ".warc", warc, &[])
    .with_kind(MimeKind::ARCHIVE)
    .with_parent(&UTF8);

// ============================================================================
// UTF-16 TEXT FORMAT VARIANTS
// ============================================================================

/// HTML format for UTF-16 Big Endian
static HTML_UTF16_BE: MimeType =
    MimeType::new(TEXT_HTML_UTF16, ".html", html_utf16_be, &[]).with_parent(&UTF16_BE);

/// HTML format for UTF-16 Little Endian  
static HTML_UTF16_LE: MimeType =
    MimeType::new(TEXT_HTML_UTF16, ".html", html_utf16_le, &[]).with_parent(&UTF16_LE);

/// XML format for UTF-16 Big Endian
static XML_UTF16_BE: MimeType = MimeType::new(TEXT_XML_UTF16, ".xml", xml_utf16_be, &[])
    .with_aliases(&[APPLICATION_XML_UTF16])
    .with_parent(&UTF16_BE);

/// XML format for UTF-16 Little Endian
static XML_UTF16_LE: MimeType = MimeType::new(TEXT_XML_UTF16, ".xml", xml_utf16_le, &[])
    .with_aliases(&[APPLICATION_XML_UTF16])
    .with_parent(&UTF16_LE);

/// SVG format for UTF-16 Big Endian
static SVG_UTF16_BE: MimeType =
    MimeType::new(IMAGE_SVG_XML_UTF16, ".svg", svg_utf16_be, &[]).with_parent(&UTF16_BE);

/// SVG format for UTF-16 Little Endian
static SVG_UTF16_LE: MimeType =
    MimeType::new(IMAGE_SVG_XML_UTF16, ".svg", svg_utf16_le, &[]).with_parent(&UTF16_LE);

/// JSON format for UTF-16 Big Endian
static JSON_UTF16_BE: MimeType =
    MimeType::new(APPLICATION_JSON_UTF16, ".json", json_utf16_be, &[]).with_parent(&UTF16_BE);

/// JSON format for UTF-16 Little Endian
static JSON_UTF16_LE: MimeType =
    MimeType::new(APPLICATION_JSON_UTF16, ".json", json_utf16_le, &[]).with_parent(&UTF16_LE);

/// CSV format for UTF-16 Big Endian
static CSV_UTF16_BE: MimeType =
    MimeType::new(TEXT_CSV_UTF16, ".csv", csv_utf16_be, &[]).with_parent(&UTF16_BE);

/// CSV format for UTF-16 Little Endian
static CSV_UTF16_LE: MimeType =
    MimeType::new(TEXT_CSV_UTF16, ".csv", csv_utf16_le, &[]).with_parent(&UTF16_LE);

/// TSV format for UTF-16 Big Endian
static TSV_UTF16_BE: MimeType =
    MimeType::new(TEXT_TAB_SEPARATED_VALUES_UTF16, ".tsv", tsv_utf16_be, &[])
        .with_parent(&UTF16_BE);

/// TSV format for UTF-16 Little Endian
static TSV_UTF16_LE: MimeType =
    MimeType::new(TEXT_TAB_SEPARATED_VALUES_UTF16, ".tsv", tsv_utf16_le, &[])
        .with_parent(&UTF16_LE);

/// SRT subtitle format for UTF-16 Big Endian
static SRT_UTF16_BE: MimeType =
    MimeType::new(APPLICATION_X_SUBRIP_UTF16, ".srt", srt_utf16_be, &[]).with_parent(&UTF16_BE);

/// SRT subtitle format for UTF-16 Little Endian
static SRT_UTF16_LE: MimeType =
    MimeType::new(APPLICATION_X_SUBRIP_UTF16, ".srt", srt_utf16_le, &[]).with_parent(&UTF16_LE);

/// VTT subtitle format for UTF-16 Big Endian
static VTT_UTF16_BE: MimeType =
    MimeType::new(TEXT_VTT_UTF16, ".vtt", vtt_utf16_be, &[]).with_parent(&UTF16_BE);

/// VTT subtitle format for UTF-16 Little Endian
static VTT_UTF16_LE: MimeType =
    MimeType::new(TEXT_VTT_UTF16, ".vtt", vtt_utf16_le, &[]).with_parent(&UTF16_LE);

/// vCard format for UTF-16 Big Endian
static VCARD_UTF16_BE: MimeType =
    MimeType::new(TEXT_VCARD_UTF16, ".vcf", vcard_utf16_be, &[]).with_parent(&UTF16_BE);

/// vCard format for UTF-16 Little Endian
static VCARD_UTF16_LE: MimeType =
    MimeType::new(TEXT_VCARD_UTF16, ".vcf", vcard_utf16_le, &[]).with_parent(&UTF16_LE);

/// iCalendar format for UTF-16 Big Endian
static ICALENDAR_UTF16_BE: MimeType =
    MimeType::new(TEXT_CALENDAR_UTF16, ".ics", icalendar_utf16_be, &[]).with_parent(&UTF16_BE);

/// iCalendar format for UTF-16 Little Endian
static ICALENDAR_UTF16_LE: MimeType =
    MimeType::new(TEXT_CALENDAR_UTF16, ".ics", icalendar_utf16_le, &[]).with_parent(&UTF16_LE);

/// RTF format for UTF-16 Big Endian
static RTF_UTF16_BE: MimeType =
    MimeType::new(TEXT_RTF_UTF16, ".rtf", rtf_utf16_be, &[]).with_parent(&UTF16_BE);

/// RTF format for UTF-16 Little Endian
static RTF_UTF16_LE: MimeType =
    MimeType::new(TEXT_RTF_UTF16, ".rtf", rtf_utf16_le, &[]).with_parent(&UTF16_LE);

// ============================================================================
// IMAGE FORMATS
// ============================================================================

static PNG: MimeType = MimeType::new(IMAGE_PNG, ".png", png, &[&APNG]).with_kind(MimeKind::IMAGE);

static APNG: MimeType =
    MimeType::new(IMAGE_VND_MOZILLA_APNG, ".apng", apng, &[]).with_kind(MimeKind::IMAGE);

static JPG: MimeType = MimeType::new(IMAGE_JPEG, ".jpg", jpg, &[])
    .with_extension_aliases(&[".jpeg", ".jpe", ".jif", ".jfif", ".jfi"])
    .with_kind(MimeKind::IMAGE);

static JP2: MimeType = MimeType::new(IMAGE_JP2, ".jp2", jp2, &[]).with_kind(MimeKind::IMAGE);

static JPX: MimeType = MimeType::new(IMAGE_JPX, ".jpx", jpx, &[]).with_kind(MimeKind::IMAGE);

static JPM: MimeType = MimeType::new(IMAGE_JPM, ".jpm", jpm, &[])
    .with_aliases(&[VIDEO_JPM])
    .with_kind(MimeKind::IMAGE);

static JXS: MimeType = MimeType::new(IMAGE_JXS, ".jxs", jxs, &[]).with_kind(MimeKind::IMAGE);

static JXR: MimeType = MimeType::new(IMAGE_JXR, ".jxr", jxr, &[])
    .with_aliases(&[IMAGE_VND_MS_PHOTO])
    .with_kind(MimeKind::IMAGE);

static JXL: MimeType = MimeType::new(IMAGE_JXL, ".jxl", jxl, &[]).with_kind(MimeKind::IMAGE);

static GIF: MimeType = MimeType::new(IMAGE_GIF, ".gif", gif, &[]).with_kind(MimeKind::IMAGE);

static WEBP: MimeType = MimeType::new(IMAGE_WEBP, ".webp", webp, &[]).with_kind(MimeKind::IMAGE);

static TIFF: MimeType = MimeType::new(IMAGE_TIFF, ".tiff", tiff, &[])
    .with_extension_aliases(&[".tif"])
    .with_kind(MimeKind::IMAGE);

static BMP: MimeType = MimeType::new(IMAGE_BMP, ".bmp", bmp, &[])
    .with_aliases(&[IMAGE_X_BMP, IMAGE_X_MS_BMP])
    .with_extension_aliases(&[".dib"])
    .with_kind(MimeKind::IMAGE);

static ICO: MimeType = MimeType::new(IMAGE_X_ICON, ".ico", ico, &[]).with_kind(MimeKind::IMAGE);

static ICNS: MimeType = MimeType::new(IMAGE_X_ICNS, ".icns", icns, &[]).with_kind(MimeKind::IMAGE);

static PSD: MimeType = MimeType::new(IMAGE_VND_ADOBE_PHOTOSHOP, ".psd", psd, &[])
    .with_aliases(&[IMAGE_X_PSD, APPLICATION_PHOTOSHOP])
    .with_kind(MimeKind::IMAGE);

static PBM: MimeType =
    MimeType::new(IMAGE_X_PORTABLE_BITMAP, ".pbm", pbm, &[]).with_kind(MimeKind::IMAGE);

static PGM: MimeType =
    MimeType::new(IMAGE_X_PORTABLE_GRAYMAP, ".pgm", pgm, &[]).with_kind(MimeKind::IMAGE);

static PPM: MimeType =
    MimeType::new(IMAGE_X_PORTABLE_PIXMAP, ".ppm", ppm, &[]).with_kind(MimeKind::IMAGE);

static PAM: MimeType =
    MimeType::new(IMAGE_X_PORTABLE_ARBITRARYMAP, ".pam", pam, &[]).with_kind(MimeKind::IMAGE);

static HEIC: MimeType = MimeType::new(IMAGE_HEIC, ".heic", heic, &[])
    .with_kind(MimeKind::IMAGE)
    .with_parent(&HEIF);

static HEIC_SEQ: MimeType = MimeType::new(IMAGE_HEIC_SEQUENCE, ".heic", heic_sequence, &[])
    .with_kind(MimeKind::IMAGE)
    .with_parent(&HEIF);

static HEIF: MimeType = MimeType::new(IMAGE_HEIF, ".heif", heif, &[]).with_kind(MimeKind::IMAGE);

static HEIF_SEQ: MimeType =
    MimeType::new(IMAGE_HEIF_SEQUENCE, ".heif", heif_sequence, &[]).with_kind(MimeKind::IMAGE);

static BPG: MimeType = MimeType::new(IMAGE_BPG, ".bpg", bpg, &[]).with_kind(MimeKind::IMAGE);

static XCF: MimeType = MimeType::new(IMAGE_X_XCF, ".xcf", xcf, &[]).with_kind(MimeKind::IMAGE);

static PAT: MimeType = MimeType::new(IMAGE_X_GIMP_PAT, ".pat", pat, &[]).with_kind(MimeKind::IMAGE);

static GBR: MimeType = MimeType::new(IMAGE_X_GIMP_GBR, ".gbr", gbr, &[]).with_kind(MimeKind::IMAGE);

static HDR: MimeType =
    MimeType::new(IMAGE_VND_RADIANCE, ".hdr", hdr, &[]).with_kind(MimeKind::IMAGE);

static XPM: MimeType = MimeType::new(IMAGE_X_XPIXMAP, ".xpm", xpm, &[]).with_kind(MimeKind::IMAGE);

static DWG: MimeType = MimeType::new(IMAGE_VND_DWG, ".dwg", dwg, &[])
    .with_aliases(&[
        IMAGE_X_DWG,
        APPLICATION_ACAD,
        APPLICATION_X_ACAD,
        APPLICATION_AUTOCAD_DWG,
        APPLICATION_DWG,
        APPLICATION_X_DWG,
        APPLICATION_X_AUTOCAD,
        DRAWING_DWG,
    ])
    .with_kind(MimeKind::IMAGE);

static DXF: MimeType = MimeType::new(IMAGE_VND_DXF, ".dxf", dxf, &[]).with_kind(MimeKind::IMAGE);

static DJVU: MimeType =
    MimeType::new(IMAGE_VND_DJVU, ".djvu", djvu, &[]).with_kind(MimeKind::IMAGE);

// ============================================================================
// AUDIO FORMATS
// ============================================================================

static MP3: MimeType = MimeType::new(AUDIO_MPEG, ".mp3", mp3, &[])
    .with_aliases(&[AUDIO_X_MPEG, AUDIO_MP3])
    .with_kind(MimeKind::AUDIO);

static FLAC: MimeType = MimeType::new(AUDIO_FLAC, ".flac", flac, &[]).with_kind(MimeKind::AUDIO);

static WAV: MimeType = MimeType::new(AUDIO_WAV, ".wav", wav, &[])
    .with_aliases(&[AUDIO_X_WAV, AUDIO_VND_WAVE, AUDIO_WAVE])
    .with_kind(MimeKind::AUDIO);

static AIFF: MimeType = MimeType::new(AUDIO_AIFF, ".aiff", aiff, &[])
    .with_extension_aliases(&[".aif"])
    .with_kind(MimeKind::AUDIO);

static MIDI: MimeType = MimeType::new(AUDIO_MIDI, ".midi", midi, &[])
    .with_aliases(&[AUDIO_MID])
    .with_extension_aliases(&[".mid"])
    .with_kind(MimeKind::AUDIO);

static OGG: MimeType = MimeType::new(APPLICATION_OGG, ".ogg", ogg, &[&OGG_AUDIO, &OGG_VIDEO])
    .with_extension_aliases(&[".oga", ".opus", ".ogv"])
    .with_kind(MimeKind::AUDIO);

static OGG_AUDIO: MimeType = MimeType::new(AUDIO_OGG, ".oga", ogg_audio, &[])
    .with_kind(MimeKind::AUDIO)
    .with_parent(&OGG);

static OGG_VIDEO: MimeType = MimeType::new(VIDEO_OGG, ".ogv", ogg_video, &[])
    .with_kind(MimeKind::VIDEO)
    .with_parent(&OGG);

static APE: MimeType = MimeType::new(AUDIO_APE, ".ape", ape, &[]).with_kind(MimeKind::AUDIO);

static MUSEPACK: MimeType =
    MimeType::new(AUDIO_MUSEPACK, ".mpc", musepack, &[]).with_kind(MimeKind::AUDIO);

static AU: MimeType = MimeType::new(AUDIO_BASIC, ".au", au, &[])
    .with_extension_aliases(&[".snd"])
    .with_kind(MimeKind::AUDIO);

static AMR: MimeType = MimeType::new(AUDIO_AMR, ".amr", amr, &[])
    .with_aliases(&[AUDIO_AMR_NB])
    .with_kind(MimeKind::AUDIO);

static VOC: MimeType = MimeType::new(AUDIO_X_UNKNOWN, ".voc", voc, &[]).with_kind(MimeKind::AUDIO);

static M3U: MimeType = MimeType::new(AUDIO_X_MPEGURL, ".m3u", m3u, &[])
    .with_aliases(&[AUDIO_MPEGURL])
    .with_extension_aliases(&[".m3u8"])
    .with_kind(MimeKind::TEXT);

static AAC: MimeType = MimeType::new(AUDIO_AAC, ".aac", aac, &[]).with_kind(MimeKind::AUDIO);

static QCP: MimeType = MimeType::new(AUDIO_QCELP, ".qcp", qcp, &[]).with_kind(MimeKind::AUDIO);

static M4A: MimeType = MimeType::new(AUDIO_X_M4A, ".m4a", m4a, &[]).with_kind(MimeKind::AUDIO);

static AMP4: MimeType = MimeType::new(AUDIO_MP4, ".mp4", amp4, &[])
    .with_aliases(&[AUDIO_X_M4A, AUDIO_X_MP4A])
    .with_kind(MimeKind::AUDIO);

// ============================================================================
// VIDEO FORMATS
// ============================================================================

static MP4_PRECISE: MimeType = MimeType::new(
    VIDEO_MP4,
    ".mp4",
    mp4_precise,
    &[
        &AVIF_FORMAT,
        &THREE_GPP,
        &THREE_GPP2,
        &AMP4,
        &M4A,
        &M4V,
        &HEIC,
        &HEIC_SEQ,
        &HEIF,
        &HEIF_SEQ,
        &MJ2,
        &DVB,
    ],
)
.with_kind(MimeKind::VIDEO);

static WEBM: MimeType = MimeType::new(VIDEO_WEBM, ".webm", webm, &[])
    .with_aliases(&[AUDIO_WEBM])
    .with_kind(MimeKind::VIDEO);

static MKV: MimeType = MimeType::new(VIDEO_X_MATROSKA, ".mkv", mkv, &[])
    .with_extension_aliases(&[".mk3d", ".mka", ".mks"])
    .with_kind(MimeKind::VIDEO);

static AVI: MimeType = MimeType::new(VIDEO_X_MSVIDEO, ".avi", avi, &[])
    .with_aliases(&[VIDEO_AVI, VIDEO_MSVIDEO])
    .with_kind(MimeKind::VIDEO);

static MPEG: MimeType = MimeType::new(VIDEO_MPEG, ".mpeg", mpeg, &[]).with_kind(MimeKind::VIDEO);

static QUICKTIME: MimeType =
    MimeType::new(VIDEO_QUICKTIME, ".mov", quicktime, &[]).with_kind(MimeKind::VIDEO);

static MQV: MimeType = MimeType::new(VIDEO_QUICKTIME, ".mqv", mqv, &[]).with_kind(MimeKind::VIDEO);

static FLV: MimeType = MimeType::new(VIDEO_X_FLV, ".flv", flv, &[]).with_kind(MimeKind::VIDEO);

static ASF: MimeType = MimeType::new(VIDEO_X_MS_ASF, ".asf", asf, &[])
    .with_aliases(&[VIDEO_ASF, VIDEO_X_MS_WMV])
    .with_kind(MimeKind::VIDEO);

static M4V: MimeType = MimeType::new(VIDEO_X_M4V, ".m4v", m4v, &[]).with_kind(MimeKind::VIDEO);

static RMVB: MimeType =
    MimeType::new(APPLICATION_VND_RN_REALMEDIA_VBR, ".rmvb", rmvb, &[]).with_kind(MimeKind::VIDEO);

// ============================================================================
// EXECUTABLE & BINARY FORMATS
// ============================================================================

static EXE: MimeType = MimeType::new(
    APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE,
    ".exe",
    exe,
    &[],
)
.with_kind(MimeKind::EXECUTABLE);

static ELF: MimeType = MimeType::new(
    APPLICATION_X_ELF,
    "",
    elf,
    &[&ELF_OBJ, &ELF_EXE, &ELF_LIB, &ELF_DUMP],
)
.with_extension_aliases(&[".so"])
.with_kind(MimeKind::EXECUTABLE);

static ELF_OBJ: MimeType =
    MimeType::new(APPLICATION_X_OBJECT, "", elf_obj, &[]).with_kind(MimeKind::EXECUTABLE);

static ELF_EXE: MimeType =
    MimeType::new(APPLICATION_X_EXECUTABLE, "", elf_exe, &[]).with_kind(MimeKind::EXECUTABLE);

static ELF_LIB: MimeType =
    MimeType::new(APPLICATION_X_SHAREDLIB, ".so", elf_lib, &[]).with_kind(MimeKind::EXECUTABLE);

static ELF_DUMP: MimeType =
    MimeType::new(APPLICATION_X_COREDUMP, "", elf_dump, &[]).with_kind(MimeKind::EXECUTABLE);

static CLASS: MimeType = MimeType::new(APPLICATION_X_JAVA_APPLET_BINARY, ".class", class, &[])
    .with_aliases(&[APPLICATION_X_JAVA_APPLET])
    .with_kind(MimeKind::APPLICATION);

static WASM: MimeType =
    MimeType::new(APPLICATION_WASM, ".wasm", wasm, &[]).with_kind(MimeKind::EXECUTABLE);

// ============================================================================
// FONT FORMATS
// ============================================================================

static TTF: MimeType = MimeType::new(FONT_TTF, ".ttf", ttf, &[])
    .with_aliases(&[FONT_SFNT, APPLICATION_X_FONT_TTF, APPLICATION_FONT_SFNT])
    .with_kind(MimeKind::FONT);

static WOFF: MimeType = MimeType::new(FONT_WOFF, ".woff", woff, &[]).with_kind(MimeKind::FONT);

static WOFF2: MimeType = MimeType::new(FONT_WOFF2, ".woff2", woff2, &[]).with_kind(MimeKind::FONT);

static OTF: MimeType = MimeType::new(FONT_OTF, ".otf", otf, &[]).with_kind(MimeKind::FONT);

static EOT: MimeType =
    MimeType::new(APPLICATION_VND_MS_FONTOBJECT, ".eot", eot, &[]).with_kind(MimeKind::FONT);

static TTC: MimeType = MimeType::new(FONT_COLLECTION, ".ttc", ttc, &[]).with_kind(MimeKind::FONT);

// ============================================================================
// WEB & MULTIMEDIA FORMATS
// ============================================================================

static SWF: MimeType =
    MimeType::new(APPLICATION_X_SHOCKWAVE_FLASH, ".swf", swf, &[]).with_kind(MimeKind::APPLICATION);

static CRX: MimeType = MimeType::new(APPLICATION_X_CHROME_EXTENSION, ".crx", crx, &[])
    .with_kind(MimeKind::APPLICATION);

static P7S: MimeType =
    MimeType::new(APPLICATION_PKCS7_SIGNATURE, ".p7s", p7s, &[]).with_kind(MimeKind::APPLICATION);

// ============================================================================
// SPECIALIZED FORMATS
// ============================================================================

static DCM: MimeType =
    MimeType::new(APPLICATION_DICOM, ".dcm", dcm, &[]).with_kind(MimeKind::IMAGE);

static MOBI: MimeType =
    MimeType::new(APPLICATION_X_MOBIPOCKET_EBOOK, ".mobi", mobi, &[]).with_kind(MimeKind::DOCUMENT);

static LIT: MimeType =
    MimeType::new(APPLICATION_X_MS_READER, ".lit", lit, &[]).with_kind(MimeKind::DOCUMENT);

static SQLITE3: MimeType = MimeType::new(APPLICATION_VND_SQLITE3, ".sqlite", sqlite, &[])
    .with_aliases(&[APPLICATION_X_SQLITE3])
    .with_kind(MimeKind::DATABASE);

static FASOO: MimeType = MimeType::new(APPLICATION_X_FASOO, "", fasoo, &[]).with_parent(&OLE);

static PGP_NET_SHARE: MimeType =
    MimeType::new(APPLICATION_X_PGP_NET_SHARE, "", pgp_net_share, &[]).with_parent(&OLE);

// ============================================================================
// MICROSOFT OFFICE & DOCUMENT FORMATS
// ============================================================================

static DOCX: MimeType = MimeType::new(
    APPLICATION_VND_OPENXML_WORDPROCESSINGML_DOCUMENT,
    ".docx",
    docx,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static XLSX: MimeType = MimeType::new(
    APPLICATION_VND_OPENXML_SPREADSHEETML_SHEET,
    ".xlsx",
    xlsx,
    &[],
)
.with_kind(MimeKind::SPREADSHEET)
.with_parent(&ZIP);

static PPTX: MimeType = MimeType::new(
    APPLICATION_VND_OPENXML_PRESENTATIONML_PRESENTATION,
    ".pptx",
    pptx,
    &[],
)
.with_kind(MimeKind::PRESENTATION)
.with_parent(&ZIP);

static VSDX: MimeType = MimeType::new(
    APPLICATION_VND_MS_VISIO_DRAWING_MAIN_XML,
    ".vsdx",
    vsdx,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static EPUB: MimeType = MimeType::new(APPLICATION_EPUB_ZIP, ".epub", epub, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&ZIP);

static JAR: MimeType = MimeType::new(APPLICATION_JAVA_ARCHIVE, ".jar", jar, &[])
    .with_aliases(&[
        APPLICATION_JAR,
        APPLICATION_JAR_ARCHIVE,
        APPLICATION_X_JAVA_ARCHIVE,
    ])
    .with_kind(MimeKind::APPLICATION)
    .with_parent(&ZIP);

static APK: MimeType = MimeType::new(APPLICATION_VND_ANDROID_PACKAGE_ARCHIVE, ".apk", apk, &[])
    .with_kind(MimeKind::APPLICATION)
    .with_parent(&ZIP);

static DOC: MimeType = MimeType::new(APPLICATION_MSWORD, ".doc", doc, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&OLE);

static WPD: MimeType =
    MimeType::new(APPLICATION_VND_WORDPERFECT, ".wpd", wpd, &[]).with_kind(MimeKind::DOCUMENT);

static XLS: MimeType = MimeType::new(APPLICATION_VND_MS_EXCEL, ".xls", xls, &[])
    .with_kind(MimeKind::SPREADSHEET)
    .with_parent(&OLE);

static PPT: MimeType = MimeType::new(APPLICATION_VND_MS_POWERPOINT, ".ppt", ppt, &[])
    .with_kind(MimeKind::PRESENTATION)
    .with_parent(&OLE);

static CHM: MimeType =
    MimeType::new(APPLICATION_VND_MS_HTMLHELP, ".chm", chm, &[]).with_kind(MimeKind::DOCUMENT);

static ONENOTE: MimeType = MimeType::new(APPLICATION_ONENOTE, ".one", onenote, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&OLE);

static PUB: MimeType = MimeType::new(APPLICATION_VND_MS_PUBLISHER, ".pub", pub_format, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&OLE);

static MSG: MimeType = MimeType::new(APPLICATION_VND_MS_OUTLOOK, ".msg", msg, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&OLE);

static MSI: MimeType = MimeType::new(APPLICATION_X_MS_INSTALLER, ".msi", msi, &[])
    .with_kind(MimeKind::ARCHIVE)
    .with_parent(&OLE);

// ============================================================================
// OPEN DOCUMENT FORMATS
// ============================================================================

static ODT: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT,
    ".odt",
    odt,
    &[&OTT],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.text"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static ODS: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET,
    ".ods",
    ods,
    &[&OTS],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.spreadsheet"])
.with_kind(MimeKind::SPREADSHEET)
.with_parent(&ZIP);

static ODP: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION,
    ".odp",
    odp,
    &[&OTP],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.presentation"])
.with_kind(MimeKind::PRESENTATION)
.with_parent(&ZIP);

static ODG: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS,
    ".odg",
    odg,
    &[&OTG],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.graphics"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static ODF: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA,
    ".odf",
    odf_format,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.formula"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static ODC: MimeType = MimeType::new(APPLICATION_VND_OASIS_OPENDOCUMENT_CHART, ".odc", odc, &[])
    .with_aliases(&["application/x-vnd.oasis.opendocument.chart"])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&ZIP);

static OTT: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE,
    ".ott",
    ott,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.text-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODT);

static OTS: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE,
    ".ots",
    ots,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.spreadsheet-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODS);

static OTP: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE,
    ".otp",
    otp,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.presentation-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODP);

static OTG: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE,
    ".otg",
    otg,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.graphics-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODG);

static SXC: MimeType = MimeType::new(APPLICATION_VND_SUN_XML_CALC, ".sxc", sxc, &[])
    .with_kind(MimeKind::SPREADSHEET)
    .with_parent(&ZIP);

static KMZ: MimeType = MimeType::new(APPLICATION_VND_GOOGLE_EARTH_KMZ, ".kmz", kmz, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&ZIP);

// ============================================================================
// DATABASE FORMATS
// ============================================================================

static MDB: MimeType =
    MimeType::new(APPLICATION_X_MSACCESS, ".mdb", mdb, &[]).with_kind(MimeKind::DATABASE);

static ACCDB: MimeType =
    MimeType::new(APPLICATION_X_MSACCESS, ".accdb", accdb, &[]).with_kind(MimeKind::DATABASE);

static DBF: MimeType =
    MimeType::new(APPLICATION_X_DBF, ".dbf", dbf, &[]).with_kind(MimeKind::DATABASE);

static LOTUS123: MimeType = MimeType::new(APPLICATION_VND_LOTUS_1_2_3, ".123", lotus123, &[])
    .with_kind(MimeKind::SPREADSHEET.union(MimeKind::DATABASE));

static MRC: MimeType = MimeType::new(APPLICATION_MARC, ".mrc", marc, &[])
    .with_kind(MimeKind::TEXT.union(MimeKind::DATABASE));

// ============================================================================
// PROGRAMMING & TEXT FORMATS
// ============================================================================

static PHP: MimeType = MimeType::new(TEXT_X_PHP, ".php", php, &[]).with_parent(&UTF8);

static JAVASCRIPT: MimeType = MimeType::new(TEXT_JAVASCRIPT, ".js", javascript, &[])
    .with_aliases(&[APPLICATION_JAVASCRIPT])
    .with_parent(&UTF8);

static PYTHON: MimeType = MimeType::new(TEXT_X_PYTHON, ".py", python, &[])
    .with_aliases(&[TEXT_X_SCRIPT_PYTHON, APPLICATION_X_PYTHON])
    .with_parent(&UTF8);

static PERL: MimeType = MimeType::new(TEXT_X_PERL, ".pl", perl, &[]).with_parent(&UTF8);

static RUBY: MimeType = MimeType::new(TEXT_X_RUBY, ".rb", ruby, &[])
    .with_aliases(&[APPLICATION_X_RUBY])
    .with_parent(&UTF8);

static LUA: MimeType = MimeType::new(TEXT_X_LUA, ".lua", lua, &[]).with_parent(&UTF8);

static SHELL: MimeType = MimeType::new(TEXT_X_SHELLSCRIPT, ".sh", shell, &[])
    .with_aliases(&[TEXT_X_SH, APPLICATION_X_SHELLSCRIPT, APPLICATION_X_SH])
    .with_parent(&UTF8);

static TCL: MimeType = MimeType::new(TEXT_X_TCL, ".tcl", tcl, &[])
    .with_aliases(&[APPLICATION_X_TCL])
    .with_parent(&UTF8);

static JSON: MimeType = MimeType::new(
    APPLICATION_JSON,
    ".json",
    json,
    &[&GEOJSON, &NDJSON, &HAR, &GLTF],
)
.with_parent(&UTF8);

static GEOJSON: MimeType =
    MimeType::new(APPLICATION_GEO_JSON, ".geojson", geojson, &[]).with_parent(&JSON);

static NDJSON: MimeType =
    MimeType::new(APPLICATION_X_NDJSON, ".ndjson", ndjson, &[]).with_parent(&JSON);

static CSV_FORMAT: MimeType = MimeType::new(TEXT_CSV, ".csv", csv_format, &[]).with_parent(&UTF8);

static TSV: MimeType =
    MimeType::new(TEXT_TAB_SEPARATED_VALUES, ".tsv", tsv, &[]).with_parent(&UTF8);

static RTF: MimeType = MimeType::new(TEXT_RTF, ".rtf", rtf, &[])
    .with_aliases(&[APPLICATION_RTF])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&UTF8);

static SRT: MimeType = MimeType::new(APPLICATION_X_SUBRIP, ".srt", srt, &[])
    .with_aliases(&[APPLICATION_X_SRT, TEXT_X_SRT])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&UTF8);

static VTT: MimeType = MimeType::new(TEXT_VTT, ".vtt", vtt, &[]).with_parent(&UTF8);

static VCARD: MimeType = MimeType::new(TEXT_VCARD, ".vcf", vcard, &[]).with_parent(&UTF8);

static ICALENDAR: MimeType =
    MimeType::new(TEXT_CALENDAR, ".ics", icalendar, &[]).with_parent(&UTF8);

static SVG: MimeType = MimeType::new(IMAGE_SVG_XML, ".svg", svg, &[])
    .with_kind(MimeKind::IMAGE)
    .with_parent(&XML);

// ============================================================================
// XML-BASED FORMATS
// ============================================================================

static RSS: MimeType = MimeType::new(APPLICATION_RSS_XML, ".rss", rss, &[])
    .with_aliases(&[TEXT_RSS])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static ATOM: MimeType = MimeType::new(APPLICATION_ATOM_XML, ".atom", atom, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static X3D: MimeType = MimeType::new(MODEL_X3D_XML, ".x3d", x3d, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static KML: MimeType = MimeType::new(APPLICATION_VND_GOOGLE_EARTH_KML_XML, ".kml", kml, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static XLIFF: MimeType = MimeType::new(APPLICATION_X_XLIFF_XML, ".xlf", xliff, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static COLLADA: MimeType = MimeType::new(MODEL_VND_COLLADA_XML, ".dae", collada, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&XML);

static GML: MimeType = MimeType::new(APPLICATION_GML_XML, ".gml", gml, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static GPX: MimeType = MimeType::new(APPLICATION_GPX_XML, ".gpx", gpx, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static TCX: MimeType = MimeType::new(APPLICATION_VND_GARMIN_TCX_XML, ".tcx", tcx, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static AMF: MimeType = MimeType::new(APPLICATION_X_AMF, ".amf", amf, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&XML);

static THREEMF: MimeType = MimeType::new(
    APPLICATION_VND_MS_PACKAGE_3DMANUFACTURING_3DMODEL_XML,
    ".3mf",
    threemf,
    &[],
)
.with_kind(MimeKind::MODEL)
.with_parent(&XML);

static XFDF: MimeType = MimeType::new(APPLICATION_VND_ADOBE_XFDF, ".xfdf", xfdf, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static OWL2: MimeType = MimeType::new(APPLICATION_OWL_XML, ".owl", owl2, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static XHTML: MimeType = MimeType::new(APPLICATION_XHTML_XML, ".html", xhtml, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static HAR: MimeType = MimeType::new(APPLICATION_JSON_HAR, ".har", har, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&JSON);

// ============================================================================
// 3D & GEOSPATIAL FORMATS
// ============================================================================

static SHP: MimeType = MimeType::new(APPLICATION_VND_SHP, ".shp", shp, &[]);

static SHX: MimeType = MimeType::new(APPLICATION_VND_SHX, ".shx", shx, &[&SHP]);

static GLB: MimeType =
    MimeType::new(MODEL_GLTF_BINARY, ".glb", glb, &[]).with_kind(MimeKind::MODEL);

static GLTF: MimeType = MimeType::new(MODEL_GLTF_JSON, ".gltf", gltf, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&JSON);

// ============================================================================
// GAMING FORMATS
// ============================================================================

static NES: MimeType = MimeType::new(APPLICATION_VND_NINTENDO_SNES_ROM, ".nes", nes, &[]);

// ============================================================================
// ADDITIONAL VIDEO FORMATS
// ============================================================================

static THREE_GPP: MimeType = MimeType::new(VIDEO_3GPP, ".3gp", three_gpp, &[])
    .with_aliases(&[VIDEO_3GP, AUDIO_3GPP])
    .with_kind(MimeKind::VIDEO);

static THREE_GPP2: MimeType = MimeType::new(VIDEO_3GPP2, ".3g2", three_gpp2, &[])
    .with_aliases(&[VIDEO_3G2, AUDIO_3GPP2])
    .with_kind(MimeKind::VIDEO);

static MJ2: MimeType = MimeType::new(VIDEO_MJ2, ".mj2", mj2, &[]);

static DVB: MimeType =
    MimeType::new(VIDEO_VND_DVB_FILE, ".dvb", dvb, &[]).with_kind(MimeKind::VIDEO);

static AVIF_FORMAT: MimeType =
    MimeType::new(IMAGE_AVIF, ".avif", avif_format, &[]).with_kind(MimeKind::IMAGE);

// ============================================================================
// MISCELLANEOUS FORMATS
// ============================================================================

static HDF: MimeType =
    MimeType::new(APPLICATION_X_HDF, ".hdf", hdf, &[]).with_kind(MimeKind::DATABASE);

static CBOR_FORMAT: MimeType = MimeType::new(APPLICATION_CBOR, ".cbor", cbor_format, &[]);

static PARQUET: MimeType = MimeType::new(APPLICATION_VND_APACHE_PARQUET, ".parquet", parquet, &[])
    .with_aliases(&[APPLICATION_X_PARQUET])
    .with_kind(MimeKind::DATABASE);

static LNK: MimeType = MimeType::new(APPLICATION_X_MS_SHORTCUT, ".lnk", lnk, &[]);

static MACHO: MimeType =
    MimeType::new(APPLICATION_X_MACH_BINARY, ".macho", macho, &[]).with_kind(MimeKind::EXECUTABLE);

static TZIF: MimeType = MimeType::new(APPLICATION_TZIF, "", tzif, &[]);

// ============================================================================
// XML FORMAT DETECTION FUNCTIONS
// ============================================================================

fn rss(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<rss")
}

fn atom(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<feed")
}

fn x3d(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<X3D")
}

fn kml(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<kml")
}

fn xliff(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<xliff")
}

fn collada(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<COLLADA")
}

fn gml(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<gml")
}

fn gpx(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<gpx")
}

fn tcx(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"TrainingCenterDataba")
}

fn amf(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<amf")
}

fn threemf(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<model")
}

fn xfdf(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<xfdf")
}

fn owl2(input: &[u8]) -> bool {
    xml(input) && (input.windows(4).any(|w| w == b"<owl") || input.windows(4).any(|w| w == b"<RDF"))
}

fn xhtml(input: &[u8]) -> bool {
    xml(input)
        && input
            .windows(26)
            .any(|w| w == b"http://www.w3.org/1999/xht")
}

fn har(input: &[u8]) -> bool {
    json(input)
        && input.windows(5).any(|w| w == b"\"log\"")
        && input.windows(9).any(|w| w == b"\"version\"")
}

// ============================================================================
// INITIALIZATION FUNCTION
// ============================================================================

/// Initializes the MIME type detection tree by registering all supported formats.
///
/// This function is called automatically on first use through lazy initialization.
/// It registers all MIME types in a specific order to ensure optimal detection:
///
/// 1. **Text formats** - Highest priority for quick text file identification
/// 2. **Documents** - Common document formats like PDF, PostScript
/// 3. **Archives** - ZIP, TAR, 7Z and other archive formats
/// 4. **Images** - Wide variety of image formats from PNG to specialized formats
/// 5. **Audio** - Music and audio formats including lossy and lossless
/// 6. **Video** - Video containers and codecs
/// 7. **Executables** - Binary executables for different platforms
/// 8. **Fonts** - Web and desktop font formats
/// 9. **Web & Multimedia** - Browser extensions and multimedia formats
/// 10. **Specialized** - Medical, CAD, database and other specialized formats
/// 11. **Generic UTF-8** - Lowest priority fallback for any remaining text
///
/// The registration order is critical for performance and accuracy, as the
/// detection algorithm stops at the first successful match.
pub fn init_tree() {
    // Register ROOT and all its children recursively
    ROOT.register();
}

// ============================================================================
// PRIVATE MATCHER FUNCTIONS
// ============================================================================
//
// These functions implement the actual magic number detection logic for each
// supported format. They are organized alphabetically within each category
// and focus on reliable signature detection while minimizing false positives.
//
// Key principles:
// - Check minimum length before accessing bytes
// - Use distinctive magic numbers when available
// - Implement enhanced detection for complex formats
// - Prioritize performance while maintaining accuracy

/// Detects 7-Zip archives by their distinctive 6-byte signature.
///
/// 7Z files begin with the signature "7z¼¯'⁴" (0x377ABCAF271C).
/// This is one of the most reliable format signatures available.
fn seven_z(input: &[u8]) -> bool {
    input.starts_with(b"7z\xbc\xaf\x27\x1c")
}

/// Detects ZIP archives and ZIP-based formats.
///
/// ZIP files use the "PK" signature (named after Phil Katz) followed by
/// different headers for various ZIP record types:
/// - PK\x03\x04: Local file header (most common)
/// - PK\x05\x06: End of central directory record
/// - PK\x07\x08: Data descriptor record
///
/// This also matches ZIP-based formats like DOCX, XLSX, EPUB, JAR, etc.
fn zip(input: &[u8]) -> bool {
    input.starts_with(b"PK\x03\x04")
        || input.starts_with(b"PK\x05\x06")
        || input.starts_with(b"PK\x07\x08")
}

fn pdf(input: &[u8]) -> bool {
    input.starts_with(b"%PDF-")
}

fn gzip(input: &[u8]) -> bool {
    input.starts_with(b"\x1f\x8b")
}

fn bz2(input: &[u8]) -> bool {
    input.starts_with(b"BZ")
}

fn xz(input: &[u8]) -> bool {
    input.starts_with(b"\xfd7zXZ\x00")
}

fn png(input: &[u8]) -> bool {
    input.starts_with(b"\x89PNG\r\n\x1a\n")
}

fn jpg(input: &[u8]) -> bool {
    input.starts_with(b"\xff\xd8\xff")
}

fn gif(input: &[u8]) -> bool {
    input.starts_with(b"GIF87a") || input.starts_with(b"GIF89a")
}

fn webp(input: &[u8]) -> bool {
    input.len() >= 12 && input.starts_with(b"RIFF") && &input[8..12] == b"WEBP"
}

fn tiff(input: &[u8]) -> bool {
    input.starts_with(b"II*\x00") || input.starts_with(b"MM\x00*")
}

fn bmp(input: &[u8]) -> bool {
    input.starts_with(b"BM")
}

fn ico(input: &[u8]) -> bool {
    input.starts_with(b"\x00\x00\x01\x00")
}

fn flac(input: &[u8]) -> bool {
    input.starts_with(b"fLaC")
}

fn wav(input: &[u8]) -> bool {
    input.len() >= 12 && input.starts_with(b"RIFF") && &input[8..12] == b"WAVE"
}

fn exe(input: &[u8]) -> bool {
    input.starts_with(b"MZ")
}

fn elf(input: &[u8]) -> bool {
    input.starts_with(b"\x7fELF")
}

// ============================================================================
// TEXT ENCODING DETECTION
// ============================================================================

/// Detects UTF-8 encoded text using WHATWG binary-vs-text classification.
///
/// This function implements the WHATWG algorithm for distinguishing binary
/// from text content:
/// 1. Checks for UTF BOM markers first
/// 2. Scans for binary control characters (0x00-0x08, 0x0B, 0x0E-0x1A, 0x1C-0x1F)
/// 3. Validates UTF-8 encoding correctness
///
/// This is used as the lowest-priority fallback for any remaining text content.
fn utf8(input: &[u8]) -> bool {
    if input.is_empty() {
        return false;
    }

    // Check for UTF BOMs first
    if utf8_bom(input) || utf16_be(input) || utf16_le(input) {
        return true;
    }

    // Check for binary content using WHATWG algorithm
    for &byte in input {
        match byte {
            0x00..=0x08 | 0x0B | 0x0E..=0x1A | 0x1C..=0x1F => return false,
            _ => {}
        }
    }

    std::str::from_utf8(input).is_ok()
}

fn utf8_bom(input: &[u8]) -> bool {
    input.starts_with(b"\xEF\xBB\xBF")
}

fn utf16_be(input: &[u8]) -> bool {
    input.starts_with(b"\xFE\xFF")
}

fn utf16_le(input: &[u8]) -> bool {
    input.starts_with(b"\xFF\xFE")
}

/// Detects HTML documents with sophisticated tag analysis.
///
/// This function implements enhanced HTML detection that:
/// - Skips leading whitespace (WHATWG compliant)
/// - Performs case-insensitive tag matching
/// - Validates proper tag termination
/// - Handles DOCTYPE declarations and comments
/// - Supports common HTML tags including HTML5 elements
///
/// The detection is more robust than simple string matching and follows
/// the WHATWG MIME Sniffing Standard for accurate HTML identification.
fn html(input: &[u8]) -> bool {
    // Use lowercase tags for efficient case-insensitive comparison with eq_ignore_ascii_case
    const HTML_TAGS_LOWER: &[&[u8]] = &[
        b"<!doctype html",
        b"<html",
        b"<head",
        b"<script",
        b"<iframe",
        b"<h1",
        b"<div",
        b"<font",
        b"<table",
        b"<a",
        b"<style",
        b"<title",
        b"<b",
        b"<body",
        b"<br",
        b"<p",
    ];

    let input = input.trim_ascii_start();
    for &tag in HTML_TAGS_LOWER {
        if case_insensitive_starts_with(input, tag) {
            // Check for proper tag termination if there are more bytes
            if input.len() > tag.len() {
                let byte = input[tag.len()];
                if byte == b' ' || byte == b'>' {
                    return true;
                }
            } else {
                // Tag matches exactly at end of input - rare but valid for some cases
                if tag == b"<!--" {
                    return true;
                }
                // For regular tags, we need proper termination
            }
        }
    }
    false
}

fn xml(input: &[u8]) -> bool {
    let input = input.trim_ascii_start();
    input.starts_with(b"<?xml")
}

fn wasm(input: &[u8]) -> bool {
    input.starts_with(b"\x00asm")
}

fn aiff(input: &[u8]) -> bool {
    input.len() >= 12 && input.starts_with(b"FORM") && &input[8..12] == b"AIFF"
}

fn eot(input: &[u8]) -> bool {
    if input.len() < 36 {
        return false;
    }
    // 34 NULL bytes followed by "LP"
    input[0..34].iter().all(|&b| b == 0) && &input[34..36] == b"LP"
}

fn ttc(input: &[u8]) -> bool {
    input.starts_with(b"ttcf")
}

fn mp4_precise(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }

    let box_size = u32::from_be_bytes([input[0], input[1], input[2], input[3]]) as usize;
    if input.len() < box_size || box_size % 4 != 0 || box_size < 12 {
        return false;
    }

    // Detect all ISOBMFF files (MP4, 3GPP, etc.) by checking for ftyp box
    &input[4..8] == b"ftyp"
}

fn midi(input: &[u8]) -> bool {
    input.starts_with(b"MThd")
}

fn ole(input: &[u8]) -> bool {
    input.starts_with(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1")
}

fn ps(input: &[u8]) -> bool {
    input.starts_with(b"%!PS-Adobe-")
}

fn psd(input: &[u8]) -> bool {
    input.starts_with(b"8BPS")
}

fn pbm(input: &[u8]) -> bool {
    input.starts_with(b"P1") || input.starts_with(b"P4")
}

fn pgm(input: &[u8]) -> bool {
    input.starts_with(b"P2") || input.starts_with(b"P5")
}

fn ppm(input: &[u8]) -> bool {
    input.starts_with(b"P3") || input.starts_with(b"P6")
}

fn pam(input: &[u8]) -> bool {
    input.starts_with(b"P7")
}

fn ogg(input: &[u8]) -> bool {
    input.starts_with(b"OggS")
}

fn ogg_audio(input: &[u8]) -> bool {
    if input.len() < 37 {
        return false;
    }

    // Check for audio codecs at offset 28
    let offset_28 = &input[28..];
    offset_28.starts_with(b"\x7fFLAC")
        || offset_28.starts_with(b"\x01vorbis")
        || offset_28.starts_with(b"OpusHead")
        || offset_28.starts_with(b"Speex   ")
}

fn ogg_video(input: &[u8]) -> bool {
    if input.len() < 37 {
        return false;
    }

    // Check for video codecs at offset 28
    let offset_28 = &input[28..];
    offset_28.starts_with(b"\x80theora")
        || offset_28.starts_with(b"fishead\x00")
        || offset_28.starts_with(b"\x01video\x00\x00\x00") // OGM video
}

fn class(input: &[u8]) -> bool {
    input.starts_with(b"\xca\xfe\xba\xbe")
}

fn swf(input: &[u8]) -> bool {
    input.starts_with(b"FWS") || input.starts_with(b"CWS") || input.starts_with(b"ZWS")
}

fn woff(input: &[u8]) -> bool {
    input.starts_with(b"wOFF")
}

fn woff2(input: &[u8]) -> bool {
    input.starts_with(b"wOF2")
}

fn ttf(input: &[u8]) -> bool {
    input.starts_with(b"\x00\x01\x00\x00")
        || input.starts_with(b"true")
        || input.starts_with(b"typ1")
}

fn otf(input: &[u8]) -> bool {
    input.starts_with(b"OTTO")
}

fn ar(input: &[u8]) -> bool {
    input.starts_with(b"!<arch>")
}

fn dcm(input: &[u8]) -> bool {
    if input.len() < 132 {
        return false;
    }
    &input[128..132] == b"DICM"
}

fn rar(input: &[u8]) -> bool {
    input.starts_with(b"Rar!\x1a\x07\x00") || input.starts_with(b"Rar!\x1a\x07\x01\x00")
}

fn djvu(input: &[u8]) -> bool {
    input.starts_with(b"AT&TFORM") && input.len() >= 12 && &input[12..16] == b"DJVU"
}

fn mobi(input: &[u8]) -> bool {
    if input.len() < 68 {
        return false;
    }
    &input[60..64] == b"BOOKMOBI"
}

fn lit(input: &[u8]) -> bool {
    input.starts_with(b"ITOLITLS")
}

fn sqlite(input: &[u8]) -> bool {
    input.starts_with(b"SQLite format 3\x00")
}

fn icns(input: &[u8]) -> bool {
    input.starts_with(b"icns")
}

fn heic(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && (&input[8..12] == b"heic" || &input[8..12] == b"heix")
}

fn heic_sequence(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && &input[8..12] == b"hevc"
}

fn heif(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && (&input[8..12] == b"mif1" || &input[8..12] == b"msf1")
}

fn heif_sequence(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && &input[8..12] == b"msf1"
}

fn cab(input: &[u8]) -> bool {
    input.starts_with(b"MSCF")
}

fn rpm(input: &[u8]) -> bool {
    input.starts_with(b"\xed\xab\xee\xdb")
}

fn lzip(input: &[u8]) -> bool {
    input.starts_with(b"LZIP")
}

fn torrent(input: &[u8]) -> bool {
    input.starts_with(b"d8:announce")
        || input.starts_with(b"d7:comment")
        || input.starts_with(b"d4:info")
}

fn cpio(input: &[u8]) -> bool {
    if input.len() < 6 {
        return false;
    }
    // Binary CPIO formats
    if input.len() >= 2 {
        let magic = u16::from_le_bytes([input[0], input[1]]);
        // Old binary format: 070707 (octal) = 0x71C7
        // Also check 0xC7C7 which appears in some binary CPIO variants
        if magic == 0o70707 || magic == 0xC7C7 {
            return true;
        }
    }
    // ASCII CPIO variants
    input.starts_with(b"070701") || input.starts_with(b"070702") || input.starts_with(b"070707")
}

fn fasoo(input: &[u8]) -> bool {
    input.starts_with(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1")
        && input.len() > 520
        && &input[512..520] == b"FASOO   "
}

fn pgp_net_share(input: &[u8]) -> bool {
    input.starts_with(b"-----BEGIN PGP")
}

fn quicktime(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && &input[8..12] == b"qt  "
}

fn mqv(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && &input[8..12] == b"mqt "
}

fn flv(input: &[u8]) -> bool {
    input.starts_with(b"FLV")
}

fn asf(input: &[u8]) -> bool {
    input.starts_with(b"\x30\x26\xb2\x75\x8e\x66\xcf\x11\xa6\xd9\x00\xaa\x00\x62\xce\x6c")
}

fn amp4(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && &input[8..12] == b"M4A "
}

fn m4a(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && &input[8..12] == b"M4A "
}

fn m4v(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    &input[4..8] == b"ftyp" && &input[8..12] == b"M4V "
}

// Additional image format detectors from new Go implementation
fn apng(input: &[u8]) -> bool {
    if input.len() < 41 {
        return false;
    }
    // Check for PNG signature first, then look for acTL chunk at offset 37
    png(input) && &input[37..41] == b"acTL"
}

fn jp2(input: &[u8]) -> bool {
    jpeg2k(input, b"jp2 ")
}

fn jpx(input: &[u8]) -> bool {
    jpeg2k(input, b"jpx ")
}

fn jpm(input: &[u8]) -> bool {
    jpeg2k(input, b"jpm ")
}

fn jpeg2k(input: &[u8], sig: &[u8]) -> bool {
    if input.len() < 24 {
        return false;
    }

    // Check for JPEG 2000 signature box
    if &input[4..8] != b"jP  " && &input[4..8] != b"jP2 " {
        return false;
    }

    &input[20..24] == sig
}

fn bpg(input: &[u8]) -> bool {
    input.starts_with(b"BPG\xFB")
}

fn xcf(input: &[u8]) -> bool {
    input.starts_with(b"gimp xcf")
}

fn pat(input: &[u8]) -> bool {
    input.len() > 24 && &input[20..24] == b"GPAT"
}

fn gbr(input: &[u8]) -> bool {
    input.len() > 24 && &input[20..24] == b"GIMP"
}

fn hdr(input: &[u8]) -> bool {
    input.starts_with(b"#?RADIANCE\n")
}

fn xpm(input: &[u8]) -> bool {
    input.starts_with(b"/* XPM */")
}

fn jxs(input: &[u8]) -> bool {
    input.starts_with(b"\x00\x00\x00\x0C\x4A\x58\x53\x20\x0D\x0A\x87\x0A")
}

fn jxr(input: &[u8]) -> bool {
    input.starts_with(b"\x49\x49\xBC\x01")
}

fn jxl(input: &[u8]) -> bool {
    input.starts_with(b"\xFF\x0A") || input.starts_with(b"\x00\x00\x00\x0CJXL \x0D\x0A\x87\x0A")
}

// Enhanced DWG detection with more versions
fn dwg(input: &[u8]) -> bool {
    if input.len() < 6 || input[0] != 0x41 || input[1] != 0x43 {
        return false;
    }

    const DWG_VERSIONS: [&[u8; 4]; 15] = [
        b"1.40", b"1.50", b"2.10", b"1002", b"1003", b"1004", b"1006", b"1009", b"1012", b"1014",
        b"1015", b"1018", b"1021", b"1024", b"1032",
    ];

    let ver = &input[2..6];
    DWG_VERSIONS.iter().any(|version| ver.eq(*version))
}

// DXF (Drawing Exchange Format) detection
fn dxf(input: &[u8]) -> bool {
    // DXF files are ASCII text files with various headers
    // Check for common DXF signatures
    input.starts_with(b"  0\x0ASECTION\x0A")
        || input.starts_with(b"  0\x0D\x0ASECTION\x0D\x0A")
        || input.starts_with(b"0\x0ASECTION\x0A")
        || input.starts_with(b"0\x0D\x0ASECTION\x0D\x0A")
}

// WordPerfect document detection
fn wpd(input: &[u8]) -> bool {
    if input.len() < 10 {
        return false;
    }
    if !input.starts_with(b"\xffWPC") {
        return false;
    }
    input[8] == 1 && input[9] == 10
}

// Additional audio format detectors
fn ape(input: &[u8]) -> bool {
    input.starts_with(b"MAC \x96\x0F\x00\x00\x34\x00\x00\x00\x18\x00\x00\x00\x90\xE3")
}

fn musepack(input: &[u8]) -> bool {
    input.starts_with(b"MPCK")
}

fn au(input: &[u8]) -> bool {
    input.starts_with(b".snd")
}

fn amr(input: &[u8]) -> bool {
    input.starts_with(b"#!AMR")
}

fn voc(input: &[u8]) -> bool {
    input.starts_with(b"Creative Voice File")
}

fn m3u(input: &[u8]) -> bool {
    input.starts_with(b"#EXTM3U")
}

fn aac(input: &[u8]) -> bool {
    input.starts_with(b"\xFF\xF1") || input.starts_with(b"\xFF\xF9")
}

fn qcp(input: &[u8]) -> bool {
    input.len() >= 12 && input.starts_with(b"RIFF") && &input[8..12] == b"QLCM"
}

// Enhanced MP3 detection
// ============================================================================
// ENHANCED AUDIO DETECTION
// ============================================================================

/// Enhanced MP3 detection supporting multiple frame types and ID3 tags.
///
/// This function detects MP3 files by:
/// 1. Checking for ID3v2 tags at the beginning
/// 2. Validating MPEG audio frame sync patterns
/// 3. Supporting multiple MPEG versions and layers
///
/// The enhanced algorithm reduces false positives while maintaining
/// compatibility with various MP3 encoding methods.
fn mp3(input: &[u8]) -> bool {
    if input.len() < 3 {
        return false;
    }

    if input.starts_with(b"ID3") {
        return true;
    }

    // Check for MPEG audio frame headers
    let header = u16::from_be_bytes([input[0], input[1]]) & 0xFFFE;
    matches!(header, 0xFFFA | 0xFFF2 | 0xFFE2)
}

// Additional video format detectors
fn rmvb(input: &[u8]) -> bool {
    input.starts_with(b".RMF")
}

fn webm(input: &[u8]) -> bool {
    if !input.starts_with(b"\x1A\x45\xDF\xA3") {
        return false;
    }
    is_matroska_file_type(input, b"webm")
}

fn mkv(input: &[u8]) -> bool {
    if !input.starts_with(b"\x1A\x45\xDF\xA3") {
        return false;
    }
    is_matroska_file_type(input, b"matroska")
}

fn is_matroska_file_type(input: &[u8], file_type: &[u8]) -> bool {
    let max_search = input.len().min(4096);
    if let Some(pos) = input[..max_search]
        .windows(2)
        .position(|w| w == b"\x42\x82")
    {
        let pos = pos + 2;
        if pos < input.len() {
            let n = vint_width(input[pos] as i32);
            if pos + n < input.len() {
                return input[pos + n..].starts_with(file_type);
            }
        }
    }
    false
}

fn vint_width(v: i32) -> usize {
    let mut mask = 128;
    let mut num = 1;
    while num < 8 && (v & mask) == 0 {
        mask >>= 1;
        num += 1;
    }
    num
}

fn mpeg(input: &[u8]) -> bool {
    input.len() > 3 && input.starts_with(b"\x00\x00\x01") && input[3] >= 0xB0 && input[3] <= 0xBF
}

fn avi(input: &[u8]) -> bool {
    input.len() > 16 && input.starts_with(b"RIFF") && &input[8..16] == b"AVI LIST"
}

// Additional archive format detectors
fn fits(input: &[u8]) -> bool {
    input.starts_with(b"SIMPLE  =                    T")
}

fn xar(input: &[u8]) -> bool {
    input.starts_with(b"xar!")
}

fn deb(input: &[u8]) -> bool {
    input.len() > 21 && &input[8..21] == b"debian-binary"
}

fn warc(input: &[u8]) -> bool {
    input.starts_with(b"WARC/1.0") || input.starts_with(b"WARC/1.1")
}

fn install_shield_cab(input: &[u8]) -> bool {
    input.len() > 7 && input.starts_with(b"ISc(") && input[6] == 0 && matches!(input[7], 1 | 2 | 4)
}

fn zstd(input: &[u8]) -> bool {
    if input.len() < 4 {
        return false;
    }

    let sig = u32::from_le_bytes([input[0], input[1], input[2], input[3]]);
    // Zstandard frames and skippable frames
    (0xFD2FB522..=0xFD2FB528).contains(&sig) || (0x184D2A50..=0x184D2A5F).contains(&sig)
}

fn crx(input: &[u8]) -> bool {
    if input.len() < 16 || !input.starts_with(b"Cr24") {
        return false;
    }

    let pubkey_len = u32::from_le_bytes([input[8], input[9], input[10], input[11]]) as usize;
    let sig_len = u32::from_le_bytes([input[12], input[13], input[14], input[15]]) as usize;
    let zip_offset = 16 + pubkey_len + sig_len;

    if input.len() < zip_offset {
        return false;
    }

    zip(&input[zip_offset..])
}

/// Detects TAR archives using header checksum validation.
///
/// TAR files don't have a distinctive magic number, so this function uses
/// checksum validation for reliable detection:
///
/// 1. Checks minimum 512-byte record size
/// 2. Excludes Gentoo GLEP binary packages (false positives)
/// 3. Parses the octal checksum from the header (bytes 148-155)
/// 4. Calculates both signed and unsigned checksums
/// 5. Validates the recorded checksum matches calculated values
///
/// This approach provides high accuracy while avoiding false positives
/// from other formats that might have similar byte patterns.
fn tar(input: &[u8]) -> bool {
    const RECORD_SIZE: usize = 512;

    if input.len() < RECORD_SIZE {
        return false;
    }

    let record = &input[..RECORD_SIZE];

    // Check for Gentoo GLEP binary package (exclude)
    if record[..100].windows(8).any(|w| w == b"/gpkg-1\x00") {
        return false;
    }

    // Parse checksum from header
    let checksum_bytes = &record[148..156];
    if let Some(recorded_checksum) = parse_octal(checksum_bytes) {
        let (unsigned_sum, signed_sum) = tar_checksum(record);
        recorded_checksum == unsigned_sum || recorded_checksum == signed_sum
    } else {
        false
    }
}

/// Parses an octal number from a byte slice.
///
/// Used by TAR checksum validation to parse the octal checksum field.
/// Handles leading/trailing whitespace and null bytes commonly found
/// in TAR headers.
fn parse_octal(bytes: &[u8]) -> Option<i64> {
    let trimmed: Vec<u8> = bytes
        .iter()
        .skip_while(|&&b| b == b' ' || b == 0)
        .take_while(|&&b| b != b' ' && b != 0)
        .copied()
        .collect();

    if trimmed.is_empty() {
        return None;
    }

    let mut result = 0i64;
    for &byte in &trimmed {
        if !(b'0'..=b'7').contains(&byte) {
            return None;
        }
        result = (result << 3) | ((byte - b'0') as i64);
    }
    Some(result)
}

/// Calculates TAR header checksums in both signed and unsigned variants.
///
/// TAR archives store a checksum in the header that some implementations
/// calculate as signed bytes and others as unsigned. This function returns
/// both variants to handle all TAR implementations correctly.
///
/// The checksum field itself (bytes 148-155) is treated as spaces during
/// calculation.
fn tar_checksum(record: &[u8]) -> (i64, i64) {
    let mut unsigned_sum = 0i64;
    let mut signed_sum = 0i64;

    for (i, &byte) in record.iter().enumerate() {
        let c = if (148..156).contains(&i) { b' ' } else { byte };
        unsigned_sum += c as i64;
        signed_sum += (c as i8) as i64;
    }

    (unsigned_sum, signed_sum)
}

// ============================================================================
// MICROSOFT OFFICE & DOCUMENT FORMAT DETECTORS
// ============================================================================

/// Microsoft Office 2007+ formats are ZIP archives with specific internal structure
fn docx(input: &[u8]) -> bool {
    msoxml(input, &[(b"word/", true)], 100)
}

fn xlsx(input: &[u8]) -> bool {
    msoxml(input, &[(b"xl/", true)], 100)
}

fn pptx(input: &[u8]) -> bool {
    msoxml(input, &[(b"ppt/", true)], 100)
}

fn vsdx(input: &[u8]) -> bool {
    msoxml(input, &[(b"visio/", true)], 100)
}

fn epub(input: &[u8]) -> bool {
    // EPUB uses offset-based detection like Go implementation
    // Go: Epub = offset([]byte("mimetypeapplication/epub+zip"), 30)
    if input.len() < 30 + 28 {
        return false;
    }
    let expected = b"mimetypeapplication/epub+zip";
    &input[30..30 + expected.len()] == expected
}

fn jar(input: &[u8]) -> bool {
    executable_jar(input)
        || zip_has(
            input,
            &[(b"META-INF/MANIFEST.MF", false), (b"META-INF/", true)],
            1,
        )
}

/// An executable Jar has a 0xCAFE flag enabled in the first zip entry.
/// The rule from file/file is:
/// >(26.s+30) leshort 0xcafe Java archive data (JAR)
fn executable_jar(input: &[u8]) -> bool {
    if input.len() < 30 {
        return false;
    }

    // Advance to position 0x1A (26)
    let offset_pos = 26;
    if offset_pos + 2 > input.len() {
        return false;
    }

    // Read uint16 offset (little-endian)
    let offset = u16::from_le_bytes([input[offset_pos], input[offset_pos + 1]]) as usize;

    // Advance by offset + 2 from position 30 (after ZIP header)
    let cafe_pos = 30 + offset;
    if cafe_pos + 2 > input.len() {
        return false;
    }

    // Read uint16 and check if it equals 0xCAFE
    let cafe_value = u16::from_le_bytes([input[cafe_pos], input[cafe_pos + 1]]);
    cafe_value == 0xCAFE
}

fn apk(input: &[u8]) -> bool {
    zip_has(
        input,
        &[
            (b"AndroidManifest.xml", false),
            (
                b"META-INF/com/android/build/gradle/app-metadata.properties",
                false,
            ),
            (b"classes.dex", false),
            (b"resources.arsc", false),
            (b"res/drawable", true),
        ],
        100,
    )
}

/// OLE-based legacy Microsoft Office formats
fn doc(input: &[u8]) -> bool {
    if !ole(input) {
        return false;
    }

    // CLSID-only matching (matching Go implementation exactly)
    const WORD_97_2003_CLSID: &[u8] = &[
        0x06, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    const WORD_6_7_CLSID: &[u8] = &[
        0x00, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    const WORD_PICTURE_CLSID: &[u8] = &[
        0x07, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];

    const CLSIDS: [&[u8]; 3] = [WORD_97_2003_CLSID, WORD_6_7_CLSID, WORD_PICTURE_CLSID];

    for clsid in &CLSIDS {
        if ole_matches_clsid(input, clsid) {
            return true;
        }
    }

    false
}

fn xls(input: &[u8]) -> bool {
    if !ole(input) {
        return false;
    }

    // Try CLSID matching first (primary method from Go implementation)
    const EXCEL_V5_CLSID: &[u8] = &[0x10, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
    const EXCEL_V7_CLSID: &[u8] = &[0x20, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00];

    if ole_matches_clsid(input, EXCEL_V5_CLSID) || ole_matches_clsid(input, EXCEL_V7_CLSID) {
        return true;
    }

    let lin = input.len();

    // Check for XLS sub-headers at offset 512 (from Go implementation)
    if lin >= 520 {
        const XLS_SUB_HEADERS: [&[u8]; 7] = [
            &[0x09, 0x08, 0x10, 0x00, 0x00, 0x06, 0x05, 0x00],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x10],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x1F],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x22],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x23],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x28],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x29],
        ];

        for &header in &XLS_SUB_HEADERS {
            if input.len() > 512 + header.len() && input[512..].starts_with(header) {
                return true;
            }
        }
    }

    // Check for UTF-16 encoded "Workbook" string at offset 1152
    if lin > 1152 {
        let end = (lin).min(4096);
        let search_range = &input[1152..end];
        // UTF-16LE encoded "Workbook": W\x00k\x00s\x00S\x00S\x00W\x00o\x00r\x00k\x00B\x00o\x00o\x00k
        if search_range
            .windows(22)
            .any(|w| w == b"W\x00k\x00s\x00S\x00S\x00W\x00o\x00r\x00k\x00B\x00o\x00o\x00k")
        {
            return true;
        }
    }

    false
}

fn ppt(input: &[u8]) -> bool {
    if !ole(input) {
        return false;
    }

    // Try CLSID matching first (from Go implementation)
    const PPT_V4_CLSID: &[u8; 16] = &[
        0x10, 0x8d, 0x81, 0x64, 0x9b, 0x4f, 0xcf, 0x11, 0x86, 0xea, 0x00, 0xaa, 0x00, 0xb9, 0x29,
        0xe8,
    ];
    const PPT_V7_CLSID: &[u8; 16] = &[
        0x70, 0xae, 0x7b, 0xea, 0x3b, 0xfb, 0xcd, 0x11, 0xa9, 0x03, 0x00, 0xaa, 0x00, 0x51, 0x0e,
        0xa3,
    ];

    if ole_matches_clsid(input, PPT_V4_CLSID) || ole_matches_clsid(input, PPT_V7_CLSID) {
        return true;
    }

    let lin = input.len();
    if lin < 520 {
        return false;
    }

    // Check for PPT sub-headers at offset 512 (from Go implementation)
    const PPT_SUB_HEADERS: [&[u8]; 3] = [
        &[0xA0, 0x46, 0x1D, 0xF0],
        &[0x00, 0x6E, 0x1E, 0xF0],
        &[0x0F, 0x00, 0xE8, 0x03],
    ];

    for &header in &PPT_SUB_HEADERS {
        if input.len() > 512 + header.len() && input[512..].starts_with(header) {
            return true;
        }
    }

    // Check for specific PPT pattern
    if input.len() > 519
        && input[512..516] == [0xFD, 0xFF, 0xFF, 0xFF]
        && input[518] == 0x00
        && input[519] == 0x00
    {
        return true;
    }

    // Check for UTF-16 encoded "PowerPoint Document" string at offset 1152
    if lin > 1152 {
        let end = lin.min(4096);
        let search_range = &input[1152..end];
        // UTF-16LE encoded "PowerPoint Document": P\x00o\x00w\x00e\x00r\x00P\x00o\x00i\x00n\x00t\x00 D\x00o\x00c\x00u\x00m\x00e\x00n\x00t
        search_range.windows(38).any(|w| {
            w == b"P\x00o\x00w\x00e\x00r\x00P\x00o\x00i\x00n\x00t\x00 D\x00o\x00c\x00u\x00m\x00e\x00n\x00t"
        })
    } else {
        false
    }
}

fn pub_format(input: &[u8]) -> bool {
    const PUBLISHER_CLSID: &[u8; 16] = &[
        0x01, 0x12, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    detect_ole_with_clsid(input, PUBLISHER_CLSID)
}

fn msg(input: &[u8]) -> bool {
    const OUTLOOK_MSG_CLSID: &[u8; 16] = &[
        0x0B, 0x0D, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    detect_ole_with_clsid(input, OUTLOOK_MSG_CLSID)
}

fn chm(input: &[u8]) -> bool {
    input.starts_with(b"ITSF\x03\x00\x00\x00")
}

fn onenote(input: &[u8]) -> bool {
    const ONENOTE_CLSID: &[u8; 16] = &[
        0x43, 0xAD, 0x43, 0x36, 0x5E, 0x47, 0x96, 0x48, 0x8B, 0x42, 0x04, 0x40, 0xE7, 0x87, 0xC9,
        0x30,
    ];
    detect_ole_with_clsid(input, ONENOTE_CLSID)
}

fn msi(input: &[u8]) -> bool {
    const MSI_CLSID: &[u8; 16] = &[
        0x84, 0x10, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    detect_ole_with_clsid(input, MSI_CLSID)
}

// ============================================================================
// OPEN DOCUMENT FORMAT DETECTORS
// ============================================================================

fn odt(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.text")
}

fn ods(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.spreadsheet")
}

fn odp(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.presentation")
}

fn odg(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.graphics")
}

fn odf_format(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.formula")
}

fn odc(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.chart")
}

fn ott(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.text-template")
}

fn ots(input: &[u8]) -> bool {
    detect_opendocument_format(
        input,
        b"application/vnd.oasis.opendocument.spreadsheet-template",
    )
}

fn otp(input: &[u8]) -> bool {
    detect_opendocument_format(
        input,
        b"application/vnd.oasis.opendocument.presentation-template",
    )
}

fn otg(input: &[u8]) -> bool {
    detect_opendocument_format(
        input,
        b"application/vnd.oasis.opendocument.graphics-template",
    )
}

fn sxc(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.sun.xml.calc")
}

fn kmz(input: &[u8]) -> bool {
    // KMZ uses zip_has to look for doc.kml file like Go implementation
    // Go: KMZ returns zipHas(raw, zipEntries{{"doc.kml"}}, 100)
    zip_has(input, &[(b"doc.kml", false)], 100)
}

// ============================================================================
// DATABASE FORMAT DETECTORS
// ============================================================================

fn mdb(input: &[u8]) -> bool {
    if input.len() < 32 {
        return false;
    }
    &input[4..19] == b"Standard Jet DB"
}

fn accdb(input: &[u8]) -> bool {
    if input.len() < 32 {
        return false;
    }
    &input[4..19] == b"Standard ACE DB"
}

fn dbf(input: &[u8]) -> bool {
    if input.len() < 32 {
        return false;
    }
    // dBase file types - but must be followed by binary data, not text
    let is_dbf_type = matches!(
        input[0],
        0x02 | 0x03 | 0x04 | 0x05 | 0x30 | 0x31 | 0x32 | 0x83 | 0x8B | 0x8E | 0xF5
    );

    if !is_dbf_type {
        return false;
    }

    // Check that this looks like binary data, not text
    // DBF files have specific header structures with mostly binary data
    let has_text_chars = input[1..16]
        .iter()
        .any(|&b| (0x20..=0x7E).contains(&b) && b != 0x00);
    !has_text_chars
}

fn lotus123(input: &[u8]) -> bool {
    if input.len() < 8 {
        return false;
    }
    let version = u32::from_le_bytes([input[4], input[5], input[6], input[7]]);
    matches!(version, 0x00000200 | 0x00001a00)
}

fn marc(input: &[u8]) -> bool {
    if input.len() < 24 {
        return false;
    }
    // MARC leader validation
    input[10] == b'2' && input[11] == b'2' && &input[20..24] == b"4500"
}

// ============================================================================
// PROGRAMMING & TEXT FORMAT DETECTORS
// ============================================================================

fn php(input: &[u8]) -> bool {
    input.starts_with(b"<?php")
        || input.starts_with(b"<?\n")
        || input.starts_with(b"<?\r")
        || input.starts_with(b"<? ")
}

fn javascript(input: &[u8]) -> bool {
    // Check for shebang
    input.starts_with(b"#!/usr/bin/env node") || 
    input.starts_with(b"#!/usr/bin/node") ||
    // Check for common JS patterns at start
    input.starts_with(b"/*") ||
    input.starts_with(b"//") ||
    has_js_keywords(input)
}

fn python(input: &[u8]) -> bool {
    input.starts_with(b"#!/usr/bin/env python")
        || input.starts_with(b"#!/usr/bin/python")
        || input.starts_with(b"#!python")
        || input.starts_with(b"# -*- coding:")
}

fn perl(input: &[u8]) -> bool {
    input.starts_with(b"#!/usr/bin/env perl")
        || input.starts_with(b"#!/usr/bin/perl")
        || input.starts_with(b"#!perl")
}

fn ruby(input: &[u8]) -> bool {
    input.starts_with(b"#!/usr/bin/env ruby")
        || input.starts_with(b"#!/usr/bin/ruby")
        || input.starts_with(b"#!ruby")
}

fn lua(input: &[u8]) -> bool {
    input.starts_with(b"#!/usr/bin/env lua")
        || input.starts_with(b"#!/usr/bin/lua")
        || input.starts_with(b"#!lua")
        || input.starts_with(b"\x1bLua") // Lua bytecode
}

fn shell(input: &[u8]) -> bool {
    input.starts_with(b"#!/bin/sh")
        || input.starts_with(b"#!/bin/bash")
        || input.starts_with(b"#!/usr/bin/env bash")
        || input.starts_with(b"#!/bin/zsh")
}

fn tcl(input: &[u8]) -> bool {
    input.starts_with(b"#!/usr/bin/env tclsh")
        || input.starts_with(b"#!/usr/bin/tclsh")
        || input.starts_with(b"#!tclsh")
}

fn json(input: &[u8]) -> bool {
    let trimmed = input.trim_ascii_start();
    (trimmed.starts_with(b"{") || trimmed.starts_with(b"[")) && is_valid_json(trimmed)
}

fn geojson(input: &[u8]) -> bool {
    json(input)
        && input.windows(6).any(|w| w == b"\"type\"")
        && input.windows(19).any(|w| w == b"\"FeatureCollection\"")
        && input.windows(10).any(|w| w == b"\"features\"")
}

fn ndjson(input: &[u8]) -> bool {
    let lines = input.split(|&b| b == b'\n');
    let mut line_count = 0;
    let mut valid_lines = 0;

    for line in lines.take(3) {
        line_count += 1;
        if line.is_empty() || json(line) {
            valid_lines += 1;
        } else {
            return false;
        }
    }

    line_count > 1 && valid_lines == line_count
}

fn csv_format(input: &[u8]) -> bool {
    let lines = input.split(|&b| b == b'\n').take(5);
    detect_csv_generic(lines, |line| count_occurrences(line, b','))
}

fn tsv(input: &[u8]) -> bool {
    let lines = input.split(|&b| b == b'\n').take(5);
    detect_csv_generic(lines, |line| count_occurrences(line, b'\t'))
}

fn rtf(input: &[u8]) -> bool {
    input.starts_with(b"{\\rtf")
}

fn srt(input: &[u8]) -> bool {
    let text = input.trim_ascii_start();
    if text.starts_with(b"1\n") || text.starts_with(b"1\r\n") {
        // Look for timestamp pattern in the next line
        let mut lines = text.split(|&b| b == b'\n');

        // Skip first line (should be "1")
        lines.next();

        // Check second line for timestamp pattern
        if let Some(timestamp_line) = lines.next() {
            // Look for SRT timestamp pattern: 00:00:00,000 --> 00:00:00,000
            timestamp_line.windows(5).any(|w| w == b" --> ")
        } else {
            false
        }
    } else {
        false
    }
}

fn vtt(input: &[u8]) -> bool {
    if input.starts_with(b"WEBVTT") {
        // Check that it's followed by a line ending, space, or end of file
        if input.len() == 6 {
            return true;
        }
        matches!(input[6], b'\n' | b'\r' | b' ' | b'\t')
    } else if input.starts_with(b"\xEF\xBB\xBFWEBVTT") {
        // UTF-8 BOM + WEBVTT
        if input.len() == 9 {
            return true;
        }
        matches!(input[9], b'\n' | b'\r' | b' ' | b'\t')
    } else {
        false
    }
}

fn vcard(input: &[u8]) -> bool {
    case_insensitive_starts_with(input, b"BEGIN:VCARD")
}

fn icalendar(input: &[u8]) -> bool {
    case_insensitive_starts_with(input, b"BEGIN:VCALENDAR")
}

fn svg(input: &[u8]) -> bool {
    let trimmed = input.trim_ascii_start();
    if trimmed.starts_with(b"<?xml") {
        // Look for SVG namespace in XML
        trimmed.windows(4).any(|w| w == b"<svg")
            || trimmed
                .windows(26)
                .any(|w| w == b"http://www.w3.org/2000/svg")
    } else {
        trimmed.starts_with(b"<svg")
    }
}

// ============================================================================
// 3D & GEOSPATIAL FORMAT DETECTORS
// ============================================================================

fn shp(input: &[u8]) -> bool {
    if input.len() < 100 {
        return false;
    }
    // ESRI Shapefile header
    let file_code = u32::from_be_bytes([input[0], input[1], input[2], input[3]]);
    file_code == 9994
}

fn shx(input: &[u8]) -> bool {
    input.starts_with(b"\x00\x00\x27\x0A")
}

fn glb(input: &[u8]) -> bool {
    input.starts_with(b"glTF\x02\x00\x00\x00") || input.starts_with(b"glTF\x01\x00\x00\x00")
}

fn gltf(input: &[u8]) -> bool {
    json(input)
        && input.windows(8).any(|w| w == b"\"scenes\"")
        && input.windows(7).any(|w| w == b"\"nodes\"")
        && input.windows(7).any(|w| w == b"\"asset\"")
}

// ============================================================================
// GAMING FORMAT DETECTORS
// ============================================================================

fn nes(input: &[u8]) -> bool {
    input.starts_with(b"NES\x1A")
}

// ============================================================================
// ADDITIONAL VIDEO FORMAT DETECTORS
// ============================================================================

fn three_gpp(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    if &input[4..8] != b"ftyp" {
        return false;
    }

    let brand = &input[8..12];
    matches!(
        brand,
        b"3gp4" | b"3gp5" | b"3gp6" | b"3gp7" | b"3gp8" | b"3gp9" | b"3gpa" | b"3gpp"
    )
}

fn three_gpp2(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    if &input[4..8] != b"ftyp" {
        return false;
    }

    let brand = &input[8..12];
    matches!(
        brand,
        b"3g24" | b"3g25" | b"3g26" | b"3g27" | b"3g28" | b"3g29" | b"3g2a" | b"3g2b" | b"3g2c"
    )
}

fn mj2(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    if &input[4..8] != b"ftyp" {
        return false;
    }

    let brand = &input[8..12];
    matches!(brand, b"mj2s" | b"mjp2")
}

fn dvb(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    if &input[4..8] != b"ftyp" {
        return false;
    }

    &input[8..12] == b"dvb1"
}

fn avif_format(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }
    if &input[4..8] != b"ftyp" {
        return false;
    }

    let brand = &input[8..12];
    matches!(brand, b"avif" | b"avis")
}

// ============================================================================
// MISCELLANEOUS FORMAT DETECTORS
// ============================================================================

fn hdf(input: &[u8]) -> bool {
    input.starts_with(b"\x89HDF\r\n\x1a\n") || input.starts_with(b"\x0e\x03\x13\x01")
}

fn cbor_format(input: &[u8]) -> bool {
    input.starts_with(b"\xd9\xd9\xf7")
}

fn parquet(input: &[u8]) -> bool {
    input.starts_with(b"PAR1")
}

fn lnk(input: &[u8]) -> bool {
    input.starts_with(b"L\x00\x00\x00\x01\x14\x02\x00")
}

fn macho(input: &[u8]) -> bool {
    if input.len() < 4 {
        return false;
    }

    let magic = u32::from_le_bytes([input[0], input[1], input[2], input[3]]);
    matches!(
        magic,
        0xfeedface | 0xfeedfacf | 0xcafebabe | 0xcffaedfe | 0xcefaedfe
    )
}

fn tzif(input: &[u8]) -> bool {
    input.starts_with(b"TZif")
}

// ============================================================================
// UTF-16 FORMAT DETECTION FUNCTIONS
// ============================================================================

/// Helper function to skip UTF-16 BOM and convert to string
fn utf16_to_text(input: &[u8], big_endian: bool) -> Option<String> {
    // UTF-16 BOM constants
    const UTF16_BE_BOM: &[u8] = &[0xFE, 0xFF];
    const UTF16_LE_BOM: &[u8] = &[0xFF, 0xFE];

    // Skip BOM if present
    let content = if (big_endian && input.starts_with(UTF16_BE_BOM))
        || (!big_endian && input.starts_with(UTF16_LE_BOM))
    {
        &input[2..]
    } else {
        input
    };

    utf16_to_string(content, big_endian)
}

/// HTML detection for UTF-16 Big Endian
fn html_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_html_content)
}

/// HTML detection for UTF-16 Little Endian
fn html_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_html_content)
}

/// XML detection for UTF-16 Big Endian
fn xml_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_xml_content)
}

/// XML detection for UTF-16 Little Endian
fn xml_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_xml_content)
}

/// SVG detection for UTF-16 Big Endian
fn svg_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_svg_content)
}

/// SVG detection for UTF-16 Little Endian
fn svg_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_svg_content)
}

/// JSON detection for UTF-16 Big Endian
fn json_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_json_content)
}

/// JSON detection for UTF-16 Little Endian
fn json_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_json_content)
}

/// CSV detection for UTF-16 Big Endian
fn csv_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_csv_content)
}

/// CSV detection for UTF-16 Little Endian
fn csv_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_csv_content)
}

/// TSV detection for UTF-16 Big Endian
fn tsv_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_tsv_content)
}

/// TSV detection for UTF-16 Little Endian
fn tsv_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_tsv_content)
}

/// SRT subtitle detection for UTF-16 Big Endian
fn srt_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_srt_content)
}

/// SRT subtitle detection for UTF-16 Little Endian
fn srt_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_srt_content)
}

/// VTT subtitle detection for UTF-16 Big Endian
fn vtt_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_vtt_content)
}

/// VTT subtitle detection for UTF-16 Little Endian
fn vtt_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_vtt_content)
}

/// vCard detection for UTF-16 Big Endian
fn vcard_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_vcard_content)
}

/// vCard detection for UTF-16 Little Endian
fn vcard_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_vcard_content)
}

/// iCalendar detection for UTF-16 Big Endian
fn icalendar_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_icalendar_content)
}

/// iCalendar detection for UTF-16 Little Endian
fn icalendar_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_icalendar_content)
}

/// RTF detection for UTF-16 Big Endian
fn rtf_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_rtf_content)
}

/// RTF detection for UTF-16 Little Endian
fn rtf_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_rtf_content)
}

// ============================================================================
// SHARED CONTENT DETECTION FUNCTIONS (ENCODING-AGNOSTIC)
// ============================================================================

/// Shared HTML content detection that works with any encoding after normalization
fn detect_html_content(text: &str) -> bool {
    const HTML_TAGS: &[&str] = &[
        "<!DOCTYPE HTML",
        "<HTML",
        "<HEAD",
        "<SCRIPT",
        "<IFRAME",
        "<H1",
        "<DIV",
        "<FONT",
        "<TABLE",
        "<A",
        "<STYLE",
        "<TITLE",
        "<B",
        "<BODY",
        "<BR",
        "<P",
    ];

    for tag in HTML_TAGS {
        if case_insensitive_starts_with(text, tag) {
            // Check for proper tag termination
            if text.len() > tag.len() {
                let next_char = text.chars().nth(tag.len()).unwrap_or(' ');
                if next_char == ' ' || next_char == '>' || next_char == '\t' || next_char == '\n' {
                    return true;
                }
            }
        }
    }
    false
}

/// Shared XML content detection that works with any encoding after normalization  
fn detect_xml_content(text: &str) -> bool {
    text.trim_start().starts_with("<?xml")
}

/// Shared SVG content detection that works with any encoding after normalization
fn detect_svg_content(text: &str) -> bool {
    let trimmed = text.trim_start();
    if trimmed.starts_with("<?xml") {
        // Look for SVG namespace in XML
        trimmed.contains("<svg") || trimmed.contains("http://www.w3.org/2000/svg")
    } else {
        trimmed.starts_with("<svg")
    }
}

/// Shared JSON content detection that works with any encoding after normalization
fn detect_json_content(text: &str) -> bool {
    let trimmed = text.trim_start();
    (trimmed.starts_with('{') || trimmed.starts_with('[')) && is_valid_json_text(trimmed)
}

/// Shared CSV content detection that works with any encoding after normalization
fn detect_csv_content(text: &str) -> bool {
    let lines = text.lines().take(5);
    detect_csv_generic(lines, |line| count_occurrences(line.as_bytes(), b','))
}

/// Shared TSV content detection that works with any encoding after normalization
fn detect_tsv_content(text: &str) -> bool {
    let lines = text.lines().take(5);
    detect_csv_generic(lines, |line| count_occurrences(line.as_bytes(), b'\t'))
}

/// Shared SRT content detection that works with any encoding after normalization
fn detect_srt_content(text: &str) -> bool {
    let trimmed = text.trim_start();
    if trimmed.starts_with("1\n") || trimmed.starts_with("1\r\n") {
        // Look for timestamp pattern in the next line
        let mut lines = trimmed.lines();

        // Skip first line (should be "1")
        lines.next();

        // Check second line for timestamp pattern
        if let Some(timestamp_line) = lines.next() {
            // Look for SRT timestamp pattern: 00:00:00,000 --> 00:00:00,000
            timestamp_line.contains(" --> ")
        } else {
            false
        }
    } else {
        false
    }
}

/// Shared VTT content detection that works with any encoding after normalization
fn detect_vtt_content(text: &str) -> bool {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("WEBVTT") {
        return false;
    }

    // WEBVTT must be followed by whitespace or end of string
    trimmed.len() == 6
        || trimmed
            .as_bytes()
            .get(6)
            .is_some_and(|&b| b.is_ascii_whitespace())
}

/// Shared vCard content detection that works with any encoding after normalization
fn detect_vcard_content(text: &str) -> bool {
    case_insensitive_starts_with(text.trim_start(), "BEGIN:VCARD")
}

/// Shared iCalendar content detection that works with any encoding after normalization
fn detect_icalendar_content(text: &str) -> bool {
    case_insensitive_starts_with(text.trim_start(), "BEGIN:VCALENDAR")
}

/// Shared RTF content detection that works with any encoding after normalization
fn detect_rtf_content(text: &str) -> bool {
    text.starts_with("{\\rtf")
}

/// Helper function for JSON validation on text
fn is_valid_json_text(text: &str) -> bool {
    // Very basic JSON validation - just check for balanced braces/brackets
    let mut brace_count = 0;
    let mut bracket_count = 0;
    let mut in_string = false;
    let mut escape_next = false;

    for c in text.chars().take(512) {
        if escape_next {
            escape_next = false;
            continue;
        }

        match c {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => brace_count += 1,
            '}' if !in_string => brace_count -= 1,
            '[' if !in_string => bracket_count += 1,
            ']' if !in_string => bracket_count -= 1,
            _ => {}
        }

        if brace_count < 0 || bracket_count < 0 {
            return false;
        }
    }

    // Must be balanced and have at least one brace or bracket
    brace_count == 0 && bracket_count == 0 && (text.contains('{') || text.contains('['))
}

/// Convert UTF-16 bytes to UTF-8 string for content detection
fn utf16_to_string(input: &[u8], big_endian: bool) -> Option<String> {
    // Input must have even length for UTF-16
    if input.len() < 2 || input.len() % 2 != 0 {
        return None;
    }

    let chars: Vec<u16> = input
        .chunks_exact(2)
        .map(|chunk| {
            if big_endian {
                u16::from_be_bytes([chunk[0], chunk[1]])
            } else {
                u16::from_le_bytes([chunk[0], chunk[1]])
            }
        })
        .collect();

    String::from_utf16(&chars).ok()
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Generic function for counting occurrences of a byte in byte sequences
/// Works with any type that can be referenced as [u8]
#[inline]
fn count_occurrences<S: AsRef<[u8]>>(input: S, target: u8) -> usize {
    input
        .as_ref()
        .iter()
        .fold(0, |acc, &b| acc + (b == target) as usize)
}

/// Case-insensitive starts_with that works for both str and [u8] types
/// Uses a trait to handle different input types uniformly
#[inline]
fn case_insensitive_starts_with<H>(input: H, needle: H) -> bool
where
    H: AsRef<[u8]>,
{
    let input_bytes = input.as_ref();
    let needle_bytes = needle.as_ref();
    input_bytes.len() >= needle_bytes.len()
        && input_bytes[..needle_bytes.len()].eq_ignore_ascii_case(needle_bytes)
}

/// Generic CSV detection helper that works with any line iterator
/// T: Iterator over lines (either &[u8] or &str)
/// F: Function to count separator in a line
#[inline]
fn detect_csv_generic<T, F>(mut lines: T, count_separator: F) -> bool
where
    T: Iterator,
    F: Fn(T::Item) -> usize,
{
    let first_line = match lines.next() {
        Some(line) => line,
        None => return false,
    };

    let first_separators = count_separator(first_line);
    if first_separators == 0 {
        return false;
    }

    let mut line_count = 1;
    for line in lines {
        line_count += 1;
        if count_separator(line) != first_separators {
            return false;
        }
    }

    line_count >= 2
}

/// Check if ZIP archive contains any files matching the given entries
fn zip_has(input: &[u8], search_for: &[(&[u8], bool)], stop_after: usize) -> bool {
    let mut iter = ZipIterator::new(input);

    for _ in 0..stop_after {
        if let Some(entry_name) = iter.next() {
            for &(name, is_dir) in search_for {
                if is_dir && entry_name.starts_with(name) {
                    return true;
                }
                if !is_dir && entry_name == name {
                    return true;
                }
            }
        } else {
            break;
        }
    }
    false
}

/// Enhanced Office XML format detection that validates the first entry
/// Matches the Go implementation's msoxml() function exactly
fn msoxml(input: &[u8], search_for: &[(&[u8], bool)], stop_after: usize) -> bool {
    let mut iter = ZipIterator::new(input);

    const EXPECTED_FIRST_ENTRIES: [&[u8]; 5] = [
        b"[Content_Types].xml",
        b"_rels/.rels",
        b"docProps",
        b"customXml",
        b"[trash]",
    ];
    for i in 0..stop_after {
        if let Some(entry_name) = iter.next() {
            // Check if this entry matches what we're looking for
            for &(name, is_dir) in search_for {
                if is_dir && entry_name.starts_with(name) {
                    return true;
                }
                if !is_dir && entry_name == name {
                    return true;
                }
            }

            // If this is the first entry, validate it's a proper Office document
            if i == 0 && !EXPECTED_FIRST_ENTRIES.contains(&entry_name) {
                return false;
            }
        } else {
            break;
        }
    }
    false
}

/// ZIP iterator for parsing ZIP file entries
struct ZipIterator<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ZipIterator<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn next(&mut self) -> Option<&'a [u8]> {
        // Look for ZIP local file header signature "PK\x03\x04"
        let pk_signature = b"PK\x03\x04";

        if let Some(pk_pos) = self.data[self.pos..]
            .windows(4)
            .position(|w| w == pk_signature)
        {
            let header_start = self.pos + pk_pos;

            // Parse ZIP local file header
            // Structure: signature(4) + version(2) + flags(2) + method(2) +
            //           time(2) + date(2) + crc32(4) + compressed_size(4) +
            //           uncompressed_size(4) + filename_length(2) + extra_length(2)

            if header_start + 30 > self.data.len() {
                return None;
            }

            // Skip to filename length field (at offset 26 from signature)
            let filename_len_pos = header_start + 26;
            if filename_len_pos + 4 > self.data.len() {
                return None;
            }

            let filename_length =
                u16::from_le_bytes([self.data[filename_len_pos], self.data[filename_len_pos + 1]])
                    as usize;

            let extra_length = u16::from_le_bytes([
                self.data[filename_len_pos + 2],
                self.data[filename_len_pos + 3],
            ]) as usize;

            // Extract filename
            let filename_start = header_start + 30; // Fixed header size
            if filename_start + filename_length > self.data.len() {
                return None;
            }

            let filename = &self.data[filename_start..filename_start + filename_length];

            // Move position past this entry
            self.pos = filename_start + filename_length + extra_length;

            return Some(filename);
        }

        None
    }
}

/// Check if OLE compound document matches a specific CLSID
/// Based on Go implementation: matchOleClsid function
fn ole_matches_clsid(input: &[u8], clsid: &[u8]) -> bool {
    // Microsoft Compound files v3 have a sector length of 512, while v4 has 4096.
    // Change sector offset depending on file version.
    let sector_length = if input.len() >= 28 && input[26] == 0x04 && input[27] == 0x00 {
        4096
    } else {
        512
    };

    if input.len() < sector_length {
        return false;
    }

    // SecID of first sector of the directory stream (offset 48-51)
    if input.len() < 52 {
        return false;
    }

    let first_sec_id = u32::from_le_bytes([input[48], input[49], input[50], input[51]]) as usize;

    // Expected offset of CLSID for root storage object
    let clsid_offset = sector_length * (1 + first_sec_id) + 80;

    // Check if CLSID matches (handle partial matches for shorter CLSIDs)
    let match_length = clsid.len().min(16);

    if input.len() < clsid_offset + match_length {
        return false;
    }

    let actual_clsid = &input[clsid_offset..clsid_offset + match_length];
    actual_clsid == clsid
}

/// Check if input contains JavaScript keywords
fn has_js_keywords(input: &[u8]) -> bool {
    const KEYWORDS: [&[u8]; 7] = [
        b"function",
        b"var ",
        b"let ",
        b"const ",
        b"class ",
        b"import ",
        b"export ",
    ];
    let sample = &input[..input.len().min(256)];
    KEYWORDS
        .iter()
        .any(|&keyword| sample.windows(keyword.len()).any(|w| w == keyword))
}

/// Simple JSON validation
fn is_valid_json(input: &[u8]) -> bool {
    // Very basic JSON validation - just check for balanced braces/brackets
    let mut brace_count = 0;
    let mut bracket_count = 0;
    let mut in_string = false;
    let mut escape_next = false;

    for &byte in input.iter().take(512) {
        // Limit check to first 512 bytes
        if escape_next {
            escape_next = false;
            continue;
        }

        match byte {
            b'\\' if in_string => escape_next = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => brace_count += 1,
            b'}' if !in_string => brace_count -= 1,
            b'[' if !in_string => bracket_count += 1,
            b']' if !in_string => bracket_count -= 1,
            _ => {}
        }

        if brace_count < 0 || bracket_count < 0 {
            return false;
        }
    }

    // Must be balanced and have at least one brace or bracket
    brace_count == 0 && bracket_count == 0 && (input.contains(&b'{') || input.contains(&b'['))
}

// ============================================================================
// ELF SUBTYPE DETECTORS
// ============================================================================

/// ELF Object File (ET_REL)
fn elf_obj(input: &[u8]) -> bool {
    if input.len() < 18 {
        return false;
    }
    elf(input) && input[16] == 1 && input[17] == 0
}

/// ELF Executable (ET_EXEC)
fn elf_exe(input: &[u8]) -> bool {
    if input.len() < 18 {
        return false;
    }
    elf(input) && input[16] == 2 && input[17] == 0
}

/// ELF Shared Library (ET_DYN)
fn elf_lib(input: &[u8]) -> bool {
    if input.len() < 18 {
        return false;
    }
    elf(input) && input[16] == 3 && input[17] == 0
}

/// ELF Core Dump (ET_CORE)
fn elf_dump(input: &[u8]) -> bool {
    if input.len() < 18 {
        return false;
    }
    elf(input) && input[16] == 4 && input[17] == 0
}

/// FDF (Forms Data Format) - PDF variant
fn fdf(input: &[u8]) -> bool {
    input.starts_with(b"%FDF-")
}

/// P7S (PKCS#7 Signature)
fn p7s(input: &[u8]) -> bool {
    input.starts_with(b"-----BEGIN PKCS7-----")
}

/// AAF (Advanced Authoring Format)
fn aaf(input: &[u8]) -> bool {
    if !ole(input) {
        return false;
    }

    // AAF uses a specific CLSID to distinguish from other OLE formats
    // This prevents it from matching generic OLE or other Office documents
    const AAF_CLSID: &[u8] = &[
        0xAA, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];

    ole_matches_clsid(input, AAF_CLSID)
}

// ============================================================================

/// Generic UTF-16 format detection helper
/// Consolidates the pattern used by all UTF-16 BE/LE detection functions
#[inline]
fn detect_utf16_format<F>(input: &[u8], big_endian: bool, detect_content: F) -> bool
where
    F: Fn(&str) -> bool,
{
    if let Some(text) = utf16_to_text(input, big_endian) {
        detect_content(&text)
    } else {
        false
    }
}

/// Generic XML-based format detection helper
/// Consolidates the pattern: check if XML, then search for specific tag
#[inline]
fn detect_xml_with_tag(input: &[u8], tag: &[u8]) -> bool {
    xml(input) && input.windows(tag.len()).any(|w| w == tag)
}

/// Generic OpenDocument format detection helper
/// Consolidates the pattern: check for mimetype string at offset 30
#[inline]
fn detect_opendocument_format(input: &[u8], mimetype: &[u8]) -> bool {
    // All OpenDocument formats have "mimetype" followed by the actual MIME type at offset 30
    const MIMETYPE_PREFIX: &[u8] = b"mimetype";
    let prefix_len = MIMETYPE_PREFIX.len();
    let total_len = prefix_len + mimetype.len();

    if input.len() < 30 + total_len {
        return false;
    }

    // Check prefix and mimetype separately to avoid allocation
    &input[30..30 + prefix_len] == MIMETYPE_PREFIX
        && &input[30 + prefix_len..30 + total_len] == mimetype
}

/// Helper for OLE-based format detection with CLSID
/// This pattern is used by multiple Office formats
#[inline]
fn detect_ole_with_clsid(input: &[u8], clsid: &[u8; 16]) -> bool {
    ole(input) && ole_matches_clsid(input, clsid)
}
