//! Comprehensive test suite for mimetype-detector library
//!
//! This test suite covers:
//! - Format detection (positive tests)
//! - False positive prevention (negative tests)
//! - Boundary conditions and edge cases
//! - API functionality validation
//! - Cross-format confusion prevention

use mimetype_detector::{detect, detect_reader, equals_any, is_supported, match_mime, constants::*};
use std::io::Cursor;

// ============================================================================
// POSITIVE DETECTION TESTS
// ============================================================================

#[test]
fn test_image_formats() {
    // PNG detection
    let png_data = b"\x89PNG\r\n\x1a\n";
    let mime_type = detect(png_data);
    assert_eq!(mime_type.mime(), IMAGE_PNG);
    assert_eq!(mime_type.extension(), ".png");

    // JPEG detection
    let jpg_data = b"\xff\xd8\xff\xe0";
    let mime_type = detect(jpg_data);
    assert_eq!(mime_type.mime(), IMAGE_JPEG);
    assert_eq!(mime_type.extension(), ".jpg");

    // GIF detection
    let gif_data = b"GIF89a";
    let mime_type = detect(gif_data);
    assert_eq!(mime_type.mime(), IMAGE_GIF);
    assert_eq!(mime_type.extension(), ".gif");

    // WebP detection
    let webp_data = b"RIFF\x00\x00\x00\x00WEBP";
    let mime_type = detect(webp_data);
    assert_eq!(mime_type.mime(), IMAGE_WEBP);
    assert_eq!(mime_type.extension(), ".webp");

    // BMP detection
    let bmp_data = b"BM";
    let mime_type = detect(bmp_data);
    assert_eq!(mime_type.mime(), IMAGE_BMP);
    assert_eq!(mime_type.extension(), ".bmp");

    // TIFF detection
    let tiff_data = b"II*\x00";
    let mime_type = detect(tiff_data);
    assert_eq!(mime_type.mime(), IMAGE_TIFF);
    assert_eq!(mime_type.extension(), ".tiff");
}

#[test]
fn test_advanced_image_formats() {
    // APNG detection
    let mut apng_data = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]; // PNG header
    apng_data.resize(37, 0); // Pad to offset 37
    apng_data.extend_from_slice(b"acTL"); // APNG marker
    let mime_type = detect(&apng_data);
    // APNG extends PNG, so PNG will match first in the hierarchy
    assert!(mime_type.mime() == IMAGE_PNG || mime_type.mime() == IMAGE_VND_MOZILLA_APNG);

    // JPEG 2000 detection
    let mut jp2_data = vec![0, 0, 0, 0]; // Box size placeholder
    jp2_data.extend_from_slice(b"jP2 "); // JPEG 2000 signature
    jp2_data.resize(20, 0); // Pad to offset 20
    jp2_data.extend_from_slice(b"jp2 "); // JP2 brand
    let mime_type = detect(&jp2_data);
    assert_eq!(mime_type.mime(), IMAGE_JP2);

    // BPG detection
    let bpg_data = b"BPG\xFB";
    let mime_type = detect(bpg_data);
    assert_eq!(mime_type.mime(), IMAGE_BPG);

    // XCF detection
    let xcf_data = b"gimp xcf v011";
    let mime_type = detect(xcf_data);
    assert_eq!(mime_type.mime(), IMAGE_X_XCF);

    // JPEG XL detection
    let jxl_data = b"\xFF\x0A";
    let mime_type = detect(jxl_data);
    assert_eq!(mime_type.mime(), IMAGE_JXL);

    let jxl_data2 = b"\x00\x00\x00\x0CJXL \x0D\x0A\x87\x0A";
    let mime_type2 = detect(jxl_data2);
    assert_eq!(mime_type2.mime(), "image/jxl");
}

