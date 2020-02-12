#![warn(clippy::all, clippy::pedantic)]
mod errors;
mod parse;
mod tests;
mod tokenize;

pub use crate::errors::{YamlParseError};

pub(crate) type Result<T> = std::result::Result<T, YamlParseError>;

use parse::Parser;
use tokenize::Tokenizer;

use std::{fmt, fmt::Display};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
/// A Yaml Element
pub enum Yaml<'a> {
    /// A literal value, losslessly interpreted as a string
    Scalar(&'a str),

    /// A sequence of values in flow style
    /// `[x, y, z]`
    /// or in block style
    /// ```yaml
    ///     - x
    ///     - y
    ///     - z
    /// ```
    Sequence(Vec<Yaml<'a>>),

    /// A mapping from key to value in flow style
    /// `{x: X, y: Y, z: Z}`
    /// or in block style
    /// ```yaml
    ///     x: X
    ///     y: Y
    ///     z: Z
    /// ```
    Mapping(Vec<Entry<'a>>),
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum PrintStyle {
    Block,
    Flow
}

fn print_indent(indent: usize, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:indent$}", "", indent=indent)
}

fn print_yaml(node: &Yaml<'_>, indent: usize, f: &mut fmt::Formatter, style: PrintStyle) -> fmt::Result {
    const INDENT_AMT: usize = 2;
    match node {
        Yaml::Scalar(slice) => {
            write!(f, "{}", slice)
        }
        Yaml::Sequence(seq) => {
            match style {
                PrintStyle::Block => {
                    for el in seq.iter() {
                        print_indent(indent, f)?;
                        write!(f, "-")?;
                        match el {
                            Yaml::Scalar(slice) => write!(f, " {scal}\n", scal=slice)?,
                            Yaml::Sequence(..) | Yaml::Mapping(..) => {
                                write!(f, "\n")?;
                                print_yaml(el, indent+INDENT_AMT, f, style)?;
                            },
                        }
                    }
                },
                PrintStyle::Flow => {
                    write!(f, "[ ")?;
                    let last_idx = seq.len() - 1;
                    for (idx, elem) in seq.iter().enumerate() {
                        if idx == last_idx {
                            write!(f, "{}", elem)?;
                        } else {
                            write!(f, "{}, ", elem)?;
                        }
                    }
                    write!(f, " ]")?;
                }
            }
            Ok(())
        }
        Yaml::Mapping(map) => {
            match style {
                PrintStyle::Block => {
                    for entry in map.iter() {
                        match &entry.key {
                            Yaml::Scalar(..) => {
                                print_indent(indent, f)?;
                                print_yaml(&entry.key, indent, f, PrintStyle::Block)?;
                                write!(f, " ")?;
                            },
                            Yaml::Sequence(..) | Yaml::Mapping(..) => {
                                print_yaml(&entry.key, indent+INDENT_AMT, f, PrintStyle::Block)?;
                                print_indent(indent, f)?;
                            },
                        }
                        write!(f, ":")?;
                        match &entry.value {
                            Yaml::Scalar(..) => {
                                write!(f, " ")?;
                                print_yaml(&entry.value, indent, f, PrintStyle::Block)?;
                                write!(f, "\n")?;
                            },
                            Yaml::Sequence(..) | Yaml::Mapping(..) => {
                                write!(f, "\n")?;
                                print_yaml(&entry.value, indent + INDENT_AMT, f, PrintStyle::Block)?
                            }
                        }
                    }
                },
                PrintStyle::Flow => {
                    write!(f, "{{")?;
                    let last_idx = map.len() - 1;
                    for (idx, entry) in map.iter().enumerate() {
                        if idx == last_idx {
                            write!(f, "{}", entry)?;
                        } else {
                            write!(f, "{}, ", entry)?;
                        }
                    }
                    write!(f, "}}")?;
                }
            }
            Ok(())
        }
    }
}

impl Display for Yaml<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print_yaml(&self, 0, f, PrintStyle::Block)
    }
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
/// A Yaml map entry
pub struct Entry<'a> {
    /// The key associated with the entry
    pub key: Yaml<'a>,
    /// The value which the key maps to
    pub value: Yaml<'a>,
}

impl<'a> Entry<'a> {
    #[allow(clippy::must_use_candidate)]
    pub fn new(key: Yaml<'a>, value: Yaml<'a>) -> Self {
        Self { key, value }
    }
}

impl<'a> Display for Entry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match self.key 
        write!(f, "{} : {}", self.key, self.value)
    }
}

/// Parse Yaml input. Returns the top level Yaml element on success
/// # Errors
/// Returns `Err` if the input is invalid Yaml, with a message indicating
/// where the error occurred and possibly more information on the cause
pub fn parse<'a>(input: &'a str) -> Result<Yaml<'a>> {
    let tokenizer = Tokenizer::from_str(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(input, &tokens)?;
    parser.parse()
}
