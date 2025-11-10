# Changelog

All notable changes to this project will be documented in this file.

Used prefixes:
`Added`
`Updated`
`Deleted`
`BREAKING`

## Unreleased

## 0.2.8 - 2025-11-10

* Updated: Fixed CLSID-based OLE format detection to prevent false positives (MSG, MSI, PUB files were incorrectly detected as PPT)

## 0.2.7 - 2025-11-07

* Updated: Updated `mimetype!` macro to have optional param and reduce cases
* Updated: Package description to reflect ~450 supported file formats

## 0.2.6 - 2025-10-30

* Added: 24 new file format detections
* Added: Prefix map optimization for fast child lookup via
* Added: Unified `mimetype!` macro that consolidates all MIME type definition functionality

## 0.2.5 - 2025-10-23

* Added: `MimeKind` categorization to 175+ file formats across 13 categories
* Added: `kind()` methods to get MimeType kind
* Updated:  changed mimetype registration to recursive
* Updated:  fixed RTF detection

## 0.2.4 - 2025-10-10

* Added: DXF (AutoCAD Drawing Exchange Format) detection (`image/vnd.dxf`)
* Added: WordPerfect document detection (`application/vnd.wordperfect`)
* Updated: Enhanced CPIO detector to support binary format variant
* Updated: Optimized JavaScript keyword detection with adaptive algorithm (memchr for large files, simple scanning for small files)

## 0.2.3 - 2025-09-29

* Updated: improve few checks (CSV,TSV, NDJSON, VCARD) improve performance only

## 0.2.2 - 2025-09-08

* Updated: improve xml like tag check
* Updated: methods with path for file changed from `&str`` to`<P: AsRef<Path>>`

## 0.2.1 - 2025-09-03

* Added: ZIP child format detection (Office documents, JAR, APK, etc.)
* Added: OLE child format detection (Office documents, AAF, MSI, etc.)
* Added: new MIME types synchronized from Go mimetype library:
* Added new ~80 mimetypes

**⚠️ BREAKING CHANGE**: Documents that were previously detected as generic `application/zip` may now be detected as their specific format (e.g., `application/vnd.openxmlformats-officedocument.wordprocessingml.document` for DOCX, `application/java-archive` for JAR, `application/vnd.android.package-archive` for APK). This provides more accurate detection but may affect applications relying on the previous generic ZIP detection.

## 0.1.2 - 2025-08-13

* Added: constants package for MIME type names
* Added: CHANGELOG.md file

## [0.1.1] - 2025-07-25

* Updated: reduced small allocations

## [0.1.0] - 2025-07-23

* Added: Initial release with comprehensive MIME type detection