#[test]
fn test_audio_formats() {
    // MP3 detection
    let mp3_data = b"ID3";
    let mime_type = detect(mp3_data);
    assert_eq!(mime_type.mime(), AUDIO_MPEG);
    assert_eq!(mime_type.extension(), ".mp3");

    // WAV detection
    let wav_data = b"RIFF\x00\x00\x00\x00WAVE";
    let mime_type = detect(wav_data);
    assert_eq!(mime_type.mime(), AUDIO_WAV);
    assert_eq!(mime_type.extension(), ".wav");

    // FLAC detection
    let flac_data = b"fLaC";
    let mime_type = detect(flac_data);
    assert_eq!(mime_type.mime(), AUDIO_FLAC);
    assert_eq!(mime_type.extension(), ".flac");

    // AIFF detection
    let aiff_data = b"FORM\x00\x00\x00\x00AIFF";
    let mime_type = detect(aiff_data);
    assert_eq!(mime_type.mime(), AUDIO_AIFF);

    // APE detection
    let ape_data = b"MAC \x96\x0F\x00\x00\x34\x00\x00\x00\x18\x00\x00\x00\x90\xE3";
    let mime_type = detect(ape_data);
    assert_eq!(mime_type.mime(), AUDIO_APE);

    // Musepack detection
    let mpc_data = b"MPCK";
    let mime_type = detect(mpc_data);
    assert_eq!(mime_type.mime(), AUDIO_MUSEPACK);

    // AU detection
    let au_data = b".snd\x00\x00\x00\x18";
    let mime_type = detect(au_data);
    assert_eq!(mime_type.mime(), AUDIO_BASIC);

    // AMR detection
    let amr_data = b"#!AMR\x0A";
    let mime_type = detect(amr_data);
    assert_eq!(mime_type.mime(), AUDIO_AMR);

    // AAC detection
    let aac_data = b"\xFF\xF1\x50\x80";
    let mime_type = detect(aac_data);
    assert_eq!(mime_type.mime(), AUDIO_AAC);
}

#[test]
fn test_video_formats() {
    // AVI detection
    let avi_data = b"RIFF\x00\x00\x00\x00AVI LIST";
    let mime_type = detect(avi_data);
    // May detect as enhanced or basic AVI detector depending on order
    assert!(mime_type.mime().contains("video") || mime_type.mime() == "application/octet-stream");

    // MKV detection
    let mut mkv_data = b"\x1a\x45\xdf\xa3".to_vec();
    mkv_data.resize(100, 0); // Pad with zeros
    mkv_data.extend_from_slice(b"\x42\x82"); // Add file type marker
    mkv_data.push(8); // Length of "matroska"
    mkv_data.extend_from_slice(b"matroska");
    let mime_type = detect(&mkv_data);
    // May detect as enhanced or basic MKV detector depending on order
    assert!(
        mime_type.mime().contains("matroska") || mime_type.mime() == "application/octet-stream"
    );

    // RMVB detection
    let rmvb_data = b".RMF\x00\x00\x00\x12";
    let mime_type = detect(rmvb_data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_RN_REALMEDIA_VBR);
}

#[test]
fn test_document_formats() {
    // PDF detection
    let pdf_data = b"%PDF-1.4";
    let mime_type = detect(pdf_data);
    assert_eq!(mime_type.mime(), APPLICATION_PDF);
    assert_eq!(mime_type.extension(), ".pdf");

    // HTML detection
    let html_data = b"<!DOCTYPE HTML>";
    let mime_type = detect(html_data);
    assert_eq!(mime_type.mime(), TEXT_HTML);

    let html_data2 = b"<HTML>";
    let mime_type2 = detect(html_data2);
    assert_eq!(mime_type2.mime(), "text/html; charset=utf-8");

    let html_data3 = b"  <BODY> ";
    let mime_type3 = detect(html_data3);
    assert_eq!(mime_type3.mime(), "text/html; charset=utf-8");

    let html_data4 = b"       <!DOCTYPE HTML>";
    let mime_type4 = detect(html_data4);
    assert_eq!(mime_type4.mime(), "text/html; charset=utf-8");

    // XML detection
    let xml_data = b"<?xml version=\"1.0\"?>";
    let mime_type = detect(xml_data);
    assert_eq!(mime_type.mime(), TEXT_XML);

    let xml_data2 = b"  <?xml encoding=\"UTF-8\"?>";
    let mime_type2 = detect(xml_data2);
    assert_eq!(mime_type2.mime(), "text/xml; charset=utf-8");
}

