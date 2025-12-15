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

build_prefix_vec! {
    /// Prefix vector for fast ROOT child lookup
    /// Uses first byte (0-255) to index into array of MimeType slices
    /// Covers 193 out of 264 ROOT children using 92 unique first bytes
    /// Static array with zero runtime overhead - no LazyLock, no mutex, no heap allocations
    static ROOT_PREFIX_VEC: [
        0x00 => [&JXS, &ICO, &SHX, &TGA, &WASM, &MRW, &WORKS_SPREADSHEET, &WORKS_XLR, &CUR, &MDB, &ACCDB, &QUARK, &AMIGA_HUNK] as __PV_00,
        0x01 => [&SGI] as __PV_01,
        0x02 => [&ARSC, &CLARISWORKS] as __PV_02,  // Android Resource Storage Container, ClarisWorks
        0x03 => [&AXML, &DBASE] as __PV_03,  // Android Binary XML and dBASE
        0x04 => [&LZ4] as __PV_04,
        0x0a => [&PCAPNG] as __PV_0A,
        0x0b => [&AC3] as __PV_0B,  // Audio Codec 3
        0x0e => [&HDF4] as __PV_0E,  // HDF4 format
        0x11 => [&FLI] as __PV_11,
        0x12 => [&FLC] as __PV_12,
        0x13 => [&ASTC] as __PV_13,
        0x1a => [&LOTUS_NOTES, &WEBM, &MKV] as __PV_1A,  // Lotus Notes, Matroska containers
        0x1b => [&LUA_BYTECODE] as __PV_1B,  // Lua bytecode
        0x1f => [&GZIP, &UNIX_COMPRESS] as __PV_1F,  // GZIP (0x1F 0x8B), Unix compress (0x1F 0x9D)
        0x21 => [&PST, &AR] as __PV_21,  // PST ('!BDN'), AR ('!<arch>')
        0x23 => [&USD_ASCII, &IQE, &AMR, &HDR, &M3U, &VMDK, &VRML] as __PV_23,  // USD ASCII ('#usda'), IQE, AMR, HDR, M3U, VMDK, VRML
        0x25 => [&PS, &FDF, &PDF] as __PV_25,
        0x28 => [&WAT, &DWF] as __PV_28,  // WebAssembly Text '(module', Design Web Format '(DWF'
        0x2d => [&CSR, &P7S, &PMA, &LHA, &LZS] as __PV_2D,  // CSR, P7S, PMA, LHA, LZS
        0x20 => [&NEO_GEO_POCKET_ROM, &WORKS_DB, &IGES] as __PV_20,  // Neo Geo Pocket (parent checks header, child refines to Color), Microsoft Works DB, IGES CAD format
        0x2e => [&NINTENDO_DS_ROM, &REALMEDIA, &AU, &REALAUDIO] as __PV_2E,  // Nintendo DS ROM, RealMedia, AU/SND, RealAudio
        0x2f => [&XPM, &MAYA_ASCII] as __PV_2F,  // XPM, Maya ASCII
        0x30 => [&ASF, &CPIO, &DER_CERT, &EVT] as __PV_30,  // ASF, CPIO ASCII variant, DER certificates, Windows Event Log
        0x31 => [&MICROSOFT_WRITE] as __PV_31,  // Microsoft Write v3.0
        0x32 => [&MICROSOFT_WRITE, &AVR] as __PV_32,  // Microsoft Write v3.1, Audio Visual Research ('2BIT')
        0x33 => [&M3D, &A3D, &OPENNURBS] as __PV_33,  // Model 3D Binary ('3DMO'), Model 3D ASCII ('3DGeometry'), OpenNURBS/Rhino 3DM ('3D Geometry')
        0x34 => [&PICTOR] as __PV_34,  // PICtor/PC Paint DOS graphics
        0x37 => [&N64_ROM, &SEVEN_Z, &ZPAQ] as __PV_37,  // N64 ROM (V64 byte-swapped), 7-Zip, ZPAQ
        0x3c => [&WPL, &DRAWIO, &XSPF, &XSL, &MATHML, &MUSICXML, &TTML, &SOAP, &TMX, &TSX, &MPD, &CDDX, &DWFX] as __PV_3C,  // XML formats: WPL, draw.io, XSPF, XSLT, MathML, MusicXML, TTML, SOAP, TMX, TSX, MPD, CDDX, DWFX
        0x40 => [&N64_ROM] as __PV_40,  // N64 ROM (N64 little-endian)
        0x3f => [&HLP] as __PV_3F,  // Windows Help
        0x38 => [&PSD] as __PV_38,
        0x41 => [&DXF_BINARY, &DJVU, &DWG, &ARROW, &ALZ, &AMV] as __PV_41,  // DXF Binary ('AutoCAD'), DJVU, DWG, Apache Arrow, ALZ, AMV (Actions Media Video)
        0x06 => [&INDESIGN, &MXF] as __PV_06,  // Adobe InDesign, Material Exchange Format
        0x42 => [&BMFONT_BINARY, &BLEND, &BMP, &BPG, &BZIP3, &BZIP, &BZ2, &LLVM_BITCODE] as __PV_42,  // BMFont, BLEND, BMP, BPG, BZIP3, BZIP before BZ2 for priority, LLVM Bitcode ('BC')
        0x43 => [&VOC, &SWF, &CRX, &COMMODORE_64_CARTRIDGE, &VMDK] as __PV_43,  // SWF ('CWS'), CRX, C64 CRT, VMDK ('COWD')
        0x44 => [&ADF, &DDS, &DSF, &DRACO] as __PV_44,  // Amiga Disk File ('DOS'), DDS, DSF, Draco ('DRACO')
        0x45 => [&XM, &EVTX] as __PV_45,  // Extended Module, Windows Event Log XML
        0x46 => [&FLV, &DFF, &FVT, &SWF, &RAF, &EIGHTSVX, &MAYA_BINARY, &FLIF] as __PV_46,  // Added SWF ('FWS'), RAF ('FUJIFILM'), 8SVX ('FORM'), Maya Binary ('FOR4'/'FOR8'), FLIF
        0x47 => [&GIF, &GRIB] as __PV_47,  // GIF, GRIB weather data
        0x48 => [&XCI] as __PV_48,  // Nintendo Switch ROM (XCI - 'HEAD')
        0x49 => [&IQM, &JXR, &LIT, &TIFF, &CHM, &INSTALL_SHIELD_CAB, &CRW, &IT, &ORF, &RW2, &KODAK_KDC, &KODAK_DCR, &STEP] as __PV_49,  // IQM, TIFF includes CR2/NEF as children, CHM already here, Kodak RAW, STEP ('ISO-10303-21')
        0x4b => [&FBX, &VMDK] as __PV_4B,  // Autodesk FBX (Kaydara), VMDK ('KDMV')
        0x4c => [&COFF, &LNK, &LZIP, &LRF, &LRZIP] as __PV_4C,  // COFF (i386), LNK, LZIP, LRF (Sony Reader), LRZIP
        0x4d => [&MODEL3D_BINARY, &MLA, &MUSEPACK, &CAB, &MIDI, &EXE, &AUTODESK_3DS, &TIFF, &ORF, &MOZILLA_ARCHIVE, &WIM, &SGI_MOVIE] as __PV_4D,  // Model3D Binary ('MD30'), MLA, 3DS (exclude TIFF), ORF (MMOR variant), Mozilla Archive, WIM, SGI Movie
        0x4e => [&NINTENDO_SWITCH_NSO, &NES] as __PV_4E,  // Nintendo Switch NSO, NES ROM
        0x4f => [&OTF, &OGG, &AVRO] as __PV_4F,  // OTF, OGG, Apache Avro
        0x50 => [&USD_BINARY, &PFM, &NINTENDO_SWITCH_NSP, &PARQUET, &ZIP, &PBM, &PGM, &PPM, &PAM, &PAK] as __PV_50,  // USD Binary ('PXR-USDC'), PFM, Nintendo Switch NSP, Parquet, ZIP, Portable formats, PAK
        0x51 => [&QCOW2, &QCOW, &CINEMA4D] as __PV_51,  // QEMU Copy-on-Write v2 ('QFI\xFB'), v1 ('QFI'), Cinema4D ('QC4DC4D6')
        0x52 => [&WINDOWS_REG, &RAR, &ANI, &SOUNDFONT2, &CDA, &RZIP] as __PV_52,  // Windows Registry, RAR, ANI (RIFF ACON), SoundFont 2 (RIFF sfbk), CDA (RIFF CDDA), RZIP
        0x53 => [&FITS, &SQLITE3, &STUFFIT, &STUFFITX, &SEQBOX, &DPX] as __PV_53,  // FITS, SQLite3, StuffIt, StuffItX, SeqBox, DPX (SDPX)
        0x54 => [&TTA, &TZIF] as __PV_54,
        0x55 => [&U3D] as __PV_55,
        0x56 => [&VOX] as __PV_56,  // MagicaVoxel ('VOX ')
        0x57 => [&AUTODESK_ALIAS] as __PV_57,  // Autodesk Alias ('WIRE')
        0x58 => [&DPX, &XBE, &XEX] as __PV_58,  // DPX (XPDS little-endian), Xbox XBE (XBEH), Xbox 360 XEX (XEX1/XEX2)
        0x59 => [&SUN_RASTER] as __PV_59,
        0x5a => [&SWF, &ZOO, &TASTY] as __PV_5A,  // SWF ('ZWS'), Zoo archive, Tasty format
        0x5b => [&PLS] as __PV_5B,  // Shoutcast Playlist ('[playlist]')
        0x5d => [&LZMA] as __PV_5D,  // LZMA compression
        0x60 => [&ARJ] as __PV_60,
        0x61 => [&AGE] as __PV_61,  // Age Encryption ('age-encryption.org/v1\n')
        0x62 => [&MACOS_ALIAS, &LZFSE] as __PV_62,  // macOS Alias ('book'), LZFSE compression ('bvx-', 'bvx1', 'bvx2', 'bvx$')
        0x63 => [&VHD] as __PV_63,  // Microsoft Virtual Hard Disk ('conectix')
        0x64 => [&TORRENT, &DEX, &DEY] as __PV_64,  // BitTorrent, DEX, DEY all start with 0x64 ('d')
        0x71 => [&QOI, &QOA] as __PV_71,  // Quite OK Image, Quite OK Audio
        0x76 => [&OPENEXR, &VHDX] as __PV_76,  // OpenEXR, VHDX ('vhdxfile')
        0x66 => [&FARBFELD, &FLAC, &FIGLET_FONT] as __PV_66,  // Farbfeld, FLAC, FigletFont
        0x67 => [&XCF, &GLB] as __PV_67,
        0x68 => [&SQUASHFS] as __PV_68,  // Squashfs little-endian ('hsqs')
        0x69 => [&MIFF, &ICNS] as __PV_69,  // MIFF ('id=ImageMagick'), Apple ICNS
        0x6B => [&DMG] as __PV_6B,  // Apple Disk Image
        0x70 => [&PLY] as __PV_70,
        0x73 => [&STL_ASCII, &SQUASHFS] as __PV_73,  // STL ASCII 3D models, Squashfs ('sqsh')
        0x74 => [&TTC] as __PV_74,
        0x77 => [&WOFF, &WOFF2, &WAVPACK] as __PV_77,
        0x78 => [&XAR] as __PV_78,
        0x7a => [&ZPAQ] as __PV_7A,  // ZPAQ also starts with "zPQ" (0x7A)
        0x7b => [&JSON_FEED, &GLYPHS] as __PV_7B,  // JSON Feed ('{"version'), Glyphs font ('{\n.appVe')
        0x7e => [&MIE] as __PV_7E,  // Meta Information Encapsulation
        0x7f => [&ELF, &DTS] as __PV_7F,  // ELF executables, DTS Audio
        0x80 => [&N64_ROM, &PYTHON_PICKLE, &CINEON] as __PV_80,  // N64 ROM (Z64 big-endian), Python Pickle (protocols 2-5), Cineon
        0x89 => [&PNG, &HDF5, &LZOP] as __PV_89,  // PNG, HDF5, LZOP all start with 0x89
        0x8a => [&MNG] as __PV_8A,  // Multiple-image Network Graphics
        0x8b => [&JNG] as __PV_8B,  // JPEG Network Graphics
        0xa1 => [&PCAP] as __PV_A1,  // NEW: PCAP big-endian
        0xab => [&KTX2, &KTX] as __PV_AB,  // Khronos Texture 2.0 first (longer signature)
        0xb7 => [&WTV] as __PV_B7,  // Windows Recorded TV Show
        0xc5 => [&EPS] as __PV_C5,  // Encapsulated PostScript (binary with preview)
        0xc7 => [&CPIO] as __PV_C7,  // NEW: CPIO binary variant
        0xca => [&CLASS] as __PV_CA,
        0xd0 => [&OLE] as __PV_D0,
        0xd4 => [&PCAP] as __PV_D4,  // NEW: PCAP little-endian
        0xd7 => [&CINEON] as __PV_D7,  // Cineon (little-endian)
        0xde => [&MO, &LLVM_BITCODE] as __PV_DE,  // Gettext MO (0xDE120495), LLVM wrapped bitcode (0xDEC017B)
        0xd9 => [&CBOR_FORMAT] as __PV_D9,
        0xed => [&RPM] as __PV_ED,
        0xef => [&UTF8_BOM] as __PV_EF,
        0xfd => [&XZ] as __PV_FD,
        0xfe => [&UTF16_BE, &JAVA_KEYSTORE] as __PV_FE,  // UTF16-BE and Java Keystore
        0xff => [&SKETCHUP, &WORKS_SPREADSHEET, &WINDOWS_REG, &JXL, &JPEG_LS, &JP2_CODESTREAM, &JPG, &MP2, &AAC, &UTF16_LE, &SNAPPY_FRAMED] as __PV_FF,  // SketchUp (UTF-16 LE + specific content), MS Works Spreadsheet, Windows Registry (UTF-16), JXL, JPEG-LS, JPEG 2000 Codestream, JPG, MP2, AAC, UTF-16 LE, Snappy framed
    ]
}

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
    "Binary Data",
    "",
    |_| true,
    &[
        // Complex formats that require offset or pattern checking
        // (Simple formats with clear first-byte signatures are in PREFIX_VEC)
        &JP2,       // Offset 4-8 check
        &JPX,       // Offset 4-8 check
        &JPM,       // Offset 4-8 check
        &WEBP,      // RIFF format (conflict)
        &TAR,       // No magic number
        &LOTUS123,  // Offset 4-7 check (parent; children WK1/WK3/WK4 refine version)
        &MP3,       // Multiple first bytes (conflict)
        &APE,       // Conflict with 0x4D
        &WAV,       // RIFF format (conflict)
        &AIFF,      // FORM format, offset 8
        &MPEG,      // Conflict with 0x00
        &QUICKTIME, // Offset 4-8 check
        &MQV,       // Offset 4-8 check
        &MP4,       // Offset 4-8 check
        &AVI,       // RIFF format (conflict)
        &MTV,       // RIFF format - MTV video
        &TTF,       // Multiple patterns (conflict)
        &EOT,       // 34 null bytes
        &DBF,       // Multiple first bytes
        &DCM,       // Offset 128 check
        &MOBI,      // Offset 60 check
        &DXF,       // Space patterns
        &WPD,       // Conflict with 0xFF
        &MACHO,     // Multiple magics (conflict)
        &QCP,       // RIFF format (conflict)
        &HDF,       // Parent format (children HDF4/HDF5 in PREFIX_VEC)
        &MRC,       // Offset checks
        &ZSTD,      // Range check on first 4 bytes
        &PAT,       // Offset 20 check
        &GBR,       // Offset 20 check
        &PCX,       // Conflict with 0x0A
        &CDR,       // RIFF format (conflict)
        &ILBM,      // IFF/FORM format
        &EMF,       // Offset 40 check
        &WMF,       // Multiple signatures
        &VDI,       // VirtualBox VDI - offset 64 check
        &OGG_OPUS,  // Offset 28 check
        &FIT,       // FIT format - offset 8 check
        &MPEG2TS,   // Pattern at offset 188
        &ACE,       // Offset 7 check
        &ISO9660,   // Large offset checks
        &ID3V2,     // Multiple signatures
        &ICC,       // Offset 36 check
        &PEM,       // Multiple signatures
        &EBML,      // Variable-length encoding
        &GBA_ROM,   // GameBoy Advance ROM - offset 4
        &GB_ROM,    // GameBoy ROM - offset 260 (parent to GBC_ROM)
        // PGP formats
        &PGP_MESSAGE,         // PGP encrypted/signed message
        &PGP_SIGNED_MESSAGE,  // PGP clear-signed message
        &PGP_PUBLIC_KEY,      // PGP public key block
        &PGP_PRIVATE_KEY,     // PGP private key block
        &PGP_SIGNATURE,       // PGP detached signature
        &MSO,                 // ActiveMime - offset 0x32 check
        &EMPTY,               // Empty file - zero-length check
        &PYTHON_BYTECODE,     // Python .pyc - checks offset 2-3
        &NINTENDO_SWITCH_NRO, // Nintendo Switch NRO - checks offset 0x10
        // Camera RAW formats (formats with clear signatures are in PREFIX_VEC)
        // Note: TIFF-based RAW formats (CR2, NEF, DNG, ARW, SR2, PEF, 3FR) are children of TIFF in PREFIX_VEC
        &CR3, // Canon Raw 3 (ISO Base Media) - offset check
        // Audio module formats (simple ones in PREFIX_VEC)
        &S3M, // Scream Tracker 3 Module - offset 44 check
        &MOD, // ProTracker Module - offset 1080 check
        // Sega game ROM formats (require larger READ_LIMIT for detection)
        &GENESIS_ROM,   // Sega Genesis/Mega Drive ROM - offset 0x100
        &GAME_GEAR_ROM, // Sega Game Gear ROM - offset 0x1ff0/0x3ff0/0x7ff0 (requires 32KB+)
        &SMS_ROM,       // Sega Master System ROM - offset 0x1ff0/0x3ff0/0x7ff0 (requires 32KB+)
        // Retro gaming formats (simple ones in PREFIX_VEC)
        &ATARI_7800_ROM,       // Atari 7800 ROM - offset 1 check
        &COMMODORE_64_PROGRAM, // Commodore 64 PRG - load address check
        // Text-based formats
        &UTF8, // Content validation (last)
    ],
)
.with_prefix_vec(&ROOT_PREFIX_VEC);

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
/// - Case insensitive matching (handles both `<html>` and `<HTML>`)
/// - Proper tag termination validation
/// - DOCTYPE declarations and comments
/// - Whitespace tolerance at the beginning of files
static HTML: MimeType = MimeType::new(TEXT_HTML, "HyperText Markup Language", ".html", html, &[])
    .with_extension_aliases(&[".htm"])
    .with_parent(&UTF8);

static XML: MimeType = MimeType::new(
    TEXT_XML,
    "Extensible Markup Language",
    ".xml",
    xml,
    &[
        &RSS, &ATOM, &X3D, &KML, &XLIFF, &COLLADA, &GML, &GPX, &TCX, &AMF, &THREEMF, &XFDF, &OWL2,
        &XHTML, &FB2, &USF,
    ],
)
.with_aliases(&[APPLICATION_XML])
.with_parent(&UTF8);

mimetype!(UTF8_BOM, TEXT_UTF8_BOM, ".txt", b"\xEF\xBB\xBF", name: "UTF-8 with BOM", kind: TEXT, children: [
    &HTML_UTF8_BOM,
    &XML_UTF8_BOM,
    &SVG_UTF8_BOM,
    &RTF_UTF8_BOM, // RTF must come before JSON (both start with {, RTF has more specific pattern)
    &JSON_UTF8_BOM,
    &CSV_UTF8_BOM,
    &TSV_UTF8_BOM,
    &PSV_UTF8_BOM,
    &SSV_UTF8_BOM,
    &SRT_UTF8_BOM,
    &VTT_UTF8_BOM,
    &VCARD_UTF8_BOM,
    &ICALENDAR_UTF8_BOM,
    &VISUAL_STUDIO_SOLUTION
]);

mimetype!(UTF16_BE, TEXT_UTF16_BE, ".txt", b"\xFE\xFF", name: "UTF-16 Big Endian", kind: TEXT, children: [
    &HTML_UTF16_BE,
    &XML_UTF16_BE,
    &SVG_UTF16_BE,
    &JSON_UTF16_BE,
    &CSV_UTF16_BE,
    &TSV_UTF16_BE,
    &PSV_UTF16_BE,
    &SSV_UTF16_BE,
    &SRT_UTF16_BE,
    &VTT_UTF16_BE,
    &VCARD_UTF16_BE,
    &ICALENDAR_UTF16_BE,
    &RTF_UTF16_BE
]);

mimetype!(UTF16_LE, TEXT_UTF16_LE, ".txt", b"\xFF\xFE", name: "UTF-16 Little Endian", kind: TEXT, children: [
    &HTML_UTF16_LE,
    &XML_UTF16_LE,
    &SVG_UTF16_LE,
    &JSON_UTF16_LE,
    &CSV_UTF16_LE,
    &TSV_UTF16_LE,
    &PSV_UTF16_LE,
    &SSV_UTF16_LE,
    &SRT_UTF16_LE,
    &VTT_UTF16_LE,
    &VCARD_UTF16_LE,
    &ICALENDAR_UTF16_LE,
    &RTF_UTF16_LE
]);

