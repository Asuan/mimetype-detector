# Changelog

All notable changes to this project will be documented in this file.

Used prefixes:
`Added`
`Updated`
`Deleted`
`BREAKING`

## Unreleased

## 0.3.5 - 2026.02.09

* Updated: move some items into prefix for performance perspective
* Added: RIFF root item to group riff child format
* Updated: improve CSV like detections
* Updated: improved bash script detection
* Updated: improved html detectors performance
* Updated: improved TIFF-based RAW format detection (DNG, ARW, PEF, SR2, 3FR, ORF) using simple string patterns

## 0.3.4 - 2025-12-15

* Added: Java (`text/x-java`), TypeScript (`text/x-typescript`), C (`text/x-c`), C++ (`text/x-c++`), Go (`text/x-go`), Rust (`text/x-rust`), C# (`text/x-csharp`), and Visual Basic (`text/x-vb`) source code detection
* Updated: C++ is now a child of C in detection tree for `.c .h` header file detection
* Updated: Optimized EBML `vint_width` function using CPU intrinsics (`leading_zeros()`)
* Added: 422 MIME type detection tests (524 total tests)

## 0.3.3 - 2025-12-04

* Added: PSV (Pipe-Separated Values) format support with MIME type `text/pipe-separated-values` (.psv extension)
* Added: SSV (Semicolon-Separated Values) format support with MIME type `text/semicolon-separated-values` (.ssv extension)
* Added: UTF-8 BOM and UTF-16 BE/LE variants for PSV and SSV formats
* Updated: PST (Personal Storage Table) detection now uses proper magic number detection ("!BDN" signatures)
* Added: OLE Compound File v4 format support - enables detection of modern MSI files (Windows 10/11, Office 2016+) when using `detect_with_limit(data, 8192)`
* Added: MSP (Windows Installer Patch) format detection with MIME types `application/x-ms-patch` and `application/x-msp`
* Added: Sega Game ROM formats to root tree (Game Gear, Master System) detect via `detect_with_limit(data, 32768)`
* Updated CSV/TSV detecting:
* Updated: ISOBMFF format detection functions (HEIC, HEIF, AVIF, 3GPP, etc.)

## 0.3.2 - 2025-11-27

* Updated: fixed panic in ZIP iterator when position exceeds data length
* Added: `detect_with_limit()`, `detect_reader_with_limit()`, and `detect_file_with_limit()` methods to allow custom read limits for MIME type detection

## 0.3.1 - 2025-11-19

* Added LTO (Link-Time Optimization) in release builds
* Updated: aliases and extension aliases for some MIME types
* Added method  `aliases()`  and `extension_aliases()` to get MIME type and extension aliases

## 0.3.0 - 2025-11-18

* Added: name parameter (human-readable name)
* Added: UTF-8 BOM format-specific children like matching UTF-16 BE/LE
* Updated: JSON detection to handle partial content in first 512 bytes
* Updated: added aliases for some MIME types

## 0.2.8 - 2025-11-10

* Updated: fixed CLSID-based OLE format detection to prevent false positives (MSG, MSI, PUB files were incorrectly detected as PPT)

## 0.2.7 - 2025-11-07

* Updated: `mimetype!` macro to have optional param and reduce cases
* Updated: package description to reflect ~450 supported file formats

## 0.2.6 - 2025-10-30

* Added: 24 new file format detections
* Added: prefix map optimization for fast child lookup
* Added: unified `mimetype!` macro that consolidates all MIME type definition functionality

## 0.2.5 - 2025-10-23

* Added: `MimeKind` categorization to 175+ file formats across 13 categories
* Added: `kind()` methods to get MimeType kind
* Updated: changed MIME type registration to recursive
* Updated: fixed RTF detection

## 0.2.4 - 2025-10-10

* Added: DXF (AutoCAD Drawing Exchange Format) detection (`image/vnd.dxf`)
* Added: WordPerfect document detection (`application/vnd.wordperfect`)
* Updated: enhanced CPIO detector to support binary format variant
* Updated: optimized JavaScript keyword detection with adaptive algorithm (memchr for large files, simple scanning for small files)

## 0.2.3 - 2025-09-29

* Updated: improved checks (CSV, TSV, NDJSON, VCARD) for performance

## 0.2.2 - 2025-09-08

* Updated: improved XML-like tag check
* Updated: methods with path for file changed from `&str` to `<P: AsRef<Path>>`

## 0.2.1 - 2025-09-03

* Added: ZIP child format detection (Office documents, JAR, APK, etc.)
* Added: OLE child format detection (Office documents, AAF, MSI, etc.)
* Added: new MIME types synchronized from Go mimetype library
* Added: ~80 new MIME types

**⚠️ BREAKING CHANGE**: Documents that were previously detected as generic `application/zip` may now be detected as their specific format (e.g., `application/vnd.openxmlformats-officedocument.wordprocessingml.document` for DOCX, `application/java-archive` for JAR, `application/vnd.android.package-archive` for APK). This provides more accurate detection but may affect applications relying on the previous generic ZIP detection.

## [0.1.2] - 2025-08-13

* Added: constants package for MIME type names
* Added: CHANGELOG.md file

## [0.1.1] - 2025-07-25

* Updated: reduced small allocations

## [0.1.0] - 2025-07-23

* Added: initial release with comprehensive MIME type detection