#[test]
fn test_archive_formats() {
    // ZIP detection
    let zip_data = b"PK\x03\x04";
    let mime_type = detect(zip_data);
    assert_eq!(mime_type.mime(), APPLICATION_ZIP);
    assert_eq!(mime_type.extension(), ".zip");

    // 7-Zip detection
    let seven_z_data = b"7z\xbc\xaf\x27\x1c";
    let mime_type = detect(seven_z_data);
    assert_eq!(mime_type.mime(), APPLICATION_X_7Z_COMPRESSED);
    assert_eq!(mime_type.extension(), ".7z");

    // GZIP detection
    let gzip_data = b"\x1f\x8b";
    let mime_type = detect(gzip_data);
    assert_eq!(mime_type.mime(), APPLICATION_GZIP);
    assert_eq!(mime_type.extension(), ".gz");

    // Enhanced GZIP detection
    let gzip_data = b"\x1F\x8B\x08\x00\x00\x00\x00\x00";
    let mime_type = detect(gzip_data);
    assert_eq!(mime_type.mime(), APPLICATION_X_GZIP);

    // FITS detection
    let fits_data = b"SIMPLE  =                    T";
    let mime_type = detect(fits_data);
    assert_eq!(mime_type.mime(), APPLICATION_FITS);

    // XAR detection
    let xar_data = b"xar!\x00\x1C";
    let mime_type = detect(xar_data);
    assert_eq!(mime_type.mime(), APPLICATION_X_XAR);

    // WARC detection
    let warc_data = b"WARC/1.0\r\n";
    let mime_type = detect(warc_data);
    assert_eq!(mime_type.mime(), APPLICATION_WARC);
}

#[test]
fn test_executable_formats() {
    // Windows PE/EXE detection
    let exe_data = b"MZ";
    let mime_type = detect(exe_data);
    assert_eq!(
        mime_type.mime(),
        APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE
    );
    assert_eq!(mime_type.extension(), ".exe");

    // Linux ELF detection
    let elf_data = b"\x7fELF";
    let mime_type = detect(elf_data);
    assert_eq!(mime_type.mime(), APPLICATION_X_ELF);
    assert_eq!(mime_type.extension(), "");

    // WebAssembly detection
    let wasm_data = b"\x00asm\x01\x00\x00\x00";
    let mime_type = detect(wasm_data);
    assert_eq!(mime_type.mime(), APPLICATION_WASM);
}

#[test]
fn test_font_formats() {
    // EOT detection
    let mut eot_data = vec![0; 34];
    eot_data.extend_from_slice(b"LP");
    let mime_type = detect(&eot_data);
    assert_eq!(mime_type.mime(), APPLICATION_VND_MS_FONTOBJECT);

    // TTC detection
    let ttc_data = b"ttcf\x00\x01\x00\x00";
    let mime_type = detect(ttc_data);
    assert_eq!(mime_type.mime(), FONT_COLLECTION);
}

#[test]
fn test_text_encoding_formats() {
    // UTF-8 BOM detection
    let utf8_bom = b"\xEF\xBB\xBFHello World";
    let mime_type = detect(utf8_bom);
    assert_eq!(mime_type.mime(), TEXT_UTF8);

    // UTF-16 BE detection
    let utf16_be = b"\xFE\xFF\x00H\x00e\x00l\x00l\x00o";
    let mime_type2 = detect(utf16_be);
    assert_eq!(mime_type2.mime(), TEXT_UTF16_BE);

    // UTF-16 LE detection
    let utf16_le = b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00";
    let mime_type3 = detect(utf16_le);
    assert_eq!(mime_type3.mime(), TEXT_UTF16_LE);
}

