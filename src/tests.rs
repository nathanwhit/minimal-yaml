#![cfg(test)]
#![allow(clippy::pedantic)]

use crate::{
    parse, Entry, Yaml,
    Yaml::{Mapping, Scalar, Sequence},
    YamlParseError,
};

impl<'a> From<&'a str> for Yaml<'a> {
    fn from(other: &'a str) -> Self {
        Yaml::Scalar(other)
    }
}

macro_rules! mk_test {
    ($($name: ident) +; $inp: expr => $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                assert_eq!(parse(input).unwrap(), $exp);
            }
        }
    };
    ($($name: ident) +; $inp: expr => err $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                assert_eq!(parse(input).unwrap_err(), $exp);
            }
        }
    };
    ($($name: ident) +; $inp: expr => matches bytes $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &[u8] = $inp;
                assert!(
                    match try_parse_from_utf8(input) {
                        $exp => true,
                        _ => false,
                    }
                );
            }
        }
    };
    ($($name: ident) +; $inp: expr => matches $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                assert!(
                    match parse(input) {
                        $exp => true,
                        _ => false,
                    }
                );
            }
        }
    };
    ($($name: ident) +; $inp: expr => err msg $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                assert_eq!(parse(input).unwrap_err().to_string(), $exp);
            }
        }
    };
}

// Scalars

mk_test!(
    double quote scalar whitespace;
    r#""a scalar value with whitespace""# => Scalar(r#""a scalar value with whitespace""#)
);

mk_test!(
    double quote scalar no whitespace;
    r#""a_scalarvaluewithout_whitespace""# => Scalar(r#""a_scalarvaluewithout_whitespace""#)
);

mk_test!(
    single quote scalar whitespace;
    r#"'a scalar value with whitespace'"# => Scalar(r#"'a scalar value with whitespace'"#)
);

mk_test!(
    single quote scalar no whitespace;
    r#"'ascalarvalue_without_whitespace'"# => Scalar(r#"'ascalarvalue_without_whitespace'"#)
);

mk_test!(
    no quote scalar whitespace;
    "an unquoted scalar value with whitespace" => Scalar("an unquoted scalar value with whitespace")
);

mk_test!(
    no quote scalar no whitespace;
    "anunquoted_scalar_value_withoutwhitespace" => Scalar("anunquoted_scalar_value_withoutwhitespace")
);

macro_rules! seq {
    ($($val: expr),*) => {
        Sequence(vec![$( $val.into() ),*])
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
    r#"[ "a", 'b' , "  c ", d, ' e  ' ]"# => seq!(r#""a""#, r#"'b'"#, r#""  c ""# , "d", r#"' e  '"#)
);

mk_test!(
    multiple flow sequence no quotes;
    r"[[ a, b, c ,d, e   ] , [ f, g, h, i , j ]]" =>
        seq!(
            seq!("a", "b","c", "d", "e"),
            seq!("f", "g", "h", "i", "j")
        )
);

mk_test!(
    mixed kind flow sequence no quotes;
    r"[[ a, b, c], el]" => seq!(seq!("a", "b", "c"), "el")
);

mk_test!(
    mixed kind flow sequence quotes;
    r#"[" elem " , [ a, 'b ' , "   c "]]"# => seq!(r#"" elem ""#, seq!("a", r"'b '", r#""   c ""#))
);

// Macro

macro_rules! map {
    { $($key : tt : $val : tt),* } => {
        Mapping(vec![$(Entry { key: Scalar($key) , value: Scalar($val) }),*])
    };
    { $($key : expr => $val : expr);* } => {
        Mapping(vec![$(Entry { key: $key.into() , value: $val.into() }),*])
    }
}

// Flow mappings

mk_test!(
    simple flow mapping;
    r"{ k : v }" => map!{ "k" : "v" }
);

mk_test!(
    multiple entry flow mapping;
    r"{ k1 : v1 ,   k2 :     v2    }" => map!{ "k1" : "v1", "k2":"v2" }
);

mk_test!(
    seq value flow mapping;
    r"{ k1 : [ a , b, c] }" => map! {
        "k1" => seq!("a", "b", "c")
    }
);

mk_test!(
    seq key flow mapping;
    r"{ [ a, map, as a key ] : val }" => map! {
        seq!("a", "map", "as a key") => "val"
    }
);

mk_test!(
    seq entry flow mapping;
    r"{ [ a, seq, as a key ] : [  a, seq, as a value ]  }" => map! {
        seq!("a", "seq", "as a key") => seq!("a", "seq", "as a value")
    }
);

mk_test!(
    map key flow mapping;
    r"{ { a map : as a key} : value }" => map! {
        map! { "a map" : "as a key" } => "value"
    }
);

mk_test!(
    map entry flow mapping;
    r"{ { a   map : as a key} : { 'a map ': as a value }   }" => map! {
        map! { "a   map" : "as a key" } => map! { r"'a map '" : "as a value" }
    }
);

// Block Sequence

mk_test!(
simple block sequence;
r#"
- a
- sequence
- of
-yaml
-   nodes
- "in"
- 'block'
- ' form '"# => seq!("a", "sequence", "of", "yaml", "nodes", r#""in""#, r"'block'", r"' form '") 
);

mk_test!(
block sequence flow seq;
r#"
- a
- sequence
- with
-       [ a, sequence, "as ", 'a', node  ]
"# => seq!("a", "sequence", "with", seq!("a", "sequence", r#""as ""#, "'a'", "node"))
);

mk_test!(
block sequence flow map;
r#"
- a
- block
- sequence
- '  "with" '
- { a : "flow", mapping : ' as ', a : " 'node' "}
"# => seq!("a", "block", "sequence", r#"'  "with" '"#, map!{ "a" : r#""flow""#, "mapping" : r"' as '", "a" : r#"" 'node' ""#})
);

mk_test!(
block sequence nested;
r#"
-
  - " a "
  - ' nested'
  - ' " block  " '
  - sequence
-
  - with
  - two
  - "'e l e m e n t s'"
"# => seq!(seq!(r#"" a ""#, r"' nested'", r#"' " block  " '"# ,"sequence"), seq!("with", "two", r#""'e l e m e n t s'""#))
);

mk_test!(
block sequence multiple nested;
r##"
-
    - "a"
    - "nested"
    - block
    -
        - sequence
        - with
    - lots
    -
        - of
        - different
-   
    - indent
    - levels
    -
        - [with, a, flow, sequence for good measure]
- "' the '"
- end
"## => 
    seq!(
        seq!(
            r#""a""#,
            r#""nested""#,
            "block",
            seq!(
                "sequence",
                "with"
            ),
            "lots",
            seq!(
                "of",
                "different"
            )
        ),
        seq!(
            "indent",
            "levels",
            seq!(
                seq!(
                    "with",
                    "a",
                    "flow",
                    "sequence for good measure"
                )
            )
        ),
        r#""' the '""#,
        "end"
    )
);

// Block mappings

mk_test!(
block mapping simple;
r#"
key : value
key2 : value2
and : another
now with : "some quotes"
'and' : "a 'few' more"
"# => map!{ "key" : "value", "key2" : "value2", "and" : "another", "now with" : r#""some quotes""#, r"'and'" : r#""a 'few' more""#}
);

mk_test!(
block mapping flow;
r#"
key : {the : " value ", 'i s' : a, flow: mapping}
mind : blown
wait : [it, works, with, flow, sequences too]
[now, how, about, one, with, the, flow, mapping, as] : a key
"# => map!{
    "key" => map!{ "the" : r#"" value ""#, r"'i s'": "a", "flow":"mapping"};
    "mind" => "blown";
    "wait" => seq!("it", "works", "with", "flow", "sequences too");
    seq!("now", "how", "about", "one", "with", "the", "flow", "mapping", "as") => "a key"
}
);

mk_test!(
block mapping nested blocks;
r#"
key : 
  the : value
  is : 
    nested : mappings
    wow : 
      - with a block seq
      - too
and : done
"# => map!{
    "key" => map! {
        "the" => "value";
        "is" => map! {
            "nested" => "mappings";
            "wow" => seq!("with a block seq", "too")
        }
    };
    "and" => "done"
}
);

