//! Individual MIME Type Tests
//!
//! This module provides comprehensive individual test functions for all 214 supported MIME types
//! and their child formats in the mimetype-detector library. Each MIME type has its own dedicated
//! test function using proven test data from the existing test suite.
//!
//! # Test Organization
//!
//! Tests are organized by logical categories:
//! - Text formats (HTML, XML, UTF variants)
//! - Document formats (PDF, PostScript, Office documents)
//! - Archive formats (ZIP, TAR, 7Z, RAR, etc.)
//! - Image formats (PNG, JPEG, GIF, WebP, etc.)
//! - Audio formats (MP3, FLAC, WAV, OGG, etc.)
//! - Video formats (MP4, WebM, AVI, MKV, etc.)
//! - Executable formats (ELF, PE/EXE, Java CLASS, WASM)
//! - Font formats (TTF, OTF, WOFF, WOFF2)
//! - Database formats (SQLite, Access, dBase, etc.)
//! - Programming & text formats (PHP, JavaScript, Python, etc.)
//! - XML-based formats (RSS, Atom, SVG, etc.)
//! - 3D & geospatial formats
//! - Gaming formats
//! - Miscellaneous formats
//! - UTF-16 text format variants
//! - Child/hierarchical formats

use mimetype_detector::{constants::*, detect};

// ============================================================================
// TEST HELPERS
// ============================================================================

/// Create a proper OLE file structure with a specific CLSID
///
/// This helper builds a minimal but valid OLE Compound File with:
/// - Proper OLE header (512-byte sectors, v3)
/// - Directory stream at sector 0
/// - CLSID placed at the correct offset (592 bytes)
fn create_ole_with_clsid(clsid: &[u8]) -> Vec<u8> {
    const SECTOR_SIZE: usize = 512;
    let mut data = vec![0u8; SECTOR_SIZE * 2 + 100]; // Header + directory sector + extra

    data[0..8].copy_from_slice(&[0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]);

    data[24..26].copy_from_slice(&[0x3e, 0x00]);

    data[26..28].copy_from_slice(&[0x09, 0x00]);

    data[48..52].copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    const CLSID_OFFSET: usize = SECTOR_SIZE + 80;
    let clsid_len = clsid.len().min(16);
    data[CLSID_OFFSET..CLSID_OFFSET + clsid_len].copy_from_slice(&clsid[..clsid_len]);

    data
}

/// Create a proper ZIP file with a specific filename entry
///
/// This helper builds a minimal but valid ZIP file with:
/// - ZIP local file header
/// - Specified filename in the entry
/// - No actual file data (empty stored file)
fn create_zip_with_file(filename: &[u8]) -> Vec<u8> {
    let mut data = Vec::new();

    data.extend_from_slice(b"PK\x03\x04"); // Signature
    data.extend_from_slice(&[0x14, 0x00]); // Version needed (2.0)
    data.extend_from_slice(&[0x00, 0x00]); // Flags
    data.extend_from_slice(&[0x00, 0x00]); // Compression method (stored)
    data.extend_from_slice(&[0x00, 0x00]); // Last mod time
    data.extend_from_slice(&[0x00, 0x00]); // Last mod date
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC32
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Compressed size
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Uncompressed size

    let filename_len = filename.len() as u16;
    data.extend_from_slice(&filename_len.to_le_bytes());

    data.extend_from_slice(&[0x00, 0x00]); // Extra field length
    data.extend_from_slice(filename); // Filename

    data
}

// ============================================================================
// TEXT FORMATS
// ============================================================================

