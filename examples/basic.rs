use mimetype_detector::{detect, equals_any};

fn main() -> std::io::Result<()> {
    // Detect from byte slice
    let data = b"\x89PNG\r\n\x1a\n";
    let mime_type = detect(data);
    println!("Detected MIME type: {}", mime_type);
    println!("Extension: {}", mime_type.extension());

    // Check if MIME type is one of several
    let is_image = equals_any("image/png", &["image/png", "image/jpeg", "image/gif"]);
    println!("Is image: {}", is_image);

    // Check specific MIME type
    if mime_type.is("image/png") {
        println!("This is a PNG image!");
    }

    Ok(())
}
