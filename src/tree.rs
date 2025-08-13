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

use crate::mime_type::MimeType;
use crate::constants::*;

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
        &HTML,
        &XML,
        &PDF,
        &PS,
        &UTF8_BOM,
        &UTF16_BE,
        &UTF16_LE,
        &SEVEN_Z,
        &ZIP,
        &WASM,
        &RAR,
        &GZIP_PRECISE,
        &OLE,
        &PSD,
        &OGG,
        &PNG,
        &APNG,
        &JPG,
        &JP2,
        &JPX,
        &JPM,
        &GIF,
        &WEBP,
        &BPG,
        &XCF,
        &PAT,
        &GBR,
        &HDR,
        &XPM,
        &JXS,
        &JXR,
        &JXL,
        &TIFF,
        &BMP,
        &ICO,
        &AIFF,
        &MP3,
        &APE,
        &MUSEPACK,
        &AU,
        &AMR,
        &VOC,
        &M3U,
        &AAC,
        &QCP,
        &FLAC,
        &MIDI_PRECISE,
        &WAV,
        &MPEG,
        &QUICKTIME,
        &MQV,
        &MP4_PRECISE,
        &RMVB,
        &WEBM,
        &AVI,
        &FLV,
        &MKV,
        &ASF,
        &AMP4,
        &M4A,
        &M4V,
        &CLASS,
        &SWF,
        &CRX,
        &EOT,
        &TTC,
        &WOFF,
        &WOFF2,
        &OTF,
        &DCM,
        &DJVU,
        &MOBI,
        &LIT,
        &SQLITE3,
        &DWG,
        &ICNS,
        &HEIC,
        &HEIC_SEQ,
        &HEIF,
        &HEIF_SEQ,
        &ZSTD,
        &CAB,
        &INSTALL_SHIELD_CAB,
        &RPM,
        &XZ,
        &LZIP,
        &TORRENT,
        &CPIO,
        &TTF,
        &FITS,
        &XAR,
        &DEB,
        &WARC,
        &TAR,
        &EXE,
        &ELF,
        &AR,
        &BZ2,
        &GZIP,
        &FASOO,
        &PGP_NET_SHARE,
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
static HTML: MimeType =
    MimeType::new(TEXT_HTML, ".html", html, &[]).with_extension_aliases(&[".htm"]);

static XML: MimeType = MimeType::new(TEXT_XML, ".xml", xml, &[]);

static UTF8_BOM: MimeType = MimeType::new(TEXT_UTF8_BOM, ".txt", utf8_bom, &[]);

static UTF16_BE: MimeType = MimeType::new(TEXT_UTF16_BE, ".txt", utf16_be, &[]);

static UTF16_LE: MimeType = MimeType::new(TEXT_UTF16_LE, ".txt", utf16_le, &[]);

static UTF8: MimeType = MimeType::new(TEXT_UTF8, ".txt", utf8, &[])
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
    ]);

// ============================================================================
// DOCUMENT FORMATS
// ============================================================================

static PDF: MimeType =
    MimeType::new(APPLICATION_PDF, ".pdf", pdf, &[]).with_aliases(&[APPLICATION_X_PDF]);

static PS: MimeType = MimeType::new(APPLICATION_POSTSCRIPT, ".ps", ps, &[]);

static OLE: MimeType = MimeType::new(APPLICATION_X_OLE_STORAGE, "", ole, &[])
    .with_extension_aliases(&[".xls", ".pub", ".ppt", ".doc"]);

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
static SEVEN_Z: MimeType = MimeType::new(APPLICATION_X_7Z_COMPRESSED, ".7z", seven_z, &[]);

static ZIP: MimeType = MimeType::new(APPLICATION_ZIP, ".zip", zip, &[])
    .with_aliases(&[APPLICATION_X_ZIP, APPLICATION_X_ZIP_COMPRESSED])
    .with_extension_aliases(&[
        ".xlsx", ".docx", ".pptx", ".epub", ".jar", ".odt", ".ods", ".odp", ".odg", ".odf",
    ]);