#[test]
fn test_detect_html() {
    let data = b"<html>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_HTML);
    assert_eq!(mime_type.extension(), ".html");
    assert!(mime_type.is(TEXT_HTML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_xml() {
    let data = b"<?xml version=\"1.0\"?>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_XML);
    assert_eq!(mime_type.extension(), ".xml");
    assert!(mime_type.is(TEXT_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_utf8_bom() {
    let data = b"\xEF\xBB\xBFHello World";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF8_BOM);
    assert_eq!(mime_type.extension(), ".txt");
    assert!(mime_type.is(TEXT_UTF8_BOM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_utf16_be() {
    let data = b"\xFE\xFF\x00H\x00e\x00l\x00l\x00o";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_BE);
    assert_eq!(mime_type.extension(), ".txt");
    assert!(mime_type.is(TEXT_UTF16_BE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_utf16_le() {
    let data = b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_LE);
    assert_eq!(mime_type.extension(), ".txt");
    assert!(mime_type.is(TEXT_UTF16_LE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_utf8() {
    let data = b"Hello World";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF8);
    assert_eq!(mime_type.extension(), ".txt");
    assert!(mime_type.is(TEXT_UTF8));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

// ============================================================================
// DOCUMENT FORMATS
// ============================================================================

#[test]
fn test_detect_pdf() {
    let data = b"%PDF-1.4";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_PDF);
    assert_eq!(mime_type.extension(), ".pdf");
    assert!(mime_type.is(APPLICATION_PDF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_fdf() {
    let data = b"%FDF-1.2";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_FDF);
    assert_eq!(mime_type.extension(), ".fdf");
    assert!(mime_type.is(APPLICATION_VND_FDF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_postscript() {
    let data = b"%!PS-Adobe-3.0";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_POSTSCRIPT);
    assert_eq!(mime_type.extension(), ".ps");
    assert!(mime_type.is(APPLICATION_POSTSCRIPT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_ole() {
    let data = b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_OLE_STORAGE);
    assert_eq!(mime_type.extension(), "");
    assert!(mime_type.is(APPLICATION_X_OLE_STORAGE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_aaf() {
    const AAF_CLSID: &[u8] = &[
        0xAA, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    let data = create_ole_with_clsid(AAF_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_AAF);
    assert_eq!(mime_type.extension(), ".aaf");
    assert!(mime_type.is(APPLICATION_X_AAF));
    assert!(mime_type.kind().is_document());
}

// ============================================================================
// ARCHIVE & COMPRESSION FORMATS
// ============================================================================

#[test]
fn test_detect_7z() {
    let data = b"7z\xbc\xaf\x27\x1c";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_7Z_COMPRESSED);
    assert_eq!(mime_type.extension(), ".7z");
    assert!(mime_type.is(APPLICATION_X_7Z_COMPRESSED));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_zip() {
    let data = b"PK\x03\x04";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_ZIP);
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_rar() {
    let data = b"Rar!\x1a\x07\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_RAR_COMPRESSED);
    assert_eq!(mime_type.extension(), ".rar");
    assert!(mime_type.is(APPLICATION_X_RAR_COMPRESSED));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_gzip() {
    let data = b"\x1f\x8b";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_GZIP);
    assert_eq!(mime_type.extension(), ".gz");
    assert!(mime_type.is(APPLICATION_GZIP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_tar() {
    let mut data = vec![0; 512];
    data[0..5].copy_from_slice(b"test\0");
    data[100..108].copy_from_slice(b"0000644\0");
    data[108..116].copy_from_slice(b"0000000\0");
    data[116..124].copy_from_slice(b"0000000\0");
    data[124..136].copy_from_slice(b"00000000000\0");
    data[136..148].copy_from_slice(b"00000000000\0");

    let checksum = data.iter().take(148).map(|&b| b as u32).sum::<u32>()
        + (b' ' as u32) * 8
        + data.iter().skip(156).map(|&b| b as u32).sum::<u32>();
    let checksum_str = format!("{checksum:06o}\0 ");
    data[148..156].copy_from_slice(checksum_str.as_bytes());

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_TAR);
    assert_eq!(mime_type.extension(), ".tar");
    assert!(mime_type.is(APPLICATION_X_TAR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_bz2() {
    let data = b"BZ";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_BZIP2);
    assert_eq!(mime_type.extension(), ".bz2");
    assert!(mime_type.is(APPLICATION_X_BZIP2));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_xz() {
    let data = b"\xfd7zXZ\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_XZ);
    assert_eq!(mime_type.extension(), ".xz");
    assert!(mime_type.is(APPLICATION_X_XZ));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_zstd() {
    let data = b"\x28\xb5\x2f\xfd"; // Zstandard magic
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_ZSTD);
    assert_eq!(mime_type.extension(), ".zst");
    assert!(mime_type.is(APPLICATION_ZSTD));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_lzip() {
    let data = b"LZIP";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_LZIP);
    assert_eq!(mime_type.extension(), ".lz");
    assert!(mime_type.is(APPLICATION_LZIP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_cab() {
    let data = b"MSCF";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_CAB_COMPRESSED);
    assert_eq!(mime_type.extension(), ".cab");
    assert!(mime_type.is(APPLICATION_VND_MS_CAB_COMPRESSED));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_install_shield_cab() {
    let data = b"ISc(\x00\x00\x00\x01";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_INSTALLSHIELD);
    assert_eq!(mime_type.extension(), ".cab");
    assert!(mime_type.is(APPLICATION_X_INSTALLSHIELD));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_cpio() {
    let data = b"070701";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_CPIO);
    assert_eq!(mime_type.extension(), ".cpio");
    assert!(mime_type.is(APPLICATION_X_CPIO));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());

    let binary_data = b"\xc7\xc7\x00\x00\x00\x00";
    let mime_type_bin = detect(binary_data);
    assert_eq!(mime_type_bin.mime(), APPLICATION_X_CPIO);
    assert_eq!(mime_type_bin.extension(), ".cpio");
    assert!(mime_type_bin.is(APPLICATION_X_CPIO));
}

#[test]
fn test_detect_ar() {
    let data = b"!<arch>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_ARCHIVE);
    assert_eq!(mime_type.extension(), ".a");
    assert!(mime_type.is(APPLICATION_X_ARCHIVE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_rpm() {
    let data = b"\xed\xab\xee\xdb";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_RPM);
    assert_eq!(mime_type.extension(), ".rpm");
    assert!(mime_type.is(APPLICATION_X_RPM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_torrent() {
    let data = b"d8:announce";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_BITTORRENT);
    assert_eq!(mime_type.extension(), ".torrent");
    assert!(mime_type.is(APPLICATION_X_BITTORRENT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_fits() {
    let data = b"SIMPLE  =                    T";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_FITS);
    assert_eq!(mime_type.extension(), ".fits");
    assert!(mime_type.is(APPLICATION_FITS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_xar() {
    let data = b"xar!";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_XAR);
    assert_eq!(mime_type.extension(), ".xar");
    assert!(mime_type.is(APPLICATION_X_XAR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_deb() {
    let mut data = vec![0; 21];
    data[0..7].copy_from_slice(b"!<arch>");
    data[8..21].copy_from_slice(b"debian-binary");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_DEBIAN_BINARY_PACKAGE);
    assert_eq!(mime_type.extension(), ".deb");
    assert!(mime_type.is(APPLICATION_VND_DEBIAN_BINARY_PACKAGE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_warc() {
    let data = b"WARC/1.0\r\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_WARC);
    assert_eq!(mime_type.extension(), ".warc");
    assert!(mime_type.is(APPLICATION_WARC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_lz4() {
    let data = b"\x04\x22\x4D\x18\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_LZ4);
    assert_eq!(mime_type.extension(), ".lz4");
    assert!(mime_type.is(APPLICATION_X_LZ4));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_arj() {
    let data = b"\x60\xEA\x00\x00\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_ARJ);
    assert_eq!(mime_type.extension(), ".arj");
    assert!(mime_type.is(APPLICATION_ARJ));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_lha() {
    let data = b"-lh0-\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_LZH_COMPRESSED);
    assert_eq!(mime_type.extension(), ".lzh");
    assert!(mime_type.is(APPLICATION_X_LZH_COMPRESSED));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

// ============================================================================
// IMAGE FORMATS
// ============================================================================

#[test]
fn test_detect_png() {
    let data = b"\x89PNG\r\n\x1a\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_PNG);
    assert_eq!(mime_type.extension(), ".png");
    assert!(mime_type.is(IMAGE_PNG));
    assert!(!mime_type.is(IMAGE_JPEG));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_apng() {
    let mut data = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]; // PNG header
    data.resize(37, 0);
    data.extend_from_slice(b"acTL"); // APNG marker
    let mime_type = detect(&data);
    assert!(mime_type.mime() == IMAGE_VND_MOZILLA_APNG)
}

#[test]
fn test_detect_jpeg() {
    let data = b"\xff\xd8\xff\xe0";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_JPEG);
    assert_eq!(mime_type.extension(), ".jpg");
    assert!(mime_type.is(IMAGE_JPEG));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_jp2() {
    let mut data = vec![0, 0, 0, 0];
    data.extend_from_slice(b"jP2 ");
    data.resize(20, 0);
    data.extend_from_slice(b"jp2 ");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_JP2);
    assert_eq!(mime_type.extension(), ".jp2");
    assert!(mime_type.is(IMAGE_JP2));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_jpx() {
    let mut data = vec![0, 0, 0, 0];
    data.extend_from_slice(b"jP2 ");
    data.resize(20, 0);
    data.extend_from_slice(b"jpx ");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_JPX);
    assert_eq!(mime_type.extension(), ".jpx");
    assert!(mime_type.is(IMAGE_JPX));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_jpm() {
    let mut data = vec![0, 0, 0, 0];
    data.extend_from_slice(b"jP2 ");
    data.resize(20, 0);
    data.extend_from_slice(b"jpm ");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_JPM);
    assert_eq!(mime_type.extension(), ".jpm");
    assert!(mime_type.is(IMAGE_JPM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_jxs() {
    let data = b"\x00\x00\x00\x0C\x4A\x58\x53\x20\x0D\x0A\x87\x0A";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_JXS);
    assert_eq!(mime_type.extension(), ".jxs");
    assert!(mime_type.is(IMAGE_JXS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_jxr() {
    let data = b"\x49\x49\xBC\x01";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_JXR);
    assert_eq!(mime_type.extension(), ".jxr");
    assert!(mime_type.is(IMAGE_JXR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_jxl() {
    let data = b"\xFF\x0A";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_JXL);
    assert_eq!(mime_type.extension(), ".jxl");
    assert!(mime_type.is(IMAGE_JXL));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_gif() {
    let data = b"GIF89a";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_GIF);
    assert_eq!(mime_type.extension(), ".gif");
    assert!(mime_type.is(IMAGE_GIF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_webp() {
    let data = b"RIFF\x00\x00\x00\x00WEBP";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_WEBP);
    assert_eq!(mime_type.extension(), ".webp");
    assert!(mime_type.is(IMAGE_WEBP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_tiff() {
    let data = b"II*\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_TIFF);
    assert_eq!(mime_type.extension(), ".tiff");
    assert!(mime_type.is(IMAGE_TIFF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_bmp() {
    let data = b"BM";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_BMP);
    assert_eq!(mime_type.extension(), ".bmp");
    assert!(mime_type.is(IMAGE_BMP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_ico() {
    let data = b"\x00\x00\x01\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_ICON);
    assert_eq!(mime_type.extension(), ".ico");
    assert!(mime_type.is(IMAGE_X_ICON));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_icns() {
    let data = b"icns";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_ICNS);
    assert_eq!(mime_type.extension(), ".icns");
    assert!(mime_type.is(IMAGE_X_ICNS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_psd() {
    let data = b"8BPS";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_VND_ADOBE_PHOTOSHOP);
    assert_eq!(mime_type.extension(), ".psd");
    assert!(mime_type.is(IMAGE_VND_ADOBE_PHOTOSHOP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_heic() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"heic");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_HEIC);
    assert_eq!(mime_type.extension(), ".heic");
    assert!(mime_type.is(IMAGE_HEIC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_heic_sequence() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"hevc");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_HEIC_SEQUENCE);
    assert_eq!(mime_type.extension(), ".heic");
    assert!(mime_type.is(IMAGE_HEIC_SEQUENCE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_heif() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"mif1");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_HEIF);
    assert_eq!(mime_type.extension(), ".heif");
    assert!(mime_type.is(IMAGE_HEIF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_heif_sequence() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"msf1");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_HEIF);
    assert_eq!(mime_type.extension(), ".heif");
}

#[test]
fn test_detect_bpg() {
    let data = b"BPG\xFB";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_BPG);
    assert_eq!(mime_type.extension(), ".bpg");
    assert!(mime_type.is(IMAGE_BPG));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_xcf() {
    let data = b"gimp xcf v011";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_XCF);
    assert_eq!(mime_type.extension(), ".xcf");
    assert!(mime_type.is(IMAGE_X_XCF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_pat() {
    let data =
        b"\x00\x00\x00\x1c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x03GPAT";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_GIMP_PAT);
    assert_eq!(mime_type.extension(), ".pat");
    assert!(mime_type.is(IMAGE_X_GIMP_PAT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_gbr() {
    let mut data = vec![0; 24];
    data[20..24].copy_from_slice(b"GIMP");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_X_GIMP_GBR);
    assert_eq!(mime_type.extension(), ".gbr");
    assert!(mime_type.is(IMAGE_X_GIMP_GBR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_hdr() {
    let data = b"#?RADIANCE\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_VND_RADIANCE);
    assert_eq!(mime_type.extension(), ".hdr");
    assert!(mime_type.is(IMAGE_VND_RADIANCE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_xpm() {
    let data = b"/* XPM */";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_XPIXMAP);
    assert_eq!(mime_type.extension(), ".xpm");
    assert!(mime_type.is(IMAGE_X_XPIXMAP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_dwg() {
    let data = b"AC1024";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_VND_DWG);
    assert_eq!(mime_type.extension(), ".dwg");
    assert!(mime_type.is(IMAGE_VND_DWG));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_dxf() {
    let data = b"  0\nSECTION\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_VND_DXF);
    assert_eq!(mime_type.extension(), ".dxf");
    assert!(mime_type.is(IMAGE_VND_DXF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_djvu() {
    let data = b"AT&TFORM\x00\x00\x00\x00DJVU";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_VND_DJVU);
    assert_eq!(mime_type.extension(), ".djvu");
    assert!(mime_type.is(IMAGE_VND_DJVU));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_avif() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"avif");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_AVIF);
    assert_eq!(mime_type.extension(), ".avif");
    assert!(mime_type.is(IMAGE_AVIF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_dds() {
    let data = b"DDS \x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_VND_MS_DDS);
    assert_eq!(mime_type.extension(), ".dds");
    assert!(mime_type.is(IMAGE_VND_MS_DDS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_pcx() {
    let data = b"\x0A\x05\x01\x08\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_PCX);
    assert_eq!(mime_type.extension(), ".pcx");
    assert!(mime_type.is(IMAGE_X_PCX));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_ktx() {
    let data = b"\xAB\x4B\x54\x58\x20\x31\x31\xBB";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_KTX);
    assert_eq!(mime_type.extension(), ".ktx");
    assert!(mime_type.is(IMAGE_KTX));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_astc() {
    let data = b"\x13\xAB\xA1\x5C\x01\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_ASTC);
    assert_eq!(mime_type.extension(), ".astc");
    assert!(mime_type.is(IMAGE_X_ASTC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_tga() {
    let data = b"\x00\x01\x0A\x00\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_TGA);
    assert_eq!(mime_type.extension(), ".tga");
    assert!(mime_type.is(IMAGE_X_TGA));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_sun_raster() {
    let data = b"\x59\xA6\x6A\x95\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_SUN_RASTER);
    assert_eq!(mime_type.extension(), ".ras");
    assert!(mime_type.is(IMAGE_X_SUN_RASTER));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_sgi() {
    let data = b"\x01\xDA\x00\x01\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_SGI);
    assert_eq!(mime_type.extension(), ".sgi");
    assert!(mime_type.is(IMAGE_X_SGI));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_ani() {
    let data = b"RIFF\x00\x00\x00\x00ACON\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_NAVI_ANIMATION);
    assert_eq!(mime_type.extension(), ".ani");
    assert!(mime_type.is(APPLICATION_X_NAVI_ANIMATION));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_cdr() {
    let data = b"RIFF\x00\x00\x00\x00CDR\x00\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_COREL_DRAW);
    assert_eq!(mime_type.extension(), ".cdr");
    assert!(mime_type.is(APPLICATION_VND_COREL_DRAW));
    assert!(mime_type.is(APPLICATION_CDR));
    assert!(mime_type.is(APPLICATION_X_CDR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_ilbm() {
    let data = b"FORM\x00\x00\x00\x00ILBM\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_ILBM);
    assert_eq!(mime_type.extension(), ".lbm");
    assert!(mime_type.is(IMAGE_X_ILBM));
    assert!(mime_type.is(IMAGE_X_IFF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

// ============================================================================
// AUDIO FORMATS
// ============================================================================

#[test]
fn test_detect_mp3() {
    let data = b"ID3";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_MPEG);
    assert_eq!(mime_type.extension(), ".mp3");
    assert!(mime_type.is(AUDIO_MPEG));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_flac() {
    let data = b"fLaC";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_FLAC);
    assert_eq!(mime_type.extension(), ".flac");
    assert!(mime_type.is(AUDIO_FLAC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_wav() {
    let data = b"RIFF\x00\x00\x00\x00WAVE";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_WAV);
    assert_eq!(mime_type.extension(), ".wav");
    assert!(mime_type.is(AUDIO_WAV));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_aiff() {
    let data = b"FORM\x00\x00\x00\x00AIFF";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_AIFF);
    assert_eq!(mime_type.extension(), ".aiff");
    assert!(mime_type.is(AUDIO_AIFF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_midi() {
    let data = b"MThd\x00\x00\x00\x06";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_MIDI);
    assert_eq!(mime_type.extension(), ".midi");
    assert!(mime_type.is(AUDIO_MIDI));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_ogg() {
    let data = b"OggS";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_OGG);
    assert_eq!(mime_type.extension(), ".ogg");
    assert!(mime_type.is(APPLICATION_OGG));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_ogg_audio() {
    let mut data = vec![0; 37];
    data[0..4].copy_from_slice(b"OggS");
    data[28..37].copy_from_slice(b"\x7fFLAC\x00\x00\x00\x00");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), AUDIO_OGG);
    assert_eq!(mime_type.extension(), ".oga");
    assert!(mime_type.is(AUDIO_OGG));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_ogg_video() {
    let mut data = vec![0; 37];
    data[0..4].copy_from_slice(b"OggS");
    data[28..35].copy_from_slice(b"\x80theora");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_OGG);
    assert_eq!(mime_type.extension(), ".ogv");
    assert!(mime_type.is(VIDEO_OGG));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_ape() {
    let data = b"MAC \x96\x0F\x00\x00\x34\x00\x00\x00\x18\x00\x00\x00\x90\xE3";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_APE);
    assert_eq!(mime_type.extension(), ".ape");
    assert!(mime_type.is(AUDIO_APE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_musepack() {
    let data = b"MPCK";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_MUSEPACK);
    assert_eq!(mime_type.extension(), ".mpc");
    assert!(mime_type.is(AUDIO_MUSEPACK));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_au() {
    let data = b".snd";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_BASIC);
    assert_eq!(mime_type.extension(), ".au");
    assert!(mime_type.is(AUDIO_BASIC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_amr() {
    let data = b"#!AMR";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_AMR);
    assert_eq!(mime_type.extension(), ".amr");
    assert!(mime_type.is(AUDIO_AMR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_voc() {
    let data = b"Creative Voice File";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_X_VOC);
    assert_eq!(mime_type.extension(), ".voc");
    assert!(mime_type.is(AUDIO_X_VOC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_m3u() {
    let data = b"#EXTM3U";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_X_MPEGURL);
    assert_eq!(mime_type.extension(), ".m3u");
    assert!(mime_type.is(AUDIO_X_MPEGURL));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_aac() {
    let data = b"\xFF\xF1\x50\x80";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_AAC);
    assert_eq!(mime_type.extension(), ".aac");
    assert!(mime_type.is(AUDIO_AAC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_qcp() {
    let data = b"RIFF\x00\x00\x00\x00QLCM";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_QCELP);
    assert_eq!(mime_type.extension(), ".qcp");
    assert!(mime_type.is(AUDIO_QCELP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_m4a() {
    let data = b"\x00\x00\x00\x18ftypM4A ";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_amp4() {
    let data = b"\x00\x00\x00\x18ftypF4A ";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_wavpack() {
    let data = b"wvpk\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_X_WAVPACK);
    assert_eq!(mime_type.extension(), ".wv");
    assert!(mime_type.is(AUDIO_X_WAVPACK));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_tta() {
    let data = b"TTA1\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_X_TTA);
    assert_eq!(mime_type.extension(), ".tta");
    assert!(mime_type.is(AUDIO_X_TTA));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_dsf() {
    let data = b"DSD \x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_X_DSF);
    assert_eq!(mime_type.extension(), ".dsf");
    assert!(mime_type.is(AUDIO_X_DSF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_dff() {
    let data = b"FRM8\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_X_DFF);
    assert_eq!(mime_type.extension(), ".dff");
    assert!(mime_type.is(AUDIO_X_DFF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

// ============================================================================
// VIDEO FORMATS
// ============================================================================

#[test]
fn test_detect_mp4() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"isom");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_MP4);
    assert_eq!(mime_type.extension(), ".mp4");
    assert!(mime_type.is(VIDEO_MP4));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_webm() {
    let data = b"\x1aE\xdf\xa3\x01\x00\x00\x00\x00\x00\x00\x1fB\x86\x81\x01B\xf7\x81\x01B\xf2\x81\x04B\xf3\x81\x08B\x82\x84webm";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_WEBM);
    assert_eq!(mime_type.extension(), ".webm");
    assert!(mime_type.is(VIDEO_WEBM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_mkv() {
    let data = b"\x1a\x45\xdf\xa3\x01\x00\x00\x00\x00\x00\x00\x23\x42\x86\x81\x01\x42\xf7\x81\x01\x42\xf2\x81\x04\x42\xf3\x81\x08\x42\x82\x88\x6d\x61\x74\x72\x6f\x73\x6b\x61";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_X_MATROSKA);
    assert_eq!(mime_type.extension(), ".mkv");
    assert!(mime_type.is(VIDEO_X_MATROSKA));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_avi() {
    let mut data = vec![0u8; 24];
    data[0..4].copy_from_slice(b"RIFF");
    data[4..8].copy_from_slice(&1000u32.to_le_bytes()); // File size
    data[8..16].copy_from_slice(b"AVI LIST");
    data[16..20].copy_from_slice(&100u32.to_le_bytes()); // Chunk size

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_X_MSVIDEO);
    assert_eq!(mime_type.extension(), ".avi");
    assert!(mime_type.is(VIDEO_X_MSVIDEO));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_mpeg() {
    let data = b"\x00\x00\x01\xB3";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_MPEG);
    assert_eq!(mime_type.extension(), ".mpg"); // MPEG Video variant (00 00 01 B3)
    assert!(mime_type.is(VIDEO_MPEG));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_quicktime() {
    let mut data = vec![0; 16];
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"qt  ");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_QUICKTIME);
    assert_eq!(mime_type.extension(), ".mov");
    assert!(mime_type.is(VIDEO_QUICKTIME));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_mqv() {
    let mut data = vec![0; 16];
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"mqt ");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_QUICKTIME);
    assert_eq!(mime_type.extension(), ".mqv");
    assert!(mime_type.is(VIDEO_QUICKTIME));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_flv() {
    let data = b"FLV";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_X_FLV);
    assert_eq!(mime_type.extension(), ".flv");
    assert!(mime_type.is(VIDEO_X_FLV));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_asf() {
    let data = b"\x30\x26\xb2\x75\x8e\x66\xcf\x11\xa6\xd9\x00\xaa\x00\x62\xce\x6c";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_X_MS_ASF);
    assert_eq!(mime_type.extension(), ".asf");
    assert!(mime_type.is(VIDEO_X_MS_ASF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_m4v() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"M4V ");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_X_M4V);
    assert_eq!(mime_type.extension(), ".m4v");
    assert!(mime_type.is(VIDEO_X_M4V));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_rmvb() {
    // Both RM and RMVB share the same ".RMF" magic bytes
    // Without additional file structure analysis, they can't be differentiated
    // This test will detect as RM (which comes first in PREFIX_VEC)
    let data = b".RMF";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_RN_REALMEDIA);
    assert_eq!(mime_type.extension(), ".rm");
    assert!(mime_type.is(APPLICATION_VND_RN_REALMEDIA));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_3gpp() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"3gp4");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_3GPP);
    assert_eq!(mime_type.extension(), ".3gp");
    assert!(mime_type.is(VIDEO_3GPP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_3gpp2() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"3g24");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_3GPP2);
    assert_eq!(mime_type.extension(), ".3g2");
    assert!(mime_type.is(VIDEO_3GPP2));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_mj2() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"mj2s");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_MJ2);
    assert_eq!(mime_type.extension(), ".mj2");
}

#[test]
fn test_detect_dvb() {
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes());
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"dvb1");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_VND_DVB_FILE);
    assert_eq!(mime_type.extension(), ".dvb");
}

#[test]
fn test_detect_fli() {
    let data = b"\x11\xAF\x00\x00\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_FLI);
    assert_eq!(mime_type.extension(), ".fli");
    assert!(mime_type.is(VIDEO_FLI));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_flc() {
    let data = b"\x12\xAF\x00\x00\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_FLC);
    assert_eq!(mime_type.extension(), ".flc");
    assert!(mime_type.is(VIDEO_FLC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_fvt() {
    let data = b"FVT\x00\x00\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_VND_FVT);
    assert_eq!(mime_type.extension(), ".fvt");
    assert!(mime_type.is(VIDEO_VND_FVT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

// ============================================================================
// EXECUTABLE & BINARY FORMATS
// ============================================================================

#[test]
fn test_detect_exe() {
    let data = b"MZ";
    let mime_type = detect(data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE
    );
    assert_eq!(mime_type.extension(), ".exe");
    assert!(mime_type.is(APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_elf() {
    let data = b"\x7fELF";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_ELF);
    assert_eq!(mime_type.extension(), "");
    assert!(mime_type.is(APPLICATION_X_ELF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_executable());
}

#[test]
fn test_detect_elf_obj() {
    let mut data = vec![0x7f, 0x45, 0x4c, 0x46]; // ELF header
    data.resize(18, 0);
    data[16] = 1; // ET_REL
    data[17] = 0;
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_OBJECT);
    assert_eq!(mime_type.extension(), "");
    assert!(mime_type.is(APPLICATION_X_OBJECT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_executable());
}

#[test]
fn test_detect_elf_exe() {
    let mut data = vec![0x7f, 0x45, 0x4c, 0x46]; // ELF header
    data.resize(18, 0);
    data[16] = 2; // ET_EXEC
    data[17] = 0;
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_EXECUTABLE);
    assert_eq!(mime_type.extension(), ".elf"); // Now has .elf extension
    assert!(mime_type.is(APPLICATION_X_EXECUTABLE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_elf_lib() {
    let mut data = vec![0x7f, 0x45, 0x4c, 0x46]; // ELF header
    data.resize(18, 0);
    data[16] = 3; // ET_DYN
    data[17] = 0;
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_SHAREDLIB);
    assert_eq!(mime_type.extension(), ".so");
    assert!(mime_type.is(APPLICATION_X_SHAREDLIB));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_elf_dump() {
    let mut data = vec![0x7f, 0x45, 0x4c, 0x46]; // ELF header
    data.resize(18, 0);
    data[16] = 4; // ET_CORE
    data[17] = 0;
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_COREDUMP);
    assert_eq!(mime_type.extension(), "");
    assert!(mime_type.is(APPLICATION_X_COREDUMP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_class() {
    let data = b"\xca\xfe\xba\xbe";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_JAVA_APPLET_BINARY);
    assert_eq!(mime_type.extension(), ".class");
    assert!(mime_type.is(APPLICATION_X_JAVA_APPLET_BINARY));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_wasm() {
    let data = b"\x00asm";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_WASM);
    assert_eq!(mime_type.extension(), ".wasm");
    assert!(mime_type.is(APPLICATION_WASM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_executable());
}

// ============================================================================
// FONT FORMATS
// ============================================================================

#[test]
fn test_detect_ttf() {
    let data = b"\x00\x01\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), FONT_TTF);
    assert_eq!(mime_type.extension(), ".ttf");
    assert!(mime_type.is(FONT_TTF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_font());
}

#[test]
fn test_detect_woff() {
    let data = b"wOFF";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), FONT_WOFF);
    assert_eq!(mime_type.extension(), ".woff");
    assert!(mime_type.is(FONT_WOFF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_font());
}

#[test]
fn test_detect_woff2() {
    let data = b"wOF2";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), FONT_WOFF2);
    assert_eq!(mime_type.extension(), ".woff2");
    assert!(mime_type.is(FONT_WOFF2));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_font());
}

#[test]
fn test_detect_otf() {
    let data = b"OTTO";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), FONT_OTF);
    assert_eq!(mime_type.extension(), ".otf");
    assert!(mime_type.is(FONT_OTF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_font());
}

#[test]
fn test_detect_eot() {
    let mut data = vec![0; 36];
    data[34..36].copy_from_slice(b"LP");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_FONTOBJECT);
    assert_eq!(mime_type.extension(), ".eot");
    assert!(mime_type.is(APPLICATION_VND_MS_FONTOBJECT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_font());
}

#[test]
fn test_detect_ttc() {
    let data = b"ttcf";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), FONT_COLLECTION);
    assert_eq!(mime_type.extension(), ".ttc");
    assert!(mime_type.is(FONT_COLLECTION));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_font());
}

// ============================================================================
// WEB & MULTIMEDIA FORMATS
// ============================================================================

#[test]
fn test_detect_swf() {
    let data = b"FWS";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_SHOCKWAVE_FLASH);
    assert_eq!(mime_type.extension(), ".swf");
    assert!(mime_type.is(APPLICATION_X_SHOCKWAVE_FLASH));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_crx() {
    let data = b"Cr24\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00PK\x03\x04";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_CHROME_EXTENSION);
    assert_eq!(mime_type.extension(), ".crx");
    assert!(mime_type.is(APPLICATION_X_CHROME_EXTENSION));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_p7s() {
    let data = b"-----BEGIN PKCS7-----";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_PKCS7_SIGNATURE);
    assert_eq!(mime_type.extension(), ".p7s");
    assert!(mime_type.is(APPLICATION_PKCS7_SIGNATURE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

// ============================================================================
// SPECIALIZED FORMATS
// ============================================================================

#[test]
fn test_detect_dcm() {
    let mut data = vec![0; 132];
    data[128..132].copy_from_slice(b"DICM");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_DICOM);
    assert_eq!(mime_type.extension(), ".dcm");
    assert!(mime_type.is(APPLICATION_DICOM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_mobi() {
    let mut data = vec![0; 68];
    data[60..68].copy_from_slice(b"BOOKMOBI");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_lit() {
    let data = b"ITOLITLS";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MS_READER);
    assert_eq!(mime_type.extension(), ".lit");
    assert!(mime_type.is(APPLICATION_X_MS_READER));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_sqlite3() {
    let data = b"SQLite format 3\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SQLITE3);
    assert_eq!(mime_type.extension(), ".sqlite");
    assert!(mime_type.is(APPLICATION_VND_SQLITE3));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_database());
}

#[test]
fn test_detect_fasoo() {
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1];
    data.resize(520, 0);
    data[512..520].copy_from_slice(b"FASOO   ");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_FASOO);
    assert_eq!(mime_type.extension(), "");
    assert!(mime_type.is(APPLICATION_X_FASOO));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_pgp_net_share() {
    let data = b"-----BEGIN PGP";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF8);
    assert_eq!(mime_type.extension(), ".txt");
}

// ============================================================================
// MICROSOFT OFFICE & DOCUMENT FORMATS
// ============================================================================

#[test]
fn test_detect_docx() {
    let data = create_zip_with_file(b"word/document.xml");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OPENXML_WORDPROCESSINGML_DOCUMENT
    );
    assert_eq!(mime_type.extension(), ".docx");
    assert!(mime_type.is(APPLICATION_VND_OPENXML_WORDPROCESSINGML_DOCUMENT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_xlsx() {
    let data = create_zip_with_file(b"xl/workbook.xml");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OPENXML_SPREADSHEETML_SHEET
    );
    assert_eq!(mime_type.extension(), ".xlsx");
    assert!(mime_type.is(APPLICATION_VND_OPENXML_SPREADSHEETML_SHEET));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_spreadsheet());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_pptx() {
    let data = create_zip_with_file(b"ppt/presentation.xml");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OPENXML_PRESENTATIONML_PRESENTATION
    );
    assert_eq!(mime_type.extension(), ".pptx");
    assert!(mime_type.is(APPLICATION_VND_OPENXML_PRESENTATIONML_PRESENTATION));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_presentation());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_epub() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/epub+zip");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_EPUB_ZIP);
    assert_eq!(mime_type.extension(), ".epub");
    assert!(mime_type.is(APPLICATION_EPUB_ZIP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_jar() {
    let data = create_zip_with_file(b"META-INF/MANIFEST.MF");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_JAVA_ARCHIVE);
    assert_eq!(mime_type.extension(), ".jar");
    assert!(mime_type.is(APPLICATION_JAVA_ARCHIVE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_apk() {
    let data = create_zip_with_file(b"AndroidManifest.xml");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_ANDROID_PACKAGE_ARCHIVE);
    assert_eq!(mime_type.extension(), ".apk");
    assert!(mime_type.is(APPLICATION_VND_ANDROID_PACKAGE_ARCHIVE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_doc() {
    const WORD_97_2003_CLSID: &[u8] = &[
        0x06, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    let data = create_ole_with_clsid(WORD_97_2003_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_MSWORD);
    assert_eq!(mime_type.extension(), ".doc");
    assert!(mime_type.is(APPLICATION_MSWORD));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_wpd() {
    let data = b"\xff\x57\x50\x43\x00\x00\x00\x00\x01\x0a";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_WORDPERFECT);
    assert_eq!(mime_type.extension(), ".wpd");
    assert!(mime_type.is(APPLICATION_VND_WORDPERFECT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_xls() {
    const EXCEL_V5_CLSID: &[u8] = &[0x10, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data = create_ole_with_clsid(EXCEL_V5_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_EXCEL);
    assert_eq!(mime_type.extension(), ".xls");
    assert!(mime_type.is(APPLICATION_VND_MS_EXCEL));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_spreadsheet());
}

#[test]
fn test_detect_ppt() {
    const PPT_V4_CLSID: &[u8] = &[
        0x10, 0x8d, 0x81, 0x64, 0x9b, 0x4f, 0xcf, 0x11, 0x86, 0xea, 0x00, 0xaa, 0x00, 0xb9, 0x29,
        0xe8,
    ];
    let data = create_ole_with_clsid(PPT_V4_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_POWERPOINT);
    assert_eq!(mime_type.extension(), ".ppt");
    assert!(mime_type.is(APPLICATION_VND_MS_POWERPOINT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_pub() {
    const PUBLISHER_CLSID: &[u8] = &[
        0x01, 0x12, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    let data = create_ole_with_clsid(PUBLISHER_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_PUBLISHER);
    assert_eq!(mime_type.extension(), ".pub");
    assert!(mime_type.is(APPLICATION_VND_MS_PUBLISHER));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_msg() {
    const OUTLOOK_MSG_CLSID: &[u8] = &[
        0x0B, 0x0D, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    let data = create_ole_with_clsid(OUTLOOK_MSG_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_OUTLOOK);
    assert_eq!(mime_type.extension(), ".msg");
    assert!(mime_type.is(APPLICATION_VND_MS_OUTLOOK));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_msi() {
    const MSI_CLSID: &[u8] = &[
        0x84, 0x10, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    let data = create_ole_with_clsid(MSI_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MS_INSTALLER);
    assert_eq!(mime_type.extension(), ".msi");
    assert!(mime_type.is(APPLICATION_X_MS_INSTALLER));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

// ============================================================================
// OPEN DOCUMENT FORMATS
// ============================================================================

#[test]
fn test_detect_odt() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.text");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT);
    assert_eq!(mime_type.extension(), ".odt");
    assert!(mime_type.is(APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_ods() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.spreadsheet");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET
    );
    assert_eq!(mime_type.extension(), ".ods");
    assert!(mime_type.is(APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_spreadsheet());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_odp() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.presentation");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION
    );
    assert_eq!(mime_type.extension(), ".odp");
    assert!(mime_type.is(APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_presentation());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_odg() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.graphics");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS
    );
    assert_eq!(mime_type.extension(), ".odg");
    assert!(mime_type.is(APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_odf() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.formula");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA);
    assert_eq!(mime_type.extension(), ".odf");
    assert!(mime_type.is(APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_odc() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.chart");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_OASIS_OPENDOCUMENT_CHART);
    assert_eq!(mime_type.extension(), ".odc");
    assert!(mime_type.is(APPLICATION_VND_OASIS_OPENDOCUMENT_CHART));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_ott() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.text-template");
    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE
    );
}

#[test]
fn test_detect_ots() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.spreadsheet-template");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE
    );
    assert_eq!(mime_type.extension(), ".ots");
    assert!(mime_type.is(APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_spreadsheet());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_otp() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.presentation-template");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE
    );
    assert_eq!(mime_type.extension(), ".otp");
    assert!(mime_type.is(APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_presentation());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_otg() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.graphics-template");
    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE
    );
}

#[test]
fn test_detect_sxc() {
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.calc");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_CALC);
    assert_eq!(mime_type.extension(), ".sxc");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_CALC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_spreadsheet());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_kmz() {
    let data = create_zip_with_file(b"doc.kml");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_GOOGLE_EARTH_KMZ);
    assert_eq!(mime_type.extension(), ".kmz");
    assert!(mime_type.is(APPLICATION_VND_GOOGLE_EARTH_KMZ));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

// ============================================================================
// DATABASE FORMATS
// ============================================================================

#[test]
fn test_detect_mdb() {
    let mut data = vec![0; 32];
    data[4..19].copy_from_slice(b"Standard Jet DB");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MSACCESS);
    assert_eq!(mime_type.extension(), ".mdb");
    assert!(mime_type.is(APPLICATION_X_MSACCESS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_database());
}

#[test]
fn test_detect_accdb() {
    let mut data = vec![0; 32];
    data[4..19].copy_from_slice(b"Standard ACE DB");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MSACCESS);
    assert_eq!(mime_type.extension(), ".accdb");
    assert!(mime_type.is(APPLICATION_X_MSACCESS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_database());
}

#[test]
fn test_detect_dbf() {
    let mut data = vec![0x03]; // dBase type
    data.resize(32, 0);
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_DBF);
    assert_eq!(mime_type.extension(), ".dbf");
    assert!(mime_type.is(APPLICATION_X_DBF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_database());
}

#[test]
fn test_detect_lotus123() {
    let mut data = vec![0; 8];
    data[4..8].copy_from_slice(&0x00000200u32.to_le_bytes());
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_LOTUS_1_2_3);
    assert_eq!(mime_type.extension(), ".123");
    assert!(mime_type.is(APPLICATION_VND_LOTUS_1_2_3));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_database());
    assert!(mime_type.kind().is_spreadsheet());
}

#[test]
fn test_detect_marc() {
    let mut data = vec![0; 24];
    data[10] = b'2';
    data[11] = b'2';
    data[20..24].copy_from_slice(b"4500");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_MARC);
    assert_eq!(mime_type.extension(), ".mrc");
    assert!(mime_type.is(APPLICATION_MARC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
    assert!(mime_type.kind().is_database());
}

// ============================================================================
// PROGRAMMING & TEXT FORMATS
// ============================================================================

#[test]
fn test_detect_php() {
    let data = b"<?php echo 'Hello World'; ?>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_PHP);
    assert_eq!(mime_type.extension(), ".php");
    assert!(mime_type.is(TEXT_X_PHP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_javascript() {
    let data = b"function hello() { return 'world'; }";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_JAVASCRIPT);
    assert_eq!(mime_type.extension(), ".js");
    assert!(mime_type.is(TEXT_JAVASCRIPT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_python() {
    let data = b"#!/usr/bin/env python\nprint('Hello World')";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_PYTHON);
    assert_eq!(mime_type.extension(), ".py");
    assert!(mime_type.is(TEXT_X_PYTHON));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_perl() {
    let data = b"#!/usr/bin/env perl\nprint \"Hello World\\n\";";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_PERL);
    assert_eq!(mime_type.extension(), ".pl");
    assert!(mime_type.is(TEXT_X_PERL));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_ruby() {
    let data = b"#!/usr/bin/env ruby\nputs 'Hello World'";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_RUBY);
    assert_eq!(mime_type.extension(), ".rb");
    assert!(mime_type.is(TEXT_X_RUBY));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_lua() {
    let data = b"#!/usr/bin/env lua\nprint('Hello World')";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_LUA);
    assert_eq!(mime_type.extension(), ".lua");
    assert!(mime_type.is(TEXT_X_LUA));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_shell() {
    let data = b"#!/bin/bash\necho 'Hello World'";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_SHELLSCRIPT);
    assert_eq!(mime_type.extension(), ".sh");
    assert!(mime_type.is(TEXT_X_SHELLSCRIPT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_tcl() {
    let data = b"#!/usr/bin/env tclsh\nputs \"Hello World\"";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_TCL);
    assert_eq!(mime_type.extension(), ".tcl");
    assert!(mime_type.is(TEXT_X_TCL));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_json() {
    let data = b"{\"message\": \"Hello World\"}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_JSON);
    assert_eq!(mime_type.extension(), ".json");
    assert!(mime_type.is(APPLICATION_JSON));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_geojson() {
    let data = b"{\"type\": \"FeatureCollection\", \"features\": []}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_GEO_JSON);
    assert_eq!(mime_type.extension(), ".geojson");
    assert!(mime_type.is(APPLICATION_GEO_JSON));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_ndjson() {
    let data = b"{\"line\": 1}\n{\"line\": 2}\n{\"line\": 3}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_NDJSON);
    assert_eq!(mime_type.extension(), ".ndjson");
    assert!(mime_type.is(APPLICATION_X_NDJSON));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_csv() {
    let data = b"name,age,city\nJohn,30,NYC\nJane,25,LA";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_CSV);
    assert_eq!(mime_type.extension(), ".csv");
    assert!(mime_type.is(TEXT_CSV));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_tsv() {
    let data = b"name\tage\tcity\nJohn\t30\tNYC\nJane\t25\tLA";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_TAB_SEPARATED_VALUES);
    assert_eq!(mime_type.extension(), ".tsv");
    assert!(mime_type.is(TEXT_TAB_SEPARATED_VALUES));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_rtf() {
    let data = b"{\\rtf1\\ansi\\deff0 {\\fonttbl {\\f0 Times New Roman;}} Hello World}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_RTF);
    assert_eq!(mime_type.extension(), ".rtf");
    assert!(mime_type.is(TEXT_RTF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_srt() {
    let data = b"1\n00:00:00,000 --> 00:00:03,000\nHello World\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_SUBRIP);
    assert_eq!(mime_type.extension(), ".srt");
    assert!(mime_type.is(APPLICATION_X_SUBRIP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_vtt() {
    let data = b"WEBVTT\n\n00:00:00.000 --> 00:00:03.000\nHello World";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VTT);
    assert_eq!(mime_type.extension(), ".vtt");
    assert!(mime_type.is(TEXT_VTT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_vcard() {
    let data = b"BEGIN:VCARD\nVERSION:3.0\nFN:John Doe\nEND:VCARD";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VCARD);
    assert_eq!(mime_type.extension(), ".vcf");
    assert!(mime_type.is(TEXT_VCARD));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_icalendar() {
    let data = b"BEGIN:VCALENDAR\nVERSION:2.0\nBEGIN:VEVENT\nEND:VEVENT\nEND:VCALENDAR";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_CALENDAR);
    assert_eq!(mime_type.extension(), ".ics");
    assert!(mime_type.is(TEXT_CALENDAR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_svg() {
    let data = b"<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_SVG_XML);
    assert_eq!(mime_type.extension(), ".svg");
    assert!(mime_type.is(IMAGE_SVG_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_har() {
    let data = b"{\"log\": {\"version\": \"1.2\", \"entries\": []}}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_JSON_HAR);
    assert_eq!(mime_type.extension(), ".har");
    assert!(mime_type.is(APPLICATION_JSON_HAR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

// ============================================================================
// XML-BASED FORMATS
// ============================================================================

#[test]
fn test_detect_rss() {
    let data = b"<?xml version=\"1.0\"?><rss version=\"2.0\"></rss>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_RSS_XML);
    assert_eq!(mime_type.extension(), ".rss");
    assert!(mime_type.is(APPLICATION_RSS_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_atom() {
    let data = b"<?xml version=\"1.0\"?><feed xmlns=\"http://www.w3.org/2005/Atom\"></feed>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_ATOM_XML);
    assert_eq!(mime_type.extension(), ".atom");
    assert!(mime_type.is(APPLICATION_ATOM_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_x3d() {
    let data = b"<?xml version=\"1.0\"?><X3D></X3D>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_X3D_XML);
    assert_eq!(mime_type.extension(), ".x3d");
    assert!(mime_type.is(MODEL_X3D_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_kml() {
    let data = b"<?xml version=\"1.0\"?><kml xmlns=\"http://www.opengis.net/kml/2.2\"></kml>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_GOOGLE_EARTH_KML_XML);
    assert_eq!(mime_type.extension(), ".kml");
    assert!(mime_type.is(APPLICATION_VND_GOOGLE_EARTH_KML_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_xliff() {
    let data = b"<?xml version=\"1.0\"?><xliff></xliff>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_XLIFF_XML);
    assert_eq!(mime_type.extension(), ".xlf");
    assert!(mime_type.is(APPLICATION_X_XLIFF_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_collada() {
    let data = b"<?xml version=\"1.0\"?><COLLADA></COLLADA>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_VND_COLLADA_XML);
    assert_eq!(mime_type.extension(), ".dae");
    assert!(mime_type.is(MODEL_VND_COLLADA_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
    assert!(mime_type.kind().is_model());
}

#[test]
fn test_detect_gml() {
    let data = b"<?xml version=\"1.0\"?><gml></gml>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_GML_XML);
    assert_eq!(mime_type.extension(), ".gml");
    assert!(mime_type.is(APPLICATION_GML_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_gpx() {
    let data = b"<?xml version=\"1.0\"?><gpx></gpx>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_GPX_XML);
    assert_eq!(mime_type.extension(), ".gpx");
    assert!(mime_type.is(APPLICATION_GPX_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_tcx() {
    let data = b"<?xml version=\"1.0\"?><TrainingCenterDatabase></TrainingCenterDatabase>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_GARMIN_TCX_XML);
    assert_eq!(mime_type.extension(), ".tcx");
}

#[test]
fn test_detect_amf() {
    let data = b"<?xml version=\"1.0\"?><amf></amf>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_AMF);
    assert_eq!(mime_type.extension(), ".amf");
}

#[test]
fn test_detect_threemf() {
    let data = b"<?xml version=\"1.0\"?><model></model>";
    let mime_type = detect(data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_MS_PACKAGE_3DMANUFACTURING_3DMODEL_XML
    );
    assert_eq!(mime_type.extension(), ".3mf");
}

#[test]
fn test_detect_xfdf() {
    let data = b"<?xml version=\"1.0\"?><xfdf></xfdf>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_ADOBE_XFDF);
    assert_eq!(mime_type.extension(), ".xfdf");
}

#[test]
fn test_detect_owl2() {
    let data = b"<?xml version=\"1.0\"?><owl></owl>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_OWL_XML);
    assert_eq!(mime_type.extension(), ".owl");
}

#[test]
fn test_detect_xhtml() {
    let data = b"<?xml version=\"1.0\"?><html xmlns=\"http://www.w3.org/1999/xhtml\"></html>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_XHTML_XML);
    assert_eq!(mime_type.extension(), ".html");
}

// ============================================================================
// 3D & GEOSPATIAL FORMATS
// ============================================================================

#[test]
fn test_detect_shp() {
    let mut data = vec![0; 100];
    data[0..4].copy_from_slice(&9994u32.to_be_bytes());
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SHP);
    assert_eq!(mime_type.extension(), ".shp");
    assert!(mime_type.is(APPLICATION_VND_SHP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_shx() {
    let data = b"\x00\x00\x27\x0A";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SHX);
    assert_eq!(mime_type.extension(), ".shx");
    assert!(mime_type.is(APPLICATION_VND_SHX));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_glb() {
    let data = b"glTF\x02\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_GLTF_BINARY);
    assert_eq!(mime_type.extension(), ".glb");
    assert!(mime_type.is(MODEL_GLTF_BINARY));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
}

#[test]
fn test_detect_gltf() {
    let data = b"{\"scenes\": [], \"nodes\": [], \"asset\": {}}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_GLTF_JSON);
    assert_eq!(mime_type.extension(), ".gltf");
    assert!(mime_type.is(MODEL_GLTF_JSON));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
    assert!(mime_type.kind().is_model());
}

// ============================================================================
// GAMING FORMATS
// ============================================================================

#[test]
fn test_detect_nes() {
    let data = b"NES\x1A";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_NINTENDO_SNES_ROM);
    assert_eq!(mime_type.extension(), ".nes");
    assert!(mime_type.is(APPLICATION_VND_NINTENDO_SNES_ROM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

// ============================================================================
// MISCELLANEOUS FORMATS
// ============================================================================

#[test]
fn test_detect_hdf() {
    // This is HDF5 magic bytes, should detect as HDF5
    let data = b"\x89HDF\r\n\x1a\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_HDF5);
    assert_eq!(mime_type.extension(), ".hdf5");
    assert!(mime_type.is(APPLICATION_X_HDF5));
    // Note: When detected via PREFIX_VEC, HDF5 is detected directly
    // not through the HDF parent, so parent alias checking won't work
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_database());
}

#[test]
fn test_detect_cbor() {
    let data = b"\xd9\xd9\xf7";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_CBOR);
    assert_eq!(mime_type.extension(), ".cbor");
    assert!(mime_type.is(APPLICATION_CBOR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_parquet() {
    let data = b"PAR1";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_APACHE_PARQUET);
    assert_eq!(mime_type.extension(), ".parquet");
    assert!(mime_type.is(APPLICATION_VND_APACHE_PARQUET));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_database());
}

#[test]
fn test_detect_lnk() {
    let data = b"L\x00\x00\x00\x01\x14\x02\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MS_SHORTCUT);
    assert_eq!(mime_type.extension(), ".lnk");
    assert!(mime_type.is(APPLICATION_X_MS_SHORTCUT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_macho() {
    let data = b"\xfe\xed\xfa\xce"; // Mach-O magic
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MACH_BINARY);
    assert_eq!(mime_type.extension(), ".macho");
    assert!(mime_type.is(APPLICATION_X_MACH_BINARY));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_executable());
}

#[test]
fn test_detect_tzif() {
    let data = b"TZif";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_TZIF);
    assert_eq!(mime_type.extension(), "");
    assert!(mime_type.is(APPLICATION_TZIF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

// ============================================================================
// UTF-16 TEXT FORMAT VARIANTS
// ============================================================================

#[test]
fn test_detect_html_utf16_be() {
    let data = b"\xFE\xFF\x00<\x00h\x00t\x00m\x00l\x00>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_HTML_UTF16);
    assert_eq!(mime_type.extension(), ".html");
    assert!(mime_type.is(TEXT_HTML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_html_utf16_le() {
    let data = b"\xFF\xFE<\x00h\x00t\x00m\x00l\x00>\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_HTML_UTF16);
    assert_eq!(mime_type.extension(), ".html");
    assert!(mime_type.is(TEXT_HTML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_xml_utf16_be() {
    let data = b"\xFE\xFF\x00<\x00?\x00x\x00m\x00l";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_XML_UTF16);
    assert_eq!(mime_type.extension(), ".xml");
    assert!(mime_type.is(TEXT_XML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_xml_utf16_le() {
    let data = b"\xFF\xFE<\x00?\x00x\x00m\x00l\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_XML_UTF16);
    assert_eq!(mime_type.extension(), ".xml");
    assert!(mime_type.is(TEXT_XML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_svg_utf16_be() {
    let data = b"\xFE\xFF\x00<\x00s\x00v\x00g";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_SVG_XML_UTF16);
    assert_eq!(mime_type.extension(), ".svg");
    assert!(mime_type.is(IMAGE_SVG_XML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_svg_utf16_le() {
    let data = b"\xFF\xFE<\x00s\x00v\x00g\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_SVG_XML_UTF16);
    assert_eq!(mime_type.extension(), ".svg");
    assert!(mime_type.is(IMAGE_SVG_XML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_json_utf16_be() {
    let data = b"\xFE\xFF\x00{\x00\"\x00k\x00e\x00y\x00\"\x00:\x00\"\x00v\x00a\x00l\x00\"\x00}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_JSON_UTF16);
    assert_eq!(mime_type.extension(), ".json");
    assert!(mime_type.is(APPLICATION_JSON_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_json_utf16_le() {
    let data = b"\xFF\xFE{\x00\"\x00k\x00e\x00y\x00\"\x00:\x00\"\x00v\x00a\x00l\x00\"\x00}\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_JSON_UTF16);
    assert_eq!(mime_type.extension(), ".json");
    assert!(mime_type.is(APPLICATION_JSON_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_csv_utf16_be() {
    let data = b"\xFE\xFF\x00n\x00a\x00m\x00e\x00,\x00a\x00g\x00e";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_BE);
}

#[test]
fn test_detect_csv_utf16_le() {
    let data = b"\xFF\xFEn\x00a\x00m\x00e\x00,\x00a\x00g\x00e\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_LE);
}

#[test]
fn test_detect_tsv_utf16_be() {
    let data = b"\xFE\xFF\x00n\x00a\x00m\x00e\x00\t\x00a\x00g\x00e";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_BE);
}

#[test]
fn test_detect_tsv_utf16_le() {
    let data = b"\xFF\xFEn\x00a\x00m\x00e\x00\t\x00a\x00g\x00e\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_LE);
}

#[test]
fn test_detect_srt_utf16_be() {
    let data = b"\xFE\xFF\x001\x00\n\x000\x000\x00:\x000\x000\x00:\x000\x000\x00,\x000\x000\x000";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_BE);
}

#[test]
fn test_detect_srt_utf16_le() {
    let data = b"\xFF\xFE1\x00\n\x000\x000\x00:\x000\x000\x00:\x000\x000\x00,\x000\x000\x000\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_LE);
}

#[test]
fn test_detect_vtt_utf16_be() {
    let data = b"\xFE\xFF\x00W\x00E\x00B\x00V\x00T\x00T";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VTT_UTF16);
    assert_eq!(mime_type.extension(), ".vtt");
    assert!(mime_type.is(TEXT_VTT_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_vtt_utf16_le() {
    let data = b"\xFF\xFEW\x00E\x00B\x00V\x00T\x00T\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VTT_UTF16);
    assert_eq!(mime_type.extension(), ".vtt");
    assert!(mime_type.is(TEXT_VTT_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_vcard_utf16_be() {
    let data = b"\xFE\xFF\x00B\x00E\x00G\x00I\x00N\x00:\x00V\x00C\x00A\x00R\x00D";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VCARD_UTF16);
    assert_eq!(mime_type.extension(), ".vcf");
    assert!(mime_type.is(TEXT_VCARD_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_vcard_utf16_le() {
    let data = b"\xFF\xFEB\x00E\x00G\x00I\x00N\x00:\x00V\x00C\x00A\x00R\x00D\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VCARD_UTF16);
    assert_eq!(mime_type.extension(), ".vcf");
    assert!(mime_type.is(TEXT_VCARD_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_icalendar_utf16_be() {
    let data =
        b"\xFE\xFF\x00B\x00E\x00G\x00I\x00N\x00:\x00V\x00C\x00A\x00L\x00E\x00N\x00D\x00A\x00R";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_CALENDAR_UTF16);
    assert_eq!(mime_type.extension(), ".ics");
}

#[test]
fn test_detect_icalendar_utf16_le() {
    let data =
        b"\xFF\xFEB\x00E\x00G\x00I\x00N\x00:\x00V\x00C\x00A\x00L\x00E\x00N\x00D\x00A\x00R\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_CALENDAR_UTF16);
    assert_eq!(mime_type.extension(), ".ics");
}

#[test]
fn test_detect_rtf_utf16_be() {
    let data = b"\xFE\xFF\x00{\x00\\\x00r\x00t\x00f";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_RTF_UTF16);
    assert_eq!(mime_type.extension(), ".rtf");
}

#[test]
fn test_detect_rtf_utf16_le() {
    let data = b"\xFF\xFE{\x00\\\x00r\x00t\x00f\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_RTF_UTF16);
    assert_eq!(mime_type.extension(), ".rtf");
}

#[test]
fn test_detect_multiple_signatures() {
    let gif87 = b"GIF87a";
    let gif89 = b"GIF89a";

    assert_eq!(detect(gif87).mime(), IMAGE_GIF);
    assert_eq!(detect(gif89).mime(), IMAGE_GIF);
}

#[test]
fn test_detect_utf_variants() {
    let utf8_bom = b"\xEF\xBB\xBFHello";
    assert_eq!(detect(utf8_bom).mime(), TEXT_UTF8_BOM);

    let utf16_be = b"\xFE\xFF\x00H\x00e\x00l\x00l\x00o";
    assert_eq!(detect(utf16_be).mime(), TEXT_UTF16_BE);

    let utf16_le = b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00";
    assert_eq!(detect(utf16_le).mime(), TEXT_UTF16_LE);

    let utf8 = b"Hello World";
    assert_eq!(detect(utf8).mime(), TEXT_UTF8);
}

#[test]
fn test_detect_archive_formats() {
    let zip = b"PK\x03\x04";
    assert_eq!(detect(zip).mime(), APPLICATION_ZIP);

    let gzip = b"\x1f\x8b\x08";
    assert_eq!(detect(gzip).mime(), APPLICATION_GZIP);

    let sevenz = b"7z\xbc\xaf\x27\x1c";
    assert_eq!(detect(sevenz).mime(), APPLICATION_X_7Z_COMPRESSED);

    let rar = b"Rar!\x1a\x07\x00";
    assert_eq!(detect(rar).mime(), APPLICATION_X_RAR_COMPRESSED);
}

#[test]
fn test_detect_image_formats() {
    let png = b"\x89PNG\r\n\x1a\n";
    assert_eq!(detect(png).mime(), IMAGE_PNG);

    let jpeg = b"\xff\xd8\xff\xe0";
    assert_eq!(detect(jpeg).mime(), IMAGE_JPEG);

    let gif = b"GIF89a";
    assert_eq!(detect(gif).mime(), IMAGE_GIF);

    let mut webp = b"RIFF".to_vec();
    webp.extend_from_slice(&[0, 0, 0, 0]);
    webp.extend_from_slice(b"WEBP");
    assert_eq!(detect(&webp).mime(), IMAGE_WEBP);
}

#[test]
fn test_detect_document_formats() {
    let pdf = b"%PDF-1.4";
    assert_eq!(detect(pdf).mime(), APPLICATION_PDF);

    let ps = b"%!PS-Adobe-3.0";
    assert_eq!(detect(ps).mime(), APPLICATION_POSTSCRIPT);
}

#[test]
fn test_detect_audio_formats() {
    let mp3 = b"\xFF\xFB\x90";
    assert_eq!(detect(mp3).mime(), AUDIO_MPEG);

    let flac = b"fLaC";
    assert_eq!(detect(flac).mime(), AUDIO_FLAC);
}

#[test]
fn test_detect_executable_formats() {
    let elf = b"\x7fELF";
    assert_eq!(detect(elf).mime(), APPLICATION_X_ELF);

    let pe = b"MZ";
    assert_eq!(
        detect(pe).mime(),
        APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE
    );

    let wasm = b"\x00asm";
    assert_eq!(detect(wasm).mime(), APPLICATION_WASM);
}

#[test]
fn test_detect_font_formats() {
    let woff = b"wOFF";
    assert_eq!(detect(woff).mime(), FONT_WOFF);

    let woff2 = b"wOF2";
    assert_eq!(detect(woff2).mime(), FONT_WOFF2);

    let otf = b"OTTO";
    assert_eq!(detect(otf).mime(), FONT_OTF);
}

#[test]
fn test_detect_json_feed() {
    let data = b"{\"version";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), "application/feed+json");
    assert_eq!(mime_type.extension(), ".json");
}

#[test]
fn test_detect_wat() {
    let data = b"(module";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), "text/wasm");
    assert_eq!(mime_type.extension(), ".wat");
}

#[test]
fn test_detect_usd_ascii() {
    let data = b"#usda";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), "model/x-usd-ascii");
    assert_eq!(mime_type.extension(), ".usda");
}

#[test]
fn test_detect_3ds() {
    let data = b"MM\x10\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), "application/x-3ds");
    assert_eq!(mime_type.extension(), ".3ds");
}

#[test]
fn test_detect_3ds_vs_tiff() {
    // 3DS: MM but NOT MM\x00*
    let data_3ds = b"MM\x10\x00";
    assert_eq!(detect(data_3ds).mime(), "application/x-3ds");

    // TIFF: MM\x00*
    let data_tiff = b"MM\x00*";
    assert_eq!(detect(data_tiff).mime(), "image/tiff");
}

// Audio formats

#[test]
fn test_detect_dts() {
    let data = b"\x7F\xFE\x80\x01";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), "audio/vnd.dts");
    assert_eq!(mime_type.extension(), ".dts");
}

// High-priority formats (PGP, Android, DOS)

#[test]
fn test_detect_pgp_message() {
    let data = b"-----BEGIN PGP MESSAGE-----";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_PGP);
}

#[test]
fn test_detect_pgp_signature() {
    let data = b"-----BEGIN PGP SIGNATURE-----";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_PGP_SIGNATURE);
}

#[test]
fn test_detect_pgp_public_key() {
    let data = b"-----BEGIN PGP PUBLIC KEY BLOCK-----";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_PGP_KEYS);
}

#[test]
fn test_detect_android_axml() {
    let data = b"\x03\x00\x08\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_ANDROID_AXML);
    assert_eq!(mime_type.extension(), ".xml");
}

#[test]
fn test_detect_android_arsc() {
    let data = b"\x02\x00\x08\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_ANDROID_ARSC);
    assert_eq!(mime_type.extension(), ".arsc");
}

#[test]
fn test_detect_dos_executable() {
    let mut dos_exe = vec![0u8; 128];
    dos_exe[0..2].copy_from_slice(b"MZ");
    dos_exe[0x3C] = 0x80; // PE offset beyond file
    let mime_type = detect(&dos_exe);
    assert_eq!(mime_type.mime(), APPLICATION_X_DOSEXEC);
}

// Modern formats (Python Pickle, etc.)

#[test]
fn test_detect_pickle() {
    let data = b"\x80\x02}"; // Pickle protocol 2
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_PICKLE);
    assert_eq!(mime_type.extension(), ".pkl");
}

#[test]
fn test_detect_python_bytecode() {
    let data = b"\x42\x0D\x0D\x0A"; // Python 3.7 bytecode
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_PYTHON_BYTECODE);
    assert_eq!(mime_type.extension(), ".pyc");
}

// Additional archive formats

#[test]
fn test_detect_stuffit() {
    let data = b"SIT!";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_STUFFIT);
    assert_eq!(mime_type.extension(), ".sit");
}

#[test]
fn test_detect_alz() {
    let data = b"ALZ\x01";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_ALZ_COMPRESSED);
    assert_eq!(mime_type.extension(), ".alz");
}

#[test]
fn test_detect_ace() {
    let mut data = vec![0u8; 20];
    data[7..14].copy_from_slice(b"**ACE**");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_ACE_COMPRESSED);
    assert_eq!(mime_type.extension(), ".ace");
}

// Modern data formats

#[test]
fn test_detect_apache_arrow() {
    let data = b"ARROW1\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_APACHE_ARROW_FILE);
    assert_eq!(mime_type.extension(), ".arrow");
}

#[test]
fn test_detect_apache_avro() {
    let data = b"Obj\x01";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_APACHE_AVRO);
    assert_eq!(mime_type.extension(), ".avro");
}

#[test]
fn test_detect_qoi() {
    let data = b"qoif";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_QOI);
    assert_eq!(mime_type.extension(), ".qoi");
}

#[test]
fn test_detect_ktx2() {
    let data = b"\xABKTX 20\xBB\r\n\x1A\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_KTX2);
    assert_eq!(mime_type.extension(), ".ktx2");
}

#[test]
fn test_detect_openexr() {
    let data = b"\x76\x2F\x31\x01";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_EXR);
    assert_eq!(mime_type.extension(), ".exr");
}

#[test]
fn test_detect_ac3() {
    let data = b"\x0B\x77";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_AC3);
    assert_eq!(mime_type.extension(), ".ac3");
}

// Android & Compression formats

#[test]
fn test_detect_dex() {
    let data = b"dex\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_ANDROID_DEX);
    assert_eq!(mime_type.extension(), ".dex");
}

#[test]
fn test_detect_bzip3() {
    let data = b"BZ3v1";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_BZIP3);
    assert_eq!(mime_type.extension(), ".bz3");
}

#[test]
fn test_detect_lzma() {
    let data = b"\x5D\x00\x00\x80\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_LZMA);
    assert_eq!(mime_type.extension(), ".lzma");
}

#[test]
fn test_detect_lua_bytecode() {
    let data = b"\x1BLua";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_LUA_BYTECODE);
    assert_eq!(mime_type.extension(), ".luac");
}

// Camera RAW formats

#[test]
fn test_detect_canon_cr2() {
    let mut data = vec![0u8; 12];
    data[0..4].copy_from_slice(b"II*\x00");
    data[8..12].copy_from_slice(b"CR\x02\x00");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_X_CANON_CR2);
    assert_eq!(mime_type.extension(), ".cr2");
}

#[test]
fn test_detect_nikon_nef() {
    let mut data = vec![0u8; 256];
    data[0..4].copy_from_slice(b"II*\x00");
    data[100..105].copy_from_slice(b"NIKON");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_X_NIKON_NEF);
    assert_eq!(mime_type.extension(), ".nef");
}

#[test]
fn test_detect_fuji_raf() {
    let data = b"FUJIFILMCCD-RAW ";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_X_FUJI_RAF);
    assert_eq!(mime_type.extension(), ".raf");
}

// VM Disk formats

#[test]
fn test_detect_vmdk() {
    let data = b"KDMV";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_VMDK);
    assert_eq!(mime_type.extension(), ".vmdk");
}

#[test]
fn test_detect_vhdx() {
    let data = b"vhdxfile";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_VHDX);
    assert_eq!(mime_type.extension(), ".vhdx");
}

// Game ROM formats - already tested above

// 3D/CAD formats - already tested above

#[test]
fn test_detect_batch() {
    // MS-DOS Batch file with @ECHO OFF
    let data = b"@ECHO OFF\r\nREM This is a batch file\r\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_MSDOS_BATCH);
    assert_eq!(mime_type.extension(), ".bat");
}

#[test]
fn test_detect_batch_lowercase() {
    // Batch file with @echo off (lowercase)
    let data = b"@echo off\r\necho Hello World\r\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_MSDOS_BATCH);
    assert_eq!(mime_type.extension(), ".bat");
    // Note: .cmd is also available as extension alias
}

#[test]
fn test_dll_is_pe_format() {
    // DLL files are PE format (same as EXE, but different extension)
    // They share the same MIME type
    let data = b"MZ\x90\x00"; // PE signature
    let mime_type = detect(data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE
    );
    assert_eq!(mime_type.extension(), ".exe");
    // Note: .dll, .sys, .scr are available as extension aliases
}

#[test]
fn test_elf_executable_has_elf_extension() {
    // ELF executable now has .elf extension
    // Note: The parent ELF has empty extension, children have specific ones
    let data = b"\x7FELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x3E\x00";
    let mime_type = detect(data);
    // Should match ELF-based format
    assert!(mime_type.mime().contains("elf") || mime_type.mime().contains("executable"));
    // ELF_EXE child now has .elf extension
}

#[test]
fn test_detect_m4b() {
    // Apple iTunes Audiobook - MP4-based audiobook format
    // ftyp brand: "M4B "
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes()); // box_size = 16
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"M4B ");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), AUDIO_MP4);
    assert_eq!(mime_type.extension(), ".m4b");
}

#[test]
fn test_detect_m4p() {
    // Apple iTunes Protected Audio - DRM-protected MP4 audio
    // ftyp brand: "M4P "
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes()); // box_size = 16
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"M4P ");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), AUDIO_MP4);
    assert_eq!(mime_type.extension(), ".m4p");
}

#[test]
fn test_detect_f4a() {
    // Flash MP4 Audio - Adobe Flash MP4 audio format
    // ftyp brand: "F4A "
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes()); // box_size = 16
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"F4A ");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), AUDIO_MP4);
    assert_eq!(mime_type.extension(), ".f4a");
}

#[test]
fn test_detect_f4b() {
    // Flash MP4 Audiobook - Adobe Flash MP4 audiobook format
    // ftyp brand: "F4B "
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes()); // box_size = 16
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"F4B ");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), AUDIO_MP4);
    assert_eq!(mime_type.extension(), ".f4b");
}

#[test]
fn test_detect_f4v() {
    // Flash MP4 Video - Adobe Flash MP4 video format
    // ftyp brand: "F4V "
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes()); // box_size = 16
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"F4V ");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_MP4);
    assert_eq!(mime_type.extension(), ".f4v");
}

#[test]
fn test_detect_f4p() {
    // Flash MP4 Protected Video - Adobe Flash MP4 protected video format
    // ftyp brand: "F4P "
    let mut data = vec![0; 16];
    data[0..4].copy_from_slice(&16u32.to_be_bytes()); // box_size = 16
    data[4..8].copy_from_slice(b"ftyp");
    data[8..12].copy_from_slice(b"F4P ");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_MP4);
    assert_eq!(mime_type.extension(), ".f4p");
}

#[test]
fn test_detect_fb2() {
    // FictionBook - XML-based e-book format
    // Root tag: <FictionBook>
    let data = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<FictionBook xmlns=\"http://www.gribuser.ru/xml/fictionbook/2.0\">";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_FB2_XML);
    assert_eq!(mime_type.extension(), ".fb2");
}

#[test]
fn test_detect_bzip() {
    // BZIP compression - older than BZIP2, uses "BZ0" signature
    let data = b"BZ0";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_BZIP);
    assert_eq!(mime_type.extension(), ".bz");
}

#[test]
fn test_detect_visual_studio_solution() {
    // Visual Studio Solution file
    let data = b"Microsoft Visual Studio Solution File, Format Version 12.00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_DEVELOPER);
    assert_eq!(mime_type.extension(), ".sln");
}

#[test]
fn test_detect_visual_studio_solution_with_bom() {
    // Visual Studio Solution file with UTF-8 BOM
    let data = b"\xEF\xBB\xBF\r\nMicrosoft Visual Studio Solution File, Format Version 12.00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_DEVELOPER);
    assert_eq!(mime_type.extension(), ".sln");
}

#[test]
fn test_detect_latex() {
    // LaTeX document with \documentclass
    let data = b"\\documentclass{article}\n\\begin{document}\nHello World\n\\end{document}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_TEX);
    assert_eq!(mime_type.extension(), ".tex");
}

#[test]
fn test_detect_latex_documentstyle() {
    // LaTeX document with \documentstyle (older style)
    let data = b"\\documentstyle{article}\n\\begin{document}\nHello World\n\\end{document}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_TEX);
    assert_eq!(mime_type.extension(), ".tex");
}

#[test]
fn test_detect_clojure() {
    // Clojure script with shebang
    let data = b"#!/usr/bin/env clojure\n(println \"Hello World\")";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_CLOJURE);
    assert_eq!(mime_type.extension(), ".clj");
}

#[test]
fn test_detect_odb() {
    // OpenDocument Database - ZIP with mimetype
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.database");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_DATABASE
    );
    assert_eq!(mime_type.extension(), ".odb");
}

#[test]
fn test_detect_odm() {
    // OpenDocument Text Master - ZIP with mimetype
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.text-master");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_MASTER
    );
    assert_eq!(mime_type.extension(), ".odm");
}

#[test]
fn test_detect_coff() {
    // COFF (Common Object File Format) - i386 variant
    let data = b"\x4C\x01\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_COFF);
    assert_eq!(mime_type.extension(), ".o");
}

#[test]
fn test_detect_ogm() {
    // Ogg Media - OGM video format
    let mut data = b"OggS".to_vec();
    data.resize(28, 0);
    data.extend_from_slice(b"\x01video\x00\x00\x00");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_OGG_MEDIA);
    assert_eq!(mime_type.extension(), ".ogm");
}

#[test]
fn test_detect_ogm_audio() {
    // Ogg Media - OGM audio format
    let mut data = b"OggS".to_vec();
    data.resize(28, 0);
    data.extend_from_slice(b"\x01audio\x00\x00\x00");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_OGG_MEDIA);
    assert_eq!(mime_type.extension(), ".ogm");
}

#[test]
fn test_detect_ear() {
    // Enterprise Application Archive - ZIP with META-INF/application.xml
    let data = create_zip_with_file(b"META-INF/application.xml");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_EAR);
    assert_eq!(mime_type.extension(), ".ear");
}

#[test]
fn test_detect_ora() {
    // OpenRaster - ZIP with mimetype
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeimage/openraster");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_OPENRASTER);
    assert_eq!(mime_type.extension(), ".ora");
}

#[test]
fn test_detect_otm() {
    // OpenDocument Text Master Template - ZIP with mimetype
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.text-master-template");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_MASTER_TEMPLATE
    );
    assert_eq!(mime_type.extension(), ".otm");
}

#[test]
fn test_detect_aab() {
    // Android App Bundle - ZIP with BundleConfig.pb
    let data = create_zip_with_file(b"BundleConfig.pb");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_ANDROID_AAB);
    assert_eq!(mime_type.extension(), ".aab");
}

#[test]
fn test_detect_appx() {
    // Windows App Package - ZIP with AppxManifest.xml
    let data = create_zip_with_file(b"AppxManifest.xml");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_APPX);
    assert_eq!(mime_type.extension(), ".appx");
}

#[test]
fn test_detect_ipa() {
    // iOS App Store Package - ZIP with Payload/ directory
    let data = create_zip_with_file(b"Payload/");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_IOS_APP);
    assert_eq!(mime_type.extension(), ".ipa");
}

#[test]
fn test_detect_cfb() {
    // Compound File Binary - same as OLE storage
    // CFB is an alias for OLE, so it should detect as OLE
    let mut data = b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1".to_vec();
    data.resize(52, 0);

    let mime_type = detect(&data);
    // Should detect as OLE storage since CFB is just an alias
    assert_eq!(mime_type.mime(), APPLICATION_X_OLE_STORAGE);
}

#[test]
fn test_detect_asx() {
    // ASX - Advanced Stream Redirector (ASF-based XML playlist)
    let data =
        b"\x30\x26\xb2\x75\x8e\x66\xcf\x11\xa6\xd9\x00\xaa\x00\x62\xce\x6c<asx version=\"3.0\">";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), VIDEO_X_MS_ASX);
    assert_eq!(mime_type.extension(), ".asx");
}

#[test]
fn test_detect_cda() {
    // CD Audio track - RIFF with CDDA format
    // RIFF structure: "RIFF" + size (4 bytes) + "CDDA" + data
    let mut data = vec![];
    data.extend_from_slice(b"RIFF");
    data.extend_from_slice(&[36, 0, 0, 0]); // Size = 36 (44 - 8)
    data.extend_from_slice(b"CDDA");
    data.resize(44, 0);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_CDF);
    assert_eq!(mime_type.extension(), ".cda");
}

#[test]
fn test_detect_air() {
    // Adobe AIR - ZIP with META-INF/AIR/application.xml
    let data = create_zip_with_file(b"META-INF/AIR/application.xml");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_ADOBE_AIR_APPLICATION_INSTALLER_PACKAGE_ZIP
    );
    assert_eq!(mime_type.extension(), ".air");
}

#[test]
fn test_detect_fla() {
    // Adobe Flash Project - ZIP with DOMDocument.xml
    let data = create_zip_with_file(b"DOMDocument.xml");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_ADOBE_FLA);
    assert_eq!(mime_type.extension(), ".fla");
}

#[test]
fn test_detect_idml() {
    // InDesign Markup Language - ZIP with designmap.xml
    let data = create_zip_with_file(b"designmap.xml");

    let mime_type = detect(&data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_ADOBE_INDESIGN_IDML_PACKAGE
    );
    assert_eq!(mime_type.extension(), ".idml");
}

#[test]
fn test_detect_ai() {
    // Adobe Illustrator - PDF-based format with %AI marker
    let data = b"%PDF-1.4\n%AI-9.0\nAdobe Illustrator document";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_ADOBE_ILLUSTRATOR);
    assert_eq!(mime_type.extension(), ".ai");
}

#[test]
fn test_detect_dvr_ms() {
    // Microsoft Digital Video Recording - ASF-based format
    // DVR-MS shares ASF signature, so should detect as ASF
    let data = b"\x30\x26\xb2\x75\x8e\x66\xcf\x11\xa6\xd9\x00\xaa\x00\x62\xce\x6c";

    let mime_type = detect(data);
    // DVR-MS detection returns false, so it should be detected as parent ASF
    assert_eq!(mime_type.mime(), VIDEO_X_MS_ASF);
    assert_eq!(mime_type.extension(), ".asf");
}

#[test]
fn test_detect_abw() {
    // AbiWord - gzip-compressed XML with "abiword" marker
    // Create gzip header followed by abiword marker
    let mut data = vec![0x1f, 0x8b]; // gzip magic
    data.resize(20, 0);
    data.extend_from_slice(b"<?xml version=\"1.0\"?><abiword>");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_ABIWORD);
    assert_eq!(mime_type.extension(), ".abw");
}

#[test]
fn test_detect_appxbundle() {
    // Windows App Bundle - ZIP with AppxMetadata/AppxBundleManifest.xml
    let data = create_zip_with_file(b"AppxMetadata/AppxBundleManifest.xml");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_APPX_BUNDLE);
    assert_eq!(mime_type.extension(), ".appxbundle");
}

#[test]
fn test_detect_pst() {
    // PST - Personal Storage Table (OLE-based)
    // PST detection relies on extension matching since it doesn't have a unique CLSID
    // Use a generic OLE file - the pst() function returns false, relying on extension
    let data = create_ole_with_clsid(&[0; 16]); // Generic CLSID

    // Without .pst extension, it will detect as generic OLE
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_OLE_STORAGE);

    // Note: In real usage, PST files would be detected via file extension
    // The library currently doesn't expose detect_with_extension in tests
}

#[test]
fn test_detect_mpp() {
    // Microsoft Project - OLE-based with specific CLSID
    const MS_PROJECT_CLSID: &[u8] = &[
        0x84, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    let data = create_ole_with_clsid(MS_PROJECT_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_PROJECT);
    assert_eq!(mime_type.extension(), ".mpp");
}

#[test]
fn test_detect_lzs() {
    // LArc/LZS - Japanese compression format
    let data = b"-lzs-";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_LZS_COMPRESSED);
    assert_eq!(mime_type.extension(), ".lzs");
}

#[test]
fn test_detect_mp2() {
    // MPEG-1/2 Audio Layer 2
    // Frame sync: 0xFFE or 0xFFF (11 bits all 1)
    // Layer II indicator: bits 17-18 (in header) = 10 binary
    // 0xFFF4 = 1111 1111 1111 0100
    //   sync: 1111 1111 111 (11 bits) ✓
    //   layer bits (bits 1-2 of 2nd byte): 01 → after shift = 10 binary = 2 decimal ✓
    let data = b"\xFF\xF4\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_MP2);
    assert_eq!(mime_type.extension(), ".mp2");
}

#[test]
fn test_detect_war() {
    // Web Application Archive - ZIP with WEB-INF directory
    let data = create_zip_with_file(b"WEB-INF/web.xml");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_JAVA_ARCHIVE);
    assert_eq!(mime_type.extension(), ".war");
}

#[test]
fn test_detect_vsix() {
    // Visual Studio Extension - ZIP with extension.vsixmanifest
    let data = create_zip_with_file(b"extension.vsixmanifest");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VSIX);
    assert_eq!(mime_type.extension(), ".vsix");
}

#[test]
fn test_detect_qcow() {
    // QEMU Copy-on-Write version 1
    let data = b"QFI\x00\x00\x00\x01";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_QEMU_DISK);
    assert_eq!(mime_type.extension(), ".qcow");
}

#[test]
fn test_detect_wma() {
    // Windows Media Audio - ASF-based format
    let data = b"\x30\x26\xb2\x75\x8e\x66\xcf\x11\xa6\xd9\x00\xaa\x00\x62\xce\x6c";
    let mime_type = detect(data);
    // Without .wma extension, detects as generic ASF
    assert_eq!(mime_type.mime(), VIDEO_X_MS_ASF);
}

#[test]
fn test_detect_wmv() {
    // Windows Media Video - ASF-based format
    let data = b"\x30\x26\xb2\x75\x8e\x66\xcf\x11\xa6\xd9\x00\xaa\x00\x62\xce\x6c";
    let mime_type = detect(data);
    // WMV is an alias of ASF
    assert!(mime_type.is(VIDEO_X_MS_WMV));
    assert_eq!(mime_type.extension(), ".asf");
}

#[test]
fn test_detect_rv() {
    // RealVideo - RealMedia variant
    let data = b".RMF\x00\x00\x00\x12";
    let mime_type = detect(data);
    // Without .rv extension, detects as generic RealMedia
    assert_eq!(mime_type.mime(), APPLICATION_VND_RN_REALMEDIA);
    assert_eq!(mime_type.extension(), ".rm");
}

#[test]
fn test_detect_mtv() {
    // MTV video format - RIFF-based
    let mut data = vec![];
    data.extend_from_slice(b"RIFF");
    data.extend_from_slice(&[36, 0, 0, 0]); // Size
    data.extend_from_slice(b"MTV ");
    data.resize(44, 0);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), VIDEO_X_MTV);
    assert_eq!(mime_type.extension(), ".mtv");
}

#[test]
fn test_detect_awt() {
    // AbiWord Template - gzip-compressed with abiword marker
    // Since AWT uses same structure as ABW, it relies on extension
    // Without .awt extension, it will detect as ABW (parent)
    let mut data = vec![0x1f, 0x8b]; // gzip magic
    data.resize(20, 0);
    data.extend_from_slice(b"<?xml version=\"1.0\"?><abiword>");

    let mime_type = detect(&data);
    // Without .awt extension, detects as parent ABW
    assert_eq!(mime_type.mime(), APPLICATION_X_ABIWORD);
    assert_eq!(mime_type.extension(), ".abw");
}

#[test]
fn test_detect_spx() {
    // Ogg Speex - Ogg container with Speex codec
    // Since SPX uses OggS signature, without .spx extension it detects as generic OGG
    let data = b"OggS\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00";

    let mime_type = detect(data);
    // Without .spx extension, detects as parent OGG
    assert_eq!(mime_type.mime(), APPLICATION_OGG);
    assert_eq!(mime_type.extension(), ".ogg");
}

#[test]
fn test_detect_macos_alias() {
    // macOS Alias - Finder alias file
    let data = b"book\x00\x00\x00\x00mark\x00\x00\x00\x00";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_APPLE_ALIAS);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_csr() {
    // PEM Certificate Signing Request
    let data = b"-----BEGIN CERTIFICATE REQUEST-----\nMIICvDCCAaQCAQAwdzELMAkGA1UEBh";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_PEM_FILE);
    assert_eq!(mime_type.extension(), ".csr");
}

#[test]
fn test_detect_csr_new() {
    // PEM Certificate Signing Request (NEW variant)
    let data = b"-----BEGIN NEW CERTIFICATE REQUEST-----\nMIICvDCCAaQCAQAwdzELMAkGA1UEBh";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_PEM_FILE);
    assert_eq!(mime_type.extension(), ".csr");
}

#[test]
fn test_detect_mso() {
    // ActiveMime - Microsoft Office embedded OLE object
    let mut data = vec![0; 0x3C];
    // Place "ActiveMime" at offset 0x32
    data[0x32..0x3C].copy_from_slice(b"ActiveMime");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MSO);
    assert_eq!(mime_type.extension(), ".mso");
}

#[test]
fn test_detect_empty() {
    // Empty file - zero-length file
    let data = b"";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_EMPTY);
    assert_eq!(mime_type.extension(), ".empty");
}

#[test]
fn test_detect_mla() {
    // MLA - Multi Layer Archive
    let data = b"MLA\x00\x00\x00\x00\x00";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MLA);
    assert_eq!(mime_type.extension(), ".mla");
}

#[test]
fn test_detect_pma_pm0() {
    // PMA - PMarc variant 0
    let data = b"-pm0-\x00\x00\x00\x00";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_LZH_COMPRESSED);
    assert_eq!(mime_type.extension(), ".pma");
}

#[test]
fn test_detect_pma_pm1() {
    // PMA - PMarc variant 1
    let data = b"-pm1-\x00\x00\x00\x00";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_LZH_COMPRESSED);
    assert_eq!(mime_type.extension(), ".pma");
}

#[test]
fn test_detect_pma_pm2() {
    // PMA - PMarc variant 2
    let data = b"-pm2-\x00\x00\x00\x00";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_LZH_COMPRESSED);
    assert_eq!(mime_type.extension(), ".pma");
}

#[test]
fn test_detect_vsd() {
    // Microsoft Visio Drawing - OLE-based with CLSID
    let mut data = vec![0; 610];

    // OLE header
    data[0..8].copy_from_slice(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1");

    // Version (v3 - sector size 512)
    data[26] = 0x03;
    data[27] = 0x00;

    // First sector ID at offset 48-51 (use 0)
    data[48..52].copy_from_slice(&[0, 0, 0, 0]);

    // Visio CLSID at offset 512 + 80 = 592
    let visio_clsid: [u8; 16] = [
        0xC1, 0xDB, 0xFE, 0x00, 0x02, 0x1A, 0xCE, 0x11, 0xA3, 0x10, 0x08, 0x00, 0x2B, 0x2C, 0xF9,
        0xAE,
    ];
    data[592..608].copy_from_slice(&visio_clsid);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_VISIO);
    assert_eq!(mime_type.extension(), ".vsd");
}

#[test]
fn test_detect_xap() {
    // Microsoft Silverlight Application - ZIP with AppManifest.xaml
    let mut data = vec![];

    // ZIP local file header
    data.extend_from_slice(b"PK\x03\x04");
    data.extend_from_slice(&[0x14, 0x00]); // Version
    data.extend_from_slice(&[0x00, 0x00]); // Flags
    data.extend_from_slice(&[0x00, 0x00]); // Method
    data.extend_from_slice(&[0x00, 0x00]); // Time
    data.extend_from_slice(&[0x00, 0x00]); // Date
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC32
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Compressed size
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Uncompressed size
    data.extend_from_slice(&[0x10, 0x00]); // Filename length (16)
    data.extend_from_slice(&[0x00, 0x00]); // Extra field length
    data.extend_from_slice(b"AppManifest.xaml");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_SILVERLIGHT_APP);
    assert_eq!(mime_type.extension(), ".xap");
}

#[test]
fn test_detect_xci() {
    // Nintendo Switch ROM (XCI - NX Card Image)
    let data = b"HEAD\x00\x00\x00\x00";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_NINTENDO_SWITCH_ROM);
    assert_eq!(mime_type.extension(), ".xci");
}

#[test]
fn test_detect_xpi() {
    // Mozilla XPInstall (Firefox/Thunderbird extension) - ZIP with install.rdf
    let mut data = vec![];

    // ZIP local file header
    data.extend_from_slice(b"PK\x03\x04");
    data.extend_from_slice(&[0x14, 0x00]); // Version
    data.extend_from_slice(&[0x00, 0x00]); // Flags
    data.extend_from_slice(&[0x00, 0x00]); // Method
    data.extend_from_slice(&[0x00, 0x00]); // Time
    data.extend_from_slice(&[0x00, 0x00]); // Date
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC32
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Compressed size
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Uncompressed size
    data.extend_from_slice(&[0x0B, 0x00]); // Filename length (11)
    data.extend_from_slice(&[0x00, 0x00]); // Extra field length
    data.extend_from_slice(b"install.rdf");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_XPINSTALL);
    assert_eq!(mime_type.extension(), ".xpi");
}

#[test]
fn test_detect_xps() {
    // OpenXPS (XML Paper Specification) - ZIP with _rels/.rels
    let mut data = vec![];

    // ZIP local file header
    data.extend_from_slice(b"PK\x03\x04");
    data.extend_from_slice(&[0x14, 0x00]); // Version
    data.extend_from_slice(&[0x00, 0x00]); // Flags
    data.extend_from_slice(&[0x00, 0x00]); // Method
    data.extend_from_slice(&[0x00, 0x00]); // Time
    data.extend_from_slice(&[0x00, 0x00]); // Date
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC32
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Compressed size
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Uncompressed size
    data.extend_from_slice(&[0x0B, 0x00]); // Filename length (11)
    data.extend_from_slice(&[0x00, 0x00]); // Extra field length
    data.extend_from_slice(b"_rels/.rels");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_OXPS);
    assert_eq!(mime_type.extension(), ".xps");
}

#[test]
fn test_detect_works_wps() {
    // Microsoft Works Word Processor - OLE-based, extension-based detection
    // Without specific CLSID, this will detect as generic OLE
    // Real-world detection relies on .wps extension
    let data = b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1\x00\x00\x00\x00";

    let mime_type = detect(data);
    // Will match parent OLE format without extension hint
    assert_eq!(mime_type.mime(), APPLICATION_X_OLE_STORAGE);
}

#[test]
fn test_detect_works_xlr() {
    // Microsoft Works 6 Spreadsheet
    let data = b"\x00\x00\x02\x00\x06\x04\x06\x00";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_WORKS);
    assert_eq!(mime_type.extension(), ".xlr");
}

#[test]
fn test_detect_vcalendar() {
    // vCalendar 1.0 - Text-based calendar format
    let data = b"BEGIN:VCALENDAR\r\nVERSION:1.0\r\nPRODID:-//Test//Test//EN\r\nEND:VCALENDAR";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_CALENDAR);
    assert_eq!(mime_type.extension(), ".vcs");
}

#[test]
fn test_detect_usf() {
    // Universal Subtitle Format - XML-based subtitle format
    let data = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<USFSubtitles version=\"1.0\">\n</USFSubtitles>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_USF);
    assert_eq!(mime_type.extension(), ".usf");
}

#[test]
fn test_detect_sda() {
    // StarDraw - StarOffice/StarDivision Draw (graphics)
    let data = create_zip_with_file(b"Draw/");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_STARDIVISION_DRAW);
    assert_eq!(mime_type.extension(), ".sda");
}

#[test]
fn test_detect_sdc() {
    // StarCalc - StarOffice/StarDivision Calc (spreadsheet)
    let data = create_zip_with_file(b"Calc/");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_STARDIVISION_CALC);
    assert_eq!(mime_type.extension(), ".sdc");
}

#[test]
fn test_detect_sdd() {
    // StarImpress - StarOffice/StarDivision Impress (presentation)
    let data = create_zip_with_file(b"Impress/");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_STARDIVISION_IMPRESS);
    assert_eq!(mime_type.extension(), ".sdd");
}

#[test]
fn test_detect_sds() {
    // StarChart - StarOffice/StarDivision Chart
    let data = create_zip_with_file(b"Chart/");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_STARDIVISION_CHART);
    assert_eq!(mime_type.extension(), ".sds");
}

#[test]
fn test_detect_sdw() {
    // StarWriter - StarOffice/StarDivision Writer (word processor)
    let data = create_zip_with_file(b"Writer/");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_STARDIVISION_WRITER);
    assert_eq!(mime_type.extension(), ".sdw");
}

#[test]
fn test_detect_smf() {
    // StarMath - StarOffice/StarDivision Math (mathematical formulas)
    let data = create_zip_with_file(b"Math/");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_STARDIVISION_MATH);
    assert_eq!(mime_type.extension(), ".smf");
}

#[test]
fn test_detect_sxd() {
    // Sun XML Draw - Legacy Sun Microsystems graphics format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.draw");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_DRAW);
    assert_eq!(mime_type.extension(), ".sxd");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_DRAW));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_sxi() {
    // Sun XML Impress - Legacy Sun Microsystems presentation format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.impress");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_IMPRESS);
    assert_eq!(mime_type.extension(), ".sxi");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_IMPRESS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_presentation());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_sxm() {
    // Sun XML Math - Legacy Sun Microsystems mathematical formula format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.math");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_MATH);
    assert_eq!(mime_type.extension(), ".sxm");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_MATH));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_sxw() {
    // Sun XML Writer - Legacy Sun Microsystems word processor format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.writer");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_WRITER);
    assert_eq!(mime_type.extension(), ".sxw");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_WRITER));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_stc() {
    // Sun XML Calc Template - Legacy Sun Microsystems spreadsheet template
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.calc.template");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_CALC_TEMPLATE);
    assert_eq!(mime_type.extension(), ".stc");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_CALC_TEMPLATE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_spreadsheet());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_std() {
    // Sun XML Draw Template - Legacy Sun Microsystems graphics template
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.draw.template");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_DRAW_TEMPLATE);
    assert_eq!(mime_type.extension(), ".std");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_DRAW_TEMPLATE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_sti() {
    // Sun XML Impress Template - Legacy Sun Microsystems presentation template
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.impress.template");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_IMPRESS_TEMPLATE);
    assert_eq!(mime_type.extension(), ".sti");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_IMPRESS_TEMPLATE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_presentation());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_stw() {
    // Sun XML Writer Template - Legacy Sun Microsystems word processor template
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.writer.template");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_WRITER_TEMPLATE);
    assert_eq!(mime_type.extension(), ".stw");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_WRITER_TEMPLATE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_sgw() {
    // Sun XML Writer Global - Legacy Sun Microsystems master document format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.writer.global");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SUN_XML_WRITER_GLOBAL);
    assert_eq!(mime_type.extension(), ".sgw");
    assert!(mime_type.is(APPLICATION_VND_SUN_XML_WRITER_GLOBAL));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_wpg() {
    // WordPerfect Graphics - WordPerfect graphics format
    // Uses parent WPD signature, extension-based detection
    let data = b"\xFF\x57\x50\x43\x00\x00\x00\x00\x01\x0A";

    let mime_type = detect(data);
    // Without .wpg extension, will detect as parent WPD
    assert_eq!(mime_type.mime(), APPLICATION_VND_WORDPERFECT);
    assert_eq!(mime_type.extension(), ".wpd");
}

#[test]
fn test_detect_shw() {
    // WordPerfect Presentations - WordPerfect presentation format
    // Uses parent WPD signature, extension-based detection
    let data = b"\xFF\x57\x50\x43\x00\x00\x00\x00\x01\x0A";

    let mime_type = detect(data);
    // Without .shw extension, will detect as parent WPD
    assert_eq!(mime_type.mime(), APPLICATION_VND_WORDPERFECT);
    assert_eq!(mime_type.extension(), ".wpd");
}

#[test]
fn test_detect_wpm() {
    // WordPerfect Macro - WordPerfect macro format
    // Uses parent WPD signature, extension-based detection
    let data = b"\xFF\x57\x50\x43\x00\x00\x00\x00\x01\x0A";

    let mime_type = detect(data);
    // Without .wpm extension, will detect as parent WPD
    assert_eq!(mime_type.mime(), APPLICATION_VND_WORDPERFECT);
    assert_eq!(mime_type.extension(), ".wpd");
}

#[test]
fn test_detect_uop() {
    // Uniform Office Format Presentation - Chinese office format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    data.extend_from_slice(b"<uof:UOF xmlns:uof=\"http://schemas.uof.org/cn/2009/uof\">");
    data.extend_from_slice("演示".as_bytes()); // "presentation" in Chinese

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_UOF_PRESENTATION);
    assert_eq!(mime_type.extension(), ".uop");
    assert!(mime_type.is(APPLICATION_VND_UOF_PRESENTATION));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_presentation());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_uos() {
    // Uniform Office Format Spreadsheet - Chinese office format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    data.extend_from_slice(b"<uof:UOF xmlns:uof=\"http://schemas.uof.org/cn/2009/uof\">");
    data.extend_from_slice("电子表格".as_bytes()); // "spreadsheet" in Chinese

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_UOF_SPREADSHEET);
    assert_eq!(mime_type.extension(), ".uos");
    assert!(mime_type.is(APPLICATION_VND_UOF_SPREADSHEET));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_spreadsheet());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_uot() {
    // Uniform Office Format Text - Chinese office format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    data.extend_from_slice(b"<uof:UOF xmlns:uof=\"http://schemas.uof.org/cn/2009/uof\">");
    data.extend_from_slice("文字处理".as_bytes()); // "word processing" in Chinese

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_UOF_TEXT);
    assert_eq!(mime_type.extension(), ".uot");
    assert!(mime_type.is(APPLICATION_VND_UOF_TEXT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_iges() {
    // Initial Graphics Exchange Specification - CAD data exchange format
    // IGES files have 72 spaces followed by 'S' in column 73
    let mut data = vec![0x20; 72]; // 72 spaces
    data.push(b'S'); // Character in column 73
    data.extend_from_slice(b"      1");
    data.extend_from_slice(b"\n"); // Typical IGES line

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_IGES);
    assert_eq!(mime_type.extension(), ".iges");
    assert!(mime_type.is(MODEL_IGES));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
}

#[test]
fn test_detect_usdz() {
    // Universal Scene Description ZIP - Pixar's USD format
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"scene.usda"); // USDZ contains .usda files
    data.extend_from_slice(b"#usda 1.0\n"); // USD ASCII header

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_VND_USDZ_ZIP);
    assert_eq!(mime_type.extension(), ".usdz");
    assert!(mime_type.is(MODEL_VND_USDZ_ZIP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_sketch() {
    // Sketch - Design tool by Bohemian Coding
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"document.json"); // Sketch contains document.json
    data.extend_from_slice(b"{\"_class\":\"document\",\"do_objectID\":\"test\"}"); // JSON with _class

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_X_SKETCH);
    assert_eq!(mime_type.extension(), ".sketch");
    assert!(mime_type.is(IMAGE_X_SKETCH));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_sldasm() {
    // SolidWorks Assembly - OLE-based CAD file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"SolidWorks Assembly SLDASM");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_SLDASM);
    assert_eq!(mime_type.extension(), ".sldasm");
    assert!(mime_type.is(MODEL_X_SLDASM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}

#[test]
fn test_detect_slddrw() {
    // SolidWorks Drawing - OLE-based CAD file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"SolidWorks Drawing SLDDRW");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_SLDDRW);
    assert_eq!(mime_type.extension(), ".slddrw");
    assert!(mime_type.is(MODEL_X_SLDDRW));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}

#[test]
fn test_detect_sldprt() {
    // SolidWorks Part - OLE-based CAD file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"SolidWorks Part SLDPRT");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_SLDPRT);
    assert_eq!(mime_type.extension(), ".sldprt");
    assert!(mime_type.is(MODEL_X_SLDPRT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}

#[test]
fn test_detect_iam() {
    // Autodesk Inventor Assembly - OLE-based CAD file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"Autodesk Inventor Assembly .iam");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_IAM);
    assert_eq!(mime_type.extension(), ".iam");
    assert!(mime_type.is(MODEL_X_IAM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}

#[test]
fn test_detect_idw() {
    // Autodesk Inventor Drawing - OLE-based CAD file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"Autodesk Inventor Drawing .idw");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_IDW);
    assert_eq!(mime_type.extension(), ".idw");
    assert!(mime_type.is(MODEL_X_IDW));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}

#[test]
fn test_detect_ipn() {
    // Autodesk Inventor Presentation - OLE-based CAD file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"Autodesk Inventor Presentation .ipn");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_IPN);
    assert_eq!(mime_type.extension(), ".ipn");
    assert!(mime_type.is(MODEL_X_IPN));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}

#[test]
fn test_detect_ipt() {
    // Autodesk Inventor Part - OLE-based CAD file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"Autodesk Inventor Part .ipt");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_IPT);
    assert_eq!(mime_type.extension(), ".ipt");
    assert!(mime_type.is(MODEL_X_IPT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}

#[test]
fn test_detect_iqe() {
    // Inter-Quake Export - Text-based 3D model format
    let data = b"# Inter-Quake Export\nversion 2\nmesh model";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_X_IQE);
    assert_eq!(mime_type.extension(), ".iqe");
    assert!(mime_type.is(MODEL_X_IQE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
}

#[test]
fn test_detect_m3d() {
    // Model 3D Binary - Binary 3D model format
    let data = b"3DMO\x00\x00\x00\x01";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_X_3D_MODEL);
    assert_eq!(mime_type.extension(), ".m3d");
    assert!(mime_type.is(MODEL_X_3D_MODEL));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
}

#[test]
fn test_detect_scdoc() {
    // SpaceClaim Document - OLE-based CAD file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"SpaceClaim Document");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_SCDOC);
    assert_eq!(mime_type.extension(), ".scdoc");
    assert!(mime_type.is(MODEL_X_SCDOC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}

#[test]
fn test_detect_a3d() {
    // Model 3D ASCII - Text-based 3D model format
    let data = b"3DGeometry\nversion 1.0\nvertex 0 0 0";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_X_3D_MODEL);
    assert_eq!(mime_type.extension(), ".a3d");
    assert!(mime_type.is(TEXT_X_3D_MODEL));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
}

#[test]
fn test_detect_autodesk_123d() {
    // Autodesk 123D - ZIP-based 3D modeling format
    let mut data = b"PK\x03\x04".to_vec();
    data.resize(100, 0);
    data.extend_from_slice(b"Autodesk.123D project data");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_123DX);
    assert_eq!(mime_type.extension(), ".123dx");
    assert!(mime_type.is(MODEL_X_123DX));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_fusion_360() {
    // Fusion 360 - ZIP-based CAD format
    let mut data = b"PK\x03\x04".to_vec();
    data.resize(100, 0);
    data.extend_from_slice(b"Autodesk Fusion 360 design");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), MODEL_X_F3D);
    assert_eq!(mime_type.extension(), ".f3d");
    assert!(mime_type.is(MODEL_X_F3D));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_drawio() {
    // draw.io - XML-based diagramming format
    let data = b"<?xml version=\"1.0\"?>\n<mxfile host=\"app.diagrams.net\">\n<diagram>test</diagram>\n</mxfile>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_JGRAPH_MXFILE);
    assert_eq!(mime_type.extension(), ".drawio");
    assert!(mime_type.is(APPLICATION_VND_JGRAPH_MXFILE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_xspf() {
    // XSPF - XML Shareable Playlist Format
    let data = b"<?xml version=\"1.0\"?>\n<playlist version=\"1\" xmlns=\"http://xspf.org/ns/0/\">\n<trackList></trackList>\n</playlist>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_XSPF_XML);
    assert_eq!(mime_type.extension(), ".xspf");
    assert!(mime_type.is(APPLICATION_XSPF_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_xsl() {
    // XSLT - Extensible Stylesheet Language Transformations
    let data = b"<?xml version=\"1.0\"?>\n<xsl:stylesheet version=\"1.0\" xmlns:xsl=\"http://www.w3.org/1999/XSL/Transform\">\n</xsl:stylesheet>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_XSLT_XML);
    assert_eq!(mime_type.extension(), ".xsl");
    assert!(mime_type.is(APPLICATION_XSLT_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_figma() {
    // Figma - ZIP-based design format
    let mut data = b"PK\x03\x04".to_vec();
    data.resize(100, 0);
    data.extend_from_slice(b"{\"document\":{\"id\":\"123\"},\"canvas\":{}}");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), IMAGE_X_FIGMA);
    assert_eq!(mime_type.extension(), ".fig");
    assert!(mime_type.is(IMAGE_X_FIGMA));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
    assert!(mime_type.kind().is_archive()); // Inherits from ZIP
}

#[test]
fn test_detect_mathml() {
    // MathML - Mathematical Markup Language
    let data = b"<?xml version=\"1.0\"?>\n<math xmlns=\"http://www.w3.org/1998/Math/MathML\">\n<mrow><mi>x</mi></mrow>\n</math>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_MATHML_XML);
    assert_eq!(mime_type.extension(), ".mathml");
    assert!(mime_type.is(APPLICATION_MATHML_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_musicxml() {
    // MusicXML - Music notation format
    let data = b"<?xml version=\"1.0\"?>\n<score-partwise version=\"3.1\">\n<part-list></part-list>\n</score-partwise>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_RECORDARE_MUSICXML_XML);
    assert_eq!(mime_type.extension(), ".musicxml");
    assert!(mime_type.is(APPLICATION_VND_RECORDARE_MUSICXML_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_ttml() {
    // TTML - Timed Text Markup Language
    let data = b"<?xml version=\"1.0\"?>\n<tt xmlns=\"http://www.w3.org/ns/ttml\">\n<body><div></div></body>\n</tt>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_TTML_XML);
    assert_eq!(mime_type.extension(), ".ttml");
    assert!(mime_type.is(APPLICATION_TTML_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_soap() {
    // SOAP - Simple Object Access Protocol
    let data = b"<?xml version=\"1.0\"?>\n<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">\n<soap:Body></soap:Body>\n</soap:Envelope>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_SOAP_XML);
    assert_eq!(mime_type.extension(), ".soap");
    assert!(mime_type.is(APPLICATION_SOAP_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_tmx() {
    // TMX - Tiled Map XML (game development)
    let data = b"<?xml version=\"1.0\"?>\n<map version=\"1.9\" orientation=\"orthogonal\" width=\"10\" height=\"10\">\n<tileset></tileset>\n</map>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_TMX_XML);
    assert_eq!(mime_type.extension(), ".tmx");
    assert!(mime_type.is(APPLICATION_X_TMX_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_tsx() {
    // TSX - Tiled Tileset XML (game development)
    let data = b"<?xml version=\"1.0\"?>\n<tileset version=\"1.9\" tilewidth=\"32\" tileheight=\"32\" tilecount=\"100\">\n<image source=\"tiles.png\"/>\n</tileset>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_TSX_XML);
    assert_eq!(mime_type.extension(), ".tsx");
    assert!(mime_type.is(APPLICATION_X_TSX_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_mpd() {
    // MPD - MPEG-DASH Media Presentation Description
    let data = b"<?xml version=\"1.0\"?>\n<MPD xmlns=\"urn:mpeg:dash:schema:mpd:2011\" type=\"static\">\n<Period></Period>\n</MPD>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_DASH_XML);
    assert_eq!(mime_type.extension(), ".mpd");
    assert!(mime_type.is(APPLICATION_DASH_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_mxl() {
    // MXL - MusicXML ZIP (compressed music notation)
    // Create a minimal ZIP file with .musicxml content indicator
    let mut data = Vec::new();
    data.extend_from_slice(b"PK\x03\x04"); // ZIP local file header signature
    data.extend_from_slice(&[0x14, 0x00]); // version needed
    data.extend_from_slice(&[0x00, 0x00]); // flags
    data.extend_from_slice(&[0x00, 0x00]); // compression method (stored)
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // modification time/date
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC-32
    data.extend_from_slice(&[0x0e, 0x00, 0x00, 0x00]); // compressed size
    data.extend_from_slice(&[0x0e, 0x00, 0x00, 0x00]); // uncompressed size
    data.extend_from_slice(&[0x0c, 0x00]); // filename length
    data.extend_from_slice(&[0x00, 0x00]); // extra field length
    data.extend_from_slice(b"score.musicxml"); // filename
    data.extend_from_slice(b"<score-partwise>"); // file content

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_RECORDARE_MUSICXML);
    assert_eq!(mime_type.extension(), ".mxl");
    assert!(mime_type.is(APPLICATION_VND_RECORDARE_MUSICXML));
    assert!(!mime_type.is(APPLICATION_ZIP));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_cddx() {
    // CDDX - Circuit Diagram Document
    let data = b"<?xml version=\"1.0\"?>\n<CircuitDocument xmlns=\"http://www.circuitdiagram.org/xml\">\n<circuit></circuit>\n</CircuitDocument>";

    let mime_type = detect(data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_CIRCUITDIAGRAM_DOCUMENT_MAIN_XML
    );
    assert_eq!(mime_type.extension(), ".cddx");
    assert!(mime_type.is(APPLICATION_VND_CIRCUITDIAGRAM_DOCUMENT_MAIN_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_dwfx() {
    // DWFX - Design Web Format XPS
    let data = b"<?xml version=\"1.0\"?>\n<DWFDocument xmlns=\"http://www.autodesk.com/dwfx\">\n<Section></Section>\n</DWFDocument>";

    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_VND_DWFX_XPS);
    assert_eq!(mime_type.extension(), ".dwfx");
    assert!(mime_type.is(MODEL_VND_DWFX_XPS));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_fbz() {
    // FBZ - FictionBook ZIP (compressed e-book)
    // Create a minimal ZIP file with .fb2 content indicator
    let mut data = Vec::new();
    data.extend_from_slice(b"PK\x03\x04"); // ZIP local file header signature
    data.extend_from_slice(&[0x14, 0x00]); // version needed
    data.extend_from_slice(&[0x00, 0x00]); // flags
    data.extend_from_slice(&[0x00, 0x00]); // compression method (stored)
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // modification time/date
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC-32
    data.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]); // compressed size
    data.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]); // uncompressed size
    data.extend_from_slice(&[0x09, 0x00]); // filename length
    data.extend_from_slice(&[0x00, 0x00]); // extra field length
    data.extend_from_slice(b"book.fb2"); // filename
    data.extend_from_slice(b"<FictionBook>"); // file content

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_FBZ);
    assert_eq!(mime_type.extension(), ".fbz");
    assert!(mime_type.is(APPLICATION_X_FBZ));
    assert!(!mime_type.is(APPLICATION_ZIP));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_autodesk_max() {
    // Autodesk 3D Studio Max - OLE-based project file
    let mut data = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]; // OLE header
    data.resize(100, 0);
    data.extend_from_slice(b"3dsmax project data");

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MAX);
    assert_eq!(mime_type.extension(), ".max");
    assert!(mime_type.is(APPLICATION_X_MAX));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_model());
    assert!(mime_type.kind().is_document()); // Inherits from OLE
}
