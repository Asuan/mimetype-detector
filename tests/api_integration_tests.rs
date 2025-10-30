//! Comprehensive API Integration Tests
//!
//! This module tests all public API functions of the mimetype-detector library,
//! including edge cases, error handling, and various usage patterns.

use mimetype_detector::{
    constants::*, detect, detect_file, detect_reader, equals_any, register_extension, register_mime,
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
}

#[test]
fn test_detect_empty_data() {
    let data = b"";
    let mime = detect(data);
    assert_eq!(mime.mime(), APPLICATION_OCTET_STREAM);
}

#[test]
fn test_detect_short_data() {
    let data = b"AB";
    let mime = detect(data);
    assert!(!mime.mime().is_empty());
}

#[test]
fn test_detect_reader_from_cursor() {
    let data = b"%PDF-1.4\ntest content";
    let cursor = Cursor::new(data);
    let mime = detect_reader(cursor).expect("Should detect from reader");
    assert_eq!(mime.mime(), APPLICATION_PDF);
    assert_eq!(mime.extension(), ".pdf");
}

#[test]
fn test_detect_reader_from_slice() {
    let data = b"\xff\xd8\xff\xe0JFIF";
    let mime = detect_reader(&data[..]).expect("Should detect from slice");
    assert_eq!(mime.mime(), IMAGE_JPEG);
}

#[test]
fn test_detect_reader_empty() {
    let data: &[u8] = b"";
    let mime = detect_reader(data).expect("Should handle empty reader");
    assert_eq!(mime.mime(), APPLICATION_OCTET_STREAM);
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
    assert_eq!(mime.mime(), APPLICATION_OCTET_STREAM);

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
}

#[test]
fn test_detect_all_null_bytes() {
    let data = vec![0u8; 100];
    let mime = detect(&data);
    assert_eq!(mime.mime(), APPLICATION_OCTET_STREAM);
}

#[test]
fn test_detect_all_printable_ascii() {
    let data = b"This is a plain text file with only printable ASCII characters.";
    let mime = detect(data);
    assert_eq!(mime.mime(), TEXT_UTF8);
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