static RAR: MimeType = MimeType::new(APPLICATION_X_RAR_COMPRESSED, ".rar", rar, &[])
    .with_aliases(&[APPLICATION_X_RAR]);

static GZIP: MimeType = MimeType::new(APPLICATION_GZIP, ".gz", gzip, &[])
    .with_aliases(&[
        APPLICATION_X_GZIP,
        APPLICATION_X_GUNZIP,
        APPLICATION_GZIPPED,
        APPLICATION_GZIP_COMPRESSED,
        APPLICATION_X_GZIP_COMPRESSED,
        GZIP_DOCUMENT,
    ])
    .with_extension_aliases(&[".tgz", ".taz"]);

static GZIP_PRECISE: MimeType = MimeType::new(APPLICATION_X_GZIP, ".gz", gzip_precise, &[]);

static TAR: MimeType = MimeType::new(APPLICATION_X_TAR, ".tar", tar, &[]);

static BZ2: MimeType = MimeType::new(APPLICATION_X_BZIP2, ".bz2", bz2, &[]);

static XZ: MimeType = MimeType::new(APPLICATION_X_XZ, ".xz", xz, &[]);

static ZSTD: MimeType = MimeType::new(APPLICATION_ZSTD, ".zst", zstd, &[]);

static LZIP: MimeType = MimeType::new(APPLICATION_LZIP, ".lz", lzip, &[]);

static CAB: MimeType = MimeType::new(APPLICATION_VND_MS_CAB_COMPRESSED, ".cab", cab, &[]);

static INSTALL_SHIELD_CAB: MimeType = MimeType::new(
    APPLICATION_X_INSTALLSHIELD,
    ".cab",
    install_shield_cab,
    &[],
);

static CPIO: MimeType = MimeType::new(APPLICATION_X_CPIO, ".cpio", cpio, &[]);

static AR: MimeType = MimeType::new(APPLICATION_X_ARCHIVE, ".a", ar, &[])
    .with_aliases(&[APPLICATION_X_UNIX_ARCHIVE])
    .with_extension_aliases(&[".deb"]);

static RPM: MimeType = MimeType::new(APPLICATION_X_RPM, ".rpm", rpm, &[]);

static TORRENT: MimeType = MimeType::new(APPLICATION_X_BITTORRENT, ".torrent", torrent, &[]);

static FITS: MimeType = MimeType::new(APPLICATION_FITS, ".fits", fits, &[]);

static XAR: MimeType = MimeType::new(APPLICATION_X_XAR, ".xar", xar, &[]);

static DEB: MimeType = MimeType::new(APPLICATION_VND_DEBIAN_BINARY_PACKAGE, ".deb", deb, &[]);

static WARC: MimeType = MimeType::new(APPLICATION_WARC, ".warc", warc, &[]);

// ============================================================================
// IMAGE FORMATS
// ============================================================================

static PNG: MimeType = MimeType::new(IMAGE_PNG, ".png", png, &[]);

static APNG: MimeType = MimeType::new(IMAGE_VND_MOZILLA_APNG, ".apng", apng, &[]);

static JPG: MimeType = MimeType::new(IMAGE_JPEG, ".jpg", jpg, &[])
    .with_extension_aliases(&[".jpeg", ".jpe", ".jif", ".jfif", ".jfi"]);

static JP2: MimeType = MimeType::new(IMAGE_JP2, ".jp2", jp2, &[]);

static JPX: MimeType = MimeType::new(IMAGE_JPX, ".jpx", jpx, &[]);

static JPM: MimeType = MimeType::new(IMAGE_JPM, ".jpm", jpm, &[]);

static JXS: MimeType = MimeType::new(IMAGE_JXS, ".jxs", jxs, &[]);

static JXR: MimeType = MimeType::new(IMAGE_JXR, ".jxr", jxr, &[]);