static UTF8: MimeType = MimeType::new(
    TEXT_UTF8,
    "UTF-8 Unicode Text",
    ".txt",
    utf8,
    &[
        &HTML,
        &XML,
        &RTF, // RTF must come before JSON (both start with {, RTF has more specific pattern)
        &VISUAL_STUDIO_SOLUTION,
        &LATEX,
        &CLOJURE,
        &PHP,
        &TYPESCRIPT, // TypeScript must come before JavaScript (TS is more specific)
        &JAVASCRIPT,
        &GO_LANG, // Go must come before Java (both use "package")
        &PERL,    // Perl must come before Java (both use "package")
        &CSHARP,  // C# must come before Java (both use "public class/interface")
        &VB,      // Visual Basic .NET language
        &JAVA,
        &C_LANG, // C++ is a child of C (detects C++ features in .c/.h files)
        &RUST_LANG,
        &RUBY, // Ruby must come before Python (both use class/def, but Ruby has "end")
        &PYTHON,
        &LUA,
        &SHELL,
        &BATCH,
        &TCL,
        &JSON,
        &CSV_FORMAT,
        &TSV,
        &PSV,
        &SSV,
        &SRT,
        &VTT,
        &VCARD,
        &VCALENDAR, // vCalendar 1.0 must come before iCalendar (both start with BEGIN:VCALENDAR)
        &ICALENDAR,
        &SVG,
        &WARC,
        &EMAIL,
        &XBM,
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
    ".ts",
    ".tsx",
    ".java",
    ".c",
    ".h",
    ".cpp",
    ".cc",
    ".cxx",
    ".hpp",
    ".hxx",
    ".h++",
    ".go",
    ".rs",
    ".lua",
    ".pl",
    ".py",
    ".rb",
    ".json",
    ".geojson",
    ".ndjson",
    ".rtf",
    ".tcl",
    ".csv",
    ".tsv",
    ".psv",
    ".ssv",
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

mimetype!(PDF, APPLICATION_PDF, ".pdf", b"%PDF-", name: "Portable Document Format", kind: DOCUMENT, aliases: [APPLICATION_X_PDF], ext_aliases: [".ai"], children: [&AI]);

mimetype!(FDF, APPLICATION_VND_FDF, ".fdf", b"%FDF-", name: "Forms Data Format", kind: DOCUMENT);

static AI: MimeType = MimeType::new(
    APPLICATION_VND_ADOBE_ILLUSTRATOR,
    "Adobe Illustrator",
    ".ai",
    ai,
    &[],
)
.with_kind(MimeKind::IMAGE)
.with_parent(&PDF);

mimetype!(PS, APPLICATION_POSTSCRIPT, ".ps", b"%!PS-Adobe-", name: "PostScript", kind: DOCUMENT);

// Encapsulated PostScript - Binary EPS with TIFF/WMF preview
mimetype!(EPS, APPLICATION_EPS, ".eps", [0xC5, 0xD0, 0xD3, 0xC6], name: "Encapsulated PostScript", kind: DOCUMENT);

// OLE (Object Linking and Embedding) container format - parent of Microsoft Office and CAD formats
// Detection uses CLSID (Class ID) at dynamic offset (512 or 4096 bytes depending on version)
//
// Ordering principles:
// 1. FREQUENCY: Most common Office formats first (DOC, XLS, PPT) for performance
// 2. SYSTEM: Windows system files (MSI, MSG) are very common
// 3. SPECIALIZED: CAD and other specialized formats last
//
// Note: OLE detection doesn't have the same specificity issues as ZIP since each
// format has a unique CLSID, making false positives unlikely regardless of order.
// The ordering here is purely for performance optimization.
static OLE: MimeType = MimeType::new(
    APPLICATION_X_OLE_STORAGE,
    "OLE Compound Document",
    "",
    |input| input.starts_with(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1"),
    &[
        // Most common: Legacy Microsoft Office formats
        &DOC, // Word 97-2003
        &XLS, // Excel 97-2003
        &PPT, // PowerPoint 97-2003
        // Very common: Windows system & email
        &MSI, // Windows Installer
        &MSP, // Windows Installer Patch
        &MSG, // Outlook messages
        // Common: Other Office applications
        &VSD,     // Visio drawings
        &MPP,     // Project files
        &PUB,     // Publisher
        &ONENOTE, // OneNote
        // Less common: Legacy & specialized
        &WORKS_WPS, // Microsoft Works
        // CAD formats (SolidWorks)
        &SLDASM, // SolidWorks Assembly
        &SLDPRT, // SolidWorks Part
        &SLDDRW, // SolidWorks Drawing
        // CAD formats (Autodesk)
        &IAM,          // Inventor Assembly
        &IPT,          // Inventor Part
        &IDW,          // Inventor Drawing
        &IPN,          // Inventor Presentation
        &AUTODESK_MAX, // 3DS Max
        &SCDOC,        // SpaceClaim
        // Rare/specialized formats
        &AAF,           // Advanced Authoring Format
        &FASOO,         // DRM format
        &PGP_NET_SHARE, // PGP
    ],
)
.with_extension_aliases(&[
    ".xls", ".pub", ".ppt", ".doc", ".chm", ".one", ".mpp", ".vsd", ".wps", ".sldasm", ".slddrw",
    ".sldprt", ".iam", ".idw", ".ipn", ".ipt", ".scdoc", ".max",
])
.with_kind(MimeKind::DOCUMENT);

static AAF: MimeType = MimeType::new(
    APPLICATION_X_AAF,
    "Advanced Authoring Format",
    ".aaf",
    aaf,
    &[],
)
.with_parent(&OLE);

// ============================================================================
// ARCHIVE & COMPRESSION FORMATS
// ============================================================================
//
// Archive formats are prioritized by popularity and detection reliability:
// 1. Common formats like ZIP and 7Z come first
// 2. Enhanced variants provide more sophisticated detection
// 3. TAR uses checksum validation for reliability
// 4. Compression formats are organized by algorithm type

// 7-Zip archive format with distinctive signature.
// 7Z files start with a unique 6-byte signature that makes detection reliable.
// This format supports multiple compression algorithms and strong encryption.
mimetype!(SEVEN_Z, APPLICATION_X_7Z_COMPRESSED, ".7z", b"7z\xbc\xaf\x27\x1c", name: "7-Zip Archive", kind: ARCHIVE);

// ZIP container format - parent of many document, archive, and application formats
// IMPORTANT: Child ordering matters for correct detection!
//
// Ordering principles:
// 1. FREQUENCY: Most common formats first (DOCX, XLSX, PPTX) for performance
// 2. SPECIFICITY: More specific patterns MUST come before general ones to avoid false positives
//    Example: AIR (META-INF/AIR/application.xml) must come before JAR (META-INF/)
//    Example: ODM (text-master) must come before ODT (text) as "text" is a prefix
// 3. PATTERN CONFLICTS: When patterns overlap, the more specific format must be first
//
// Current ordering balances performance (common formats first) with correctness (specific before general)
mimetype!(ZIP, APPLICATION_ZIP, ".zip", b"PK\x03\x04" | b"PK\x05\x06" | b"PK\x07\x08", name: "ZIP Archive", kind: ARCHIVE,
aliases: [APPLICATION_X_ZIP, APPLICATION_X_ZIP_COMPRESSED],
ext_aliases: [".xlsx", ".docx", ".pptx", ".vsdx", ".epub", ".jar", ".war", ".ear", ".odt", ".ods", ".odp", ".odg", ".odf", ".sxc", ".kmz", ".ora", ".aab", ".appx", ".appxbundle", ".ipa", ".xap", ".air", ".fla", ".idml", ".vsix", ".xpi", ".xps", ".sda", ".sdc", ".sdd", ".sds", ".sdw", ".smf", ".sxd", ".sxi", ".sxm", ".sxw", ".stc", ".std", ".sti", ".stw", ".sgw", ".uop", ".uos", ".uot", ".usdz", ".sketch", ".123dx", ".f3d", ".fig", ".mxl", ".fbz"],
children: [
    // Most common: Office Open XML (checked first for performance)
    &DOCX, &XLSX, &PPTX,

    // Common: Android, eBooks
    &APK, &EPUB,

    // Common: OpenDocument formats (more specific patterns first)
    &ODM,      // text-master (must come before ODT)
    &ODT, &ODS, &ODP,

    // More specific META-INF patterns (must come before JAR)
    &AIR,      // META-INF/AIR/application.xml
    &EAR,      // META-INF/application.xml
    &WAR,      // WEB-INF/web.xml

    // Generic Java (after specific META-INF patterns)
    &JAR,      // META-INF/ or META-INF/MANIFEST.MF

    // Development tools
    &VSIX,

    // Mobile apps
    &IPA, &AAB, &APPX, &APPXBUNDLE,

    // Design & creative tools
    &SKETCH, &FIGMA, &IDML, &FLA,

    // Geographic & 3D
    &KMZ, &USDZ,

    // Other Office/productivity
    &VSDX, &XPS, &ODG, &ODF, &ODC, &ODB, &ORA,

    // StarOffice/legacy formats (templates have specific patterns)
    &STC, &STD, &STI, &STW, &SGW,  // Templates first (more specific)
    &SXC, &SXW, &SXI, &SXM, &SXD,  // Then base formats
    &SDA, &SDC, &SDD, &SDS, &SDW, &SMF,

    // Uniform Office Format (Chinese)
    &UOP, &UOS, &UOT,

    // CAD & 3D modeling
    &AUTODESK_123D, &FUSION_360,

    // Other specialized formats
    &XPI, &XAP, &MXL, &FBZ
]);

mimetype!(RAR, APPLICATION_X_RAR_COMPRESSED, ".rar", b"Rar!\x1a\x07\x00" | b"Rar!\x1a\x07\x01\x00", name: "RAR Archive", kind: ARCHIVE, aliases: [APPLICATION_X_RAR]);

mimetype!(GZIP, APPLICATION_GZIP, ".gz", b"\x1f\x8b", name: "GNU Zip", kind: ARCHIVE,
    aliases: [APPLICATION_X_GZIP, APPLICATION_X_GUNZIP, APPLICATION_GZIPPED,
              APPLICATION_GZIP_COMPRESSED, APPLICATION_X_GZIP_COMPRESSED, GZIP_DOCUMENT],
    ext_aliases: [".tgz", ".taz", ".abw"],
    children: [&ABW]);

static ABW: MimeType = MimeType::new(
    APPLICATION_X_ABIWORD,
    "AbiWord Document",
    ".abw",
    abw,
    &[&AWT],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&GZIP);

static TAR: MimeType =
    MimeType::new(APPLICATION_X_TAR, "Tape Archive", ".tar", tar, &[]).with_kind(MimeKind::ARCHIVE);

mimetype!(BZIP, APPLICATION_X_BZIP, ".bz", b"BZ0", name: "Bzip Archive", kind: ARCHIVE);

mimetype!(BZ2, APPLICATION_X_BZIP2, ".bz2", b"BZ", name: "Bzip2 Archive", kind: ARCHIVE);

mimetype!(XZ, APPLICATION_X_XZ, ".xz", b"\xfd7zXZ\x00", name: "XZ Compressed Archive", kind: ARCHIVE);

static ZSTD: MimeType = MimeType::new(APPLICATION_ZSTD, "Zstandard Compression", ".zst", zstd, &[])
    .with_kind(MimeKind::ARCHIVE);

mimetype!(LZIP, APPLICATION_LZIP, ".lz", b"LZIP", name: "Lzip Compressed Archive", kind: ARCHIVE, aliases: [APPLICATION_X_LZIP]);

// LZ4 - Fast compression format
mimetype!(LZ4, APPLICATION_X_LZ4, ".lz4", [0x04, 0x22, 0x4D, 0x18], name: "LZ4 Compressed Archive", kind: ARCHIVE);

mimetype!(CAB, APPLICATION_VND_MS_CAB_COMPRESSED, ".cab", b"MSCF", name: "Microsoft Cabinet Archive", kind: ARCHIVE);

static INSTALL_SHIELD_CAB: MimeType = MimeType::new(
    APPLICATION_X_INSTALLSHIELD,
    "InstallShield Cabinet Archive",
    ".cab",
    install_shield_cab,
    &[],
)
.with_kind(MimeKind::ARCHIVE);

static CPIO: MimeType = MimeType::new(APPLICATION_X_CPIO, "CPIO Archive", ".cpio", cpio, &[])
    .with_kind(MimeKind::ARCHIVE);

mimetype!(AR, APPLICATION_X_ARCHIVE, ".a", b"!<arch>", name: "Unix Archive", kind: ARCHIVE, aliases: [APPLICATION_X_UNIX_ARCHIVE], ext_aliases: [".deb"], children: [&DEB]);

mimetype!(RPM, APPLICATION_X_RPM, ".rpm", b"\xed\xab\xee\xdb", name: "Red Hat Package Manager", kind: ARCHIVE);

mimetype!(TORRENT, APPLICATION_X_BITTORRENT, ".torrent", b"d8:announce" | b"d7:comment" | b"d4:info", name: "BitTorrent Metadata", kind: ARCHIVE);

mimetype!(FITS, APPLICATION_FITS, ".fits", b"SIMPLE  =                    T", name: "Flexible Image Transport System", kind: IMAGE, aliases: [IMAGE_FITS]);

mimetype!(XAR, APPLICATION_X_XAR, ".xar", b"xar!", name: "eXtensible ARchive", kind: ARCHIVE);

// ARJ - Legacy DOS compression format
mimetype!(ARJ, APPLICATION_ARJ, ".arj", [0x60, 0xEA], name: "ARJ Archive", kind: ARCHIVE);

// LHA/LZH - Japanese compression standard
mimetype!(LHA, APPLICATION_X_LZH_COMPRESSED, ".lzh", b"-lh", name: "LHA Archive", kind: ARCHIVE);

// LArc/LZS - Legacy Japanese compression format (similar to LZH)
mimetype!(LZS, APPLICATION_X_LZS_COMPRESSED, ".lzs", b"-lz", name: "LArc Archive", kind: ARCHIVE);

// DEB - Debian package, checks for "debian-binary" at offset 8
mimetype!(DEB, APPLICATION_VND_DEBIAN_BINARY_PACKAGE, ".deb", offset: (8, b"debian-binary"), name: "Debian Package", kind: ARCHIVE, parent: &AR);

// ACE Archive - Popular compression format in the early 2000s.
static ACE: MimeType = MimeType::new(
    APPLICATION_X_ACE_COMPRESSED,
    "ACE Archive",
    ".ace",
    |input| input.len() >= 14 && &input[7..14] == b"**ACE**",
    &[],
)
.with_kind(MimeKind::ARCHIVE);

// ISO 9660 CD/DVD Image - Standard format for optical disc images.
static ISO9660: MimeType = MimeType::new(
    APPLICATION_X_ISO9660_IMAGE,
    "ISO 9660",
    ".iso",
    |input| {
        (input.len() >= 32774 && &input[32769..32774] == b"CD001")
            || (input.len() >= 34822 && &input[34817..34822] == b"CD001")
            || (input.len() >= 36870 && &input[36865..36870] == b"CD001")
    },
    &[],
)
.with_kind(MimeKind::ARCHIVE);

// ALZ Archive - Korean compression format.
mimetype!(ALZ, APPLICATION_X_ALZ_COMPRESSED, ".alz", b"ALZ\x01", name: "ALZ Archive", kind: ARCHIVE);

// StuffIt Archive - Classic Mac compression format.
mimetype!(STUFFIT, APPLICATION_X_STUFFIT, ".sit", b"SIT!", name: "StuffIt Archive", kind: ARCHIVE);

// StuffIt X Archive - Improved Mac compression format.
mimetype!(STUFFITX, APPLICATION_X_STUFFITX, ".sitx", b"StuffIt ", name: "StuffIt X Archive", kind: ARCHIVE);

mimetype!(WARC, APPLICATION_WARC, ".warc", b"WARC/1.0" | b"WARC/1.1", name: "Web Archive", kind: ARCHIVE, parent: &UTF8);

/// Email message (RFC822)
static EMAIL: MimeType = MimeType::new(
    MESSAGE_RFC822,
    "Email Message",
    ".eml",
    |input| {
        // Email messages typically start with "From " or "From: " or other RFC822 headers
        input.len() >= 5
            && (input.starts_with(b"From ")
                || input.starts_with(b"From:")
                || input.starts_with(b"Date:")
                || input.starts_with(b"Subject:")
                || input.starts_with(b"To:")
                || input.starts_with(b"Received:"))
    },
    &[],
)
.with_kind(MimeKind::TEXT)
.with_parent(&UTF8);

// ============================================================================
// UTF-16 TEXT FORMAT VARIANTS
// ============================================================================

/// HTML format for UTF-16 Big Endian
static HTML_UTF16_BE: MimeType = MimeType::new(
    TEXT_HTML_UTF16,
    "HyperText Markup Language (UTF-16 BE)",
    ".html",
    html_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// HTML format for UTF-16 Little Endian
static HTML_UTF16_LE: MimeType = MimeType::new(
    TEXT_HTML_UTF16,
    "HyperText Markup Language (UTF-16 LE)",
    ".html",
    html_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// XML format for UTF-16 Big Endian
static XML_UTF16_BE: MimeType = MimeType::new(
    TEXT_XML_UTF16,
    "Extensible Markup Language (UTF-16 BE)",
    ".xml",
    xml_utf16_be,
    &[],
)
.with_aliases(&[APPLICATION_XML_UTF16])
.with_parent(&UTF16_BE);

/// XML format for UTF-16 Little Endian
static XML_UTF16_LE: MimeType = MimeType::new(
    TEXT_XML_UTF16,
    "Extensible Markup Language (UTF-16 LE)",
    ".xml",
    xml_utf16_le,
    &[],
)
.with_aliases(&[APPLICATION_XML_UTF16])
.with_parent(&UTF16_LE);

/// SVG format for UTF-16 Big Endian
static SVG_UTF16_BE: MimeType = MimeType::new(
    IMAGE_SVG_XML_UTF16,
    "Scalable Vector Graphics (UTF-16 BE)",
    ".svg",
    svg_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// SVG format for UTF-16 Little Endian
static SVG_UTF16_LE: MimeType = MimeType::new(
    IMAGE_SVG_XML_UTF16,
    "Scalable Vector Graphics (UTF-16 LE)",
    ".svg",
    svg_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// JSON format for UTF-16 Big Endian
static JSON_UTF16_BE: MimeType = MimeType::new(
    APPLICATION_JSON_UTF16,
    "JavaScript Object Notation (UTF-16 BE)",
    ".json",
    json_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// JSON format for UTF-16 Little Endian
static JSON_UTF16_LE: MimeType = MimeType::new(
    APPLICATION_JSON_UTF16,
    "JavaScript Object Notation (UTF-16 BE)",
    ".json",
    json_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// CSV format for UTF-16 Big Endian
static CSV_UTF16_BE: MimeType = MimeType::new(
    TEXT_CSV_UTF16,
    "Comma-Separated Values (UTF-16 BE)",
    ".csv",
    csv_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// CSV format for UTF-16 Little Endian
static CSV_UTF16_LE: MimeType = MimeType::new(
    TEXT_CSV_UTF16,
    "Comma-Separated Values (UTF-16 BE)",
    ".csv",
    csv_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// TSV format for UTF-16 Big Endian
static TSV_UTF16_BE: MimeType = MimeType::new(
    TEXT_TAB_SEPARATED_VALUES_UTF16,
    "Tab-Separated Values (UTF-16 BE)",
    ".tsv",
    tsv_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// TSV format for UTF-16 Little Endian
static TSV_UTF16_LE: MimeType = MimeType::new(
    TEXT_TAB_SEPARATED_VALUES_UTF16,
    "Tab-Separated Values (UTF-16 BE)",
    ".tsv",
    tsv_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// PSV format for UTF-16 Big Endian
static PSV_UTF16_BE: MimeType = MimeType::new(
    TEXT_PIPE_SEPARATED_VALUES_UTF16,
    "Pipe-Separated Values (UTF-16 BE)",
    ".psv",
    psv_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// PSV format for UTF-16 Little Endian
static PSV_UTF16_LE: MimeType = MimeType::new(
    TEXT_PIPE_SEPARATED_VALUES_UTF16,
    "Pipe-Separated Values (UTF-16 LE)",
    ".psv",
    psv_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// SSV format for UTF-16 Big Endian
static SSV_UTF16_BE: MimeType = MimeType::new(
    TEXT_SEMICOLON_SEPARATED_VALUES_UTF16,
    "Semicolon-Separated Values (UTF-16 BE)",
    ".ssv",
    ssv_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// SSV format for UTF-16 Little Endian
static SSV_UTF16_LE: MimeType = MimeType::new(
    TEXT_SEMICOLON_SEPARATED_VALUES_UTF16,
    "Semicolon-Separated Values (UTF-16 LE)",
    ".ssv",
    ssv_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// SRT subtitle format for UTF-16 Big Endian
static SRT_UTF16_BE: MimeType = MimeType::new(
    APPLICATION_X_SUBRIP_UTF16,
    "SubRip Subtitle (UTF-16 BE)",
    ".srt",
    srt_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// SRT subtitle format for UTF-16 Little Endian
static SRT_UTF16_LE: MimeType = MimeType::new(
    APPLICATION_X_SUBRIP_UTF16,
    "SubRip Subtitle (UTF-16 BE)",
    ".srt",
    srt_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// VTT subtitle format for UTF-16 Big Endian
static VTT_UTF16_BE: MimeType = MimeType::new(
    TEXT_VTT_UTF16,
    "Web Video Text Tracks (UTF-16 BE)",
    ".vtt",
    vtt_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// VTT subtitle format for UTF-16 Little Endian
static VTT_UTF16_LE: MimeType = MimeType::new(
    TEXT_VTT_UTF16,
    "Web Video Text Tracks (UTF-16 BE)",
    ".vtt",
    vtt_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// vCard format for UTF-16 Big Endian
static VCARD_UTF16_BE: MimeType = MimeType::new(
    TEXT_VCARD_UTF16,
    "vCard (UTF-16)",
    ".vcf",
    vcard_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// vCard format for UTF-16 Little Endian
static VCARD_UTF16_LE: MimeType = MimeType::new(
    TEXT_VCARD_UTF16,
    "vCard (UTF-16)",
    ".vcf",
    vcard_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// iCalendar format for UTF-16 Big Endian
static ICALENDAR_UTF16_BE: MimeType = MimeType::new(
    TEXT_CALENDAR_UTF16,
    "iCalendar (UTF-16)",
    ".ics",
    icalendar_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// iCalendar format for UTF-16 Little Endian
static ICALENDAR_UTF16_LE: MimeType = MimeType::new(
    TEXT_CALENDAR_UTF16,
    "iCalendar (UTF-16)",
    ".ics",
    icalendar_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// RTF format for UTF-16 Big Endian
static RTF_UTF16_BE: MimeType = MimeType::new(
    TEXT_RTF_UTF16,
    "Rich Text Format (UTF-16 BE)",
    ".rtf",
    rtf_utf16_be,
    &[],
)
.with_parent(&UTF16_BE);

/// RTF format for UTF-16 Little Endian
static RTF_UTF16_LE: MimeType = MimeType::new(
    TEXT_RTF_UTF16,
    "Rich Text Format (UTF-16 BE)",
    ".rtf",
    rtf_utf16_le,
    &[],
)
.with_parent(&UTF16_LE);

/// HTML format for UTF-8 with BOM
static HTML_UTF8_BOM: MimeType = MimeType::new(
    TEXT_HTML,
    "HyperText Markup Language (UTF-8 BOM)",
    ".html",
    html_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// XML format for UTF-8 with BOM
static XML_UTF8_BOM: MimeType = MimeType::new(
    TEXT_XML,
    "Extensible Markup Language (UTF-8 BOM)",
    ".xml",
    xml_utf8_bom,
    &[],
)
.with_aliases(&[APPLICATION_XML])
.with_parent(&UTF8_BOM);

/// SVG format for UTF-8 with BOM
static SVG_UTF8_BOM: MimeType = MimeType::new(
    IMAGE_SVG_XML,
    "Scalable Vector Graphics (UTF-8 BOM)",
    ".svg",
    svg_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// JSON format for UTF-8 with BOM
static JSON_UTF8_BOM: MimeType = MimeType::new(
    APPLICATION_JSON,
    "JavaScript Object Notation (UTF-8 BOM)",
    ".json",
    json_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// CSV format for UTF-8 with BOM
static CSV_UTF8_BOM: MimeType = MimeType::new(
    TEXT_CSV,
    "Comma-Separated Values (UTF-8 BOM)",
    ".csv",
    csv_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// TSV format for UTF-8 with BOM
static TSV_UTF8_BOM: MimeType = MimeType::new(
    TEXT_TAB_SEPARATED_VALUES,
    "Tab-Separated Values (UTF-8 BOM)",
    ".tsv",
    tsv_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// PSV format for UTF-8 with BOM
static PSV_UTF8_BOM: MimeType = MimeType::new(
    TEXT_PIPE_SEPARATED_VALUES,
    "Pipe-Separated Values (UTF-8 BOM)",
    ".psv",
    psv_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// SSV format for UTF-8 with BOM
static SSV_UTF8_BOM: MimeType = MimeType::new(
    TEXT_SEMICOLON_SEPARATED_VALUES,
    "Semicolon-Separated Values (UTF-8 BOM)",
    ".ssv",
    ssv_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// SRT subtitle format for UTF-8 with BOM
static SRT_UTF8_BOM: MimeType = MimeType::new(
    APPLICATION_X_SUBRIP,
    "SubRip Subtitle (UTF-8 BOM)",
    ".srt",
    srt_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// VTT subtitle format for UTF-8 with BOM
static VTT_UTF8_BOM: MimeType = MimeType::new(
    TEXT_VTT,
    "WebVTT Subtitle (UTF-8 BOM)",
    ".vtt",
    vtt_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// vCard format for UTF-8 with BOM
static VCARD_UTF8_BOM: MimeType =
    MimeType::new(TEXT_VCARD, "vCard (UTF-8 BOM)", ".vcf", vcard_utf8_bom, &[])
        .with_parent(&UTF8_BOM);

/// iCalendar format for UTF-8 with BOM
static ICALENDAR_UTF8_BOM: MimeType = MimeType::new(
    TEXT_CALENDAR,
    "iCalendar (UTF-8 BOM)",
    ".ics",
    icalendar_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

/// RTF format for UTF-8 with BOM
static RTF_UTF8_BOM: MimeType = MimeType::new(
    TEXT_RTF,
    "Rich Text Format (UTF-8 BOM)",
    ".rtf",
    rtf_utf8_bom,
    &[],
)
.with_parent(&UTF8_BOM);

// ============================================================================
// IMAGE FORMATS
// ============================================================================

mimetype!(PNG, IMAGE_PNG, ".png", b"\x89PNG\r\n\x1a\n", name: "Portable Network Graphics", kind: IMAGE, children: [&APNG]);

// APNG - Animated PNG, checks for acTL (Animation Control) chunk at offset 37
mimetype!(APNG, IMAGE_VND_MOZILLA_APNG, ".apng", offset: (37, b"acTL", prefix: (0, b"\x89PNG\r\n\x1a\n")), name: "Animated Portable Network Graphics", kind: IMAGE, parent: &PNG);

// MNG - Multiple-image Network Graphics, animated PNG-like format.
mimetype!(MNG, IMAGE_X_MNG, ".mng", [0x8A, 0x4D, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], name: "Multiple-image Network Graphics", kind: IMAGE);

// JNG - JPEG Network Graphics, JPEG with PNG-style chunks and optional alpha channel.
mimetype!(JNG, IMAGE_X_JNG, ".jng", [0x8B, 0x4A, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], name: "JPEG Network Graphics", kind: IMAGE);

mimetype!(JPG, IMAGE_JPEG, ".jpg", b"\xff\xd8\xff", name: "Joint Photographic Experts Group", kind: IMAGE, ext_aliases: [".jpeg", ".jpe", ".jif", ".jfif", ".jfi"]);

static JP2: MimeType =
    MimeType::new(IMAGE_JP2, "JPEG 2000 Image", ".jp2", jp2, &[]).with_kind(MimeKind::IMAGE);

static JPX: MimeType =
    MimeType::new(IMAGE_JPX, "JPEG 2000 Extended", ".jpx", jpx, &[]).with_kind(MimeKind::IMAGE);

static JPM: MimeType = MimeType::new(IMAGE_JPM, "JPEG 2000 Part 6", ".jpm", jpm, &[])
    .with_aliases(&[VIDEO_JPM])
    .with_kind(MimeKind::IMAGE);

// JPEG 2000 Codestream - Raw codestream without JP2 container
mimetype!(JP2_CODESTREAM, IMAGE_X_JP2_CODESTREAM, ".j2c", b"\xff\x4f\xff\x51", name: "JPEG 2000 Codestream", kind: IMAGE, ext_aliases: [".jpc", ".j2k"]);

mimetype!(JXS, IMAGE_JXS, ".jxs", b"\x00\x00\x00\x0C\x4A\x58\x53\x20\x0D\x0A\x87\x0A", name: "JPEG XS", kind: IMAGE);

mimetype!(JXR, IMAGE_JXR, ".jxr", b"\x49\x49\xBC\x01", name: "JPEG XR", kind: IMAGE, aliases: [IMAGE_VND_MS_PHOTO]);

mimetype!(JXL, IMAGE_JXL, ".jxl", b"\xFF\x0A" | b"\x00\x00\x00\x0CJXL \x0D\x0A\x87\x0A", name: "JPEG XL", kind: IMAGE);

mimetype!(GIF, IMAGE_GIF, ".gif", b"GIF87a" | b"GIF89a", name: "Graphics Interchange Format", kind: IMAGE);

mimetype!(WEBP, IMAGE_WEBP, ".webp", offset: (8, b"WEBP", prefix: (0, b"RIFF")), name: "WebP Image", kind: IMAGE);

// Forward declarations for TIFF children
// Canon Raw 2 - TIFF-based with CR2 marker
mimetype!(CR2, IMAGE_X_CANON_CR2, ".cr2", offset: (8, b"CR\x02\x00"), name: "Canon Raw 2", kind: IMAGE);

// Nikon Electronic File - TIFF-based
static NEF: MimeType = MimeType::new(
    IMAGE_X_NIKON_NEF,
    "Nikon NEF",
    ".nef",
    |input| {
        // Don't check TIFF header here - parent already checked it
        // Look for Nikon signature in the file
        input.len() >= 256 && input[0..256].windows(5).any(|w| w == b"NIKON")
    },
    &[],
)
.with_kind(MimeKind::IMAGE);

static TIFF: MimeType = MimeType::new(
    IMAGE_TIFF,
    "Tagged Image File Format",
    ".tiff",
    |input| input.starts_with(b"II*\x00") || input.starts_with(b"MM\x00*"),
    &[&CR2, &NEF, &HASSELBLAD_3FR, &SR2, &ARW, &PEF, &DNG], // TIFF-based RAW formats as children (larger/more specific first)
)
.with_extension_aliases(&[".tif"])
.with_kind(MimeKind::IMAGE);

mimetype!(BMP, IMAGE_BMP, ".bmp", b"BM", name: "Bitmap Image File", kind: IMAGE, aliases: [IMAGE_X_BMP, IMAGE_X_MS_BMP], ext_aliases: [".dib"]);

mimetype!(ICO, IMAGE_X_ICON, ".ico", b"\x00\x00\x01\x00", name: "Icon File", kind: IMAGE);

mimetype!(ICNS, IMAGE_X_ICNS, ".icns", b"icns", name: "Apple Icon Image", kind: IMAGE);

mimetype!(PSD, IMAGE_VND_ADOBE_PHOTOSHOP, ".psd", b"8BPS", name: "Adobe Photoshop Document", kind: IMAGE, aliases: [IMAGE_X_PSD, APPLICATION_PHOTOSHOP]);

mimetype!(PBM, IMAGE_X_PORTABLE_BITMAP, ".pbm", b"P1" | b"P4", name: "Portable Bitmap", kind: IMAGE);

mimetype!(PGM, IMAGE_X_PORTABLE_GRAYMAP, ".pgm", b"P2" | b"P5", name: "Portable Graymap", kind: IMAGE);

mimetype!(PPM, IMAGE_X_PORTABLE_PIXMAP, ".ppm", b"P3" | b"P6", name: "Portable Pixmap", kind: IMAGE);

mimetype!(PAM, IMAGE_X_PORTABLE_ARBITRARYMAP, ".pam", b"P7", name: "Portable Arbitrary Map", kind: IMAGE);

static HEIC: MimeType = MimeType::new(
    IMAGE_HEIC,
    "High Efficiency Image Container",
    ".heic",
    heic,
    &[],
)
.with_kind(MimeKind::IMAGE)
.with_parent(&HEIF);

static HEIF: MimeType = MimeType::new(
    IMAGE_HEIF,
    "High Efficiency Image Format",
    ".heif",
    heif,
    &[],
)
.with_kind(MimeKind::IMAGE);

mimetype!(HEIF_SEQ, IMAGE_HEIF_SEQUENCE, ".heif", offset: (4, b"ftypmsf1"), name: "High Efficiency Image Format Sequence", kind: IMAGE, ext_aliases: [".heifs"]);

mimetype!(HEIC_SEQ, IMAGE_HEIC_SEQUENCE, ".heic", offset: (4, b"ftyphevc"), name: "High Efficiency Image Container Sequence", kind: IMAGE, ext_aliases: [".heics"], parent: &HEIF);

mimetype!(BPG, IMAGE_BPG, ".bpg", b"BPG\xFB", name: "Better Portable Graphics", kind: IMAGE);

mimetype!(XCF, IMAGE_X_XCF, ".xcf", b"gimp xcf", name: "GIMP Image", kind: IMAGE);

mimetype!(PAT, IMAGE_X_GIMP_PAT, ".pat", offset: (20, b"GPAT"), name: "GIMP Pattern", kind: IMAGE);

mimetype!(GBR, IMAGE_X_GIMP_GBR, ".gbr", offset: (20, b"GIMP"), name: "GIMP Brush", kind: IMAGE);

mimetype!(HDR, IMAGE_VND_RADIANCE, ".hdr", b"#?RADIANCE\n", name: "Radiance HDR Image", kind: IMAGE);

mimetype!(XPM, IMAGE_X_XPIXMAP, ".xpm", b"/* XPM */", name: "X PixMap", kind: IMAGE);

// X11 Bitmap format (C header format)
mimetype!(XBM, IMAGE_X_XBITMAP, ".xbm", b"#define ", name: "X BitMap", kind: IMAGE, parent: &UTF8);

static DWG: MimeType = MimeType::new(IMAGE_VND_DWG, "AutoCAD Drawing", ".dwg", dwg, &[])
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

mimetype!(DXF, IMAGE_VND_DXF, ".dxf", b"  0\x0ASECTION\x0A" | b"  0\x0D\x0ASECTION\x0D\x0A" | b"0\x0ASECTION\x0A" | b"0\x0D\x0ASECTION\x0D\x0A", name: "Drawing Exchange Format", kind: IMAGE);

// DXF Binary - AutoCAD Drawing Exchange Format binary variant
mimetype!(DXF_BINARY, APPLICATION_X_DXF, ".dxf", b"AutoCAD Binary DXF", name: "Drawing Exchange Format Binary", kind: MODEL);

// DjVu document format
mimetype!(DJVU, IMAGE_VND_DJVU, ".djvu", offset: (12, b"DJVU", prefix: (0, b"AT&TFORM")), name: "DjVu Document", kind: IMAGE);

// DirectDraw Surface - Game textures
mimetype!(DDS, IMAGE_VND_MS_DDS, ".dds", b"DDS ", name: "DirectDraw Surface", kind: IMAGE);

// PC Paintbrush - Classic bitmap format
mimetype!(PCX, IMAGE_X_PCX, ".pcx", [0x0A], name: "PC Paintbrush", kind: IMAGE);

// PICtor/PC Paint - DOS graphics format
mimetype!(PICTOR, IMAGE_X_PICTOR, ".pic", [0x34, 0x12], name: "PICtor PC Paint", kind: IMAGE);

// Khronos Texture - OpenGL/Vulkan textures
mimetype!(KTX, IMAGE_KTX, ".ktx", [0xAB, 0x4B, 0x54, 0x58, 0x20], name: "Khronos Texture", kind: IMAGE);

// ARM Texture Compression - Mobile GPU textures
mimetype!(ASTC, IMAGE_X_ASTC, ".astc", [0x13, 0xAB, 0xA1, 0x5C], name: "Adaptive Scalable Texture Compression", kind: IMAGE);

// Truevision TGA/Targa - Gaming and 3D graphics format
mimetype!(TGA, IMAGE_X_TGA, ".tga", [0x00, 0x01, 0x0A, 0x00], name: "Truevision Targa", kind: IMAGE);

// Sun Raster - Legacy Unix image format
mimetype!(SUN_RASTER, IMAGE_X_SUN_RASTER, ".ras", [0x59, 0xA6, 0x6A, 0x95], name: "Sun Raster Image", kind: IMAGE);

// Silicon Graphics Image - Film/VFX format
mimetype!(SGI, IMAGE_X_SGI, ".sgi", [0x01, 0xDA], name: "Silicon Graphics Image", kind: IMAGE);

// Windows Animated Cursor - RIFF container
mimetype!(ANI, APPLICATION_X_NAVI_ANIMATION, ".ani", offset: (8, b"ACON", prefix: (0, b"RIFF")), name: "Windows Animated Cursor", kind: IMAGE);

// CorelDRAW - RIFF container
mimetype!(CDR, APPLICATION_VND_COREL_DRAW, ".cdr", offset: (8, b"CDR", prefix: (0, b"RIFF")), name: "CorelDRAW Image", kind: IMAGE, aliases: [APPLICATION_CDR, APPLICATION_X_CDR]);

// IFF/ILBM - Amiga graphics format (FORM container)
mimetype!(ILBM, IMAGE_X_ILBM, ".lbm", offset: (8, b"ILBM", prefix: (0, b"FORM")), name: "Interchange File Format", kind: IMAGE, aliases: [IMAGE_X_IFF], ext_aliases: [".iff", ".ilbm"]);

// AVIF Sequence - Animated AVIF images
mimetype!(AVIF_SEQUENCE, IMAGE_AVIF_SEQUENCE, ".avifs", offset: (4, b"ftypavis"), name: "AV1 Image File Format Sequence", kind: IMAGE);

mimetype!(AVIF_FORMAT, IMAGE_AVIF, ".avif", offset: (4, b"ftypavif"), name: "AV1 Image File Format", kind: IMAGE, children: [&AVIF_SEQUENCE]);

// Quite OK Image Format - A fast, lossless image format.
mimetype!(QOI, IMAGE_X_QOI, ".qoi", b"qoif", name: "Quite OK Image Format", kind: IMAGE);

// FLIF - Free Lossless Image Format (deprecated, concepts moved to JPEG XL).
mimetype!(FLIF, IMAGE_FLIF, ".flif", b"FLIF", name: "Free Lossless Image Format", kind: IMAGE);

// Khronos Texture 2.0 - Modern GPU texture format for games and 3D applications.
mimetype!(KTX2, IMAGE_KTX2, ".ktx2", b"\xABKTX 20\xBB\r\n\x1A\n", name: "Khronos Texture 2.0", kind: IMAGE);

// OpenEXR - High Dynamic Range imaging format used in visual effects and film.
mimetype!(OPENEXR, IMAGE_X_EXR, ".exr", b"\x76\x2F\x31\x01", name: "OpenEXR High Dynamic Range Image", kind: IMAGE);

// Farbfeld - Suckless lossless image format designed for simplicity and piping in UNIX.
mimetype!(FARBFELD, IMAGE_X_FARBFELD, ".ff", b"farbfeld", name: "Farbfeld Image Format", kind: IMAGE);

// JPEG-LS - Lossless/near-lossless JPEG compression standard (ISO-14495-1).
// Used primarily in medical imaging applications.
mimetype!(JPEG_LS, IMAGE_JLS, ".jls", [0xFF, 0xD8, 0xFF, 0xF7], name: "JPEG-LS Lossless Image", kind: IMAGE);

// MIFF - Magick Image File Format, ImageMagick's native format.
mimetype!(MIFF, IMAGE_X_MIFF, ".miff", b"id=ImageMagick", name: "Magick Image File Format", kind: IMAGE);

// PFM - Portable FloatMap, Netpbm HDR format for floating-point pixel data.
// Magic bytes: "PF\n" (RGB color) or "Pf\n" (grayscale).
mimetype!(PFM, IMAGE_X_PFM, ".pfm", b"PF\n" | b"Pf\n", name: "Portable FloatMap", kind: IMAGE);

// Enhanced Metafile - Windows vector graphics format.
mimetype!(EMF, IMAGE_EMF, ".emf", offset: (40, b" EMF", prefix: (0, b"\x01\x00\x00\x00")), name: "Enhanced Metafile", kind: IMAGE);

// Windows Metafile - Legacy Windows vector graphics format.
mimetype!(WMF, IMAGE_WMF, ".wmf", b"\x01\x00\x09\x00" | b"\x02\x00\x09\x00" | b"\xD7\xCD\xC6\x9A", name: "Windows Metafile", kind: IMAGE);

// ============================================================================
// AUDIO FORMATS
// ============================================================================

static MP3: MimeType = MimeType::new(AUDIO_MPEG, "MPEG Audio Layer III", ".mp3", mp3, &[])
    .with_aliases(&[AUDIO_X_MPEG, AUDIO_MP3])
    .with_kind(MimeKind::AUDIO);

// MPEG-1/2 Audio Layer 2 - Predecessor to MP3, still used in broadcasting
static MP2: MimeType =
    MimeType::new(AUDIO_MP2, "MPEG Audio Layer II", ".mp2", mp2, &[]).with_kind(MimeKind::AUDIO);

mimetype!(FLAC, AUDIO_FLAC, ".flac", b"fLaC", name: "Free Lossless Audio Codec", kind: AUDIO, aliases: [AUDIO_X_FLAC]);

mimetype!(WAV, AUDIO_WAV, ".wav", offset: (8, b"WAVE", prefix: (0, b"RIFF")), name: "Waveform Audio File", kind: AUDIO, aliases: [AUDIO_X_WAV, AUDIO_VND_WAVE, AUDIO_WAVE]);

// SoundFont 2 format (RIFF-based)
mimetype!(SOUNDFONT2, AUDIO_X_SOUNDFONT, ".sf2", offset: (8, b"sfbk", prefix: (0, b"RIFF")), name: "SoundFont 2.0", kind: AUDIO);

mimetype!(AIFF, AUDIO_AIFF, ".aiff", offset: (8, b"AIFF", prefix: (0, b"FORM")), name: "Audio Interchange File Format", kind: AUDIO, aliases: [AUDIO_X_AIFF], ext_aliases: [".aif"]);

mimetype!(MIDI, AUDIO_MIDI, ".midi", b"MThd", name: "Musical Instrument Digital Interface", kind: AUDIO, aliases: [AUDIO_MID], ext_aliases: [".mid"]);

mimetype!(OGG, APPLICATION_OGG, ".ogg", b"OggS", name: "Ogg Container Format", kind: AUDIO, aliases: [APPLICATION_X_OGG], children: [&OGG_AUDIO, &OGG_MEDIA, &OGG_VIDEO, &OGG_MULTIPLEXED, &SPX]);

static OGG_AUDIO: MimeType = MimeType::new(AUDIO_OGG, "Ogg Audio", ".oga", ogg_audio, &[])
    .with_extension_aliases(&[".opus"])
    .with_kind(MimeKind::AUDIO)
    .with_parent(&OGG);

static OGG_VIDEO: MimeType = MimeType::new(VIDEO_OGG, "Ogg Video", ".ogv", ogg_video, &[])
    .with_kind(MimeKind::VIDEO)
    .with_parent(&OGG);

static OGG_MEDIA: MimeType = MimeType::new(VIDEO_OGG_MEDIA, "Ogg Media", ".ogm", ogg_media, &[])
    .with_kind(MimeKind::VIDEO)
    .with_parent(&OGG);

static OGG_MULTIPLEXED: MimeType = MimeType::new(
    APPLICATION_OGG_MULTIPLEXED,
    "Ogg Multiplexed",
    ".ogx",
    ogg_multiplexed,
    &[],
)
.with_kind(MimeKind::VIDEO)
.with_parent(&OGG);

mimetype!(APE, AUDIO_APE, ".ape", b"MAC \x96\x0F\x00\x00\x34\x00\x00\x00\x18\x00\x00\x00\x90\xE3", name: "Monkey's Audio", kind: AUDIO);

mimetype!(MUSEPACK, AUDIO_MUSEPACK, ".mpc", b"MPCK", name: "Musepack Audio", kind: AUDIO);

mimetype!(AU, AUDIO_BASIC, ".au", b".snd", name: "Sun/NeXT Audio", kind: AUDIO, ext_aliases: [".snd"]);

mimetype!(AMR, AUDIO_AMR, ".amr", b"#!AMR", name: "Adaptive Multi-Rate Audio", kind: AUDIO, aliases: [AUDIO_AMR_NB]);

// Creative Voice audio format (DOS/Sound Blaster)
mimetype!(VOC, AUDIO_X_VOC, ".voc", b"Creative Voice File", name: "Creative Voice Audio", kind: AUDIO);

// RealAudio format
// RealAudio can start with .RA\xFD or .ra\xFD
mimetype!(REALAUDIO, AUDIO_X_REALAUDIO, ".ra", [0x2E, 0x52, 0x41, 0xFD] | [0x2E, 0x72, 0x61, 0xFD], name: "RealAudio", kind: AUDIO);

// Audio Codec 3 (Dolby Digital) - Common audio codec used in DVDs and digital TV.
mimetype!(AC3, AUDIO_AC3, ".ac3", b"\x0B\x77", name: "Dolby Digital Audio", kind: AUDIO);

// DTS Audio - Digital Theater Systems surround sound, used in Blu-ray and home theater.
mimetype!(DTS, AUDIO_DTS, ".dts", b"\x7F\xFE\x80\x01", name: "Digital Theater Systems Audio", kind: AUDIO, aliases: [AUDIO_DTS_HD]);

// Ogg Opus - Modern, high-quality audio codec with low latency.
static OGG_OPUS: MimeType = MimeType::new(
    AUDIO_OPUS,
    "Opus Audio",
    ".opus",
    |input| input.len() >= 36 && input.starts_with(b"OggS") && &input[28..36] == b"OpusHead",
    &[],
)
.with_kind(MimeKind::AUDIO)
.with_parent(&OGG);

mimetype!(M3U, AUDIO_X_MPEGURL, ".m3u", b"#EXTM3U", name: "M3U Playlist", kind: TEXT, aliases: [AUDIO_MPEGURL], ext_aliases: [".m3u8"]);

mimetype!(AAC, AUDIO_AAC, ".aac", b"\xFF\xF1" | b"\xFF\xF9", name: "Advanced Audio Coding", kind: AUDIO);

mimetype!(QCP, AUDIO_QCELP, ".qcp", offset: (8, b"QLCM", prefix: (0, b"RIFF")), name: "Qualcomm PureVoice Audio", kind: AUDIO);

mimetype!(M4A, AUDIO_X_M4A, ".m4a", offset: (8, b"M4A ", prefix: (4, b"ftyp")), name: "MPEG-4 Audio", kind: AUDIO);

// Apple iTunes Audiobook - MP4-based audiobook format
mimetype!(M4B, AUDIO_MP4, ".m4b", offset: (8, b"M4B ", prefix: (4, b"ftyp")), name: "Apple iTunes Audiobook", kind: AUDIO);

// Apple iTunes Protected Audio - DRM-protected MP4 audio
mimetype!(M4P, AUDIO_MP4, ".m4p", offset: (8, b"M4P ", prefix: (4, b"ftyp")), name: "Apple iTunes Protected Audio", kind: AUDIO);

// Flash MP4 Audio - Adobe Flash MP4 audio format
mimetype!(F4A, AUDIO_MP4, ".f4a", offset: (8, b"F4A ", prefix: (4, b"ftyp")), name: "Flash MP4 Audio", kind: AUDIO);

// Flash MP4 Audiobook - Adobe Flash MP4 audiobook format
mimetype!(F4B, AUDIO_MP4, ".f4b", offset: (8, b"F4B ", prefix: (4, b"ftyp")), name: "Flash MP4 Audiobook", kind: AUDIO);

// Merged AMP4 into MP4 below

// WavPack - Lossless/lossy audio compression
mimetype!(WAVPACK, AUDIO_X_WAVPACK, ".wv", b"wvpk", name: "WavPack Audio", kind: AUDIO);

// True Audio - Lossless audio codec
mimetype!(TTA, AUDIO_X_TTA, ".tta", b"TTA1", name: "True Audio", kind: AUDIO);

// DSD Stream File - Direct Stream Digital audio
mimetype!(DSF, AUDIO_X_DSF, ".dsf", b"DSD ", name: "DSD Stream Audio", kind: AUDIO);

// DSD Interchange File - Direct Stream Digital audio
mimetype!(DFF, AUDIO_X_DFF, ".dff", b"FRM8", name: "DSD Interchange Audio", kind: AUDIO);

// Quite OK Audio - Modern lossless audio format
mimetype!(QOA, AUDIO_X_QOA, ".qoa", b"qoaf", name: "Quite OK Audio", kind: AUDIO);

// 8SVX Audio - Amiga IFF audio format
mimetype!(EIGHTSVX, AUDIO_X_8SVX, ".8svx", offset: (8, b"8SVX", prefix: (0, b"FORM")), name: "Amiga 8SVX Audio", kind: AUDIO, ext_aliases: [".8sv"]);

// Audio Visual Research - Audio format used in Atari ST applications
mimetype!(AVR, AUDIO_X_AVR, ".avr", b"2BIT", name: "Audio Visual Research", kind: AUDIO);

// ============================================================================
// VIDEO FORMATS
// ============================================================================

static MP4: MimeType = MimeType::new(
    VIDEO_MP4,
    "Video Mp4",
    ".mp4",
    mp4_precise,
    &[
        &AVIF_FORMAT,
        &AVIF_SEQUENCE,
        &THREE_GPP,
        &THREE_GPP2,
        &M4A,
        &M4B,
        &M4P,
        &M4V,
        &F4A,
        &F4B,
        &F4V,
        &F4P,
        &HEIC,
        &HEIC_SEQ,
        &HEIF,
        &HEIF_SEQ,
        &MJ2,
        &DVB,
    ],
)
.with_aliases(&[AUDIO_MP4, AUDIO_X_M4A, AUDIO_X_MP4A])
.with_kind(MimeKind::AUDIO.union(MimeKind::VIDEO));

static WEBM: MimeType = MimeType::new(VIDEO_WEBM, "WebM", ".webm", webm, &[])
    .with_aliases(&[AUDIO_WEBM])
    .with_kind(MimeKind::VIDEO);

static MKV: MimeType = MimeType::new(VIDEO_X_MATROSKA, "Matroska", ".mkv", mkv, &[])
    .with_extension_aliases(&[".mk3d", ".mka", ".mks"])
    .with_kind(MimeKind::VIDEO);

mimetype!(AVI, VIDEO_X_MSVIDEO, ".avi", offset: (8, b"AVI LIST", prefix: (0, b"RIFF")), name: "Audio Video Interleave", kind: VIDEO, aliases: [VIDEO_AVI, VIDEO_MSVIDEO]);

// MPEG Video (.mpg) - 00 00 01 B3
static MPEG_VIDEO: MimeType = MimeType::new(
    VIDEO_MPEG,
    "MPEG Video",
    ".mpg",
    |input| input.len() >= 4 && matches!(input, [0x00, 0x00, 0x01, 0xB3, ..]),
    &[],
)
.with_kind(MimeKind::VIDEO);

// DVD Video Object / MPEG-2 Program Stream (.vob, .m2p) - 00 00 01 BA
static VOB: MimeType = MimeType::new(
    VIDEO_MPEG,
    "DVD Video Object",
    ".vob",
    |input| input.len() >= 4 && matches!(input, [0x00, 0x00, 0x01, 0xBA, ..]),
    &[],
)
.with_extension_aliases(&[".m2p"])
.with_kind(MimeKind::VIDEO);

static MPEG: MimeType = MimeType::new(
    VIDEO_MPEG,
    "MPEG Video",
    ".mpeg",
    mpeg,
    &[&MPEG_VIDEO, &VOB],
)
.with_kind(MimeKind::VIDEO);

mimetype!(QUICKTIME, VIDEO_QUICKTIME, ".mov", offset: (8, b"qt  ", prefix: (4, b"ftyp")), name: "QuickTime Video", kind: VIDEO);

mimetype!(MQV, VIDEO_QUICKTIME, ".mqv", offset: (8, b"mqt ", prefix: (4, b"ftyp")), name: "QuickTime MQV Video", kind: VIDEO);

mimetype!(FLV, VIDEO_X_FLV, ".flv", b"FLV", name: "Flash Video", kind: VIDEO);

mimetype!(ASF, VIDEO_X_MS_ASF, ".asf", b"\x30\x26\xb2\x75\x8e\x66\xcf\x11\xa6\xd9\x00\xaa\x00\x62\xce\x6c", name: "Advanced Systems Format", kind: VIDEO, aliases: [VIDEO_ASF, VIDEO_X_MS_WMV], ext_aliases: [".asx", ".dvr-ms", ".wma", ".wmv"], children: [&WMA, &WMV, &DVR_MS, &ASX]);

static DVR_MS: MimeType = MimeType::new(
    VIDEO_X_MS_DVR,
    "Microsoft Digital Video Recording",
    ".dvr-ms",
    dvr_ms,
    &[],
)
.with_kind(MimeKind::VIDEO)
.with_parent(&ASF);

static ASX: MimeType = MimeType::new(
    VIDEO_X_MS_ASX,
    "Advanced Stream Redirector",
    ".asx",
    asx,
    &[],
)
.with_kind(MimeKind::VIDEO)
.with_parent(&ASF);

static WMA: MimeType = MimeType::new(AUDIO_X_MS_WMA, "Windows Media Audio", ".wma", wma, &[])
    .with_kind(MimeKind::AUDIO)
    .with_parent(&ASF);

static WMV: MimeType = MimeType::new(VIDEO_X_MS_WMV, "Windows Media Video", ".wmv", wmv, &[])
    .with_kind(MimeKind::VIDEO)
    .with_parent(&ASF);

mimetype!(CDA, APPLICATION_X_CDF, ".cda", offset: (8, b"CDDA", prefix: (0, b"RIFF")), name: "CD Audio Track", kind: AUDIO);

mimetype!(M4V, VIDEO_X_M4V, ".m4v", offset: (8, b"M4V ", prefix: (4, b"ftyp")), name: "iTunes Video", kind: VIDEO);

// Flash MP4 Video - Adobe Flash MP4 video format
mimetype!(F4V, VIDEO_MP4, ".f4v", offset: (8, b"F4V ", prefix: (4, b"ftyp")), name: "Flash MP4 Video", kind: VIDEO);

// Flash MP4 Protected Video - Adobe Flash MP4 protected video format
mimetype!(F4P, VIDEO_MP4, ".f4p", offset: (8, b"F4P ", prefix: (4, b"ftyp")), name: "Flash MP4 Protected Video", kind: VIDEO);

// RealMedia Variable Bitrate - Child of RealMedia
// RMVB is a variant of RealMedia with variable bitrate encoding
// RealVideo - RealNetworks video format variant
static RV: MimeType = MimeType::new(
    VIDEO_X_PN_REALVIDEO,
    "RealVideo",
    ".rv",
    |_input| {
        // Parent REALMEDIA already verified .RMF signature
        // RV uses same signature, rely on extension for distinction
        false
    },
    &[],
)
.with_kind(MimeKind::VIDEO);

// NOTE: RMVB and RealMedia share identical .RMF signature and cannot be distinguished
// without deep chunk structure analysis. This child exists for future VBR-specific detection.
// For now, detection falls back to parent REALMEDIA.
static RMVB: MimeType = MimeType::new(
    APPLICATION_VND_RN_REALMEDIA_VBR,
    "RealMedia VBR",
    ".rmvb",
    |_input| {
        // Parent REALMEDIA already verified .RMF signature
        // TODO: Implement VBR-specific detection by parsing MDPR chunks for VBR flags
        // For now, return false to fall back to parent REALMEDIA
        false
    },
    &[],
)
.with_kind(MimeKind::VIDEO);

// RealMedia - Legacy streaming media format (parent to RMVB)
static REALMEDIA: MimeType = MimeType::new(
    APPLICATION_VND_RN_REALMEDIA,
    "RealMedia",
    ".rm",
    |input| input.starts_with(b".RMF"),
    &[&RV, &RMVB], // RV and RMVB are child variants
)
.with_kind(MimeKind::VIDEO);

// Silicon Graphics Movie - SGI movie/video format from IRIX systems
mimetype!(SGI_MOVIE, VIDEO_X_SGI_MOVIE, ".sgi", b"MOVI", name: "Silicon Graphics Movie", kind: VIDEO);

static THREE_GPP: MimeType = MimeType::new(VIDEO_3GPP, "3GPP Multimedia", ".3gp", three_gpp, &[])
    .with_aliases(&[VIDEO_3GP, AUDIO_3GPP])
    .with_kind(MimeKind::VIDEO);

static THREE_GPP2: MimeType =
    MimeType::new(VIDEO_3GPP2, "3GPP2 Multimedia", ".3g2", three_gpp2, &[])
        .with_aliases(&[VIDEO_3G2, AUDIO_3GPP2])
        .with_kind(MimeKind::VIDEO);

static MJ2: MimeType = MimeType::new(VIDEO_MJ2, "Motion JPEG 2000", ".mj2", mj2, &[]);

mimetype!(DVB, VIDEO_VND_DVB_FILE, ".dvb", offset: (4, b"ftypdvb1"), name: "Digital Video Broadcasting", kind: VIDEO);

// Autodesk FLIC Animation - Game development animation format
mimetype!(FLI, VIDEO_FLI, ".fli", [0x11, 0xAF], name: "Autodesk FLIC Animation", kind: VIDEO);
mimetype!(FLC, VIDEO_FLC, ".flc", [0x12, 0xAF], name: "Autodesk FLIC Animation", kind: VIDEO);

// Fast Search and Transfer Video - Surveillance video format
mimetype!(FVT, VIDEO_VND_FVT, ".fvt", b"FVT", name: "Fast Search & Transfer Video", kind: VIDEO);

// MTV - MTV video format (RIFF-based)
mimetype!(MTV, VIDEO_X_MTV, ".mtv", offset: (8, b"MTV", prefix: (0, b"RIFF")), name: "MTV Video", kind: VIDEO);

// AbiWord Template - Template variant of AbiWord (gzip-compressed)
static AWT: MimeType = MimeType::new(
    APPLICATION_X_ABIWORD_TEMPLATE,
    "AbiWord Template",
    ".awt",
    |_input| {
        // Parent ABW already verified gzip + abiword marker
        // AWT uses same structure, rely on extension for distinction
        false
    },
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ABW);

// Ogg Speex - Audio codec for voice in Ogg container
static SPX: MimeType = MimeType::new(
    AUDIO_OGG,
    "Ogg Audio",
    ".spx",
    |_input| {
        // Parent OGG already verified OggS signature
        // SPX uses Speex codec, rely on extension for distinction
        false
    },
    &[],
)
.with_kind(MimeKind::AUDIO)
.with_parent(&OGG);

// PEM Certificate Signing Request
mimetype!(CSR, APPLICATION_X_PEM_FILE, ".csr",
    b"-----BEGIN CERTIFICATE REQUEST-----" | b"-----BEGIN NEW CERTIFICATE REQUEST-----",
    name: "Certificate Signing Request",
    kind: APPLICATION,
    ext_aliases: [".pem"]);

// ActiveMime - Microsoft Office embedded OLE object
// ActiveMime signature: "ActiveMime" at offset 0x32
mimetype!(MSO, APPLICATION_X_MSO, ".mso", offset: (0x32, b"ActiveMime"), name: "Microsoft Office Embedded Object", kind: APPLICATION);

// Empty file - Zero-length file
// seek way to say file is empty
static EMPTY: MimeType = MimeType::new(
    APPLICATION_X_EMPTY,
    "Empty File",
    ".empty",
    |input| input.is_empty(),
    &[],
)
.with_kind(MimeKind::APPLICATION);

// MLA - Multi Layer Archive
mimetype!(MLA, APPLICATION_X_MLA, ".mla", b"MLA\x00", name: "Multi Layer Archive", kind: ARCHIVE);

// PMA - PMarc (LZH variant)
mimetype!(PMA, APPLICATION_X_LZH_COMPRESSED, ".pma", b"-pm0-" | b"-pm1-" | b"-pm2-", name: "PMarc Archive", kind: ARCHIVE);

// XCI - Nintendo Switch ROM (NX Card Image)
mimetype!(XCI, APPLICATION_X_NINTENDO_SWITCH_ROM, ".xci", b"HEAD", name: "Nintendo Switch ROM", kind: APPLICATION);

// MXF - Material Exchange Format for professional video/audio (SMPTE standard).
mimetype!(MXF, APPLICATION_MXF, ".mxf", [0x06, 0x0E, 0x2B, 0x34], name: "Material Exchange Format", kind: VIDEO);

// WTV - Windows Recorded TV Show format (successor to DVR-MS)
mimetype!(WTV, VIDEO_X_WTV, ".wtv", [0xB7, 0xD8, 0x00, 0x20, 0x37, 0x49, 0xDA, 0x11, 0xA6, 0x4E, 0x00, 0x07, 0xE9, 0x5E, 0xAD, 0x8D], name: "Windows Recorded TV Show", kind: VIDEO);

// MPEG-2 Transport Stream - Used for broadcasting and streaming.
static MPEG2TS: MimeType = MimeType::new(
    VIDEO_MP2T,
    "MPEG-2 TS",
    ".ts",
    |input| {
        input.len() >= 189 && input[0] == 0x47 && input[188] == 0x47 // Sync pattern repeats every 188 bytes
    },
    &[],
)
.with_extension_aliases(&[".m2ts", ".mts"])
.with_kind(MimeKind::VIDEO);

// Actions Media Video - Used in portable media players.
mimetype!(AMV, VIDEO_X_AMV, ".amv", b"AMV", name: "Actions Media Video", kind: VIDEO);

// XPI - Mozilla XPInstall (Firefox/Thunderbird extension)
static XPI: MimeType = MimeType::new(
    APPLICATION_X_XPINSTALL,
    "Mozilla XPInstall Extension",
    ".xpi",
    xpi,
    &[],
)
.with_kind(MimeKind::ARCHIVE)
.with_parent(&ZIP);

// XPS - OpenXPS (XML Paper Specification)
static XPS: MimeType = MimeType::new(
    APPLICATION_OXPS,
    "XML Paper Specification",
    ".xps",
    xps,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// Microsoft Works Word Processor - OLE-based, extension-based detection only
static WORKS_WPS: MimeType = MimeType::new(
    APPLICATION_VND_MS_WORKS,
    "Microsoft Works Word Processor",
    ".wps",
    |_input| {
        // Parent OLE already verified signature
        // No reliable CLSID for Works WPS, rely on extension for distinction
        false
    },
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&OLE);

// Microsoft Works 6 Spreadsheet
mimetype!(WORKS_XLR, APPLICATION_VND_MS_WORKS, ".xlr", b"\x00\x00\x02\x00\x06\x04\x06\x00", name: "Microsoft Works Spreadsheet", kind: SPREADSHEET);

// vCalendar 1.0 - Text-based calendar format (predecessor to iCalendar 2.0)
static VCALENDAR: MimeType =
    MimeType::new(TEXT_CALENDAR, "vCalendar", ".vcs", vcalendar, &[]).with_parent(&UTF8);

// Universal Subtitle Format - XML-based subtitle format
static USF: MimeType = MimeType::new(
    APPLICATION_X_USF,
    "Universal Subtitle Format",
    ".usf",
    usf,
    &[],
)
.with_parent(&XML);

// StarDraw - StarOffice/StarDivision Draw (graphics)
static SDA: MimeType = MimeType::new(
    APPLICATION_VND_STARDIVISION_DRAW,
    "StarDraw",
    ".sda",
    sda,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// StarCalc - StarOffice/StarDivision Calc (spreadsheet)
static SDC: MimeType = MimeType::new(
    APPLICATION_VND_STARDIVISION_CALC,
    "StarCalc",
    ".sdc",
    sdc,
    &[],
)
.with_kind(MimeKind::SPREADSHEET)
.with_parent(&ZIP);

// StarImpress - StarOffice/StarDivision Impress (presentation)
static SDD: MimeType = MimeType::new(
    APPLICATION_VND_STARDIVISION_IMPRESS,
    "StarImpress",
    ".sdd",
    sdd,
    &[],
)
.with_kind(MimeKind::PRESENTATION)
.with_parent(&ZIP);

// StarChart - StarOffice/StarDivision Chart
static SDS: MimeType = MimeType::new(
    APPLICATION_VND_STARDIVISION_CHART,
    "StarChart",
    ".sds",
    sds,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// StarWriter - StarOffice/StarDivision Writer (word processor)
static SDW: MimeType = MimeType::new(
    APPLICATION_VND_STARDIVISION_WRITER,
    "StarWriter",
    ".sdw",
    sdw,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// StarMath - StarOffice/StarDivision Math (mathematical formulas)
static SMF: MimeType = MimeType::new(
    APPLICATION_VND_STARDIVISION_MATH,
    "StarMath",
    ".smf",
    smf,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// Sun XML Draw - Legacy Sun Microsystems graphics format
static SXD: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_DRAW,
    "Sun XML Draw",
    ".sxd",
    sxd,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// Sun XML Impress - Legacy Sun Microsystems presentation format
static SXI: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_IMPRESS,
    "Sun XML Impress",
    ".sxi",
    sxi,
    &[],
)
.with_kind(MimeKind::PRESENTATION)
.with_parent(&ZIP);

// Sun XML Math - Legacy Sun Microsystems mathematical formula format
static SXM: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_MATH,
    "Sun XML Math",
    ".sxm",
    sxm,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// Sun XML Writer - Legacy Sun Microsystems word processor format
static SXW: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_WRITER,
    "Sun XML Writer",
    ".sxw",
    sxw,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// Sun XML Calc Template - Legacy Sun Microsystems spreadsheet template
static STC: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_CALC_TEMPLATE,
    "Sun XML Calc Template",
    ".stc",
    stc,
    &[],
)
.with_kind(MimeKind::SPREADSHEET)
.with_parent(&ZIP);

// Sun XML Draw Template - Legacy Sun Microsystems graphics template
static STD: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_DRAW_TEMPLATE,
    "Sun XML Draw Template",
    ".std",
    std,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// Sun XML Impress Template - Legacy Sun Microsystems presentation template
static STI: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_IMPRESS_TEMPLATE,
    "Sun XML Impress Template",
    ".sti",
    sti,
    &[],
)
.with_kind(MimeKind::PRESENTATION)
.with_parent(&ZIP);

// Sun XML Writer Template - Legacy Sun Microsystems word processor template
static STW: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_WRITER_TEMPLATE,
    "Sun XML Writer Template",
    ".stw",
    stw,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// Sun XML Writer Global - Legacy Sun Microsystems master document format
static SGW: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_WRITER_GLOBAL,
    "Sun XML Writer Global",
    ".sgw",
    sgw,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// WordPerfect Graphics - WordPerfect graphics format
static WPG: MimeType = MimeType::new(
    APPLICATION_VND_WORDPERFECT_GRAPHICS,
    "WordPerfect Graphics",
    ".wpg",
    wpg,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&WPD);

// WordPerfect Presentations - WordPerfect presentation format
static SHW: MimeType = MimeType::new(
    APPLICATION_VND_WORDPERFECT,
    "WordPerfect Presentations",
    ".shw",
    shw,
    &[],
)
.with_kind(MimeKind::PRESENTATION)
.with_parent(&WPD);

// WordPerfect Macro - WordPerfect macro format
static WPM: MimeType = MimeType::new(
    APPLICATION_VND_WORDPERFECT,
    "WordPerfect Macro",
    ".wpm",
    wpm,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&WPD);

// Uniform Office Format Presentation - Chinese office format (ZIP-based)
static UOP: MimeType = MimeType::new(
    APPLICATION_VND_UOF_PRESENTATION,
    "UOF Presentation",
    ".uop",
    uop,
    &[],
)
.with_kind(MimeKind::PRESENTATION)
.with_parent(&ZIP);

// Uniform Office Format Spreadsheet - Chinese office format (ZIP-based)
static UOS: MimeType = MimeType::new(
    APPLICATION_VND_UOF_SPREADSHEET,
    "UOF Spreadsheet",
    ".uos",
    uos,
    &[],
)
.with_kind(MimeKind::SPREADSHEET)
.with_parent(&ZIP);

// Uniform Office Format Text - Chinese office format (ZIP-based)
static UOT: MimeType = MimeType::new(APPLICATION_VND_UOF_TEXT, "UOF Text", ".uot", uot, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&ZIP);

// Initial Graphics Exchange Specification (IGES) - CAD data exchange format
// IGES files start with spaces followed by 'S' in column 73
mimetype!(IGES, MODEL_IGES, ".iges", b"                                                                        S", name: "Initial Graphics Exchange Specification", kind: MODEL, ext_aliases: [".igs"]);

// Universal Scene Description ZIP (USDZ) - Pixar's USD format in ZIP container
// USDZ files are ZIP archives containing USD files, commonly used for AR/VR
static USDZ: MimeType = MimeType::new(
    MODEL_VND_USDZ_ZIP,
    "Universal Scene Description ZIP",
    ".usdz",
    usdz,
    &[],
)
.with_kind(MimeKind::MODEL)
.with_parent(&ZIP);

// Sketch - Design tool by Bohemian Coding (ZIP-based)
// Sketch 43+ uses JSON metadata inside ZIP archive
static SKETCH: MimeType = MimeType::new(IMAGE_X_SKETCH, "Sketch Design", ".sketch", sketch, &[])
    .with_kind(MimeKind::IMAGE)
    .with_parent(&ZIP);

// SolidWorks Assembly - OLE-based CAD file
static SLDASM: MimeType = MimeType::new(
    MODEL_X_SLDASM,
    "SolidWorks Assembly",
    ".sldasm",
    sldasm,
    &[],
)
.with_kind(MimeKind::MODEL)
.with_parent(&OLE);

// SolidWorks Drawing - OLE-based CAD file
static SLDDRW: MimeType =
    MimeType::new(MODEL_X_SLDDRW, "SolidWorks Drawing", ".slddrw", slddrw, &[])
        .with_kind(MimeKind::MODEL)
        .with_parent(&OLE);

// SolidWorks Part - OLE-based CAD file
static SLDPRT: MimeType = MimeType::new(MODEL_X_SLDPRT, "SolidWorks Part", ".sldprt", sldprt, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&OLE);

// Autodesk Inventor Assembly - OLE-based CAD file
static IAM: MimeType = MimeType::new(MODEL_X_IAM, "Inventor Assembly", ".iam", iam, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&OLE);

// Autodesk Inventor Drawing - OLE-based CAD file
static IDW: MimeType = MimeType::new(MODEL_X_IDW, "Inventor Drawing", ".idw", idw, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&OLE);

// Autodesk Inventor Presentation - OLE-based CAD file
static IPN: MimeType = MimeType::new(MODEL_X_IPN, "Inventor Presentation", ".ipn", ipn, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&OLE);

// Autodesk Inventor Part - OLE-based CAD file
static IPT: MimeType = MimeType::new(MODEL_X_IPT, "Inventor Part", ".ipt", ipt, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&OLE);

// Inter-Quake Export - Text-based 3D model format
mimetype!(IQE, MODEL_X_IQE, ".iqe", b"# Inter-Quake Export", name: "Inter-Quake Export", kind: MODEL);

// Model 3D Binary - Binary 3D model format
mimetype!(M3D, MODEL_X_3D_MODEL, ".m3d", b"3DMO", name: "Model 3D Binary", kind: MODEL);

// SpaceClaim Document - OLE-based CAD format
static SCDOC: MimeType = MimeType::new(MODEL_X_SCDOC, "SpaceClaim Document", ".scdoc", scdoc, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&OLE);

// Model 3D ASCII - Text-based 3D model format
mimetype!(A3D, TEXT_X_3D_MODEL, ".a3d", b"3DGeometry", name: "Model 3D ASCII", kind: MODEL);

// Autodesk 123D - ZIP-based 3D modeling format
static AUTODESK_123D: MimeType =
    MimeType::new(MODEL_X_123DX, "Autodesk 123D", ".123dx", autodesk_123d, &[])
        .with_kind(MimeKind::MODEL)
        .with_parent(&ZIP);

// Fusion 360 - ZIP-based CAD format
static FUSION_360: MimeType =
    MimeType::new(MODEL_X_F3D, "Autodesk Fusion 360", ".f3d", fusion_360, &[])
        .with_kind(MimeKind::MODEL)
        .with_parent(&ZIP);

// draw.io - XML-based diagramming format
static DRAWIO: MimeType = MimeType::new(
    APPLICATION_VND_JGRAPH_MXFILE,
    "draw.io",
    ".drawio",
    drawio,
    &[],
)
.with_kind(MimeKind::DOCUMENT);

// XML Shareable Playlist Format - XML-based playlist
static XSPF: MimeType = MimeType::new(APPLICATION_XSPF_XML, "Xspf XML", ".xspf", xspf, &[])
    .with_kind(MimeKind::DOCUMENT);

// XSLT - Extensible Stylesheet Language Transformations
static XSL: MimeType =
    MimeType::new(APPLICATION_XSLT_XML, "Xslt XML", ".xsl", xsl, &[]).with_kind(MimeKind::DOCUMENT);

// Figma - ZIP-based design format
static FIGMA: MimeType = MimeType::new(IMAGE_X_FIGMA, "Figma", ".fig", figma, &[])
    .with_kind(MimeKind::IMAGE)
    .with_parent(&ZIP);

// MathML - Mathematical Markup Language
static MATHML: MimeType =
    MimeType::new(APPLICATION_MATHML_XML, "Mathml XML", ".mathml", mathml, &[])
        .with_kind(MimeKind::DOCUMENT);

// MusicXML - Music notation format
static MUSICXML: MimeType = MimeType::new(
    APPLICATION_VND_RECORDARE_MUSICXML_XML,
    "MusicXML",
    ".musicxml",
    musicxml,
    &[],
)
.with_kind(MimeKind::DOCUMENT);

// TTML - Timed Text Markup Language (subtitles)
static TTML: MimeType = MimeType::new(APPLICATION_TTML_XML, "Ttml XML", ".ttml", ttml, &[])
    .with_kind(MimeKind::DOCUMENT);

// SOAP - Simple Object Access Protocol
static SOAP: MimeType = MimeType::new(APPLICATION_SOAP_XML, "Soap XML", ".soap", soap, &[])
    .with_kind(MimeKind::DOCUMENT);

// TMX - Tiled Map XML (game development)
static TMX: MimeType =
    MimeType::new(APPLICATION_X_TMX_XML, "Tmx XML", ".tmx", tmx, &[]).with_kind(MimeKind::DOCUMENT);

// TSX - Tiled Tileset XML (game development)
static TSX: MimeType =
    MimeType::new(APPLICATION_X_TSX_XML, "Tsx XML", ".tsx", tsx, &[]).with_kind(MimeKind::DOCUMENT);

// MPD - MPEG-DASH Media Presentation Description
static MPD: MimeType =
    MimeType::new(APPLICATION_DASH_XML, "Dash XML", ".mpd", mpd, &[]).with_kind(MimeKind::DOCUMENT);

// MXL - MusicXML ZIP (compressed music notation)
static MXL: MimeType = MimeType::new(
    APPLICATION_VND_RECORDARE_MUSICXML,
    "MusicXML",
    ".mxl",
    mxl,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

// CDDX - Circuit Diagram Document (XML)
static CDDX: MimeType = MimeType::new(
    APPLICATION_VND_CIRCUITDIAGRAM_DOCUMENT_MAIN_XML,
    "Circuit Diagram Document",
    ".cddx",
    cddx,
    &[],
)
.with_kind(MimeKind::DOCUMENT);

// DWFX - Design Web Format XPS (XML)
static DWFX: MimeType =
    MimeType::new(MODEL_VND_DWFX_XPS, "Dwfx Xps", ".dwfx", dwfx, &[]).with_kind(MimeKind::DOCUMENT);

// FBZ - FictionBook ZIP (compressed e-book)
static FBZ: MimeType = MimeType::new(APPLICATION_X_FBZ, "FictionBook ZIP", ".fbz", fbz, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&ZIP);

// ============================================================================
// EXECUTABLE & BINARY FORMATS
// ============================================================================

// MS-DOS Executable - Classic DOS .exe format (defined first for forward reference)
// Child of EXE, matches when it's NOT a PE file
static MSDOS_EXE: MimeType = MimeType::new(
    APPLICATION_X_DOSEXEC,
    "MS-DOS Executable",
    ".exe",
    |input| {
        // This will be checked as a child of EXE
        // The parent EXE already verified MZ signature
        // We just need to verify it's NOT a PE file

        // If the file is too small to contain a valid DOS header, it's not MS-DOS
        if input.len() < 0x40 {
            return false;
        }

        // Get PE header offset from DOS header (at offset 0x3C)
        let pe_offset = u32::from_le_bytes(input[0x3C..0x40].try_into().unwrap()) as usize;

        // If we can read the PE signature location
        if pe_offset + 4 <= input.len() && pe_offset < 0x10000 {
            // It's MS-DOS if it does NOT have PE signature
            return &input[pe_offset..pe_offset + 4] != b"PE\0\0";
        }

        // If PE offset points beyond file, it's a DOS exe
        true
    },
    &[],
)
.with_extension_aliases(&[".com"])
.with_kind(MimeKind::EXECUTABLE);

// Windows/DOS Executable - Starts with "MZ"
// Parent matches ANY MZ file, child differentiates MS-DOS
static EXE: MimeType = MimeType::new(
    APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE,
    "Windows Executable",
    ".exe",
    |input| {
        // Match any file starting with MZ
        // The tree will check MSDOS_EXE child first
        // If child matches, it returns APPLICATION_X_DOSEXEC
        // If child doesn't match, this parent is returned as PE
        input.starts_with(b"MZ")
    },
    &[&MSDOS_EXE], // MS-DOS executable is a child
)
.with_extension_aliases(&[".dll", ".sys", ".scr"])
.with_kind(MimeKind::EXECUTABLE);

static ELF: MimeType = MimeType::new(
    APPLICATION_X_ELF,
    "ELF",
    "",
    |input| input.starts_with(b"\x7fELF"),
    &[&APPIMAGE, &ELF_OBJ, &ELF_EXE, &ELF_LIB, &ELF_DUMP],
)
.with_extension_aliases(&[".so"])
.with_kind(MimeKind::EXECUTABLE);

static ELF_OBJ: MimeType = MimeType::new(APPLICATION_X_OBJECT, "ELF Object", "", elf_obj, &[])
    .with_kind(MimeKind::EXECUTABLE)
    .with_parent(&ELF);

static ELF_EXE: MimeType = MimeType::new(
    APPLICATION_X_EXECUTABLE,
    "ELF Executable",
    ".elf",
    elf_exe,
    &[],
)
.with_kind(MimeKind::EXECUTABLE)
.with_parent(&ELF);

static ELF_LIB: MimeType = MimeType::new(
    APPLICATION_X_SHAREDLIB,
    "Shared Library",
    ".so",
    elf_lib,
    &[],
)
.with_kind(MimeKind::EXECUTABLE)
.with_parent(&ELF);

static ELF_DUMP: MimeType = MimeType::new(APPLICATION_X_COREDUMP, "Core Dump", "", elf_dump, &[])
    .with_kind(MimeKind::EXECUTABLE)
    .with_parent(&ELF);

mimetype!(CLASS, APPLICATION_X_JAVA_APPLET_BINARY, ".class", b"\xca\xfe\xba\xbe", name: "Java Class File", kind: APPLICATION, aliases: [APPLICATION_X_JAVA_APPLET]);

// Apache Arrow - Columnar data format for analytics.
mimetype!(ARROW, APPLICATION_VND_APACHE_ARROW_FILE, ".arrow", b"ARROW1", name: "Apache Arrow", kind: DATABASE);

// Apache Avro - Data serialization format.
mimetype!(AVRO, APPLICATION_VND_APACHE_AVRO, ".avro", b"Obj\x01", name: "Apache Avro", kind: DATABASE);

// ID3v2 Audio Metadata - Metadata tags for audio files.
static ID3V2: MimeType = MimeType::new(
    APPLICATION_X_ID3V2,
    "ID3v2 Tag",
    ".id3",
    |input| {
        input.starts_with(b"ID3\x02")
            || input.starts_with(b"ID3\x03")
            || input.starts_with(b"ID3\x04")
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// Amiga Hunk Executable - Legacy Amiga executable format
mimetype!(AMIGA_HUNK, APPLICATION_X_AMIGA_EXECUTABLE, ".amiga", b"\x00\x00\x03\xF3", name: "Amiga Hunk Executable", kind: EXECUTABLE);

// Xbox Executable - Original Xbox executable format
mimetype!(XBE, APPLICATION_X_XBOX_EXECUTABLE, ".xbe", b"XBEH", name: "Xbox Executable", kind: EXECUTABLE);

// Xbox 360 Executable - Xbox 360 executable formats (XEX1 and XEX2)
mimetype!(XEX, APPLICATION_X_XBOX360_EXECUTABLE, ".xex", b"XEX2" | b"XEX1", name: "Xbox 360 Executable", kind: EXECUTABLE);

// AppImage - Linux application packaging format (Type 2).
// AppImage files are ELF executables with magic at offset 8.
// Type 2 AppImages have 0x41 0x49 0x02 ("AI" + version) at offset 8
// First 4 bytes are ELF magic (7F 45 4C 46)
mimetype!(APPIMAGE, APPLICATION_X_APPIMAGE, ".appimage", offset: (8, b"\x41\x49\x02", prefix: (0, b"\x7FELF")), name: "AppImage", kind: EXECUTABLE, parent: &ELF);

// LLVM Bitcode - LLVM compiler intermediate representation.
// Raw bitcode: starts with 'BC' (0x42 0x43)
// Wrapped bitcode: starts with 0xDE 0xC0 0x17 0x0B (little-endian 0x0B17C0DE)
mimetype!(LLVM_BITCODE, APPLICATION_X_LLVM, ".bc", b"BC" | b"\xDE\xC0\x17\x0B", name: "LLVM Bitcode", kind: APPLICATION);

// ICC Color Profile - Color management profiles.
mimetype!(ICC, APPLICATION_VND_ICCPROFILE, ".icc", offset: (36, b"acsp"), name: "ICC Color Profile", kind: APPLICATION, ext_aliases: [".icm"]);

// PEM Certificate/Key formats - Cryptographic certificates and keys.
mimetype!(PEM, APPLICATION_X_PEM_FILE, ".pem",
    b"-----BEGIN CERTIFICATE-----" |
    b"-----BEGIN PRIVATE KEY-----" |
    b"-----BEGIN RSA PRIVATE KEY-----" |
    b"-----BEGIN DSA PRIVATE KEY-----" |
    b"-----BEGIN EC PRIVATE KEY-----" |
    b"-----BEGIN ECDSA PRIVATE KEY-----" |
    b"-----BEGIN ENCRYPTED PRIVATE KEY-----" |
    b"-----BEGIN PUBLIC KEY-----",
    name: "PEM Certificate",
    kind: TEXT, ext_aliases: [".crt", ".key", ".cert"]);

// Age Encryption - Modern, simple file encryption format
mimetype!(AGE, APPLICATION_X_AGE_ENCRYPTION, ".age", b"age-encryption.org/v1\n", name: "Age Encryption", kind: DOCUMENT);

// EBML - Extensible Binary Meta Language, base for Matroska/WebM.
static EBML: MimeType = MimeType::new(
    APPLICATION_X_EBML,
    "EBML",
    ".ebml",
    |input| {
        // EBML magic number: 0x1A45DFA3
        // WebM and MKV are specific EBML formats but defined later
        // They remain in ROOT children for detection priority
        input.starts_with(b"\x1A\x45\xDF\xA3")
            && !is_matroska_file_type(input, b"webm")
            && !is_matroska_file_type(input, b"matroska")
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

mimetype!(WASM, APPLICATION_WASM, ".wasm", b"\x00asm", name: "WebAssembly Binary", kind: EXECUTABLE);

// WebAssembly Text Format (WAT) - human-readable text format for WebAssembly
mimetype!(WAT, TEXT_WASM, ".wat", b"(module", name: "WebAssembly Text", kind: TEXT);

// ============================================================================
// ANDROID FORMATS
// ============================================================================

// Dalvik Executable - Android app bytecode
mimetype!(DEX, APPLICATION_VND_ANDROID_DEX, ".dex", b"dex\n", name: "Dalvik Executable", kind: EXECUTABLE);

// Optimized Dalvik Executable
mimetype!(DEY, APPLICATION_VND_ANDROID_DEY, ".dey", b"dey\n", name: "Optimized Dalvik Executable", kind: EXECUTABLE);

// ============================================================================
// ADDITIONAL COMPRESSION FORMATS
// ============================================================================

// BZIP3 - Next generation BZIP compression
mimetype!(BZIP3, APPLICATION_X_BZIP3, ".bz3", b"BZ3v1", name: "BZIP3 Compressed Archive", kind: ARCHIVE);

// LZMA - Lempel-Ziv-Markov chain Algorithm
mimetype!(LZMA, APPLICATION_X_LZMA, ".lzma", b"\x5D\x00\x00\x80\x00", name: "LZMA Compressed Archive", kind: ARCHIVE);

// LZOP - LZO compression with header
mimetype!(LZOP, APPLICATION_X_LZOP, ".lzo", b"\x89LZO\0\r\n\x1A\n", name: "LZOP Compressed Archive", kind: ARCHIVE);

// LZFSE - Apple's Lempel-Ziv Finite State Entropy compression
// "bvx-" - uncompressed block, "bvx1" - compressed v1, "bvx2" - compressed v2, "bvx$" - end of stream
mimetype!(LZFSE, APPLICATION_X_LZFSE, ".lzfse", b"bvx-" | b"bvx1" | b"bvx2" | b"bvx$", name: "LZFSE Compressed Archive", kind: ARCHIVE);

// ============================================================================
// GAME ROM FORMATS
// ============================================================================

// Nintendo Entertainment System ROM
// Already defined as NES above, but using wrong constant - let's skip duplicate

// GameBoy Advance ROM - Has signature at offset 4
mimetype!(GBA_ROM, APPLICATION_X_GBA_ROM, ".gba", offset: (4, b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"), name: "Game Boy Advance ROM", kind: APPLICATION);

// GameBoy Color ROM - More specific version of GB_ROM with color flag
// Defined first due to forward reference in parent
static GBC_ROM: MimeType = MimeType::new(
    APPLICATION_X_GAMEBOY_COLOR_ROM,
    "Game Boy Color ROM",
    ".gbc",
    |input| {
        input.len() >= 324
            && &input[260..268] == b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
            && (input[323] == 0x80 || input[323] == 0xC0)
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// GameBoy ROM - Has signature at offset 260
// Parent to GameBoy Color ROM which adds a color flag check
static GB_ROM: MimeType = MimeType::new(
    APPLICATION_X_GAMEBOY_ROM,
    "Game Boy ROM",
    ".gb",
    |input| input.len() >= 268 && &input[260..268] == b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B",
    &[&GBC_ROM], // GBC_ROM is a child - more specific version with color flag
)
.with_kind(MimeKind::APPLICATION);

// ============================================================================
// NINTENDO 64 ROM FORMATS
// ============================================================================

// Nintendo 64 ROM - Supports all 3 byte order variants
// NOTE: Appears in THREE PREFIX_VEC buckets (0x37, 0x40, 0x80) - this is intentional!
// N64 ROMs were distributed in different byte orders for compatibility with different systems:
//   Z64 (big-endian):    0x80 0x37 0x12 0x40  [PREFIX_VEC 0x80] - Native N64 format
//   N64 (little-endian): 0x40 0x12 0x37 0x80  [PREFIX_VEC 0x40] - Doctor V64 format
//   V64 (byte-swapped):  0x37 0x80 0x40 0x12  [PREFIX_VEC 0x37] - Mr. Backup Z64 format
// All represent the same ROM data, just in different byte orders.
mimetype!(N64_ROM, APPLICATION_X_N64_ROM, ".n64", [0x80, 0x37, 0x12, 0x40] | [0x40, 0x12, 0x37, 0x80] | [0x37, 0x80, 0x40, 0x12], name: "Nintendo 64 ROM", ext_aliases: [".z64", ".v64"], kind: APPLICATION);

// ============================================================================
// NINTENDO DS ROM
// ============================================================================

// Nintendo DS ROM
// Magic: 0x2E 0x00 0x00 0xEA at offset 0
mimetype!(NINTENDO_DS_ROM, APPLICATION_X_NINTENDO_DS_ROM, ".nds", [0x2E, 0x00, 0x00, 0xEA], name: "Nintendo DS ROM", kind: APPLICATION);

// ============================================================================
// NINTENDO SWITCH FORMATS
// ============================================================================

// Nintendo Switch Package (NSP) - Uses PFS0 container format
// Magic: "PFS0" at offset 0
mimetype!(NINTENDO_SWITCH_NSP, APPLICATION_X_NINTENDO_SWITCH_PACKAGE, ".nsp", b"PFS0", name: "Nintendo Switch Package", kind: APPLICATION);

// Nintendo Switch Relocatable Object (NRO)
// Magic: "NRO0" at offset 0x10
static NINTENDO_SWITCH_NRO: MimeType = MimeType::new(
    APPLICATION_X_NINTENDO_SWITCH_EXECUTABLE,
    "Nintendo Switch Relocatable Object",
    ".nro",
    |input| input.len() >= 0x14 && &input[0x10..0x14] == b"NRO0",
    &[],
)
.with_kind(MimeKind::APPLICATION);

// Nintendo Switch Shared Object (NSO)
// Magic: "NSO0" at offset 0
mimetype!(NINTENDO_SWITCH_NSO, APPLICATION_X_NINTENDO_SWITCH_SO, ".nso", b"NSO0", name: "Nintendo Switch Shared Object", kind: APPLICATION);

// ============================================================================
// NEO GEO POCKET ROM FORMATS
// ============================================================================

// Neo Geo Pocket Color ROM - Child variant with color support
// Parent NEO_GEO_POCKET_ROM checks for COPYRIGHT/LICENSED header
// Child checks for color system identifier 0x10 at offset 0x23
static NEO_GEO_POCKET_COLOR_ROM: MimeType = MimeType::new(
    APPLICATION_X_NEO_GEO_POCKET_COLOR_ROM,
    "Neo Geo Pocket Color ROM",
    ".ngc",
    |input| {
        // Parent already verified COPYRIGHT/LICENSED header and length
        // Just check for color system identifier at offset 0x23
        input.len() >= 0x24 && input[0x23] == 0x10
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// Neo Geo Pocket ROM - Parent format (monochrome and color variants)
// Checks for common " COPYRIGHT" or " LICENSED" header
// Color variant (child) refines detection by checking byte at offset 0x23
static NEO_GEO_POCKET_ROM: MimeType = MimeType::new(
    APPLICATION_X_NEO_GEO_POCKET_ROM,
    "Neo Geo Pocket ROM",
    ".ngp",
    |input| {
        // Check for copyright/licensed header (common to both variants)
        // If Color child doesn't match (0x10 at offset 0x23), this parent represents monochrome (0x00)
        input.len() > 0x24 && input.starts_with(b" COPYRIGHT") || input.starts_with(b" LICENSED")
    },
    &[&NEO_GEO_POCKET_COLOR_ROM], // Color variant as child
)
.with_kind(MimeKind::APPLICATION);

// ============================================================================
// CERTIFICATE AND KEY FORMATS
// ============================================================================

// DER Certificate - X.509 certificate in binary format
mimetype!(DER_CERT, APPLICATION_X_X509_CA_CERT, ".der", b"\x30\x82", name: "DER Certificate", kind: APPLICATION, ext_aliases: [".cer", ".crt"]);

// Java Keystore
mimetype!(JAVA_KEYSTORE, APPLICATION_X_JAVA_KEYSTORE, ".jks", b"\xFE\xED\xFE\xED", name: "Java Keystore", kind: APPLICATION);

// ============================================================================
// BYTECODE FORMATS
// ============================================================================

// Lua Bytecode
mimetype!(LUA_BYTECODE, APPLICATION_X_LUA_BYTECODE, ".luac", b"\x1BLua", name: "Lua Bytecode", kind: APPLICATION);

// Python Pickle (protocols 2-5)
// Protocols 2-5 start with 0x80 followed by protocol version (0x02-0x05)
static PYTHON_PICKLE: MimeType = MimeType::new(
    APPLICATION_X_PICKLE,
    "Python Pickle",
    ".pkl",
    |input| {
        // Check for PROTO opcode (0x80) followed by protocol version (2-5)
        input.len() >= 2 && input[0] == 0x80 && matches!(input[1], 0x02..=0x05)
    },
    &[],
)
.with_extension_aliases(&[".pickle"])
.with_kind(MimeKind::APPLICATION);

// Python Bytecode (.pyc files)
// All Python bytecode files have 0x0D 0x0A at bytes 2-3 after the magic number
static PYTHON_BYTECODE: MimeType = MimeType::new(
    APPLICATION_X_PYTHON_BYTECODE,
    "Python Bytecode",
    ".pyc",
    |input| {
        // Check for CRLF at offset 2 (bytes 2-3)
        input.len() >= 4 && input[2] == 0x0D && input[3] == 0x0A
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// ============================================================================
// PGP/GPG FORMATS
// ============================================================================

// PGP Message - Encrypted or signed message
static PGP_MESSAGE: MimeType = MimeType::new(
    APPLICATION_PGP,
    "PGP Message",
    ".pgp",
    |input| input.starts_with(b"-----BEGIN PGP MESSAGE-----"),
    &[],
)
.with_extension_aliases(&[".gpg", ".asc"])
.with_kind(MimeKind::APPLICATION);

// PGP Signed Message - Clear-signed message
static PGP_SIGNED_MESSAGE: MimeType = MimeType::new(
    APPLICATION_PGP_SIGNED,
    "PGP Signed Message",
    ".asc",
    |input| input.starts_with(b"-----BEGIN PGP SIGNED MESSAGE-----"),
    &[],
)
.with_extension_aliases(&[".sig"])
.with_kind(MimeKind::APPLICATION);

// PGP Public Key Block
static PGP_PUBLIC_KEY: MimeType = MimeType::new(
    APPLICATION_PGP_KEYS,
    "PGP Public Key",
    ".asc",
    |input| input.starts_with(b"-----BEGIN PGP PUBLIC KEY BLOCK-----"),
    &[],
)
.with_extension_aliases(&[".pgp", ".gpg", ".key"])
.with_kind(MimeKind::APPLICATION);

// PGP Private Key Block
static PGP_PRIVATE_KEY: MimeType = MimeType::new(
    APPLICATION_PGP_KEYS,
    "PGP Public Key",
    ".asc",
    |input| input.starts_with(b"-----BEGIN PGP PRIVATE KEY BLOCK-----"),
    &[],
)
.with_extension_aliases(&[".pgp", ".gpg", ".key"])
.with_kind(MimeKind::APPLICATION);

// PGP Signature - Detached signature
static PGP_SIGNATURE: MimeType = MimeType::new(
    APPLICATION_PGP_SIGNATURE,
    "PGP Signature",
    ".sig",
    |input| input.starts_with(b"-----BEGIN PGP SIGNATURE-----"),
    &[],
)
.with_extension_aliases(&[".asc"])
.with_kind(MimeKind::APPLICATION);

// ============================================================================
// ANDROID BINARY FORMATS
// ============================================================================

// Android Binary XML (AXML) - Used for AndroidManifest.xml
// AXML files have a specific binary header
static AXML: MimeType = MimeType::new(
    APPLICATION_VND_ANDROID_AXML,
    "Android Binary XML",
    ".xml",
    |input| {
        // Android Binary XML magic: 0x00080003 (little-endian)
        input.len() >= 4
            && input[0] == 0x03
            && input[1] == 0x00
            && input[2] == 0x08
            && input[3] == 0x00
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// Android Resource Storage Container (ARSC) - Binary resource table
static ARSC: MimeType = MimeType::new(
    APPLICATION_VND_ANDROID_ARSC,
    "Android Resources",
    ".arsc",
    |input| {
        // ARSC magic: 0x00080002 (little-endian)
        input.len() >= 4
            && input[0] == 0x02
            && input[1] == 0x00
            && input[2] == 0x08
            && input[3] == 0x00
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// ============================================================================
// CAMERA RAW FORMATS
// ============================================================================

// Canon Raw (original format) - CIFF-based
mimetype!(CRW, IMAGE_X_CANON_CRW, ".crw", b"II\x1a\x00\x00\x00HEAPCCDR", name: "Canon Raw Image", kind: IMAGE);

// Canon Raw 3 - ISO Base Media File Format
mimetype!(CR3, IMAGE_X_CANON_CR3, ".cr3", offset: (4, b"ftypcrx "), name: "Canon Raw 3", kind: IMAGE);

// Fujifilm RAF - RAW format with distinct signature
mimetype!(RAF, IMAGE_X_FUJI_RAF, ".raf", b"FUJIFILMCCD-RAW ", name: "Fujifilm RAF Image", kind: IMAGE);

// Olympus ORF - TIFF-based with custom magic
mimetype!(ORF, IMAGE_X_OLYMPUS_ORF, ".orf", b"IIRO" | b"IIRS" | b"MMOR", name: "Olympus Raw Image", kind: IMAGE);

// Panasonic RW2 - TIFF-based with IIU signature
static RW2: MimeType = MimeType::new(
    IMAGE_X_PANASONIC_RW2,
    "Panasonic RW2 Image",
    ".rw2",
    |input| {
        // Panasonic RW2: 49 49 55 00 with specific Panasonic markers
        // Check for full 4-byte TIFF header and distinguish from Kodak DCR
        if input.len() < 4 {
            return false;
        }
        // RW2 uses 0x49 0x49 0x55 0x00 but has different internal structure than Kodak DCR
        // For now, we'll check for additional Panasonic-specific markers if available
        input.starts_with(&[0x49, 0x49, 0x55, 0x00]) && input.len() > 100
    },
    &[],
)
.with_kind(MimeKind::IMAGE);

// ============================================================================
// AUDIO MODULE FORMATS
// ============================================================================

// FastTracker 2 Extended Module
mimetype!(XM, AUDIO_X_XM, ".xm", b"Extended Module: ", name: "FastTracker 2 Extended Module", kind: AUDIO);

// Impulse Tracker Module
mimetype!(IT, AUDIO_X_IT, ".it", b"IMPM", name: "Impulse Tracker Module", kind: AUDIO);

// Scream Tracker 3 Module - signature at offset 44
mimetype!(S3M, AUDIO_X_S3M, ".s3m", offset: (44, b"SCRM"), name: "Scream Tracker 3 Module", kind: AUDIO, aliases: [AUDIO_S3M]);

// ProTracker Module - complex detection at offset 1080
static MOD: MimeType = MimeType::new(
    AUDIO_X_MOD,
    "ProTracker MOD",
    ".mod",
    |input| {
        if input.len() < 1084 {
            return false;
        }
        // Check for various MOD signatures at offset 1080
        let sig = &input[1080..1084];
        matches!(
            sig,
            b"M.K."
                | b"M!K!"
                | b"4CHN"
                | b"6CHN"
                | b"8CHN"
                | b"FLT4"
                | b"FLT8"
                | b"CD81"
                | b"OCTA"
                | b"FA04"
                | b"FA06"
                | b"FA08"
        )
    },
    &[],
)
.with_kind(MimeKind::AUDIO);

// Shoutcast Playlist - text-based playlist format
mimetype!(PLS, AUDIO_X_SCPLS, ".pls", b"[playlist]", name: "Shoutcast Playlist", kind: AUDIO);

// Windows Media Playlist - XML-based playlist format for Windows Media Player
mimetype!(WPL, APPLICATION_VND_MS_WPL, ".wpl", b"<?wpl ", name: "Windows Media Playlist", kind: AUDIO);

// ============================================================================
// APPLE FORMATS
// ============================================================================

// Apple Disk Image
mimetype!(DMG, APPLICATION_X_APPLE_DISKIMAGE, ".dmg", b"koly", name: "Apple Disk Image", kind: ARCHIVE);

// macOS Alias File - Finder alias files
mimetype!(MACOS_ALIAS, APPLICATION_X_APPLE_ALIAS, "", b"book\x00\x00\x00\x00mark\x00\x00\x00\x00", name: "macOS Alias File", kind: APPLICATION);

// ============================================================================
// SEGA GAME ROM FORMATS
// ============================================================================

// Sega Game Gear ROM - "TMR SEGA" at specific offsets
// ⚠️ NOTE: Requires reading beyond default READ_LIMIT (3072 bytes)
// Signature appears at offsets 0x1ff0 (8KB), 0x3ff0 (16KB), or 0x7ff0 (32KB)
// Use detect_with_limit(data, 32768) for proper detection.
static GAME_GEAR_ROM: MimeType = MimeType::new(
    APPLICATION_X_GAMEGEAR_ROM,
    "Game Gear ROM",
    ".gg",
    |input| {
        // Check for "TMR SEGA" at offsets 0x1ff0, 0x3ff0, or 0x7ff0
        const TMR_SEGA: &[u8] = b"TMR SEGA";
        const OFFSETS: [usize; 3] = [0x1ff0, 0x3ff0, 0x7ff0];

        for &offset in &OFFSETS {
            if input.len() >= offset + TMR_SEGA.len()
                && &input[offset..offset + TMR_SEGA.len()] == TMR_SEGA
            {
                return true;
            }
        }
        false
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// Sega Master System ROM - "TMR SEGA" at specific offsets (same as Game Gear)
// ⚠️ NOTE: Requires reading beyond default READ_LIMIT (3072 bytes)
// Use detect_with_limit(data, 32768) for proper detection.
static SMS_ROM: MimeType = MimeType::new(
    APPLICATION_X_SMS_ROM,
    "Sega Master System ROM",
    ".sms",
    |input| {
        // Check for "TMR SEGA" at offsets 0x1ff0, 0x3ff0, or 0x7ff0
        const TMR_SEGA: &[u8] = b"TMR SEGA";
        const OFFSETS: [usize; 3] = [0x1ff0, 0x3ff0, 0x7ff0];

        for &offset in &OFFSETS {
            if input.len() >= offset + TMR_SEGA.len()
                && &input[offset..offset + TMR_SEGA.len()] == TMR_SEGA
            {
                return true;
            }
        }
        false
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// Sega Genesis/Mega Drive ROM - "SEGA" at offset 0x100
static GENESIS_ROM: MimeType = MimeType::new(
    APPLICATION_X_GENESIS_ROM,
    "Genesis ROM",
    ".gen",
    |input| {
        // Check for "SEGA MEGA DRIVE" or "SEGA GENESIS" at offset 0x100
        if input.len() < 272 {
            return false;
        }

        let header = &input[0x100..];
        header.starts_with(b"SEGA MEGA DRIVE") || header.starts_with(b"SEGA GENESIS")
    },
    &[],
)
.with_extension_aliases(&[".md", ".smd", ".bin"])
.with_kind(MimeKind::APPLICATION);

// ============================================================================
// SIMPLE ARCHIVE FORMATS
// ============================================================================

/// Zoo archive format
static ZOO: MimeType = MimeType::new(
    APPLICATION_X_ZOO,
    "Zoo Archive",
    ".zoo",
    |input| {
        // Zoo archives have magic bytes 0xFDC4A7DC at offset 20 (after the text header)
        // Or they can start with "ZOO " followed by version like "ZOO 2.10 Archive."
        if input.len() < 24 {
            return false;
        }
        // Check for the Zoo magic bytes at offset 20
        if input[20..24] == [0xFD, 0xC4, 0xA7, 0xDC] {
            return true;
        }
        // Also check if it starts with "ZOO "
        input.starts_with(b"ZOO ")
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// ZPAQ archive format
// ZPAQ archives begin with "zPQ" or "7kSt"
mimetype!(ZPAQ, APPLICATION_X_ZPAQ, ".zpaq", b"zPQ" | b"7kSt", name: "ZPAQ Archive", kind: APPLICATION);

// Unix compress format
mimetype!(UNIX_COMPRESS, APPLICATION_X_COMPRESS, ".Z", [0x1F, 0x9D], name: "Unix Compress", kind: APPLICATION);

// ============================================================================
// RETRO GAMING FORMATS (ADDITIONAL)
// ============================================================================

/// Atari 7800 ROM format
static ATARI_7800_ROM: MimeType = MimeType::new(
    APPLICATION_X_ATARI_7800_ROM,
    "Atari 7800 ROM",
    ".a78",
    |input| {
        // A78 header: byte 0 is version, bytes 1-16 contain "ATARI7800" with padding
        if input.len() < 17 {
            return false;
        }
        // Check for "ATARI7800" starting at byte 1
        input[1..].starts_with(b"ATARI7800")
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

/// Commodore 64 Program
static COMMODORE_64_PROGRAM: MimeType = MimeType::new(
    APPLICATION_X_COMMODORE_64_PROGRAM,
    "C64 Program",
    ".prg",
    |input| {
        // C64 PRG files have a 2-byte load address at the start
        // We only detect the most common BASIC program load addresses to avoid false positives
        if input.len() < 10 {
            return false;
        }
        // Check for common C64 BASIC load addresses
        let load_addr = u16::from_le_bytes([input[0], input[1]]);

        // Most common C64 BASIC programs start at 0x0801
        // and typically have BASIC tokens after the load address
        if load_addr == 0x0801 || load_addr == 0x0800 {
            // Check for typical BASIC program structure
            // The next bytes are usually line links and line numbers
            return true;
        }

        // Machine language programs at 0x0C00 (3072) or 0x2000 (8192)
        // These need more bytes to distinguish from random data
        if (load_addr == 0x0C00 || load_addr == 0x2000) && input.len() >= 100 {
            return true;
        }

        false
    },
    &[],
)
.with_kind(MimeKind::APPLICATION);

// Commodore 64 Cartridge - C64 cartridge files start with "C64 CARTRIDGE   " (16 bytes)
mimetype!(COMMODORE_64_CARTRIDGE, APPLICATION_X_COMMODORE_64_CARTRIDGE, ".crt", b"C64 CARTRIDGE   ", name: "Commodore 64 Cartridge", kind: APPLICATION);

// ============================================================================
// EBOOK FORMATS
// ============================================================================

// BroadBand eBook (Sony Reader) - LRF files start with "LRF" followed by version bytes
mimetype!(LRF, APPLICATION_X_LRF, ".lrf", b"L\x00R\x00F\x00\x00\x00", name: "BroadBand eBook", kind: APPLICATION);

// ============================================================================
// OTHER APPLICATION FORMATS
// ============================================================================

// FigletFont ASCII art font - FigletFont files must start with "flf2a"
mimetype!(FIGLET_FONT, APPLICATION_X_FIGLET, ".flf", b"flf2a", name: "FigletFont", kind: APPLICATION, aliases: [APPLICATION_X_FIGLET_FONT]);

// SeqBox archive format - SeqBox files start with "SBx" followed by version
mimetype!(SEQBOX, APPLICATION_X_SBX, ".sbx", b"SBx", name: "SeqBox Container", kind: APPLICATION, aliases: [APPLICATION_X_SEQBOX]);

// Snappy framed format - Snappy framed format starts with the stream identifier "sNaPpY"
// Full identifier is 0xFF 0x06 0x00 0x00 "sNaPpY"
mimetype!(SNAPPY_FRAMED, APPLICATION_X_SNAPPY_FRAMED, ".sz", [0xFF, 0x06, 0x00, 0x00, 0x73, 0x4E, 0x61, 0x50, 0x70, 0x59], name: "Snappy Framed Compression", kind: APPLICATION);

// Tasty format - Tasty files have magic bytes 0x5A 0x54 at the start ("ZT" in ASCII)
mimetype!(TASTY, APPLICATION_X_TASTY, ".tasty", [0x5A, 0x54], name: "TASTY Format", kind: APPLICATION);

// ============================================================================
// ADDITIONAL ARCHIVE FORMATS
// ============================================================================

// PAK archive format - PAK archives start with "PACK"
mimetype!(PAK, APPLICATION_X_PAK, ".pak", b"PACK", name: "PAK Archive", kind: ARCHIVE);

// Mozilla Archive format (used for Firefox/Thunderbird updates)
mimetype!(MOZILLA_ARCHIVE, APPLICATION_X_MOZILLA_ARCHIVE, ".mar", b"MAR1", name: "Mozilla Archive", kind: ARCHIVE);

// RZIP archive format (long-range compression)
mimetype!(RZIP, APPLICATION_X_RZIP, ".rz", b"RZIP", name: "RZIP Archive", kind: ARCHIVE);

// LRZIP archive format (long-range ZIP compression)
mimetype!(LRZIP, APPLICATION_X_LRZIP, ".lrz", b"LRZI", name: "LRZIP Archive", kind: ARCHIVE);

// ============================================================================
// DATABASE FORMATS
// ============================================================================

/// dBASE database format
static DBASE: MimeType = MimeType::new(
    APPLICATION_X_DBF,
    "dBASE",
    ".dbf",
    |input| {
        // dBASE files have version byte in first position
        // Common versions: 0x03 (dBASE III), 0x83 (dBASE III+), 0x8B (dBASE IV), 0xCB, 0xF5, 0xFB
        if input.is_empty() {
            return false;
        }
        matches!(input[0], 0x03 | 0x83 | 0x8B | 0xCB | 0xF5 | 0xFB)
    },
    &[],
)
.with_aliases(&[APPLICATION_X_DBASE])
.with_kind(MimeKind::DATABASE);

// ============================================================================
// ADDITIONAL IMAGE FORMATS
// ============================================================================

/// Adobe Digital Negative (DNG)
static DNG: MimeType = MimeType::new(
    IMAGE_X_ADOBE_DNG,
    "Adobe DNG",
    ".dng",
    |input| {
        // DNG is TIFF-based, check for TIFF header and DNG-specific tags
        // We'll make it a child of TIFF
        if input.len() < 8 {
            return false;
        }
        // Check TIFF header (little or big endian)
        let is_tiff =
            (input[0] == 0x49 && input[1] == 0x49 && input[2] == 0x2A && input[3] == 0x00)
                || (input[0] == 0x4D && input[1] == 0x4D && input[2] == 0x00 && input[3] == 0x2A);

        // For now, we'll detect as DNG if it has TIFF header and check file size/content
        // Real DNG detection would check for specific IFD tags
        is_tiff && input.len() > 1000 // DNG files are typically larger
    },
    &[],
)
.with_kind(MimeKind::IMAGE)
.with_parent(&TIFF); // DNG is based on TIFF

/// Sony ARW Raw format
static ARW: MimeType = MimeType::new(
    IMAGE_X_SONY_ARW,
    "Sony ARW",
    ".arw",
    |input| {
        // ARW is TIFF-based, check for Sony-specific markers
        if input.len() < 8 {
            return false;
        }
        // Check for TIFF header (little-endian common for Sony)
        if !(input[0] == 0x49 && input[1] == 0x49 && input[2] == 0x2A && input[3] == 0x00) {
            return false;
        }
        // Look for Sony markers (simplified check)
        input.len() > 100
    },
    &[],
)
.with_kind(MimeKind::IMAGE)
.with_parent(&TIFF);

/// Pentax PEF Raw format
static PEF: MimeType = MimeType::new(
    IMAGE_X_PENTAX_PEF,
    "Pentax PEF",
    ".pef",
    |input| {
        // PEF is TIFF-based
        if input.len() < 8 {
            return false;
        }
        // Check for TIFF header
        let is_tiff =
            (input[0] == 0x49 && input[1] == 0x49 && input[2] == 0x2A && input[3] == 0x00)
                || (input[0] == 0x4D && input[1] == 0x4D && input[2] == 0x00 && input[3] == 0x2A);

        // Simplified check for PEF
        is_tiff && input.len() > 500
    },
    &[],
)
.with_kind(MimeKind::IMAGE)
.with_parent(&TIFF);

/// Sony SR2 Raw format
static SR2: MimeType = MimeType::new(
    IMAGE_X_SONY_SR2,
    "Sony SR2",
    ".sr2",
    |input| {
        // SR2 is TIFF-based, older Sony format
        if input.len() < 8 {
            return false;
        }
        // Check for TIFF header (little-endian for Sony)
        if !(input[0] == 0x49 && input[1] == 0x49 && input[2] == 0x2A && input[3] == 0x00) {
            return false;
        }
        // Simplified check - SR2 files are consumer camera format, smaller than professional formats
        input.len() > 50 && input.len() < 3_000_000
    },
    &[],
)
.with_kind(MimeKind::IMAGE)
.with_parent(&TIFF);

/// Hasselblad 3FR Raw format
static HASSELBLAD_3FR: MimeType = MimeType::new(
    IMAGE_X_HASSELBLAD_3FR,
    "Hasselblad 3FR",
    ".3fr",
    |input| {
        // 3FR is TIFF-based
        if input.len() < 8 {
            return false;
        }
        // Check for TIFF header (can be either endian)
        let is_tiff =
            (input[0] == 0x49 && input[1] == 0x49 && input[2] == 0x2A && input[3] == 0x00)
                || (input[0] == 0x4D && input[1] == 0x4D && input[2] == 0x00 && input[3] == 0x2A);

        // Simplified check for 3FR - professional camera format, typically large files
        is_tiff && input.len() > 1000
    },
    &[],
)
.with_kind(MimeKind::IMAGE)
.with_parent(&TIFF);

// Minolta MRW Raw format
mimetype!(MRW, IMAGE_X_MINOLTA_MRW, ".mrw", [0x00, 0x4D, 0x52, 0x4D], name: "Minolta Raw Image", kind: IMAGE);

// Kodak KDC Raw format
mimetype!(KODAK_KDC, IMAGE_X_KODAK_KDC, ".kdc", [0x49, 0x49, 0x42, 0x00], name: "Kodak KDC Raw Image", kind: IMAGE);

// Kodak DCR Raw format
mimetype!(KODAK_DCR, IMAGE_X_KODAK_DCR, ".dcr", [0x49, 0x49, 0x55, 0x00], name: "Kodak DCR Raw Image", kind: IMAGE);

// ============================================================================
// CINEMA FORMATS
// ============================================================================

// Cineon digital cinema format
// Cineon can be big-endian or little-endian
mimetype!(CINEON, IMAGE_CINEON, ".cin", [0x80, 0x2A, 0x5F, 0xD7] | [0xD7, 0x5F, 0x2A, 0x80], name: "Cineon Digital Cinema", kind: IMAGE);

// Digital Picture Exchange (DPX) cinema format
// DPX can start with "SDPX" (big-endian) or "XPDS" (little-endian)
mimetype!(DPX, IMAGE_X_DPX, ".dpx", b"SDPX" | b"XPDS", name: "Digital Picture Exchange", kind: IMAGE);

// ============================================================================
// FONT FORMATS
// ============================================================================

mimetype!(TTF, FONT_TTF, ".ttf", b"\x00\x01\x00\x00" | b"true" | b"typ1", name: "TrueType Font", kind: FONT, aliases: [FONT_SFNT, APPLICATION_X_FONT_TTF, APPLICATION_FONT_SFNT]);

mimetype!(WOFF, FONT_WOFF, ".woff", b"wOFF", name: "Web Open Font Format", kind: FONT);

mimetype!(WOFF2, FONT_WOFF2, ".woff2", b"wOF2", name: "Web Open Font Format 2", kind: FONT);

mimetype!(OTF, FONT_OTF, ".otf", b"OTTO", name: "OpenType Font", kind: FONT);

mimetype!(EOT, APPLICATION_VND_MS_FONTOBJECT, ".eot", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b'L', b'P'], name: "Embedded OpenType Font", kind: FONT);

mimetype!(TTC, FONT_COLLECTION, ".ttc", b"ttcf", name: "TrueType Collection", kind: FONT);

// BMFont Binary - AngelCode bitmap font generator binary format
mimetype!(BMFONT_BINARY, APPLICATION_X_ANGELCODE_BMFONT, ".fnt", b"BMF\x03", name: "BMFont Binary", kind: FONT);

// Glyphs - Font editor format (plist-based)
mimetype!(GLYPHS, FONT_X_GLYPHS, ".glyphs", b"{\n.appVe", name: "Glyphs Font", kind: FONT);

// ============================================================================
// WEB & MULTIMEDIA FORMATS
// ============================================================================

// Adobe Flash SWF (Shockwave Flash) - Multimedia container format
// NOTE: Appears in THREE PREFIX_VEC buckets (0x43, 0x46, 0x5A) - this is intentional!
// SWF files use different compression methods indicated by the signature:
//   FWS (0x46): Uncompressed SWF           [PREFIX_VEC 0x46] - Flash 1+
//   CWS (0x43): ZLIB compressed SWF        [PREFIX_VEC 0x43] - Flash 6+
//   ZWS (0x5A): LZMA compressed SWF        [PREFIX_VEC 0x5A] - Flash 13+
// All represent the same SWF format, just with different compression algorithms.
mimetype!(SWF, APPLICATION_X_SHOCKWAVE_FLASH, ".swf", b"FWS" | b"CWS" | b"ZWS", name: "Adobe Flash", kind: APPLICATION);

static CRX: MimeType = MimeType::new(
    APPLICATION_X_CHROME_EXTENSION,
    "Chrome Extension",
    ".crx",
    crx,
    &[],
)
.with_kind(MimeKind::APPLICATION);

mimetype!(P7S, APPLICATION_PKCS7_SIGNATURE, ".p7s", b"-----BEGIN PKCS7-----", name: "PKCS#7 Signature", kind: APPLICATION);

// ============================================================================
// SPECIALIZED FORMATS
// ============================================================================

mimetype!(DCM, APPLICATION_DICOM, ".dcm", offset: (128, b"DICM"), name: "DICOM Medical Image", kind: IMAGE);

static MOBI: MimeType = MimeType::new(
    APPLICATION_X_MOBIPOCKET_EBOOK,
    "Mobipocket Ebook",
    ".mobi",
    mobi,
    &[],
)
.with_kind(MimeKind::DOCUMENT);

mimetype!(LIT, APPLICATION_X_MS_READER, ".lit", b"ITOLITLS", name: "Microsoft Reader eBook", kind: DOCUMENT);

mimetype!(SQLITE3, APPLICATION_VND_SQLITE3, ".sqlite", b"SQLite format 3\x00", name: "SQLite Database", kind: DATABASE, aliases: [APPLICATION_X_SQLITE3]);

mimetype!(FASOO, APPLICATION_X_FASOO, "", offset: (512, b"FASOO   "), name: "Fasoo DRM Document", kind: DOCUMENT, parent: &OLE);

// Adobe InDesign Document - Professional desktop publishing software
mimetype!(INDESIGN, APPLICATION_X_INDESIGN, ".indd", [0x06, 0x06, 0xED, 0xF5, 0xD8, 0x1D, 0x46, 0xE5], name: "Adobe InDesign Document", kind: DOCUMENT);

// Meta Information Encapsulation - Phil Harvey's metadata container format
mimetype!(MIE, APPLICATION_X_MIE, ".mie", [0x7E, 0x10, 0xD4, 0x40, 0x5E, 0x78], name: "Meta Information Encapsulation", kind: APPLICATION);

static PGP_NET_SHARE: MimeType = MimeType::new(
    APPLICATION_X_PGP_NET_SHARE,
    "PGP NetShare",
    "",
    |input| input.starts_with(b"-----BEGIN PGP"),
    &[],
)
.with_parent(&OLE);

// ============================================================================
// MICROSOFT OFFICE & DOCUMENT FORMATS
// ============================================================================

static DOCX: MimeType = MimeType::new(
    APPLICATION_VND_OPENXML_WORDPROCESSINGML_DOCUMENT,
    "Word 2007+",
    ".docx",
    docx,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static XLSX: MimeType = MimeType::new(
    APPLICATION_VND_OPENXML_SPREADSHEETML_SHEET,
    "Excel 2007+",
    ".xlsx",
    xlsx,
    &[],
)
.with_kind(MimeKind::SPREADSHEET)
.with_parent(&ZIP);

static PPTX: MimeType = MimeType::new(
    APPLICATION_VND_OPENXML_PRESENTATIONML_PRESENTATION,
    "PowerPoint 2007+",
    ".pptx",
    pptx,
    &[],
)
.with_kind(MimeKind::PRESENTATION)
.with_parent(&ZIP);

static VSDX: MimeType = MimeType::new(
    APPLICATION_VND_MS_VISIO_DRAWING_MAIN_XML,
    "Visio 2007+",
    ".vsdx",
    vsdx,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static EPUB: MimeType = MimeType::new(APPLICATION_EPUB_ZIP, "EPUB", ".epub", epub, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&ZIP);

static JAR: MimeType = MimeType::new(APPLICATION_JAVA_ARCHIVE, "JAR", ".jar", jar, &[])
    .with_aliases(&[
        APPLICATION_JAR,
        APPLICATION_JAR_ARCHIVE,
        APPLICATION_X_JAVA_ARCHIVE,
    ])
    .with_kind(MimeKind::APPLICATION)
    .with_parent(&ZIP);

static EAR: MimeType = MimeType::new(APPLICATION_X_EAR, "Enterprise Archive", ".ear", ear, &[])
    .with_kind(MimeKind::ARCHIVE)
    .with_parent(&ZIP);

static WAR: MimeType = MimeType::new(APPLICATION_JAVA_ARCHIVE, "JAR", ".war", war, &[])
    .with_kind(MimeKind::APPLICATION)
    .with_parent(&ZIP);

static VSIX: MimeType = MimeType::new(
    APPLICATION_VSIX,
    "Visual Studio Extension",
    ".vsix",
    vsix,
    &[],
)
.with_kind(MimeKind::APPLICATION)
.with_parent(&ZIP);

static APK: MimeType = MimeType::new(
    APPLICATION_VND_ANDROID_PACKAGE_ARCHIVE,
    "Android Package",
    ".apk",
    apk,
    &[],
)
.with_kind(MimeKind::APPLICATION)
.with_parent(&ZIP);

static AAB: MimeType = MimeType::new(
    APPLICATION_VND_ANDROID_AAB,
    "Android App Bundle",
    ".aab",
    aab,
    &[],
)
.with_kind(MimeKind::ARCHIVE)
.with_parent(&ZIP);

static APPX: MimeType = MimeType::new(
    APPLICATION_VND_MS_APPX,
    "Windows App Package",
    ".appx",
    appx,
    &[],
)
.with_kind(MimeKind::APPLICATION)
.with_parent(&ZIP);

static APPXBUNDLE: MimeType = MimeType::new(
    APPLICATION_VND_MS_APPX_BUNDLE,
    "Windows App Bundle",
    ".appxbundle",
    appxbundle,
    &[],
)
.with_kind(MimeKind::APPLICATION)
.with_parent(&ZIP);

static IPA: MimeType = MimeType::new(APPLICATION_X_IOS_APP, "iOS App", ".ipa", ipa, &[])
    .with_kind(MimeKind::APPLICATION)
    .with_parent(&ZIP);

static XAP: MimeType = MimeType::new(
    APPLICATION_X_SILVERLIGHT_APP,
    "Silverlight App",
    ".xap",
    xap,
    &[],
)
.with_kind(MimeKind::APPLICATION)
.with_parent(&ZIP);

static AIR: MimeType = MimeType::new(
    APPLICATION_VND_ADOBE_AIR_APPLICATION_INSTALLER_PACKAGE_ZIP,
    "Adobe AIR Application",
    ".air",
    air,
    &[],
)
.with_kind(MimeKind::APPLICATION)
.with_parent(&ZIP);

static FLA: MimeType = MimeType::new(APPLICATION_VND_ADOBE_FLA, "Adobe Flash", ".fla", fla, &[])
    .with_kind(MimeKind::APPLICATION)
    .with_parent(&ZIP);

static IDML: MimeType = MimeType::new(
    APPLICATION_VND_ADOBE_INDESIGN_IDML_PACKAGE,
    "InDesign Markup Language",
    ".idml",
    idml,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static DOC: MimeType = MimeType::new(APPLICATION_MSWORD, "Word Document", ".doc", doc, &[])
    .with_aliases(&[APPLICATION_VND_MS_WORD])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&OLE);

static WPD: MimeType = MimeType::new(
    APPLICATION_VND_WORDPERFECT,
    "WordPerfect",
    ".wpd",
    wpd,
    &[&WPG, &SHW, &WPM],
)
.with_kind(MimeKind::DOCUMENT);

// ClarisWorks - Apple legacy document format
mimetype!(CLARISWORKS, APPLICATION_X_CLARISWORKS, ".cwk", b"\x02\x00ZWRT", name: "ClarisWorks Document", kind: DOCUMENT);

// Quark Express - Professional publishing software
// Big-endian: 00 00 49 49 58 50 52 ("IIXPR")
// Little-endian: 00 00 4D 4D 58 50 52 ("MMXPR")
mimetype!(QUARK, APPLICATION_VND_QUARK_QUARKXPRESS, ".qxd", b"\x00\x00IIXPR" | b"\x00\x00MMXPR", name: "Quark Express Document", kind: DOCUMENT);

static XLS: MimeType = MimeType::new(APPLICATION_VND_MS_EXCEL, "Excel 97-2003", ".xls", xls, &[])
    .with_aliases(&[APPLICATION_MSEXCEL])
    .with_kind(MimeKind::SPREADSHEET)
    .with_parent(&OLE);

static PPT: MimeType = MimeType::new(
    APPLICATION_VND_MS_POWERPOINT,
    "PowerPoint 97-2003",
    ".ppt",
    ppt,
    &[],
)
.with_aliases(&[APPLICATION_MSPOWERPOINT])
.with_kind(MimeKind::PRESENTATION)
.with_parent(&OLE);

mimetype!(CHM, APPLICATION_VND_MS_HTMLHELP, ".chm", b"ITSF\x03\x00\x00\x00", name: "HTML Help", kind: DOCUMENT);

static ONENOTE: MimeType = MimeType::new(APPLICATION_ONENOTE, "OneNote", ".one", onenote, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&OLE);

static PUB: MimeType = MimeType::new(
    APPLICATION_VND_MS_PUBLISHER,
    "Publisher",
    ".pub",
    pub_format,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&OLE);

static MSG: MimeType = MimeType::new(APPLICATION_VND_MS_OUTLOOK, "Outlook MSG", ".msg", msg, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&OLE);

static PST: MimeType = MimeType::new(
    APPLICATION_VND_MS_OUTLOOK_PST,
    "Outlook PST",
    ".pst",
    pst,
    &[],
)
.with_kind(MimeKind::DOCUMENT);

static MPP: MimeType = MimeType::new(
    APPLICATION_VND_MS_PROJECT,
    "Microsoft Project",
    ".mpp",
    mpp,
    &[],
)
.with_kind(MimeKind::DOCUMENT)
.with_parent(&OLE);

// Microsoft Visio Drawing - OLE-based diagram/drawing format
static VSD: MimeType = MimeType::new(APPLICATION_VND_VISIO, "Visio", ".vsd", vsd, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&OLE);

// Microsoft Works Database - Early version (v1-2) with unique header
mimetype!(WORKS_DB, APPLICATION_VND_MS_WORKS_DB, ".wdb", [0x20, 0x54, 0x02, 0x00, 0x00, 0x00, 0x05, 0x54, 0x02, 0x00], name: "Works Database", kind: DOCUMENT);

// Microsoft Works Spreadsheet - Two signature variants
mimetype!(WORKS_SPREADSHEET, APPLICATION_VND_MS_WORKS, ".wks", b"\x00\x00\x02\x00\x04\x04\x05\x54\x02\x00" | b"\xFF\x00\x02\x00\x04\x04\x05\x54\x02\x00", name: "Works Spreadsheet", kind: DOCUMENT);

// Microsoft Write - Legacy word processor (v3.0 and v3.1)
// NOTE: Appears in TWO PREFIX_VEC buckets (0x31 and 0x32) - this is intentional!
// Different versions start with different first bytes:
//   v3.0: 0x31 0xBE 0x00 0x00 0x00 0xAB
//   v3.1: 0x32 0xBE 0x00 0x00 0x00 0xAB
// Both are the same format, just different version markers.
mimetype!(MICROSOFT_WRITE, APPLICATION_X_MSWRITE, ".wri", b"\x31\xBE\x00\x00\x00\xAB" | b"\x32\xBE\x00\x00\x00\xAB", name: "Microsoft Write", kind: DOCUMENT);

static MSI: MimeType = MimeType::new(
    APPLICATION_X_MS_INSTALLER,
    "Windows Installer",
    ".msi",
    msi,
    &[],
)
.with_aliases(&[APPLICATION_X_WINDOWS_INSTALLER, APPLICATION_X_MSI])
.with_kind(MimeKind::ARCHIVE.union(MimeKind::EXECUTABLE))
.with_parent(&OLE);

static MSP: MimeType = MimeType::new(
    APPLICATION_X_MS_PATCH,
    "Windows Installer Patch",
    ".msp",
    msp,
    &[],
)
.with_aliases(&[APPLICATION_X_MSP])
.with_kind(MimeKind::ARCHIVE.union(MimeKind::EXECUTABLE))
.with_parent(&OLE);

// ============================================================================
// OPEN DOCUMENT FORMATS
// ============================================================================

static ODT: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT,
    "OpenDocument Text",
    ".odt",
    odt,
    &[&OTT],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.text"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static ODS: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET,
    "OpenDocument Spreadsheet",
    ".ods",
    ods,
    &[&OTS],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.spreadsheet"])
.with_kind(MimeKind::SPREADSHEET)
.with_parent(&ZIP);

static ODP: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION,
    "OpenDocument Presentation",
    ".odp",
    odp,
    &[&OTP],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.presentation"])
.with_kind(MimeKind::PRESENTATION)
.with_parent(&ZIP);

static ODG: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS,
    "OpenDocument Graphics",
    ".odg",
    odg,
    &[&OTG],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.graphics"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static ODF: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA,
    "OpenDocument Formula",
    ".odf",
    odf_format,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.formula"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static ODC: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_CHART,
    "ODF Chart",
    ".odc",
    odc,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.chart"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static ODB: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_DATABASE,
    "OpenDocument Database",
    ".odb",
    odb,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.database"])
.with_kind(MimeKind::DATABASE)
.with_parent(&ZIP);

static ODM: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_MASTER,
    "OpenDocument Text Master",
    ".odm",
    odm,
    &[&OTM],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.text-master"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ZIP);

static ORA: MimeType = MimeType::new(IMAGE_OPENRASTER, "OpenRaster", ".ora", ora, &[])
    .with_kind(MimeKind::IMAGE)
    .with_parent(&ZIP);

static OTT: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE,
    "OpenDocument Text Template",
    ".ott",
    ott,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.text-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODT);

static OTS: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE,
    "OpenDocument Spreadsheet Template",
    ".ots",
    ots,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.spreadsheet-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODS);

static OTP: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE,
    "OpenDocument Presentation Template",
    ".otp",
    otp,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.presentation-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODP);

static OTG: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE,
    "OpenDocument Graphics Template",
    ".otg",
    otg,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.graphics-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODG);

static OTM: MimeType = MimeType::new(
    APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_MASTER_TEMPLATE,
    "OpenDocument Text Master Template",
    ".otm",
    otm,
    &[],
)
.with_aliases(&["application/x-vnd.oasis.opendocument.text-master-template"])
.with_kind(MimeKind::DOCUMENT)
.with_parent(&ODM);

static SXC: MimeType = MimeType::new(
    APPLICATION_VND_SUN_XML_CALC,
    "StarOffice Calc",
    ".sxc",
    sxc,
    &[],
)
.with_kind(MimeKind::SPREADSHEET)
.with_parent(&ZIP);

static KMZ: MimeType = MimeType::new(APPLICATION_VND_GOOGLE_EARTH_KMZ, "KMZ", ".kmz", kmz, &[])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&ZIP);

// ============================================================================
// DATABASE FORMATS
// ============================================================================

static MDB: MimeType = MimeType::new(APPLICATION_X_MSACCESS, "Msaccess", ".mdb", mdb, &[])
    .with_kind(MimeKind::DATABASE);

static ACCDB: MimeType = MimeType::new(APPLICATION_X_MSACCESS, "Msaccess", ".accdb", accdb, &[])
    .with_kind(MimeKind::DATABASE);

static DBF: MimeType =
    MimeType::new(APPLICATION_X_DBF, "Dbf", ".dbf", dbf, &[]).with_kind(MimeKind::DATABASE);

// Lotus 1-2-3 v1 (.wk1)
static LOTUS_WK1: MimeType = MimeType::new(
    APPLICATION_VND_LOTUS_1_2_3,
    "Lotus 1-2-3",
    ".wk1",
    |input| {
        // Check for v1 signature: 00 00 02 00 06 04 06 00
        // Version at offset 4-7: 06 04 06 00 = 0x00060406 (little-endian)
        input.len() >= 8
            && u32::from_le_bytes([input[4], input[5], input[6], input[7]]) == 0x00060406
    },
    &[],
)
.with_kind(MimeKind::SPREADSHEET.union(MimeKind::DATABASE));

// Lotus 1-2-3 v3 (.wk3)
static LOTUS_WK3: MimeType = MimeType::new(
    APPLICATION_VND_LOTUS_1_2_3,
    "Lotus 1-2-3",
    ".wk3",
    |input| {
        // Check for v3 signature: 00 00 1A 00 00 10 04 00
        // Version at offset 4-7: 00 10 04 00 = 0x00041000 (little-endian)
        input.len() >= 8
            && u32::from_le_bytes([input[4], input[5], input[6], input[7]]) == 0x00041000
    },
    &[],
)
.with_kind(MimeKind::SPREADSHEET.union(MimeKind::DATABASE));

// Lotus 1-2-3 v4/v5 (.wk4)
static LOTUS_WK4: MimeType = MimeType::new(
    APPLICATION_VND_LOTUS_1_2_3,
    "Lotus 1-2-3",
    ".wk4",
    |input| {
        // Check for v4/v5 signature: 00 00 1A 00 02 10 04 00
        // Version at offset 4-7: 02 10 04 00 = 0x00041002 (little-endian)
        input.len() >= 8
            && u32::from_le_bytes([input[4], input[5], input[6], input[7]]) == 0x00041002
    },
    &[],
)
.with_kind(MimeKind::SPREADSHEET.union(MimeKind::DATABASE))
.with_extension_aliases(&[".wk5"]);

// Lotus 1-2-3 parent format (.123) - matches all versions (v1-v5)
// Children (WK1, WK3, WK4) refine detection to specific versions based on version field
static LOTUS123: MimeType = MimeType::new(
    APPLICATION_VND_LOTUS_1_2_3,
    "Lotus 1-2-3",
    ".123",
    lotus123,
    &[&LOTUS_WK1, &LOTUS_WK3, &LOTUS_WK4], // Children for specific versions
)
.with_kind(MimeKind::SPREADSHEET.union(MimeKind::DATABASE));

// Lotus Notes Database - Enterprise collaboration database
mimetype!(LOTUS_NOTES, APPLICATION_VND_LOTUS_NOTES, ".nsf", b"\x1A\x00\x00", name: "Lotus Notes Database", kind: DATABASE);

static MRC: MimeType = MimeType::new(APPLICATION_MARC, "MARC", ".mrc", marc, &[])
    .with_kind(MimeKind::TEXT.union(MimeKind::DATABASE));

// ============================================================================
// PROGRAMMING & TEXT FORMATS
// ============================================================================

static PHP: MimeType =
    MimeType::new(TEXT_X_PHP, "PHP Source Code", ".php", php, &[]).with_parent(&UTF8);

static JAVASCRIPT: MimeType = MimeType::new(TEXT_JAVASCRIPT, "JavaScript", ".js", javascript, &[])
    .with_aliases(&[APPLICATION_JAVASCRIPT])
    .with_parent(&UTF8);

static JAVA: MimeType =
    MimeType::new(TEXT_X_JAVA, "Java Source Code", ".java", java, &[]).with_parent(&UTF8);

static TYPESCRIPT: MimeType = MimeType::new(
    TEXT_X_TYPESCRIPT,
    "TypeScript Source Code",
    ".ts",
    typescript,
    &[],
)
.with_aliases(&[APPLICATION_X_TYPESCRIPT])
.with_extension_aliases(&[".tsx"])
.with_parent(&UTF8);

static CPP: MimeType = MimeType::new(TEXT_X_CPP, "C++ Source Code", ".cpp", cpp, &[])
    .with_aliases(&[TEXT_X_CXX, TEXT_X_CPPSRC])
    .with_extension_aliases(&[".cc", ".cxx", ".hpp", ".hxx", ".h++"])
    .with_parent(&C_LANG);

static C_LANG: MimeType = MimeType::new(TEXT_X_C, "C Source Code", ".c", c_lang, &[&CPP])
    .with_aliases(&[TEXT_X_CSRC])
    .with_extension_aliases(&[".h"])
    .with_parent(&UTF8);

static GO_LANG: MimeType =
    MimeType::new(TEXT_X_GO, "Go Source Code", ".go", go_lang, &[]).with_parent(&UTF8);

static RUST_LANG: MimeType =
    MimeType::new(TEXT_X_RUST, "Rust Source Code", ".rs", rust_lang, &[]).with_parent(&UTF8);

static CSHARP: MimeType =
    MimeType::new(TEXT_X_CSHARP, "C# Source Code", ".cs", csharp, &[]).with_parent(&UTF8);

static VB: MimeType =
    MimeType::new(TEXT_X_VB, "Visual Basic Source Code", ".vb", vb, &[]).with_parent(&UTF8);

static PYTHON: MimeType = MimeType::new(TEXT_X_PYTHON, "Python Source Code", ".py", python, &[])
    .with_aliases(&[TEXT_X_SCRIPT_PYTHON, APPLICATION_X_PYTHON])
    .with_parent(&UTF8);

static PERL: MimeType =
    MimeType::new(TEXT_X_PERL, "Perl Source Code", ".pl", perl, &[]).with_parent(&UTF8);

static RUBY: MimeType = MimeType::new(TEXT_X_RUBY, "Ruby Source Code", ".rb", ruby, &[])
    .with_aliases(&[APPLICATION_X_RUBY])
    .with_parent(&UTF8);

static LUA: MimeType =
    MimeType::new(TEXT_X_LUA, "Lua Source Code", ".lua", lua, &[]).with_parent(&UTF8);

mimetype!(SHELL, TEXT_X_SHELLSCRIPT, ".sh", b"#!/bin/sh" | b"#!/bin/bash" | b"#!/usr/bin/env bash" | b"#!/bin/zsh", name: "Shell Script", kind: TEXT, aliases: [TEXT_X_SH, APPLICATION_X_SHELLSCRIPT, APPLICATION_X_SH], parent: &UTF8);

mimetype!(BATCH, TEXT_X_MSDOS_BATCH, ".bat", b"REM " | b"@ECHO OFF" | b"@echo off" | b"@Echo Off", name: "Batch Script", kind: TEXT, ext_aliases: [".cmd"], parent: &UTF8);

mimetype!(TCL, TEXT_X_TCL, ".tcl", b"#!/usr/bin/env tclsh" | b"#!/usr/bin/tclsh" | b"#!tclsh", name: "Tcl Script", kind: TEXT, aliases: [APPLICATION_X_TCL], parent: &UTF8);

mimetype!(CLOJURE, TEXT_X_CLOJURE, ".clj", b"#!/usr/local/bin/clojure" | b"#!/usr/bin/env clojure" | b"#!/usr/local/bin/clj" | b"#!/usr/bin/env clj" | b"#!clojure", name: "Clojure Source Code", kind: TEXT, parent: &UTF8);

mimetype!(LATEX, TEXT_X_TEX, ".tex", b"\\documentclass" | b"\\documentstyle", name: "LaTeX Document", kind: TEXT, parent: &UTF8);

static VISUAL_STUDIO_SOLUTION: MimeType = MimeType::new(
    APPLICATION_VND_MS_DEVELOPER,
    "Visual Studio Solution",
    ".sln",
    visual_studio_solution,
    &[],
)
.with_parent(&UTF8);

// JSON Feed - RSS/Atom alternative in JSON format
mimetype!(JSON_FEED, APPLICATION_FEED_JSON, ".json", b"{\"version", name: "JSON Feed", kind: TEXT);

static JSON: MimeType = MimeType::new(
    APPLICATION_JSON,
    "Application Json",
    ".json",
    json,
    &[&GEOJSON, &NDJSON, &HAR, &GLTF],
)
.with_parent(&UTF8);

static GEOJSON: MimeType =
    MimeType::new(APPLICATION_GEO_JSON, "Geo JSON", ".geojson", geojson, &[]).with_parent(&JSON);

static NDJSON: MimeType =
    MimeType::new(APPLICATION_X_NDJSON, "Ndjson", ".ndjson", ndjson, &[]).with_parent(&JSON);

static CSV_FORMAT: MimeType =
    MimeType::new(TEXT_CSV, "CSV", ".csv", csv_format, &[]).with_parent(&UTF8);

static TSV: MimeType = MimeType::new(
    TEXT_TAB_SEPARATED_VALUES,
    "Tab Separated Values",
    ".tsv",
    tsv,
    &[],
)
.with_parent(&UTF8);

static PSV: MimeType = MimeType::new(
    TEXT_PIPE_SEPARATED_VALUES,
    "Pipe Separated Values",
    ".psv",
    psv,
    &[],
)
.with_parent(&UTF8);

static SSV: MimeType = MimeType::new(
    TEXT_SEMICOLON_SEPARATED_VALUES,
    "Semicolon Separated Values",
    ".ssv",
    ssv,
    &[],
)
.with_parent(&UTF8);

mimetype!(RTF, TEXT_RTF, ".rtf", b"{\\rtf", name: "Rich Text Format", kind: DOCUMENT, aliases: [APPLICATION_RTF], parent: &UTF8);

static SRT: MimeType = MimeType::new(APPLICATION_X_SUBRIP, "SubRip", ".srt", srt, &[])
    .with_aliases(&[APPLICATION_X_SRT, TEXT_X_SRT])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&UTF8);

static VTT: MimeType = MimeType::new(TEXT_VTT, "WebVTT", ".vtt", vtt, &[]).with_parent(&UTF8);

static VCARD: MimeType = MimeType::new(TEXT_VCARD, "vCard", ".vcf", vcard, &[]).with_parent(&UTF8);

static ICALENDAR: MimeType =
    MimeType::new(TEXT_CALENDAR, "Calendar", ".ics", icalendar, &[]).with_parent(&UTF8);

static SVG: MimeType = MimeType::new(IMAGE_SVG_XML, "SVG", ".svg", svg, &[])
    .with_kind(MimeKind::IMAGE)
    .with_parent(&XML);

// ============================================================================
// XML-BASED FORMATS
// ============================================================================

static RSS: MimeType = MimeType::new(APPLICATION_RSS_XML, "RSS", ".rss", rss, &[])
    .with_aliases(&[TEXT_RSS])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static ATOM: MimeType = MimeType::new(APPLICATION_ATOM_XML, "Atom", ".atom", atom, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static X3D: MimeType = MimeType::new(MODEL_X3D_XML, "X3D XML", ".x3d", x3d, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static KML: MimeType = MimeType::new(
    APPLICATION_VND_GOOGLE_EARTH_KML_XML,
    "KML",
    ".kml",
    kml,
    &[],
)
.with_kind(MimeKind::TEXT)
.with_parent(&XML);

static XLIFF: MimeType = MimeType::new(APPLICATION_X_XLIFF_XML, "XLIFF", ".xlf", xliff, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static COLLADA: MimeType = MimeType::new(MODEL_VND_COLLADA_XML, "COLLADA", ".dae", collada, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&XML);

static GML: MimeType = MimeType::new(APPLICATION_GML_XML, "GML", ".gml", gml, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static GPX: MimeType = MimeType::new(APPLICATION_GPX_XML, "GPX", ".gpx", gpx, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static TCX: MimeType = MimeType::new(APPLICATION_VND_GARMIN_TCX_XML, "TCX", ".tcx", tcx, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static AMF: MimeType = MimeType::new(APPLICATION_X_AMF, "AMF", ".amf", amf, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&XML);

static THREEMF: MimeType = MimeType::new(
    APPLICATION_VND_MS_PACKAGE_3DMANUFACTURING_3DMODEL_XML,
    "3D Manufacturing Format",
    ".3mf",
    threemf,
    &[],
)
.with_kind(MimeKind::MODEL)
.with_parent(&XML);

static XFDF: MimeType = MimeType::new(APPLICATION_VND_ADOBE_XFDF, "XFDF", ".xfdf", xfdf, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static OWL2: MimeType = MimeType::new(APPLICATION_OWL_XML, "OWL", ".owl", owl2, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static XHTML: MimeType = MimeType::new(APPLICATION_XHTML_XML, "XHTML", ".html", xhtml, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&XML);

static FB2: MimeType = MimeType::new(APPLICATION_X_FB2_XML, "Fb2 XML", ".fb2", fb2, &[])
    .with_aliases(&[APPLICATION_X_FICTIONBOOK_XML])
    .with_kind(MimeKind::DOCUMENT)
    .with_parent(&XML);

static HAR: MimeType = MimeType::new(APPLICATION_JSON_HAR, "HAR", ".har", har, &[])
    .with_kind(MimeKind::TEXT)
    .with_parent(&JSON);

// ============================================================================
// 3D & GEOSPATIAL FORMATS
// ============================================================================

static SHP: MimeType = MimeType::new(APPLICATION_VND_SHP, "Shapefile", ".shp", shp, &[]);

static SHX: MimeType = MimeType::new(
    APPLICATION_VND_SHX,
    "Shapefile Index",
    ".shx",
    |input| input.starts_with(b"\x00\x00\x27\x0A"),
    &[&SHP],
);

mimetype!(GLB, MODEL_GLTF_BINARY, ".glb", b"glTF\x02\x00\x00\x00" | b"glTF\x01\x00\x00\x00", name: "glTF Binary", kind: MODEL);

static GLTF: MimeType = MimeType::new(MODEL_GLTF_JSON, "glTF JSON", ".gltf", gltf, &[])
    .with_kind(MimeKind::MODEL)
    .with_parent(&JSON);

// Universal 3D - PDF 3D embedding format
mimetype!(U3D, MODEL_U3D, ".u3d", b"U3D\0", name: "Universal 3D", kind: MODEL);

// ============================================================================
// GAMING FORMATS
// ============================================================================

mimetype!(NES, APPLICATION_VND_NINTENDO_SNES_ROM, ".nes", b"NES\x1A", name: "Nintendo NES ROM", kind: APPLICATION);

// ============================================================================
// MISCELLANEOUS FORMATS
// ============================================================================

// HDF4 - Hierarchical Data Format version 4
mimetype!(HDF4, APPLICATION_X_HDF, ".hdf", b"\x0e\x03\x13\x01", name: "Hierarchical Data Format 4", kind: DATABASE, ext_aliases: [".hdf4"]);

// HDF5 - Hierarchical Data Format version 5
mimetype!(HDF5, APPLICATION_X_HDF5, ".hdf5", b"\x89HDF\r\n\x1a\n", name: "Hierarchical Data Format 5", kind: DATABASE, ext_aliases: [".h5"]);

// HDF parent - for backward compatibility, checks both HDF4 and HDF5
mimetype!(HDF, APPLICATION_X_HDF, ".hdf", b"\x89HDF\r\n\x1a\n" | b"\x0e\x03\x13\x01", name: "Hierarchical Data Format", kind: DATABASE, children: [&HDF4, &HDF5]);

// GRIB weather data format (used by meteorology services)
mimetype!(GRIB, APPLICATION_X_GRIB, ".grib", b"GRIB", name: "GRIB Weather Data", kind: APPLICATION);

mimetype!(CBOR_FORMAT, APPLICATION_CBOR, ".cbor", b"\xd9\xd9\xf7", name: "CBOR Data Format", kind: APPLICATION);

mimetype!(PARQUET, APPLICATION_VND_APACHE_PARQUET, ".parquet", b"PAR1", name: "Apache Parquet", kind: DATABASE, aliases: [APPLICATION_X_PARQUET]);

mimetype!(LNK, APPLICATION_X_MS_SHORTCUT, ".lnk", b"L\x00\x00\x00\x01\x14\x02\x00", name: "Windows Shortcut", kind: APPLICATION);

// Windows Help format
mimetype!(HLP, APPLICATION_WINHELP, ".hlp", b"\x3F\x5F\x03\x00", name: "Windows Help", kind: APPLICATION);

// Windows Event Log
mimetype!(EVT, APPLICATION_X_MS_EVT, ".evt", b"\x30\x00\x00\x00\x4C\x66\x4C\x65", name: "Windows Event Log", kind: APPLICATION);

// Windows Event Log XML
mimetype!(EVTX, APPLICATION_X_MS_EVTX, ".evtx", b"ElfFile", name: "Windows Event Log XML", kind: APPLICATION);

// Windows Registry file
static WINDOWS_REG: MimeType = MimeType::new(
    TEXT_PLAIN,
    "Windows Registry",
    ".reg",
    |input| {
        if input.len() < 7 {
            return false;
        }
        // Check for "REGEDIT" (ASCII) or "REGEDIT4"
        if input.starts_with(b"REGEDIT") {
            return true;
        }
        // Check for UTF-16 BOM (FF FE) followed by "REGEDIT" in UTF-16
        if input.len() >= 16 && input[0] == 0xFF && input[1] == 0xFE {
            // Check for "REGEDIT" in UTF-16LE: R=0x52, E=0x45, G=0x47, etc.
            return input[2] == 0x52 && input[3] == 0x00  // R
                && input[4] == 0x65 && input[5] == 0x00  // e
                && input[6] == 0x67 && input[7] == 0x00; // g
        }
        false
    },
    &[],
)
.with_kind(MimeKind::TEXT);

// Windows Static Cursor
mimetype!(CUR, IMAGE_X_WIN_CUR, ".cur", b"\x00\x00\x02\x00", name: "Windows Cursor", kind: IMAGE);

static MACHO: MimeType = MimeType::new(APPLICATION_X_MACH_BINARY, "Mach-O", ".macho", macho, &[])
    .with_kind(MimeKind::EXECUTABLE);

mimetype!(TZIF, APPLICATION_TZIF, "", b"TZif", name: "Time Zone Information Format", kind: APPLICATION);

// Amiga Disk File - Amiga floppy disk image
static ADF: MimeType = MimeType::new(
    APPLICATION_X_AMIGA_DISK_FORMAT,
    "Amiga Disk File",
    ".adf",
    |input| {
        // ADF starts with "DOS" followed by a byte 0x00-0x05
        input.len() >= 4 && input.starts_with(b"DOS") && input[3] <= 0x05
    },
    &[],
)
.with_kind(MimeKind::DOCUMENT);

// Common Object File Format - i386 variant
mimetype!(COFF, APPLICATION_X_COFF, ".o", [0x4C, 0x01], name: "Common Object File Format", kind: EXECUTABLE);

// Gettext Machine Object - Compiled translation file (little-endian)
mimetype!(MO, APPLICATION_X_GETTEXT_TRANSLATION, ".mo", [0xDE, 0x12, 0x04, 0x95], name: "Gettext Translation", kind: DOCUMENT);

// ============================================================================
// NETWORK & DEBUGGING FORMATS
// ============================================================================

// PCAP - Network packet capture (libpcap format) - big-endian or little-endian
mimetype!(PCAP, APPLICATION_VND_TCPDUMP_PCAP, ".pcap", [0xA1, 0xB2, 0xC3, 0xD4] | [0xD4, 0xC3, 0xB2, 0xA1], name: "Packet Capture", kind: DOCUMENT);

// PCAPNG - Next generation packet capture
mimetype!(PCAPNG, APPLICATION_X_PCAPNG, ".pcapng", [0x0A, 0x0D, 0x0D, 0x0A], name: "Next Generation Packet Capture", kind: DOCUMENT);

// ============================================================================
// 3D & CAD FORMATS
// ============================================================================

// Blender - 3D modeling and animation
mimetype!(BLEND, APPLICATION_X_BLENDER, ".blend", b"BLENDER", name: "Blender 3D", kind: DOCUMENT);

// 3D Studio Max - Autodesk 3DS mesh format
// Starts with 0x4D4D but must distinguish from TIFF big-endian (MM\x00*) and Olympus ORF (MMOR)
static AUTODESK_3DS: MimeType = MimeType::new(
    APPLICATION_X_3DS,
    "3DS Model",
    ".3ds",
    |input| {
        input.starts_with(b"MM") && !input.starts_with(b"MM\x00*") && !input.starts_with(b"MMOR")
    },
    &[],
)
.with_kind(MimeKind::MODEL);

// 3D Studio Max - Autodesk .max project file format
// OLE-based binary format with structured storage
static AUTODESK_MAX: MimeType =
    MimeType::new(APPLICATION_X_MAX, "3DS Max", ".max", autodesk_max, &[])
        .with_kind(MimeKind::MODEL)
        .with_parent(&OLE);

// PLY - Polygon File Format (3D models)
mimetype!(PLY, APPLICATION_PLY, ".ply", b"ply\n", name: "Polygon File Format", kind: DOCUMENT);

// FBX - Autodesk Filmbox (3D interchange format)
mimetype!(FBX, APPLICATION_VND_AUTODESK_FBX, ".fbx", b"Kaydara FBX Binary  \x00", name: "Autodesk Filmbox", kind: DOCUMENT);

// FIT - Flexible and Interoperable Data Transfer (Garmin fitness/GPS data format)
mimetype!(FIT, APPLICATION_X_FIT, ".fit", offset: (8, b".FIT"), name: "Garmin FIT", kind: DOCUMENT);

// STL ASCII - STereoLithography ASCII format (3D printing)
// STL ASCII files start with "solid " followed by an optional name
mimetype!(STL_ASCII, MODEL_X_STL_ASCII, ".stl", b"solid ", name: "STL ASCII", kind: DOCUMENT, aliases: [MODEL_STL]);

// Maya Binary - Autodesk Maya binary scene file
// Maya binary files start with "FOR4" (32-bit) or "FOR8" (64-bit)
mimetype!(MAYA_BINARY, APPLICATION_X_MAYA_BINARY, ".mb", b"FOR4" | b"FOR8", name: "Autodesk Maya Binary", kind: DOCUMENT);

// Maya ASCII - Autodesk Maya ASCII scene file
// Maya ASCII files start with "//Maya ASCII" followed by version
mimetype!(MAYA_ASCII, APPLICATION_X_MAYA_ASCII, ".ma", b"//Maya ASCII", name: "Autodesk Maya ASCII", kind: DOCUMENT);

// InterQuake Model - 3D model format for games
mimetype!(IQM, MODEL_X_IQM, ".iqm", b"INTERQUAKEMODEL\0", name: "InterQuake Model", kind: MODEL);

// MagicaVoxel - Voxel model format
mimetype!(VOX, MODEL_X_VOX, ".vox", b"VOX ", name: "MagicaVoxel", kind: MODEL);

// Google Draco - 3D geometry compression format
mimetype!(DRACO, MODEL_X_DRACO, ".drc", b"DRACO", name: "Google Draco", kind: MODEL);

// STEP - ISO 10303-21 3D CAD data exchange format
mimetype!(STEP, MODEL_STEP, ".stp", b"ISO-10303-21;", name: "STEP CAD", kind: MODEL);

// VRML - Virtual Reality Modeling Language (supports both v1.0 and v2.0)
mimetype!(VRML, MODEL_VRML, ".wrl", b"#VRML V", name: "Virtual Reality Modeling Language", kind: MODEL);

// Cinema4D - Maxon Cinema 4D 3D model format
mimetype!(CINEMA4D, MODEL_X_C4D, ".c4d", b"QC4DC4D6", name: "Cinema 4D", kind: MODEL);

// Autodesk Alias - 3D modeling and industrial design format
mimetype!(AUTODESK_ALIAS, MODEL_X_WIRE, ".wire", b"WIRE", name: "Autodesk Alias", kind: MODEL);

// Design Web Format - Autodesk DWF CAD format
mimetype!(DWF, MODEL_VND_DWF, ".dwf", b"(DWF", name: "Design Web Format", kind: MODEL);

// OpenNURBS - Rhino 3D model format
mimetype!(OPENNURBS, MODEL_X_3DM, ".3dm", b"3D Geometry File", name: "OpenNURBS", kind: MODEL);

// Universal Scene Description Binary - Pixar USD format
mimetype!(USD_BINARY, MODEL_X_USD, ".usd", b"PXR-USDC", name: "Universal Scene Description Binary", kind: MODEL);

// Universal Scene Description ASCII - Pixar USD text format
mimetype!(USD_ASCII, MODEL_X_USD_ASCII, ".usda", b"#usda", name: "Universal Scene Description ASCII", kind: MODEL);

// Model3D Binary - Binary 3D model format
mimetype!(MODEL3D_BINARY, MODEL_X_3D_BINARY, ".3d", b"MD30", name: "Model3D Binary", kind: MODEL);

// SketchUp - Trimble SketchUp 3D model format
mimetype!(SKETCHUP, APPLICATION_VND_SKETCHUP_SKP, ".skp", [0xFF, 0xFE, 0xFF, 0x0E, 0x53, 0x00, 0x6B, 0x00], name: "SketchUp", kind: MODEL);

// ============================================================================
// VIRTUAL MACHINE & DISK IMAGE FORMATS
// ============================================================================

// QCOW - QEMU Copy-on-Write version 1 disk image
mimetype!(QCOW, APPLICATION_X_QEMU_DISK, ".qcow", b"QFI", name: "QEMU Copy-on-Write", kind: DOCUMENT);

// QCOW2 - QEMU Copy-on-Write version 2 disk image
mimetype!(QCOW2, APPLICATION_X_QEMU_DISK, ".qcow2", b"QFI\xFB", name: "QEMU Copy-on-Write 2", kind: DOCUMENT);

// VHD - Microsoft Virtual Hard Disk (legacy format)
// VHD files have "conectix" magic either at the beginning (dynamic) or at offset from end (fixed)
mimetype!(VHD, APPLICATION_X_VHD, ".vhd", b"conectix", name: "Microsoft Virtual Hard Disk", kind: DOCUMENT);

// VHDX - Microsoft Virtual Hard Disk v2
mimetype!(VHDX, APPLICATION_X_VHDX, ".vhdx", b"vhdxfile", name: "Microsoft Virtual Hard Disk v2", kind: DOCUMENT);

// VMDK - VMware Virtual Disk
// VMDK has multiple possible magic bytes:
// - "KDMV" - VMware 4 hosted sparse extent
// - "COWD" - VMware 3 hosted sparse extent
// - "# Disk DescriptorFile" - descriptor file (text-based)
mimetype!(VMDK, APPLICATION_X_VMDK, ".vmdk", b"KDMV" | b"COWD" | b"# Disk DescriptorFile", name: "VMware Virtual Disk", kind: DOCUMENT);

// VDI - VirtualBox Virtual Disk Image
// VDI signature is at offset 0x40 (64 bytes): 0x7F 0x10 0xDA 0xBE
mimetype!(VDI, APPLICATION_X_VIRTUALBOX_VDI, ".vdi", offset: (64, b"\x7F\x10\xDA\xBE"), name: "VirtualBox Virtual Disk Image", kind: DOCUMENT);

// WIM - Windows Imaging Format
mimetype!(WIM, APPLICATION_X_MS_WIM, ".wim", b"MSWIM\x00\x00\x00", name: "Windows Imaging Format", kind: DOCUMENT);

// ============================================================================
// FILESYSTEM FORMATS
// ============================================================================

// Squashfs - Compressed read-only filesystem used in embedded systems and live CDs.
// Squashfs can be big-endian 'sqsh' or little-endian 'hsqs'
mimetype!(SQUASHFS, APPLICATION_X_SQUASHFS, ".squashfs", b"sqsh" | b"hsqs", name: "Squashfs", kind: DOCUMENT);

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

fn fb2(input: &[u8]) -> bool {
    detect_xml_with_tag(input, b"<FictionBook")
}

fn usf(input: &[u8]) -> bool {
    // Universal Subtitle Format - XML-based with <USFSubtitles> root element
    detect_xml_with_tag(input, b"<USFSubtitles")
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

// Detects ZIP archives and ZIP-based formats.
// ZIP files use the "PK" signature (named after Phil Katz) followed by
// different headers for various ZIP record types:
// - PK\x03\x04: Local file header (most common)
// - PK\x05\x06: End of central directory record
// - PK\x07\x08: Data descriptor record
// This also matches ZIP-based formats like DOCX, XLSX, EPUB, JAR, etc.
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
    if input.starts_with(b"\xEF\xBB\xBF")
        || input.starts_with(b"\xFE\xFF")
        || input.starts_with(b"\xFF\xFE")
    {
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
    input.trim_ascii_start().starts_with(b"<?xml")
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

fn ogg_media(input: &[u8]) -> bool {
    if input.len() < 37 {
        return false;
    }

    // OGM (Ogg Media) specific headers at offset 28
    let offset_28 = &input[28..];
    offset_28.starts_with(b"\x01video\x00\x00\x00")
        || offset_28.starts_with(b"\x01audio\x00\x00\x00")
}

fn ogg_multiplexed(_input: &[u8]) -> bool {
    // OGX (Ogg Multiplexed) is difficult to detect via signature alone
    // It requires checking for multiple stream types, which is complex
    // For now, this will not be auto-detected
    false
}

fn mobi(input: &[u8]) -> bool {
    input.len() >= 68 && &input[60..64] == b"BOOKMOBI"
}

fn heic(input: &[u8]) -> bool {
    input.len() >= 12 && &input[4..12] == b"ftypheic"
        || input.len() >= 12 && &input[4..12] == b"ftypheix"
}

fn heif(input: &[u8]) -> bool {
    input.len() >= 12 && &input[4..12] == b"ftypmif1"
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

// Additional image format detectors from new Go implementation
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

fn mp2(input: &[u8]) -> bool {
    // MP2 (MPEG-1/2 Audio Layer 2) detection
    // Starts with MPEG frame sync pattern 0xFFE or 0xFFF
    // Layer bits should indicate Layer II (01 in bits 17-18)
    if input.len() < 2 {
        return false;
    }

    // Check for MPEG sync word (11 bits set) and Layer II indicator
    let header = u16::from_be_bytes([input[0], input[1]]);
    let sync = (header & 0xFFE0) == 0xFFE0; // Check 11-bit sync word
    let layer = (header & 0x0006) >> 1; // Extract layer bits

    sync && layer == 0x02 // Layer II = 10 binary = 2 decimal
}

// Additional video format detectors
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
    // EBML variable-length integer width is determined by the position of the first set bit
    // Returns (number of leading zeros + 1), clamped to maximum of 8
    let byte = (v & 0xFF) as u8;
    (byte.leading_zeros() as usize + 1).min(8)
}

fn mpeg(input: &[u8]) -> bool {
    input.len() > 3 && input.starts_with(b"\x00\x00\x01") && input[3] >= 0xB0 && input[3] <= 0xBF
}

// Additional archive format detectors
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

    {
        let data = &input[zip_offset..];
        data.starts_with(b"PK\x03\x04")
            || data.starts_with(b"PK\x05\x06")
            || data.starts_with(b"PK\x07\x08")
    }
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
    let expected = b"mimetypeapplication/epub+zip";
    input.len() >= 30 + expected.len() && &input[30..30 + expected.len()] == expected
}

fn jar(input: &[u8]) -> bool {
    executable_jar(input)
        || zip_has(
            input,
            &[(b"META-INF/MANIFEST.MF", false), (b"META-INF/", true)],
            1,
        )
}

fn war(input: &[u8]) -> bool {
    // Web Application Archive - check for WEB-INF directory
    zip_has(input, &[(b"WEB-INF/", true)], 1)
}

fn vsix(input: &[u8]) -> bool {
    // Visual Studio Extension - check for extension.vsixmanifest
    zip_has(input, &[(b"extension.vsixmanifest", false)], 1)
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
/// Note: Parent OLE already validated signature, no need to re-check
fn doc(input: &[u8]) -> bool {
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

    if let Some(actual_clsid) = get_ole_clsid(input) {
        return CLSIDS.contains(&actual_clsid);
    }

    false
}

fn xls(input: &[u8]) -> bool {
    // Try CLSID matching first (primary method from Go implementation)
    // Note: Parent OLE already validated signature
    const EXCEL_V5_CLSID: &[u8] = &[0x10, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
    const EXCEL_V7_CLSID: &[u8] = &[0x20, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00];

    if let Some(actual_clsid) = get_ole_clsid(input) {
        if actual_clsid.starts_with(EXCEL_V5_CLSID) || actual_clsid.starts_with(EXCEL_V7_CLSID) {
            return true;
        }
    }

    let lin = input.len();

    // Check for XLS sub-headers at various offsets (from Go implementation)
    // Note: Workbook stream can start at different offsets depending on file structure
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

        // Check at multiple sector-aligned offsets
        for offset in [512, 1024, 1536, 2048, 2560] {
            if input.len() <= offset {
                break;
            }
            for &header in &XLS_SUB_HEADERS {
                if input.len() > offset + header.len() && input[offset..].starts_with(header) {
                    return true;
                }
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
    // Try CLSID matching first (from Go implementation)
    // Note: Parent OLE already validated signature
    const PPT_V4_CLSID: &[u8; 16] = &[
        0x10, 0x8d, 0x81, 0x64, 0x9b, 0x4f, 0xcf, 0x11, 0x86, 0xea, 0x00, 0xaa, 0x00, 0xb9, 0x29,
        0xe8,
    ];
    const PPT_V7_CLSID: &[u8; 16] = &[
        0x70, 0xae, 0x7b, 0xea, 0x3b, 0xfb, 0xcd, 0x11, 0xa9, 0x03, 0x00, 0xaa, 0x00, 0x51, 0x0e,
        0xa3,
    ];

    if let Some(actual_clsid) = get_ole_clsid(input) {
        if actual_clsid == PPT_V4_CLSID || actual_clsid == PPT_V7_CLSID {
            return true;
        }
    }

    let lin = input.len();
    if lin < 520 {
        return false;
    }

    // Check for PPT sub-headers at offset 512 (from Go implementation)
    const PPT_SUB_HEADERS: [&[u8]; 4] = [
        &[0xA0, 0x46, 0x1D, 0xF0],
        &[0x00, 0x6E, 0x1E, 0xF0],
        &[0x0F, 0x00, 0xE8, 0x03],
        &[0x60, 0x21, 0x1B, 0xF0], // Additional PPT record container
    ];

    for &header in &PPT_SUB_HEADERS {
        if input.len() > 512 + header.len() && input[512..].starts_with(header) {
            return true;
        }
    }

    // Note: Removed overly broad FD FF FF FF pattern check at offset 512
    // as this is a common FAT sector marker present in many OLE files (MSG, MSI, PUB, etc.)
    // and was causing false positives. Relying on CLSID and more specific patterns instead.

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
    get_ole_clsid(input).is_some_and(|actual| actual == PUBLISHER_CLSID)
}

fn msg(input: &[u8]) -> bool {
    const OUTLOOK_MSG_CLSID: &[u8; 16] = &[
        0x0B, 0x0D, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    get_ole_clsid(input).is_some_and(|actual| actual == OUTLOOK_MSG_CLSID)
}

fn pst(input: &[u8]) -> bool {
    // PST (Personal Storage Table) file format detection
    // Magic bytes: "!BDN" at offset 0-3, optional "SM" at offset 8-9
    // Version at offset 10: 0x17 (Unicode), 0x0E or 0x0F (ANSI)
    input.len() > 12 && &input[0..4] == b"!BDN"
}

fn mpp(input: &[u8]) -> bool {
    // Microsoft Project files - check for known CLSIDs
    const MS_PROJECT_CLSID: &[u8; 16] = &[
        0x84, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    get_ole_clsid(input).is_some_and(|actual| actual == MS_PROJECT_CLSID)
}

fn vsd(input: &[u8]) -> bool {
    // Microsoft Visio Drawing - check for known CLSIDs
    const VISIO_DRAWING_CLSID: &[u8; 16] = &[
        0xC1, 0xDB, 0xFE, 0x00, 0x02, 0x1A, 0xCE, 0x11, 0xA3, 0x10, 0x08, 0x00, 0x2B, 0x2C, 0xF9,
        0xAE,
    ];
    get_ole_clsid(input).is_some_and(|actual| actual == VISIO_DRAWING_CLSID)
}

fn onenote(input: &[u8]) -> bool {
    const ONENOTE_CLSID: &[u8; 16] = &[
        0x43, 0xAD, 0x43, 0x36, 0x5E, 0x47, 0x96, 0x48, 0x8B, 0x42, 0x04, 0x40, 0xE7, 0x87, 0xC9,
        0x30,
    ];
    get_ole_clsid(input).is_some_and(|actual| actual == ONENOTE_CLSID)
}

fn msi(input: &[u8]) -> bool {
    const MSI_CLSID: &[u8; 16] = &[
        0x84, 0x10, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    get_ole_clsid(input).is_some_and(|actual| actual == MSI_CLSID)
}

fn msp(input: &[u8]) -> bool {
    const MSP_CLSID: &[u8; 16] = &[
        0x86, 0x10, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    get_ole_clsid(input).is_some_and(|actual| actual == MSP_CLSID)
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

fn odb(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.database")
}

fn odm(input: &[u8]) -> bool {
    detect_opendocument_format(input, b"application/vnd.oasis.opendocument.text-master")
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

fn otm(input: &[u8]) -> bool {
    detect_opendocument_format(
        input,
        b"application/vnd.oasis.opendocument.text-master-template",
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

fn ora(input: &[u8]) -> bool {
    // OpenRaster (layered image format) - check for mimetype "image/openraster"
    detect_opendocument_format(input, b"image/openraster")
}

fn ear(input: &[u8]) -> bool {
    // Enterprise Application Archive - check for META-INF/application.xml
    zip_has(input, &[(b"META-INF/application.xml", false)], 1)
}

fn aab(input: &[u8]) -> bool {
    // Android App Bundle - check for BundleConfig.pb file
    zip_has(input, &[(b"BundleConfig.pb", false)], 1)
}

fn appx(input: &[u8]) -> bool {
    // Windows App Package - check for AppxManifest.xml
    zip_has(input, &[(b"AppxManifest.xml", false)], 1)
}

fn appxbundle(input: &[u8]) -> bool {
    // Windows App Bundle - check for AppxMetadata/AppxBundleManifest.xml
    zip_has(input, &[(b"AppxMetadata/AppxBundleManifest.xml", false)], 1)
}

fn ipa(input: &[u8]) -> bool {
    // iOS App Store Package - check for Payload/ directory
    zip_has(input, &[(b"Payload/", true)], 1)
}

fn xap(input: &[u8]) -> bool {
    // Microsoft Silverlight Application - check for AppManifest.xaml
    zip_has(input, &[(b"AppManifest.xaml", false)], 1)
}

fn xpi(input: &[u8]) -> bool {
    // Mozilla XPInstall (Firefox/Thunderbird extension) - check for install.rdf or manifest.json
    zip_has(
        input,
        &[(b"install.rdf", false), (b"manifest.json", false)],
        1,
    )
}

fn xps(input: &[u8]) -> bool {
    // OpenXPS (XML Paper Specification) - check for _rels/.rels or [Content_Types].xml
    zip_has(
        input,
        &[(b"_rels/.rels", false), (b"[Content_Types].xml", false)],
        1,
    )
}

fn sda(input: &[u8]) -> bool {
    // StarDraw - StarOffice/StarDivision Draw (graphics)
    // Check for StarDraw-specific files in ZIP
    zip_has(
        input,
        &[(b"Draw", true), (b"ObjectReplacements", true)],
        100,
    )
}

fn sdc(input: &[u8]) -> bool {
    // StarCalc - StarOffice/StarDivision Calc (spreadsheet)
    // Check for StarCalc-specific files in ZIP
    zip_has(input, &[(b"Calc", true)], 100)
}

fn sdd(input: &[u8]) -> bool {
    // StarImpress - StarOffice/StarDivision Impress (presentation)
    // Check for StarImpress-specific files in ZIP
    zip_has(input, &[(b"Impress", true)], 100)
}

fn sds(input: &[u8]) -> bool {
    // StarChart - StarOffice/StarDivision Chart
    // Check for StarChart-specific files in ZIP
    zip_has(input, &[(b"Chart", true)], 100)
}

fn sdw(input: &[u8]) -> bool {
    // StarWriter - StarOffice/StarDivision Writer (word processor)
    // Check for StarWriter-specific files in ZIP
    zip_has(input, &[(b"Writer", true)], 100)
}

fn smf(input: &[u8]) -> bool {
    // StarMath - StarOffice/StarDivision Math (mathematical formulas)
    // Check for StarMath-specific files in ZIP
    zip_has(input, &[(b"Math", true)], 100)
}

fn sxd(input: &[u8]) -> bool {
    // Sun XML Draw - Legacy Sun Microsystems graphics format
    detect_opendocument_format(input, b"application/vnd.sun.xml.draw")
}

fn sxi(input: &[u8]) -> bool {
    // Sun XML Impress - Legacy Sun Microsystems presentation format
    detect_opendocument_format(input, b"application/vnd.sun.xml.impress")
}

fn sxm(input: &[u8]) -> bool {
    // Sun XML Math - Legacy Sun Microsystems mathematical formula format
    detect_opendocument_format(input, b"application/vnd.sun.xml.math")
}

fn sxw(input: &[u8]) -> bool {
    // Sun XML Writer - Legacy Sun Microsystems word processor format
    detect_opendocument_format(input, b"application/vnd.sun.xml.writer")
}

fn stc(input: &[u8]) -> bool {
    // Sun XML Calc Template - Legacy Sun Microsystems spreadsheet template
    detect_opendocument_format(input, b"application/vnd.sun.xml.calc.template")
}

fn std(input: &[u8]) -> bool {
    // Sun XML Draw Template - Legacy Sun Microsystems graphics template
    detect_opendocument_format(input, b"application/vnd.sun.xml.draw.template")
}

fn sti(input: &[u8]) -> bool {
    // Sun XML Impress Template - Legacy Sun Microsystems presentation template
    detect_opendocument_format(input, b"application/vnd.sun.xml.impress.template")
}

fn stw(input: &[u8]) -> bool {
    // Sun XML Writer Template - Legacy Sun Microsystems word processor template
    detect_opendocument_format(input, b"application/vnd.sun.xml.writer.template")
}

fn sgw(input: &[u8]) -> bool {
    // Sun XML Writer Global - Legacy Sun Microsystems master document format
    detect_opendocument_format(input, b"application/vnd.sun.xml.writer.global")
}

fn wpg(_input: &[u8]) -> bool {
    // WordPerfect Graphics - WordPerfect graphics format
    // Parent WPD already verified signature, rely on extension
    false
}

fn shw(_input: &[u8]) -> bool {
    // WordPerfect Presentations - WordPerfect presentation format
    // Parent WPD already verified signature, rely on extension
    false
}

fn wpm(_input: &[u8]) -> bool {
    // WordPerfect Macro - WordPerfect macro format
    // Parent WPD already verified signature, rely on extension
    false
}

fn uop(input: &[u8]) -> bool {
    // Uniform Office Format Presentation - Chinese office format
    // UOF files are ZIP-based with XML content, check for UOF namespace
    let s = String::from_utf8_lossy(input);
    s.contains("uof:UOF") && s.contains("演示") // "演示" = presentation in Chinese
}

fn uos(input: &[u8]) -> bool {
    // Uniform Office Format Spreadsheet - Chinese office format
    // UOF files are ZIP-based with XML content, check for UOF namespace
    let s = String::from_utf8_lossy(input);
    s.contains("uof:UOF") && s.contains("电子表格") // "电子表格" = spreadsheet in Chinese
}

fn uot(input: &[u8]) -> bool {
    // Uniform Office Format Text - Chinese office format
    // UOF files are ZIP-based with XML content, check for UOF namespace
    let s = String::from_utf8_lossy(input);
    s.contains("uof:UOF") && s.contains("文字处理") // "文字处理" = word processing in Chinese
}

fn usdz(input: &[u8]) -> bool {
    // Universal Scene Description ZIP - Pixar's USD format in ZIP container
    // USDZ files contain .usda or .usdc files, look for USD-specific content
    let s = String::from_utf8_lossy(input);
    s.contains(".usda") || s.contains(".usdc") || s.contains("#usda")
}

fn sketch(input: &[u8]) -> bool {
    // Sketch - Design tool by Bohemian Coding
    // Sketch 43+ files contain document.json or meta.json with _class identifiers
    let s = String::from_utf8_lossy(input);
    (s.contains("document.json") || s.contains("meta.json")) && s.contains("\"_class\"")
}

fn sldasm(input: &[u8]) -> bool {
    // SolidWorks Assembly - OLE-based CAD file
    // Contains "SolidWorks" string and assembly-specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("SolidWorks") && (s.contains("Assembly") || s.contains("SLDASM"))
}

fn slddrw(input: &[u8]) -> bool {
    // SolidWorks Drawing - OLE-based CAD file
    // Contains "SolidWorks" string and drawing-specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("SolidWorks") && (s.contains("Drawing") || s.contains("SLDDRW"))
}

fn sldprt(input: &[u8]) -> bool {
    // SolidWorks Part - OLE-based CAD file
    // Contains "SolidWorks" string and part-specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("SolidWorks") && (s.contains("Part") || s.contains("SLDPRT"))
}

fn iam(input: &[u8]) -> bool {
    // Autodesk Inventor Assembly - OLE-based CAD file
    // Contains "Inventor" string and assembly-specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("Inventor") && (s.contains("Assembly") || s.contains(".iam"))
}

fn idw(input: &[u8]) -> bool {
    // Autodesk Inventor Drawing - OLE-based CAD file
    // Contains "Inventor" string and drawing-specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("Inventor") && (s.contains("Drawing") || s.contains(".idw"))
}

fn ipn(input: &[u8]) -> bool {
    // Autodesk Inventor Presentation - OLE-based CAD file
    // Contains "Inventor" string and presentation-specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("Inventor") && (s.contains("Presentation") || s.contains(".ipn"))
}

fn ipt(input: &[u8]) -> bool {
    // Autodesk Inventor Part - OLE-based CAD file
    // Contains "Inventor" string and part-specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("Inventor") && (s.contains("Part") || s.contains(".ipt"))
}

fn scdoc(input: &[u8]) -> bool {
    // SpaceClaim Document - OLE-based CAD file
    // Contains "SpaceClaim" string or specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("SpaceClaim") || s.contains("scdoc")
}

fn autodesk_max(input: &[u8]) -> bool {
    // Autodesk 3D Studio Max - OLE-based project file
    // Contains "3dsmax" or "3D Studio Max" strings in metadata
    let s = String::from_utf8_lossy(input);
    s.contains("3dsmax") || s.contains("3D Studio Max") || s.contains(".max")
}

fn autodesk_123d(input: &[u8]) -> bool {
    // Autodesk 123D - ZIP-based 3D modeling format
    // Contains specific 123D project files or metadata
    let s = String::from_utf8_lossy(input);
    s.contains("123D") || s.contains("Autodesk.123D")
}

fn fusion_360(input: &[u8]) -> bool {
    // Fusion 360 - ZIP-based CAD format
    // Contains Fusion 360 specific metadata
    let s = String::from_utf8_lossy(input);
    s.contains("Fusion360") || s.contains("fusion360") || s.contains("Autodesk Fusion")
}

fn drawio(input: &[u8]) -> bool {
    // draw.io - XML-based diagramming format
    // Contains mxfile or mxGraphModel elements
    let s = String::from_utf8_lossy(input);
    s.contains("<mxfile") || s.contains("<mxGraphModel")
}

fn xspf(input: &[u8]) -> bool {
    // XSPF - XML Shareable Playlist Format
    // Contains playlist element with XSPF namespace
    let s = String::from_utf8_lossy(input);
    s.contains("<playlist") && s.contains("xspf")
}

fn xsl(input: &[u8]) -> bool {
    // XSLT - Extensible Stylesheet Language Transformations
    // Contains stylesheet element with XSLT namespace
    let s = String::from_utf8_lossy(input);
    (s.contains("<xsl:stylesheet") || s.contains("<xsl:transform"))
        && s.contains("http://www.w3.org/1999/XSL/Transform")
}

fn figma(input: &[u8]) -> bool {
    // Figma - ZIP-based design format
    // Contains Figma-specific metadata or canvas data
    let s = String::from_utf8_lossy(input);
    s.contains("figma") || s.contains("\"document\":{\"id\"") || s.contains("\"canvas\"")
}

fn mathml(input: &[u8]) -> bool {
    // MathML - Mathematical Markup Language
    // Contains math or MathML elements with MathML namespace
    let s = String::from_utf8_lossy(input);
    (s.contains("<math") || s.contains("<MathML"))
        && s.contains("http://www.w3.org/1998/Math/MathML")
}

fn musicxml(input: &[u8]) -> bool {
    // MusicXML - Music notation format
    // Contains score-partwise or score-timewise root elements
    let s = String::from_utf8_lossy(input);
    s.contains("<score-partwise") || s.contains("<score-timewise")
}

fn ttml(input: &[u8]) -> bool {
    // TTML - Timed Text Markup Language
    // Contains tt element with TTML namespace
    let s = String::from_utf8_lossy(input);
    s.contains("<tt ") && s.contains("http://www.w3.org/ns/ttml")
}

fn soap(input: &[u8]) -> bool {
    // SOAP - Simple Object Access Protocol
    // Contains Envelope element with SOAP namespace
    let s = String::from_utf8_lossy(input);
    (s.contains("<Envelope") || s.contains("<soap:Envelope") || s.contains("<SOAP-ENV:Envelope"))
        && (s.contains("http://schemas.xmlsoap.org/soap/envelope")
            || s.contains("http://www.w3.org/2003/05/soap-envelope"))
}

fn tmx(input: &[u8]) -> bool {
    // TMX - Tiled Map XML
    // Game development map format, contains <map> element
    let s = String::from_utf8_lossy(input);
    s.contains("<map ") && (s.contains("version=") || s.contains("orientation="))
}

fn tsx(input: &[u8]) -> bool {
    // TSX - Tiled Tileset XML
    // Game development tileset format, contains <tileset> element
    let s = String::from_utf8_lossy(input);
    s.contains("<tileset ") && (s.contains("version=") || s.contains("tilewidth="))
}

fn mpd(input: &[u8]) -> bool {
    // MPD - MPEG-DASH Media Presentation Description
    // Streaming manifest, contains <MPD> element with DASH namespace
    let s = String::from_utf8_lossy(input);
    s.contains("<MPD ") && s.contains("urn:mpeg:dash:schema:mpd:")
}

fn mxl(input: &[u8]) -> bool {
    // MXL - MusicXML ZIP
    // Compressed MusicXML format (ZIP-based)
    // Contains .musicxml or META-INF/container.xml files
    let s = String::from_utf8_lossy(input);
    s.contains(".musicxml") || (s.contains("META-INF") && s.contains("container.xml"))
}

fn cddx(input: &[u8]) -> bool {
    // CDDX - Circuit Diagram Document
    // Electronic circuit diagram format (XML)
    let s = String::from_utf8_lossy(input);
    s.contains("<circuit") || (s.contains("<CircuitDocument") && s.contains("circuitdiagram"))
}

fn dwfx(input: &[u8]) -> bool {
    // DWFX - Design Web Format XPS
    // Autodesk CAD exchange format (XML/XPS based)
    let s = String::from_utf8_lossy(input);
    s.contains("<DWFDocument") || (s.contains("dwf") && s.contains(".dwfx"))
}

fn fbz(input: &[u8]) -> bool {
    // FBZ - FictionBook ZIP
    // Compressed FictionBook e-book (ZIP-based, contains .fb2 files)
    let s = String::from_utf8_lossy(input);
    s.contains(".fb2")
        || (s.contains("FictionBook") && s.contains("http://www.gribuser.ru/xml/fictionbook"))
}

fn asx(input: &[u8]) -> bool {
    // ASX (Advanced Stream Redirector) - XML playlist for Windows Media
    // Contains <asx version= or <ASX version=
    let s = String::from_utf8_lossy(input);
    s.contains("<asx ") || s.contains("<ASX ")
}

fn wma(_input: &[u8]) -> bool {
    // Windows Media Audio - ASF-based, parent already verified signature
    // Rely on extension for distinction from video variants
    false
}

fn wmv(_input: &[u8]) -> bool {
    // Windows Media Video - ASF-based, parent already verified signature
    // Rely on extension for distinction from audio variants
    false
}

fn air(input: &[u8]) -> bool {
    // Adobe AIR - check for META-INF/AIR/application.xml
    zip_has(input, &[(b"META-INF/AIR/application.xml", false)], 1)
}

fn fla(input: &[u8]) -> bool {
    // Adobe Flash Project (CS5+) - ZIP-based format
    // Check for common FLA files: DOMDocument.xml or PublishSettings.xml
    zip_has(
        input,
        &[(b"DOMDocument.xml", false), (b"PublishSettings.xml", false)],
        1,
    )
}

fn idml(input: &[u8]) -> bool {
    // InDesign Markup Language - ZIP-based format
    // Check for designmap.xml or mimetype file
    zip_has(input, &[(b"designmap.xml", false), (b"mimetype", false)], 1)
}

fn ai(input: &[u8]) -> bool {
    // Adobe Illustrator - PDF-based format
    // AI files are PDF files with additional Adobe-specific metadata
    // Check for %AI or Adobe_Illustrator markers in the file
    let s = String::from_utf8_lossy(input);
    s.contains("%AI") || s.contains("Adobe_Illustrator") || s.contains("Adobe Illustrator")
}

fn dvr_ms(_input: &[u8]) -> bool {
    // Microsoft Digital Video Recording - ASF-based format
    // DVR-MS files are ASF files, so any ASF file could be DVR-MS
    // We can check for specific DVR-MS metadata or just return false to use parent ASF
    // For now, return false to keep it as generic ASF unless we find specific markers
    false
}

fn abw(input: &[u8]) -> bool {
    // AbiWord - gzip-compressed XML document
    // After decompressing gzip, should contain <?xml and <abiword
    // We can check if after gzip header there are typical XML patterns
    if input.len() < 20 {
        return false;
    }
    // For now, we'll check for common patterns after gzip decompression
    // This is a simplified check - a full implementation would decompress
    let s = String::from_utf8_lossy(input);
    s.contains("abiword") || s.contains("AbiWord")
}

// ============================================================================
// DATABASE FORMAT DETECTORS
// ============================================================================

fn mdb(input: &[u8]) -> bool {
    input.len() >= 32 && &input[4..19] == b"Standard Jet DB"
}

fn accdb(input: &[u8]) -> bool {
    input.len() >= 32 && &input[4..19] == b"Standard ACE DB"
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
    // Match all Lotus 1-2-3 versions by checking common 0x00 0x00 prefix
    // Children (WK1, WK3, WK4) will refine based on version field at offset 4-7
    if !input.starts_with(b"\x00\x00") {
        return false;
    }

    let version = u32::from_le_bytes([input[4], input[5], input[6], input[7]]);
    matches!(
        version,
        0x00060406 |  // v1 (WK1)
        0x00000200 |  // v2
        0x00001a00 |  // v2 variant
        0x00041000 |  // v3 (WK3)
        0x00041002 // v4/v5 (WK4)
    )
}

fn marc(input: &[u8]) -> bool {
    // MARC leader validation
    input.len() >= 24 && input[10] == b'2' && input[11] == b'2' && &input[20..24] == b"4500"
}

// ============================================================================
// COMMON LANGUAGE DETECTION UTILITIES
// ============================================================================

/// Pattern definition for language detection
#[derive(Debug, Clone)]
#[doc(hidden)]
struct LangPattern {
    bytes: &'static [u8],
    weight: u8,
}

impl LangPattern {
    const fn new(bytes: &'static [u8], weight: u8) -> Self {
        LangPattern { bytes, weight }
    }

    const fn simple(bytes: &'static [u8]) -> Self {
        LangPattern { bytes, weight: 1 }
    }
}

/// Single-pass pattern matcher for language detection
#[doc(hidden)]
struct SinglePassMatcher<'a> {
    sample: &'a [u8],
    patterns: &'a [LangPattern],
    found: Vec<bool>,
}

impl<'a> SinglePassMatcher<'a> {
    fn new(sample: &'a [u8], patterns: &'a [LangPattern]) -> Self {
        let found = vec![false; patterns.len()];
        SinglePassMatcher {
            sample,
            patterns,
            found,
        }
    }

    /// Perform single-pass matching of all patterns and return score
    fn scan(mut self) -> (Vec<bool>, u8) {
        let mut i = 0;

        'outer: while i < self.sample.len() {
            // Check each pattern at current position
            for (idx, pattern) in self.patterns.iter().enumerate() {
                if !self.found[idx] && self.matches_at(i, pattern.bytes) {
                    self.found[idx] = true;
                    i += pattern.bytes.len();
                    continue 'outer;
                }
            }

            // No pattern matched, advance by 1
            i += 1;
        }

        // Calculate total score
        let score = self
            .patterns
            .iter()
            .enumerate()
            .map(|(idx, p)| if self.found[idx] { p.weight } else { 0 })
            .sum();

        (self.found, score)
    }

    /// Scan with early stop when threshold is exceeded
    /// Returns true if threshold was exceeded (meaning antipatterns detected)
    fn scan_early_stop(mut self, threshold: u8) -> bool {
        let mut score = 0u8;
        let mut i = 0;

        'outer: while i < self.sample.len() {
            // Check each pattern at current position
            for (idx, pattern) in self.patterns.iter().enumerate() {
                if !self.found[idx] && self.matches_at(i, pattern.bytes) {
                    self.found[idx] = true;
                    score = score.saturating_add(pattern.weight);

                    // Early return if threshold exceeded
                    if score > threshold {
                        return true;
                    }

                    i += pattern.bytes.len();
                    continue 'outer;
                }
            }

            // No pattern matched, advance by 1
            i += 1;
        }

        false
    }

    #[inline]
    fn matches_at(&self, pos: usize, pattern: &[u8]) -> bool {
        pos + pattern.len() <= self.sample.len()
            && &self.sample[pos..pos + pattern.len()] == pattern
    }
}

/// Check for common shebangs (checks if shebang line contains any of the patterns)
#[inline]
fn has_lang_shebang(input: &[u8], shebangs: &[&[u8]]) -> bool {
    if !input.starts_with(b"#!") {
        return false;
    }

    // Find end of shebang line
    let end = input
        .iter()
        .position(|&b| b == b'\n')
        .unwrap_or(input.len())
        .min(256);
    let shebang_line = &input[..end];

    // Check if any pattern appears in the shebang line
    shebangs
        .iter()
        .any(|&pattern| shebang_line.windows(pattern.len()).any(|w| w == pattern))
}

// ============================================================================
// PROGRAMMING & TEXT FORMAT DETECTORS
// ============================================================================

fn javascript(input: &[u8]) -> bool {
    // Check for shebang first
    const NODE_SHEBANGS: &[&[u8]] = &[b"#!/usr/bin/env node", b"#!/usr/bin/node"];

    if has_lang_shebang(input, NODE_SHEBANGS) {
        return true;
    }

    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (indicates NOT JavaScript)
    const ANTI_JS: &[LangPattern] = &[
        LangPattern::simple(b"#include"), // C/C++
        LangPattern::simple(b"package "), // Java/Go
        LangPattern::simple(b"using "),   // C#
        LangPattern::simple(b"fn "),      // Rust
    ];

    // Check anti-patterns FIRST - early stop on first match
    if SinglePassMatcher::new(sample, ANTI_JS).scan_early_stop(0) {
        return false;
    }

    // JavaScript requires braces for code blocks
    let has_braces = sample.contains(&b'{') && sample.contains(&b'}');
    if !has_braces {
        return false;
    }

    // JavaScript patterns with weights
    const JS_PATTERNS: &[LangPattern] = &[
        LangPattern::new(b"=>", 2), // arrow function - strong
        LangPattern::new(b"const ", 1),
        LangPattern::new(b"let ", 1),
        LangPattern::new(b"function ", 1),
        LangPattern::new(b"var ", 1),
        LangPattern::new(b"export ", 2),  // export - strong
        LangPattern::new(b"require(", 2), // require - strong
        LangPattern::new(b"console.", 1),
        LangPattern::new(b"return ", 1),
    ];

    // Check for TypeScript false positive (interface with type annotation)
    let mut has_interface_ts = false;
    let mut has_colon_space = false;
    for i in 0..sample.len() {
        if i + 10 <= sample.len() && &sample[i..i + 10] == b"interface " {
            has_interface_ts = true;
        }
        if i + 2 <= sample.len() && &sample[i..i + 2] == b": " {
            has_colon_space = true;
        }
        if has_interface_ts && has_colon_space {
            return false;
        }
    }

    let js_matcher = SinglePassMatcher::new(sample, JS_PATTERNS);
    let (found, score) = js_matcher.scan();

    // Has strong indicator (arrow, export, require) OR multiple weaker ones
    let has_strong = found[0] || found[5] || found[6];
    let has_function_return = found[3] && found[8];

    has_strong || has_function_return || score >= 2
}

fn java(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (JavaScript/TypeScript/C# false positives) - check FIRST
    let anti_patterns = [
        LangPattern::new(b"=>", 10),           // JavaScript arrow function
        LangPattern::new(b"using System", 10), // C#
        LangPattern::new(b"export ", 5),       // TypeScript/JavaScript
        LangPattern::new(b"type ", 5),         // TypeScript
        LangPattern::new(b"namespace ", 5),    // C#/C++
        LangPattern::new(b"function ", 5),     // JavaScript
        LangPattern::new(b"async ", 5),        // JavaScript
        LangPattern::new(b"await ", 5),        // JavaScript
        LangPattern::new(b"{ get", 3),         // C# property
        LangPattern::new(b"{ set", 3),         // C# property
        LangPattern::new(b"const ", 2),        // JavaScript
        LangPattern::new(b"let ", 2),          // JavaScript
        LangPattern::simple(b"var "),          // JavaScript (but also Java)
    ];

    // Check antipatterns FIRST - early stop if exceed threshold of 2
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(2) {
        return false;
    }

    // Java requires braces for code blocks
    let has_braces = sample.contains(&b'{') && sample.contains(&b'}');
    if !has_braces {
        return false;
    }

    // Java patterns with weights
    let patterns = [
        LangPattern::new(b"public static void main", 4),
        LangPattern::new(b"public class ", 3),
        LangPattern::new(b"public interface ", 3),
        LangPattern::new(b"public abstract ", 3),
        LangPattern::new(b"public enum ", 3),
        LangPattern::new(b"package ", 3),
        LangPattern::new(b"import java.", 3),
        LangPattern::new(b"import javax.", 3),
        LangPattern::new(b"@Override", 3),
        LangPattern::new(b"@Autowired", 3),
        LangPattern::new(b"@Component", 3),
        LangPattern::new(b"@Service", 3),
        LangPattern::new(b"@Repository", 3),
        LangPattern::new(b"@RestController", 3),
        LangPattern::new(b"@RequestMapping", 3),
        LangPattern::new(b"System.out.", 3),
        LangPattern::new(b"private class ", 2),
        LangPattern::new(b"protected class ", 2),
        LangPattern::new(b"import com.", 2),
        LangPattern::new(b"import org.", 2),
        LangPattern::new(b"extends ", 2),
        LangPattern::new(b"implements ", 2),
        LangPattern::new(b"throws ", 2),
        LangPattern::new(b"catch (", 2),
        LangPattern::simple(b"import "),
        LangPattern::simple(b"class "),
        LangPattern::simple(b"interface "),
        LangPattern::simple(b"final "),
        LangPattern::simple(b"try {"),
        LangPattern::simple(b"finally {"),
    ];

    SinglePassMatcher::new(sample, &patterns).scan().1 >= 3
}

fn typescript(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (Java, C++, C# false positives) - check FIRST
    let anti_patterns = [
        LangPattern::new(b"using System", 10),  // C#
        LangPattern::new(b"{ get; set; }", 10), // C# property
        LangPattern::new(b"Task<", 8),          // C# async
        LangPattern::new(b"public class ", 10),
        LangPattern::new(b"package ", 10),
        LangPattern::new(b"import java.", 10),
        LangPattern::new(b"import javax.", 10),
        LangPattern::new(b"System.out", 10),
        LangPattern::new(b"std::", 10),
        LangPattern::new(b"iostream", 10),
        LangPattern::new(b"#include", 10),
        LangPattern::new(b"template<", 10),
        LangPattern::new(b"extern \"C\"", 10),
        LangPattern::new(b"cout", 8),
        LangPattern::new(b"cin", 8),
        LangPattern::new(b"public:", 8),
        LangPattern::new(b"private:", 8),
        LangPattern::new(b"protected:", 8),
        LangPattern::new(b"@Override", 5),
        LangPattern::new(b"import com.", 5),
        LangPattern::new(b"import org.", 5),
    ];

    // Check antipatterns FIRST - early stop if exceed threshold of 5
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(5) {
        return false;
    }

    // TypeScript requires braces for code blocks
    let has_braces = sample.contains(&b'{') && sample.contains(&b'}');
    if !has_braces {
        return false;
    }

    // Check for custom type annotations like ": Config", ": User", etc.
    let has_custom_type_annotation = sample
        .windows(3)
        .any(|w| w[0] == b':' && w[1] == b' ' && w[2].is_ascii_uppercase());

    // TypeScript patterns with weights
    let patterns = [
        LangPattern::new(b"type ", 3), // TypeScript-specific
        LangPattern::new(b"enum ", 3),
        LangPattern::new(b"readonly ", 3), // TypeScript-specific
        LangPattern::new(b"namespace ", 3),
        LangPattern::new(b"declare ", 3),  // TypeScript-specific
        LangPattern::new(b"keyof ", 3),    // TypeScript-specific
        LangPattern::new(b"Record<", 3),   // TypeScript utility type
        LangPattern::new(b"Partial<", 3),  // TypeScript utility type
        LangPattern::new(b"Required<", 3), // TypeScript utility type
        LangPattern::new(b"Pick<", 3),     // TypeScript utility type
        LangPattern::new(b"Omit<", 3),     // TypeScript utility type
        LangPattern::new(b"interface ", 2),
        LangPattern::new(b" as ", 2), // Type assertion
        LangPattern::new(b": string", 2),
        LangPattern::new(b"): string", 2),
        LangPattern::new(b": number", 2),
        LangPattern::new(b"): number", 2),
        LangPattern::new(b": boolean", 2),
        LangPattern::new(b"): boolean", 2),
        LangPattern::new(b": void", 2),
        LangPattern::new(b"): void", 2),
        LangPattern::new(b": any", 2),
        LangPattern::new(b"): any", 2),
        LangPattern::new(b": string[]", 2),
        LangPattern::new(b"): string[]", 2),
        LangPattern::new(b": number[]", 2),
        LangPattern::new(b"): number[]", 2),
        LangPattern::new(b"<string>", 2),
        LangPattern::new(b"<number>", 2),
        LangPattern::new(b"<any>", 2),
        LangPattern::new(b"<T>", 2),
        LangPattern::new(b": {", 2), // Object type annotation
        LangPattern::new(b"implements ", 2),
        LangPattern::new(b"abstract ", 2),
        LangPattern::new(b"typeof ", 2),
        LangPattern::new(b"never", 2),
        LangPattern::new(b"unknown", 2),
        LangPattern::simple(b"export "),
        LangPattern::simple(b"import "),
        LangPattern::simple(b"from "),
        LangPattern::simple(b"const "),
        LangPattern::simple(b"function "),
        LangPattern::simple(b"extends "),
        LangPattern::simple(b"private "),
        LangPattern::simple(b"public "),
        LangPattern::simple(b"protected "),
        LangPattern::simple(b"async "),
        LangPattern::simple(b"await "),
    ];

    let score = SinglePassMatcher::new(sample, &patterns).scan().1;

    has_custom_type_annotation || score >= 3
}

fn c_lang(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // Avoid Python/Ruby false positives (they are detected earlier in tree order)
    let has_python_shebang = has_lang_shebang(input, &[b"python"]);
    let has_ruby_shebang = has_lang_shebang(input, &[b"ruby"]);
    let has_def = sample.windows(4).any(|w| w == b"def ");
    let has_end =
        sample.windows(4).any(|w| w == b"end\n") || sample.windows(4).any(|w| w == b"end ");
    let has_print = sample.windows(6).any(|w| w == b"print(");
    let has_import = sample.windows(7).any(|w| w == b"import ");

    // Python: def with print/import, or has both def and print
    // Ruby: def with end keyword
    if has_python_shebang
        || has_ruby_shebang
        || (has_def && has_end)
        || (has_def && (has_print || has_import))
    {
        return false;
    }

    // C-specific patterns (also valid in C++)
    let has_include = sample.windows(8).any(|w| w == b"#include");
    let has_define = sample.windows(7).any(|w| w == b"#define");
    let has_ifdef = sample.windows(6).any(|w| w == b"#ifdef");
    let has_ifndef = sample.windows(7).any(|w| w == b"#ifndef");
    let has_endif = sample.windows(6).any(|w| w == b"#endif");
    let has_typedef = sample.windows(8).any(|w| w == b"typedef ");
    let has_struct = sample.windows(7).any(|w| w == b"struct ");
    let has_main = sample.windows(9).any(|w| w == b"int main(");
    let has_void = sample.windows(5).any(|w| w == b"void ");
    let has_printf = sample.windows(7).any(|w| w == b"printf(");
    let has_malloc = sample.windows(7).any(|w| w == b"malloc(");
    let has_sizeof = sample.windows(7).any(|w| w == b"sizeof(");
    let has_return = sample.windows(7).any(|w| w == b"return ");

    // Also match on C/C++ common patterns to ensure C++ child gets checked
    let has_class = sample.windows(6).any(|w| w == b"class ");
    let has_public = sample.windows(7).any(|w| w == b"public:");
    let has_private = sample.windows(8).any(|w| w == b"private:");
    let has_protected = sample.windows(10).any(|w| w == b"protected:");
    let has_int = sample.windows(4).any(|w| w == b"int ");
    let has_char = sample.windows(5).any(|w| w == b"char ");
    let has_float = sample.windows(6).any(|w| w == b"float ");
    let has_double = sample.windows(7).any(|w| w == b"double ");

    // Strong C indicators - preprocessor directives are very C/C++ specific
    let has_preprocessor = has_include || has_define || has_ifdef || has_ifndef || has_endif;

    // C/C++ requires braces for code blocks
    let has_braces = sample.contains(&b'{') && sample.contains(&b'}');
    if !has_braces {
        return false;
    }

    // Calculate confidence with weighted scoring
    let c_score = (has_include as u8) * 2  // #include is strong indicator
        + (has_define as u8)
        + (has_ifdef as u8)
        + (has_ifndef as u8)
        + (has_endif as u8)
        + (has_typedef as u8)
        + (has_struct as u8)
        + (has_main as u8) * 2  // main() is strong indicator
        + (has_void as u8)
        + (has_printf as u8)
        + (has_malloc as u8)
        + (has_sizeof as u8)
        + (has_return as u8)
        + (has_class as u8)
        + (has_public as u8) * 2    // C++ access specifiers boost score
        + (has_private as u8) * 2
        + (has_protected as u8) * 2
        + (has_int as u8)
        + (has_char as u8)
        + (has_float as u8)
        + (has_double as u8);

    // Require either preprocessor directives OR multiple C indicators
    // This is still permissive enough for C++ child to override
    (has_preprocessor && c_score >= 2) || c_score >= 3
}

fn cpp(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (Python, C#, Java, Go false positives)
    let anti_patterns = [
        LangPattern::new(b"import ", 10),       // Python, Java
        LangPattern::new(b"from ", 10),         // Python
        LangPattern::new(b"def __init__", 10),  // Python
        LangPattern::new(b"using System", 10),  // C#
        LangPattern::new(b"package ", 10),      // Java, Go
        LangPattern::new(b"@Override", 10),     // Java
        LangPattern::new(b"public class ", 10), // Java, C#
    ];

    // Check antipatterns FIRST - early stop if exceeds threshold
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(9) {
        return false;
    }

    // C++ patterns with weights
    let patterns = [
        // Strong C++ indicators (weight 10 = alone is enough)
        LangPattern::new(b"iostream", 10),
        LangPattern::new(b"namespace ", 10),
        LangPattern::new(b"std::", 10),
        LangPattern::new(b"template<", 10),
        LangPattern::new(b"extern \"C\"", 10),
        // Access specifiers
        LangPattern::new(b"public:", 5),
        LangPattern::new(b"private:", 5),
        LangPattern::new(b"protected:", 5),
        // Other C++ patterns
        LangPattern::new(b"class ", 2),
        LangPattern::new(b"vector", 2),
        LangPattern::new(b"string", 2),
        LangPattern::new(b"cout", 2),
        LangPattern::new(b"cin", 2),
    ];

    let (found, score) = SinglePassMatcher::new(sample, &patterns).scan();

    // Strong indicators - any one is enough
    let has_strong_indicator = found[0]  // iostream
        || found[1]  // namespace
        || found[2]  // std::
        || found[3]  // template<
        || found[4]; // extern "C"

    // class with access specifiers is also strong
    let has_class = found[8];
    let has_access = found[5] || found[6] || found[7]; // public/private/protected
    let has_class_with_access = has_class && has_access;

    if has_strong_indicator || has_class_with_access {
        return true;
    }

    // Require at least 2 weaker patterns (score >= 4, since each weak pattern has weight 2)
    score >= 4
}

fn go_lang(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (Java, C# false positives) - check FIRST
    let anti_patterns = [
        LangPattern::new(b"public class ", 10),
        LangPattern::new(b"private class ", 10),
        LangPattern::new(b"protected class ", 10),
        LangPattern::new(b"@Override", 10),
        LangPattern::new(b"System.out", 10),
        LangPattern::new(b"using System", 10),  // C#
        LangPattern::new(b"{ get; set; }", 10), // C#
        LangPattern::new(b"class ", 5),
        LangPattern::new(b"extends ", 5),
        LangPattern::new(b"implements ", 5),
    ];

    // Check antipatterns FIRST - early stop on first Java/C# antipattern found
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(0) {
        return false;
    }

    // Go requires braces for code blocks
    let has_braces = sample.contains(&b'{') && sample.contains(&b'}');
    if !has_braces {
        return false;
    }

    // Go patterns with weights
    let patterns = [
        LangPattern::new(b" := ", 3),       // Go-specific short declaration
        LangPattern::new(b"defer ", 3),     // Go-specific
        LangPattern::new(b"go ", 3),        // goroutine
        LangPattern::new(b"chan ", 3),      // channel
        LangPattern::new(b"select ", 3),    // select statement
        LangPattern::new(b"err != nil", 3), // Go error handling idiom
        LangPattern::new(b"func main()", 3),
        LangPattern::new(b"recover()", 3),
        LangPattern::new(b"package ", 2),
        LangPattern::new(b"func ", 2),
        LangPattern::new(b"import (", 2),
        LangPattern::new(b"import \"", 2),
        LangPattern::new(b"fmt.", 2),
        LangPattern::new(b"struct {", 2),
        LangPattern::new(b"interface {", 2),
        LangPattern::new(b"interface{}", 2), // empty interface
        LangPattern::new(b"range ", 2),
        LangPattern::new(b"make(", 2),
        LangPattern::new(b"append(", 2),
        LangPattern::new(b"if err", 2),
        LangPattern::new(b"return err", 2),
        LangPattern::new(b"panic(", 2),
        LangPattern::new(b"context.", 2),
        LangPattern::new(b"http.", 2),
        LangPattern::new(b"func (", 2), // method receiver
        LangPattern::simple(b"type "),
        LangPattern::simple(b"len("),
        LangPattern::simple(b"nil"),
    ];

    SinglePassMatcher::new(sample, &patterns).scan().1 >= 3
}

fn rust_lang(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // Rust requires braces for code blocks
    let has_braces = sample.contains(&b'{') && sample.contains(&b'}');
    if !has_braces {
        return false;
    }

    // Check for macro calls (Rust-specific: name!(...))
    let has_macro_call = sample.windows(3).any(|w| {
        (w[0].is_ascii_alphanumeric() || w[0] == b'_')
            && w[1] == b'!'
            && (w[2] == b'(' || w[2] == b'[' || w[2] == b'{')
    });

    // Rust patterns with weights
    let patterns = [
        LangPattern::new(b"let mut ", 3),      // Rust-specific
        LangPattern::new(b"crate::", 3),       // Rust-specific
        LangPattern::new(b"#[derive", 3),      // Rust-specific
        LangPattern::new(b"&self", 3),         // Rust-specific
        LangPattern::new(b"Self::", 3),        // Rust-specific
        LangPattern::new(b"'static", 3),       // Rust lifetime
        LangPattern::new(b"unsafe ", 3),       // Rust-specific
        LangPattern::new(b"extern crate ", 3), // Rust-specific
        LangPattern::new(b"println!(", 3),     // Rust macro
        LangPattern::new(b"vec![", 3),         // Rust macro
        LangPattern::new(b"format!(", 3),      // Rust macro
        LangPattern::new(b"panic!(", 3),       // Rust macro
        LangPattern::new(b"#[test]", 3),
        LangPattern::new(b"#[cfg(", 3),
        LangPattern::new(b"async fn", 3),
        LangPattern::new(b".await", 3),
        LangPattern::new(b"fn ", 2),
        LangPattern::new(b"use ", 2),
        LangPattern::new(b"mod ", 2),
        LangPattern::new(b"impl ", 2),
        LangPattern::new(b"trait ", 2),
        LangPattern::new(b"match ", 2),
        LangPattern::new(b"Some(", 2),
        LangPattern::new(b"Ok(", 2),
        LangPattern::new(b"Err(", 2),
        LangPattern::new(b"Vec<", 2),
        LangPattern::new(b"Box<", 2),
        LangPattern::new(b"Option<", 2),
        LangPattern::new(b"Result<", 2),
        LangPattern::new(b"&mut ", 2),
        LangPattern::new(b"self.", 2),
        LangPattern::new(b"unwrap()", 2),
        LangPattern::new(b"expect(", 2),
        LangPattern::simple(b"pub "),
        LangPattern::simple(b"None"),
    ];

    let score = SinglePassMatcher::new(sample, &patterns).scan().1;

    has_macro_call || score >= 3
}

fn csharp(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (C++, Java, TypeScript false positives)
    let anti_patterns = [
        LangPattern::new(b"import java.", 10),
        LangPattern::new(b"import javax.", 10),
        LangPattern::new(b"import com.", 10),
        LangPattern::new(b"import org.", 10),
        LangPattern::new(b"package ", 10),
        LangPattern::new(b"iostream", 10),
        LangPattern::new(b"#include", 10),
        LangPattern::new(b"cout", 8),
        LangPattern::new(b"std::", 8),
        LangPattern::new(b"export ", 5), // TypeScript
        LangPattern::new(b"const ", 3),  // TypeScript/JavaScript
    ];

    // Check antipatterns FIRST - early stop if exceed threshold
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(7) {
        return false;
    }

    // C# requires braces for code blocks
    let has_braces = sample.contains(&b'{') && sample.contains(&b'}');
    if !has_braces {
        return false;
    }

    // C# patterns with weights
    let patterns = [
        LangPattern::new(b"using System", 3), // C#-specific
        LangPattern::new(b"namespace ", 2),
        LangPattern::new(b"{ get; set; }", 3), // C# property
        LangPattern::new(b"string ", 2),       // C#-specific type
        LangPattern::new(b"async ", 2),
        LangPattern::new(b"await ", 2),
        LangPattern::new(b"public ", 2),
        LangPattern::new(b"private ", 2),
        LangPattern::new(b"static ", 2),
        LangPattern::simple(b"using "),
        LangPattern::simple(b"class "),
        LangPattern::simple(b"void "),
        LangPattern::simple(b"var "),
        LangPattern::simple(b"{ get"),
        LangPattern::simple(b"{ set"),
    ];

    SinglePassMatcher::new(sample, &patterns).scan().1 >= 3
}

fn vb(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // Helper function for case-insensitive pattern matching
    fn has_pattern_ci(data: &[u8], pattern: &[u8]) -> bool {
        data.windows(pattern.len())
            .any(|w| w.eq_ignore_ascii_case(pattern))
    }

    // VB-specific patterns (case-insensitive)
    let has_imports_system = has_pattern_ci(sample, b"Imports System");
    let has_end_function = has_pattern_ci(sample, b"End Function");
    let has_end_sub = has_pattern_ci(sample, b"End Sub");
    let has_end_class = has_pattern_ci(sample, b"End Class");
    let has_for_each = has_pattern_ci(sample, b"For Each");
    let has_byval = has_pattern_ci(sample, b"ByVal");
    let has_byref = has_pattern_ci(sample, b"ByRef");
    let has_nothing = has_pattern_ci(sample, b"Nothing");
    let has_module = has_pattern_ci(sample, b"Module");
    let has_imports = has_pattern_ci(sample, b"Imports");
    let has_dim = has_pattern_ci(sample, b"Dim ");
    let has_sub = has_pattern_ci(sample, b"Sub ");
    let has_function = has_pattern_ci(sample, b"Function");
    let has_as = has_pattern_ci(sample, b" As ");

    // Calculate VB score
    let vb_score = (has_dim as u8)
        + (has_sub as u8)
        + (has_function as u8)
        + (has_end_sub as u8) * 2
        + (has_end_function as u8) * 2
        + (has_end_class as u8) * 2
        + (has_module as u8) * 2
        + (has_imports as u8) * 2
        + (has_imports_system as u8) * 3
        + (has_as as u8)
        + (has_byval as u8) * 2
        + (has_byref as u8) * 2
        + (has_nothing as u8)
        + (has_for_each as u8);

    vb_score >= 3
}

fn php(input: &[u8]) -> bool {
    let sample = &input[..input.len().min(1024)];

    // PHP must have opening tag
    let has_php_tag = sample.starts_with(b"<?php")
        || sample.starts_with(b"<?\n")
        || sample.starts_with(b"<?\r")
        || sample.windows(5).any(|w| w == b"<?php")
        || sample.windows(3).any(|w| w == b"<?\n" || w == b"<?\r");

    if !has_php_tag {
        return false;
    }

    // PHP patterns with weights
    let patterns = [
        LangPattern::new(b"namespace ", 3), // PHP-specific
        LangPattern::new(b"function ", 2),
        LangPattern::new(b"echo ", 2),
        LangPattern::new(b"$_", 3), // PHP superglobals
        LangPattern::new(b"->", 2), // Object method call
        LangPattern::new(b"class ", 2),
        LangPattern::new(b"require ", 2),
        LangPattern::new(b"include ", 2),
        LangPattern::new(b"isset(", 2),
        LangPattern::new(b"empty(", 2),
        LangPattern::simple(b"use "),
        LangPattern::simple(b"public "),
        LangPattern::simple(b"private "),
        LangPattern::simple(b"protected "),
    ];

    // Check for $ variable sigil (very PHP-specific)
    let has_dollar = sample.contains(&b'$');

    let score = SinglePassMatcher::new(sample, &patterns).scan().1;

    (score >= 1) || has_dollar
}

fn python(input: &[u8]) -> bool {
    // Check for shebang first
    if has_lang_shebang(input, &[b"python", b"# -*- coding:"]) {
        return true;
    }

    let sample = &input[..input.len().min(1024)];

    // Python requires colons for control structures (def:, class:, if:, for:, etc.)
    if !sample.contains(&b':') {
        return false;
    }

    // Python patterns with weights
    let patterns = [
        LangPattern::new(b"def ", 2),
        LangPattern::simple(b"class "),
        LangPattern::simple(b"import "),
        LangPattern::simple(b"from "),
        LangPattern::simple(b"print("),
        LangPattern::simple(b"if "),
        LangPattern::new(b"elif ", 2), // Python-specific
        LangPattern::simple(b"else:"),
        LangPattern::simple(b"for "),
        LangPattern::simple(b"while "),
        LangPattern::new(b"with ", 2), // quite Python-specific
        LangPattern::simple(b"try:"),
        LangPattern::new(b"except:", 2), // Python-specific
        LangPattern::new(b"except ", 2), // except Exception
        LangPattern::new(b"finally:", 2),
        LangPattern::new(b"lambda ", 2),
        LangPattern::new(b"yield ", 2),
        LangPattern::new(b"async def ", 3),
        LangPattern::new(b"await ", 2),
        LangPattern::new(b"@property", 3),
        LangPattern::new(b"@staticmethod", 3),
        LangPattern::new(b"@classmethod", 3),
        LangPattern::new(b"__init__", 3),
        LangPattern::new(b"__name__", 2),
        LangPattern::new(b"__main__", 2),
    ];

    // Anti-patterns (C++ false positives)
    let anti_patterns = [
        LangPattern::new(b"class {", 10),
        LangPattern::new(b"class\n{", 10),
        LangPattern::new(b"class {\n", 10),
        LangPattern::new(b"namespace ", 5),
        LangPattern::new(b"#include", 5),
        LangPattern::new(b"std::", 5),
    ];

    // Check antipatterns FIRST - early stop on first C++ antipattern found
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(0) {
        return false;
    }

    let matcher = SinglePassMatcher::new(sample, &patterns);
    let (found, score) = matcher.scan();

    // Check for Python-specific indentation pattern (colon followed by indented line)
    let has_python_indentation = {
        let lines_iter = sample.split(|&b| b == b'\n');
        let mut looking_for_indent = false;

        for line in lines_iter {
            if looking_for_indent {
                let trimmed = line.trim_ascii();
                if !trimmed.is_empty() {
                    if line.starts_with(b"    ") || line.starts_with(b"\t") {
                        return true;
                    }
                    looking_for_indent = false;
                }
                // Skip empty lines while looking for indent
                continue;
            }

            let trimmed_line = line.trim_ascii_end();
            if trimmed_line.ends_with(b":") && !trimmed_line.starts_with(b"#") {
                looking_for_indent = true;
            }
        }
        false
    };

    // def or class with Python indentation pattern is Python-specific
    let has_def_or_class = found[0] || found[1];
    if has_def_or_class && has_python_indentation {
        return true;
    }

    score + (has_python_indentation as u8) * 3 >= 4
}

fn ruby(input: &[u8]) -> bool {
    // Check for shebang first
    const RUBY_SHEBANGS: &[&[u8]] = &[b"ruby"];
    if has_lang_shebang(input, RUBY_SHEBANGS) {
        return true;
    }

    // Exclude TCL (which also uses 'puts' but has different shebang)
    const TCL_SHEBANGS: &[&[u8]] = &[b"tclsh"];
    if has_lang_shebang(input, TCL_SHEBANGS) {
        return false;
    }

    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (Python false positives)
    let anti_patterns = [
        LangPattern::new(b"import ", 5),
        LangPattern::new(b"from ", 5),
        LangPattern::new(b"def __init__", 10),
        LangPattern::new(b"self.", 5),
    ];

    // Check antipatterns FIRST
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(4) {
        return false;
    }

    // Ruby requires 'end' keyword for blocks
    let has_end = sample.windows(3).any(|w| w == b"end");
    if !has_end {
        return false;
    }

    // Ruby patterns with weights
    let patterns = [
        LangPattern::new(b"attr_accessor", 3), // Ruby-specific
        LangPattern::new(b"attr_reader", 3),   // Ruby-specific
        LangPattern::new(b"attr_writer", 3),   // Ruby-specific
        LangPattern::new(b"require ", 2),
        LangPattern::new(b"puts ", 2),
        LangPattern::new(b"def ", 2),
        LangPattern::new(b"class ", 2),
        LangPattern::new(b"end\n", 2),
        LangPattern::new(b"end\r", 2),
        LangPattern::new(b"module ", 2),
        LangPattern::simple(b"end "),
        LangPattern::simple(b"do "),
        LangPattern::simple(b"elsif "),
        LangPattern::simple(b"unless "),
        LangPattern::simple(b"until "),
    ];

    let (found, score) = SinglePassMatcher::new(sample, &patterns).scan();

    // Ruby requires "end" keyword when using class/def (unlike Python)
    let has_def_or_class = found[5] || found[6]; // def or class
    let has_end = found[7] || found[8] || found[10]; // end\n, end\r, end

    // Check for "end" at the very end of sample (without trailing space/newline)
    let has_end_at_eof = sample.ends_with(b"end");

    if has_def_or_class && !has_end && !has_end_at_eof {
        return false;
    }

    score >= 2
}

fn perl(input: &[u8]) -> bool {
    // Check for shebang first
    const PERL_SHEBANGS: &[&[u8]] = &[b"perl"];
    if has_lang_shebang(input, PERL_SHEBANGS) {
        return true;
    }

    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (Java, Go, Rust, C#, C++ false positives)
    let anti_patterns = [
        LangPattern::new(b"public class ", 10),     // Java
        LangPattern::new(b"public enum ", 10),      // Java
        LangPattern::new(b"public interface ", 10), // Java
        LangPattern::new(b"import java.", 10),      // Java
        LangPattern::new(b"import javax.", 10),     // Java
        LangPattern::new(b"@Override", 10),         // Java
        LangPattern::new(b"System.out", 10),        // Java
        LangPattern::new(b"package main", 10),      // Go
        LangPattern::new(b"func main()", 10),       // Go, Rust
        LangPattern::new(b"func (", 10),            // Go method receiver
        LangPattern::new(b" := ", 10),              // Go
        LangPattern::new(b"using System", 10),      // C#
        LangPattern::new(b"namespace ", 10),        // C++, C#
        LangPattern::new(b"fn ", 10),               // Rust
        LangPattern::new(b"impl ", 10),             // Rust
    ];

    // Check antipatterns FIRST
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(9) {
        return false;
    }

    // Perl requires semicolons for statements
    if !sample.contains(&b';') {
        return false;
    }

    // Perl patterns with weights
    let patterns = [
        LangPattern::new(b"use strict;", 3),   // Perl-specific
        LangPattern::new(b"use warnings;", 3), // Perl-specific
        LangPattern::new(b"package ", 2),
        LangPattern::new(b"sub ", 2),
        LangPattern::new(b"use ", 2),
        LangPattern::new(b"my $", 2),
        LangPattern::new(b"$_", 2), // Default variable
        LangPattern::simple(b"my "),
        LangPattern::simple(b"our "),
        LangPattern::simple(b"local "),
        LangPattern::simple(b"foreach "),
        LangPattern::simple(b"unless "),
    ];

    // Check for $ variable sigil (very Perl-specific)
    let has_dollar = sample.contains(&b'$');

    let score = SinglePassMatcher::new(sample, &patterns).scan().1;

    (score >= 2) || (has_dollar && score >= 1)
}

fn lua(input: &[u8]) -> bool {
    // Check for shebang first
    const LUA_SHEBANGS: &[&[u8]] = &[b"lua"];
    if has_lang_shebang(input, LUA_SHEBANGS) {
        return true;
    }

    let sample = &input[..input.len().min(1024)];

    // Anti-patterns (Python, Perl, Shell false positives)
    let anti_patterns = [
        LangPattern::new(b"import ", 10),       // Python
        LangPattern::new(b"from ", 10),         // Python
        LangPattern::new(b"def __init__", 10),  // Python
        LangPattern::new(b"use strict;", 10),   // Perl
        LangPattern::new(b"use warnings;", 10), // Perl
        LangPattern::new(b"echo ", 5),          // Shell
        LangPattern::new(b"export ", 5),        // Shell
    ];

    // Check antipatterns FIRST
    if SinglePassMatcher::new(sample, &anti_patterns).scan_early_stop(9) {
        return false;
    }

    // Lua requires 'end' keyword for blocks
    let has_end = sample.windows(3).any(|w| w == b"end");
    if !has_end {
        return false;
    }

    // Lua patterns with weights
    let patterns = [
        LangPattern::new(b"function ", 3), // Lua-specific
        LangPattern::new(b"local ", 3),    // Lua-specific
        LangPattern::new(b"end\n", 2),     // Lua end keyword
        LangPattern::new(b"end ", 2),
        LangPattern::new(b"then\n", 2), // Lua conditional
        LangPattern::new(b"then ", 2),
        LangPattern::new(b"elseif ", 2), // Lua-specific (not 'elif' or 'elsif')
        LangPattern::new(b"do\n", 2),    // Lua do block
        LangPattern::new(b"do ", 2),
        LangPattern::simple(b"require("),  // Lua module import
        LangPattern::simple(b"require\""), // Lua module import
        LangPattern::simple(b"require'"),  // Lua module import
        LangPattern::simple(b"return "),
        LangPattern::simple(b"if "),
        LangPattern::simple(b"for "),
        LangPattern::simple(b"while "),
    ];

    SinglePassMatcher::new(sample, &patterns).scan().1 >= 3
}

fn visual_studio_solution(input: &[u8]) -> bool {
    // Microsoft Visual Studio Solution File
    // Can optionally start with UTF-8 BOM (EF BB BF)
    let data = if input.starts_with(b"\xEF\xBB\xBF") {
        &input[3..] // Skip BOM
    } else {
        input
    };

    // Skip optional leading whitespace/newlines
    let trimmed = data.trim_ascii_start();
    trimmed.starts_with(b"Microsoft Visual Studio Solution File, Format Version ")
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

/// Generic function to detect delimited text formats (CSV, TSV, etc.)
#[inline]
fn detect_delimited_format(input: &[u8], separator: u8) -> bool {
    // Split on both \n and \r to handle all line ending styles (Unix, Windows, old Mac)
    let lines = input
        .split(|&b| b == b'\n' || b == b'\r')
        .filter(|line| !line.trim_ascii().is_empty()) // Skip empty/whitespace lines
        .take(15);
    detect_csv_generic(lines, |line| count_csv_separators_quoted(line, separator))
}

fn csv_format(input: &[u8]) -> bool {
    detect_delimited_format(input, b',')
}

fn tsv(input: &[u8]) -> bool {
    detect_delimited_format(input, b'\t')
}

fn psv(input: &[u8]) -> bool {
    detect_delimited_format(input, b'|')
}

fn ssv(input: &[u8]) -> bool {
    detect_delimited_format(input, b';')
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

fn vcalendar(input: &[u8]) -> bool {
    // vCalendar 1.0 also starts with BEGIN:VCALENDAR but has VERSION:1.0
    // Check for both the BEGIN and VERSION:1.0 to distinguish from iCalendar 2.0
    case_insensitive_starts_with(input, b"BEGIN:VCALENDAR")
        && input.windows(11).any(|w| w == b"VERSION:1.0")
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

fn gltf(input: &[u8]) -> bool {
    json(input)
        && input.windows(8).any(|w| w == b"\"scenes\"")
        && input.windows(7).any(|w| w == b"\"nodes\"")
        && input.windows(7).any(|w| w == b"\"asset\"")
}

// ============================================================================
// GAMING FORMAT DETECTORS
// ============================================================================

// ============================================================================
// VIDEO FORMAT DETECTORS
// ============================================================================

fn three_gpp(input: &[u8]) -> bool {
    input.len() >= 12
        && matches!(
            &input[4..12],
            b"ftyp3gp4"
                | b"ftyp3gp5"
                | b"ftyp3gp6"
                | b"ftyp3gp7"
                | b"ftyp3gp8"
                | b"ftyp3gp9"
                | b"ftyp3gpa"
                | b"ftyp3gpp"
        )
}

fn three_gpp2(input: &[u8]) -> bool {
    input.len() >= 12
        && matches!(
            &input[4..12],
            b"ftyp3g24"
                | b"ftyp3g25"
                | b"ftyp3g26"
                | b"ftyp3g27"
                | b"ftyp3g28"
                | b"ftyp3g29"
                | b"ftyp3g2a"
                | b"ftyp3g2b"
                | b"ftyp3g2c"
        )
}

fn mj2(input: &[u8]) -> bool {
    input.len() >= 12 && matches!(&input[4..12], b"ftypmj2s" | b"ftypmjp2")
}

// ============================================================================
// MAC FORMAT DETECTORS
// ============================================================================

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

/// PSV detection for UTF-16 Big Endian
fn psv_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_psv_content)
}

/// PSV detection for UTF-16 Little Endian
fn psv_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_psv_content)
}

/// SSV detection for UTF-16 Big Endian
fn ssv_utf16_be(input: &[u8]) -> bool {
    detect_utf16_format(input, true, detect_ssv_content)
}

/// SSV detection for UTF-16 Little Endian
fn ssv_utf16_le(input: &[u8]) -> bool {
    detect_utf16_format(input, false, detect_ssv_content)
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

/// HTML detection for UTF-8 with BOM
fn html_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_html_content)
}

/// XML detection for UTF-8 with BOM
fn xml_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_xml_content)
}

/// SVG detection for UTF-8 with BOM
fn svg_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_svg_content)
}

/// JSON detection for UTF-8 with BOM
fn json_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_json_content)
}

/// CSV detection for UTF-8 with BOM
fn csv_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_csv_content)
}

/// TSV detection for UTF-8 with BOM
fn tsv_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_tsv_content)
}

/// PSV detection for UTF-8 with BOM
fn psv_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_psv_content)
}

/// SSV detection for UTF-8 with BOM
fn ssv_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_ssv_content)
}

/// SRT subtitle detection for UTF-8 with BOM
fn srt_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_srt_content)
}

/// VTT subtitle detection for UTF-8 with BOM
fn vtt_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_vtt_content)
}

/// vCard detection for UTF-8 with BOM
fn vcard_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_vcard_content)
}

/// iCalendar detection for UTF-8 with BOM
fn icalendar_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_icalendar_content)
}

/// RTF detection for UTF-8 with BOM
fn rtf_utf8_bom(input: &[u8]) -> bool {
    detect_utf8_bom_format(input, detect_rtf_content)
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
    (trimmed.starts_with('{') || trimmed.starts_with('[')) && is_valid_json(trimmed.as_bytes())
}

/// Generic function to detect delimited text in decoded text (for UTF-16 and UTF-8 BOM)
#[inline]
fn detect_delimited_content(text: &str, separator: u8) -> bool {
    let lines = text
        .lines()
        .filter(|line| !line.trim().is_empty()) // Skip empty lines
        .take(15); // Increased sample size
    detect_csv_generic(lines, |line| {
        count_csv_separators_quoted(line.as_bytes(), separator)
    })
}

/// Shared CSV content detection that works with any encoding after normalization
fn detect_csv_content(text: &str) -> bool {
    detect_delimited_content(text, b',')
}

/// Shared TSV content detection that works with any encoding after normalization
fn detect_tsv_content(text: &str) -> bool {
    detect_delimited_content(text, b'\t')
}

/// Shared PSV content detection that works with any encoding after normalization
fn detect_psv_content(text: &str) -> bool {
    detect_delimited_content(text, b'|')
}

/// Shared SSV content detection that works with any encoding after normalization
fn detect_ssv_content(text: &str) -> bool {
    detect_delimited_content(text, b';')
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

/// Count separators in a line while respecting quoted fields (RFC 4180)
/// This function skips separators that appear inside quoted fields
#[inline]
fn count_csv_separators_quoted(line: &[u8], separator: u8) -> usize {
    let mut count = 0;
    let mut in_quotes = false;
    let mut i = 0;

    while i < line.len() {
        let byte = line[i];

        match byte {
            b'"' => {
                // Handle doubled quotes ("") as escaped quotes
                if in_quotes && i + 1 < line.len() && line[i + 1] == b'"' {
                    i += 1; // Skip the next quote
                } else {
                    in_quotes = !in_quotes;
                }
            }
            _ if byte == separator && !in_quotes => {
                count += 1;
            }
            _ => {}
        }
        i += 1;
    }

    count
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
fn detect_csv_generic<T, F>(lines: T, count_separator: F) -> bool
where
    T: Iterator,
    F: Fn(T::Item) -> usize,
{
    // Use stack-allocated array to avoid heap allocation (max 15 lines sampled)
    const MAX_LINES: usize = 15;
    let mut separator_counts: [usize; MAX_LINES] = [0; MAX_LINES];
    let mut line_count = 0;
    let mut total_separators = 0;

    for line in lines {
        if line_count >= MAX_LINES {
            break;
        }
        let count = count_separator(line);
        separator_counts[line_count] = count;
        total_separators += count;
        line_count += 1;
    }

    if line_count < 2 {
        return false;
    }

    let average = total_separators as f32 / line_count as f32;

    // Require at least 1 separator on average for CSV/TSV detection
    if average < 1.0 {
        return false;
    }

    let expected_count = average.round() as usize;

    let matching_lines = separator_counts[..line_count]
        .iter()
        .filter(|&&count| count == expected_count || count.abs_diff(expected_count) == 1)
        .count();

    let match_ratio = matching_lines as f32 / line_count as f32;

    // Allow 80% of lines to match expected count (handles ragged CSV)
    match_ratio >= 0.8
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

        if self.pos + 4 >= self.data.len() {
            return None;
        }

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

/// Extract the CLSID from an OLE compound document
/// Returns a 16-byte slice containing the CLSID if successful
/// Based on Go implementation: matchOleClsid function
fn get_ole_clsid(input: &[u8]) -> Option<&[u8]> {
    // Microsoft Compound files v3 have a sector length of 512, while v4 has 4096.
    // Change sector offset depending on file version.
    let sector_length = if input.len() >= 28 && input[26] == 0x04 && input[27] == 0x00 {
        4096
    } else {
        512
    };

    if input.len() < sector_length {
        return None;
    }

    // SecID of first sector of the directory stream (offset 48-51)
    if input.len() < 52 {
        return None;
    }

    let first_sec_id = u32::from_le_bytes([input[48], input[49], input[50], input[51]]) as usize;

    // Expected offset of CLSID for root storage object
    let clsid_offset = sector_length * (1 + first_sec_id) + 80;

    // Return the 16-byte CLSID if it exists
    if input.len() < clsid_offset + 16 {
        return None;
    }

    Some(&input[clsid_offset..clsid_offset + 16])
}

/// Simple JSON validation
fn is_valid_json(input: &[u8]) -> bool {
    // JSON validation optimized for partial content (first 1024bytes)
    // For large files, we can't expect balanced brackets, so we look for JSON patterns
    let mut brace_count = 0;
    let mut bracket_count = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let mut has_colon = false;
    let mut has_comma = false;
    let mut has_opening = false;

    for &byte in input.iter().take(1024) {
        // Limit check to first 512 bytes
        if escape_next {
            escape_next = false;
            continue;
        }

        match byte {
            b'\\' if in_string => escape_next = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => {
                brace_count += 1;
                has_opening = true;
            }
            b'}' if !in_string => brace_count -= 1,
            b'[' if !in_string => {
                bracket_count += 1;
                has_opening = true;
            }
            b']' if !in_string => bracket_count -= 1,
            b':' if !in_string => has_colon = true,
            b',' if !in_string => has_comma = true,
            _ => {}
        }

        // Brackets should never go negative (more closes than opens)
        if brace_count < 0 || bracket_count < 0 {
            return false;
        }
    }

    // Must have opening bracket/brace and look like JSON
    // For objects: expect colons (key:value pairs)
    // For arrays or objects: might have commas
    // Don't require perfect balance since we only check first 512 bytes
    has_opening && (has_colon || has_comma || (brace_count == 0 && bracket_count == 0))
}

// ============================================================================
// ELF SUBTYPE DETECTORS
// ============================================================================

/// ELF Object File (ET_REL)
fn elf_obj(input: &[u8]) -> bool {
    input.len() >= 18 && input.starts_with(b"\x7fELF") && input[16] == 1 && input[17] == 0
}

/// ELF Executable (ET_EXEC)
fn elf_exe(input: &[u8]) -> bool {
    input.len() >= 18 && input.starts_with(b"\x7fELF") && input[16] == 2 && input[17] == 0
}

/// ELF Shared Library (ET_DYN)
fn elf_lib(input: &[u8]) -> bool {
    input.len() >= 18 && input.starts_with(b"\x7fELF") && input[16] == 3 && input[17] == 0
}

/// ELF Core Dump (ET_CORE)
fn elf_dump(input: &[u8]) -> bool {
    input.len() >= 18 && input.starts_with(b"\x7fELF") && input[16] == 4 && input[17] == 0
}

/// AAF (Advanced Authoring Format)
/// Note: Parent OLE already validated signature
fn aaf(input: &[u8]) -> bool {
    // AAF uses a specific CLSID to distinguish from other OLE formats
    // This prevents it from matching generic OLE or other Office documents
    const AAF_CLSID: &[u8] = &[
        0xAA, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];

    get_ole_clsid(input).is_some_and(|actual| actual == AAF_CLSID)
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
        return detect_content(&text);
    }
    false
}

/// Generic UTF-8 BOM format detection helper
/// Consolidates the pattern used by all UTF-8 BOM detection functions
#[inline]
fn detect_utf8_bom_format<F>(input: &[u8], detect_content: F) -> bool
where
    F: Fn(&str) -> bool,
{
    // UTF-8 BOM is 0xEF 0xBB 0xBF (3 bytes)
    // Input should already start with BOM since this is only called for UTF8_BOM children
    if input.len() >= 3 && input.starts_with(b"\xEF\xBB\xBF") {
        // Skip the BOM (3 bytes) and convert to str
        if let Ok(text) = std::str::from_utf8(&input[3..]) {
            return detect_content(text);
        }
    }
    false
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

    // Check prefix and mimetype separately to avoid allocation
    input.len() >= 30 + total_len
        && &input[30..30 + prefix_len] == MIMETYPE_PREFIX
        && &input[30 + prefix_len..30 + total_len] == mimetype
}
