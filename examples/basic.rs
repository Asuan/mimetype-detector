use mimetype_detector::{constants::*, detect, equals_any};

fn main() -> std::io::Result<()> {
    // Detect from byte slice
    let data = b"\x89PNG\r\n\x1a\n";
    let mime_type = detect(data);
    println!("Detected MIME type: {mime_type}");
    println!("Extension: {}", mime_type.extension());

    // Check if MIME type is one of several
    let is_image = equals_any(IMAGE_PNG, &[IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF]);
    println!("Is image: {is_image}");

    // Check specific MIME type
    if mime_type.is(IMAGE_PNG) {
        println!("This is a PNG image!");
    }

    // Demonstrate generic path support (requires a real file)
    // Works with various path types:
    // let _mime_from_str = detect_file("test.txt")?;           // &str
    // let _mime_from_string = detect_file(String::from("test.txt"))?; // String
    // let _mime_from_pathbuf = detect_file(PathBuf::from("test.txt"))?; // PathBuf

    Ok(())
}
