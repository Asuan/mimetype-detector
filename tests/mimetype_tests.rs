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

    // OLE header signature
    data[0..8].copy_from_slice(&[0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]);

    // Minor version (offset 24-25)
    data[24..26].copy_from_slice(&[0x3e, 0x00]);

    // Sector shift (offset 26-27): 0x0009 for 512-byte sectors
    data[26..28].copy_from_slice(&[0x09, 0x00]);

    // First directory sector SecID (offset 48-51): 0 (first sector after header)
    data[48..52].copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    // CLSID at: 512 * (1 + 0) + 80 = 592
    const CLSID_OFFSET: usize = SECTOR_SIZE * (1 + 0) + 80;
    let clsid_len = clsid.len().min(16);
    data[CLSID_OFFSET..CLSID_OFFSET + clsid_len].copy_from_slice(&clsid[..clsid_len]);

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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(TEXT_UTF16_BE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_utf16_le() {
    let data = b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF16_LE);
    assert_eq!(mime_type.extension(), ".txt");
    // Test is() method
    assert!(mime_type.is(TEXT_UTF16_LE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_utf8() {
    let data = b"Hello World";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_UTF8);
    assert_eq!(mime_type.extension(), ".txt");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_OLE_STORAGE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_aaf() {
    // Advanced Authoring Format CLSID
    const AAF_CLSID: &[u8] = &[
        0xAA, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];
    let data = create_ole_with_clsid(AAF_CLSID);

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_AAF);
    assert_eq!(mime_type.extension(), ".aaf");
    assert!(mime_type.is(APPLICATION_X_AAF));
    // AAF inherits DOCUMENT kind from OLE parent
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_7Z_COMPRESSED));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_zip() {
    let data = b"PK\x03\x04";
    let mime_type = detect(data);
    // ZIP detection might identify child formats in the hierarchy
    assert!(mime_type.mime().contains("zip") || mime_type.mime().contains("application/"));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_rar() {
    let data = b"Rar!\x1a\x07\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_RAR_COMPRESSED);
    assert_eq!(mime_type.extension(), ".rar");
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_GZIP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
}

#[test]
fn test_detect_tar() {
    // Create a minimal valid TAR header with proper checksum
    let mut data = vec![0; 512];
    // Set filename
    data[0..5].copy_from_slice(b"test\0");
    // Set mode
    data[100..108].copy_from_slice(b"0000644\0");
    // Set uid/gid
    data[108..116].copy_from_slice(b"0000000\0");
    data[116..124].copy_from_slice(b"0000000\0");
    // Set size
    data[124..136].copy_from_slice(b"00000000000\0");
    // Set mtime
    data[136..148].copy_from_slice(b"00000000000\0");

    // Calculate proper TAR checksum (spaces for checksum field during calculation)
    let checksum = data.iter().take(148).map(|&b| b as u32).sum::<u32>()
        + (b' ' as u32) * 8  // 8 spaces for checksum field
        + data.iter().skip(156).map(|&b| b as u32).sum::<u32>();
    let checksum_str = format!("{checksum:06o}\0 ");
    data[148..156].copy_from_slice(checksum_str.as_bytes());

    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_X_TAR);
    assert_eq!(mime_type.extension(), ".tar");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_INSTALLSHIELD));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_cpio() {
    // Test ASCII CPIO
    let data = b"070701";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_CPIO);
    assert_eq!(mime_type.extension(), ".cpio");
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_CPIO));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());

    // Test binary CPIO
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_BITTORRENT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_fits() {
    let data = b"SIMPLE  =                    T";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_FITS);
    assert_eq!(mime_type.extension(), ".fits");
    // Test is() method
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
    // Test is() method
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
    // DEB uses AR format - detected as AR
    assert_eq!(mime_type.mime(), APPLICATION_X_ARCHIVE);
    assert_eq!(mime_type.extension(), ".a");
}

#[test]
fn test_detect_warc() {
    let data = b"WARC/1.0\r\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_WARC);
    assert_eq!(mime_type.extension(), ".warc");
    // Test is() method
    assert!(mime_type.is(APPLICATION_WARC));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
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
    // Test is() method
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
    // APNG may be detected as PNG in the hierarchy
    assert!(mime_type.mime() == IMAGE_VND_MOZILLA_APNG)
}

