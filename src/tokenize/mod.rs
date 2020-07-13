use core::fmt;
use core::iter::{Iterator, Peekable};
use core::mem;
use core::ops::Add;
use core::str::Bytes;

mod tests;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ByteIdx(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ByteLen(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Span {
    start: ByteIdx,
    end: ByteIdx,
}

impl Span {
    fn new<T: Into<ByteIdx>>(start: T, end: T) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    pub(crate) fn dummy() -> Self {
        Self::new(0, 0)
    }
}

impl ByteIdx {
    /// Returns a span from the current byte index
    /// to `other`.
    pub(crate) fn to(self, other: ByteIdx) -> Span {
        Span {
            start: self,
            end: other,
        }
    }
}

impl From<usize> for ByteIdx {
    fn from(other: usize) -> Self {
        Self(other)
    }
}

impl From<usize> for ByteLen {
    fn from(other: usize) -> Self {
        Self(other)
    }
}

impl From<ByteIdx> for usize {
    fn from(other: ByteIdx) -> Self {
        other.0
    }
}

impl From<ByteLen> for usize {
    fn from(other: ByteLen) -> Self {
        other.0
    }
}

impl Add<ByteLen> for ByteIdx {
    type Output = ByteIdx;

    fn add(self, rhs: ByteLen) -> Self::Output {
        ByteIdx(self.0 + rhs.0)
    }
}

impl Add<ByteIdx> for ByteIdx {
    type Output = ByteIdx;

    fn add(self, rhs: ByteIdx) -> Self::Output {
        ByteIdx(self.0 + rhs.0)
    }
}

impl Add<ByteIdx> for ByteLen {
    type Output = ByteIdx;

    fn add(self, rhs: ByteIdx) -> Self::Output {
        ByteIdx(self.0 + rhs.0)
    }
}

impl Add<ByteLen> for ByteLen {
    type Output = ByteLen;

    fn add(self, rhs: ByteLen) -> Self::Output {
        ByteLen(self.0 + rhs.0)
    }
}

trait ByteExt {
    fn is_indicator(self) -> bool;
    fn is_linebreak(self) -> bool;
    fn span(self, pos: impl Into<ByteIdx>) -> Span;
}

impl ByteExt for u8 {
    fn is_indicator(self) -> bool {
        match self {
            b'-' | b'?' | b':' | b',' | b'[' | b']' | b'{' | b'}' | b'#' | b'&' | b'*' | b'!'
            | b'|' | b'>' | b'\"' | b'\'' | b'%' | b'@' | b'`' => true,
            _ => false,
        }
    }
    fn is_linebreak(self) -> bool {
        self == b'\n' || self == b'\r'
    }
    fn span(self, pos: impl Into<ByteIdx>) -> Span {
        let pos = pos.into();
        Span {
            start: ByteIdx(pos.0 - 1),
            end: pos,
        }
    }
}

trait UsizeExt {
    fn to(self, other: usize) -> Span;
}

impl UsizeExt for usize {
    fn to(self, other: usize) -> Span {
        ByteIdx(self).to(ByteIdx(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TokenKind<'a> {
    Literal(&'a str),
    Whitespace(usize),
    Ampersand,
    Asterisk,
    Colon,
    Comma,
    Dash,
    Fold,
    Newline,
    LeftBrace,
    LeftBracket,
    Pipe,
    Plus,
    RightBrace,
    RightBracket,
    SingleQuote,
    DoubleQuote,
    QuestionMark,
    Dummy,
}

impl<'a> TokenKind<'a> {
    #[allow(unused)]
    pub fn is_indicator(&self) -> bool {
        use TokenKind::*;
        matches!(
            self,
            Colon
                | Comma
                | Dash
                | LeftBrace
                | LeftBracket
                | Pipe
                | RightBrace
                | RightBracket
                | SingleQuote
                | DoubleQuote
                | QuestionMark
        )
    }

    #[allow(unused)]
    pub fn is_flow_indicator(&self) -> bool {
        use TokenKind::*;
        matches!(
            self,
            Comma | LeftBracket | LeftBrace | RightBracket | RightBrace
        )
    }
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind::*;
        write!(
            f,
            "{}",
            match self {
                Literal(val) => val,
                Whitespace(..) => "WHITESPACE",
                Ampersand => "&",
                Asterisk => "*",
                Colon => ":",
                Comma => ",",
                Dash => "-",
                Fold => ">",
                LeftBrace => "{",
                LeftBracket => "[",
                Newline => "\n",
                Pipe => "|",
                Plus => "+",
                RightBrace => "}",
                RightBracket => "]",
                SingleQuote => r"'",
                DoubleQuote => r#"""#,
                QuestionMark => "?",
                Dummy => "DUMMY",
            }
        )
    }
}

impl<'a> Default for TokenKind<'a> {
    fn default() -> Self {
        Self::Dummy
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind<'a>,
    pub(crate) span: Span,
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            kind: TokenKind::Dummy,
            span: Span::dummy(),
        }
    }
}

impl<'a> Token<'a> {
    /// Constructs a new token.
    pub(crate) fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }

    pub(crate) fn start(&self) -> ByteIdx {
        self.span.start
    }

    pub(crate) fn end(&self) -> ByteIdx {
        self.span.end
    }
}

pub(crate) struct Tokenizer<'a> {
    source: &'a str,
    bytes: Peekable<Bytes<'a>>,
    tokens: Vec<Token<'a>>,
    idx: usize,
    current: Option<u8>,
}

impl<'a> Tokenizer<'a> {
    pub fn from_str(raw: &'a str) -> Self {
        let mut bytes = raw.bytes().peekable();
        let current = bytes.next();
        Self {
            bytes,
            source: raw,
            tokens: Vec::new(),
            idx: 0,
            current,
        }
    }

    #[allow(unused)]
    pub fn new(raw: &'a str) -> Self {
        Self::from_str(raw)
    }
}

struct SourceStr<'a> {
    slice: &'a str,
    span: Span,
}

impl<'a> SourceStr<'a> {
    pub fn new(slice: &'a str, span: Span) -> Self {
        Self { slice, span }
    }
}

impl<'a> Tokenizer<'a> {
    /// Returns the next character, if one exists, without advancing
    /// the source position.
    #[allow(unused)]
    fn peek(&mut self) -> Option<u8> {
        self.bytes.peek().map(|&b| b)
    }

    /// Returns the next character, if one exists, and advances
    /// the source position.
    fn next(&mut self) -> Option<u8> {
        self.idx += 1;
        let next = self.bytes.next();
        mem::replace(&mut self.current, next)
    }

    /// Advances the source position by one character. Returns `false`
    /// if the source is exhausted, `true` otherwise.
    fn advance(&mut self) -> bool {
        self.next().is_some()
    }

    fn consume_whitespace(&mut self) -> Token<'a> {
        let SourceStr { slice, span } = self.consume_matches(|chr| chr == b' ' || chr == b'\t');
        Token::new(TokenKind::Whitespace(slice.len()), span)
    }

    fn consume_literal(&mut self) -> Token<'a> {
        let SourceStr { slice, span } =
            self.consume_matches(|chr| !(chr.is_indicator() || chr.is_ascii_whitespace()));
        Token::new(TokenKind::Literal(slice), span)
    }

    fn consume_matches(&mut self, matches: impl Fn(u8) -> bool) -> SourceStr<'a> {
        let tok_start = self.idx - 1;
        let mut tok_end = tok_start + 1;
        loop {
            match self.current {
                Some(chr) if matches(chr) => {
                    tok_end += 1;
                    self.advance();
                }
                _ => {
                    return SourceStr::new(&self.source[tok_start..tok_end], tok_start.to(tok_end))
                }
            }
        }
    }

    /// Tokenizes the source, consuming the Tokenizer.
    /// Returns the tokenized source.
    pub fn tokenize(mut self) -> Vec<Token<'a>> {
        use TokenKind::*;
        while let Some(chr) = self.next() {
            // TODO: Consider preserving comments instead of just throwing them out
            match chr {
                b'&' => self.push_tok_with(Ampersand, chr.span(self.idx)),
                b'*' => self.push_tok_with(Asterisk, chr.span(self.idx)),
                b':' => self.push_tok_with(Colon, chr.span(self.idx)),
                b',' => self.push_tok_with(Comma, chr.span(self.idx)),
                b'-' => self.push_tok_with(Dash, chr.span(self.idx)),
                b'\"' => self.push_tok_with(DoubleQuote, chr.span(self.idx)),
                b'>' => self.push_tok_with(Fold, chr.span(self.idx)),
                b'\n' => self.push_tok_with(Newline, chr.span(self.idx)),
                b'\r' => {
                    let span = match self.peek() {
                        Some(nc @ b'\n') => {
                            Span::new(chr.span(self.idx).start, nc.span(self.idx + 1).end)
                        }
                        _ => chr.span(self.idx),
                    };
                    self.push_tok_with(Newline, span);
                }
                b'{' => self.push_tok_with(LeftBrace, chr.span(self.idx)),
                b'[' => self.push_tok_with(LeftBracket, chr.span(self.idx)),
                b'|' => self.push_tok_with(Pipe, chr.span(self.idx)),
                b'+' => self.push_tok_with(Plus, chr.span(self.idx)),
                b'}' => self.push_tok_with(RightBrace, chr.span(self.idx)),
                b']' => self.push_tok_with(RightBracket, chr.span(self.idx)),
                b'\'' => self.push_tok_with(SingleQuote, chr.span(self.idx)),
                b'?' => self.push_tok_with(QuestionMark, chr.span(self.idx)),
                b'\t' | b' ' => {
                    let tok = self.consume_whitespace();
                    self.push_tok(tok);
                    if let Some(b'#') = self.current {
                        self.consume_matches(|c| !c.is_linebreak());
                    }
                }
                _ => {
                    if let Token {
                        kind: Literal(slice),
                        span,
                    } = self.consume_literal()
                    {
                        self.push_tok_with(Literal(slice), span);
                    }
                }
            }
        }
        self.tokens
    }

    fn push_tok(&mut self, tok: Token<'a>) {
        self.tokens.push(tok)
    }

    /// Constructs a token and pushes it onto the token stream
    fn push_tok_with(&mut self, kind: TokenKind<'a>, span: Span) {
        self.tokens.push(Token::new(kind, span))
    }
}
