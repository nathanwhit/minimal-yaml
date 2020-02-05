use crate::tokenize::{Token, TokenKind};
use crate::{Entry, MiniYamlError, Yaml};
use core::iter::{Enumerate, Iterator, Peekable};
use core::slice::Iter;

use crate::Result;

// Implementation lifted from std, as it's currently only on Nightly. It's such a simple macro that it's low risk to duplicate it here (and better than writing one myself)
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    }
}

pub(crate) struct Parser<'a, 'b> {
    token: &'b Token<'a>,
    stream: Peekable<Enumerate<Iter<'b, Token<'a>>>>,
    tok_stream: &'b [Token<'a>],
    source: &'a str,
    tok_idx: usize,
    indent: usize,
    expected: Vec<TokenKind<'a>>,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub(crate) fn new(source: &'a str, tok_stream: &'b [Token<'a>]) -> Self {
        let mut stream = tok_stream.iter().enumerate().peekable();
        let first = stream.next().unwrap();
        Self {
            token: &first.1,
            stream,
            tok_stream,
            source,
            tok_idx: first.0,
            indent: 0,
            expected: Vec::new(),
        }
    }

    fn bump(&mut self) -> bool {
        match self.stream.next() {
            Some(tok) => {
                self.tok_idx = tok.0;
                self.token = tok.1;
                true
            }
            None => false,
        }
    }

    fn peek(&mut self) -> Option<&Token<'a>> {
        self.stream.peek().map(|t| t.1)
    }

    pub(crate) fn parse(&mut self) -> Result<Yaml<'a>> {
        use TokenKind::*;
        let res = match self.token.kind {
            DoubleQuote | SingleQuote | Literal(..) => {
                let node = self.parse_scalar()?;
                self.chomp_whitespace();
                match self.token.kind {
                    Colon
                        if match self.expected.last() {
                            Some(RightBrace) | Some(Colon) => false,
                            _ => true,
                        } =>
                    {
                        self.parse_mapping_block(node)?
                    }
                    _ => node,
                }
            }
            LeftBrace => {
                self.expected.push(RightBrace);
                let res = self.parse_mapping_flow()?;
                match self.expected.last() {
                    Some(RightBrace) => {
                        self.pop_if_match(RightBrace)?;
                    }
                    _ => (),
                }
                res
            }
            LeftBracket => self.parse_sequence_flow()?,
            Dash => match self.peek() {
                Some(Token { kind: Dash, .. }) => {
                    if self.check_ahead_n(2, |tk| matches!(tk, Dash)) {
                        self.bump();
                        self.bump();
                        self.bump();
                        self.parse()?
                    } else {
                        return self.parse_error();
                    }
                }
                _ => self.parse_sequence_block()?,
            },
            RightBrace | RightBracket => return Err(MiniYamlError::ParseError),
            Whitespace(amt) => {
                self.indent = amt;
                self.bump();
                self.parse()?
            }
            Newline => {
                self.indent = 0;
                self.bump();
                self.parse()?
            }
            _ => return self.parse_error(),
        };
        Ok(res)
    }

    pub(crate) fn parse_scalar(&mut self) -> Result<Yaml<'a>> {
        use TakeUntilCond::*;
        use TokenKind::*;
        match self.token.kind {
            // TODO: currently qouble quote/single quote scalars are handled identically. maybe handle as defined
            // by the YAML spec?
            DoubleQuote => {
                self.bump();
                let tok_range = self.take_until(MatchOrErr, |tok, _| matches!(tok, DoubleQuote))?;
                debug_assert!(matches!(self.token.kind, DoubleQuote));
                self.bump();
                let entire_literal = self.slice_tok_range(tok_range);
                Ok(Yaml::Scalar(entire_literal))
            }
            SingleQuote => {
                self.bump();
                let tok_range = self.take_until(MatchOrErr, |tok, _| matches!(tok, SingleQuote))?;
                debug_assert!(matches!(self.token.kind, SingleQuote));
                self.bump();
                let entire_literal = self.slice_tok_range(tok_range);
                Ok(Yaml::Scalar(entire_literal))
            }
            Literal(..) => {
                let stop = |tok: &TokenKind<'_>| {
                    matches!(tok, Comma | Colon | RightBrace | RightBracket | Newline)
                };
                let tok_range = self.take_until(MatchOrEnd, |tok, nxt| {
                    stop(tok) || (matches!(tok, Whitespace(..)) && stop(nxt))
                })?;
                let entire_literal = self.slice_tok_range(tok_range);
                Ok(Yaml::Scalar(entire_literal))
            }
            _ => self.parse_error(),
        }
    }

    fn parse_error<T>(&self) -> Result<T> {
        Err(MiniYamlError::ParseError)
    }

    pub(crate) fn parse_mapping_flow(&mut self) -> Result<Yaml<'a>> {
        use TokenKind::*;
        self.bump();
        let mut entries: Vec<Entry<'a>> = Vec::new();
        loop {
            match self.token.kind {
                RightBrace => {
                    self.bump();
                    return Ok(Yaml::Mapping(entries));
                }
                Comma => {
                    self.bump();
                }
                _ => {
                    self.expected.push(Colon);
                    let key = self.parse()?;
                    self.chomp_whitespace();
                    match self.token.kind {
                        Colon => {
                            self.pop_if_match(Colon)?;
                            self.bump();
                            self.chomp_whitespace();
                            let value = self.parse()?;
                            self.chomp_whitespace();
                            entries.push(Entry { key, value })
                        }
                        _ => return self.parse_error(),
                    }
                }
            }
        }
    }

    pub(crate) fn parse_mapping_block(&mut self, start_key: Yaml<'a>) -> Result<Yaml<'a>> {
        use TokenKind::*;
        let indent = self.indent;
        match self.token.kind {
            Colon => {
                self.bump();
                let mut entries = Vec::new();
                self.chomp_whitespace();
                let value = self.parse()?;
                entries.push(Entry::new(start_key, value));
                loop {
                    match self.token.kind {
                        Newline => {
                            self.indent = 0;
                            if self.bump() {
                                continue;
                            } else {
                                break;
                            }
                        }
                        Whitespace(idt) => {
                            self.bump();
                            self.indent = idt;
                        }
                        _ if self.indent < indent => break,
                        _ => {
                            self.expected.push(Colon);
                            let key = self.parse()?;
                            self.chomp_whitespace();
                            if let Colon = self.token.kind {
                                self.pop_if_match(Colon)?;
                                self.bump();
                                self.chomp_whitespace();
                                let value = self.parse()?;
                                entries.push(Entry::new(key, value));
                            } else {
                                return self.parse_error();
                            }
                        }
                    }
                }
                Ok(Yaml::Mapping(entries))
            }
            _ => self.parse_error(),
        }
    }

    fn slice_tok_range(&self, range: (usize, usize)) -> &'a str {
        let start = self.tok_stream[range.0].start();
        let end = match self.tok_stream.get(range.1) {
            Some(tok) => tok.start(),
            None => self.tok_stream.last().unwrap().end(),
        };
        &self.source[start.into()..end.into()]
    }

    fn chomp_whitespace(&mut self) {
        while let TokenKind::Whitespace(..) = self.token.kind {
            self.bump();
        }
    }

    pub(crate) fn parse_sequence_flow(&mut self) -> Result<Yaml<'a>> {
        use TokenKind::*;
        match self.token.kind {
            LeftBracket => {
                self.bump();
                let mut elements = Vec::new();
                loop {
                    match self.token.kind {
                        RightBracket => {
                            self.bump();
                            return Ok(Yaml::Sequence(elements));
                        }
                        Whitespace(..) => {
                            self.bump();
                        }
                        _ => {
                            let elem = self.parse()?;
                            elements.push(elem);
                            self.chomp_whitespace();
                            match self.token.kind {
                                Comma => {
                                    self.bump();
                                    continue;
                                }
                                RightBracket => {
                                    self.bump();
                                    return Ok(Yaml::Sequence(elements));
                                }
                                _ => return Err(MiniYamlError::ParseError),
                            }
                        }
                    }
                }
            }
            _ => self.parse_error(),
        }
    }

    fn check_ahead_1(&mut self, stop: impl Fn(&TokenKind<'a>) -> bool) -> bool {
        match self.peek() {
            Some(tok) => stop(&tok.kind),
            None => false,
        }
    }

    pub(crate) fn parse_sequence_block(&mut self) -> Result<Yaml<'a>> {
        use TokenKind::*;
        let indent = self.indent;
        match self.token.kind {
            Dash => {
                let mut seq = Vec::new();
                loop {
                    match self.token.kind {
                        Newline => {
                            self.indent = 0;
                            if self.bump() {
                                continue;
                            } else {
                                break;
                            }
                        }
                        Whitespace(idt) => {
                            self.bump();
                            self.indent = idt;
                        }
                        _ if self.indent < indent => break,
                        Dash => {
                            if self.check_ahead_1(|t| matches!(t, Newline)) {
                                self.bump();
                                self.bump();
                                self.indent = 0;
                                if let Whitespace(idt) = self.token.kind {
                                    if idt < indent {
                                        break;
                                    } else {
                                        let node = self.parse()?;
                                        seq.push(node);
                                    }
                                } else if 0 < indent {
                                    break;
                                } else {
                                    let node = self.parse()?;
                                    seq.push(node);
                                }
                            } else {
                                self.bump();
                                let node = self.parse()?;
                                seq.push(node);
                            }
                        }
                        _ => break,
                    }
                }
                Ok(Yaml::Sequence(seq))
            }
            _ => self.parse_error(),
        }
    }

    fn check_ahead_n(&self, n: usize, stop: impl Fn(&TokenKind<'a>) -> bool) -> bool {
        match self.tok_stream.get(self.tok_idx + n) {
            Some(Token { kind: tok_kind, .. }) => stop(tok_kind),
            None => false,
        }
    }

    fn peekahead_n(&self, n: usize) -> Option<&TokenKind<'a>> {
        match self.tok_stream.get(self.tok_idx + n) {
            Some(Token { kind: tok_kind, .. }) => Some(tok_kind),
            None => None,
        }
    }

    fn take_until(
        &mut self,
        cond: TakeUntilCond,
        stop: impl Fn(&TokenKind<'a>, &TokenKind<'a>) -> bool,
    ) -> Result<(usize, usize)> {
        let start = self.tok_idx;
        let mut end = start;
        loop {
            let peeked = self.peekahead_n(1);
            if match peeked {
                Some(tok_kind) => stop(&self.token.kind, tok_kind),
                None => stop(&self.token.kind, &TokenKind::default()),
            } {
                break;
            } else if !self.bump() {
                return match cond {
                    TakeUntilCond::MatchOrEnd => Ok((start, self.tok_stream.len())),
                    TakeUntilCond::MatchOrErr => self.parse_error(),
                };
            }
            end += 1;
        }
        Ok((start, end))
    }

    fn pop_if_match(&mut self, expect: TokenKind<'a>) -> Result<()> {
        match self.expected.last() {
            Some(tk) if tk == &expect => {
                self.expected.pop();
                Ok(())
            }
            _ => self.parse_error(),
        }
    }
}

enum TakeUntilCond {
    MatchOrEnd,
    MatchOrErr,
}
