//! MIME type categorization using bitflags
//!
//! This module provides a bitmask-based categorization system for MIME types,
//! allowing efficient type checking and multiple category membership.

/// Bitmask flags representing different MIME type categories
///
/// A MIME type can belong to multiple categories (e.g., an executable can also be an archive).
/// Use bitwise operations to combine or check for multiple kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MimeKind(u32);

impl MimeKind {
    /// No specific kind assigned
    pub const UNKNOWN: MimeKind = MimeKind(0);

    /// Archive formats (ZIP, TAR, 7Z, RAR, etc.)
    pub const ARCHIVE: MimeKind = MimeKind(1 << 0);

    /// Video formats (MP4, WebM, AVI, MKV, etc.)
    pub const VIDEO: MimeKind = MimeKind(1 << 1);

    /// Audio formats (MP3, FLAC, WAV, AAC, etc.)
    pub const AUDIO: MimeKind = MimeKind(1 << 2);

    /// Image formats (PNG, JPEG, GIF, WebP, etc.)
    pub const IMAGE: MimeKind = MimeKind(1 << 3);

    /// Document formats (PDF, DOCX, ODT, etc.)
    pub const DOCUMENT: MimeKind = MimeKind(1 << 4);

    /// Text formats (plain text, HTML, XML, JSON, etc.)
    pub const TEXT: MimeKind = MimeKind(1 << 5);

    /// Font formats (TTF, OTF, WOFF, etc.)
    pub const FONT: MimeKind = MimeKind(1 << 6);

    /// Executable/binary formats (ELF, PE, Mach-O, WASM, etc.)
    pub const EXECUTABLE: MimeKind = MimeKind(1 << 7);

    /// Application-specific formats (APK, JAR, MSI, etc.)
    pub const APPLICATION: MimeKind = MimeKind(1 << 8);

    /// 3D model formats (glTF, STL, OBJ, etc.)
    pub const MODEL: MimeKind = MimeKind(1 << 9);

    /// Database formats (SQLite, etc.)
    pub const DATABASE: MimeKind = MimeKind(1 << 10);

    /// Spreadsheet formats (XLSX, ODS, CSV, etc.)
    pub const SPREADSHEET: MimeKind = MimeKind(1 << 11);

    /// Presentation formats (PPTX, ODP, etc.)
    pub const PRESENTATION: MimeKind = MimeKind(1 << 12);

    /// Check if this kind contains the specified flag(s)
    #[inline]
    pub const fn contains(&self, other: MimeKind) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Combine this kind with another using bitwise OR
    #[inline]
    pub const fn union(self, other: MimeKind) -> MimeKind {
        MimeKind(self.0 | other.0)
    }

    /// Check if this is an archive format
    #[inline]
    pub const fn is_archive(&self) -> bool {
        self.contains(MimeKind::ARCHIVE)
    }

    /// Check if this is a video format
    #[inline]
    pub const fn is_video(&self) -> bool {
        self.contains(MimeKind::VIDEO)
    }

    /// Check if this is an audio format
    #[inline]
    pub const fn is_audio(&self) -> bool {
        self.contains(MimeKind::AUDIO)
    }

    /// Check if this is an image format
    #[inline]
    pub const fn is_image(&self) -> bool {
        self.contains(MimeKind::IMAGE)
    }

    /// Check if this is a document format
    #[inline]
    pub const fn is_document(&self) -> bool {
        self.contains(MimeKind::DOCUMENT)
    }

    /// Check if this is a text format
    #[inline]
    pub const fn is_text(&self) -> bool {
        self.contains(MimeKind::TEXT)
    }

    /// Check if this is a font format
    #[inline]
    pub const fn is_font(&self) -> bool {
        self.contains(MimeKind::FONT)
    }

    /// Check if this is an executable/binary format
    #[inline]
    pub const fn is_executable(&self) -> bool {
        self.contains(MimeKind::EXECUTABLE)
    }

    /// Check if this is an application-specific format
    #[inline]
    pub const fn is_application(&self) -> bool {
        self.contains(MimeKind::APPLICATION)
    }

    /// Check if this is a 3D model format
    #[inline]
    pub const fn is_model(&self) -> bool {
        self.contains(MimeKind::MODEL)
    }

    /// Check if this is a database format
    #[inline]
    pub const fn is_database(&self) -> bool {
        self.contains(MimeKind::DATABASE)
    }

