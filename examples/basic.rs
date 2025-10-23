use mimetype_detector::{constants::*, detect, equals_any, MimeKind};

fn main() -> std::io::Result<()> {
    println!("=== Basic MIME Detection ===\n");

    // Detect from byte slice
    let data = b"\x89PNG\r\n\x1a\n";
    let mime_type = detect(data);
    println!("PNG Detection:");
    println!("  MIME type: {}", mime_type.mime());
    println!("  Extension: {}", mime_type.extension());
    println!("  Kind: {}", mime_type.kind());
    println!("  Is image: {}", mime_type.kind().is_image());

    // Check specific MIME type
    if mime_type.is(IMAGE_PNG) {
        println!("  ✓ Confirmed: This is a PNG image!");
    }

    // Check if MIME type is one of several
    let is_image = equals_any(IMAGE_PNG, &[IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF]);
    println!("  Is in image list: {is_image}");

    println!("\n=== MimeKind Examples ===\n");

    // DOCUMENT
    let pdf_data = b"%PDF-1.4";
    let pdf = detect(pdf_data);
    println!(
        "PDF: {} - Kind: {} (is_document: {})",
        pdf.mime(),
        pdf.kind(),
        pdf.kind().is_document()
    );

    // ARCHIVE
    let zip_data = b"PK\x03\x04";
    let zip = detect(zip_data);
    println!(
        "ZIP: {} - Kind: {} (is_archive: {})",
        zip.mime(),
        zip.kind(),
        zip.kind().is_archive()
    );

    // TEXT
    let html_data = b"<!DOCTYPE html>";
    let html = detect(html_data);
    println!(
        "HTML: {} - Kind: {} (is_text: {})",
        html.mime(),
        html.kind(),
        html.kind().is_text()
    );

    // AUDIO
    let mp3_data = b"\xff\xfb\x90\x00";
    let mp3 = detect(mp3_data);
    println!(
        "MP3: {} - Kind: {} (is_audio: {})",
        mp3.mime(),
        mp3.kind(),
        mp3.kind().is_audio()
    );

    // VIDEO
    let mut mp4_data = vec![0u8; 12];
    mp4_data[4..8].copy_from_slice(b"ftyp");
    mp4_data[8..12].copy_from_slice(b"isom");
    let mp4 = detect(&mp4_data);
    println!(
        "MP4: {} - Kind: {} (is_video: {})",
        mp4.mime(),
        mp4.kind(),
        mp4.kind().is_video()
    );

    // FONT
    let ttf_data = b"\x00\x01\x00\x00";
    let ttf = detect(ttf_data);
    println!(
        "TTF: {} - Kind: {} (is_font: {})",
        ttf.mime(),
        ttf.kind(),
        ttf.kind().is_font()
    );

    // EXECUTABLE
    let wasm_data = b"\x00asm";
    let wasm = detect(wasm_data);
    println!(
        "WASM: {} - Kind: {} (is_executable: {})",
        wasm.mime(),
        wasm.kind(),
        wasm.kind().is_executable()
    );

    // DATABASE
    let sqlite_data = b"SQLite format 3\x00";
    let sqlite = detect(sqlite_data);
    println!(
        "SQLite: {} - Kind: {} (is_database: {})",
        sqlite.mime(),
        sqlite.kind(),
        sqlite.kind().is_database()
    );

    println!("\n=== Kind Inheritance ===\n");

    // DOCX inherits ARCHIVE from ZIP parent
    let mut docx_data = vec![0u8; 100];
    docx_data[0..4].copy_from_slice(b"PK\x03\x04");
    // Add word/ marker to indicate DOCX
    for (i, &byte) in b"word/".iter().enumerate() {
        docx_data[30 + i] = byte;
    }
    let docx = detect(&docx_data);
    println!("DOCX Detection:");
    println!("  MIME: {}", docx.mime());
    println!("  Kind: {}", docx.kind());
    if docx.kind().contains(MimeKind::ARCHIVE) {
        println!("  ✓ Contains ARCHIVE (inherited from ZIP)");
    }
    if docx.kind().contains(MimeKind::DOCUMENT) {
        println!("  ✓ Contains DOCUMENT");
    }

    println!("\n=== Kind Comparison ===\n");
    println!("PNG kind == IMAGE: {}", mime_type.kind() == MimeKind::IMAGE);
    println!("ZIP kind == ARCHIVE: {}", zip.kind() == MimeKind::ARCHIVE);
    println!(
        "HTML contains TEXT: {}",
        html.kind().contains(MimeKind::TEXT)
    );

    // Demonstrate generic path support (requires a real file)
    // Works with various path types:
    // let _mime_from_str = detect_file("test.txt")?;                    // &str
    // let _mime_from_string = detect_file(String::from("test.txt"))?;  // String
    // let _mime_from_pathbuf = detect_file(PathBuf::from("test.txt"))?; // PathBuf

    Ok(())
}
