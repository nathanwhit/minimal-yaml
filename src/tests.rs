#![cfg(test)]

use crate::{
    parse,
    Yaml::{Scalar, Sequence},
};

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

// Scalars

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

use Scalar as S;

macro_rules! seq {
    ($($val: expr),*) => {
        Sequence(vec![$( Scalar($val )),*])
    };

    ($($val: expr);*) => {
        Sequence(vec![$( $val ),*])
    }
}

// Flow Sequences

mk_test!(
    empty flow sequence;
    r"[   ]" => seq!()
);

mk_test!(
    one element flow sequence;
    r"[   element]" => seq!("element")
);

mk_test!(
    simple flow sequence no quotes;
    r"[      a,      b     , c  ,d , e       ]" => seq!("a", "b", "c", "d", "e")
);

mk_test!(
    simple flow sequence mixed quotes;
    r#"[ "a", 'b' , "  c ", d, ' e  ' ]"# => seq!("a", "b", "  c ", "d", " e  ")
);

mk_test!(
    multiple flow sequence no quotes;
    r"[[ a, b, c ,d, e   ] , [ f, g, h, i , j ]]" => 
        seq!(
            seq!("a", "b","c", "d", "e");
            seq!("f", "g", "h", "i", "j")
        )
);

mk_test!(
    mixed kind flow sequence no quotes;
    r"[[ a, b, c], el]" => seq!(seq!("a", "b", "c"); S("el"))
);

mk_test!(
    mixed kind flow sequence quotes;
    r#"[" elem " , [ a, 'b ' , "   c "]]"# => seq!(S(" elem "); seq!("a", "b ", "   c "))
);
