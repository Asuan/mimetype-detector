use crate::{register_extension, register_mime};

pub struct MimeType {
    mime: &'static str,
    aliases: &'static [&'static str],
    extension: &'static str,
    extension_aliases: &'static [&'static str],
    matcher: fn(&[u8]) -> bool,
    children: &'static [&'static MimeType],
    parent: Option<&'static MimeType>,
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

    pub fn register(&'static self) {
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