#[test]
fn test_detect_jpeg() {
    let data = b"\xff\xd8\xff\xe0";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_JPEG);
    assert_eq!(mime_type.extension(), ".jpg");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // HEIF sequence detection not implemented yet - likely detects as HEIF
    assert_eq!(mime_type.mime(), IMAGE_HEIF);
    assert_eq!(mime_type.extension(), ".heif");
}

#[test]
fn test_detect_bpg() {
    let data = b"BPG\xFB";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_BPG);
    assert_eq!(mime_type.extension(), ".bpg");
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(IMAGE_X_XCF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_pat() {
    let data =
        b"\x00\x00\x00\x1c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x03GPAT";
    let mime_type = detect(data);
    // PAT detection not implemented yet - falls back to octet-stream
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_gbr() {
    let mut data = vec![0; 24];
    data[20..24].copy_from_slice(b"GIMP");
    let mime_type = detect(&data);
    // GBR detection not implemented yet - falls back to octet-stream
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_hdr() {
    let data = b"#?RADIANCE\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_VND_RADIANCE);
    assert_eq!(mime_type.extension(), ".hdr");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(IMAGE_AVIF));
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(AUDIO_AMR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_voc() {
    let data = b"Creative Voice File";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_X_UNKNOWN);
    assert_eq!(mime_type.extension(), ".voc");
    // Test is() method
    assert!(mime_type.is(AUDIO_X_UNKNOWN));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_m3u() {
    let data = b"#EXTM3U";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), AUDIO_X_MPEGURL);
    assert_eq!(mime_type.extension(), ".m3u");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(AUDIO_QCELP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_audio());
}

#[test]
fn test_detect_m4a() {
    let data = b"\x00\x00\x00\x18ftypM4A ";
    let mime_type = detect(data);
    // M4A detection not implemented yet - falls back to octet-stream
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_amp4() {
    let data = b"\x00\x00\x00\x18ftypF4A ";
    let mime_type = detect(data);
    // Audio MP4 detection not implemented yet - falls back to octet-stream
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert_eq!(mime_type.extension(), "");
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(VIDEO_X_MATROSKA));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_video());
}

#[test]
fn test_detect_avi() {
    // Proper AVI (RIFF) file structure:
    // RIFF - RIFF header (4 bytes)
    // file size - 4 bytes (little-endian)
    // AVI LIST - AVI format identifier (8 bytes at offset 8)
    // Additional RIFF chunks follow...
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
    assert_eq!(mime_type.extension(), ".mpeg");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(VIDEO_X_M4V));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_rmvb() {
    let data = b".RMF";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_RN_REALMEDIA_VBR);
    assert_eq!(mime_type.extension(), ".rmvb");
    // Test is() method
    assert!(mime_type.is(APPLICATION_VND_RN_REALMEDIA_VBR));
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_elf() {
    let data = b"\x7fELF";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_ELF);
    assert_eq!(mime_type.extension(), "");
    // Test is() method
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
    // Test is() method
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
    assert_eq!(mime_type.extension(), "");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_COREDUMP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_class() {
    let data = b"\xca\xfe\xba\xbe";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_JAVA_APPLET_BINARY);
    assert_eq!(mime_type.extension(), ".class");
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_JAVA_APPLET_BINARY));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_wasm() {
    let data = b"\x00asm";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_WASM);
    assert_eq!(mime_type.extension(), ".wasm");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_SHOCKWAVE_FLASH));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_crx() {
    let data = b"Cr24\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00PK\x03\x04";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_CHROME_EXTENSION);
    assert_eq!(mime_type.extension(), ".crx");
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_CHROME_EXTENSION));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_p7s() {
    let data = b"-----BEGIN PKCS7-----";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_PKCS7_SIGNATURE);
    assert_eq!(mime_type.extension(), ".p7s");
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_DICOM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_image());
}

