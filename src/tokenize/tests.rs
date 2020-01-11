use super::*;

#[test]
fn test_list() {
    use TokenKind::*;
    const INPUT: &str = r"[val1, val2, val3]
        - val1
        - val2
        - val3
    ";
    let mut tokenizer = Tokenizer::from_str(INPUT);
    let tokens = tokenizer.tokenize();
    // println!("{:?}", tokens);
    let expected = vec![Token { kind: LeftBracket, span: Span { start: ByteIdx(0), end: ByteIdx(1) } }, Token { kind: Literal("val1"), span: Span { start: ByteIdx(1), end: ByteIdx(5) } }, Token { kind: Comma, span: Span { start: ByteIdx(5), end: ByteIdx(6) } }, Token { kind: Literal("val2"), span: Span { start: ByteIdx(7), end: ByteIdx(11) } }, Token { kind: Comma, span: Span { start: ByteIdx(11), end: ByteIdx(12) } }, Token { kind: Literal("val3"), span: Span { start: ByteIdx(13), end: ByteIdx(17) } }, Token { kind: RightBracket, span: Span { start: ByteIdx(17), end: ByteIdx(18) } }, Token { kind: Newline, span: Span { start: ByteIdx(18), end: ByteIdx(19) } }, Token { kind: Whitespace("        "), span: Span { start: ByteIdx(19), end: ByteIdx(27) } }, Token { kind: Dash, span: Span { start: ByteIdx(27), end: ByteIdx(28) } }, Token { kind: Literal("val1"), span: Span { start: ByteIdx(29), end: ByteIdx(33) } }, Token { kind: Newline, span: Span { start: ByteIdx(33), end: ByteIdx(34) } }, Token { kind: Whitespace("        "), span: Span { start: ByteIdx(34), end: ByteIdx(42) } }, Token { kind: Dash, span: Span { start: ByteIdx(42), end: ByteIdx(43) } }, Token { kind: Literal("val2"), span: Span { start: ByteIdx(44), end: ByteIdx(48) } }, Token { kind: Newline, span: Span { start: ByteIdx(48), end: ByteIdx(49) } }, Token { kind: Whitespace("        "), span: Span { start: ByteIdx(49), end: ByteIdx(57) } }, Token { kind: Dash, span: Span { start: ByteIdx(57), end: ByteIdx(58) } }, Token { kind: Literal("val3"), span: Span { start: ByteIdx(59), end: ByteIdx(63) } }, Token { kind: Newline, span: Span { start: ByteIdx(63), end: ByteIdx(64) } }, Token { kind: Whitespace("    "), span: Span { start: ByteIdx(64), end: ByteIdx(68) } }]
    ;
    assert_eq!(expected, tokens)
}

#[test]
fn test_whitespace() {
    const INPUT: &str =
    r#"["quoted literal, with , commas,  and  whitespace" , lit]
    "#;
    let mut tokenizer = Tokenizer::from_str(INPUT);
    let tokens = tokenizer.tokenize();
    println!("{:#?}", tokens);
}

#[test]
fn test_map() {
    const INPUT: &str = 
    r"{ key1: val1, key2:val2, key3 : val3}
    key1: value1
    key2: value2

    ";
}