// Misc

mk_test!(
input with comments;
r#"
key: #comment 1
   - value line 1
   #comment 2
   - value line 2
   #comment 3
   - value line 3
"# => map!{
    "key" => seq!(
        "value line 1",
        "value line 2",
        "value line 3"
    )
}
);

mk_test!(
input with error;
r#"
{key: value, missing : }
"# => err YamlParseError{ line: 2, col: 25, msg: Some(String::from(r#"unexpected symbol '}'"#)), source: None }
);

mk_test!(
error msg;
r#"
{key: value, missing : }
"# => err msg r#"error occurred parsing the input at line 2, column 25 : unexpected symbol '}'"#
);

mk_test!(
infinite loop;
std::str::from_utf8(&[45, 49, 58, 91, 32]).unwrap() => err YamlParseError{ line: 1, col: 6, msg: Some(String::from(r#"unexpected end of input"#)), source: None }
);

mk_test!(
input with doc start;
r"
---
- this
- is
- a
- 
  sequence : of
  values : in
  a : yaml file
" => seq!(
    "this", "is", "a",
    map! {
        "sequence" : "of", "values" : "in", "a" : "yaml file"
    }
)
);

mk_test!(
input with seq and dash;
r"
---
- this
- is
- a
- valid
- minimal-yaml
- sequence
" => seq!("this", "is", "a", "valid", "minimal-yaml", "sequence")
);

mk_test!(
odd structure;
r"
this is :
 - totally
 - valid
 - input : to the parser
" => map!{ "this is" => seq!("totally", "valid", map!{ "input": "to the parser"})  }
);

mk_test!(
readme example;
r"
[this, is] :
 -
  -totally
  - valid
 - input
 - {to : the parser}
 " => map!{ seq!("this", "is") => seq!( seq!("totally","valid"), "input", map!{"to":"the parser"})}
);

mk_test!(
block mapping missing value;
r"
a : block
mapping : missing
a value for this key:

" => err YamlParseError { line: 5, col: 1, msg: Some("unexpected end of input".into()), source: None}
);

// Round trip

#[test]
fn test_round_trip_basic_literal_eq() {
    let input = r#"foo : bar
baz :
  - qux
  - quux
  -
    corge : grault
    garply : waldo
      - fred
      - plugh
      - xyzzy
    : thud
"#;
    assert_eq!(
        crate::parse(input).unwrap().to_string(),
        String::from(input)
    )
}

#[test]
fn test_round_trip_basic_structural_eq() {
    let input = r#"
key : 
  the : value
  is : 
    nested : mappings
    wow : 
      - with a block seq
      - too
and : done
"#;
    assert_eq!(
        crate::parse(&crate::parse(input).unwrap().to_string()).unwrap(),
        map! {
            "key" => map! {
                "the" => "value";
                "is" => map! {
                    "nested" => "mappings";
                    "wow" => seq!("with a block seq", "too")
                }
            };
            "and" => "done"
        }
    )
}

testgen_minimal_yaml::yaml_test_gen!("./yaml-test-suite");
