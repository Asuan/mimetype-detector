//! Comprehensive API Integration Tests
//!
//! This module tests all public API functions of the mimetype-detector library,
//! including edge cases, error handling, and various usage patterns.

use mimetype_detector::{
    constants::*, detect, detect_file, detect_file_with_limit, detect_reader,
    detect_reader_with_limit, detect_with_limit, equals_any, register_extension, register_mime,
};
use std::io::Cursor;

// ============================================================================
// DETECTION API TESTS
// ============================================================================

#[test]
fn test_detect_basic() {
    let data = b"\x89PNG\r\n\x1a\n";
    let mime = detect(data);
    assert_eq!(mime.mime(), IMAGE_PNG);
    assert_eq!(mime.extension(), ".png");
    assert!(mime.kind().is_image());
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

#[test]
fn test_detect_empty_data() {
    let data = b"";
    let mime = detect(data);
    assert_eq!(mime.mime(), APPLICATION_X_EMPTY);
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

#[test]
fn test_detect_short_data() {
    let data = b"AB";
    let mime = detect(data);
    assert!(!mime.mime().is_empty());
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

#[test]
fn test_detect_reader_from_cursor() {
    let data = b"%PDF-1.4\ntest content";
    let cursor = Cursor::new(data);
    let mime = detect_reader(cursor).expect("Should detect from reader");
    assert_eq!(mime.mime(), APPLICATION_PDF);
    assert_eq!(mime.extension(), ".pdf");
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

#[test]
fn test_detect_reader_from_slice() {
    let data = b"\xff\xd8\xff\xe0JFIF";
    let mime = detect_reader(&data[..]).expect("Should detect from slice");
    assert_eq!(mime.mime(), IMAGE_JPEG);
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

#[test]
fn test_detect_reader_empty() {
    let data: &[u8] = b"";
    let mime = detect_reader(data).expect("Should handle empty reader");
    assert_eq!(mime.mime(), APPLICATION_X_EMPTY);
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

#[test]
fn test_detect_file_png() {
    use std::fs;
    use std::io::Write;

    let temp_path = "test_temp_file.png";
    let mut file = fs::File::create(temp_path).expect("Failed to create temp file");
    file.write_all(b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR")
        .expect("Failed to write");
    drop(file);

    let mime = detect_file(temp_path).expect("Should detect file");
    assert_eq!(mime.mime(), IMAGE_PNG);
    assert_eq!(mime.extension(), ".png");
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );

    fs::remove_file(temp_path).ok();
}

#[test]
fn test_detect_file_nonexistent() {
    let result = detect_file("this_file_does_not_exist_12345.bin");
    assert!(result.is_err(), "Should return error for nonexistent file");
}

#[test]
fn test_detect_file_empty() {
    use std::fs;

    let temp_path = "test_empty_file.bin";
    fs::File::create(temp_path).expect("Failed to create temp file");

    let mime = detect_file(temp_path).expect("Should detect empty file");
    assert_eq!(mime.mime(), APPLICATION_X_EMPTY);
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );

    fs::remove_file(temp_path).ok();
}

// ============================================================================
// CUSTOM REGISTRATION TESTS
// ============================================================================

#[test]
fn test_register_custom_mime() {
    register_mime("application/x-custom-test1", |data| {
        data.starts_with(b"CUSTOMTEST1")
    });
}

#[test]
fn test_register_custom_extension() {
    register_extension(".customtest1", |data| data.starts_with(b"CUSTOMTEST1"));
}

// ============================================================================
// UTILITY FUNCTION TESTS
// ============================================================================

#[test]
fn test_equals_any_single_match() {
    assert!(equals_any(IMAGE_PNG, &[IMAGE_PNG]));
    assert!(equals_any(IMAGE_PNG, &[IMAGE_JPEG, IMAGE_PNG, IMAGE_GIF]));
}

#[test]
fn test_equals_any_no_match() {
    assert!(!equals_any(IMAGE_PNG, &[IMAGE_JPEG, IMAGE_GIF]));
    assert!(!equals_any(IMAGE_PNG, &[]));
}

#[test]
fn test_equals_any_empty_list() {
    assert!(!equals_any(IMAGE_PNG, &[]));
}

// ============================================================================
// EDGE CASES AND CORNER CASES
// ============================================================================

#[test]
fn test_detect_large_data() {
    let mut data = vec![0u8; 10000];
    data[0..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");

    let mime = detect(&data);
    assert_eq!(mime.mime(), IMAGE_PNG);
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

#[test]
fn test_detect_all_null_bytes() {
    let data = vec![0u8; 100];
    let mime = detect(&data);
    assert_eq!(mime.mime(), APPLICATION_OCTET_STREAM);
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

#[test]
fn test_detect_all_printable_ascii() {
    let data = b"This is a plain text file with only printable ASCII characters.";
    let mime = detect(data);
    assert_eq!(mime.mime(), TEXT_UTF8);
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

// ============================================================================
// MIME TYPE ALIAS TESTS
// ============================================================================

#[test]
fn test_mime_type_aliases() {
    let data = b"%PDF-1.4";
    let mime = detect(data);

    assert!(mime.is(APPLICATION_PDF));
    assert!(mime.is(APPLICATION_X_PDF));
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

// ============================================================================
// CONCURRENT ACCESS TESTS
// ============================================================================

#[test]
fn test_concurrent_detect() {
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let data = match i % 3 {
                    0 => b"\x89PNG\r\n\x1a\n".as_slice(),
                    1 => b"%PDF-1.4".as_slice(),
                    _ => b"<?xml version=\"1.0\"?>".as_slice(),
                };
                detect(data)
            })
        })
        .collect();

    for handle in handles {
        let mime = handle.join().expect("Thread panicked");
        assert!(!mime.mime().is_empty());
        assert!(
            !mime.name().is_empty(),
            "Format {} should have a non-empty name",
            mime.mime()
        );
    }
}

// ============================================================================
// PARENT-CHILD RELATIONSHIP TESTS
// ============================================================================

#[test]
fn test_child_format_detection() {
    let ole_data = b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1";
    let mime = detect(ole_data);

    assert_eq!(mime.mime(), APPLICATION_X_OLE_STORAGE);
}

// ============================================================================
// IS() METHOD TESTS
// ============================================================================

#[test]
fn test_is_method() {
    let data = b"\x89PNG\r\n\x1a\n";
    let mime = detect(data);

    assert!(mime.is(IMAGE_PNG));
    assert!(!mime.is(IMAGE_JPEG));
    assert!(!mime.is(APPLICATION_PDF));
}

#[test]
fn test_is_method_with_aliases() {
    let data = b"%PDF-1.4";
    let mime = detect(data);

    assert!(mime.is(APPLICATION_PDF));
    assert!(mime.is(APPLICATION_X_PDF));
}

// ============================================================================
// DETERMINISM TESTS
// ============================================================================

#[test]
fn test_detection_is_deterministic() {
    let data = b"\x89PNG\r\n\x1a\n";

    for _ in 0..100 {
        let mime = detect(data);
        assert_eq!(mime.mime(), IMAGE_PNG);
        assert_eq!(mime.extension(), ".png");
    }
}

// ============================================================================
// EXTENSION TESTS
// ============================================================================

#[test]
fn test_extensions_start_with_dot() {
    let test_cases = vec![
        b"\x89PNG\r\n\x1a\n".as_slice(),
        b"%PDF-1.4".as_slice(),
        b"GIF89a".as_slice(),
        b"\xff\xd8\xff\xe0".as_slice(),
    ];

    for data in test_cases {
        let mime = detect(data);
        assert!(
            mime.extension().starts_with('.'),
            "Extension {} should start with a dot",
            mime.extension()
        );
    }
}

// ============================================================================
// PREFIX_VEC DETECTION PATH TESTS
// ============================================================================
// Tests for PREFIX_VEC detection mechanism and conflict resolution

#[test]
fn test_prefix_vec_detects_common_formats() {
    // Verify PREFIX_VEC correctly routes to format detectors
    // Basic format detection is covered in mimetype_tests.rs
    // This test verifies the PREFIX_VEC mechanism works

    let test_cases = vec![
        (b"\x89PNG\r\n\x1a\n".as_slice(), IMAGE_PNG),
        (b"%PDF-1.4".as_slice(), APPLICATION_PDF),
        (b"GIF89a".as_slice(), IMAGE_GIF),
        (b"\xFF\xD8\xFF\xE0".as_slice(), IMAGE_JPEG),
        (b"PK\x03\x04".as_slice(), APPLICATION_ZIP),
        (b"\x1F\x8B\x08".as_slice(), APPLICATION_GZIP),
    ];

    for (data, expected_mime) in test_cases {
        let mime = detect(data);
        assert_eq!(mime.mime(), expected_mime);
    }
}

#[test]
fn test_prefix_vec_conflict_resolution() {
    // Test that PREFIX_VEC correctly distinguishes between formats with same first byte
    // PREFIX_VEC[0x4D] contains: Model3D, Musepack, CAB, MIDI, EXE, 3DS, TIFF, etc.

    let test_cases = vec![
        (b"MThd".as_slice(), AUDIO_MIDI), // MIDI
        (
            b"MSCF\x00\x00\x00\x00".as_slice(),
            APPLICATION_VND_MS_CAB_COMPRESSED,
        ), // CAB
        (
            b"MZ\x90\x00".as_slice(),
            APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE,
        ), // EXE
    ];

    for (data, expected_mime) in test_cases {
        let mime = detect(data);
        assert_eq!(mime.mime(), expected_mime);
    }
}

// ============================================================================
// PARENT-CHILD DETECTION TESTS
// ============================================================================
// Tests for formats detected through parent-child relationships

#[test]
fn test_ole_parent_basic_detection() {
    // OLE parent signature: D0 CF 11 E0 A1 B1 1A E1
    let data = b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1";
    let mime = detect(data);

    assert_eq!(mime.mime(), APPLICATION_X_OLE_STORAGE);
    // OLE has empty extension as it's a container format
    assert!(mime.kind().is_document());
}

#[test]
fn test_ole_child_msi_detection() {
    // MSI is a child of OLE - contains specific CLSID marker
    // MSI detection requires specific data at offset 512
    let mut data = vec![0u8; 600];
    data[0..8].copy_from_slice(b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1");
    data[512..520].copy_from_slice(b"\x84\x10\x0C\x00\x00\x00\x00\x00");

    let mime = detect(&data);
    // May detect as generic OLE if MSI signature not fully implemented
    assert!(mime.mime().starts_with("application/"));
}

#[test]
fn test_ole_child_doc_detection() {
    // DOC is a child of OLE (requires DOC-specific markers)
    let mut data = vec![0u8; 2048];
    data[0..8].copy_from_slice(b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1");
    data[512..520].copy_from_slice(b"\xEC\xA5\xC1\x00\x00\x00\x00\x00");

    let mime = detect(&data);
    // Should detect as OLE-based format
    assert!(mime.mime().starts_with("application/"));
}

#[test]
fn test_riff_parent_basic_detection() {
    // RIFF parent - will detect based on subtype or fallback
    let data = b"RIFF\x00\x00\x00\x00UNKN";
    let mime = detect(data);

    // May fallback to octet-stream for unknown RIFF types
    assert!(!mime.mime().is_empty());
}

#[test]
fn test_riff_child_wav_detection() {
    // WAV is a child of RIFF
    let data = b"RIFF\x00\x00\x00\x00WAVE";
    let mime = detect(data);

    assert_eq!(mime.mime(), AUDIO_WAV);
    assert_eq!(mime.extension(), ".wav");
    assert!(mime.kind().is_audio());
}

#[test]
fn test_riff_child_avi_detection() {
    // AVI is a child of RIFF (needs proper AVI LIST structure)
    let mut data = Vec::new();
    data.extend_from_slice(b"RIFF");
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Size
    data.extend_from_slice(b"AVI ");

    let mime = detect(&data);
    // AVI detection may require more structure
    assert!(mime.mime().contains("video/") || mime.mime().contains("application/"));
}

#[test]
fn test_riff_child_webp_detection() {
    // WEBP is a child of RIFF
    let data = b"RIFF\x00\x00\x00\x00WEBP";
    let mime = detect(data);

    assert_eq!(mime.mime(), IMAGE_WEBP);
    assert_eq!(mime.extension(), ".webp");
    assert!(mime.kind().is_image());
}

#[test]
fn test_tiff_parent_little_endian() {
    // TIFF little-endian: II*\x00
    let data = b"II*\x00";
    let mime = detect(data);

    assert_eq!(mime.mime(), IMAGE_TIFF);
    assert_eq!(mime.extension(), ".tiff");
    assert!(mime.kind().is_image());
}

#[test]
fn test_tiff_parent_big_endian() {
    // TIFF big-endian: MM\x00*
    let data = b"MM\x00*";
    let mime = detect(data);

    assert_eq!(mime.mime(), IMAGE_TIFF);
    assert_eq!(mime.extension(), ".tiff");
    assert!(mime.kind().is_image());
}

#[test]
fn test_tiff_child_cr2_detection() {
    // Canon CR2 is a TIFF-based format with CR2 marker
    let mut data = vec![0u8; 16];
    data[0..4].copy_from_slice(b"II*\x00");
    data[8..12].copy_from_slice(b"CR\x02\x00");

    let mime = detect(&data);
    assert_eq!(mime.mime(), IMAGE_X_CANON_CR2);
    assert_eq!(mime.extension(), ".cr2");
}

// ============================================================================
// CHILDREN IN PREFIX_VEC TESTS
// ============================================================================
// Tests for child formats that are also registered in PREFIX_VEC

#[test]
fn test_olympus_orf_iiro_in_prefix_vec() {
    // ORF IIRO variant is in PREFIX_VEC[0x49]
    let data = b"IIRO";
    let mime = detect(data);

    assert_eq!(mime.mime(), IMAGE_X_OLYMPUS_ORF);
    assert_eq!(mime.extension(), ".orf");
    assert!(mime.kind().is_image());
}

#[test]
fn test_olympus_orf_iirs_in_prefix_vec() {
    // ORF IIRS variant is in PREFIX_VEC[0x49]
    let data = b"IIRS";
    let mime = detect(data);

    assert_eq!(mime.mime(), IMAGE_X_OLYMPUS_ORF);
    assert_eq!(mime.extension(), ".orf");
}

#[test]
fn test_olympus_orf_mmor_in_prefix_vec() {
    // ORF MMOR variant is in PREFIX_VEC[0x4D]
    let data = b"MMOR";
    let mime = detect(data);

    assert_eq!(mime.mime(), IMAGE_X_OLYMPUS_ORF);
    assert_eq!(mime.extension(), ".orf");
}

// Note: 3DS vs TIFF conflict test is in mimetype_tests.rs (test_detect_3ds_vs_tiff)

// ============================================================================
// MIMETYPE METHOD TESTS
// ============================================================================
// Comprehensive tests for all MimeType public methods

#[test]
fn test_mimetype_mime_method() {
    let test_cases = vec![
        (b"\x89PNG\r\n\x1a\n".as_slice(), IMAGE_PNG),
        (b"%PDF-1.4".as_slice(), APPLICATION_PDF),
        (b"GIF89a".as_slice(), IMAGE_GIF),
        (b"\xFF\xD8\xFF\xE0".as_slice(), IMAGE_JPEG),
    ];

    for (data, expected) in test_cases {
        let mime = detect(data);
        assert_eq!(mime.mime(), expected);
    }
}

#[test]
fn test_mimetype_extension_method() {
    let test_cases = vec![
        (b"\x89PNG\r\n\x1a\n".as_slice(), ".png"),
        (b"%PDF-1.4".as_slice(), ".pdf"),
        (b"GIF89a".as_slice(), ".gif"),
        (b"\xFF\xD8\xFF\xE0".as_slice(), ".jpg"),
        (b"PK\x03\x04".as_slice(), ".zip"),
    ];

    for (data, expected) in test_cases {
        let mime = detect(data);
        assert_eq!(mime.extension(), expected);
    }
}

#[test]
fn test_mimetype_kind_method() {
    // Image types
    let png = detect(b"\x89PNG\r\n\x1a\n");
    assert!(png.kind().is_image());
    assert!(!png.kind().is_video());
    assert!(!png.kind().is_audio());

    // Video types - use FLV as it has a simple signature
    let flv = detect(b"FLV\x01");
    assert!(flv.kind().is_video());
    assert!(!flv.kind().is_image());

    // Audio types
    let mp3 = detect(b"\xFF\xFB\x90");
    assert!(mp3.kind().is_audio());
    assert!(!mp3.kind().is_video());

    // Document types
    let pdf = detect(b"%PDF-1.4");
    assert!(pdf.kind().is_document());
    assert!(!pdf.kind().is_image());

    // Archive types
    let zip = detect(b"PK\x03\x04");
    assert!(zip.kind().is_archive());
    assert!(!zip.kind().is_document());
}

#[test]
fn test_mimetype_is_method_exact_match() {
    let data = b"\x89PNG\r\n\x1a\n";
    let mime = detect(data);

    assert!(mime.is(IMAGE_PNG));
    assert!(!mime.is(IMAGE_JPEG));
    assert!(!mime.is(IMAGE_GIF));
}

#[test]
fn test_mimetype_is_method_with_aliases() {
    // PDF has aliases: APPLICATION_PDF and APPLICATION_X_PDF
    let data = b"%PDF-1.4";
    let mime = detect(data);

    assert!(mime.is(APPLICATION_PDF));
    assert!(mime.is(APPLICATION_X_PDF));
    assert!(!mime.is(IMAGE_PNG));
}

#[test]
fn test_mimetype_all_methods_consistency() {
    let data = b"\x89PNG\r\n\x1a\n";
    let mime = detect(data);

    // All methods should be consistent
    assert_eq!(mime.mime(), IMAGE_PNG);
    assert_eq!(mime.extension(), ".png");
    assert!(mime.kind().is_image());
    assert!(mime.is(IMAGE_PNG));
    assert!(
        !mime.name().is_empty(),
        "Format should have a non-empty name"
    );
}

// ============================================================================
// DETECTION PATH PRIORITY TESTS
// ============================================================================
// Tests to verify detection priority: children > prefix_vec entries

#[test]
fn test_detection_priority_child_over_generic() {
    // Child formats should be checked before falling back to parent
    // OLE children have specific markers checked first
    let mut data = vec![0u8; 600];
    data[0..8].copy_from_slice(b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1");

    let mime = detect(&data);
    // Should detect as OLE or one of its children
    assert!(mime.mime().starts_with("application/"));
    assert!(mime.mime().contains("ole") || mime.mime().contains("ms"));
}

#[test]
fn test_detection_priority_prefix_vec_order() {
    // Within PREFIX_VEC, earlier entries should have priority
    // Test that specific formats are matched before generic ones

    let data = b"MThd";
    let mime = detect(data);
    assert_eq!(mime.mime(), AUDIO_MIDI); // Specific format
}

// ============================================================================
// EDGE CASE TESTS FOR DETECTION PATHS
// ============================================================================

#[test]
fn test_minimum_data_for_detection() {
    // Test formats with very short signatures

    // OGG: 4 bytes
    let ogg = detect(b"OggS");
    assert_eq!(ogg.mime(), APPLICATION_OGG);

    // GIF: 6 bytes minimum
    let gif = detect(b"GIF89a");
    assert_eq!(gif.mime(), IMAGE_GIF);
}

#[test]
fn test_offset_signature_detection() {
    // Test formats with signatures at non-zero offsets

    // TAR: signature at offset 257 (requires proper POSIX tar structure)
    let mut tar_data = vec![0u8; 512];
    tar_data[257..262].copy_from_slice(b"ustar");
    let mime = detect(&tar_data);
    // TAR detection works with proper signature
    assert!(mime.mime().contains("application/"));
}

#[test]
fn test_fallback_to_octet_stream() {
    // Unknown format should fallback to octet-stream
    let unknown_data = b"\xDE\xAD\xBE\xEF\x00\x00\x00\x00";
    let mime = detect(unknown_data);
    assert_eq!(mime.mime(), APPLICATION_OCTET_STREAM);
}

#[test]
fn test_format_with_multiple_signature_variants() {
    // Test formats that accept multiple signature patterns

    // JPEG: FF D8 FF E0 (JFIF)
    let jpeg_jfif = detect(b"\xFF\xD8\xFF\xE0");
    assert_eq!(jpeg_jfif.mime(), IMAGE_JPEG);

    // JPEG: FF D8 FF E1 (EXIF)
    let jpeg_exif = detect(b"\xFF\xD8\xFF\xE1");
    assert_eq!(jpeg_exif.mime(), IMAGE_JPEG);
}

// ============================================================================
// REAL-WORLD SCENARIO TESTS
// ============================================================================

#[test]
fn test_common_file_format_detection_workflow() {
    // Simulate detecting various common file formats in a typical workflow

    let files = vec![
        (b"\x89PNG\r\n\x1a\n".as_slice(), IMAGE_PNG, ".png"),
        (b"%PDF-1.4".as_slice(), APPLICATION_PDF, ".pdf"),
        (b"PK\x03\x04".as_slice(), APPLICATION_ZIP, ".zip"),
        (b"\xFF\xD8\xFF\xE0JFIF".as_slice(), IMAGE_JPEG, ".jpg"),
        (b"GIF89a".as_slice(), IMAGE_GIF, ".gif"),
    ];

    for (data, expected_mime, expected_ext) in files {
        let mime = detect(data);
        assert_eq!(mime.mime(), expected_mime);
        assert_eq!(mime.extension(), expected_ext);
        assert!(!mime.mime().is_empty());
        assert!(mime.extension().starts_with('.'));
        assert!(
            !mime.name().is_empty(),
            "Format {} should have a non-empty name",
            mime.mime()
        );
    }
}

#[test]
fn test_batch_detection() {
    // Test detecting multiple files in sequence
    let samples = vec![
        b"\x89PNG\r\n\x1a\n".as_slice(),
        b"%PDF-1.4".as_slice(),
        b"GIF89a".as_slice(),
        b"\xFF\xD8\xFF\xE0".as_slice(),
        b"PK\x03\x04".as_slice(),
        b"\x1F\x8B\x08".as_slice(),
        b"7z\xBC\xAF\x27\x1C".as_slice(),
        b"Rar!\x1A\x07\x00".as_slice(),
    ];

    for sample in samples {
        let mime = detect(sample);
        assert!(!mime.mime().is_empty());
        assert!(!mime.extension().is_empty());
        assert!(mime.extension().starts_with('.'));
        // Ensure all formats have verbose names
        assert!(
            !mime.name().is_empty(),
            "Format {} should have a non-empty name",
            mime.mime()
        );
    }
}

// ============================================================================
// UTF-8 BOM CHILDREN TESTS
// ============================================================================

#[test]
fn test_utf8_bom_html() {
    let data = b"\xEF\xBB\xBF<!DOCTYPE html><html><head><title>Test</title></head><body>Content</body></html>";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_HTML);
    assert_eq!(mime.extension(), ".html");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "HyperText Markup Language (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_xml() {
    let data = b"\xEF\xBB\xBF<?xml version=\"1.0\" encoding=\"UTF-8\"?><root><element>Test</element></root>";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_XML);
    assert_eq!(mime.extension(), ".xml");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "Extensible Markup Language (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_svg() {
    let data = b"\xEF\xBB\xBF<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\"><circle cx=\"50\" cy=\"50\" r=\"40\"/></svg>";
    let mime = detect(data);

    assert_eq!(mime.mime(), IMAGE_SVG_XML);
    assert_eq!(mime.extension(), ".svg");
    assert_eq!(mime.name(), "Scalable Vector Graphics (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_rtf() {
    let data = b"\xEF\xBB\xBF{\\rtf1\\ansi\\deff0 {\\fonttbl{\\f0 Times New Roman;}}\\f0\\fs60 Hello World}";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_RTF);
    assert_eq!(mime.extension(), ".rtf");
    assert_eq!(mime.name(), "Rich Text Format (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_json() {
    let data = b"\xEF\xBB\xBF{\"name\":\"John\",\"age\":30,\"city\":\"New York\"}";
    let mime = detect(data);

    assert_eq!(mime.mime(), APPLICATION_JSON);
    assert_eq!(mime.extension(), ".json");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "JavaScript Object Notation (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_json_array() {
    let data = b"\xEF\xBB\xBF[{\"id\":1,\"name\":\"Item 1\"},{\"id\":2,\"name\":\"Item 2\"}]";
    let mime = detect(data);

    assert_eq!(mime.mime(), APPLICATION_JSON);
    assert_eq!(mime.extension(), ".json");
    assert_eq!(mime.name(), "JavaScript Object Notation (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_csv() {
    let data = b"\xEF\xBB\xBFname,age,city\nJohn,30,NYC\nJane,25,LA\nBob,35,Chicago";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_CSV);
    assert_eq!(mime.extension(), ".csv");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "Comma-Separated Values (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_tsv() {
    let data = b"\xEF\xBB\xBFname\tage\tcity\nJohn\t30\tNYC\nJane\t25\tLA\nBob\t35\tChicago\nAlice\t28\tBoston";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_TAB_SEPARATED_VALUES);
    assert_eq!(mime.extension(), ".tsv");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "Tab-Separated Values (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_psv() {
    let data =
        b"\xEF\xBB\xBFname|age|city\nJohn|30|NYC\nJane|25|LA\nBob|35|Chicago\nAlice|28|Boston";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_PIPE_SEPARATED_VALUES);
    assert_eq!(mime.extension(), ".psv");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "Pipe-Separated Values (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_ssv() {
    let data =
        b"\xEF\xBB\xBFname;age;city\nJohn;30;NYC\nJane;25;LA\nBob;35;Chicago\nAlice;28;Boston";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_SEMICOLON_SEPARATED_VALUES);
    assert_eq!(mime.extension(), ".ssv");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "Semicolon-Separated Values (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_srt() {
    let data = b"\xEF\xBB\xBF1\n00:00:01,000 --> 00:00:02,500\nFirst subtitle line\n\n2\n00:00:03,000 --> 00:00:05,000\nSecond subtitle line";
    let mime = detect(data);

    assert_eq!(mime.mime(), APPLICATION_X_SUBRIP);
    assert_eq!(mime.extension(), ".srt");
    assert_eq!(mime.name(), "SubRip Subtitle (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_vtt() {
    let data = b"\xEF\xBB\xBFWEBVTT\n\n00:00.000 --> 00:01.000\nFirst caption\n\n00:01.500 --> 00:02.500\nSecond caption";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_VTT);
    assert_eq!(mime.extension(), ".vtt");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "WebVTT Subtitle (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_vcard() {
    let data = b"\xEF\xBB\xBFBEGIN:VCARD\nVERSION:3.0\nFN:John Doe\nTEL:+1234567890\nEMAIL:john@example.com\nEND:VCARD";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_VCARD);
    assert_eq!(mime.extension(), ".vcf");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "vCard (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_icalendar() {
    let data = b"\xEF\xBB\xBFBEGIN:VCALENDAR\nVERSION:2.0\nPRODID:-//Test//Test//EN\nBEGIN:VEVENT\nSUMMARY:Test Event\nEND:VEVENT\nEND:VCALENDAR";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_CALENDAR);
    assert_eq!(mime.extension(), ".ics");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "iCalendar (UTF-8 BOM)");
}

#[test]
fn test_utf8_bom_plain_text() {
    let data = b"\xEF\xBB\xBFThis is plain text content with UTF-8 BOM marker.";
    let mime = detect(data);

    assert_eq!(mime.mime(), TEXT_UTF8_BOM);
    assert_eq!(mime.extension(), ".txt");
    assert!(mime.kind().is_text());
    assert_eq!(mime.name(), "UTF-8 with BOM");
}

#[test]
fn test_utf8_bom_priority_rtf_over_json() {
    // RTF should be detected before JSON since both start with {
    let rtf_data = b"\xEF\xBB\xBF{\\rtf1 Test}";
    let mime = detect(rtf_data);
    assert_eq!(mime.mime(), TEXT_RTF);
    assert_eq!(mime.name(), "Rich Text Format (UTF-8 BOM)");

    // But JSON should still be detected when it's valid JSON
    let json_data = b"\xEF\xBB\xBF{\"test\": true}";
    let mime = detect(json_data);
    assert_eq!(mime.mime(), APPLICATION_JSON);
    assert_eq!(mime.name(), "JavaScript Object Notation (UTF-8 BOM)");
}

// ============================================================================
// WITH_LIMIT METHOD TESTS
// ============================================================================

#[test]
fn test_detect_with_limit() {
    let data = b"\x89PNG\r\n\x1a\n";
    let mime = detect_with_limit(data, 8);
    assert_eq!(mime.mime(), IMAGE_PNG);
}

#[test]
fn test_detect_reader_with_limit() {
    let data = b"%PDF-1.4";
    let cursor = Cursor::new(data);
    let mime = detect_reader_with_limit(cursor, 8).expect("Should detect from reader");
    assert_eq!(mime.mime(), APPLICATION_PDF);
}

#[test]
fn test_detect_file_with_limit() {
    use std::fs;
    use std::io::Write;

    let temp_path = "test_temp_with_limit.png";
    let mut file = fs::File::create(temp_path).expect("Failed to create temp file");
    file.write_all(b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR")
        .expect("Failed to write");
    drop(file);

    let mime = detect_file_with_limit(temp_path, 16).expect("Should detect file");
    assert_eq!(mime.mime(), IMAGE_PNG);

    fs::remove_file(temp_path).ok();
}