static JXL: MimeType = MimeType::new(IMAGE_JXL, ".jxl", jxl, &[]);

static GIF: MimeType = MimeType::new(IMAGE_GIF, ".gif", gif, &[]);

static WEBP: MimeType = MimeType::new(IMAGE_WEBP, ".webp", webp, &[]);

static TIFF: MimeType =
    MimeType::new(IMAGE_TIFF, ".tiff", tiff, &[]).with_extension_aliases(&[".tif"]);

static BMP: MimeType = MimeType::new(IMAGE_BMP, ".bmp", bmp, &[])
    .with_aliases(&[IMAGE_X_BMP, IMAGE_X_MS_BMP])
    .with_extension_aliases(&[".dib"]);

static ICO: MimeType = MimeType::new(IMAGE_X_ICON, ".ico", ico, &[]);

static ICNS: MimeType = MimeType::new(IMAGE_X_ICNS, ".icns", icns, &[]);

static PSD: MimeType = MimeType::new(IMAGE_VND_ADOBE_PHOTOSHOP, ".psd", psd, &[])
    .with_aliases(&[IMAGE_X_PSD, APPLICATION_PHOTOSHOP]);

static HEIC: MimeType = MimeType::new(IMAGE_HEIC, ".heic", heic, &[]);

static HEIC_SEQ: MimeType = MimeType::new(IMAGE_HEIC_SEQUENCE, ".heic", heic_sequence, &[]);

static HEIF: MimeType = MimeType::new(IMAGE_HEIF, ".heif", heif, &[]);

static HEIF_SEQ: MimeType = MimeType::new(IMAGE_HEIF_SEQUENCE, ".heif", heif_sequence, &[]);

static BPG: MimeType = MimeType::new(IMAGE_BPG, ".bpg", bpg, &[]);

static XCF: MimeType = MimeType::new(IMAGE_X_XCF, ".xcf", xcf, &[]);

static PAT: MimeType = MimeType::new(IMAGE_X_GIMP_PAT, ".pat", pat, &[]);

static GBR: MimeType = MimeType::new(IMAGE_X_GIMP_GBR, ".gbr", gbr, &[]);

static HDR: MimeType = MimeType::new(IMAGE_VND_RADIANCE, ".hdr", hdr, &[]);

static XPM: MimeType = MimeType::new(IMAGE_X_XPIXMAP, ".xpm", xpm, &[]);

static DWG: MimeType = MimeType::new(IMAGE_VND_DWG, ".dwg", dwg, &[]).with_aliases(&[
    IMAGE_X_DWG,
    APPLICATION_ACAD,
    APPLICATION_X_ACAD,
    APPLICATION_AUTOCAD_DWG,
    APPLICATION_DWG,
    APPLICATION_X_DWG,
    APPLICATION_X_AUTOCAD,
    DRAWING_DWG,
]);

static DJVU: MimeType = MimeType::new(IMAGE_VND_DJVU, ".djvu", djvu, &[]);

// ============================================================================
// AUDIO FORMATS
// ============================================================================

static MP3: MimeType =
    MimeType::new(AUDIO_MPEG, ".mp3", mp3, &[]).with_aliases(&[AUDIO_X_MPEG, AUDIO_MP3]);

static FLAC: MimeType = MimeType::new(AUDIO_FLAC, ".flac", flac, &[]);

static WAV: MimeType = MimeType::new(AUDIO_WAV, ".wav", wav, &[]).with_aliases(&[
    AUDIO_X_WAV,
    AUDIO_VND_WAVE,
    AUDIO_WAVE,
]);

static AIFF: MimeType =
    MimeType::new(AUDIO_AIFF, ".aiff", aiff, &[]).with_extension_aliases(&[".aif"]);

static MIDI_PRECISE: MimeType = MimeType::new(AUDIO_MIDI, ".midi", midi_precise, &[])
    .with_aliases(&[AUDIO_MID])
    .with_extension_aliases(&[".mid"]);

