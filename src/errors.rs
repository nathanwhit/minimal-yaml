use core::fmt;
use crate::tokenize::Span;

#[derive(Debug)]
pub enum MiniYamlError {
    /// error produced when an alias is encountered in the parser input
    AliasesDisallowed(Span),
    /// error produced when an anchor is encountered in the parser input
    AnchorsDisallowed(Span),
    /// error produced when a tag is encountered in the parser input
    TagsDisallowed(Span),
    /// error produced when parsing invalid Yaml
    ParseError(Span),
}

impl std::error::Error for MiniYamlError {}

impl fmt::Display for MiniYamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MiniYamlError::AliasesDisallowed(..) => "aliases are disallowed in minimal-yaml",
                MiniYamlError::AnchorsDisallowed(..) => "anchors are disallowed in minimal-yaml",
                MiniYamlError::TagsDisallowed(..) => "tags are disallowed in minimal-yaml",
                MiniYamlError::ParseError(..) => "parsing error occurred",
            }
        )
    }
}
