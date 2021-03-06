#![cfg(test)]
#![allow(clippy::pedantic)]

use crate::YamlParseError;

impl<'a> From<&'a str> for crate::Yaml<'a> {
    fn from(other: &'a str) -> Self {
        crate::Yaml::Scalar(other)
    }
}

// Scalars

mk_test!(
    double quote scalar whitespace;
    r#""a scalar value with whitespace""# => r#""a scalar value with whitespace""#
);

mk_test!(
    double quote scalar no whitespace;
    r#""a_scalarvaluewithout_whitespace""# => r#""a_scalarvaluewithout_whitespace""#
);

mk_test!(
    single quote scalar whitespace;
    r#"'a scalar value with whitespace'"# => r#"'a scalar value with whitespace'"#
);

mk_test!(
    single quote scalar no whitespace;
    r#"'ascalarvalue_without_whitespace'"# => r#"'ascalarvalue_without_whitespace'"#
);

mk_test!(
    no quote scalar whitespace;
    "an unquoted scalar value with whitespace" => "an unquoted scalar value with whitespace"
);

mk_test!(
    no quote scalar no whitespace;
    "anunquoted_scalar_value_withoutwhitespace" => "anunquoted_scalar_value_withoutwhitespace"
);

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
- yaml
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
super simple block sequence nested;
r#"
-
  - " a "
  - ' nested'
"# => seq!(seq!(r#"" a ""#, r"' nested'"))
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
super simple;
r#"
key : value
key2 : value2
"# => map! { "key" : "value", "key2" : "value2"}
);

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
scalar with pound in middle;
r#"
- foo#bar
- "baz#bax"
- 'quux#xyzzy'
"# => seq!(
        "foo",
        r##""baz#bax""##,
        r"'quux#xyzzy'"
    )
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
  - totally
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

mk_test!(
input with indicators;
r"
stuff:
    - this::thing::with::colons::and::all-these-other-indicator-characters-:used:-in--an:unquoted:::::::string

" => map! { "stuff" => seq!("this::thing::with::colons::and::all-these-other-indicator-characters-:used:-in--an:unquoted:::::::string")}
);

// Regression tests

mk_test!(issue_13a;
r"
foo:
- baz
bar: bax
" => map! { "foo" => seq!("baz"); "bar" => "bax"}
);

mk_test!(issue_13b;
r"
value: {x: -0}
" => map! { "value" => map! { "x": "-0" }}
);

mk_test!(malformed seq;
r"
- a
-b
" => fail
);

mk_test!(issue_14;
r"a: -1" => map! { "a": "-1" }
);

mk_test!(issue_15a;
r"a: foo[0]" => map! { "a": "foo[0]" }
);

mk_test!(issue_15b;
r"a: a - a" => map! { "a": "a - a"}
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