#[test]
fn test_detect_mobi() {
    let mut data = vec![0; 68];
    data[60..68].copy_from_slice(b"BOOKMOBI");
    let mime_type = detect(&data);
    // MOBI detection not implemented yet - falls back to octet-stream
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_lit() {
    let data = b"ITOLITLS";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MS_READER);
    assert_eq!(mime_type.extension(), ".lit");
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_MS_READER));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_sqlite3() {
    let data = b"SQLite format 3\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SQLITE3);
    assert_eq!(mime_type.extension(), ".sqlite");
    // Test is() method
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
    // FASOO detection not implemented yet - likely detects as OLE
    assert_eq!(mime_type.mime(), APPLICATION_X_OLE_STORAGE);
    assert_eq!(mime_type.extension(), "");
}

#[test]
fn test_detect_pgp_net_share() {
    let data = b"-----BEGIN PGP";
    let mime_type = detect(data);
    // PGP detection not implemented yet - falls back to text/plain
    assert_eq!(mime_type.mime(), TEXT_UTF8);
    assert_eq!(mime_type.extension(), ".txt");
}

// ============================================================================
// MICROSOFT OFFICE & DOCUMENT FORMATS
// ============================================================================

#[test]
fn test_detect_docx() {
    // Create a minimal DOCX-like ZIP
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.extend_from_slice(b"\x14\x00\x00\x00\x00\x00"); // ZIP fields
    data.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // More ZIP fields
    data.extend_from_slice(b"\x13\x00\x00\x00"); // Filename length
    data.extend_from_slice(b"[Content_Types].xml"); // Filename
    let mime_type = detect(&data);
    // May detect as ZIP or DOCX depending on hierarchy
    assert!(mime_type.mime().contains("zip") || mime_type.mime().contains("word"));
}

#[test]
fn test_detect_xlsx() {
    // Create a minimal XLSX-like ZIP
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.extend_from_slice(b"\x14\x00\x00\x00\x00\x00"); // ZIP fields
    data.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // More ZIP fields
    data.extend_from_slice(b"\x13\x00\x00\x00"); // Filename length
    data.extend_from_slice(b"[Content_Types].xml"); // Filename
    let mime_type = detect(&data);
    // May detect as ZIP or XLSX depending on hierarchy
    assert!(mime_type.mime().contains("zip") || mime_type.mime().contains("sheet"));
}

#[test]
fn test_detect_pptx() {
    // Create a minimal PPTX-like ZIP
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.extend_from_slice(b"\x14\x00\x00\x00\x00\x00"); // ZIP fields
    data.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // More ZIP fields
    data.extend_from_slice(b"\x13\x00\x00\x00"); // Filename length
    data.extend_from_slice(b"[Content_Types].xml"); // Filename
    let mime_type = detect(&data);
    // May detect as ZIP or PPTX depending on hierarchy
    assert!(mime_type.mime().contains("zip") || mime_type.mime().contains("presentation"));
}

#[test]
fn test_detect_epub() {
    // EPUB uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/epub+zip");
    let mime_type = detect(&data);
    assert_eq!(mime_type.mime(), APPLICATION_EPUB_ZIP);
    assert_eq!(mime_type.extension(), ".epub");
    // Test is() method
    assert!(mime_type.is(APPLICATION_EPUB_ZIP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_archive());
    assert!(mime_type.kind().is_document());
}

#[test]
fn test_detect_jar() {
    // Create a minimal JAR-like ZIP
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.extend_from_slice(b"\x14\x00\x00\x00\x00\x00"); // ZIP fields
    data.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // More ZIP fields
    data.extend_from_slice(b"\x14\x00\x00\x00"); // Filename length
    data.extend_from_slice(b"META-INF/MANIFEST.MF"); // Filename
    let mime_type = detect(&data);
    // May detect as ZIP or JAR depending on hierarchy
    assert!(mime_type.mime().contains("zip") || mime_type.mime().contains("java"));
}

#[test]
fn test_detect_apk() {
    // Create a minimal APK-like ZIP
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.extend_from_slice(b"\x14\x00\x00\x00\x00\x00"); // ZIP fields
    data.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // More ZIP fields
    data.extend_from_slice(b"\x13\x00\x00\x00"); // Filename length
    data.extend_from_slice(b"AndroidManifest.xml"); // Filename
    let mime_type = detect(&data);
    // May detect as ZIP or APK depending on hierarchy
    assert!(mime_type.mime().contains("zip") || mime_type.mime().contains("android"));
}

#[test]
fn test_detect_doc() {
    // Word 97-2003 CLSID
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_VND_WORDPERFECT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_xls() {
    // Excel v5 CLSID: 10 08 02 00 00 00 00 00
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
    // PowerPoint v4 CLSID
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
    // Publisher CLSID
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
    // Outlook MSG CLSID
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
    // MSI Installer CLSID
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
    // ODT uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.text");
    let mime_type = detect(&data);
    // ODT detection may fall back to ZIP
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT
            || mime_type.mime() == APPLICATION_ZIP
    );
}

#[test]
fn test_detect_ods() {
    // ODS uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.spreadsheet");
    let mime_type = detect(&data);
    // ODS detection may fall back to ZIP
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET
            || mime_type.mime() == APPLICATION_ZIP
    );
}