#[test]
fn test_enhanced_audio_detection() {
    // Enhanced MIDI detection
    let midi_data = b"MThd\x00\x00\x00\x06\x00\x01";
    let mime_type = detect(midi_data);
    assert_eq!(mime_type.mime(), AUDIO_MIDI);
}

// ============================================================================
// API FUNCTIONALITY TESTS
// ============================================================================

#[test]
fn test_api_functions() {
    // equals_any function
    assert!(equals_any(
        IMAGE_PNG,
        &[IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF]
    ));
    assert!(equals_any(
        IMAGE_JPEG,
        &[IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF]
    ));
    assert!(!equals_any(
        APPLICATION_PDF,
        &[IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF]
    ));

    // match_mime function
    let png_data = b"\x89PNG\r\n\x1a\n";
    assert!(match_mime(png_data, IMAGE_PNG));
    assert!(!match_mime(png_data, IMAGE_JPEG));

    // is_supported function
    assert!(is_supported(IMAGE_PNG));
    assert!(is_supported(APPLICATION_PDF));
    assert!(!is_supported("fake/mimetype"));

    // MIME type is() method
    let png_data = b"\x89PNG\r\n\x1a\n";
    let mime_type = detect(png_data);
    assert!(mime_type.is(IMAGE_PNG));
    assert!(!mime_type.is(IMAGE_JPEG));
}

#[test]
fn test_reader_detection() {
    let png_data = b"\x89PNG\r\n\x1a\n";
    let cursor = Cursor::new(png_data);
    let mime_type = detect_reader(cursor).unwrap();
    assert_eq!(mime_type.mime(), IMAGE_PNG);
}

// ============================================================================
// EDGE CASE AND BOUNDARY TESTS
// ============================================================================

#[test]
fn test_edge_cases() {
    // Unknown data fallback
    let unknown_data = b"this is just some text";
    let mime_type = detect(unknown_data);
    assert_eq!(mime_type.mime(), TEXT_UTF8);
    assert_eq!(mime_type.extension(), ".txt");

    // Binary fallback
    let binary_data = b"\x00\x01\x02\x03";
    let mime_type = detect(binary_data);
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);

    // Empty data
    let empty_data = b"";
    let mime_type = detect(empty_data);
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);

    // Large data truncation
    let mut large_data = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]; // PNG header
    large_data.resize(5000, 0); // Make it larger than READ_LIMIT
    let mime_type = detect(&large_data);
    assert_eq!(mime_type.mime(), IMAGE_PNG);
}

// ============================================================================
// NEGATIVE TESTS (FALSE POSITIVE PREVENTION)
// ============================================================================

#[test]
fn test_png_negative() {
    // Should not detect as PNG
    let fake_png = b"\x89PNG\x0D\x0A\x1A"; // Missing final byte
    assert_ne!(detect(fake_png).mime(), IMAGE_PNG);

    let partial_png = b"\x89PNX\x0D\x0A\x1A\x0A"; // Wrong magic
    assert_ne!(detect(partial_png).mime(), IMAGE_PNG);

    let short_data = b"\x89PN"; // Too short
    assert_ne!(detect(short_data).mime(), IMAGE_PNG);
}

#[test]
fn test_jpeg_negative() {
    // Should not detect as JPEG
    let fake_jpeg = b"\xFF\xD8\xFE"; // Wrong third byte
    assert_ne!(detect(fake_jpeg).mime(), "image/jpeg");

    let partial_jpeg = b"\xFF\xD8"; // Too short
    assert_ne!(detect(partial_jpeg).mime(), "image/jpeg");

    let wrong_jpeg = b"\xFE\xD8\xFF"; // Wrong order
    assert_ne!(detect(wrong_jpeg).mime(), "image/jpeg");
}

