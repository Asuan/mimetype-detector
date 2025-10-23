use crate::{register_extension, register_mime, MimeKind};

pub struct MimeType {
    mime: &'static str,
    aliases: &'static [&'static str],
    extension: &'static str,
    extension_aliases: &'static [&'static str],
    matcher: fn(&[u8]) -> bool,
    children: &'static [&'static MimeType],
    parent: Option<&'static MimeType>,
    kind: MimeKind,
}

impl MimeType {
    pub const fn new(
        mime: &'static str,
        extension: &'static str,
        matcher: fn(&[u8]) -> bool,
        children: &'static [&'static MimeType],
    ) -> Self {
        Self {
            mime,
            aliases: &[],
            extension,
            extension_aliases: &[],
            matcher,
            children,
            parent: None,
            kind: MimeKind::UNKNOWN,
        }
    }

    pub const fn with_aliases(mut self, aliases: &'static [&'static str]) -> Self {
        self.aliases = aliases;
        self
    }

    pub const fn with_extension_aliases(
        mut self,
        extension_aliases: &'static [&'static str],
    ) -> Self {
        self.extension_aliases = extension_aliases;
        self
    }

    pub const fn with_parent(mut self, parent: &'static MimeType) -> Self {
        self.parent = Some(parent);
        self
    }

    pub const fn with_kind(mut self, kind: MimeKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn register(&'static self) {
        // Register this MIME type
        register_mime(self.mime, self.matcher);
        if !self.extension.is_empty() {
            register_extension(self.extension, self.matcher);
        }

        for alias in self.aliases {
            register_mime(alias, self.matcher);
        }

        for ext_alias in self.extension_aliases {
            register_extension(ext_alias, self.matcher);
        }

        // Recursively register all children
        for child in self.children {
            child.register();
        }
    }

    pub fn mime(&self) -> &'static str {
        self.mime
    }

    pub fn extension(&self) -> &'static str {
        self.extension
    }

    pub fn parent(&self) -> Option<&'static MimeType> {
        self.parent
    }

    /// Get the combined kind including all parent kinds
    ///
    /// This method returns a MimeKind that includes both the current type's kind
    /// and all parent kinds merged together using bitwise OR.
    ///
    /// For example, DOCX has kind DOCUMENT, but since its parent is ZIP (ARCHIVE),
    /// the returned kind will be DOCUMENT | ARCHIVE.
    ///
    /// If no explicit parent is set, returns only this type's own kind.
    /// (ROOT is used as implicit parent for tree structure, but its kind is not inherited)
    pub fn kind(&'static self) -> MimeKind {
        if let Some(parent) = self.parent {
            return self.kind.union(parent.kind());
        }

        // No explicit parent - just return own kind
        self.kind
    }

    pub fn is(&self, expected_mime: &str) -> bool {
        let expected = expected_mime.split(';').next().unwrap_or("").trim();
        let found = self.mime.split(';').next().unwrap_or("").trim();

        if expected == found {
            return true;
        }
        self.aliases.iter().any(|alias| alias == &expected)
    }

    pub fn match_bytes(&'static self, input: &[u8]) -> &'static MimeType {
        for child in self.children {
            if (child.matcher)(input) {
                return child.match_bytes(input);
            }
        }
        self
    }

    pub fn flatten(&'static self) -> Vec<&'static MimeType> {
        let mut result = vec![self];
        for child in self.children {
            result.extend(child.flatten());
        }
        result
    }
}

impl std::fmt::Display for MimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mime)
    }
}
