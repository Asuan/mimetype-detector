use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mimetype_detector::detect;

fn benchmark_detection(c: &mut Criterion) {
    // PNG
    let png_data = b"\x89PNG\r\n\x1a\n";
    c.bench_function("detect PNG", |b| {
        b.iter(|| detect(black_box(png_data)))
    });

    // JPEG
    let jpeg_data = b"\xff\xd8\xff";
    c.bench_function("detect JPEG", |b| {
        b.iter(|| detect(black_box(jpeg_data)))
    });

    // PDF
    let pdf_data = b"%PDF-1.4";
    c.bench_function("detect PDF", |b| {
        b.iter(|| detect(black_box(pdf_data)))
    });

    // ZIP
    let zip_data = b"PK\x03\x04";
    c.bench_function("detect ZIP", |b| {
        b.iter(|| detect(black_box(zip_data)))
    });

    // HTML (UTF-8)
    let html_data = b"<!DOCTYPE html>";
    c.bench_function("detect HTML", |b| {
        b.iter(|| detect(black_box(html_data)))
    });

    // XML
    let xml_data = b"<?xml version=\"1.0\"?>";
    c.bench_function("detect XML", |b| {
        b.iter(|| detect(black_box(xml_data)))
    });

    // JSON
    let json_data = br#"{"key": "value"}"#;
    c.bench_function("detect JSON", |b| {
        b.iter(|| detect(black_box(json_data)))
    });

    // MP3
    let mp3_data = b"\xff\xfb\x90\x00";
    c.bench_function("detect MP3", |b| {
        b.iter(|| detect(black_box(mp3_data)))
    });

    // MP4
    let mut mp4_data = vec![0u8; 12];
    mp4_data[4..8].copy_from_slice(b"ftyp");
    c.bench_function("detect MP4", |b| {
        b.iter(|| detect(black_box(&mp4_data)))
    });

    // GIF
    let gif_data = b"GIF89a";
    c.bench_function("detect GIF", |b| {
        b.iter(|| detect(black_box(gif_data)))
    });

    // Plain text (UTF-8) - slowest as it's the fallback
    let text_data = b"Hello, World!";
    c.bench_function("detect plain text", |b| {
        b.iter(|| detect(black_box(text_data)))
    });
}

criterion_group!(benches, benchmark_detection);
criterion_main!(benches);
