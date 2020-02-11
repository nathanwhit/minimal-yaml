use core::fmt;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
#[allow(dead_code)]
#[doc(hidden)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum MiniYamlError {
    /// error produced when an alias is encountered in the parser input
    AliasesDisallowed,
    /// error produced when an anchor is encountered in the parser input
    AnchorsDisallowed,
    /// error produced when a tag is encountered in the parser input
    TagsDisallowed,
}

/// An error generated while parsing input
#[derive(Debug, PartialEq, Clone)]
pub struct YamlParseError {
    /// the line in the input on which the error occurred
    pub(crate) line: usize,
    /// the column in the input on which the error occurred
    pub(crate) col: usize,
    /// more information about the error, if there is any
    pub(crate) msg: Option<String>,
    pub(crate) source: Option<MiniYamlError>,
}

impl Error for YamlParseError {}

impl fmt::Display for YamlParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.msg {
            Some(ref msg) => write!(
                f,
                "error occurred parsing the input at line {}, column {} : {}",
                self.line, self.col, msg
            ),
            None => write!(
                f,
                "error occurred parsing the input at line {}, column {}",
                self.line, self.col
            ),
        }
    }
}

impl Error for MiniYamlError {}

impl fmt::Display for MiniYamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MiniYamlError::AliasesDisallowed => "aliases are disallowed in minimal-yaml",
                MiniYamlError::AnchorsDisallowed => "anchors are disallowed in minimal-yaml",
                MiniYamlError::TagsDisallowed => "tags are disallowed in minimal-yaml",
            }
        )
    }
}
