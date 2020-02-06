use core::fmt;

#[derive(Debug, PartialEq)]
pub enum MiniYamlError {
    /// error produced when an alias is encountered in the parser input
    AliasesDisallowed,
    /// error produced when an anchor is encountered in the parser input
    AnchorsDisallowed,
    /// error produced when a tag is encountered in the parser input
    TagsDisallowed,
    /// error produced when parsing invalid Yaml
    ParseError {
        /// the line in the input on which the error occurred
        line: usize, 
        /// the column in the input on which the error occurred
        col: usize,
        /// more information about the error, if there is any
        msg: Option<String>,
    },
}

impl std::error::Error for MiniYamlError {}

impl fmt::Display for MiniYamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MiniYamlError::ParseError{line, col, msg: None} => write!(f, "parsing error occurred at line {} col {}", line, col),
            MiniYamlError::ParseError{line, col, msg: Some(msg)} => write!(f, "parsing error occurred at line {} col {} : {}", line, col, msg),
            _ => write!(
                f,
                "{}",
                match self {
                    MiniYamlError::AliasesDisallowed => "aliases are disallowed in minimal-yaml",
                    MiniYamlError::AnchorsDisallowed => "anchors are disallowed in minimal-yaml",
                    MiniYamlError::TagsDisallowed => "tags are disallowed in minimal-yaml",
                    _ => unreachable!()
                }
            )
        }
    }
}
