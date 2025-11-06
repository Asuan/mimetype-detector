/// Generates a prefix vector for O(1) lookup by first byte
/// Uses provided bucket name mapping to generate statics with custom names
macro_rules! build_prefix_vec {
    (
        $(#[$meta:meta])*
        $vis:vis static $name:ident: [
            $( $byte:literal => [ $($types:expr),* $(,)? ] as $bucket_name:ident ),* $(,)?
        ]
    ) => {
        // Generate static slices for each bucket
        $(
            static $bucket_name: &[&MimeType] = &[$($types),*];
        )*

        // Generate the array with documentation
        $(#[$meta])*
        $vis static $name: [&[&MimeType]; 256] = {
            const EMPTY: &[&MimeType] = &[];
            let mut arr = [EMPTY; 256];
            $(
                arr[$byte] = $bucket_name;
            )*
            arr
        };
    };
}

/// Unified macro for MimeType generation
///
/// This macro provides a single, flexible interface for defining MIME types
/// with optional parameters for kind, aliases, extension aliases, children, and parent.
///
/// # Basic Usage
/// ```rust,ignore
/// mimetype!(PNG, IMAGE_PNG, ".png", b"\x89PNG\r\n\x1a\n");
/// ```
///
/// # With Optional Parameters
/// ```rust,ignore
/// mimetype!(ZIP, APPLICATION_ZIP, ".zip", b"PK\x03\x04",
///     kind: ARCHIVE,
///     aliases: [APPLICATION_X_ZIP_COMPRESSED],
///     ext_aliases: [".zipx"],
///     children: [&DOCX, &XLSX]
/// );
/// ```
///
/// # Pattern Types Supported
/// - Simple prefix: `b"PNG"`
/// - Byte array: `[0x89, 0x50, 0x4E, 0x47]`
/// - Multiple patterns: `b"GIF87a" | b"GIF89a"`
/// - Offset check: `offset(257, b"ustar")`
/// - Offset with prefix: `offset(8, b"WEBP", prefix: b"RIFF")`
macro_rules! mimetype {
    // Build function that actually creates the MimeType with all parameters
    (@build $static_name:ident, $mime:expr, $ext:expr, $matcher:expr, $children:expr,
     $kind:expr, $aliases:expr, $ext_aliases:expr, $parent:expr) => {
        static $static_name: $crate::MimeType = {
            let mut mime = $crate::MimeType::new($mime, $ext, $matcher, $children);
            if let Some(k) = $kind {
                mime = mime.with_kind(k);
            }
            if let Some(a) = $aliases {
                mime = mime.with_aliases(a);
            }
            if let Some(ea) = $ext_aliases {
                mime = mime.with_extension_aliases(ea);
            }
            if let Some(p) = $parent {
                mime = mime.with_parent(p);
            }
            mime
        };
    };

    // Simple literal prefix
    ($static_name:ident, $mime:expr, $ext:expr, $prefix:literal) => {
        mimetype!(@build $static_name, $mime, $ext,
            |input| input.starts_with($prefix),
            &[],
            None, None, None, None
        );
    };

    // Single literal prefix (unified pattern with optional parameters)
    // Note: Parameters must be in this order: kind, aliases, ext_aliases, children, parent
    ($static_name:ident, $mime:expr, $ext:expr, $prefix:literal,
     kind: $kind:ident
     $(, aliases: [$($alias:expr),* $(,)?])?
     $(, ext_aliases: [$($ext_alias:expr),* $(,)?])?
     $(, children: [$($child:expr),* $(,)?])?
     $(, parent: $parent:expr)?
    ) => {
        mimetype!(@build $static_name, $mime, $ext,
            |input| input.starts_with($prefix),
            &[$($($child),*)?],
            Some($crate::MimeKind::$kind),
            mimetype!(@opt_slice $($($alias),*)?),
            mimetype!(@opt_slice $($($ext_alias),*)?),
            mimetype!(@opt_expr $($parent)?)
        );
    };

    // Array pattern
    ($static_name:ident, $mime:expr, $ext:expr, [$($byte:expr),+ $(,)?], kind: $kind:ident) => {
        mimetype!(@build $static_name, $mime, $ext,
            |input| {
                const PREFIX: &[u8] = &[$($byte),+];
                input.starts_with(PREFIX)
            },
            &[],
            Some($crate::MimeKind::$kind), None, None, None
        );
    };

    // Multiple byte array alternatives
    ($static_name:ident, $mime:expr, $ext:expr, [$($first_byte:expr),+ $(,)?] $(| [$($rest_byte:expr),+ $(,)?])+, kind: $kind:ident) => {
        mimetype!(@build $static_name, $mime, $ext,
            |input| {
                const FIRST: &[u8] = &[$($first_byte),+];
                input.starts_with(FIRST) $(|| input.starts_with(&[$($rest_byte),+]))+
            },
            &[],
            Some($crate::MimeKind::$kind), None, None, None
        );
    };

    // Multiple byte array alternatives with extension aliases
    ($static_name:ident, $mime:expr, $ext:expr, [$($first_byte:expr),+ $(,)?] $(| [$($rest_byte:expr),+ $(,)?])+, kind: $kind:ident, ext_aliases: [$($ext_alias:literal),* $(,)?]) => {
        mimetype!(@build $static_name, $mime, $ext,
            |input| {
                const FIRST: &[u8] = &[$($first_byte),+];
                input.starts_with(FIRST) $(|| input.starts_with(&[$($rest_byte),+]))+
            },
            &[],
            Some($crate::MimeKind::$kind), None, Some(&[$($ext_alias),*]), None
        );
    };

    // Multiple literal prefixes (unified pattern with optional parameters)
    // Note: Parameters must be in this order: kind, aliases, ext_aliases, children, parent
    ($static_name:ident, $mime:expr, $ext:expr, $first:literal $(| $rest:literal)+,
     kind: $kind:ident
     $(, aliases: [$($alias:expr),* $(,)?])?
     $(, ext_aliases: [$($ext_alias:expr),* $(,)?])?
     $(, children: [$($child:expr),* $(,)?])?
     $(, parent: $parent:expr)?
    ) => {
        mimetype!(@build $static_name, $mime, $ext,
            |input| input.starts_with($first) $(|| input.starts_with($rest))+,
            &[$($($child),*)?],
            Some($crate::MimeKind::$kind),
            mimetype!(@opt_slice $($($alias),*)?),
            mimetype!(@opt_slice $($($ext_alias),*)?),
            mimetype!(@opt_expr $($parent)?)
        );
    };

    // Helper for optional slice parameters
    (@opt_slice $($items:expr),+) => { Some(&[$($items),+]) };
    (@opt_slice) => { None };

    // Helper for optional expression parameters
    (@opt_expr $item:expr) => { Some($item) };
    (@opt_expr) => { None };

    // Simple offset patterns (unified with optional parameters)
    // Note: Parameters must be in this order: kind, aliases, ext_aliases, children, parent
    ($static_name:ident, $mime:expr, $ext:expr, offset: ($offset:expr, $bytes:expr),
     kind: $kind:ident
     $(, aliases: [$($alias:expr),* $(,)?])?
     $(, ext_aliases: [$($ext_alias:expr),* $(,)?])?
     $(, children: [$($child:expr),* $(,)?])?
     $(, parent: $parent:expr)?
    ) => {
        mimetype!(@build $static_name, $mime, $ext,
            |input| {
                let offset = $offset;
                let bytes: &[u8] = $bytes;
                input.len() >= offset + bytes.len() && &input[offset..offset + bytes.len()] == bytes
            },
            &[$($($child),*)?],
            Some($crate::MimeKind::$kind),
            mimetype!(@opt_slice $($($alias),*)?),
            mimetype!(@opt_slice $($($ext_alias),*)?),
            mimetype!(@opt_expr $($parent)?)
        );
    };

    // Offset with prefix patterns (unified with optional parameters)
    // Note: Parameters must be in this order: kind, aliases, ext_aliases, children, parent
    ($static_name:ident, $mime:expr, $ext:expr, offset: ($offset:expr, $bytes:expr, prefix: ($prefix_offset:expr, $prefix_bytes:expr)),
     kind: $kind:ident
     $(, aliases: [$($alias:expr),* $(,)?])?
     $(, ext_aliases: [$($ext_alias:expr),* $(,)?])?
     $(, children: [$($child:expr),* $(,)?])?
     $(, parent: $parent:expr)?
    ) => {
        mimetype!(@build $static_name, $mime, $ext,
            |input| {
                let prefix_offset = $prefix_offset;
                let prefix_bytes: &[u8] = $prefix_bytes;
                let offset = $offset;
                let bytes: &[u8] = $bytes;
                input.len() >= prefix_offset + prefix_bytes.len()
                    && &input[prefix_offset..prefix_offset + prefix_bytes.len()] == prefix_bytes
                    && input.len() >= offset + bytes.len()
                    && &input[offset..offset + bytes.len()] == bytes
            },
            &[$($($child),*)?],
            Some($crate::MimeKind::$kind),
            mimetype!(@opt_slice $($($alias),*)?),
            mimetype!(@opt_slice $($($ext_alias),*)?),
            mimetype!(@opt_expr $($parent)?)
        );
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prefix_matching() {
        let test_pdf = |input: &[u8]| input.starts_with(b"%PDF-");
        assert!(test_pdf(b"%PDF-1.4"));
        assert!(!test_pdf(b"not pdf"));
        assert!(!test_pdf(b""));
    }

    #[test]
    fn test_array_prefix_matching() {
        let test_png =
            |input: &[u8]| input.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
        assert!(test_png(b"\x89PNG\r\n\x1a\n"));
        assert!(!test_png(b"not png"));
    }

    #[test]
    fn test_multi_prefix_matching() {
        let test_zip = |input: &[u8]| {
            input.starts_with(b"PK\x03\x04")
                || input.starts_with(b"PK\x05\x06")
                || input.starts_with(b"PK\x07\x08")
        };
        assert!(test_zip(b"PK\x03\x04"));
        assert!(test_zip(b"PK\x05\x06"));
        assert!(test_zip(b"PK\x07\x08"));
        assert!(!test_zip(b"PK\x01\x02"));
    }

    #[test]
    fn test_offset_matching() {
        let test_tar = |input: &[u8]| input.len() >= 262 && &input[257..262] == b"ustar";
        let mut data = vec![0u8; 300];
        data[257..262].copy_from_slice(b"ustar");
        assert!(test_tar(&data));
        assert!(!test_tar(b"too short"));
    }

    #[test]
    fn test_offset_prefix_matching() {
        let test_webp = |input: &[u8]| {
            input.len() >= 12 && input.starts_with(b"RIFF") && &input[8..12] == b"WEBP"
        };
        let mut data = vec![0u8; 20];
        data[0..4].copy_from_slice(b"RIFF");
        data[8..12].copy_from_slice(b"WEBP");
        assert!(test_webp(&data));

        let mut bad_data = vec![0u8; 20];
        bad_data[8..12].copy_from_slice(b"WEBP");
        assert!(!test_webp(&bad_data));
    }

    use crate::constants::*;
    use crate::MimeKind;

    mimetype!(TEST_FLV, VIDEO_X_FLV, ".flv", b"FLV", kind: VIDEO);

    #[test]
    fn test_mimetype_simple_with_kind() {
        let data = b"FLV\x01 test data";
        assert!((TEST_FLV.matcher)(data));
        assert_eq!(TEST_FLV.mime(), VIDEO_X_FLV);
        assert_eq!(TEST_FLV.extension(), ".flv");
        assert!(TEST_FLV.kind().contains(MimeKind::VIDEO));
    }

    mimetype!(TEST_CUSTOM, "application/x-test", ".test", b"TEST");

    #[test]
    fn test_mimetype_simple_no_kind() {
        let data = b"TEST data";
        assert!((TEST_CUSTOM.matcher)(data));
        assert_eq!(TEST_CUSTOM.mime(), "application/x-test");
        assert_eq!(TEST_CUSTOM.extension(), ".test");
    }

    mimetype!(TEST_MULTI, IMAGE_GIF, ".gif", b"GIF87a" | b"GIF89a", kind: IMAGE);

    #[test]
    fn test_mimetype_multiple_prefixes() {
        assert!((TEST_MULTI.matcher)(b"GIF87a"));
        assert!((TEST_MULTI.matcher)(b"GIF89a"));
        assert!(!(TEST_MULTI.matcher)(b"GIF90"));
        assert_eq!(TEST_MULTI.mime(), IMAGE_GIF);
        assert!(TEST_MULTI.kind().contains(MimeKind::IMAGE));
    }

    mimetype!(TEST_TAR_FMT, APPLICATION_X_TAR, ".tar", offset: (257, b"ustar"), kind: ARCHIVE);

    #[test]
    fn test_mimetype_offset() {
        let mut data = vec![0u8; 300];
        data[257..262].copy_from_slice(b"ustar");
        assert!((TEST_TAR_FMT.matcher)(&data));
        assert_eq!(TEST_TAR_FMT.mime(), APPLICATION_X_TAR);
        assert!(TEST_TAR_FMT.kind().contains(MimeKind::ARCHIVE));
    }

    mimetype!(TEST_WAV_FMT, AUDIO_WAV, ".wav", offset: (8, b"WAVE", prefix: (0, b"RIFF")), kind: AUDIO);

    #[test]
    fn test_mimetype_offset_prefix() {
        let mut data = vec![0u8; 20];
        data[0..4].copy_from_slice(b"RIFF");
        data[8..12].copy_from_slice(b"WAVE");
        assert!((TEST_WAV_FMT.matcher)(&data));
        assert_eq!(TEST_WAV_FMT.mime(), AUDIO_WAV);
        assert!(TEST_WAV_FMT.kind().contains(MimeKind::AUDIO));
    }

    static TEST_PDF_SEP: crate::MimeType = crate::MimeType::new(
        APPLICATION_PDF,
        ".pdf",
        |input| input.starts_with(b"%PDF-"),
        &[],
    )
    .with_kind(crate::MimeKind::DOCUMENT);

    #[test]
    fn test_manual_mimetype_creation() {
        let test_separate_pdf = |input: &[u8]| input.starts_with(b"%PDF-");
        assert!(test_separate_pdf(b"%PDF-1.4"));
        assert_eq!(TEST_PDF_SEP.mime(), APPLICATION_PDF);
        assert_eq!(TEST_PDF_SEP.extension(), ".pdf");
        assert!(TEST_PDF_SEP.kind().contains(MimeKind::DOCUMENT));
    }

    mimetype!(TEST_PNG_ARR, IMAGE_PNG, ".png", [0x89, 0x50, 0x4E, 0x47], kind: IMAGE);

    #[test]
    fn test_mimetype_array() {
        let data = b"\x89PNG\r\n\x1a\n";
        assert!((TEST_PNG_ARR.matcher)(data));
        assert_eq!(TEST_PNG_ARR.mime(), IMAGE_PNG);
        assert_eq!(TEST_PNG_ARR.extension(), ".png");
        assert!(TEST_PNG_ARR.kind().contains(MimeKind::IMAGE));
    }

    // Test aliases parameter
    mimetype!(
        TEST_JPG_ALIASES,
        IMAGE_JPEG,
        ".jpg",
        b"\xff\xd8\xff",
        kind: IMAGE,
        aliases: ["image/jpg", "image/pjpeg"]
    );

    #[test]
    fn test_mimetype_aliases_simple() {
        let data = b"\xff\xd8\xff\xe0 JFIF";
        assert!((TEST_JPG_ALIASES.matcher)(data));
        assert_eq!(TEST_JPG_ALIASES.mime(), IMAGE_JPEG);
        assert_eq!(TEST_JPG_ALIASES.extension(), ".jpg");
        assert!(TEST_JPG_ALIASES.kind().contains(MimeKind::IMAGE));
        assert!(TEST_JPG_ALIASES.is("image/jpg"));
        assert!(TEST_JPG_ALIASES.is("image/pjpeg"));
        assert!(TEST_JPG_ALIASES.is(IMAGE_JPEG));
    }

    mimetype!(
        TEST_GZIP_ALIASES,
        APPLICATION_GZIP,
        ".gz",
        b"\x1f\x8b",
        kind: ARCHIVE,
        aliases: ["application/x-gzip", "application/gzip-compressed"]
    );

    #[test]
    fn test_mimetype_aliases_multiple() {
        let data = b"\x1f\x8b\x08\x00\x00";
        assert!((TEST_GZIP_ALIASES.matcher)(data));
        assert_eq!(TEST_GZIP_ALIASES.mime(), APPLICATION_GZIP);
        assert!(TEST_GZIP_ALIASES.is("application/x-gzip"));
        assert!(TEST_GZIP_ALIASES.is("application/gzip-compressed"));
    }

    mimetype!(
        TEST_ZIP_MULTI_ALIASES,
        APPLICATION_ZIP,
        ".zip",
        b"PK\x03\x04" | b"PK\x05\x06",
        kind: ARCHIVE,
        aliases: ["application/x-zip-compressed"]
    );

    #[test]
    fn test_mimetype_aliases_multi_prefix() {
        assert!((TEST_ZIP_MULTI_ALIASES.matcher)(b"PK\x03\x04"));
        assert!((TEST_ZIP_MULTI_ALIASES.matcher)(b"PK\x05\x06"));
        assert!(TEST_ZIP_MULTI_ALIASES.is("application/x-zip-compressed"));
    }

    // Test extension aliases parameter
    mimetype!(
        TEST_TIFF_EXT,
        IMAGE_TIFF,
        ".tiff",
        b"II*\x00",
        kind: IMAGE,
        ext_aliases: [".tif"]
    );

    #[test]
    fn test_mimetype_ext_aliases() {
        let data = b"II*\x00\x08\x00\x00\x00";
        assert!((TEST_TIFF_EXT.matcher)(data));
        assert_eq!(TEST_TIFF_EXT.mime(), IMAGE_TIFF);
        assert_eq!(TEST_TIFF_EXT.extension(), ".tiff");
        assert!(TEST_TIFF_EXT.kind().contains(MimeKind::IMAGE));
    }

    mimetype!(
        TEST_JPEG_EXT,
        IMAGE_JPEG,
        ".jpg",
        b"\xff\xd8\xff",
        kind: IMAGE,
        ext_aliases: [".jpeg", ".jpe", ".jif"]
    );

    #[test]
    fn test_mimetype_ext_aliases_multiple() {
        assert!((TEST_JPEG_EXT.matcher)(b"\xff\xd8\xff\xe0"));
        assert_eq!(TEST_JPEG_EXT.mime(), IMAGE_JPEG);
    }

    // Test children parameter
    // Create a test child type
    static TEST_CHILD: crate::MimeType =
        crate::MimeType::new("application/x-test-child", ".child", |_| false, &[]);

    mimetype!(
        TEST_PNG_CHILDREN,
        IMAGE_PNG,
        ".png",
        b"\x89PNG\r\n\x1a\n",
        kind: IMAGE,
        children: [&TEST_CHILD]
    );

    #[test]
    fn test_mimetype_children() {
        let data = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0d";
        assert!((TEST_PNG_CHILDREN.matcher)(data));
        assert_eq!(TEST_PNG_CHILDREN.mime(), IMAGE_PNG);
        assert!(TEST_PNG_CHILDREN.kind().contains(MimeKind::IMAGE));
    }

    // Test unified mimetype! macro with parent parameter
    // Create a test parent type
    static TEST_PARENT: crate::MimeType = crate::MimeType::new("text/plain", ".txt", |_| true, &[]);

    mimetype!(
        TEST_WARC_PARENT,
        APPLICATION_WARC,
        ".warc",
        b"WARC/1.0",
        kind: ARCHIVE,
        parent: &TEST_PARENT
    );

    #[test]
    fn test_mimetype_with_parent() {
        let data = b"WARC/1.0\r\n";
        assert!((TEST_WARC_PARENT.matcher)(data));
        assert_eq!(TEST_WARC_PARENT.mime(), APPLICATION_WARC);
        assert!(TEST_WARC_PARENT.parent().is_some());
        assert!(TEST_WARC_PARENT.kind().contains(MimeKind::ARCHIVE));
    }

    mimetype!(
        TEST_RTF_PARENT,
        APPLICATION_RTF,
        ".rtf",
        b"{\\rtf1" | b"{\\rtf0",
        kind: DOCUMENT,
        parent: &TEST_PARENT
    );

    #[test]
    fn test_mimetype_parent_multi_prefix() {
        assert!((TEST_RTF_PARENT.matcher)(b"{\\rtf1\\ansi"));
        assert!((TEST_RTF_PARENT.matcher)(b"{\\rtf0\\ansi"));
        assert!(TEST_RTF_PARENT.parent().is_some());
    }
}