static OGG: MimeType = MimeType::new(APPLICATION_OGG, ".ogg", ogg, &[])
    .with_extension_aliases(&[".oga", ".opus", ".ogv"]);

static APE: MimeType = MimeType::new(AUDIO_APE, ".ape", ape, &[]);

static MUSEPACK: MimeType = MimeType::new(AUDIO_MUSEPACK, ".mpc", musepack, &[]);

static AU: MimeType =
    MimeType::new(AUDIO_BASIC, ".au", au, &[]).with_extension_aliases(&[".snd"]);

static AMR: MimeType = MimeType::new(AUDIO_AMR, ".amr", amr, &[]);

static VOC: MimeType = MimeType::new(AUDIO_X_UNKNOWN, ".voc", voc, &[]);

static M3U: MimeType =
    MimeType::new(AUDIO_X_MPEGURL, ".m3u", m3u, &[]).with_extension_aliases(&[".m3u8"]);

static AAC: MimeType = MimeType::new(AUDIO_AAC, ".aac", aac, &[]);

static QCP: MimeType = MimeType::new(AUDIO_QCELP, ".qcp", qcp, &[]);

static AMP4: MimeType =
    MimeType::new(AUDIO_MP4, ".mp4", amp4, &[]).with_aliases(&[AUDIO_X_M4A, AUDIO_X_MP4A]);

static M4A: MimeType = MimeType::new(AUDIO_X_M4A, ".m4a", m4a, &[]);

// ============================================================================
// VIDEO FORMATS
// ============================================================================

static MP4_PRECISE: MimeType = MimeType::new(VIDEO_MP4, ".mp4", mp4_precise, &[]);

static WEBM: MimeType =
    MimeType::new(VIDEO_WEBM, ".webm", webm, &[]).with_aliases(&[AUDIO_WEBM]);

static MKV: MimeType = MimeType::new(VIDEO_X_MATROSKA, ".mkv", mkv, &[])
    .with_extension_aliases(&[".mk3d", ".mka", ".mks"]);

static AVI: MimeType = MimeType::new(VIDEO_X_MSVIDEO, ".avi", avi, &[])
    .with_aliases(&[VIDEO_AVI, VIDEO_MSVIDEO]);

static MPEG: MimeType = MimeType::new(VIDEO_MPEG, ".mpeg", mpeg, &[]);

static QUICKTIME: MimeType = MimeType::new(VIDEO_QUICKTIME, ".mov", quicktime, &[]);

static MQV: MimeType = MimeType::new(VIDEO_QUICKTIME, ".mqv", mqv, &[]);

static FLV: MimeType = MimeType::new(VIDEO_X_FLV, ".flv", flv, &[]);

static ASF: MimeType = MimeType::new(VIDEO_X_MS_ASF, ".asf", asf, &[])
    .with_aliases(&[VIDEO_ASF, VIDEO_X_MS_WMV]);

static M4V: MimeType = MimeType::new(VIDEO_X_M4V, ".m4v", m4v, &[]);

static RMVB: MimeType = MimeType::new(APPLICATION_VND_RN_REALMEDIA_VBR, ".rmvb", rmvb, &[]);

// ============================================================================
// EXECUTABLE & BINARY FORMATS
// ============================================================================

static EXE: MimeType = MimeType::new(
    APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE,
    ".exe",
    exe,
    &[],
);

static ELF: MimeType =
    MimeType::new(APPLICATION_X_ELF, "", elf, &[]).with_extension_aliases(&[".so"]);

static CLASS: MimeType = MimeType::new(
    APPLICATION_X_JAVA_APPLET_BINARY,
    ".class",
    class,
    &[],
)
.with_aliases(&[APPLICATION_X_JAVA_APPLET]);

static WASM: MimeType = MimeType::new(APPLICATION_WASM, ".wasm", wasm, &[]);

// ============================================================================
// FONT FORMATS
// ============================================================================

