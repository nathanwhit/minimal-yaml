#[allow(non_snake_case, unused)] mod yaml_test_suite
{
    #[test] fn test_spec_example_7_7_single_quoted_characters_4GC6()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4GC6/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8,
         104u8, 101u8, 114u8, 101u8, 39u8, 115u8, 32u8, 116u8, 111u8, 32u8,
         34u8, 113u8, 117u8, 111u8, 116u8, 101u8, 115u8, 34u8, 39u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_mixed_block_mapping_explicit_to_implicit_GH63()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/GH63/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 51u8, 0u8,
         0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 46u8,
         53u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         100u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_2_11_mapping_between_sequences_M5DY()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/M5DY/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 68u8, 101u8, 116u8, 114u8,
         111u8, 105u8, 116u8, 32u8, 84u8, 105u8, 103u8, 101u8, 114u8, 115u8,
         0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 67u8,
         104u8, 105u8, 99u8, 97u8, 103u8, 111u8, 32u8, 99u8, 117u8, 98u8,
         115u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 50u8,
         48u8, 48u8, 49u8, 45u8, 48u8, 55u8, 45u8, 50u8, 51u8, 1u8, 0u8, 0u8,
         0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 78u8, 101u8, 119u8, 32u8,
         89u8, 111u8, 114u8, 107u8, 32u8, 89u8, 97u8, 110u8, 107u8, 101u8,
         101u8, 115u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 65u8, 116u8, 108u8, 97u8, 110u8, 116u8, 97u8, 32u8, 66u8, 114u8,
         97u8, 118u8, 101u8, 115u8, 1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 50u8, 48u8, 48u8, 49u8, 45u8, 48u8, 55u8, 45u8, 48u8,
         50u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         50u8, 48u8, 48u8, 49u8, 45u8, 48u8, 56u8, 45u8, 49u8, 50u8, 0u8, 0u8,
         0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 50u8, 48u8, 48u8,
         49u8, 45u8, 48u8, 56u8, 45u8, 49u8, 52u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_escaped_slash_in_double_quotes_3UYS()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/3UYS/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 115u8,
         99u8, 97u8, 112u8, 101u8, 100u8, 32u8, 115u8, 108u8, 97u8, 115u8,
         104u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         34u8, 97u8, 47u8, 98u8, 34u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_16_indentation_determines_scope_HMK4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/HMK4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 97u8, 109u8,
         101u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         77u8, 97u8, 114u8, 107u8, 32u8, 77u8, 99u8, 71u8, 119u8, 105u8,
         114u8, 101u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 97u8, 99u8, 99u8, 111u8, 109u8, 112u8, 108u8, 105u8, 115u8,
         104u8, 109u8, 101u8, 110u8, 116u8, 0u8, 0u8, 0u8, 0u8, 51u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 77u8, 97u8, 114u8, 107u8, 32u8,
         115u8, 101u8, 116u8, 32u8, 97u8, 32u8, 109u8, 97u8, 106u8, 111u8,
         114u8, 32u8, 108u8, 101u8, 97u8, 103u8, 117u8, 101u8, 32u8, 104u8,
         111u8, 109u8, 101u8, 32u8, 114u8, 117u8, 110u8, 32u8, 114u8, 101u8,
         99u8, 111u8, 114u8, 100u8, 32u8, 105u8, 110u8, 32u8, 49u8, 57u8,
         57u8, 56u8, 46u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 116u8, 97u8, 116u8, 115u8, 0u8, 0u8,
         0u8, 0u8, 38u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 54u8, 53u8,
         32u8, 72u8, 111u8, 109u8, 101u8, 32u8, 82u8, 117u8, 110u8, 115u8,
         92u8, 110u8, 48u8, 46u8, 50u8, 55u8, 56u8, 32u8, 66u8, 97u8, 116u8,
         116u8, 105u8, 110u8, 103u8, 32u8, 65u8, 118u8, 101u8, 114u8, 97u8,
         103u8, 101u8, 92u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_scalar_strip_1_3_753E()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/753E/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         97u8, 98u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_6_3_separation_spaces_6BCT()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6BCT/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8, 111u8, 0u8,
         0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 97u8,
         114u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8,
         97u8, 122u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 98u8, 97u8, 122u8] ; let res = crate :: parse(& buf) . unwrap()
        ; let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_various_empty_or_newline_only_quoted_strings_NAT4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/NAT4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 32u8, 39u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8,
         32u8, 39u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 99u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 34u8, 32u8, 34u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 100u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 34u8, 32u8, 34u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 92u8, 110u8, 39u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 92u8, 110u8, 34u8, 0u8,
         0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 0u8,
         0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 92u8,
         110u8, 92u8, 110u8, 39u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 104u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 34u8, 92u8, 110u8, 92u8, 110u8, 34u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_21_single_pair_implicit_entries_KZN9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/KZN9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 89u8, 65u8, 77u8, 76u8, 0u8, 0u8, 0u8,
         0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 101u8, 112u8,
         97u8, 114u8, 97u8, 116u8, 101u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         101u8, 109u8, 112u8, 116u8, 121u8, 32u8, 107u8, 101u8, 121u8, 32u8,
         101u8, 110u8, 116u8, 114u8, 121u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         74u8, 83u8, 79u8, 78u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 108u8, 105u8, 107u8, 101u8, 0u8, 0u8, 0u8, 0u8, 8u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 100u8, 106u8, 97u8, 99u8,
         101u8, 110u8, 116u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_5_5_comment_indicator_98YD()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/98YD/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_7_literal_scalar_M9B4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/M9B4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         108u8, 105u8, 116u8, 101u8, 114u8, 97u8, 108u8, 92u8, 110u8, 92u8,
         116u8, 116u8, 101u8, 120u8, 116u8, 92u8, 110u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_various_trailing_comments_1_3_RZP5()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/RZP5/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 100u8,
         111u8, 117u8, 98u8, 108u8, 101u8, 32u8, 113u8, 117u8, 111u8, 116u8,
         101u8, 115u8, 34u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 112u8, 108u8, 97u8, 105u8, 110u8, 32u8, 118u8, 97u8, 108u8,
         117u8, 101u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 99u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 100u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         115u8, 101u8, 113u8, 49u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 115u8, 101u8, 113u8, 50u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 120u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 121u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 98u8, 108u8, 111u8, 99u8, 107u8, 0u8, 0u8, 0u8, 0u8, 8u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 97u8, 98u8, 99u8, 100u8,
         101u8, 92u8, 110u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_multi_level_mapping_indent_9FMG()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9FMG/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 2u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 2u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 102u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 103u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 104u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 105u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_9_3_bare_documents_M7A3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/M7A3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 38u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         37u8, 33u8, 80u8, 83u8, 45u8, 65u8, 100u8, 111u8, 98u8, 101u8, 45u8,
         50u8, 46u8, 48u8, 32u8, 35u8, 32u8, 78u8, 111u8, 116u8, 32u8, 116u8,
         104u8, 101u8, 32u8, 102u8, 105u8, 114u8, 115u8, 116u8, 32u8, 108u8,
         105u8, 110u8, 101u8, 92u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_2_block_indentation_indicator_R4YG()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/R4YG/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 100u8,
         101u8, 116u8, 101u8, 99u8, 116u8, 101u8, 100u8, 92u8, 110u8, 0u8,
         0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 92u8,
         110u8, 92u8, 110u8, 35u8, 32u8, 100u8, 101u8, 116u8, 101u8, 99u8,
         116u8, 101u8, 100u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 32u8, 101u8, 120u8, 112u8, 108u8,
         105u8, 99u8, 105u8, 116u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8, 15u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 92u8, 116u8, 92u8, 110u8,
         100u8, 101u8, 116u8, 101u8, 99u8, 116u8, 101u8, 100u8, 92u8, 110u8] ;
        let res = crate :: parse(& buf) . unwrap() ; let expected : Yaml =
        bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_lookahead_test_cases_AZW3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/AZW3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 108u8, 97u8, 34u8,
         107u8, 101u8, 107u8, 115u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 102u8, 111u8, 111u8, 2u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 108u8, 97u8, 93u8, 107u8, 101u8,
         107u8, 115u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 102u8, 111u8, 111u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_8_play_by_play_feed_from_a_game_U9NS()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/U9NS/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 105u8,
         109u8, 101u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 50u8, 48u8, 58u8, 48u8, 51u8, 58u8, 52u8, 55u8, 0u8, 0u8, 0u8,
         0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 112u8, 108u8, 97u8,
         121u8, 101u8, 114u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 83u8, 97u8, 109u8, 109u8, 121u8, 32u8, 83u8, 111u8,
         115u8, 97u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 97u8, 99u8, 116u8, 105u8, 111u8, 110u8, 0u8, 0u8, 0u8, 0u8,
         10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 114u8, 97u8, 110u8,
         100u8, 32u8, 115u8, 108u8, 97u8, 109u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_2_mapping_scalars_to_scalars_SYW4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/SYW4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 114u8, 0u8,
         0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 54u8, 53u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8,
         118u8, 103u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 48u8, 46u8, 50u8, 55u8, 56u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 114u8, 98u8, 105u8, 0u8, 0u8, 0u8, 0u8, 3u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 52u8, 55u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_5_3_block_structure_indicators_S9E8()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/S9E8/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 101u8,
         113u8, 117u8, 101u8, 110u8, 99u8, 101u8, 1u8, 0u8, 0u8, 0u8, 2u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 110u8, 101u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 119u8, 111u8, 0u8,
         0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 109u8, 97u8,
         112u8, 112u8, 105u8, 110u8, 103u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 115u8, 107u8, 121u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 108u8, 117u8, 101u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 101u8, 97u8,
         0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
         114u8, 101u8, 101u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_7_literal_scalar_1_3_T5N4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/T5N4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         108u8, 105u8, 116u8, 101u8, 114u8, 97u8, 108u8, 92u8, 110u8, 92u8,
         116u8, 116u8, 101u8, 120u8, 116u8, 92u8, 110u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_5_7_block_scalar_indicators_5BVJ()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/5BVJ/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 108u8, 105u8,
         116u8, 101u8, 114u8, 97u8, 108u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 115u8, 111u8, 109u8, 101u8, 92u8,
         110u8, 116u8, 101u8, 120u8, 116u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8,
         6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8, 108u8, 100u8,
         101u8, 100u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 62u8, 115u8, 111u8, 109u8, 101u8, 32u8, 116u8, 101u8, 120u8,
         116u8, 92u8, 110u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_multiple_entry_block_sequence_K4SU()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/K4SU/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         98u8, 97u8, 114u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 52u8, 50u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn
    test_zero_indented_block_scalar_with_line_that_looks_like_a_comment_DK3J()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/DK3J/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8,
         108u8, 105u8, 110u8, 101u8, 49u8, 32u8, 35u8, 32u8, 110u8, 111u8,
         32u8, 99u8, 111u8, 109u8, 109u8, 101u8, 110u8, 116u8, 32u8, 108u8,
         105u8, 110u8, 101u8, 51u8, 92u8, 110u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_5_4_flow_collection_indicators_UDR7()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/UDR7/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 101u8,
         113u8, 117u8, 101u8, 110u8, 99u8, 101u8, 1u8, 0u8, 0u8, 0u8, 2u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 110u8, 101u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 119u8, 111u8, 0u8,
         0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 109u8, 97u8,
         112u8, 112u8, 105u8, 110u8, 103u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 115u8, 107u8, 121u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 108u8, 117u8, 101u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 101u8, 97u8,
         0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
         114u8, 101u8, 101u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_6_10_comment_lines_8G76()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/8G76/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_2_block_indentation_indicator_1_3_4QFQ()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4QFQ/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 100u8,
         101u8, 116u8, 101u8, 99u8, 116u8, 101u8, 100u8, 92u8, 110u8, 0u8,
         0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 92u8,
         110u8, 92u8, 110u8, 35u8, 32u8, 100u8, 101u8, 116u8, 101u8, 99u8,
         116u8, 101u8, 100u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 32u8, 101u8, 120u8, 112u8, 108u8,
         105u8, 99u8, 105u8, 116u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8, 11u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 100u8, 101u8, 116u8, 101u8,
         99u8, 116u8, 101u8, 100u8, 92u8, 110u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_flow_sequence_in_flow_mapping_SBG9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/SBG9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 1u8, 0u8, 0u8, 0u8,
         2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_6_empty_scalar_chomping_K858()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/K858/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 116u8,
         114u8, 105u8, 112u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 62u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 99u8, 108u8, 105u8, 112u8, 0u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 0u8, 0u8, 0u8, 0u8, 4u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8, 101u8, 112u8, 0u8,
         0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 92u8,
         110u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_allowed_characters_in_keys_2EBW()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/2EBW/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 40u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 33u8, 34u8,
         35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8, 43u8, 44u8, 45u8,
         46u8, 47u8, 48u8, 57u8, 58u8, 59u8, 60u8, 61u8, 62u8, 63u8, 64u8,
         65u8, 90u8, 91u8, 92u8, 92u8, 93u8, 94u8, 95u8, 96u8, 97u8, 122u8,
         123u8, 124u8, 125u8, 126u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 102u8, 101u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 63u8, 102u8, 111u8, 111u8,
         0u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8,
         97u8, 102u8, 101u8, 32u8, 113u8, 117u8, 101u8, 115u8, 116u8, 105u8,
         111u8, 110u8, 32u8, 109u8, 97u8, 114u8, 107u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 58u8, 102u8, 111u8, 111u8,
         0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8,
         97u8, 102u8, 101u8, 32u8, 99u8, 111u8, 108u8, 111u8, 110u8, 0u8, 0u8,
         0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 45u8, 102u8, 111u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         115u8, 97u8, 102u8, 101u8, 32u8, 100u8, 97u8, 115u8, 104u8, 0u8, 0u8,
         0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 104u8,
         105u8, 115u8, 32u8, 105u8, 115u8, 35u8, 110u8, 111u8, 116u8, 0u8,
         0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 32u8,
         99u8, 111u8, 109u8, 109u8, 101u8, 110u8, 116u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_14_block_sequence_JQ4R()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/JQ4R/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 108u8,
         111u8, 99u8, 107u8, 32u8, 115u8, 101u8, 113u8, 117u8, 101u8, 110u8,
         99u8, 101u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         111u8, 110u8, 101u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 116u8, 119u8, 111u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 116u8, 104u8, 114u8, 101u8, 101u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn
    test_sequence_entry_that_looks_like_two_with_wrong_indentation_AB8U()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/AB8U/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 33u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 105u8,
         110u8, 103u8, 108u8, 101u8, 32u8, 109u8, 117u8, 108u8, 116u8, 105u8,
         108u8, 105u8, 110u8, 101u8, 32u8, 45u8, 32u8, 115u8, 101u8, 113u8,
         117u8, 101u8, 110u8, 99u8, 101u8, 32u8, 101u8, 110u8, 116u8, 114u8,
         121u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn
    test_spec_example_2_13_in_literals_newlines_are_preserved_6JQW()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6JQW/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         92u8, 92u8, 47u8, 47u8, 124u8, 124u8, 92u8, 92u8, 47u8, 124u8, 124u8,
         92u8, 110u8, 47u8, 47u8, 32u8, 124u8, 124u8, 32u8, 32u8, 124u8,
         124u8, 95u8, 95u8, 92u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_13_flow_sequence_5KJE()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/5KJE/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 110u8, 101u8, 0u8,
         0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 119u8,
         111u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8,
         104u8, 114u8, 101u8, 101u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 102u8, 111u8, 117u8, 114u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_blank_lines_H2RW()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/H2RW/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         49u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         98u8, 97u8, 114u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 50u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 116u8, 101u8, 120u8, 116u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 97u8, 92u8, 110u8, 32u8, 32u8,
         92u8, 110u8, 98u8, 92u8, 110u8, 92u8, 110u8, 99u8, 92u8, 110u8, 92u8,
         110u8, 100u8, 92u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_multiline_plain_value_with_tabs_on_empty_lines_NB6Z()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/NB6Z/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         121u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         118u8, 97u8, 108u8, 117u8, 101u8, 32u8, 119u8, 105u8, 116u8, 104u8,
         92u8, 110u8, 116u8, 97u8, 98u8, 115u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_3_mapping_scalars_to_sequences_PBJ2()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/PBJ2/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 109u8, 101u8,
         114u8, 105u8, 99u8, 97u8, 110u8, 1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 66u8, 111u8, 115u8, 116u8, 111u8, 110u8, 32u8,
         82u8, 101u8, 100u8, 32u8, 83u8, 111u8, 120u8, 0u8, 0u8, 0u8, 0u8,
         14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 68u8, 101u8, 116u8, 114u8,
         111u8, 105u8, 116u8, 32u8, 84u8, 105u8, 103u8, 101u8, 114u8, 115u8,
         0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 78u8,
         101u8, 119u8, 32u8, 89u8, 111u8, 114u8, 107u8, 32u8, 89u8, 97u8,
         110u8, 107u8, 101u8, 101u8, 115u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 97u8, 116u8, 105u8, 111u8, 110u8,
         97u8, 108u8, 1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         78u8, 101u8, 119u8, 32u8, 89u8, 111u8, 114u8, 107u8, 32u8, 77u8,
         101u8, 116u8, 115u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 67u8, 104u8, 105u8, 99u8, 97u8, 103u8, 111u8, 32u8,
         67u8, 117u8, 98u8, 115u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 65u8, 116u8, 108u8, 97u8, 110u8, 116u8, 97u8,
         32u8, 66u8, 114u8, 97u8, 118u8, 101u8, 115u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_sequence_indent_RLU9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/RLU9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8,
         50u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         98u8, 97u8, 114u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         52u8, 52u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_multiline_scalar_in_mapping_A984()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/A984/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 32u8, 99u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8,
         32u8, 102u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_simple_mapping_indent_9J7A()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9J7A/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8,
         97u8, 114u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 98u8, 97u8, 122u8] ; let res = crate :: parse(& buf) . unwrap()
        ; let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_empty_lines_at_end_of_document_NHX8()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/NHX8/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_6_double_quoted_lines_1_3_9TFX()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9TFX/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 46u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8,
         32u8, 49u8, 115u8, 116u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8,
         109u8, 112u8, 116u8, 121u8, 92u8, 110u8, 50u8, 110u8, 100u8, 32u8,
         110u8, 111u8, 110u8, 45u8, 101u8, 109u8, 112u8, 116u8, 121u8, 32u8,
         51u8, 114u8, 100u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8, 109u8,
         112u8, 116u8, 121u8, 32u8, 34u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_scalar_indicator_order_D83L()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/D83L/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 101u8,
         120u8, 112u8, 108u8, 105u8, 99u8, 105u8, 116u8, 32u8, 105u8, 110u8,
         100u8, 101u8, 110u8, 116u8, 32u8, 97u8, 110u8, 100u8, 32u8, 99u8,
         104u8, 111u8, 109u8, 112u8, 0u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 124u8, 99u8, 104u8, 111u8, 109u8, 112u8, 32u8,
         97u8, 110u8, 100u8, 32u8, 101u8, 120u8, 112u8, 108u8, 105u8, 99u8,
         105u8, 116u8, 32u8, 105u8, 110u8, 100u8, 101u8, 110u8, 116u8] ; let
        res = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode
        :: deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_whitespace_after_scalars_in_flow_LP6E()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/LP6E/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_implicit_flow_mapping_key_on_one_line_LX3P()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/LX3P/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 108u8, 111u8, 119u8,
         0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8,
         108u8, 111u8, 99u8, 107u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_two_document_start_markers_6XDY()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6XDY/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_colon_followed_by_comma_S7BG()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/S7BG/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 58u8, 44u8] ; let
        res = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode
        :: deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_17_quoted_scalars_G4RS()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/G4RS/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 117u8, 110u8,
         105u8, 99u8, 111u8, 100u8, 101u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 83u8, 111u8, 115u8, 97u8, 32u8, 100u8,
         105u8, 100u8, 32u8, 102u8, 105u8, 110u8, 101u8, 46u8, 226u8, 152u8,
         186u8, 34u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 99u8, 111u8, 110u8, 116u8, 114u8, 111u8, 108u8, 0u8, 0u8, 0u8,
         0u8, 22u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 92u8, 98u8, 49u8,
         57u8, 57u8, 56u8, 92u8, 116u8, 49u8, 57u8, 57u8, 57u8, 92u8, 116u8,
         50u8, 48u8, 48u8, 48u8, 92u8, 110u8, 34u8, 0u8, 0u8, 0u8, 0u8, 7u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 101u8, 120u8, 32u8, 101u8,
         115u8, 99u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 34u8, 92u8, 114u8, 92u8, 110u8, 32u8, 105u8, 115u8, 32u8, 92u8,
         114u8, 92u8, 110u8, 34u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 115u8, 105u8, 110u8, 103u8, 108u8, 101u8, 0u8,
         0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 34u8,
         72u8, 111u8, 119u8, 100u8, 121u8, 33u8, 34u8, 32u8, 104u8, 101u8,
         32u8, 99u8, 114u8, 105u8, 101u8, 100u8, 46u8, 39u8, 0u8, 0u8, 0u8,
         0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 113u8, 117u8, 111u8,
         116u8, 101u8, 100u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 39u8, 32u8, 35u8, 32u8, 78u8, 111u8, 116u8, 32u8,
         97u8, 32u8, 39u8, 99u8, 111u8, 109u8, 109u8, 101u8, 110u8, 116u8,
         39u8, 46u8, 39u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 116u8, 105u8, 101u8, 45u8, 102u8, 105u8, 103u8, 104u8,
         116u8, 101u8, 114u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 39u8, 124u8, 92u8, 92u8, 45u8, 42u8, 45u8, 47u8,
         124u8, 39u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn
    test_spec_example_7_21_single_pair_implicit_entries_1_3_9MMW()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9MMW/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 89u8, 65u8, 77u8, 76u8, 0u8, 0u8, 0u8,
         0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 101u8, 112u8,
         97u8, 114u8, 97u8, 116u8, 101u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         101u8, 109u8, 112u8, 116u8, 121u8, 32u8, 107u8, 101u8, 121u8, 32u8,
         101u8, 110u8, 116u8, 114u8, 121u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 34u8, 74u8, 83u8, 79u8, 78u8, 32u8, 108u8, 105u8, 107u8,
         101u8, 34u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 97u8, 100u8, 106u8, 97u8, 99u8, 101u8, 110u8, 116u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn
    test_multiline_doublequoted_flow_mapping_key_without_value_9BXH()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9BXH/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 115u8, 105u8, 110u8,
         103u8, 108u8, 101u8, 32u8, 108u8, 105u8, 110u8, 101u8, 34u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8,
         0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 2u8, 0u8, 0u8,
         0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 109u8, 117u8, 108u8,
         116u8, 105u8, 32u8, 108u8, 105u8, 110u8, 101u8, 34u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_flow_mapping_colon_on_line_after_key_4MUZ()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4MUZ/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 102u8, 111u8,
         111u8, 34u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 34u8, 98u8, 97u8, 114u8, 34u8] ; let res = crate :: parse(& buf)
        . unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_document_with_footer_S4T7()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/S4T7/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 97u8, 97u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8,
         98u8, 98u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_9_6_stream_6ZKB()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6ZKB/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 109u8, 97u8, 116u8,
         99u8, 104u8, 101u8, 115u8, 32u8, 37u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 50u8, 48u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_14_flow_sequence_entries_8UDB()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/8UDB/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 100u8,
         111u8, 117u8, 98u8, 108u8, 101u8, 32u8, 113u8, 117u8, 111u8, 116u8,
         101u8, 100u8, 34u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 39u8, 115u8, 105u8, 110u8, 103u8, 108u8, 101u8, 32u8,
         113u8, 117u8, 111u8, 116u8, 101u8, 100u8, 39u8, 0u8, 0u8, 0u8, 0u8,
         10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 112u8, 108u8, 97u8, 105u8,
         110u8, 32u8, 116u8, 101u8, 120u8, 116u8, 1u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 101u8, 115u8, 116u8, 101u8, 100u8,
         2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 105u8,
         110u8, 103u8, 108u8, 101u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 112u8, 97u8, 105u8, 114u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_scalar_strip_MYW6()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/MYW6/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         97u8, 98u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_5_sequence_of_sequences_YD5X()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/YD5X/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 97u8, 109u8, 101u8,
         0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8,
         114u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         97u8, 118u8, 103u8, 1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 77u8, 97u8, 114u8, 107u8, 32u8, 77u8, 99u8, 71u8, 119u8, 105u8,
         114u8, 101u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 54u8, 53u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 48u8, 46u8, 50u8, 55u8, 56u8, 1u8, 0u8, 0u8, 0u8, 3u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 109u8, 109u8, 121u8, 32u8, 83u8,
         111u8, 115u8, 97u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 54u8, 51u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 48u8, 46u8, 50u8, 56u8, 56u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_9_single_quoted_lines_1_3_T4YY()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/T4YY/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 46u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8,
         32u8, 49u8, 115u8, 116u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8,
         109u8, 112u8, 116u8, 121u8, 92u8, 110u8, 50u8, 110u8, 100u8, 32u8,
         110u8, 111u8, 110u8, 45u8, 101u8, 109u8, 112u8, 116u8, 121u8, 32u8,
         51u8, 114u8, 100u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8, 109u8,
         112u8, 116u8, 121u8, 32u8, 39u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_backslashes_in_singlequotes_6H3V()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6H3V/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 102u8,
         111u8, 111u8, 58u8, 32u8, 98u8, 97u8, 114u8, 92u8, 92u8, 39u8, 0u8,
         0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 97u8,
         122u8, 39u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_6_double_quoted_lines_7A4E()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/7A4E/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 46u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8,
         32u8, 49u8, 115u8, 116u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8,
         109u8, 112u8, 116u8, 121u8, 92u8, 110u8, 50u8, 110u8, 100u8, 32u8,
         110u8, 111u8, 110u8, 45u8, 101u8, 109u8, 112u8, 116u8, 121u8, 32u8,
         51u8, 114u8, 100u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8, 109u8,
         112u8, 116u8, 121u8, 32u8, 34u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_zero_indented_sequences_in_explicit_mapping_keys_6PBE()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6PBE/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 1u8, 0u8, 0u8, 0u8,
         2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_plain_scalar_with_backslashes_4V8U()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4V8U/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 112u8,
         108u8, 97u8, 105u8, 110u8, 92u8, 92u8, 118u8, 97u8, 108u8, 117u8,
         101u8, 92u8, 92u8, 119u8, 105u8, 116u8, 104u8, 92u8, 92u8, 98u8,
         97u8, 99u8, 107u8, 115u8, 108u8, 97u8, 115u8, 104u8, 101u8, 115u8] ;
        let res = crate :: parse(& buf) . unwrap() ; let expected : Yaml =
        bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_2_12_compact_nested_mapping_9U5K()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9U5K/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 105u8, 116u8, 101u8, 109u8,
         0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8,
         117u8, 112u8, 101u8, 114u8, 32u8, 72u8, 111u8, 111u8, 112u8, 0u8,
         0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 113u8, 117u8,
         97u8, 110u8, 116u8, 105u8, 116u8, 121u8, 0u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 2u8, 0u8, 0u8, 0u8, 2u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 105u8, 116u8, 101u8, 109u8, 0u8, 0u8, 0u8,
         0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 66u8, 97u8, 115u8,
         107u8, 101u8, 116u8, 98u8, 97u8, 108u8, 108u8, 0u8, 0u8, 0u8, 0u8,
         8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 113u8, 117u8, 97u8, 110u8,
         116u8, 105u8, 116u8, 121u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 52u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 105u8, 116u8, 101u8, 109u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 66u8, 105u8, 103u8, 32u8, 83u8, 104u8,
         111u8, 101u8, 115u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 113u8, 117u8, 97u8, 110u8, 116u8, 105u8, 116u8, 121u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8] ;
        let res = crate :: parse(& buf) . unwrap() ; let expected : Yaml =
        bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_flow_sequence_in_flow_sequence_FUP4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/FUP4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_5_chomping_trailing_lines_F8F9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/F8F9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 116u8,
         114u8, 105u8, 112u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 124u8, 35u8, 32u8, 116u8, 101u8, 120u8, 116u8, 0u8,
         0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 108u8,
         105u8, 112u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 124u8, 35u8, 32u8, 116u8, 101u8, 120u8, 116u8, 92u8, 110u8, 0u8,
         0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         101u8, 112u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 124u8, 35u8, 32u8, 116u8, 101u8, 120u8, 116u8, 92u8, 110u8,
         92u8, 110u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_three_dashes_and_content_without_space_1_3_EXG3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/EXG3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 45u8,
         45u8, 45u8, 119u8, 111u8, 114u8, 100u8, 49u8, 32u8, 119u8, 111u8,
         114u8, 100u8, 50u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_empty_flow_collections_7ZZ5()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/7ZZ5/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_9_6_stream_1_3_9DXL()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9DXL/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 109u8, 97u8, 116u8,
         99u8, 104u8, 101u8, 115u8, 32u8, 37u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 50u8, 48u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_26_ordered_mappings_J7PZ()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/J7PZ/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 77u8, 97u8, 114u8, 107u8,
         32u8, 77u8, 99u8, 71u8, 119u8, 105u8, 114u8, 101u8, 0u8, 0u8, 0u8,
         0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 54u8, 53u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 109u8, 109u8,
         121u8, 32u8, 83u8, 111u8, 115u8, 97u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 54u8, 51u8, 2u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 75u8, 101u8, 110u8, 32u8, 71u8, 114u8,
         105u8, 102u8, 102u8, 121u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 53u8, 56u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_empty_lines_between_mapping_elements_J7VC()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/J7VC/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 50u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 51u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_scalars_on_line_KSS4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/KSS4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 38u8,
         110u8, 111u8, 100u8, 101u8, 32u8, 58u8, 102u8, 111u8, 111u8] ; let
        res = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode
        :: deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_flow_mapping_in_block_sequence_MXS3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/MXS3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_multiline_double_quoted_flow_mapping_key_9SA2()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9SA2/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 115u8, 105u8, 110u8,
         103u8, 108u8, 101u8, 32u8, 108u8, 105u8, 110u8, 101u8, 34u8, 0u8,
         0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8,
         108u8, 117u8, 101u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 34u8, 109u8, 117u8, 108u8, 116u8, 105u8, 32u8, 108u8,
         105u8, 110u8, 101u8, 34u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8, 101u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_various_trailing_comments_XW4D()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/XW4D/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 100u8,
         111u8, 117u8, 98u8, 108u8, 101u8, 32u8, 113u8, 117u8, 111u8, 116u8,
         101u8, 115u8, 34u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 112u8, 108u8, 97u8, 105u8, 110u8, 32u8, 118u8, 97u8, 108u8,
         117u8, 101u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 99u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 100u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         115u8, 101u8, 113u8, 49u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 115u8, 101u8, 113u8, 50u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 120u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 121u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 98u8, 108u8, 111u8, 99u8, 107u8, 0u8, 0u8, 0u8, 0u8, 8u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 97u8, 98u8, 99u8, 100u8,
         101u8, 92u8, 110u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_comment_in_flow_sequence_before_comma_7TMG()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/7TMG/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 119u8, 111u8,
         114u8, 100u8, 49u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 119u8, 111u8, 114u8, 100u8, 50u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_6_11_multi_line_comments_P94K()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/P94K/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         121u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         118u8, 97u8, 108u8, 117u8, 101u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_8_literal_content_DWX9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/DWX9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         92u8, 110u8, 92u8, 110u8, 108u8, 105u8, 116u8, 101u8, 114u8, 97u8,
         108u8, 92u8, 110u8, 32u8, 92u8, 110u8, 92u8, 110u8, 116u8, 101u8,
         120u8, 116u8, 92u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_9_single_quoted_lines_PRH3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/PRH3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 46u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8,
         32u8, 49u8, 115u8, 116u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8,
         109u8, 112u8, 116u8, 121u8, 92u8, 110u8, 50u8, 110u8, 100u8, 32u8,
         110u8, 111u8, 110u8, 45u8, 101u8, 109u8, 112u8, 116u8, 121u8, 32u8,
         51u8, 114u8, 100u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8, 109u8,
         112u8, 116u8, 121u8, 32u8, 39u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_three_explicit_integers_in_a_block_sequence_33X3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/33X3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 60u8, 116u8, 97u8,
         103u8, 58u8, 121u8, 97u8, 109u8, 108u8, 46u8, 111u8, 114u8, 103u8,
         44u8, 50u8, 48u8, 48u8, 50u8, 58u8, 105u8, 110u8, 116u8, 62u8, 32u8,
         58u8, 49u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 60u8, 116u8, 97u8, 103u8, 58u8, 121u8, 97u8, 109u8, 108u8, 46u8,
         111u8, 114u8, 103u8, 44u8, 50u8, 48u8, 48u8, 50u8, 58u8, 105u8,
         110u8, 116u8, 62u8, 32u8, 58u8, 45u8, 50u8, 0u8, 0u8, 0u8, 0u8, 27u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 60u8, 116u8, 97u8, 103u8, 58u8,
         121u8, 97u8, 109u8, 108u8, 46u8, 111u8, 114u8, 103u8, 44u8, 50u8,
         48u8, 48u8, 50u8, 58u8, 105u8, 110u8, 116u8, 62u8, 32u8, 58u8, 51u8,
         51u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_2_4_sequence_of_mappings_229Q()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/229Q/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 97u8, 109u8, 101u8,
         0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 77u8,
         97u8, 114u8, 107u8, 32u8, 77u8, 99u8, 71u8, 119u8, 105u8, 114u8,
         101u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         104u8, 114u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 54u8, 53u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 97u8, 118u8, 103u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 48u8, 46u8, 50u8, 55u8, 56u8, 2u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 97u8, 109u8, 101u8, 0u8, 0u8,
         0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 109u8,
         109u8, 121u8, 32u8, 83u8, 111u8, 115u8, 97u8, 0u8, 0u8, 0u8, 0u8,
         2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 114u8, 0u8, 0u8, 0u8,
         0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 54u8, 51u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 118u8, 103u8,
         0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 48u8,
         46u8, 50u8, 56u8, 56u8] ; let res = crate :: parse(& buf) . unwrap()
        ; let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn
    test_spec_example_2_9_single_document_with_two_comments_J9HZ()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/J9HZ/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 114u8, 1u8,
         0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 77u8, 97u8, 114u8,
         107u8, 32u8, 77u8, 99u8, 71u8, 119u8, 105u8, 114u8, 101u8, 0u8, 0u8,
         0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 109u8,
         109u8, 121u8, 32u8, 83u8, 111u8, 115u8, 97u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 114u8, 98u8, 105u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 109u8, 109u8,
         121u8, 32u8, 83u8, 111u8, 115u8, 97u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 75u8, 101u8, 110u8, 32u8, 71u8, 114u8,
         105u8, 102u8, 102u8, 101u8, 121u8] ; let res = crate :: parse(& buf)
        . unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_single_pair_block_mapping_D9TU()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/D9TU/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         98u8, 97u8, 114u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_mixed_block_mapping_implicit_to_explicit_RR7F()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/RR7F/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 46u8, 50u8,
         0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 50u8,
         51u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         100u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_block_sequence_in_block_sequence_3ALJ()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/3ALJ/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 49u8, 95u8, 105u8,
         49u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         115u8, 49u8, 95u8, 105u8, 50u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 50u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_nested_top_level_flow_mapping_ZK9H()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/ZK9H/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         121u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8,
         101u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_8_19_compact_block_mappings_V9D5()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/V9D5/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 117u8, 110u8, 0u8,
         0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 121u8, 101u8,
         108u8, 108u8, 111u8, 119u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         101u8, 97u8, 114u8, 116u8, 104u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 108u8, 117u8, 101u8, 2u8, 0u8, 0u8,
         0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 109u8, 111u8, 111u8, 110u8, 0u8,
         0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 119u8, 104u8,
         105u8, 116u8, 101u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_5_12_tabs_and_spaces_J3BT()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/J3BT/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 113u8, 117u8,
         111u8, 116u8, 101u8, 100u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 34u8, 81u8, 117u8, 111u8, 116u8, 101u8, 100u8,
         32u8, 92u8, 116u8, 34u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 98u8, 108u8, 111u8, 99u8, 107u8, 0u8, 0u8, 0u8, 0u8,
         50u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 118u8, 111u8, 105u8,
         100u8, 32u8, 109u8, 97u8, 105u8, 110u8, 40u8, 41u8, 32u8, 123u8,
         92u8, 110u8, 92u8, 116u8, 112u8, 114u8, 105u8, 110u8, 116u8, 102u8,
         40u8, 34u8, 72u8, 101u8, 108u8, 108u8, 111u8, 44u8, 32u8, 119u8,
         111u8, 114u8, 108u8, 100u8, 33u8, 92u8, 92u8, 110u8, 34u8, 41u8,
         59u8, 92u8, 110u8, 125u8, 92u8, 110u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_mappings_in_block_sequence_93JH()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/93JH/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8, 121u8, 0u8,
         0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8,
         108u8, 117u8, 101u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 107u8, 101u8, 121u8, 50u8, 0u8, 0u8, 0u8, 0u8, 6u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8, 101u8,
         50u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8,
         101u8, 121u8, 51u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 118u8, 97u8, 108u8, 117u8, 101u8, 51u8] ; let res = crate
        :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_nested_flow_collections_on_one_line_F3CP()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/F3CP/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 1u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 2u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 101u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 102u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_9_2_document_markers_RTP8()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/RTP8/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 68u8,
         111u8, 99u8, 117u8, 109u8, 101u8, 110u8, 116u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_tab_after_document_header_K54U()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/K54U/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8,
         99u8, 97u8, 108u8, 97u8, 114u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_7_single_quoted_characters_1_3_SSW6()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/SSW6/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8,
         104u8, 101u8, 114u8, 101u8, 39u8, 115u8, 32u8, 116u8, 111u8, 32u8,
         34u8, 113u8, 117u8, 111u8, 116u8, 101u8, 115u8, 34u8, 39u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_11_plain_implicit_keys_L9U5()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/L9U5/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 105u8, 109u8,
         112u8, 108u8, 105u8, 99u8, 105u8, 116u8, 32u8, 98u8, 108u8, 111u8,
         99u8, 107u8, 32u8, 107u8, 101u8, 121u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 105u8, 109u8, 112u8, 108u8, 105u8, 99u8, 105u8, 116u8,
         32u8, 102u8, 108u8, 111u8, 119u8, 32u8, 107u8, 101u8, 121u8, 0u8,
         0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8,
         108u8, 117u8, 101u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_23_flow_content_Q88A()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/Q88A/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 2u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 97u8, 34u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 98u8, 39u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_4_double_quoted_implicit_keys_LQZ7()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/LQZ7/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 105u8,
         109u8, 112u8, 108u8, 105u8, 99u8, 105u8, 116u8, 32u8, 98u8, 108u8,
         111u8, 99u8, 107u8, 32u8, 107u8, 101u8, 121u8, 34u8, 1u8, 0u8, 0u8,
         0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 105u8, 109u8, 112u8, 108u8,
         105u8, 99u8, 105u8, 116u8, 32u8, 102u8, 108u8, 111u8, 119u8, 32u8,
         107u8, 101u8, 121u8, 34u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8, 101u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_multiline_plain_scalar_with_empty_line_36F6()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/36F6/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 112u8, 108u8, 97u8,
         105u8, 110u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 97u8, 32u8, 98u8, 92u8, 110u8, 99u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_18_flow_mapping_adjacent_values_C2DT()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/C2DT/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 97u8, 100u8,
         106u8, 97u8, 99u8, 101u8, 110u8, 116u8, 34u8, 0u8, 0u8, 0u8, 0u8,
         5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8,
         101u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         34u8, 114u8, 101u8, 97u8, 100u8, 97u8, 98u8, 108u8, 101u8, 34u8, 0u8,
         0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8,
         108u8, 117u8, 101u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 34u8, 101u8, 109u8, 112u8, 116u8, 121u8, 34u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn
    test_colon_and_adjacent_value_after_comment_on_next_line_K3WX()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/K3WX/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 102u8, 111u8,
         111u8, 34u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 98u8, 97u8, 114u8] ; let res = crate :: parse(& buf) . unwrap()
        ; let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_12_plain_lines_HS5T()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/HS5T/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8,
         115u8, 116u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8, 109u8, 112u8,
         116u8, 121u8, 92u8, 110u8, 50u8, 110u8, 100u8, 32u8, 110u8, 111u8,
         110u8, 45u8, 101u8, 109u8, 112u8, 116u8, 121u8, 32u8, 51u8, 114u8,
         100u8, 32u8, 110u8, 111u8, 110u8, 45u8, 101u8, 109u8, 112u8, 116u8,
         121u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_nested_flow_collections_M7NX()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/M7NX/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 1u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 2u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 101u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 102u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_7_two_documents_in_a_stream_JHB9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/JHB9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 67u8, 104u8,
         105u8, 99u8, 97u8, 103u8, 111u8, 32u8, 67u8, 117u8, 98u8, 115u8, 0u8,
         0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 116u8,
         32u8, 76u8, 111u8, 117u8, 105u8, 115u8, 32u8, 67u8, 97u8, 114u8,
         100u8, 105u8, 110u8, 97u8, 108u8, 115u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn
    test_tab_at_beginning_of_line_followed_by_a_flow_mapping_Q5MG()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/Q5MG/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_document_end_marker_HWV9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/HWV9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_5_double_quoted_line_breaks_NP9H()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/NP9H/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 57u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8,
         102u8, 111u8, 108u8, 100u8, 101u8, 100u8, 32u8, 116u8, 111u8, 32u8,
         97u8, 32u8, 115u8, 112u8, 97u8, 99u8, 101u8, 44u8, 92u8, 110u8,
         116u8, 111u8, 32u8, 97u8, 32u8, 108u8, 105u8, 110u8, 101u8, 32u8,
         102u8, 101u8, 101u8, 100u8, 44u8, 32u8, 111u8, 114u8, 32u8, 92u8,
         116u8, 32u8, 92u8, 116u8, 110u8, 111u8, 110u8, 45u8, 99u8, 111u8,
         110u8, 116u8, 101u8, 110u8, 116u8, 34u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_mapping_with_multiline_scalars_JTV5()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/JTV5/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 32u8, 116u8,
         114u8, 117u8, 101u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 110u8, 117u8, 108u8, 108u8, 32u8, 100u8, 0u8, 0u8,
         0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 32u8, 52u8,
         50u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ;
        let res = crate :: parse(& buf) . unwrap() ; let expected : Yaml =
        bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_literal_unicode_H3Z8()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/H3Z8/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 119u8, 97u8, 110u8,
         116u8, 101u8, 100u8, 0u8, 0u8, 0u8, 0u8, 22u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 108u8, 111u8, 118u8, 101u8, 32u8, 226u8, 153u8, 165u8,
         32u8, 97u8, 110u8, 100u8, 32u8, 112u8, 101u8, 97u8, 99u8, 101u8,
         32u8, 226u8, 152u8, 174u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_construct_binary_565N()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/565N/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 97u8, 110u8,
         111u8, 110u8, 105u8, 99u8, 97u8, 108u8, 0u8, 0u8, 0u8, 0u8, 20u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 60u8, 116u8, 97u8, 103u8, 58u8,
         121u8, 97u8, 109u8, 108u8, 46u8, 111u8, 114u8, 103u8, 44u8, 50u8,
         48u8, 48u8, 50u8, 58u8, 98u8, 105u8, 110u8, 97u8, 114u8, 121u8, 62u8,
         32u8, 34u8, 82u8, 48u8, 108u8, 71u8, 79u8, 68u8, 108u8, 104u8, 68u8,
         65u8, 65u8, 77u8, 65u8, 73u8, 81u8, 65u8, 65u8, 80u8, 47u8, 47u8,
         57u8, 47u8, 88u8, 49u8, 55u8, 117u8, 110u8, 112u8, 53u8, 87u8, 90u8,
         109u8, 90u8, 103u8, 65u8, 65u8, 65u8, 79u8, 102u8, 110u8, 53u8, 49u8,
         53u8, 101u8, 88u8, 118u8, 80u8, 122u8, 55u8, 89u8, 54u8, 79u8, 106u8,
         117u8, 68u8, 103u8, 52u8, 74u8, 43u8, 102u8, 110u8, 53u8, 79u8, 84u8,
         107u8, 54u8, 101u8, 110u8, 112u8, 53u8, 54u8, 101u8, 110u8, 109u8,
         108u8, 112u8, 97u8, 87u8, 78u8, 106u8, 89u8, 54u8, 79u8, 106u8,
         111u8, 52u8, 83u8, 69u8, 104u8, 80u8, 47u8, 43u8, 43u8, 102u8, 47u8,
         43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8,
         47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8,
         102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8,
         43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8,
         43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 83u8, 72u8, 43u8, 68u8, 107u8,
         49u8, 104u8, 90u8, 71u8, 85u8, 103u8, 100u8, 50u8, 108u8, 48u8, 97u8,
         67u8, 66u8, 72u8, 83u8, 85u8, 49u8, 81u8, 65u8, 67u8, 119u8, 65u8,
         65u8, 65u8, 65u8, 65u8, 68u8, 65u8, 65u8, 77u8, 65u8, 65u8, 65u8,
         70u8, 76u8, 67u8, 65u8, 103u8, 106u8, 111u8, 69u8, 119u8, 110u8,
         117u8, 78u8, 65u8, 70u8, 79u8, 104u8, 112u8, 69u8, 77u8, 84u8, 82u8,
         105u8, 103u8, 103u8, 99u8, 122u8, 52u8, 66u8, 78u8, 74u8, 72u8,
         114u8, 118u8, 47u8, 122u8, 67u8, 70u8, 99u8, 76u8, 105u8, 119u8,
         77u8, 87u8, 89u8, 78u8, 71u8, 56u8, 52u8, 66u8, 119u8, 119u8, 69u8,
         101u8, 69u8, 67u8, 99u8, 103u8, 103u8, 103u8, 111u8, 66u8, 65u8,
         68u8, 115u8, 61u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 103u8, 101u8, 110u8, 101u8, 114u8, 105u8, 99u8, 0u8, 0u8,
         0u8, 0u8, 28u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 60u8, 116u8, 97u8,
         103u8, 58u8, 121u8, 97u8, 109u8, 108u8, 46u8, 111u8, 114u8, 103u8,
         44u8, 50u8, 48u8, 48u8, 50u8, 58u8, 98u8, 105u8, 110u8, 97u8, 114u8,
         121u8, 62u8, 32u8, 124u8, 82u8, 48u8, 108u8, 71u8, 79u8, 68u8, 108u8,
         104u8, 68u8, 65u8, 65u8, 77u8, 65u8, 73u8, 81u8, 65u8, 65u8, 80u8,
         47u8, 47u8, 57u8, 47u8, 88u8, 49u8, 55u8, 117u8, 110u8, 112u8, 53u8,
         87u8, 90u8, 109u8, 90u8, 103u8, 65u8, 65u8, 65u8, 79u8, 102u8, 110u8,
         53u8, 49u8, 53u8, 101u8, 88u8, 118u8, 80u8, 122u8, 55u8, 89u8, 54u8,
         79u8, 106u8, 117u8, 68u8, 103u8, 52u8, 74u8, 43u8, 102u8, 110u8,
         53u8, 92u8, 110u8, 79u8, 84u8, 107u8, 54u8, 101u8, 110u8, 112u8,
         53u8, 54u8, 101u8, 110u8, 109u8, 108u8, 112u8, 97u8, 87u8, 78u8,
         106u8, 89u8, 54u8, 79u8, 106u8, 111u8, 52u8, 83u8, 69u8, 104u8, 80u8,
         47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8,
         102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8,
         43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8,
         43u8, 92u8, 110u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8,
         43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8, 43u8, 43u8, 102u8, 47u8,
         43u8, 43u8, 83u8, 72u8, 43u8, 68u8, 107u8, 49u8, 104u8, 90u8, 71u8,
         85u8, 103u8, 100u8, 50u8, 108u8, 48u8, 97u8, 67u8, 66u8, 72u8, 83u8,
         85u8, 49u8, 81u8, 65u8, 67u8, 119u8, 65u8, 65u8, 65u8, 65u8, 65u8,
         68u8, 65u8, 65u8, 77u8, 65u8, 65u8, 65u8, 70u8, 76u8, 67u8, 92u8,
         110u8, 65u8, 103u8, 106u8, 111u8, 69u8, 119u8, 110u8, 117u8, 78u8,
         65u8, 70u8, 79u8, 104u8, 112u8, 69u8, 77u8, 84u8, 82u8, 105u8, 103u8,
         103u8, 99u8, 122u8, 52u8, 66u8, 78u8, 74u8, 72u8, 114u8, 118u8, 47u8,
         122u8, 67u8, 70u8, 99u8, 76u8, 105u8, 119u8, 77u8, 87u8, 89u8, 78u8,
         71u8, 56u8, 52u8, 66u8, 119u8, 119u8, 69u8, 101u8, 69u8, 67u8, 99u8,
         103u8, 103u8, 103u8, 111u8, 66u8, 65u8, 68u8, 115u8, 61u8, 92u8,
         110u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         100u8, 101u8, 115u8, 99u8, 114u8, 105u8, 112u8, 116u8, 105u8, 111u8,
         110u8, 0u8, 0u8, 0u8, 0u8, 62u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         84u8, 104u8, 101u8, 32u8, 98u8, 105u8, 110u8, 97u8, 114u8, 121u8,
         32u8, 118u8, 97u8, 108u8, 117u8, 101u8, 32u8, 97u8, 98u8, 111u8,
         118u8, 101u8, 32u8, 105u8, 115u8, 32u8, 97u8, 32u8, 116u8, 105u8,
         110u8, 121u8, 32u8, 97u8, 114u8, 114u8, 111u8, 119u8, 32u8, 101u8,
         110u8, 99u8, 111u8, 100u8, 101u8, 100u8, 32u8, 97u8, 115u8, 32u8,
         97u8, 32u8, 103u8, 105u8, 102u8, 32u8, 105u8, 109u8, 97u8, 103u8,
         101u8, 46u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_three_dashes_and_content_without_space_82AN()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/82AN/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 45u8,
         45u8, 45u8, 119u8, 111u8, 114u8, 100u8, 49u8, 32u8, 119u8, 111u8,
         114u8, 100u8, 50u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_comment_and_document_end_marker_QT73()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/QT73/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_17_explicit_block_mapping_entries_5WE3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/5WE3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 120u8,
         112u8, 108u8, 105u8, 99u8, 105u8, 116u8, 32u8, 107u8, 101u8, 121u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 98u8,
         108u8, 111u8, 99u8, 107u8, 32u8, 107u8, 101u8, 121u8, 92u8, 110u8,
         1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 110u8,
         101u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         116u8, 119u8, 111u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_doublequoted_scalar_starting_with_a_tab_CPZ3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/CPZ3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 97u8, 98u8,
         0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8,
         92u8, 116u8, 115u8, 116u8, 114u8, 105u8, 110u8, 103u8, 34u8] ; let
        res = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode
        :: deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_2_empty_content_WZ62()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/WZ62/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         60u8, 116u8, 97u8, 103u8, 58u8, 121u8, 97u8, 109u8, 108u8, 46u8,
         111u8, 114u8, 103u8, 44u8, 50u8, 48u8, 48u8, 50u8, 58u8, 115u8,
         116u8, 114u8, 62u8, 32u8, 58u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 60u8, 116u8, 97u8, 103u8, 58u8, 121u8, 97u8,
         109u8, 108u8, 46u8, 111u8, 114u8, 103u8, 44u8, 50u8, 48u8, 48u8,
         50u8, 58u8, 115u8, 116u8, 114u8, 62u8, 32u8, 58u8, 0u8, 0u8, 0u8,
         0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 97u8, 114u8] ; let
        res = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode
        :: deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_plain_mapping_key_ending_with_colon_8CWC()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/8CWC/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         121u8, 32u8, 101u8, 110u8, 100u8, 115u8, 32u8, 119u8, 105u8, 116u8,
         104u8, 32u8, 116u8, 119u8, 111u8, 32u8, 99u8, 111u8, 108u8, 111u8,
         110u8, 115u8, 58u8, 58u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8, 101u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_6_9_separated_comment_5NYZ()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/5NYZ/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         121u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         118u8, 97u8, 108u8, 117u8, 101u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_scalar_keep_6FWR()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6FWR/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         97u8, 98u8, 92u8, 110u8, 92u8, 110u8, 32u8, 92u8, 110u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_flow_sequence_in_block_mapping_D88J()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/D88J/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_mapping_with_missing_keys_2JQS()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/2JQS/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8] ; let res = crate :: parse(& buf)
        . unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_16_flow_mapping_entries_DFF7()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/DFF7/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 120u8,
         112u8, 108u8, 105u8, 99u8, 105u8, 116u8, 0u8, 0u8, 0u8, 0u8, 5u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 110u8, 116u8, 114u8, 121u8,
         0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 105u8,
         109u8, 112u8, 108u8, 105u8, 99u8, 105u8, 116u8, 0u8, 0u8, 0u8, 0u8,
         5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 110u8, 116u8, 114u8,
         121u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_19_single_pair_flow_mappings_QF4Y()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/QF4Y/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8, 111u8, 0u8,
         0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 97u8,
         114u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_multiline_plain_flow_mapping_key_NJ66()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/NJ66/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 105u8, 110u8, 103u8,
         108u8, 101u8, 32u8, 108u8, 105u8, 110u8, 101u8, 0u8, 0u8, 0u8, 0u8,
         5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8,
         101u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 109u8,
         117u8, 108u8, 116u8, 105u8, 32u8, 108u8, 105u8, 110u8, 101u8, 0u8,
         0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8,
         108u8, 117u8, 101u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_6_4_line_prefixes_4ZYM()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4ZYM/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 112u8, 108u8, 97u8,
         105u8, 110u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 116u8, 101u8, 120u8, 116u8, 32u8, 108u8, 105u8, 110u8, 101u8,
         115u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         113u8, 117u8, 111u8, 116u8, 101u8, 100u8, 0u8, 0u8, 0u8, 0u8, 12u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 116u8, 101u8, 120u8, 116u8,
         32u8, 108u8, 105u8, 110u8, 101u8, 115u8, 34u8, 0u8, 0u8, 0u8, 0u8,
         5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 108u8, 111u8, 99u8,
         107u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         124u8, 116u8, 101u8, 120u8, 116u8, 92u8, 110u8, 32u8, 92u8, 116u8,
         108u8, 105u8, 110u8, 101u8, 115u8, 92u8, 110u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_4_chomping_final_line_break_A6F9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/A6F9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 116u8,
         114u8, 105u8, 112u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 124u8, 116u8, 101u8, 120u8, 116u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 108u8, 105u8, 112u8,
         0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         116u8, 101u8, 120u8, 116u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8, 4u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8, 101u8, 112u8, 0u8,
         0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 116u8,
         101u8, 120u8, 116u8, 92u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_bare_document_after_document_end_marker_7Z25()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/7Z25/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         121u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         118u8, 97u8, 108u8, 117u8, 101u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_multiline_scalar_at_top_level_9YRD()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/9YRD/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8,
         32u8, 98u8, 32u8, 99u8, 32u8, 100u8, 92u8, 110u8, 101u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_8_single_quoted_implicit_keys_87E4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/87E4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 105u8,
         109u8, 112u8, 108u8, 105u8, 99u8, 105u8, 116u8, 32u8, 98u8, 108u8,
         111u8, 99u8, 107u8, 32u8, 107u8, 101u8, 121u8, 39u8, 1u8, 0u8, 0u8,
         0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 105u8, 109u8, 112u8, 108u8,
         105u8, 99u8, 105u8, 116u8, 32u8, 102u8, 108u8, 111u8, 119u8, 32u8,
         107u8, 101u8, 121u8, 39u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8, 101u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_nested_flow_mapping_sequence_and_mappings_R52L()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/R52L/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 111u8,
         112u8, 49u8, 1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         105u8, 116u8, 101u8, 109u8, 49u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 107u8, 101u8, 121u8, 50u8, 0u8, 0u8, 0u8, 0u8, 6u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8, 117u8, 101u8,
         50u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         105u8, 116u8, 101u8, 109u8, 51u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 111u8, 112u8, 50u8, 0u8, 0u8, 0u8,
         0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 118u8, 97u8, 108u8,
         117u8, 101u8, 50u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_1_sequence_of_scalars_FQ7F()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/FQ7F/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 77u8, 97u8, 114u8,
         107u8, 32u8, 77u8, 99u8, 71u8, 119u8, 105u8, 114u8, 101u8, 0u8, 0u8,
         0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 97u8, 109u8,
         109u8, 121u8, 32u8, 83u8, 111u8, 115u8, 97u8, 0u8, 0u8, 0u8, 0u8,
         11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 75u8, 101u8, 110u8, 32u8,
         71u8, 114u8, 105u8, 102u8, 102u8, 101u8, 121u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_18_multi_line_flow_scalars_4CQQ()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4CQQ/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 112u8, 108u8, 97u8,
         105u8, 110u8, 0u8, 0u8, 0u8, 0u8, 38u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 84u8, 104u8, 105u8, 115u8, 32u8, 117u8, 110u8, 113u8, 117u8,
         111u8, 116u8, 101u8, 100u8, 32u8, 115u8, 99u8, 97u8, 108u8, 97u8,
         114u8, 32u8, 115u8, 112u8, 97u8, 110u8, 115u8, 32u8, 109u8, 97u8,
         110u8, 121u8, 32u8, 108u8, 105u8, 110u8, 101u8, 115u8, 46u8, 0u8,
         0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 113u8, 117u8,
         111u8, 116u8, 101u8, 100u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 34u8, 83u8, 111u8, 32u8, 100u8, 111u8, 101u8,
         115u8, 32u8, 116u8, 104u8, 105u8, 115u8, 32u8, 113u8, 117u8, 111u8,
         116u8, 101u8, 100u8, 32u8, 115u8, 99u8, 97u8, 108u8, 97u8, 114u8,
         46u8, 92u8, 110u8, 34u8] ; let res = crate :: parse(& buf) . unwrap()
        ; let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_20_single_pair_explicit_entry_CT4Q()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/CT4Q/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8, 111u8, 32u8,
         98u8, 97u8, 114u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 98u8, 97u8, 122u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_sequence_in_block_mapping_8QBE()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/8QBE/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         121u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 105u8,
         116u8, 101u8, 109u8, 49u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 105u8, 116u8, 101u8, 109u8, 50u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_multiple_pair_block_mapping_J5UC()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/J5UC/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         98u8, 108u8, 117u8, 101u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 98u8, 97u8, 114u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 114u8, 114u8, 114u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 97u8, 122u8,
         0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 106u8,
         97u8, 122u8, 122u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_6_12_separation_spaces_Q9WF()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/Q9WF/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 105u8, 114u8, 115u8,
         116u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         83u8, 97u8, 109u8, 109u8, 121u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 108u8, 97u8, 115u8, 116u8, 0u8, 0u8, 0u8,
         0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 111u8, 115u8,
         97u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8,
         114u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         54u8, 53u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 97u8, 118u8, 103u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 48u8, 46u8, 50u8, 55u8, 56u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_single_entry_block_sequence_65WH()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/65WH/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_6_5_empty_lines_5GBF()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/5GBF/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 70u8, 111u8, 108u8,
         100u8, 105u8, 110u8, 103u8, 0u8, 0u8, 0u8, 0u8, 28u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 34u8, 69u8, 109u8, 112u8, 116u8, 121u8, 32u8,
         108u8, 105u8, 110u8, 101u8, 92u8, 110u8, 97u8, 115u8, 32u8, 97u8,
         32u8, 108u8, 105u8, 110u8, 101u8, 32u8, 102u8, 101u8, 101u8, 100u8,
         34u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         67u8, 104u8, 111u8, 109u8, 112u8, 105u8, 110u8, 103u8, 0u8, 0u8, 0u8,
         0u8, 22u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 67u8, 108u8,
         105u8, 112u8, 112u8, 101u8, 100u8, 32u8, 101u8, 109u8, 112u8, 116u8,
         121u8, 32u8, 108u8, 105u8, 110u8, 101u8, 115u8, 92u8, 110u8] ; let
        res = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode
        :: deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_10_plain_characters_DBG4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/DBG4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 58u8, 58u8, 118u8,
         101u8, 99u8, 116u8, 111u8, 114u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 58u8, 32u8, 45u8, 32u8, 40u8, 41u8,
         34u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         85u8, 112u8, 44u8, 32u8, 117u8, 112u8, 44u8, 32u8, 97u8, 110u8,
         100u8, 32u8, 97u8, 119u8, 97u8, 121u8, 33u8, 0u8, 0u8, 0u8, 0u8, 4u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 45u8, 49u8, 50u8, 51u8, 0u8, 0u8,
         0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 116u8,
         116u8, 112u8, 58u8, 47u8, 47u8, 101u8, 120u8, 97u8, 109u8, 112u8,
         108u8, 101u8, 46u8, 99u8, 111u8, 109u8, 47u8, 102u8, 111u8, 111u8,
         35u8, 98u8, 97u8, 114u8, 1u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 58u8, 58u8, 118u8, 101u8, 99u8, 116u8, 111u8, 114u8, 0u8, 0u8,
         0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 58u8, 32u8,
         45u8, 32u8, 40u8, 41u8, 34u8, 0u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 85u8, 112u8, 44u8, 32u8, 117u8, 112u8,
         32u8, 97u8, 110u8, 100u8, 32u8, 97u8, 119u8, 97u8, 121u8, 33u8, 34u8,
         0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 45u8,
         49u8, 50u8, 51u8, 0u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 104u8, 116u8, 116u8, 112u8, 58u8, 47u8, 47u8, 101u8, 120u8,
         97u8, 109u8, 112u8, 108u8, 101u8, 46u8, 99u8, 111u8, 109u8, 47u8,
         102u8, 111u8, 111u8, 35u8, 98u8, 97u8, 114u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_6_5_empty_lines_1_3_XV9V()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/XV9V/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 70u8, 111u8, 108u8,
         100u8, 105u8, 110u8, 103u8, 0u8, 0u8, 0u8, 0u8, 28u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 34u8, 69u8, 109u8, 112u8, 116u8, 121u8, 32u8,
         108u8, 105u8, 110u8, 101u8, 92u8, 110u8, 97u8, 115u8, 32u8, 97u8,
         32u8, 108u8, 105u8, 110u8, 101u8, 32u8, 102u8, 101u8, 101u8, 100u8,
         34u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         67u8, 104u8, 111u8, 109u8, 112u8, 105u8, 110u8, 103u8, 0u8, 0u8, 0u8,
         0u8, 22u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 67u8, 108u8,
         105u8, 112u8, 112u8, 101u8, 100u8, 32u8, 101u8, 109u8, 112u8, 116u8,
         121u8, 32u8, 108u8, 105u8, 110u8, 101u8, 115u8, 92u8, 110u8] ; let
        res = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode
        :: deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_15_block_sequence_entry_types_W42U()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/W42U/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 98u8, 108u8, 111u8,
         99u8, 107u8, 32u8, 110u8, 111u8, 100u8, 101u8, 92u8, 110u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 110u8, 101u8, 0u8,
         0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 119u8,
         111u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8,
         110u8, 101u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 116u8, 119u8, 111u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_various_combinations_of_explicit_block_mappings_KK5P()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/KK5P/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 111u8, 109u8,
         112u8, 108u8, 101u8, 120u8, 49u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 97u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         99u8, 111u8, 109u8, 112u8, 108u8, 101u8, 120u8, 50u8, 2u8, 0u8, 0u8,
         0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 111u8, 109u8, 112u8, 108u8, 101u8,
         120u8, 51u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8,
         0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 98u8,
         92u8, 110u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 99u8, 111u8, 109u8, 112u8, 108u8, 101u8, 120u8, 52u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 97u8, 92u8, 110u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 111u8, 109u8,
         112u8, 108u8, 101u8, 120u8, 53u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 97u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         98u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_8_16_block_mappings_TE2A()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/TE2A/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 108u8,
         111u8, 99u8, 107u8, 32u8, 109u8, 97u8, 112u8, 112u8, 105u8, 110u8,
         103u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8,
         101u8, 121u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 118u8, 97u8, 108u8, 117u8, 101u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_2_6_mapping_of_mappings_ZF4X()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/ZF4X/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 77u8, 97u8, 114u8,
         107u8, 32u8, 77u8, 99u8, 71u8, 119u8, 105u8, 114u8, 101u8, 2u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 114u8, 0u8, 0u8, 0u8,
         0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 54u8, 53u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 118u8, 103u8,
         0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 48u8,
         46u8, 50u8, 55u8, 56u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 83u8, 97u8, 109u8, 109u8, 121u8, 32u8, 83u8, 111u8,
         115u8, 97u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         104u8, 114u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 54u8, 51u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 97u8, 118u8, 103u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 48u8, 46u8, 50u8, 56u8, 56u8] ; let res = crate
        :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_5_double_quoted_line_breaks_1_3_Q8AD()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/Q8AD/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 57u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8,
         102u8, 111u8, 108u8, 100u8, 101u8, 100u8, 32u8, 116u8, 111u8, 32u8,
         97u8, 32u8, 115u8, 112u8, 97u8, 99u8, 101u8, 44u8, 92u8, 110u8,
         116u8, 111u8, 32u8, 97u8, 32u8, 108u8, 105u8, 110u8, 101u8, 32u8,
         102u8, 101u8, 101u8, 100u8, 44u8, 32u8, 111u8, 114u8, 32u8, 92u8,
         116u8, 32u8, 92u8, 116u8, 110u8, 111u8, 110u8, 45u8, 99u8, 111u8,
         110u8, 116u8, 101u8, 110u8, 116u8, 34u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_8_literal_content_1_3_T26H()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/T26H/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8,
         92u8, 110u8, 92u8, 110u8, 108u8, 105u8, 116u8, 101u8, 114u8, 97u8,
         108u8, 92u8, 110u8, 32u8, 92u8, 110u8, 92u8, 110u8, 116u8, 101u8,
         120u8, 116u8, 92u8, 110u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_empty_stream_AVM7()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/AVM7/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_various_trailing_tabs_DC7X()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/DC7X/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 101u8,
         113u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8] ;
        let res = crate :: parse(& buf) . unwrap() ; let expected : Yaml =
        bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_nested_implicit_complex_keys_4FJ6()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4FJ6/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8,
         0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8,
         0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 50u8,
         51u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_6_2_indentation_indicators_A2M4()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/A2M4/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 1u8, 0u8, 0u8, 0u8,
         2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_allowed_characters_in_plain_scalars_FBC9()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/FBC9/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 102u8,
         101u8, 0u8, 0u8, 0u8, 0u8, 80u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         97u8, 33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8,
         43u8, 44u8, 45u8, 46u8, 47u8, 48u8, 57u8, 58u8, 59u8, 60u8, 61u8,
         62u8, 63u8, 64u8, 65u8, 90u8, 91u8, 92u8, 92u8, 93u8, 94u8, 95u8,
         96u8, 97u8, 122u8, 123u8, 124u8, 125u8, 126u8, 32u8, 33u8, 34u8,
         35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8, 43u8, 44u8, 45u8,
         46u8, 47u8, 48u8, 57u8, 58u8, 59u8, 60u8, 61u8, 62u8, 63u8, 64u8,
         65u8, 90u8, 91u8, 92u8, 92u8, 93u8, 94u8, 95u8, 96u8, 97u8, 122u8,
         123u8, 124u8, 125u8, 126u8, 0u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 102u8, 101u8, 32u8, 113u8, 117u8,
         101u8, 115u8, 116u8, 105u8, 111u8, 110u8, 32u8, 109u8, 97u8, 114u8,
         107u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         63u8, 102u8, 111u8, 111u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 102u8, 101u8, 32u8, 99u8, 111u8,
         108u8, 111u8, 110u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 58u8, 102u8, 111u8, 111u8, 0u8, 0u8, 0u8, 0u8, 9u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 102u8, 101u8, 32u8,
         100u8, 97u8, 115u8, 104u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 45u8, 102u8, 111u8, 111u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_mapping_with_missing_values_7W2P()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/7W2P/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_multiline_plain_flow_mapping_key_without_value_8KB6()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/8KB6/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 105u8, 110u8, 103u8,
         108u8, 101u8, 32u8, 108u8, 105u8, 110u8, 101u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 109u8, 117u8, 108u8, 116u8, 105u8, 32u8,
         108u8, 105u8, 110u8, 101u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 98u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_9_4_explicit_documents_UT92()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/UT92/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_plain_dashes_in_flow_sequence_G5U8()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/G5U8/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 45u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 45u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_17_flow_mapping_separate_values_4ABK()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4ABK/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 117u8, 110u8,
         113u8, 117u8, 111u8, 116u8, 101u8, 100u8, 0u8, 0u8, 0u8, 0u8, 10u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 115u8, 101u8, 112u8, 97u8,
         114u8, 97u8, 116u8, 101u8, 34u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 116u8, 116u8, 112u8, 58u8, 47u8,
         47u8, 102u8, 111u8, 111u8, 46u8, 99u8, 111u8, 109u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 109u8, 105u8, 116u8,
         116u8, 101u8, 100u8, 32u8, 118u8, 97u8, 108u8, 117u8, 101u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 109u8, 105u8, 116u8,
         116u8, 101u8, 100u8, 32u8, 107u8, 101u8, 121u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_8_1_block_scalar_header_P2AD()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/P2AD/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 108u8,
         105u8, 116u8, 101u8, 114u8, 97u8, 108u8, 92u8, 110u8, 0u8, 0u8, 0u8,
         0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 32u8, 102u8,
         111u8, 108u8, 100u8, 101u8, 100u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8,
         9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 107u8, 101u8, 101u8,
         112u8, 92u8, 110u8, 92u8, 110u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 62u8, 32u8, 115u8, 116u8, 114u8, 105u8,
         112u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_8_18_implicit_block_mapping_entries_S3PD()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/S3PD/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 112u8, 108u8, 97u8,
         105u8, 110u8, 32u8, 107u8, 101u8, 121u8, 0u8, 0u8, 0u8, 0u8, 13u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 105u8, 110u8, 45u8, 108u8, 105u8,
         110u8, 101u8, 32u8, 118u8, 97u8, 108u8, 117u8, 101u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 113u8, 117u8, 111u8, 116u8,
         101u8, 100u8, 32u8, 107u8, 101u8, 121u8, 34u8, 1u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 110u8, 116u8, 114u8, 121u8] ;
        let res = crate :: parse(& buf) . unwrap() ; let expected : Yaml =
        bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_explicit_key_and_value_seperated_by_comment_X8DW()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/X8DW/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 101u8,
         121u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         118u8, 97u8, 108u8, 117u8, 101u8] ; let res = crate :: parse(& buf) .
        unwrap() ; let expected : Yaml = bincode :: deserialize(& exp_lit) .
        unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_plain_url_in_flow_mapping_UDM2()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/UDM2/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 117u8, 114u8, 108u8, 0u8,
         0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 116u8,
         116u8, 112u8, 58u8, 47u8, 47u8, 101u8, 120u8, 97u8, 109u8, 112u8,
         108u8, 101u8, 46u8, 111u8, 114u8, 103u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_block_submapping_KMK3()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/KMK3/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8,
         97u8, 114u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 49u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 98u8, 97u8, 122u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 50u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_flow_sequence_DHP8()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/DHP8/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         98u8, 97u8, 114u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 52u8, 50u8] ; let res = crate :: parse(& buf) . unwrap() ;
        let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_allowed_characters_in_quoted_mapping_key_6SLA()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6SLA/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 28u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 102u8,
         111u8, 111u8, 92u8, 110u8, 98u8, 97u8, 114u8, 58u8, 98u8, 97u8,
         122u8, 92u8, 116u8, 120u8, 32u8, 92u8, 92u8, 36u8, 37u8, 94u8, 38u8,
         42u8, 40u8, 41u8, 120u8, 34u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 50u8, 51u8, 0u8, 0u8, 0u8, 0u8, 22u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 120u8, 92u8, 92u8, 110u8, 121u8,
         58u8, 122u8, 92u8, 92u8, 116u8, 120u8, 32u8, 36u8, 37u8, 94u8, 38u8,
         42u8, 40u8, 41u8, 120u8, 39u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 50u8, 52u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_flow_mapping_54T7()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/54T7/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         121u8, 111u8, 117u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 98u8, 97u8, 114u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 97u8, 114u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_colon_and_adjacent_value_on_next_line_5MUD()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/5MUD/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 102u8, 111u8,
         111u8, 34u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 98u8, 97u8, 114u8] ; let res = crate :: parse(& buf) . unwrap()
        ; let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_colon_in_double_quoted_string_4UYU()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/4UYU/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8,
         102u8, 111u8, 111u8, 58u8, 32u8, 98u8, 97u8, 114u8, 34u8, 58u8, 32u8,
         98u8, 97u8, 122u8, 34u8] ; let res = crate :: parse(& buf) . unwrap()
        ; let expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_sequence_with_same_indentation_as_parent_mapping_AZ63()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/AZ63/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 1u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 50u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 51u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8,
         1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 53u8] ; let res = crate ::
        parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_zero_indented_block_scalar_FP8R()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/FP8R/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 62u8,
         108u8, 105u8, 110u8, 101u8, 49u8, 32u8, 108u8, 105u8, 110u8, 101u8,
         50u8, 32u8, 108u8, 105u8, 110u8, 101u8, 51u8, 92u8, 110u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_7_15_flow_mappings_5C5M()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/5C5M/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 110u8, 101u8, 0u8,
         0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 119u8,
         111u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         116u8, 104u8, 114u8, 101u8, 101u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 111u8, 117u8, 114u8, 2u8, 0u8, 0u8,
         0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 105u8, 118u8, 101u8, 0u8,
         0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 105u8,
         120u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         115u8, 101u8, 118u8, 101u8, 110u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 105u8, 103u8, 104u8, 116u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_multiline_scalar_at_top_level_1_3_EX5H()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/EX5H/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8,
         32u8, 98u8, 32u8, 99u8, 32u8, 100u8, 92u8, 110u8, 101u8] ; let res =
        crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_document_start_on_last_line_PUW8()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/PUW8/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] ; let res
        = crate :: parse(& buf) . unwrap() ; let expected : Yaml = bincode ::
        deserialize(& exp_lit) . unwrap() ; assert_eq ! (expected, res) ;
    } #[test] fn test_literal_block_scalar_M29M()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/M29M/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8,
         0u8, 0u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 97u8, 98u8,
         92u8, 110u8, 92u8, 110u8, 99u8, 100u8, 92u8, 110u8, 101u8, 102u8,
         92u8, 110u8] ; let res = crate :: parse(& buf) . unwrap() ; let
        expected : Yaml = bincode :: deserialize(& exp_lit) . unwrap() ;
        assert_eq ! (expected, res) ;
    } #[test] fn test_spec_example_6_1_indentation_spaces_6HB6()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/6HB6/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 78u8, 111u8,
         116u8, 32u8, 105u8, 110u8, 100u8, 101u8, 110u8, 116u8, 101u8, 100u8,
         2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 66u8, 121u8, 32u8,
         111u8, 110u8, 101u8, 32u8, 115u8, 112u8, 97u8, 99u8, 101u8, 0u8, 0u8,
         0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 66u8,
         121u8, 32u8, 102u8, 111u8, 117u8, 114u8, 92u8, 110u8, 32u8, 32u8,
         115u8, 112u8, 97u8, 99u8, 101u8, 115u8, 92u8, 110u8, 0u8, 0u8, 0u8,
         0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 70u8, 108u8, 111u8,
         119u8, 32u8, 115u8, 116u8, 121u8, 108u8, 101u8, 1u8, 0u8, 0u8, 0u8,
         3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 66u8, 121u8, 32u8, 116u8, 119u8, 111u8,
         0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 65u8,
         108u8, 115u8, 111u8, 32u8, 98u8, 121u8, 32u8, 116u8, 119u8, 111u8,
         0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8,
         116u8, 105u8, 108u8, 108u8, 32u8, 98u8, 121u8, 32u8, 116u8, 119u8,
         111u8] ; let res = crate :: parse(& buf) . unwrap() ; let expected :
        Yaml = bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    } #[test] fn test_spec_example_2_28_log_file_RZT7()
    {
        use crate :: Yaml :: * ; use crate :: Yaml ; use serde :: Deserialize
        ; use std :: io :: Read ; let mut file = std :: fs :: File ::
        open("./yaml-test-suite/RZT7/in.yaml") . unwrap() ; let mut buf =
        String :: new() ; file . read_to_string(& mut buf) . unwrap() ; let
        exp_lit = vec !
        [2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 68u8, 97u8, 116u8,
         101u8, 0u8, 0u8, 0u8, 0u8, 22u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         50u8, 48u8, 48u8, 49u8, 45u8, 49u8, 49u8, 45u8, 50u8, 51u8, 32u8,
         49u8, 53u8, 58u8, 48u8, 51u8, 58u8, 49u8, 55u8, 32u8, 45u8, 53u8,
         0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 85u8,
         115u8, 101u8, 114u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 101u8, 100u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 70u8, 97u8, 116u8, 97u8, 108u8, 0u8, 0u8, 0u8,
         0u8, 22u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 85u8, 110u8, 107u8,
         110u8, 111u8, 119u8, 110u8, 32u8, 118u8, 97u8, 114u8, 105u8, 97u8,
         98u8, 108u8, 101u8, 32u8, 34u8, 98u8, 97u8, 114u8, 34u8, 0u8, 0u8,
         0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 116u8, 97u8,
         99u8, 107u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 102u8, 105u8,
         108u8, 101u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 84u8, 111u8, 112u8, 67u8, 108u8, 97u8, 115u8, 115u8, 46u8,
         112u8, 121u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 108u8, 105u8, 110u8, 101u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 50u8, 51u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 111u8, 100u8, 101u8, 0u8, 0u8,
         0u8, 0u8, 27u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 124u8, 120u8,
         32u8, 61u8, 32u8, 77u8, 111u8, 114u8, 101u8, 79u8, 98u8, 106u8,
         101u8, 99u8, 116u8, 40u8, 34u8, 51u8, 52u8, 53u8, 92u8, 92u8, 110u8,
         34u8, 41u8, 92u8, 110u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 102u8, 105u8, 108u8, 101u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 77u8, 111u8, 114u8, 101u8, 67u8, 108u8,
         97u8, 115u8, 115u8, 46u8, 112u8, 121u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8,
         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 108u8, 105u8, 110u8, 101u8, 0u8, 0u8,
         0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 53u8, 56u8, 0u8,
         0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 111u8,
         100u8, 101u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
         0u8, 124u8, 102u8, 111u8, 111u8, 32u8, 61u8, 32u8, 98u8, 97u8, 114u8]
        ; let res = crate :: parse(& buf) . unwrap() ; let expected : Yaml =
        bincode :: deserialize(& exp_lit) . unwrap() ; assert_eq !
        (expected, res) ;
    }
}