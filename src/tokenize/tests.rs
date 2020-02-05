#![cfg(test)]
use super::*;
use TokenKind::*;

#[test]
fn test_list() {
    const INPUT: &str = r"[val1, val2, val3]
        - val1
        - val2
        - val3
    ";
    let tokenizer = Tokenizer::from_str(INPUT);
    let tokens = tokenizer.tokenize();
    let expected = vec![
        Token {
            kind: LeftBracket,
            span: Span {
                start: ByteIdx(0),
                end: ByteIdx(1),
            },
        },
        Token {
            kind: Literal("val1"),
            span: Span {
                start: ByteIdx(1),
                end: ByteIdx(5),
            },
        },
        Token {
            kind: Comma,
            span: Span {
                start: ByteIdx(5),
                end: ByteIdx(6),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(6),
                end: ByteIdx(7),
            },
        },
        Token {
            kind: Literal("val2"),
            span: Span {
                start: ByteIdx(7),
                end: ByteIdx(11),
            },
        },
        Token {
            kind: Comma,
            span: Span {
                start: ByteIdx(11),
                end: ByteIdx(12),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(12),
                end: ByteIdx(13),
            },
        },
        Token {
            kind: Literal("val3"),
            span: Span {
                start: ByteIdx(13),
                end: ByteIdx(17),
            },
        },
        Token {
            kind: RightBracket,
            span: Span {
                start: ByteIdx(17),
                end: ByteIdx(18),
            },
        },
        Token {
            kind: Newline,
            span: Span {
                start: ByteIdx(18),
                end: ByteIdx(19),
            },
        },
        Token {
            kind: Whitespace(8),
            span: Span {
                start: ByteIdx(19),
                end: ByteIdx(27),
            },
        },
        Token {
            kind: Dash,
            span: Span {
                start: ByteIdx(27),
                end: ByteIdx(28),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(28),
                end: ByteIdx(29),
            },
        },
        Token {
            kind: Literal("val1"),
            span: Span {
                start: ByteIdx(29),
                end: ByteIdx(33),
            },
        },
        Token {
            kind: Newline,
            span: Span {
                start: ByteIdx(33),
                end: ByteIdx(34),
            },
        },
        Token {
            kind: Whitespace(8),
            span: Span {
                start: ByteIdx(34),
                end: ByteIdx(42),
            },
        },
        Token {
            kind: Dash,
            span: Span {
                start: ByteIdx(42),
                end: ByteIdx(43),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(43),
                end: ByteIdx(44),
            },
        },
        Token {
            kind: Literal("val2"),
            span: Span {
                start: ByteIdx(44),
                end: ByteIdx(48),
            },
        },
        Token {
            kind: Newline,
            span: Span {
                start: ByteIdx(48),
                end: ByteIdx(49),
            },
        },
        Token {
            kind: Whitespace(8),
            span: Span {
                start: ByteIdx(49),
                end: ByteIdx(57),
            },
        },
        Token {
            kind: Dash,
            span: Span {
                start: ByteIdx(57),
                end: ByteIdx(58),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(58),
                end: ByteIdx(59),
            },
        },
        Token {
            kind: Literal("val3"),
            span: Span {
                start: ByteIdx(59),
                end: ByteIdx(63),
            },
        },
        Token {
            kind: Newline,
            span: Span {
                start: ByteIdx(63),
                end: ByteIdx(64),
            },
        },
        Token {
            kind: Whitespace(4),
            span: Span {
                start: ByteIdx(64),
                end: ByteIdx(68),
            },
        },
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_whitespace() {
    const INPUT: &str = r#"["quoted literal, with , commas,  and  whitespace" , lit]"#;
    let tokenizer = Tokenizer::from_str(INPUT);
    let tokens = tokenizer.tokenize();
    let expected = vec![
        Token {
            kind: LeftBracket,
            span: Span {
                start: ByteIdx(0),
                end: ByteIdx(1),
            },
        },
        Token {
            kind: DoubleQuote,
            span: Span {
                start: ByteIdx(1),
                end: ByteIdx(2),
            },
        },
        Token {
            kind: Literal("quoted"),
            span: Span {
                start: ByteIdx(2),
                end: ByteIdx(8),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(8),
                end: ByteIdx(9),
            },
        },
        Token {
            kind: Literal("literal"),
            span: Span {
                start: ByteIdx(9),
                end: ByteIdx(16),
            },
        },
        Token {
            kind: Comma,
            span: Span {
                start: ByteIdx(16),
                end: ByteIdx(17),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(17),
                end: ByteIdx(18),
            },
        },
        Token {
            kind: Literal("with"),
            span: Span {
                start: ByteIdx(18),
                end: ByteIdx(22),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(22),
                end: ByteIdx(23),
            },
        },
        Token {
            kind: Comma,
            span: Span {
                start: ByteIdx(23),
                end: ByteIdx(24),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(24),
                end: ByteIdx(25),
            },
        },
        Token {
            kind: Literal("commas"),
            span: Span {
                start: ByteIdx(25),
                end: ByteIdx(31),
            },
        },
        Token {
            kind: Comma,
            span: Span {
                start: ByteIdx(31),
                end: ByteIdx(32),
            },
        },
        Token {
            kind: Whitespace(2),
            span: Span {
                start: ByteIdx(32),
                end: ByteIdx(34),
            },
        },
        Token {
            kind: Literal("and"),
            span: Span {
                start: ByteIdx(34),
                end: ByteIdx(37),
            },
        },
        Token {
            kind: Whitespace(2),
            span: Span {
                start: ByteIdx(37),
                end: ByteIdx(39),
            },
        },
        Token {
            kind: Literal("whitespace"),
            span: Span {
                start: ByteIdx(39),
                end: ByteIdx(49),
            },
        },
        Token {
            kind: DoubleQuote,
            span: Span {
                start: ByteIdx(49),
                end: ByteIdx(50),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(50),
                end: ByteIdx(51),
            },
        },
        Token {
            kind: Comma,
            span: Span {
                start: ByteIdx(51),
                end: ByteIdx(52),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(52),
                end: ByteIdx(53),
            },
        },
        Token {
            kind: Literal("lit"),
            span: Span {
                start: ByteIdx(53),
                end: ByteIdx(56),
            },
        },
        Token {
            kind: RightBracket,
            span: Span {
                start: ByteIdx(56),
                end: ByteIdx(57),
            },
        },
    ];
    assert_eq!(expected, tokens);
}

#[test]
fn test_map() {
    const INPUT: &str = r"{ key1: val1, key2:val2, key3 : val3}
    key1: value1
    key2: value2
    key3: value3
    ";
    let tokenizer = Tokenizer::from_str(INPUT);
    let tokens = tokenizer.tokenize();
    let expected = vec![
        Token {
            kind: LeftBrace,
            span: Span {
                start: ByteIdx(0),
                end: ByteIdx(1),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(1),
                end: ByteIdx(2),
            },
        },
        Token {
            kind: Literal("key1"),
            span: Span {
                start: ByteIdx(2),
                end: ByteIdx(6),
            },
        },
        Token {
            kind: Colon,
            span: Span {
                start: ByteIdx(6),
                end: ByteIdx(7),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(7),
                end: ByteIdx(8),
            },
        },
        Token {
            kind: Literal("val1"),
            span: Span {
                start: ByteIdx(8),
                end: ByteIdx(12),
            },
        },
        Token {
            kind: Comma,
            span: Span {
                start: ByteIdx(12),
                end: ByteIdx(13),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(13),
                end: ByteIdx(14),
            },
        },
        Token {
            kind: Literal("key2"),
            span: Span {
                start: ByteIdx(14),
                end: ByteIdx(18),
            },
        },
        Token {
            kind: Colon,
            span: Span {
                start: ByteIdx(18),
                end: ByteIdx(19),
            },
        },
        Token {
            kind: Literal("val2"),
            span: Span {
                start: ByteIdx(19),
                end: ByteIdx(23),
            },
        },
        Token {
            kind: Comma,
            span: Span {
                start: ByteIdx(23),
                end: ByteIdx(24),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(24),
                end: ByteIdx(25),
            },
        },
        Token {
            kind: Literal("key3"),
            span: Span {
                start: ByteIdx(25),
                end: ByteIdx(29),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(29),
                end: ByteIdx(30),
            },
        },
        Token {
            kind: Colon,
            span: Span {
                start: ByteIdx(30),
                end: ByteIdx(31),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(31),
                end: ByteIdx(32),
            },
        },
        Token {
            kind: Literal("val3"),
            span: Span {
                start: ByteIdx(32),
                end: ByteIdx(36),
            },
        },
        Token {
            kind: RightBrace,
            span: Span {
                start: ByteIdx(36),
                end: ByteIdx(37),
            },
        },
        Token {
            kind: Newline,
            span: Span {
                start: ByteIdx(37),
                end: ByteIdx(38),
            },
        },
        Token {
            kind: Whitespace(4),
            span: Span {
                start: ByteIdx(38),
                end: ByteIdx(42),
            },
        },
        Token {
            kind: Literal("key1"),
            span: Span {
                start: ByteIdx(42),
                end: ByteIdx(46),
            },
        },
        Token {
            kind: Colon,
            span: Span {
                start: ByteIdx(46),
                end: ByteIdx(47),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(47),
                end: ByteIdx(48),
            },
        },
        Token {
            kind: Literal("value1"),
            span: Span {
                start: ByteIdx(48),
                end: ByteIdx(54),
            },
        },
        Token {
            kind: Newline,
            span: Span {
                start: ByteIdx(54),
                end: ByteIdx(55),
            },
        },
        Token {
            kind: Whitespace(4),
            span: Span {
                start: ByteIdx(55),
                end: ByteIdx(59),
            },
        },
        Token {
            kind: Literal("key2"),
            span: Span {
                start: ByteIdx(59),
                end: ByteIdx(63),
            },
        },
        Token {
            kind: Colon,
            span: Span {
                start: ByteIdx(63),
                end: ByteIdx(64),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(64),
                end: ByteIdx(65),
            },
        },
        Token {
            kind: Literal("value2"),
            span: Span {
                start: ByteIdx(65),
                end: ByteIdx(71),
            },
        },
        Token {
            kind: Newline,
            span: Span {
                start: ByteIdx(71),
                end: ByteIdx(72),
            },
        },
        Token {
            kind: Whitespace(4),
            span: Span {
                start: ByteIdx(72),
                end: ByteIdx(76),
            },
        },
        Token {
            kind: Literal("key3"),
            span: Span {
                start: ByteIdx(76),
                end: ByteIdx(80),
            },
        },
        Token {
            kind: Colon,
            span: Span {
                start: ByteIdx(80),
                end: ByteIdx(81),
            },
        },
        Token {
            kind: Whitespace(1),
            span: Span {
                start: ByteIdx(81),
                end: ByteIdx(82),
            },
        },
        Token {
            kind: Literal("value3"),
            span: Span {
                start: ByteIdx(82),
                end: ByteIdx(88),
            },
        },
        Token {
            kind: Newline,
            span: Span {
                start: ByteIdx(88),
                end: ByteIdx(89),
            },
        },
        Token {
            kind: Whitespace(4),
            span: Span {
                start: ByteIdx(89),
                end: ByteIdx(93),
            },
        },
    ];
    assert_eq!(expected, tokens);
}