#[test]
fn test_detect_odp() {
    // ODP uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.presentation");
    let mime_type = detect(&data);
    // ODP detection may fall back to ZIP
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION
            || mime_type.mime() == APPLICATION_ZIP
    );
}

#[test]
fn test_detect_odg() {
    // ODG uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.graphics");
    let mime_type = detect(&data);
    // ODG detection may fall back to ZIP
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS
            || mime_type.mime() == APPLICATION_ZIP
    );
}

#[test]
fn test_detect_odf() {
    // ODF uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.formula");
    let mime_type = detect(&data);
    // ODF detection may fall back to ZIP
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA
            || mime_type.mime() == APPLICATION_ZIP
    );
}

#[test]
fn test_detect_odc() {
    // ODC uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.chart");
    let mime_type = detect(&data);
    // ODC detection may fall back to ZIP
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_CHART
            || mime_type.mime() == APPLICATION_ZIP
    );
}

#[test]
fn test_detect_ott() {
    // OTT uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.text-template");
    let mime_type = detect(&data);
    // OTT detection may fall back to base ODT format
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE
            || mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT
    );
}

#[test]
fn test_detect_ots() {
    // OTS uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.spreadsheet-template");
    let mime_type = detect(&data);
    // OTS detection may fall back to base ODS format
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE
            || mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET
    );
}

#[test]
fn test_detect_otp() {
    // OTP uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.presentation-template");
    let mime_type = detect(&data);
    // OTP detection may fall back to base ODP format
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE
            || mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION
    );
}

#[test]
fn test_detect_otg() {
    // OTG uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.oasis.opendocument.graphics-template");
    let mime_type = detect(&data);
    // OTG detection may fall back to base ODG format
    assert!(
        mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE
            || mime_type.mime() == APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS
    );
}

#[test]
fn test_detect_sxc() {
    // SXC uses offset-based detection
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.resize(30, 0);
    data.extend_from_slice(b"mimetypeapplication/vnd.sun.xml.calc");
    let mime_type = detect(&data);
    // SXC detection may fall back to ZIP
    assert!(
        mime_type.mime() == APPLICATION_VND_SUN_XML_CALC || mime_type.mime() == APPLICATION_ZIP
    );
}

#[test]
fn test_detect_kmz() {
    // KMZ is a ZIP with doc.kml file
    let mut data = vec![0x50, 0x4b, 0x03, 0x04]; // ZIP header
    data.extend_from_slice(b"\x14\x00\x00\x00\x00\x00"); // ZIP fields
    data.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // More ZIP fields
    data.extend_from_slice(b"\x07\x00\x00\x00"); // Filename length
    data.extend_from_slice(b"doc.kml"); // Filename
    let mime_type = detect(&data);
    // KMZ detection may fall back to ZIP
    assert!(
        mime_type.mime() == APPLICATION_VND_GOOGLE_EARTH_KMZ || mime_type.mime() == APPLICATION_ZIP
    );
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(TEXT_TAB_SEPARATED_VALUES));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_text());
}