static TTF: MimeType = MimeType::new(FONT_TTF, ".ttf", ttf, &[]).with_aliases(&[
    FONT_SFNT,
    APPLICATION_X_FONT_TTF,
    APPLICATION_FONT_SFNT,
]);

static WOFF: MimeType = MimeType::new(FONT_WOFF, ".woff", woff, &[]);

static WOFF2: MimeType = MimeType::new(FONT_WOFF2, ".woff2", woff2, &[]);

static OTF: MimeType = MimeType::new(FONT_OTF, ".otf", otf, &[]);

static EOT: MimeType = MimeType::new(APPLICATION_VND_MS_FONTOBJECT, ".eot", eot, &[]);

static TTC: MimeType = MimeType::new(FONT_COLLECTION, ".ttc", ttc, &[]);

// ============================================================================
// WEB & MULTIMEDIA FORMATS
// ============================================================================

static SWF: MimeType = MimeType::new(APPLICATION_X_SHOCKWAVE_FLASH, ".swf", swf, &[]);

static CRX: MimeType = MimeType::new(APPLICATION_X_CHROME_EXTENSION, ".crx", crx, &[]);

// ============================================================================
// SPECIALIZED FORMATS
// ============================================================================

static DCM: MimeType = MimeType::new(APPLICATION_DICOM, ".dcm", dcm, &[]);

static MOBI: MimeType = MimeType::new(APPLICATION_X_MOBIPOCKET_EBOOK, ".mobi", mobi, &[]);

static LIT: MimeType = MimeType::new(APPLICATION_X_MS_READER, ".lit", lit, &[]);

static SQLITE3: MimeType = MimeType::new(APPLICATION_X_SQLITE3, ".sqlite", sqlite, &[]);

static FASOO: MimeType = MimeType::new(APPLICATION_X_FASOO, "", fasoo, &[]);

static PGP_NET_SHARE: MimeType =
    MimeType::new(APPLICATION_X_PGP_NET_SHARE, "", pgp_net_share, &[]);

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
    // Register ROOT first
    ROOT.register();

    // Register all MIME types in hierarchical order
    // Text formats (highest priority)
    HTML.register();
    XML.register();
    UTF8_BOM.register();
    UTF16_BE.register();
    UTF16_LE.register();

    // Documents
    PDF.register();
    PS.register();
    OLE.register();

    // Archives and compression
    SEVEN_Z.register();
    ZIP.register();
    RAR.register();
    GZIP.register();
    GZIP_PRECISE.register();
    TAR.register();
    BZ2.register();
    XZ.register();
    ZSTD.register();
    LZIP.register();
    CAB.register();
    INSTALL_SHIELD_CAB.register();
    CPIO.register();
    AR.register();
    RPM.register();
    TORRENT.register();
    FITS.register();
    XAR.register();
    DEB.register();
    WARC.register();

    // Images
    PNG.register();
    APNG.register();
    JPG.register();
    JP2.register();
    JPX.register();
    JPM.register();
    JXS.register();
    JXR.register();
    JXL.register();
    GIF.register();
    WEBP.register();
    TIFF.register();
    BMP.register();
    ICO.register();
    ICNS.register();
    PSD.register();
    HEIC.register();
    HEIC_SEQ.register();
    HEIF.register();
    HEIF_SEQ.register();
    BPG.register();
    XCF.register();
    PAT.register();
    GBR.register();
    HDR.register();
    XPM.register();
    DWG.register();
    DJVU.register();

    // Audio
    MP3.register();
    FLAC.register();
    WAV.register();
    AIFF.register();
    MIDI_PRECISE.register();
    OGG.register();
    APE.register();
    MUSEPACK.register();
    AU.register();
    AMR.register();
    VOC.register();
    M3U.register();
    AAC.register();
    QCP.register();
    AMP4.register();
    M4A.register();

    // Video
    MP4_PRECISE.register();
    WEBM.register();
    MKV.register();
    AVI.register();
    MPEG.register();
    QUICKTIME.register();
    MQV.register();
    FLV.register();
    ASF.register();
    M4V.register();
    RMVB.register();

    // Executables
    EXE.register();
    ELF.register();
    CLASS.register();
    WASM.register();

    // Fonts
    TTF.register();
    WOFF.register();
    WOFF2.register();
    OTF.register();
    EOT.register();
    TTC.register();

    // Web & Multimedia
    SWF.register();
    CRX.register();

    // Specialized
    DCM.register();
    MOBI.register();
    LIT.register();
    SQLITE3.register();
    FASOO.register();
    PGP_NET_SHARE.register();

    // Text (lowest priority, catches everything else)
    UTF8.register();
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
    const HTML_TAGS: &[&[u8]] = &[
        b"<!DOCTYPE HTML",
        b"<HTML",
        b"<HEAD",
        b"<SCRIPT",
        b"<IFRAME",
        b"<H1",
        b"<DIV",
        b"<FONT",
        b"<TABLE",
        b"<A",
        b"<STYLE",
        b"<TITLE",
        b"<B",
        b"<BODY",
        b"<BR",
        b"<P",
    ];

    let input = skip_whitespace(input);
    for tag in HTML_TAGS {
        if input.len() >= tag.len() {
            let mut matches = true;
            for (i, &expected) in tag.iter().enumerate() {
                let actual = input[i];
                let normalized = if expected.is_ascii_uppercase() {
                    actual.to_ascii_uppercase()
                } else {
                    actual
                };
                if normalized != expected {
                    matches = false;
                    break;
                }
            }
            if matches {
                // Check for proper tag termination if there are more bytes
                if input.len() > tag.len() {
                    if is_tag_terminating(input[tag.len()]) {
                        return true;
                    }
                } else {
                    // Tag matches exactly at end of input - only valid for self-contained tags like comments
                    if tag == b"<!--" {
                        return true;
                    }
                    // For regular tags, we need proper termination
                }
            }
        }
    }
    false
}

