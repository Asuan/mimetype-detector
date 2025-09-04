# Changelog

All notable changes to this project will be documented in this file.

Used prefixes:
`Added`
`Updated`
`Deleted`
`BREAKING`

## unrealized
* Updated: improve html tag check


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