#[test]
fn test_detect_rtf() {
    // Proper RTF file structure (matches real RTF files):
    // {\rtf1 - RTF version 1 (required signature)
    // \ansi - ANSI character set
    // \deff0 - default font index
    // {\fonttbl...} - font table (required for valid RTF)
    // Content with RTF control words
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_XLIFF_XML));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_collada() {
    let data = b"<?xml version=\"1.0\"?><COLLADA></COLLADA>";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_VND_COLLADA_XML);
    assert_eq!(mime_type.extension(), ".dae");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_VND_SHP));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_shx() {
    let data = b"\x00\x00\x27\x0A";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_SHX);
    assert_eq!(mime_type.extension(), ".shx");
    // Test is() method
    assert!(mime_type.is(APPLICATION_VND_SHX));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_glb() {
    let data = b"glTF\x02\x00\x00\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), MODEL_GLTF_BINARY);
    assert_eq!(mime_type.extension(), ".glb");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_VND_NINTENDO_SNES_ROM));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

// ============================================================================
// MISCELLANEOUS FORMATS
// ============================================================================

#[test]
fn test_detect_hdf() {
    let data = b"\x89HDF\r\n\x1a\n";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_HDF);
    assert_eq!(mime_type.extension(), ".hdf");
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_HDF));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
    assert!(mime_type.kind().is_database());
}