    /// Check if this is a spreadsheet format
    #[inline]
    pub const fn is_spreadsheet(&self) -> bool {
        self.contains(MimeKind::SPREADSHEET)
    }

    /// Check if this is a presentation format
    #[inline]
    pub const fn is_presentation(&self) -> bool {
        self.contains(MimeKind::PRESENTATION)
    }
}

impl Default for MimeKind {
    fn default() -> Self {
        MimeKind::UNKNOWN
    }
}

impl std::fmt::Display for MimeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            return write!(f, "UNKNOWN");
        }

        let mut first = true;

        macro_rules! write_kind {
            ($check:expr, $name:expr) => {
                if $check {
                    if !first {
                        write!(f, " | ")?;
                    }
                    write!(f, $name)?;
                    first = false;
                }
            };
        }

        write_kind!(self.is_archive(), "ARCHIVE");
        write_kind!(self.is_video(), "VIDEO");
        write_kind!(self.is_audio(), "AUDIO");
        write_kind!(self.is_image(), "IMAGE");
        write_kind!(self.is_document(), "DOCUMENT");
        write_kind!(self.is_text(), "TEXT");
        write_kind!(self.is_font(), "FONT");
        write_kind!(self.is_executable(), "EXECUTABLE");
        write_kind!(self.is_application(), "APPLICATION");
        write_kind!(self.is_model(), "MODEL");
        write_kind!(self.is_database(), "DATABASE");
        write_kind!(self.is_spreadsheet(), "SPREADSHEET");
        write_kind!(self.is_presentation(), "PRESENTATION");

        if first {
            // No kinds were written, shouldn't happen but handle it
            write!(f, "UNKNOWN")
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_kind() {
        let kind = MimeKind::IMAGE;
        assert!(kind.is_image());
        assert!(!kind.is_video());
        assert!(!kind.is_audio());
    }

    #[test]
    fn test_multiple_kinds() {
        let kind = MimeKind::ARCHIVE.union(MimeKind::EXECUTABLE);
        assert!(kind.is_archive());
        assert!(kind.is_executable());
        assert!(!kind.is_image());
    }

    #[test]
    fn test_contains() {
        let kind = MimeKind::ARCHIVE.union(MimeKind::APPLICATION);
        assert!(kind.contains(MimeKind::ARCHIVE));
        assert!(kind.contains(MimeKind::APPLICATION));
        assert!(!kind.contains(MimeKind::VIDEO));
    }

    #[test]
    fn test_unknown() {
        let kind = MimeKind::UNKNOWN;
        assert!(!kind.is_archive());
        assert!(!kind.is_video());
        assert_eq!(kind, MimeKind::UNKNOWN);
    }

    #[test]
    fn test_display_single_kind() {
        assert_eq!(MimeKind::IMAGE.to_string(), "IMAGE");
        assert_eq!(MimeKind::VIDEO.to_string(), "VIDEO");
        assert_eq!(MimeKind::AUDIO.to_string(), "AUDIO");
        assert_eq!(MimeKind::ARCHIVE.to_string(), "ARCHIVE");
        assert_eq!(MimeKind::DOCUMENT.to_string(), "DOCUMENT");
        assert_eq!(MimeKind::TEXT.to_string(), "TEXT");
        assert_eq!(MimeKind::FONT.to_string(), "FONT");
        assert_eq!(MimeKind::EXECUTABLE.to_string(), "EXECUTABLE");
        assert_eq!(MimeKind::APPLICATION.to_string(), "APPLICATION");
        assert_eq!(MimeKind::MODEL.to_string(), "MODEL");
        assert_eq!(MimeKind::DATABASE.to_string(), "DATABASE");
        assert_eq!(MimeKind::SPREADSHEET.to_string(), "SPREADSHEET");
        assert_eq!(MimeKind::PRESENTATION.to_string(), "PRESENTATION");
    }

    #[test]
    fn test_display_unknown() {
        assert_eq!(MimeKind::UNKNOWN.to_string(), "UNKNOWN");
    }

    #[test]
    fn test_display_multiple_kinds() {
        let combined = MimeKind::ARCHIVE.union(MimeKind::EXECUTABLE);
        assert_eq!(combined.to_string(), "ARCHIVE | EXECUTABLE");

        let triple = MimeKind::DOCUMENT
            .union(MimeKind::ARCHIVE)
            .union(MimeKind::TEXT);
        assert_eq!(triple.to_string(), "ARCHIVE | DOCUMENT | TEXT");
    }
}