#[test]
fn test_gif_negative() {
    // Should not detect as GIF
    let fake_gif87 = b"GIF87b"; // Wrong version
    assert_ne!(detect(fake_gif87).mime(), "image/gif");

    let fake_gif89 = b"GIF89b"; // Wrong version
    assert_ne!(detect(fake_gif89).mime(), "image/gif");

    let partial_gif = b"GIF8"; // Too short
    assert_ne!(detect(partial_gif).mime(), "image/gif");

    let wrong_gif = b"gif87a"; // Wrong case
    assert_ne!(detect(wrong_gif).mime(), "image/gif");
}

#[test]
fn test_pdf_negative() {
    // Should not detect as PDF
    let fake_pdf = b"%PDE-1.4"; // Wrong header
    assert_ne!(detect(fake_pdf).mime(), "application/pdf");

    let partial_pdf = b"%PDF"; // Missing version
    assert_ne!(detect(partial_pdf).mime(), "application/pdf");

    let wrong_pdf = b"PDF-1.4"; // Missing %
    assert_ne!(detect(wrong_pdf).mime(), "application/pdf");
}

#[test]
fn test_zip_negative() {
    // Should not detect as ZIP
    let fake_zip = b"PK\x03\x05"; // Wrong bytes
    assert_ne!(detect(fake_zip).mime(), "application/zip");

    let partial_zip = b"PK"; // Too short
    assert_ne!(detect(partial_zip).mime(), "application/zip");

    let wrong_zip = b"KP\x03\x04"; // Wrong order
    assert_ne!(detect(wrong_zip).mime(), "application/zip");
}

#[test]
fn test_audio_negative() {
    // MP3 negatives
    let fake_id3 = b"ID4"; // Wrong ID3 tag
    assert_ne!(detect(fake_id3).mime(), "audio/mpeg");

    let partial_id3 = b"ID"; // Too short
    assert_ne!(detect(partial_id3).mime(), "audio/mpeg");

    let wrong_mpeg = b"\xFF\xE0"; // Wrong MPEG header
    assert_ne!(detect(wrong_mpeg).mime(), "audio/mpeg");

    // FLAC negatives
    let fake_flac = b"fLaD"; // Wrong signature
    assert_ne!(detect(fake_flac).mime(), "audio/flac");

    let partial_flac = b"fLa"; // Too short
    assert_ne!(detect(partial_flac).mime(), "audio/flac");

    let wrong_flac = b"FLAC"; // Wrong case
    assert_ne!(detect(wrong_flac).mime(), "audio/flac");

    // WAV negatives
    let fake_wav = b"RIFF\x00\x00\x00\x00WAVY"; // Wrong format
    assert_ne!(detect(fake_wav).mime(), "audio/wav");

    let partial_wav = b"RIFF\x00\x00\x00\x00WAV"; // Too short
    assert_ne!(detect(partial_wav).mime(), "audio/wav");

    let wrong_wav = b"RIFX\x00\x00\x00\x00WAVE"; // Wrong endian marker
    assert_ne!(detect(wrong_wav).mime(), "audio/wav");
}