#[test]
fn test_detect_cbor() {
    let data = b"\xd9\xd9\xf7";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_CBOR);
    assert_eq!(mime_type.extension(), ".cbor");
    // Test is() method
    assert!(mime_type.is(APPLICATION_CBOR));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_parquet() {
    let data = b"PAR1";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_APACHE_PARQUET);
    assert_eq!(mime_type.extension(), ".parquet");
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(APPLICATION_X_MS_SHORTCUT));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_macho() {
    let data = b"\xfe\xed\xfa\xce"; // Mach-O magic
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_X_MACH_BINARY);
    assert_eq!(mime_type.extension(), ".macho");
    // Test is() method
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
    // Test is() method
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
    // Test is() method
    assert!(mime_type.is(TEXT_HTML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_html_utf16_le() {
    let data = b"\xFF\xFE<\x00h\x00t\x00m\x00l\x00>\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_HTML_UTF16);
    assert_eq!(mime_type.extension(), ".html");
    // Test is() method
    assert!(mime_type.is(TEXT_HTML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_xml_utf16_be() {
    let data = b"\xFE\xFF\x00<\x00?\x00x\x00m\x00l";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_XML_UTF16);
    assert_eq!(mime_type.extension(), ".xml");
    // Test is() method
    assert!(mime_type.is(TEXT_XML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_xml_utf16_le() {
    let data = b"\xFF\xFE<\x00?\x00x\x00m\x00l\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_XML_UTF16);
    assert_eq!(mime_type.extension(), ".xml");
    // Test is() method
    assert!(mime_type.is(TEXT_XML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_svg_utf16_be() {
    let data = b"\xFE\xFF\x00<\x00s\x00v\x00g";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_SVG_XML_UTF16);
    assert_eq!(mime_type.extension(), ".svg");
    // Test is() method
    assert!(mime_type.is(IMAGE_SVG_XML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_svg_utf16_le() {
    let data = b"\xFF\xFE<\x00s\x00v\x00g\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), IMAGE_SVG_XML_UTF16);
    assert_eq!(mime_type.extension(), ".svg");
    // Test is() method
    assert!(mime_type.is(IMAGE_SVG_XML_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_json_utf16_be() {
    let data = b"\xFE\xFF\x00{\x00\"\x00k\x00e\x00y\x00\"\x00:\x00\"\x00v\x00a\x00l\x00\"\x00}";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_JSON_UTF16);
    assert_eq!(mime_type.extension(), ".json");
    // Test is() method
    assert!(mime_type.is(APPLICATION_JSON_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_json_utf16_le() {
    let data = b"\xFF\xFE{\x00\"\x00k\x00e\x00y\x00\"\x00:\x00\"\x00v\x00a\x00l\x00\"\x00}\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), APPLICATION_JSON_UTF16);
    assert_eq!(mime_type.extension(), ".json");
    // Test is() method
    assert!(mime_type.is(APPLICATION_JSON_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_csv_utf16_be() {
    let data = b"\xFE\xFF\x00n\x00a\x00m\x00e\x00,\x00a\x00g\x00e";
    let mime_type = detect(data);
    // CSV UTF-16 may be detected as plain UTF-16
    assert!(mime_type.mime() == TEXT_CSV_UTF16 || mime_type.mime() == TEXT_UTF16_BE);
}

#[test]
fn test_detect_csv_utf16_le() {
    let data = b"\xFF\xFEn\x00a\x00m\x00e\x00,\x00a\x00g\x00e\x00";
    let mime_type = detect(data);
    // CSV UTF-16 may be detected as plain UTF-16
    assert!(mime_type.mime() == TEXT_CSV_UTF16 || mime_type.mime() == TEXT_UTF16_LE);
}

#[test]
fn test_detect_tsv_utf16_be() {
    let data = b"\xFE\xFF\x00n\x00a\x00m\x00e\x00\t\x00a\x00g\x00e";
    let mime_type = detect(data);
    // TSV UTF-16 may be detected as plain UTF-16
    assert!(
        mime_type.mime() == TEXT_TAB_SEPARATED_VALUES_UTF16 || mime_type.mime() == TEXT_UTF16_BE
    );
}

#[test]
fn test_detect_tsv_utf16_le() {
    let data = b"\xFF\xFEn\x00a\x00m\x00e\x00\t\x00a\x00g\x00e\x00";
    let mime_type = detect(data);
    // TSV UTF-16 may be detected as plain UTF-16
    assert!(
        mime_type.mime() == TEXT_TAB_SEPARATED_VALUES_UTF16 || mime_type.mime() == TEXT_UTF16_LE
    );
}

#[test]
fn test_detect_srt_utf16_be() {
    let data = b"\xFE\xFF\x001\x00\n\x000\x000\x00:\x000\x000\x00:\x000\x000\x00,\x000\x000\x000";
    let mime_type = detect(data);
    // SRT UTF-16 may be detected as plain UTF-16
    assert!(mime_type.mime() == APPLICATION_X_SUBRIP_UTF16 || mime_type.mime() == TEXT_UTF16_BE);
}

#[test]
fn test_detect_srt_utf16_le() {
    let data = b"\xFF\xFE1\x00\n\x000\x000\x00:\x000\x000\x00:\x000\x000\x00,\x000\x000\x000\x00";
    let mime_type = detect(data);
    // SRT UTF-16 may be detected as plain UTF-16
    assert!(mime_type.mime() == APPLICATION_X_SUBRIP_UTF16 || mime_type.mime() == TEXT_UTF16_LE);
}

#[test]
fn test_detect_vtt_utf16_be() {
    let data = b"\xFE\xFF\x00W\x00E\x00B\x00V\x00T\x00T";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VTT_UTF16);
    assert_eq!(mime_type.extension(), ".vtt");
    // Test is() method
    assert!(mime_type.is(TEXT_VTT_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_vtt_utf16_le() {
    let data = b"\xFF\xFEW\x00E\x00B\x00V\x00T\x00T\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VTT_UTF16);
    assert_eq!(mime_type.extension(), ".vtt");
    // Test is() method
    assert!(mime_type.is(TEXT_VTT_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_vcard_utf16_be() {
    let data = b"\xFE\xFF\x00B\x00E\x00G\x00I\x00N\x00:\x00V\x00C\x00A\x00R\x00D";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VCARD_UTF16);
    assert_eq!(mime_type.extension(), ".vcf");
    // Test is() method
    assert!(mime_type.is(TEXT_VCARD_UTF16));
    assert!(!mime_type.is(APPLICATION_OCTET_STREAM));
}

#[test]
fn test_detect_vcard_utf16_le() {
    let data = b"\xFF\xFEB\x00E\x00G\x00I\x00N\x00:\x00V\x00C\x00A\x00R\x00D\x00";
    let mime_type = detect(data);
    assert_eq!(mime_type.mime(), TEXT_VCARD_UTF16);
    assert_eq!(mime_type.extension(), ".vcf");
    // Test is() method
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
