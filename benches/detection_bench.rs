use criterion::{criterion_group, criterion_main, Criterion};
use mimetype_detector::detect;
use std::hint::black_box;

/// Build a minimal stored ZIP (local file header + name + payload per entry).
/// Names deliberately match no known format so detection falls through to the
/// generic ZIP walker.
fn stored_zip(entries: usize) -> Vec<u8> {
    let mut out = Vec::new();
    for i in 0..entries {
        let name = format!("zz/{i:02}.txt");
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes()); // version needed
        out.extend_from_slice(&0u16.to_le_bytes()); // flags
        out.extend_from_slice(&0u16.to_le_bytes()); // stored (no compression)
        out.extend_from_slice(&0u16.to_le_bytes()); // mod time
        out.extend_from_slice(&0u16.to_le_bytes()); // mod date
        out.extend_from_slice(&0u32.to_le_bytes()); // crc32
        out.extend_from_slice(&16u32.to_le_bytes()); // compressed size
        out.extend_from_slice(&16u32.to_le_bytes()); // uncompressed size
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // extra length
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(b"0123456789abcdef");
    }
    out
}

fn benchmark_detection(c: &mut Criterion) {
    // PNG
    let png_data = b"\x89PNG\r\n\x1a\n";
    c.bench_function("detect PNG", |b| b.iter(|| detect(black_box(png_data))));

    // JPEG
    let jpeg_data = b"\xff\xd8\xff";
    c.bench_function("detect JPEG", |b| b.iter(|| detect(black_box(jpeg_data))));

    // PDF
    let pdf_data = b"%PDF-1.4";
    c.bench_function("detect PDF", |b| b.iter(|| detect(black_box(pdf_data))));

    // ZIP
    let zip_data = b"PK\x03\x04";
    c.bench_function("detect ZIP", |b| b.iter(|| detect(black_box(zip_data))));

    // HTML (UTF-8)
    let html_data = b"<!DOCTYPE html>";
    c.bench_function("detect HTML", |b| b.iter(|| detect(black_box(html_data))));

    // XML
    let xml_data = b"<?xml version=\"1.0\"?>";
    c.bench_function("detect XML", |b| b.iter(|| detect(black_box(xml_data))));

    // JSON
    let json_data = br#"{"key": "value"}"#;
    c.bench_function("detect JSON", |b| b.iter(|| detect(black_box(json_data))));

    // MP3
    let mp3_data = b"\xff\xfb\x90\x00";
    c.bench_function("detect MP3", |b| b.iter(|| detect(black_box(mp3_data))));

    // MP4
    let mut mp4_data = vec![0u8; 12];
    mp4_data[4..8].copy_from_slice(b"ftyp");
    c.bench_function("detect MP4", |b| b.iter(|| detect(black_box(&mp4_data))));

    // GIF
    let gif_data = b"GIF89a";
    c.bench_function("detect GIF", |b| b.iter(|| detect(black_box(gif_data))));

    // Plain text (UTF-8) - slowest as it's the fallback
    let text_data = b"Hello, World!";
    c.bench_function("detect plain text", |b| {
        b.iter(|| detect(black_box(text_data)))
    });

    // ------------------------------------------------------------------
    // Larger, realistic inputs. The stubs above exercise the magic-number
    // fast path; these exercise the literal-search and container walkers.
    // ------------------------------------------------------------------

    // 3 KB PDF: the AI child scans the first 2 KB twice via contains_bytes.
    let mut pdf_data_3k = Vec::from(&b"%PDF-1.4\n"[..]);
    pdf_data_3k.resize(3072, b'x');
    c.bench_function("detect PDF 3KB", |b| {
        b.iter(|| detect(black_box(pdf_data_3k.as_slice())))
    });

    // Generic ZIP with 40 entries; exercises one full ZipIterator walk.
    let zip_data_40 = stored_zip(40);
    c.bench_function("detect ZIP 40 entries", |b| {
        b.iter(|| detect(black_box(zip_data_40.as_slice())))
    });

    // 3 KB of prose; exercises the text fallback chain.
    let prose_3k = b"The quick brown fox jumps over the lazy dog. This is ordinary prose text \
        used to exercise the UTF-8 fallback detection path. It contains no markup, no \
        programming language keywords, and no binary control bytes. "
        .repeat(20);
    c.bench_function("detect plain text 3KB", |b| {
        b.iter(|| detect(black_box(prose_3k.as_slice())))
    });

    // Worst-case: full-buffer literal searches that find no match.

    // TIFF, no maker-note: drives the RAW child chain. Varied fill (a constant
    // fill collides with the PEF `AOC` needle at every byte).
    let mut tiff_no_makernote: Vec<u8> = (0..3072u32)
        .map(|i| (i.wrapping_mul(31) ^ 0x5B) as u8)
        .collect();
    tiff_no_makernote[0..4].copy_from_slice(b"II*\x00");
    c.bench_function("detect TIFF no-makernote 3KB", |b| {
        b.iter(|| detect(black_box(tiff_no_makernote.as_slice())))
    });

    // ASF, no stream GUID: drives the wma/wmv 16-byte GUID search.
    let mut asf_no_guid = vec![b'A'; 3072];
    asf_no_guid[0..16]
        .copy_from_slice(b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C");
    asf_no_guid[16..24].copy_from_slice(&3072u64.to_le_bytes()); // header size
    c.bench_function("detect ASF no-GUID 3KB", |b| {
        b.iter(|| detect(black_box(asf_no_guid.as_slice())))
    });

    // XML with no family tag: drives the xhtml/owl2/nuspec full-buffer fallbacks.
    let mut xml_no_tag = Vec::from(&b"<?xml version=\"1.0\"?>\n"[..]);
    xml_no_tag.resize(3072, b'a');
    c.bench_function("detect XML no-tag 3KB", |b| {
        b.iter(|| detect(black_box(xml_no_tag.as_slice())))
    });
}

criterion_group!(benches, benchmark_detection);
criterion_main!(benches);