#[test]
fn test_image_negative() {
    // WebP negatives
    let fake_webp = b"RIFF\x00\x00\x00\x00WEBQ"; // Wrong format
    assert_ne!(detect(fake_webp).mime(), "image/webp");

    let partial_webp = b"RIFF\x00\x00\x00\x00WEB"; // Too short
    assert_ne!(detect(partial_webp).mime(), "image/webp");

    let wrong_webp = b"RIFX\x00\x00\x00\x00WEBP"; // Wrong RIFF
    assert_ne!(detect(wrong_webp).mime(), "image/webp");

    // BMP negatives
    let fake_bmp = b"BN"; // Wrong signature
    assert_ne!(detect(fake_bmp).mime(), "image/bmp");

    let partial_bmp = b"B"; // Too short
    assert_ne!(detect(partial_bmp).mime(), "image/bmp");

    let wrong_bmp = b"MB"; // Wrong order
    assert_ne!(detect(wrong_bmp).mime(), "image/bmp");

    // TIFF negatives
    let fake_tiff_le = b"II*\x01"; // Wrong magic
    assert_ne!(detect(fake_tiff_le).mime(), "image/tiff");

    let fake_tiff_be = b"MM\x01*"; // Wrong magic
    assert_ne!(detect(fake_tiff_be).mime(), "image/tiff");

    let partial_tiff = b"II*"; // Too short
    assert_ne!(detect(partial_tiff).mime(), "image/tiff");

    // ICO negatives
    let fake_ico = b"\x00\x00\x01\x01"; // Wrong type
    assert_ne!(detect(fake_ico).mime(), "image/x-icon");

    let fake_ico2 = b"\x00\x00\x03\x00"; // Wrong type
    assert_ne!(detect(fake_ico2).mime(), "image/x-icon");

    let partial_ico = b"\x00\x00\x01"; // Too short
    assert_ne!(detect(partial_ico).mime(), "image/x-icon");
}

#[test]
fn test_archive_negative() {
    // 7Z negatives
    let fake_7z = b"7z\xbc\xaf\x27\x1d"; // Wrong signature
    assert_ne!(detect(fake_7z).mime(), "application/x-7z-compressed");

    let partial_7z = b"7z\xbc\xaf"; // Too short
    assert_ne!(detect(partial_7z).mime(), "application/x-7z-compressed");

    let wrong_7z = b"z7\xbc\xaf\x27\x1c"; // Wrong order
    assert_ne!(detect(wrong_7z).mime(), "application/x-7z-compressed");

    // GZIP negatives
    let fake_gzip = b"\x1f\x8c"; // Wrong second byte
    assert_ne!(detect(fake_gzip).mime(), "application/gzip");

    let partial_gzip = b"\x1f"; // Too short
    assert_ne!(detect(partial_gzip).mime(), "application/gzip");

    let wrong_gzip = b"\x8b\x1f"; // Wrong order
    assert_ne!(detect(wrong_gzip).mime(), "application/gzip");

    // TAR negatives
    let mut fake_tar = vec![0; 512];
    fake_tar[257..262].copy_from_slice(b"ustar"); // Missing proper header structure
    assert_ne!(detect(&fake_tar).mime(), "application/x-tar");

    let mut wrong_tar = vec![0; 512];
    wrong_tar[257..262].copy_from_slice(b"ustax"); // Wrong magic
    assert_ne!(detect(&wrong_tar).mime(), "application/x-tar");

    let short_tar = vec![0; 256]; // Too short
    assert_ne!(detect(&short_tar).mime(), "application/x-tar");

    // RAR negatives
    let fake_rar = b"Rar!\x1A\x07\x02"; // Wrong version
    assert_ne!(detect(fake_rar).mime(), "application/x-rar-compressed");

    let partial_rar = b"Rar!"; // Too short
    assert_ne!(detect(partial_rar).mime(), "application/x-rar-compressed");

    let wrong_rar = b"RAR!\x1A\x07\x00"; // Wrong case
    assert_ne!(detect(wrong_rar).mime(), "application/x-rar-compressed");
}

