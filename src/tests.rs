#[cfg(test)]
use crate::{parse, Result, Yaml::{Scalar}};

#[test]
fn test_parse_double_quote_scalar_w_ws() {
    const INPUT: &str = r#""a scalar value with whitespace""#;
    let res = parse(INPUT).unwrap();
    assert_eq!(res, Scalar("a scalar value with whitespace"))
}


#[test]
fn test_parse_double_quote_scalar_wo_ws() -> Result<()> {
    const INPUT: &str = r#""a_scalarvaluewithout_whitespace""#;
    assert_eq!(parse(INPUT).unwrap(), Scalar("a_scalarvaluewithout_whitespace"));
    Ok(())
}

#[test]
fn test_parse_single_quote_scalar_w_ws() {
    const INPUT: &str = r#"'a scalar value with whitespace'"#;
    assert_eq!(parse(INPUT).unwrap(), Scalar("a scalar value with whitespace"))
}

#[test]
fn test_parse_single_quote_scalar_wo_ws() {
    const INPUT: &str = r#"'ascalarvalue_without_whitespace'"#;
    assert_eq!(parse(INPUT).unwrap(), Scalar("ascalarvalue_without_whitespace"))

}

#[test]
fn test_parse_no_quote_scalar_w_ws() {
    const INPUT: &str = r#"an unquoted scalar value with whitespace"#;
    assert_eq!(parse(INPUT).unwrap(), Scalar("an unquoted scalar value with whitespace"))
}

#[test]
fn test_parse_no_quote_scalar_wo_ws() {
    const INPUT: &str = r#"anunquoted_scalar_value_withoutwhitespace"#;
    assert_eq!(parse(INPUT).unwrap(), Scalar("anunquoted_scalar_value_withoutwhitespace"))
}