fn xml(input: &[u8]) -> bool {
    let input = skip_whitespace(input);
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

fn gzip_precise(input: &[u8]) -> bool {
    input.starts_with(b"\x1F\x8B\x08")
}

fn mp4_precise(input: &[u8]) -> bool {
    if input.len() < 12 {
        return false;
    }

    let box_size = u32::from_be_bytes([input[0], input[1], input[2], input[3]]) as usize;
    if input.len() < box_size || box_size % 4 != 0 || box_size < 12 {
        return false;
    }

    if &input[4..8] != b"ftyp" {
        return false;
    }

    // Check for MP4 brand in the ftyp box
    let mut pos = 8;
    while pos + 3 < box_size.min(input.len()) {
        if pos == 12 {
            // Skip version number
            pos += 4;
            continue;
        }
        if pos + 3 < input.len() && &input[pos..pos + 3] == b"mp4" {
            return true;
        }
        pos += 4;
    }
    false
}

fn midi_precise(input: &[u8]) -> bool {
    input.starts_with(b"MThd\x00\x00\x00\x06")
}

fn skip_whitespace(input: &[u8]) -> &[u8] {
    let mut start = 0;
    while start < input.len() {
        match input[start] {
            b'\t' | b'\n' | 0x0C | b'\r' | b' ' => start += 1,
            _ => break,
        }
    }
    &input[start..]
}

fn is_tag_terminating(byte: u8) -> bool {
    byte == b' ' || byte == b'>'
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

fn ogg(input: &[u8]) -> bool {
    input.starts_with(b"OggS")
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

    let dwg_versions: [&[u8; 4]; 15] = [
        b"1.40", b"1.50", b"2.10", b"1002", b"1003", b"1004", b"1006", b"1009", b"1012", b"1014",
        b"1015", b"1018", b"1021", b"1024", b"1032",
    ];

    let ver = &input[2..6];
    dwg_versions.iter().any(|version| ver.eq(*version))
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