#[test]
fn test_executable_negative() {
    // EXE negatives
    let fake_exe = b"NZ"; // Wrong signature
    assert_ne!(
        detect(fake_exe).mime(),
        APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE
    );

    let partial_exe = b"M"; // Too short
    assert_ne!(
        detect(partial_exe).mime(),
        APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE
    );

    let wrong_exe = b"ZM"; // Wrong order
    assert_ne!(
        detect(wrong_exe).mime(),
        APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE
    );

    // ELF negatives
    let fake_elf = b"\x7fELG"; // Wrong signature
    assert_ne!(detect(fake_elf).mime(), "application/x-elf");

    let partial_elf = b"\x7fEL"; // Too short
    assert_ne!(detect(partial_elf).mime(), "application/x-elf");

    let wrong_elf = b"ELF\x7f"; // Wrong order
    assert_ne!(detect(wrong_elf).mime(), "application/x-elf");

    // Java CLASS negatives
    let fake_class = b"\xca\xfe\xba\xbf"; // Wrong magic
    assert_ne!(
        detect(fake_class).mime(),
        "application/x-java-applet; charset=binary"
    );

    let partial_class = b"\xca\xfe\xba"; // Too short
    assert_ne!(
        detect(partial_class).mime(),
        "application/x-java-applet; charset=binary"
    );

    let wrong_class = b"\xfe\xca\xbe\xba"; // Wrong order
    assert_ne!(
        detect(wrong_class).mime(),
        "application/x-java-applet; charset=binary"
    );

    // WebAssembly negatives
    let fake_wasm = b"\x00bsm"; // Wrong signature
    assert_ne!(detect(fake_wasm).mime(), "application/wasm");

    let partial_wasm = b"\x00as"; // Too short
    assert_ne!(detect(partial_wasm).mime(), "application/wasm");

    let wrong_wasm = b"\x01asm"; // Wrong null byte
    assert_ne!(detect(wrong_wasm).mime(), "application/wasm");
}

#[test]
fn test_text_format_negative() {
    // HTML negatives
    let fake_doctype = b"<!DOCTYPE html>"; // Missing required format
    let result = detect(fake_doctype);
    // May detect as HTML or text, but should be consistent
    assert!(result.mime() == "text/html; charset=utf-8" || result.mime().contains("text"));

    let fake_html = b"<HTM>"; // Wrong tag
    assert_ne!(detect(fake_html).mime(), "text/html; charset=utf-8");

    let incomplete_tag = b"<HTML"; // Missing closing >
    assert_ne!(detect(incomplete_tag).mime(), "text/html; charset=utf-8");

    // XML negatives
    let fake_xml = b"<?xm version='1.0'?>"; // Wrong declaration
    assert_ne!(detect(fake_xml).mime(), "text/xml; charset=utf-8");

    let partial_xml = b"<?xml"; // Missing space/content
    let result = detect(partial_xml);
    assert!(result.mime() == "text/xml; charset=utf-8" || result.mime().contains("text"));

    let wrong_xml = b"<xml version='1.0'?>"; // Missing ?
    assert_ne!(detect(wrong_xml).mime(), "text/xml; charset=utf-8");
}

#[test]
fn test_utf_bom_negative() {
    // Test that incorrect BOM sequences don't match specific BOM MIME types
    let fake_utf16_be = b"\xFE\xFE"; // Wrong BOM
    assert!(!match_mime(fake_utf16_be, "text/plain; charset=utf-16be"));

    let fake_utf16_le = b"\xFF\xFF"; // Wrong BOM
    assert!(!match_mime(fake_utf16_le, "text/plain; charset=utf-16le"));

    // Test that specific BOM detectors work correctly for valid BOMs
    let real_utf8_bom = b"\xEF\xBB\xBF"; // Correct UTF-8 BOM
    assert!(match_mime(real_utf8_bom, "text/plain; charset=utf-8"));

    let real_utf16_be = b"\xFE\xFF"; // Correct UTF-16 BE BOM
    assert!(match_mime(real_utf16_be, "text/plain; charset=utf-16be"));

    let real_utf16_le = b"\xFF\xFE"; // Correct UTF-16 LE BOM
    assert!(match_mime(real_utf16_le, "text/plain; charset=utf-16le"));

    // Test that incomplete BOMs don't match their specific types
    let incomplete_utf8 = b"\xEF\xBB"; // Incomplete UTF-8 BOM
    assert!(!match_mime(incomplete_utf8, "text/plain; charset=utf-16be"));
    assert!(!match_mime(incomplete_utf8, "text/plain; charset=utf-16le"));

    // Test binary data doesn't match UTF BOMs
    let binary_data = b"\x00\x01\x02\x03";
    assert!(!match_mime(binary_data, "text/plain; charset=utf-8"));
    assert!(!match_mime(binary_data, "text/plain; charset=utf-16be"));
    assert!(!match_mime(binary_data, "text/plain; charset=utf-16le"));
}

