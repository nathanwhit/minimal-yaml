use core::fmt;
use core::iter::{Peekable, Iterator};
use super::Result;
enum MiniYamlTokenKind<'a> {
    Literal(&'a str),
    Whitespace(&'a str),
    Block,
    Colon,
    Comma,
    Dash,
    Fold,
    Newline,
    LeftBrace,
    LeftBracket,
    Plus,
    RightBrace,
    RightBracket,
    TripleDash,
    TripleDot,
}

impl<'a> fmt::Display for MiniYamlTokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MiniYamlTokenKind::*;
        write!(f, "{}", match self {
            Literal(val) => val,
            Block => "|",
            Colon => ":",
            Comma => ",",
            Dash => "-",
            Fold => ">",
            LeftBrace => "{",
            LeftBracket => "[",
            Newline => "\n",
            Plus => "+",
            RightBrace => "}",
            RightBracket => "]",
            TripleDash => "---",
            TripleDot => "...",
            Whitespace(space) => space,
        })
    }
}

impl<'a> MiniYamlTokenKind<'a> {

}

struct MiniYamlToken<'a> {
    start_idx: usize,
    end_idx: usize,
    kind: MiniYamlTokenKind<'a>
}

use core::str::CharIndices;

struct MiniYamlTokenizer<'a> {
    source: Peekable<CharIndices<'a>>,
    tokens: Vec<MiniYamlToken<'a>>
}


impl<'a> MiniYamlTokenizer<'a> {
    pub fn from_str(raw: &'a str) -> Self {
        Self {
            source: raw.char_indices().peekable(),
            tokens: Vec::new(),
        }
    }
    
    pub fn try_from_bytes(raw: &'a [u8]) -> std::result::Result<Self, std::str::Utf8Error> {
        Ok(Self::from_str(std::str::from_utf8(raw)?))
    }
}

struct SourceChar {
    idx: usize,
    val: char,
}

impl From<&(usize, char)> for SourceChar {
    fn from(other: &(usize, char)) -> Self {
        Self {
            idx: other.0,
            val: other.1
        }
    }
}

impl<'a> MiniYamlTokenizer<'a> {
    fn peek(&mut self) -> Option<SourceChar> {
        self.source.peek().map(SourceChar::from)
    }

    fn next_char(&mut self) -> Option<SourceChar> {
        self.source.next().map(|ref tup| SourceChar::from(tup))
    }
    fn advance(&mut self) -> bool {
        self.source.next().is_some()
    }
}