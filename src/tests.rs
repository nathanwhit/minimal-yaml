#![cfg(test)]

use crate::{parse, Yaml::{Scalar, Sequence}};

macro_rules! mk_test {
    ($($name: ident) +; $inp: expr => $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                const INPUT: &str = $inp;
                assert_eq!(parse(INPUT).unwrap(), $exp);
            }
        }
    };
}
}

mk_test!(
    double quote scalar whitespace;
    r#""a scalar value with whitespace""# => Scalar("a scalar value with whitespace")
);

mk_test!(
    double quote scalar no whitespace;
    r#""a_scalarvaluewithout_whitespace""# => Scalar("a_scalarvaluewithout_whitespace")
);

mk_test!(
    single quote scalar whitespace;
    r#"'a scalar value with whitespace'"# => Scalar("a scalar value with whitespace")
);

mk_test!(
    single quote scalar no whitespace;
    r#"'ascalarvalue_without_whitespace'"# => Scalar("ascalarvalue_without_whitespace")
);

mk_test!(
    no quote scalar whitespace;
    "an unquoted scalar value with whitespace" => Scalar("an unquoted scalar value with whitespace")
);

mk_test!(
    no quote scalar no whitespace;
    "anunquoted_scalar_value_withoutwhitespace" => Scalar("anunquoted_scalar_value_withoutwhitespace")
);