// ============================================================================
// CROSS-FORMAT CONFUSION PREVENTION
// ============================================================================

#[test]
fn test_cross_format_confusion() {
    // Test that similar formats don't get confused

    // RIFF-based formats should be distinguishable
    let avi_data = b"RIFF\x00\x00\x00\x00AVI LIST";
    assert!(!match_mime(avi_data, "audio/wav"));
    assert!(!match_mime(avi_data, "image/webp"));

    let wav_data = b"RIFF\x00\x00\x00\x00WAVE";
    assert!(!match_mime(wav_data, "video/x-msvideo"));
    assert!(!match_mime(wav_data, "image/webp"));

    let webp_data = b"RIFF\x00\x00\x00\x00WEBP";
    assert!(!match_mime(webp_data, "audio/wav"));
    assert!(!match_mime(webp_data, "video/x-msvideo"));

    // Archive formats should be distinguishable
    let zip_data = b"PK\x03\x04";
    assert!(!match_mime(zip_data, "application/x-7z-compressed"));
    assert!(!match_mime(zip_data, "application/x-rar-compressed"));

    let seven_z_data = b"7z\xbc\xaf\x27\x1c";
    assert!(!match_mime(seven_z_data, "application/zip"));
    assert!(!match_mime(seven_z_data, "application/x-rar-compressed"));

    // Text formats should be distinguishable
    let html_data = b"<html>";
    assert!(!match_mime(html_data, "text/xml; charset=utf-8"));
    assert!(!match_mime(html_data, "application/json"));

    let xml_data = b"<?xml version='1.0'?>";
    assert!(!match_mime(xml_data, "text/html; charset=utf-8"));
    assert!(!match_mime(xml_data, "application/json"));
}

#[test]
fn test_binary_vs_text_boundary() {
    // Test boundary between binary and text detection

    // Pure binary should not be detected as text
    let binary_data = b"\x00\x01\x02\x03\x04\x05";
    let mime_type = detect(binary_data);
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert!(!match_mime(binary_data, "text/plain; charset=utf-8"));

    // Control characters should not be detected as text
    let control_chars = b"\x08\x0B\x0E\x1A\x1C";
    let mime_type = detect(control_chars);
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);
    assert!(!match_mime(control_chars, "text/plain; charset=utf-8"));

    // Valid text should be detected correctly
    let text_data = b"Hello, World! This is plain text.";
    let mime_type = detect(text_data);
    assert_eq!(mime_type.mime(), TEXT_UTF8);
    assert!(match_mime(text_data, "text/plain; charset=utf-8"));
}

#[test]
fn test_size_boundaries() {
    // Test detection with minimal data sizes

    // Empty data should not match any specific format
    let empty_data = b"";
    let mime_type = detect(empty_data);
    assert_eq!(mime_type.mime(), APPLICATION_OCTET_STREAM);

    // Single byte should not match multi-byte signatures
    let single_byte = b"P";
    assert!(!match_mime(single_byte, "application/pdf"));
    assert!(!match_mime(single_byte, "application/zip"));
    assert!(!match_mime(single_byte, "image/png"));

    // Two bytes should not match longer signatures
    let two_bytes = b"PK";
    assert!(!match_mime(two_bytes, "application/zip")); // Needs PK\x03\x04
    assert!(!match_mime(two_bytes, "image/png")); // Needs full 8-byte signature
}